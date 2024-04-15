
use esp_idf_hal::{delay::{Delay, FreeRtos}, 
    gpio::*, i2c, io::Write, 
    peripherals::Peripherals, 
    units::FromValueType
};

use liquid_crystal::{
    Bus4Bits, Clear, Command, LiquidCrystal, Text, I2C as LiquidCrystal_I2C, LCD16X2
};

use esp_idf_svc::{eventloop::EspSystemEventLoop, 
    mqtt::client::{EspMqttClient, MqttClientConfiguration, QoS}, 
    nvs::EspDefaultNvsPartition, systime::EspSystemTime, wifi::{EspWifi, AsyncWifi},
    http::server::{Configuration as EspHttpConf, EspHttpServer},
    timer::EspTaskTimerService};

use embedded_svc::{wifi::{AuthMethod, ClientConfiguration, Configuration},
    http::Method};

use std::sync::{Arc, Mutex};

use heapless::String as hpString;
use anyhow;
use futures::executor::block_on;

mod web;

const HD44780_ADDR: u8 = 0x27;
const WIFI_SSID: &str = "Galaxy M62";
const WIFI_PASS: &str = "chapolin";
//const MQTT_ENDPOINT: &str = "mqtt://192.168.100.141:1883";
const MQTT_ENDPOINT: &str = "mqtt://test.mosquitto.org";
const MQTT_CLIENT_ID: &str = "NODEMCU_ESP-32s";
const MQTT_TOPIC_NAME: &str = "test/topic";
fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = 
    EspSystemEventLoop::take().unwrap();
    let nvs = 
    EspDefaultNvsPartition::take().unwrap();
    let timer_service = 
    EspTaskTimerService::new().unwrap();

    let mut wifi = AsyncWifi::wrap(EspWifi::new(
        peripherals.modem, 
        sysloop.clone(),
        Some(nvs))?,
        sysloop,
        timer_service.clone(),
    )?;
    block_on(connect_wifi(&mut wifi))?; 

    let mqtt_config = MqttClientConfiguration{
        client_id: Some(MQTT_CLIENT_ID),
        ..Default::default()
    };

    let (mut _client, mut _connection) 
    = EspMqttClient::new(MQTT_ENDPOINT, &mqtt_config).unwrap();
    println!("MQTT Client started");

    //client.subscribe(MQTT_TOPIC_NAME, QoS::AtLeastOnce)?;

    // client.publish(
    //     MQTT_TOPIC_NAME,
    //     QoS::AtMostOnce,
    //     false,
    //     format!("Hello from {}!", MQTT_TOPIC_NAME).as_bytes(),
    // )?;

    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    let _cfg = i2c::config::Config::new()
        .baudrate(400.kHz().into());
    let _i2c = i2c::I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &_cfg,
    ).unwrap();

    let mut interface= 
    LiquidCrystal_I2C::new(_i2c, HD44780_ADDR);

    let mut lcd = 
    LiquidCrystal::new(&mut interface, Bus4Bits, LCD16X2);

    let mut delay = Delay::new(1*(u32::pow(10, 3)));
    lcd.begin(&mut delay);
    
    lcd.write(&mut delay,Command(Clear))
        .write(&mut delay,Text("Distance:"));

    let mut _trig = PinDriver::output(peripherals.pins.gpio32).unwrap();
    let _echo = PinDriver::input(peripherals.pins.gpio33).unwrap();

    let distance_value = Arc::new(Mutex::new(0.0));
    let distance_value_clone = distance_value.clone();
    let _unused = drop(distance_value_clone.lock().unwrap());

    let mut server = EspHttpServer::new(&EspHttpConf::default())?;
    server.fn_handler("/", Method::Get, move |request| {
        let html = web::distance_html();
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        anyhow::Ok(())
    })?;

    server.fn_handler("/distance", Method::Get, move |request| {
        let temp = distance_value_clone.lock().unwrap();
        let dist_value = *temp;
        let html = format!("{:.3}", dist_value);
        let mut response = request.into_ok_response()?;    
        response.write_all(html.as_bytes())?;
        anyhow::Ok(())
    })?;

    loop {
        lcd.set_cursor(&mut delay, 1, 3);
        FreeRtos::delay_ms(300);
        PinDriver::set_high(&mut _trig).unwrap();
        Delay::new(10);
        PinDriver::set_low(&mut _trig).unwrap();
        
        while !_echo.is_high() {}
        
        let echo_start = EspSystemTime{}.now().as_micros();
        
        while !_echo.is_low() {}

        let echo_end = EspSystemTime{}.now().as_micros();

        let pulse_duration = echo_end.wrapping_sub(echo_start);
        let distance_cm = (pulse_duration as f32 * 0.034483)/ 2.0;
        
        FreeRtos::delay_ms(200);
        
        lcd.write(&mut delay,Text(&format!("{:.3} cm  ", distance_cm)));

        let mut distance_value_lock = distance_value.lock().unwrap();
        *distance_value_lock = distance_cm;
        }
}

async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: hpString::try_from(WIFI_SSID).unwrap(),
        password: hpString::try_from(WIFI_PASS).unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    wifi.start().await?;
    wifi.connect().await?;
    wifi.wait_netif_up().await?;
    println!("Wifi is ready!!");

    Ok(())
}

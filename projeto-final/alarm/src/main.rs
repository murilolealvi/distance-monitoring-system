use std::str::from_utf8;
use futures::executor::block_on;
use anyhow;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_sys::esp_http_client_read;
use heapless::String as hpString;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals, gpio::*};
use embedded_svc::http::client::Client;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::{Configuration as HttpConfig, EspHttpConnection};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi, AsyncWifi};
use embassy_executor::Spawner;




const WIFI_SSID: &str = "Dimitry";
const WIFI_PASS: &str = "010987#@";
const HTTPSERVER_API: &str = "http://192.168.100.147/distance";
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

    let mut red = PinDriver::output(peripherals.pins.gpio25).unwrap();
    let mut green = PinDriver::output(peripherals.pins.gpio33).unwrap();
    let mut blue = PinDriver::output(peripherals.pins.gpio32).unwrap();
    let mut buzzer  = PinDriver::output(peripherals.pins.gpio26).unwrap();


    let mut wifi = AsyncWifi::wrap(EspWifi::new(
        peripherals.modem, 
        sysloop.clone(),
        Some(nvs))?,
        sysloop,
        timer_service.clone(),
    )?;
    block_on(connect_wifi(&mut wifi))?;  


    let httpconnection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    let mut httpclient = Client::wrap(httpconnection);
    let mut buffer = [0_u8; 256];

    loop {
        let request = httpclient.get(HTTPSERVER_API)?;
        let mut response = request.submit()?;
        println!("HTTP code: {}\n", response.status());
        let bytes = response.read(&mut buffer)?;
        let string = from_utf8(&buffer[0..bytes])?;
        let distance_cm: f32 = string.parse().unwrap();
        println!("Distance: {}",string);

        if distance_cm > 40.0 {
            red.set_low()?;
            blue.set_low()?;
            buzzer.set_low()?;
            FreeRtos::delay_ms(100);
            green.set_high()?;
        }
        else if distance_cm > 10.0 && distance_cm < 40.0 {
            red.set_low()?;
            green.set_low()?;
            buzzer.set_low()?;
            FreeRtos::delay_ms(100);
            blue.set_high()?;
        }
        else {
            green.set_low()?;
            blue.set_low()?;
            FreeRtos::delay_ms(100);
            red.set_high()?;
            buzzer.set_high()?;
        }
        
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


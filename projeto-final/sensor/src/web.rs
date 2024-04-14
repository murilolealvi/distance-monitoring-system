
pub fn distance_html() -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>RUST WEBSERVER</title>
                <style>
                    body {{
                        font-family: Arial, sans-serif;
                        text-align: center;
                    }}

                    #dist {{
                        font-size: 3rem;
                        margin: 20px;
                    }}
                </style>
            </head>
            <body>
                <h1>SENSORIAMENTO DA DISTÂNCIA</h1>
                <div id="dist"></div>

                <script>
                    async function getDistanceValue() {{
                        var geturl = window.location.href.slice(0, -1) + "/distance";
                        const distanceElement = document.getElementById('dist');
                        const distanceValue = await fetch(geturl);
                        const distance = await distanceValue.text();
                        const distanceString = `Distância : ${{distance}} cm`;
                        distanceElement.textContent = distanceString;
                    }}


                    setInterval(getDistanceValue, 500);
                    getDistanceValue();
                </script>
            </body>
        </html>
        "#
    )
}
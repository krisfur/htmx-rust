use axum::{
    Form, Router,
    response::Html,
    routing::{get, post},
};
use reqwest;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/validate", post(validate))
        .route("/weather", get(weather))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct UserInput {
    name: String,
}

async fn validate(Form(input): Form<UserInput>) -> Html<String> {
    if input.name.is_empty() {
        Html("<p style=\"color: red;\">Name cannot be empty!</p>".to_string())
    } else {
        Html(format!("<p>Hello, {}!</p>", input.name))
    }
}

#[derive(Deserialize)]
struct WeatherResponse {
    temperature: String,
    wind: String,
    description: String,
}

async fn weather() -> Html<String> {
    let weather_api_url = "https://goweather.xyz/weather/london";
    let response = reqwest::get(weather_api_url).await;

    match response {
        Ok(res) => {
            let weather_data = res.json::<WeatherResponse>().await;
            match weather_data {
                Ok(data) => Html(format!(
                    "<div><h3>Weather in London</h3><p>Temperature: {}</p><p>Wind: {}</p><p>Description: {}</p></div>",
                    data.temperature, data.wind, data.description
                )),
                Err(_) => Html("<p>Could not parse weather data.</p>".to_string()),
            }
        }
        Err(_) => Html("<p>Could not fetch weather data.</p>".to_string()),
    }
}

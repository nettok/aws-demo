use lambda_http::tracing;

pub async fn get_hello() -> &'static str {
    tracing::info!("Calling get_hello");
    "Hola Mundo!"
}

use grafana_api::{
    api::auth::ListApiKeysRequest,
    client::{AuthMethod, Client},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let token = dotenv::var("GRAFANA_API_TOKEN").unwrap_or_default();

    let client =
        Client::new("https://yvesbonami.grafana.net").set_auth_method(AuthMethod::Bearer(token));

    let request = ListApiKeysRequest::default();
    let keys = request.send(&client).await?;

    keys.iter().for_each(|s| println!("{:#?}", s));

    Ok(())
}

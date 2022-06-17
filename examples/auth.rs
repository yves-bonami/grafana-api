use grafana_api::{
    api::auth::ListApiKeysRequest,
    client::{AuthMethod, Client},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let client = Client::new("http://localhost:3000").set_auth_method(AuthMethod::Basic {
        username: "admin".into(),
        password: "admin".into(),
    });

    let request = ListApiKeysRequest::default();
    let keys = request.send(&client).await?;

    keys.iter().for_each(|s| println!("{:?}", s));

    Ok(())
}

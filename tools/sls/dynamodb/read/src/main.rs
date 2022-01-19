use lambda_runtime::{handler_fn, Context, Error};
use aws_config::meta::region::RegionProviderChain;
use serde::{Serialize, Deserialize};
use serde_json::{ json, Value };
use tracing::info;
use aws_sdk_dynamodb::model::{
     AttributeValue
};
use aws_sdk_dynamodb::{ Client };

#[derive(Deserialize, Debug)]
struct Request {
    command: String,
}
#[derive(Serialize, Debug)]
struct Response {
    message: String,
}

async fn handler(event: Request, _context: Context) -> Result<Response, Error> {
    info!("[handler-fn] Received event {:?}", event);

    Ok(Response {
        message: event.command.to_uppercase(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .without_time()
        .init();

    lambda_runtime::run(handler_fn(handler)).await
}
/*
async fn get_item(
    client: &Client,
    table: &str,
    key: &str,
) -> Result<Infallible, Error> {

    let request = client
        .get_item()
        .table_name(table)
        .attributes_to_get(key);

    println!("sending the request");
    let resp = request.send().await?;
    resp
}

async fn  xhandler(event: Request, _:Context) -> Result<Value, Error> {
    info!("handle the request");

    println!("the event is {}", req);

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    get_item(&client, "newtable", "justanotherkey")
*/

use lambda_runtime::{handler_fn, Context, Error};
use std::env;
use log::{debug, error, log_enabled, info, Level};
use serde_json::{json, Value};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType
};
use aws_sdk_dynamodb::{Client, Region, PKG_VERSION};
use std::process;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
    println!("-----   Region secured  -----");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    println!("-----   Client -----");

    let resp = client.list_tables().send().await?;

    println!("-----   Tables -----");

    let names= resp.table_names().unwrap_or_default();
    let len = names.len();

    for name in names {
        println!(" --  {} -- ", name);
    }

    let response = format!("**** found some tables ***");
    log::info!("{}", response);

    Ok(json!({"response": response}))
}

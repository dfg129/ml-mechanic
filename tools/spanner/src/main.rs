use lambda_runtime::{handler_fn, Context, Error};
use std::env;
use log::{debug, error, log_enabled, info, Level};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    println!("#################");

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let message = event["message"].as_str().unwrap_or("world");
    let first_name = event["firstName"].as_str().unwrap_or("Anonymous");
    
    println!("{}", env::var("AWS_LAMBDA_FUNCTION_NAME").is_err());
    println!("------------------");
    
    let response = format!("Hello, {}! Your nom de jour  is {}", message,  env::var("AWS_LAMBDA_FUNCTION_NAME").is_err());
    println!("------------------");
    log::info!("{}", response);

    Ok(json!({ "response": response }))
}

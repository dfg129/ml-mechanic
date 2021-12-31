use lambda_runtime::{handler_fn, Context, Error};
// use log::LevelFilter;
use log::{debug, error, log_enabled, info, Level};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let message = event["message"].as_str().unwrap_or("world");
    let first_name = event["firstName"].as_str().unwrap_or("Anonymous");
    
    let response = format!("Hello, {}! Your name is {}", message, first_name);
    log::info!("{}", response);

    Ok(json!({ "response": response }))
}

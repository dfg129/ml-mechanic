use lambda_runtime::handler_fn;
use log::{debug, error, info};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Request {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl std::error::Error for FailureResponse{}

type Response = Result<SuccessResponse, FailureResponse>;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt::init();
    debug!("logger is set up");

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn  handler(req: Request, _ctx: lambda_runtime::Context) -> Response {
    info!("handle the request");

    let bucket_name = std::env::var("BUCKET_NAME").expect("A BUCKET_NAME must be set");

    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

   // let client = Client::new(&config);
    let filename = format!("{}.txt", time::OffsetDateTime::now_utc().unix_timestamp());

    let _ = s3_client
        .put_object()
        .bucket(bucket_name)
        .body(req.body.as_bytes().to_owned().into())
        .key(&filename)
        .content_type("text/plani")
        .send()
        .await
        .map_err(|err| {
            error!(
                "failed to upload file '{}' to s3 with error: {}",
                &filename, err

            );
            FailureResponse {
                body: "The lambda encountered trouble - you screwed".to_owned(),
            }
        })?;

    info!(
        "Successfully stored in {}", &&filename
    );

    Ok(SuccessResponse {
        body: format!(
            "the lambda has landed"
        ),
    })
}

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use bcrypt::{verify};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let password = event["password"].as_str().unwrap_or("");
    let hash = event["hash"].as_str().unwrap_or("");

    let valid = verify(password, hash)?;

    // Ok(json!({ "valid": format!("Hello, {}!", first_name) }))
    Ok(json!({ "valid": format!("{}", valid) }))
}

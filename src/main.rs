use lambda_runtime::{Context, Error};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Event {
    first_name: String,
    last_name:String,
}

#[derive(Serialize)]
struct OutPut {
    message:String,
    request_id:String,
}

async fn handler(event: Event, context: Context) -> Result<OutPut, Error> {
    let message = format!("Gidday {} {}", event.first_name, event.last_name);
    Ok(OutPut { message, request_id: context.request_id,})
}
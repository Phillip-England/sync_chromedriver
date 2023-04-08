
use std::error::Error;

use get_chromedriver::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await;
    Ok(())
}

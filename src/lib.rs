use std::error::Error;

pub async fn get_chrome_driver_version() -> String {
    let chrome_driver_latest_url = String::from("https://chromedriver.storage.googleapis.com/LATEST_RELEASE");
    let res = reqwest::get(chrome_driver_latest_url).await.unwrap();
    let body = res.text().await.unwrap();
    body

}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let driver_version = get_chrome_driver_version().await;
    println!("Current Chromedriver Version = {:?}", driver_version);


    Ok(())
}
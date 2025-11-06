use serde::Deserialize;
use std::error::Error;
use std::fs::{self, File};
use std::io::copy;
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}


fn download_image(url: &str, index: usize) -> Result<()>
{
    // Send a request to the URL
    let mut response = reqwest::blocking::get(url)?;

    //folder path
    let path = format!("./dog_pics/dog_{}.jpg", index);

    //File to write the downloaded image to
    let mut file = File::create(&path)?;

    //Copy the image data to the file
    copy(&mut response, &mut file)?;

    println!("Image saved to {}", path);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    //Create a folder to place images
    fs::create_dir_all("./dog_pics").unwrap();

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            //Success
            ApiResult::Success(dog_image) => {
                //Display output
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                //Download & save image
                if let Err(e) = download_image(&dog_image.message, i){
                    println!("Failed to download image: {}", e);
                }
            },
            //API Error
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            //Network Error
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}
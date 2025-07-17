use reqwest;
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read the file containing website URLs
    let file_path = "data/site.txt";
    let file_content = fs::read_to_string(file_path)?;
    let websites: Vec<&str> = file_content.lines().collect();

    if websites.is_empty() {
        eprintln!("Error: The file is empty or contains no valid website URLs.");
        return Ok(());
    }

    // Check each website
    for site in websites {
        let full_url = if !site.starts_with("http://") && !site.starts_with("https://") {
            format!("http://{}", site)
        } else {
            site.to_string()
        };

        match check_website(&full_url).await {
            Ok(true) => println!("{} is UP!", full_url),
            Ok(false) => println!("{} might be DOWN!", full_url),
            Err(e) => println!("Error checking {}: {}", full_url, e),
        }
    }

    Ok(())
}

// Function to check if a website is up
async fn check_website(url: &str) -> Result<bool, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    Ok(response.status().is_success())
}

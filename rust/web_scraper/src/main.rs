use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use std::{error::Error, fs::File};

#[derive(Debug,Serialize)]
struct Quote {
    text: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut page = 1;
    let base_url = "https://quotes.toscrape.com/page/{}/";
    let mut quotes: Vec<Quote> = Vec::new();

    loop {
        let url = base_url.replace("{}", &page.to_string());
        println!("Fetching page {}: {}", page, url);

        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            println!("No more pages found (HTTP({}))", response.status());
            break;
        }
       
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        // Selectors
        let quote_selector = Selector::parse(".quote").unwrap();
        let text_selector = Selector::parse(".text").unwrap();
        let author_selector = Selector::parse(".author").unwrap();
        let next_selector = Selector::parse(".next > a").unwrap();

        let mut found_quotes = 0;

        for quote in document.select(&quote_selector) {
            let text = quote
                .select(&text_selector)
                .next()
                .map(|t| t.text().collect::<Vec<_>>().join(""))
                .unwrap_or_default();
            
            let author = quote
                .select(&author_selector)
                .next()
                .map(|t| t.text().collect::<Vec<_>>().join(""))
                .unwrap_or_default();
       
            quotes.push(Quote { text, author });
            found_quotes += 1;
        }

        println!("\nScraped {} quotes on page {}", found_quotes, page);
        
        // Check if the next link exists
        let has_next = document.select(&next_selector).next().is_some();
    
        if has_next {
            page += 1;
        } else {
            println!("\nNo more quotes to scrape.");
            break;
        } 
    }
   
    println!("\nTotal quotes scraped {}.", quotes.len());

    // Save results
    save_to_json(&quotes)?;
    save_to_csv(&quotes)?;

    println!("\nData saved to quotes.json and quotes.csv\n");
    Ok(())
} 

fn save_to_json(quotes: &Vec<Quote>) -> Result<(), Box<dyn Error>> {
    let file = File::create("quotes.json")?;
    
    serde_json::to_writer_pretty(file, quotes)?;
    
    Ok(())
}

fn save_to_csv(quotes: &Vec<Quote>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("quotes.csv")?;
    
    for quote in quotes {
        writer.serialize(quote)?;
    }
    
    writer.flush()?;

    Ok(())
}












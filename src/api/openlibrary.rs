use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BookMetadata {
    pub title: String,
    pub authors: Vec<String>,
    pub pages: Option<u32>,
    pub publish_year: Option<u32>,
    pub publisher: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AuthorData {
    name: String,
}

#[derive(Deserialize, Debug)]
struct PublisherData {
    name: String,
}

#[derive(Deserialize, Debug)]
struct OpenLibraryBookData {
    title: String,
    #[serde(default)]
    authors: Vec<AuthorData>,
    number_of_pages: Option<u32>,
    publish_date: Option<String>,
    #[serde(default)]
    publishers: Vec<PublisherData>,
}

pub async fn fetch_by_isbn(isbn: &str) -> Result<Option<BookMetadata>, reqwest::Error> {
    let url = format!(
        "https://openlibrary.org/api/books?bibkeys=ISBN:{}&format=json&jscmd=data",
        isbn
    );

    let client = reqwest::Client::builder()
        .user_agent("Readeon/MVP")
        .build()?;

    let response = client.get(&url).send().await?.json::<HashMap<String, OpenLibraryBookData>>().await?;

    let key = format!("ISBN:{}", isbn);
    if let Some(data) = response.get(&key) {
        let authors = data.authors.iter().map(|a| a.name.clone()).collect();
        let publish_year = data.publish_date.as_ref().and_then(|d| {
            // Extract the first 4 digits as year
            d.chars().take(4).collect::<String>().parse::<u32>().ok()
        });
        let publisher = data.publishers.first().map(|p| p.name.clone());

        Ok(Some(BookMetadata {
            title: data.title.clone(),
            authors,
            pages: data.number_of_pages,
            publish_year,
            publisher,
        }))
    } else {
        Ok(None)
    }
}

pub mod date_detection;
pub mod fetch;
pub mod reviewer;

use anyhow::Result;
use chrono::{DateTime, Utc};
pub use reviewer::{NewsReviewReport, NewsReviewer};

pub struct Article {
    pub url: String,
    pub title: String,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
}

pub async fn load_article_from_url(url: &str) -> Result<Article> {
    let page = fetch::fetch_page(url).await?;
    let detected_date = date_detection::detect_published_date(url, &page.html).await;
    let title = extract_title_from_html(&page.html).unwrap_or_else(|| url.to_string());

    Ok(Article {
        url: url.to_string(),
        title,
        content: page.text_content,
        published_at: detected_date,
    })
}

fn extract_title_from_html(html: &str) -> Option<String> {
    let document = scraper::Html::parse_document(html);
    if let Ok(title_sel) = scraper::Selector::parse("title, h1") {
        if let Some(el) = document.select(&title_sel).next() {
            let text = el.text().collect::<Vec<_>>().join(" ");
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

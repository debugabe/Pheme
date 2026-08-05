use anyhow::{Context, Result};
use reqwest::Client;

pub struct FetchedPage {
    pub url: String,
    pub html: String,
    pub text_content: String,
}

pub async fn fetch_page(url: &str) -> Result<FetchedPage> {
    let client = Client::builder()
        .user_agent("PhemePodcastGenerator/1.0")
        .build()?;

    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Falha ao fazer requisição para {}", url))?;

    let html = response
        .text()
        .await
        .with_context(|| format!("Falha ao ler o corpo HTML de {}", url))?;

    let text_content = extract_text_from_html(&html);

    Ok(FetchedPage {
        url: url.to_string(),
        html,
        text_content,
    })
}

pub fn extract_text_from_html(html: &str) -> String {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("p, h1, h2, h3, article, section").unwrap();

    let mut text_buf = String::new();
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        let trimmed = text.trim();
        if !trimmed.is_empty() && trimmed.len() > 30 {
            text_buf.push_str(trimmed);
            text_buf.push('\n');
        }
    }

    if text_buf.trim().is_empty() {
        // Fallback para texto bruto se seletores falharem
        return document.root_element().text().collect::<Vec<_>>().join(" ");
    }

    text_buf
}

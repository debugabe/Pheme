use chrono::{DateTime, Utc};
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;

pub async fn detect_published_date(url: &str, html: &str) -> Option<DateTime<Utc>> {
    // Step 1: RSS/Atom autodiscovery
    if let Some(rss_date) = try_rss_autodiscovery(url, html).await {
        return Some(rss_date);
    }

    // Step 2: News sitemap
    if let Some(sitemap_date) = try_news_sitemap(url).await {
        return Some(sitemap_date);
    }

    // Step 3: Structured Metadata (meta tags & JSON-LD)
    if let Some(meta_date) = try_structured_metadata(html) {
        return Some(meta_date);
    }

    None
}

async fn try_rss_autodiscovery(base_url: &str, html: &str) -> Option<DateTime<Utc>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("link[type='application/rss+xml'], link[type='application/atom+xml']").ok()?;

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let full_url = if href.starts_with("http") {
                href.to_string()
            } else if let Ok(base) = reqwest::Url::parse(base_url) {
                base.join(href).ok()?.to_string()
            } else {
                continue;
            };

            if let Ok(client) = Client::builder().user_agent("Pheme/1.0").build() {
                if let Ok(resp) = client.get(&full_url).send().await {
                    if let Ok(bytes) = resp.bytes().await {
                        if let Ok(feed) = feed_rs::parser::parse(&bytes[..]) {
                            if let Some(first_entry) = feed.entries.first() {
                                if let Some(published) = first_entry.published.or(first_entry.updated) {
                                    return Some(published);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

async fn try_news_sitemap(base_url: &str) -> Option<DateTime<Utc>> {
    if let Ok(parsed) = reqwest::Url::parse(base_url) {
        if let Some(host) = parsed.host_str() {
            let scheme = parsed.scheme();
            let sitemap_urls = vec![
                format!("{}://{}/sitemap-news.xml", scheme, host),
                format!("{}://{}/sitemap.xml", scheme, host),
            ];

            let client = Client::builder().user_agent("Pheme/1.0").build().ok()?;
            for sm_url in sitemap_urls {
                if let Ok(resp) = client.get(&sm_url).send().await {
                    if let Ok(content) = resp.text().await {
                        if let Some(dt) = parse_iso_date_from_str(&content) {
                            return Some(dt);
                        }
                    }
                }
            }
        }
    }
    None
}

fn try_structured_metadata(html: &str) -> Option<DateTime<Utc>> {
    let document = Html::parse_document(html);

    // Meta tags
    let meta_selectors = vec![
        "meta[property='article:published_time']",
        "meta[name='article:published_time']",
        "meta[name='pubdate']",
        "meta[name='publishdate']",
        "meta[name='date']",
    ];

    for sel_str in meta_selectors {
        if let Ok(selector) = Selector::parse(sel_str) {
            for element in document.select(&selector) {
                if let Some(content) = element.value().attr("content") {
                    if let Some(dt) = parse_iso_date_from_str(content) {
                        return Some(dt);
                    }
                }
            }
        }
    }

    // JSON-LD
    if let Ok(json_ld_selector) = Selector::parse("script[type='application/ld+json']") {
        for element in document.select(&json_ld_selector) {
            let text = element.text().collect::<Vec<_>>().join("");
            if let Ok(val) = serde_json::from_str::<Value>(&text) {
                if let Some(dt_str) = extract_json_ld_date(&val) {
                    if let Some(dt) = parse_iso_date_from_str(&dt_str) {
                        return Some(dt);
                    }
                }
            }
        }
    }

    None
}

fn extract_json_ld_date(val: &Value) -> Option<String> {
    if let Some(dt) = val.get("datePublished").and_then(|v| v.as_str()) {
        return Some(dt.to_string());
    }
    if let Some(graph) = val.get("@graph").and_then(|v| v.as_array()) {
        for item in graph {
            if let Some(dt) = item.get("datePublished").and_then(|v| v.as_str()) {
                return Some(dt.to_string());
            }
        }
    }
    None
}

fn parse_iso_date_from_str(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(DateTime::from_naive_utc_and_offset(dt, Utc));
    }
    if let Ok(dt) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        if let Some(naive_dt) = dt.and_hms_opt(0, 0, 0) {
            return Some(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
        }
    }
    None
}

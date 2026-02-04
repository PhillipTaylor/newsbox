use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use rss::Channel;

use crate::model::Article;

#[derive(Clone, Debug)]
pub struct Feed {
    pub name: &'static str,
    pub url: &'static str,
}

pub fn default_feeds() -> Vec<Feed> {
    vec![
        // BBC RSS
        Feed { name: "BBC", url: "https://feeds.bbci.co.uk/news/rss.xml" },
        // Sky News RSS
        Feed { name: "Sky", url: "https://feeds.skynews.com/feeds/rss/home.xml" },
        // FT has multiple feeds; one common public feed is “World”
        // If this changes, you can replace with any FT RSS you have access to.
        Feed { name: "FT", url: "https://www.ft.com/world?format=rss" },
        Feed { name: "New York Times", url: "https://www.nytimes.com/svc/collections/v1/publish/https://www.nytimes.com/section/world/rss.xml" },
        Feed { name: "Independent", url: "http://www.independent.co.uk/news/world/rss" },
        Feed { name: "Guardian", url: "https://www.theguardian.com/world/rss" }
    ]
}

pub async fn fetch_all(client: &Client, feeds: &[Feed]) -> Result<Vec<Article>> {
    let mut all = Vec::new();

    for feed in feeds {
        let items = fetch_feed(client, feed).await
            .with_context(|| format!("fetch_feed failed for {}", feed.name))?;
        all.extend(items);
    }

    // Sort newest first
    all.sort_by(|a, b| b.published.cmp(&a.published));
    Ok(all)
}

async fn fetch_feed(client: &Client, feed: &Feed) -> Result<Vec<Article>> {
    let bytes = client
        .get(feed.url)
        .header("User-Agent", "newsbox/0.1 (Rust; TUI)")
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    let channel = Channel::read_from(&bytes[..])?;

    let mut out = Vec::new();
    for item in channel.items() {
        let title = item.title().unwrap_or("Untitled").trim().to_string();
        let link = item.link().unwrap_or("").trim().to_string();
        if link.is_empty() {
            continue;
        }

        let summary = item
            .description()
            .or_else(|| item.content())
            .unwrap_or("")
            .trim()
            .to_string();

        let published = item.pub_date()
            .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // A stable-ish id: source + link
        let id = format!("{}::{}", feed.name, link);

        out.push(Article {
            id,
            source: feed.name.to_string(),
            title,
            summary,
            link,
            published,
        });
    }

    Ok(out)
}


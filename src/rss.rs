use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use rss::Channel;
use std::fs;
use std::path::Path;

use crate::model::Article;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Feed {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct FeedConfig {
    feeds: Vec<Feed>,
}

pub fn load_feeds_from_yaml() -> Result<Vec<Feed>> {
    let path = Path::new("feeds.yml");

    let contents = fs::read_to_string(path)
        .with_context(|| "Could not read feeds.yml from current directory")?;

    let cfg: FeedConfig = serde_yaml::from_str(&contents)
        .with_context(|| "feeds.yml is not valid YAML")?;

    if cfg.feeds.is_empty() {
        anyhow::bail!("feeds.yml contains no feeds");
    }

    Ok(cfg.feeds)
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
        .get(&feed.url)
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


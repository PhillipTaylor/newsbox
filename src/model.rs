use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Article {
    pub id: String,
    pub source: String,   // e.g. "BBC"
    pub title: String,    // like email subject
    pub summary: String,  // preview body
    pub link: String,
    pub published: Option<DateTime<Utc>>,
}

impl Article {
    pub fn sender_line(&self) -> String {
        self.source.clone()
    }

    pub fn date_line(&self) -> String {
        match self.published {
            Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
            None => "—".to_string(),
        }
    }
}


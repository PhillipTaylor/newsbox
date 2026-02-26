use crate::model::Article;

#[derive(Debug)]
pub struct App {
    pub articles: Vec<Article>,
    pub filtered: Vec<usize>, // indices into articles
    pub selected: usize,       // index into filtered
    pub show_full: bool,
    pub inbox_view_offset: usize,
    pub filter: String,
    pub status: String,
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            articles: vec![],
            filtered: vec![],
            selected: 0,
            show_full: false,
            inbox_view_offset: 0,
            filter: String::new(),
            status: "Press r to refresh. q to quit.".to_string(),
            loading: false,
        }
    }

    pub fn set_articles(&mut self, items: Vec<Article>) {
        self.articles = items;
        self.selected = 0;
        self.inbox_view_offset = 0;
        self.apply_filter();
        self.status = format!("Loaded {} articles", self.filtered.len());
    }

    pub fn apply_filter(&mut self) {
        let f = self.filter.to_lowercase();
        self.filtered = self.articles.iter().enumerate()
            .filter_map(|(i, a)| {
                if f.is_empty() {
                    Some(i)
                } else {
                    let hay = format!("{} {} {}", a.source, a.title, a.summary).to_lowercase();
                    hay.contains(&f).then_some(i)
                }
            })
            .collect();

        if self.selected >= self.filtered.len() {
            self.selected = self.filtered.len().saturating_sub(1);
        }
    }

    pub fn selected_article(&self) -> Option<&Article> {
        let idx = *self.filtered.get(self.selected)?;
        self.articles.get(idx)
    }

    pub fn move_down(&mut self) {
        if self.filtered.is_empty() { return; }
        self.selected = (self.selected + 1).min(self.filtered.len() - 1);
    }

    pub fn move_up(&mut self) {
        if self.filtered.is_empty() { return; }
        self.selected = self.selected.saturating_sub(1);
    }
}


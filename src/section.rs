use maud::{Markup, PreEscaped, Render, html};

pub struct Section {
    pub title: String,
    pub content: Box<PreEscaped<String>>,
}

impl Render for Section {
    fn render(&self) -> Markup {
        html! {
            a.title href = ("#".to_owned() + self.title.as_str()) { (self.title) }
            hr;
            p.content { (*&self.content) }
            hr;
        }
    }
}

use std::fmt;
use maud::{Render,Markup,html,PreEscaped};
use crate::sections;

pub struct Section {
    pub title: String,
    pub content: String,
}

impl Render for Section {
    fn render(&self) -> Markup{
        html! {
            h2.title { (self.title) }
            hr;
            p.content { (PreEscaped(&self.content)) }
            hr;
        }
    }
}

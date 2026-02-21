use crate::fonts::google_fonts;
use maud::{Markup, PreEscaped, html};

const TITLE: &'static str = "yet another website";

pub fn generate() -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { (TITLE) }

            (google_fonts())

                style {
                    (PreEscaped("
                    body { 
                        font-family: 'Pixelify Sans', monospace; 
                        background: #1a1a1a; 
                        color: #eee; 
                    }
                    h2 { font-family: 'Pixelify Sans', sans-serif; }
                    h1 { font-family: 'Pixelify Sans', sans-serif; }
                "))
                }
        }
        h1 { (TITLE) }
    }
}

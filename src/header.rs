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
                            font-family: 'Saira Extra Condensed', sans-serif; 
                            background: #1a1a1a; 
                            color: #eee; 
                            /* Increased body text for high-impact readability */
                            font-size: 2.5rem; 
                            line-height: 1.1;
                            margin: 5vw; /* Margin based on viewport width */
                            font-weight: 400;
                        }

                        h1, h2 { 
                            font-family: 'Saira Extra Condensed', sans-serif; 
                            text-transform: uppercase;
                            line-height: 1;
                            margin-bottom: 1rem;
                        }

                        /* 50rem was massive. 4rem-6rem is usually 'hero' size */
                        h1 { font-size: 5rem; } 
                        h2 { font-size: 3rem; }
                "))
                }
        }
        h1 { (TITLE) }
    }
}

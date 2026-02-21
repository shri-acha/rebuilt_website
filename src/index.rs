use maud::{html,PreEscaped};
use crate::sections::{room};
use crate::fonts::google_fonts;

pub fn generate_index()-> String {
    let sections = vec![room::generate()];

    html!{
        head {
                meta charset="utf-8";
                title { "Yet Another Website in Rust" }  

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
        (google_fonts())
        h1 { "i'm sort of bored!" }

        @for section in &sections {
            (section)
        }
    }.into_string()
}

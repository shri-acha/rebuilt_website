use maud::{html,PreEscaped};
use crate::section::Section;
use crate::sections::{room};

pub fn generate_index()-> String {
    let sections = vec![room::generate()];

    html!{
        h1 { "i'm sort of bored!" }

        @for section in &sections {
            (section)
        }
    }.into_string()
}

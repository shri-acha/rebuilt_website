use crate::section::Section;
use maud::html;

pub fn generate() -> Section {
    Section {
        title: "Building yet another website".to_string(),
        content: Box::new(html! {
            p{
                "I just got bored that I had to do something, so I just ended up using "
                 a href="https://maud.lambda.xyz/" {"maud"}
            }
            br;
        }),
    }
}

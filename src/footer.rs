use maud::{Markup, html};

pub fn generate() -> Markup {
    html! {
        span {
            p { "Shriharsh S. Acharya" }
            img height="50" width="50" src="https://media1.tenor.com/m/l03_S-DyCc8AAAAd/frog-dance.gif"{}
        }
    }
}

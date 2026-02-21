use crate::section::Section;
use maud::html;

pub fn generate() -> Section {
    Section {
        title: "Building yet another website".to_string(),
        content: Box::new(html! {
            p{
                "I just got bored that I had to do something, so I just ended up using "
                 a href="https://maud.lambda.xyz/" {"maud."}
                " here's some of music i've prompted ai to generate:"
                audio controls{
                    source src="assets/Eigener_Rhythmus.mp3" type="audio/mpeg";
                }
                audio controls{
                    source src="assets/Stahlharte_Melodie.mp3" type="audio/mpeg";
                }
            }
            br;
        }),
    }
}

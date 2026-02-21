use maud::{html};
use crate::section::Section;

pub fn generate()->Section{
    Section{
        title: "Hello World".to_string(),
        content: "I was in a room, a rubber room.".to_string()
    }
}

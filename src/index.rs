use crate::sections::{byaw,bulb};
use crate::{footer, header};
use maud::{PreEscaped, html};

pub fn generate_index() -> String {
    html! {

        (header::generate())

        @for section in &vec![
        byaw::generate(),
        // bulb::generate()
        ]{
            (section)
        }

        (footer::generate())
    }
    .into_string()
}

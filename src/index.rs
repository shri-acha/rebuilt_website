use crate::sections::{byaw,bulb,tracker,etymology};
use crate::{footer, header};
use maud::{PreEscaped, html};

pub fn generate_index() -> String {
    html! {

        (header::generate())

        @for section in &vec![
        etymology::generate(),
        tracker::generate(),
        byaw::generate(),
        // bulb::generate()
        ]{
            (section)
        }

        (footer::generate())
    }
    .into_string()
}

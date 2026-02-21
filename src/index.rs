use maud::html;

pub fn generate_index()-> String { 
    html!{
        p { "i'm sort of bored!" }
    }.into_string()
}

use crate::section::Section;
use crate::table::Table;
use maud::html;

pub fn generate() -> Section {
    Section {
        title: "Health".to_string(),
        content: Box::new(html! {

            p{
                "I'll be using this section to track my physical stats from 2026."
            }

            p.markers {
                These are my current health markers: 
                ul {
                    li {"height: 5'7\""}
                    li {"weight: 62 kgs"}
                        (Table {
                        headers: vec!["date".to_string(),"weight".to_string()],
                        data: vec![
                            vec!["26-01-01".to_string(),"62.2".to_string()],
                            vec!["26-02-01".to_string(),"62.6".to_string()],
                            ]
                        })
                }
            }
            br;
            img src="assets/goal.png" {}
        }),
    }
}

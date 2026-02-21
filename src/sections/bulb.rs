use crate::section::Section;
use maud::html;

pub fn generate() -> Section {
    Section {
        title: "i really want to work rework my lamp to be wireless".to_string(),
        content: Box::new(html! {
            p{ 
            "ever since I got my lamp i've been wanting to make it wireless for the power,
            as it has two switches for some odd reason, one for activating the lamp and other for
            actually setting the level of the lamp light." 
            br;
            img src="assets/existing.jpg" height="100" width="100" {}
            br;

            "Now, there are two possible parts I will be able to modify,"
            ul {
                li {"activation switch"}
                li {"touch switch"}
            }
            "the latter one i don't think it is any hard for me to do,
            there's a switch that i have to bypass it with an esp which i have lying around, i'll just have figure out
            what the actual current flowing through the switch is and just replace it with my gpio on the esp to just handle
            the switching through an interface. Maybe I should try to just open it lol"
            br;
            img src="assets/circboard.jpg" height="100" width="100" {}
            br;
            }
            br;
            "So, there's probably a missing diode, near the switch, which I'm not totally sure about, and after that there's the switch.
            I'll try soldering some wires to the points where the wires are already soldered
            "
        }),
    }
}

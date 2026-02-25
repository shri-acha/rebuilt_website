use crate::section::Section;
use maud::html;

pub fn generate() -> Section {
    Section {
        title: "etymology".to_string(),
        content: Box::new(html! {
            p{ 
                p.introduction {
                "now that it has been more than half a decade of surge of popularity of AI."
                "yet I still remember the day I looked up about artificial intelligence on wikipedia"
                "and first thing I was looked was into its history"
                }
                p.body {
                    "honestly, i am not really somebody who looks at things and wonders how i can leverage this "
                    "rather i prefer myself to be somebody who likes to look at the origins of how it began, "
                    "how it was started, because the origins of something reveals facts that are taken for granted."
                    "for eg:"
                    " as somebody who's into developing software, i see myself writing whole lots of abstractions."
                    "abstractions are something very fundamental to computer science, as it is the only way how"
                    " we're able to build big systems like say, a computer."
                    "a computer is able to receive, process and output data, but it all comes of how we're provided the abstractions."
                    "a computer's design and architecture reveals the computers how the computer is able to processes bunch of abstract keywords "
                    "into 0s and 1s. which to me, is very interesting."
                    br;
                    br;
                    "now with the information in my mind that all the keywords that i write aren't something i pulled out of my ass."
                    "this makes me aware about things that were very 'abstracted' away in the first place."
                    br;
                    br;
                    "now, i know this is not an ideal trait but rather this is something that would kill me in the stone age."
                    "as abstractions are really something built to help with the overcome the complexity with leveraging power of well-constructed unknown."
                    "so, that's how humans were able to use axes and maces."
                    "but, imagine if there was nobody who was there to ponder on why? or how?."
                    "that sounds too generic, but it is what it is."
                    "this is what we as humans value."
                    "now there are alot of things that might count as a factor towards an innovation, or an invention"
                    "but i belive the study of start of something definitely points somebody towards the right direction."
                    br;
                    br;
                    "now in software, i utilize many abstractions in form of functions, objects that are provided by libraries."
                    "but there is a common practice that is used within developement of software that if something is simple enough."
                    "implement it by ownself."
                    "if not, use the library."
                    "this makes sense but only when something is verbose enough to provide its description within itself."
                    "now that comes from the fundamentals of the abstractions."
                    "and how we have accepted the norm."
                    br;
                    br;
                    "now, to come to the present, we have progressed alot from and(s) and not(s)."
                    "to making computers non-deterministic by making them \"think by themselves\"."
                    "now, if you think about it in an abstract way, it is just a closed loop system."
                    "this abstraction makes things really simple to understand, but unfortunately there are more levels to it."
                    "there are various model architectures within the closed loop system that we rely on"
                    "it might be a transformer, a neural network."
                    "a special thing about abstractions is that they can be of recursive nature."
                    "recursive in the sense that an abstraction might hold an object of the level of abstraction."
                    "with all this in mind, now if we have to use an abstraction we might just use it through a simple layer of abstraction,"
                    " which in on itself contains a layers of complicated abstraction."
                }
            }
        })
    }
}

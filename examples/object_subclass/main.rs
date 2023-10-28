mod author;

use glib::prelude::*;

fn main() {
    let author = author::Author::new("John", "Doe");
    author.set_name("Jane");
    author.connect("awarded", true, |_author| {
        println!("Author received a new award!");
        None
    });

    println!("Author: {} {}", author.name(), author.surname());
}

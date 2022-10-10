pub mod generated {
    use proc_macros::generate;

    generate!("code/proc_macros/fixtures/person.json");
}

use generated::*;

fn main() {
    let person = Person {
        first_name: "changfeng".to_string(),
        last_name: "lou".to_string(),
        skill: Skill { num: 1.0 },
    };
    println!("{:#?}", person);
}

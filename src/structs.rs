// Structs - custom types!

// Tradtional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// Tuple struct

// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
    age: u16,
    developer: bool,
}

impl Person {
    // Construct a person

    fn new(first: &str, last: &str, _age: u16, dev: bool) -> Person {
        return Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age: _age,
            developer: dev,
        };
    }

    // Get full name

    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    // hehe rust and python do share a naming convention and a weird alias for this.

    // Set last name

    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // Name as tuple


}
pub fn run() {
    //   let mut c = Color {
    //       red: 255,
    //       green: 0,
    //       blue: 0
    //   };

    //   c.red = 150;
    //   println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut c = Color(55, 200, 0);

    // c.0 = 0;

    // Tuple structs don't need attribute naming

    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Rafayel", "Amirkhanyan", 14, true);
    let mut married_p = Person::new("Jane", "Doe", 28, false);

    

    married_p.set_last_name("Dough");

    if p.developer == false {
        println!("Person {} {} is not a developer", married_p.first_name, married_p.last_name) //  you could also use the full_name() method
    } else {
        println!(
            "Person {} is {} years old and a developer",
            married_p.full_name(), married_p.age
        );
    }
}

// decided to skip tuple example for now

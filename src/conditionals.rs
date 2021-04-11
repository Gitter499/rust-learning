// Checks condition and evalutes - ifs, whiles, etc

pub fn run() {
    let age = 22;
    let check_id: bool = false;
    let regular_bloke: bool = true;

    if age >= 21 && check_id || regular_bloke {
        println!("Bartender: What do you want to drink?");
    } else if age < 21 && check_id || regular_bloke {
        println!("Bartender: I am gonna call the police!");
    } else {
        println!("Bartender: ID please")
    }

    // Short if

    let is_of_age = if age >= 21 { true } else { false };

    println!("Is old enough? {}", is_of_age);

    // While loops

    let mut finished_task: bool = false;
    let mut num = 3;

    while !finished_task {
        num += 2;

        if num >= 5 {
            finished_task = true;
            println!("Finished task, number is {}", num);
        }
    }
}

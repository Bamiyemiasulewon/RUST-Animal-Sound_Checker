use std::io;
//Animal sound checker
fn main() {
    println!("Animal Sound Checker");

    loop {
        println!("Enter the name of an animal (type 'exit' to quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input == "exit" {
            println!("Exiting the Animal Sound Checker. Goodbye!");
            break;
        }

        match check_animal_sound(&input) {
            Some(sound) => println!("The {} says: {}", input, sound),
            None => println!("Sorry, I don't know the sound of a {}", input),
        }
    }
}

fn check_animal_sound(animal: &str) -> Option<&'static str> {
    match animal {
        "dog" => Some("Woof!"),
        "cat" => Some("Meow!"),
        "cow" => Some("Moo!"),
        "duck" => Some("Quack!"),
        _ => None,
    }
}

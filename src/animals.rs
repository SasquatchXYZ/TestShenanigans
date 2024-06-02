use std::fmt;
use std::fmt::Formatter;

struct Animal {
    name: String,
}

struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}

trait DogLike {
    fn bark(&self);
    fn run(&self);
}

impl DogLike for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }

    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

pub fn test_animals() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("{mr_mantle}");

    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();
}

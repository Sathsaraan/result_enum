#[derive(Debug)]
struct Adult {
    name: String,
    age: u8
}

impl Adult {
    fn new(&self) -> Result<&Adult, String> {
        if self.age >= 21 {
            Ok(self)
        } else {
            Err("Not an Adult!".to_owned())
        }
    }
}

fn print_value(value: Result<&Adult, String>) {
    match value {
        Ok(adult) => println!("{:?}", adult),
        Err(e) => println!("{:?}", e)
    }
}

fn main() {
    let people = vec![
        Adult {
            name: "Sam".to_owned(),
            age: 20
        },
        Adult {
            name: "Gwen".to_owned(),
            age: 25
        }
    ];

    for person in people {
        let value = Adult::new(&person);
        print_value(value);
    }
}
/*
struct Adult {
    age: u8,
    name: String
}
impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

fn main() {
    let child = Adult::new(15, "sam");
    let adult = Adult::new(30, "Anne");

    match child {
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        Err(e) => println!("{e}")
    }

    match adult {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(e) => println!("{e}")
    }
}
*/
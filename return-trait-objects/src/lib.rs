pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        format!("Woof")
    }
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        format!("Beep boop")
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => {
            // Return a Dog instance here
            Box::new(Dog {
                name: "Rex".to_string(),
                breed: "Labrador".to_string(),
            })
        }
        "robot" => {
            // Return a Robot instance here
            Box::new(Robot {
                model: "R2-D2".to_string(),
                purpose: "assistance".to_string(),
            })
        }
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}

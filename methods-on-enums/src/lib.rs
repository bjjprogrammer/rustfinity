pub enum VehicleStatus {
    // Define the VehicleStatus variants here
    Parked,
    Driving { speed: u32 },
    BrokenDown(String),
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        // Your code here...
        match &self {
            VehicleStatus::Parked | VehicleStatus::Driving { speed: _ } => true,
            VehicleStatus::BrokenDown(_) => false,
        }
    }

    pub fn description(&self) -> String {
        // Your code here...
        match &self {
            VehicleStatus::Parked => format!("The vehicle is parked."),
            VehicleStatus::Driving { speed } => {
                format!("The vehicle is driving at {} km/h.", speed)
            }
            VehicleStatus::BrokenDown(description) => {
                format!("The vehicle is broken down: {}.", description)
            }
        }
    }
}

// Example use case
pub fn main() {
    let parked = VehicleStatus::Parked;
    assert!(parked.is_operational());
    assert_eq!(parked.description(), "The vehicle is parked.");

    let driving = VehicleStatus::Driving { speed: 80 };
    assert!(driving.is_operational());
    assert_eq!(driving.description(), "The vehicle is driving at 80 km/h.");

    let broken_down = VehicleStatus::BrokenDown("Flat tire".to_string());
    assert!(!broken_down.is_operational());
    assert_eq!(
        broken_down.description(),
        "The vehicle is broken down: Flat tire."
    );
}

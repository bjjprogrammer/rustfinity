pub enum TrafficLight {
    // Define enum variants here
    Red,
    Green,
    Yellow,
}

pub fn light_action(light: &TrafficLight) -> &'static str {
    // Your code here...
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Green => "Go",
        TrafficLight::Yellow => "Caution",
    }
}

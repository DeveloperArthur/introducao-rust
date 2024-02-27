enum TrafficLight {
    Red, 
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green; //não incluímos Green na declaração use
}

// Fazer use TrafficLight::*; também é possível
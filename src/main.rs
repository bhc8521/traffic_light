pub trait Period {
    fn time_last(&self) -> i32;
}

enum TrafficLight {
    Red,
    Green,
    Yellow 
}

impl Period for TrafficLight {
    fn time_last<'a>(&self) -> i32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 70,
            TrafficLight::Yellow => 80,
        }
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    let time = light.time_last();
    println!("{}", time);
}

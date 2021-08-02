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

fn sum(list: &[u32]) -> Option<u32> {
    let mut sum:u32 = 0;
    for item in list.iter() {
        let res = sum.checked_add(*item);
        sum = match res {
            Some(v) => v,
            None => return None
        };
    };
    Some(sum)
}

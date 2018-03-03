use std::thread;
use std::time::Duration;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    caluculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(caluculation: T) -> Cacher<T> {
        Cacher {
            caluculation: caluculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.caluculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("caluculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // failed test
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn capture_environment() {
        let x = 4;

        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));
    }

    // compile error
    // #[test]
    // fn move_environment() {
    //     let x = vec![1, 2, 3];
    //
    //     let equal_to_x = move |z| z == x;
    //
    //     println!("can't use x here: {:?}", x);
    //
    //     let y = vec![1, 2, 3];
    //
    //     assert!(equal_to_x(y));
    // }
}

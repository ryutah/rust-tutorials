pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger: messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentave_of_max = self.value as f64 / self.max as f64;

        if 0.75 <= percentave_of_max && percentave_of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if 0.9 <= percentave_of_max && percentave_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentave_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        send_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.send_message.borrow_mut().push(String::from(message));
        }
    }

    struct Foo {
        foo: RefCell<Vec<String>>,
    }

    impl Foo {
        fn new() -> Foo {
            Foo {
                foo: RefCell::new(vec![]),
            }
        }

        fn append(&self, s: &str) {
            self.foo.borrow_mut().push(String::from(s));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.send_message.borrow().len(), 1);
    }

    #[test]
    fn test_foo() {
        let f = Foo::new();
        f.append("FOO");
        assert_eq!(f.foo.borrow().len(), 1);
    }
}

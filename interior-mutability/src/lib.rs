// using unsafe code to mutate
// the interior of immutably borrowed objects.

// NB: RefCell<T> is only for use
// in SINGLE-THREADED scenarios.


// Here we have a LimitTracker that takes
// a generic messenger which implements
// message-sending functionality.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { 
            messenger, 
            value: 0, 
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// we may want to test that the message-sending
// is working correctly by tracking the sent messages,
// which is normally handled by the messenger.

// We can run the test using a mock object
// which exposes a method to mutate its state.
// RefCell lets us do this without changing
// the trait signature to take a mutable ref.
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { 
                sent_messages: RefCell::new(vec![]), 
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut() is usable here,
            // despite self being immutably borrowed,
            // because we made sent_messages a RefCell.
            self.sent_messages.borrow_mut().push(String::from(message));

            // // The code below would cause a panic,
            // // as the runtime will catch two mutable
            // // borrows, which violates the borrow rules.
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // borrow() must be used to get an 
        // immutable reference to the vector.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
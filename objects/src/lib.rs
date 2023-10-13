// demo OOP features in Rust

// A struct is a kind of object.
// Its data is not pub, only mutable through methods
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// every time the list is updated,
// the average is recomputed too.
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Inheritance is not a Rust feature.
// I.e. you can't define a struct that inherits
// the parent struct's fields and
// method implementations without a macro.

// However, you can use default trait method
// implementations to reuse an implementation
// between types.
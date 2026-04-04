pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {

    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self {
            name,
            age,
            weight
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight
    }
}

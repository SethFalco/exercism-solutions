use std::{collections::{BTreeMap}};

#[allow(clippy::new_without_default)]
pub struct School {
    roster: BTreeMap<u32, Vec<&'static str>>
}

impl School {
    pub fn new() -> School {
        Self {
            roster: BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &'static str) {
        let students = self.roster.entry(grade).or_insert(Vec::new());

        let index = students.binary_search(&student).unwrap_or_else(|e| e);
        students.insert(index, student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let opt_students = self.roster.get(&grade);

        if opt_students.is_none() {
            return Vec::new();
        }

        opt_students.unwrap().iter().map(|&x| x.to_owned()).collect()
    }
}

use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Gender {
    Unspecified,
    Male,
    Female,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    #[allow(dead_code)]
    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let user = serde_json::from_str(&data)?;
        Ok(user)
    }

    #[allow(dead_code)]
    pub fn persist(&self, filename: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(filename)?;
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;

        Ok(data.len())
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("Unknown Name".into(), 0, Gender::Unspecified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let filename = "./fixtures/user";

        let user1 = User::new("changfeng lou".into(), 20, Gender::Male);
        user1.persist(filename).unwrap();

        let user2 = User::load(filename).unwrap();

        assert_eq!(user1, user2);
        println!("{:#?}", user2);
    }
}

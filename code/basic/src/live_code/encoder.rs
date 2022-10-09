use anyhow::{Ok, Result};

trait Encode {
    fn encode(&self) -> Result<Vec<u8>>;
}

struct Event<Id, Data>
where
    Id: Encode,
    Data: Encode,
{
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data>
where
    Id: Encode,
    Data: Encode,
{
    #[allow(dead_code)]
    pub fn new(id: Id, data: Data) -> Self {
        Self { id, data }
    }

    #[allow(dead_code)]
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut id = self.id.encode()?;
        let mut data = self.data.encode()?;
        id.append(&mut data);
        Ok(id)
    }
}

impl Encode for u64 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.to_be_bytes().to_vec())
    }
}

impl Encode for String {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let event = Event::new(1234_u64, "hello world".to_string());
        let encode = event.encode().unwrap();
        println!("{:#?}", encode);
    }
}

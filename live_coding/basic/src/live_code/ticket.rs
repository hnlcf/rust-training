use tokio::sync::{Semaphore, SemaphorePermit};

pub struct Museum {
    remaining_permits: Semaphore,
}

impl Museum {
    pub fn new(permits: usize) -> Self {
        Self {
            remaining_permits: Semaphore::new(permits),
        }
    }

    pub fn get_ticket(&self) -> Option<Ticket<'_>> {
        match self.remaining_permits.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn tickets(&self) -> usize {
        self.remaining_permits.available_permits()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("ticket freed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_museum() {
        let museum = Museum::new(50);
        let ticket = museum.get_ticket().unwrap();
        assert_eq!(museum.tickets(), 49);

        let _tickets: Vec<Ticket> = (0..museum.tickets())
            .map(|_| museum.get_ticket().unwrap())
            .collect();
        assert_eq!(museum.tickets(), 0);

        assert!(museum.get_ticket().is_none());

        drop(ticket);
        {
            let ticket = museum.get_ticket().unwrap();
            println!("got a ticket {:#?}", ticket);
        }
        println!("---------------------");
        assert_eq!(museum.tickets(), 1);
    }
}

use std::marker::PhantomData;

// type state pattern
pub struct InProgress;
pub struct Committed;
pub struct RolledBack;

pub struct Transaction<State> {
    id: u32,
    state: PhantomData<State>,
}

impl Transaction<InProgress> {
    pub fn new(id: u32) -> Self {
        Transaction {
            id,
            state: PhantomData,
        }
    }
    pub fn commit(self) -> Transaction<Committed> {
        println!("Transaction {} committed.", self.id);
        Transaction {
            id: self.id,
            state: PhantomData,
        }
    }
    fn rollback(self) -> Transaction<RolledBack> {
        println!("Transaction {} rolled back.", self.id);
        Transaction {
            id: self.id,
            state: PhantomData,
        }
    }
}

fn process_in_progress_transaction(tx: &Transaction<InProgress>) {
    println!("Processing transaction {} which is in progress.", tx.id);
}

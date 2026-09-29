//! Alliance Management for Guild Wars
use soroban_sdk::{contracttype, Address, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Alliance {
    pub id: u32,
    pub leader: Address,
    pub members: Vec<Address>,
}

pub fn create_alliance(leader: &Address) -> Alliance {
    Alliance {
        id: 0,
        leader: leader.clone(),
        members: Vec::new(&soroban_sdk::Env::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_create_alliance() {
        let env = Env::default();
        let leader = Address::generate(&env);
        let alliance = create_alliance(&leader);
        assert_eq!(alliance.leader, leader);
    }
}

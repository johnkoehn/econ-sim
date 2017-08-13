pub mod village_manager;

use self::village_manager::*;
use village::*;
use village::resource::*;
use village_mind::*;
use village_mind::trade_request::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PriceDirection {
    Upward,
    Downward,
    Equilibrium,
}

pub struct Simulation {
    village_managers: Vec<VillageManager>,
    pub prices: HashMap<ResourceType, u32>,
    pub price_directions: HashMap<ResourceType, PriceDirection>,
}

impl Simulation {
    pub fn new() -> Simulation {

        let mut simulation = Simulation {
            village_managers: vec!(),
            prices: HashMap::new(),
            price_directions: HashMap::new(),
        };

        //for each resource (other than gold), put the starting price at one gold
        for resource_type in ResourceType::iterator() {
            if *resource_type != ResourceType::Gold {
                simulation.prices.insert(*resource_type, 1);
                simulation.price_directions.insert(*resource_type, PriceDirection::Equilibrium);
            }
        }
        simulation
    }

    pub fn add_village(&mut self, village: Village) {
        let villageRC = Rc::new(RefCell::new(village));
        let village_manager = VillageManager {
            village: villageRC.clone(),
            village_mind: VillageMind::new(villageRC.clone()),
        };

        self.village_managers.push(village_manager);
    }

    pub fn village_managers(&self) -> &Vec<VillageManager> {
        &self.village_managers
    }

    pub fn simulate(&mut self) {
        // update the villages and village minds with the new information
        for vm in self.village_managers.iter_mut() {
            vm.village.borrow_mut().simulate();
            vm.village_mind.manage_village();
        }

        // do the trading phase until no more trade request are given
        let mut trading = true;
        let mut trade_requests : Vec<TradeRequest> = Vec::new();

        while trading {
            trade_requests.clear();

            for mut vm in self.village_managers.iter_mut() {
                trade_requests.append(&mut vm.village_mind.trade());
            }

            if trade_requests.len() == 0 {
                trading = false;
            }
        }

        // update village minds
        for vm in self.village_managers.iter_mut() {
            vm.village_mind.manage_village();
        }
    }

    pub fn handleTrades(&mut self, trade_requests: &mut Vec<TradeRequest>) {
        // get the trade request for each resource type
        for resource_type in ResourceType::iterator() {
            for trade_request_of_type in trade_requests.iter_mut().filter(|t| t.resource_type == *resource_type) {
                trade_request_of_type.success = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use village::*;
    use village_mind::*;
    use simulation::*;
    use village::resource::*;

    fn default_village() -> Village {
        Village::new(|w: &worker::Worker| false)
    }

    #[test]
    fn add_village() {
        let mut simulation = Simulation::new();
        simulation.add_village(default_village());

        assert_eq!(1, simulation.village_managers().len());
    }

    #[test]
    fn initial_resource_prices() {
        let mut simulation = Simulation::new();

        assert_eq!(1, *simulation.prices.get(&ResourceType::Food).unwrap());
    }

    #[test]
    fn initial_price_directions() {
        let mut simulation = Simulation::new();

        assert_eq!(PriceDirection::Equilibrium, *simulation.price_directions.get(&ResourceType::Food).unwrap());
    }
}
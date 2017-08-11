pub mod village_manager;

use self::village_manager::*;
use village::*;
use village_mind::*;
use village_mind::trade_request::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Simulation {
    village_managers: Vec<VillageManager>,
}

impl Simulation {
    pub fn new() -> Simulation {

        Simulation {
            village_managers: vec!(),
        }
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
}

#[cfg(test)]
mod tests {
    use village::*;
    use village_mind::*;
    use simulation::*;

    fn default_village() -> Village {
        Village::new(|w: &worker::Worker| false)
    }

    #[test]
    fn add_village() {
        let mut simulation = Simulation::new();
        simulation.add_village(default_village());

        assert_eq!(1, simulation.village_managers().len());
    }
}
pub mod village_manager;

use self::village_manager::*;
use village::*;
use village_mind::*;
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
}

#[cfg(test)]
mod tests {
    use village::*;
    use village_mind::*;
    use simulation::*;

    #[test]
    fn add_village() {
        let mut simulation = Simulation::new();
        let v1 = Village::new();
        simulation.add_village(v1);

        assert_eq!(1, simulation.village_managers().len());
    }
}
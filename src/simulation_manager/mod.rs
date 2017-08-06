use village_manager::VillageManager;
use village_manager::village::*;
use village_manager::village_mind::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SimulationManager {
    village_managers: Vec<VillageManager>,
}

impl SimulationManager {
    pub fn new(number_of_villages: u32) -> SimulationManager {

        let mut simulation_manager = SimulationManager {
            village_managers: vec!(),
        };

        //create the villagemanagers
        for x in 0..number_of_villages {
            let new_village = Rc::new(RefCell::new(Village::new()));

            let village_manager = VillageManager {
                village: new_village.clone(),
                village_mind: VillageMind::new(new_village.clone()),
            };

            simulation_manager.village_managers.push(village_manager);
        }
        simulation_manager
    }

    pub fn village_managers(&self) -> &Vec<VillageManager> {
        &self.village_managers
    }
}

#[cfg(test)]
mod tests {
    use village_manager::VillageManager;
    use village_manager::village::*;
    use village_manager::village_mind::*;
    use simulation_manager::*;

    #[test]
    fn create_village_managers() {
        let x = SimulationManager::new(10);
        assert_eq!(10, x.village_managers().len());
    }
}
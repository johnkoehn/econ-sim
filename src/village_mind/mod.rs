pub mod trade_request;

use village::*;
use village_mind::trade_request::*;

pub struct VillageMind {
    village: VillageRef,
}

impl VillageMind {
    pub fn new(village: VillageRef) -> VillageMind {
        VillageMind {
            village: village,
        }
    }

    /// Decides what to buy and sell
    /// Returns a vector of trade requests
    pub fn trade(&mut self) -> Vec<TradeRequest> {
        vec!()
    }

    /// Manages the village (e.g. prioritizing resources)
    pub fn manage_village(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use village::*;
    use village::worker::*;
    use village_mind::*;
    use simulation::simulation_settings::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn default_village_ref() -> Rc<RefCell<Village>> {
        Rc::new(RefCell::new(Village::new(|w: &Worker| false, RefCell::new(SimulationSettings::new("test_files/settings.txt")))))
    }

    #[test]
    fn trade_test() {
        let v1 = default_village_ref();
        let mut mind1 = VillageMind::new(v1);

        assert_eq!(0, mind1.trade().len());
    }
}
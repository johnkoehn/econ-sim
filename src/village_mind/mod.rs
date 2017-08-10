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

    //The village mind decides what it wants to buy and sell
    //The function returns a vector of trade requests
    pub fn trade(&mut self) -> Vec<TradeRequest> {
        vec!()
    }

    //The village mind in will manage the village in this function call
    //This involves prioritizing resources, reassigning workers, general village management
    //and deciding what resources it wants to sell and buy
    pub fn manage_village(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use village::*;
    use village::worker::*;
    use village_mind::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn default_village_ref() -> Rc<RefCell<Village>> {
        Rc::new(RefCell::new(Village::new(|w: &Worker| false)))
    }

    #[test]
    fn trade_test() {
        let v1 = default_village_ref();
        let mut mind1 = VillageMind::new(v1);
        assert_eq!(0, mind1.trade().len());
    }
}
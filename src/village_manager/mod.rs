pub mod village;
pub mod village_mind;

use village_manager::village::*;
use village_manager::village_mind::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct VillageManager {
    pub village: Rc<RefCell<Village>>,
    pub village_mind: VillageMind,
}



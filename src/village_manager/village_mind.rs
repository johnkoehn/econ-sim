use village_manager::village::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct VillageMind {
    village: Rc<RefCell<Village>>,
}

impl VillageMind {
    pub fn new(village: Rc<RefCell<Village>>) -> VillageMind {
        VillageMind {
            village: village,
        }
    }
}
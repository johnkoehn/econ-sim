use village::*;

pub struct VillageMind {
    village: VillageRef,
}

impl VillageMind {
    pub fn new(village: VillageRef) -> VillageMind {
        VillageMind {
            village: village,
        }
    }
}
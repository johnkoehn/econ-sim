pub mod village;
pub mod village_mind;

use village_manager::village_mind::*;
use village_manager::village::*;

pub struct VillageManager {
    pub village: VillageRef,
    pub village_mind: VillageMind,
}



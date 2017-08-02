use std::slice::Iter;
use self::ResourceType::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Gold,
    Food,
    Wood,
    Stone,
}

impl ResourceType {
    pub fn iterator() -> Iter<'static, ResourceType> {
        static RESOURCE_TYPES: [ResourceType; 4] = [Gold, Food, Wood, Stone];
        RESOURCE_TYPES.into_iter()
    }
}

pub struct Resource {
    pub resource_type : ResourceType,
    pub worker_count : u32,
    pub resource_id : u32,
}

impl Resource {
    pub fn collect(&self) -> u32 {
        self.worker_count
    }
}
#[derive(PartialEq)]
pub enum ResourceType {
    Gold,
    Food,
    Wood,
    Stone,
}

pub struct Resource {
    pub resource_type : ResourceType,
    pub worker_count : u32,
}
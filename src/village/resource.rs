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

pub type ResourceId = u32;
pub type CollectResource = fn(worker_power : u32) -> f64;

pub struct Resource {
    pub resource_type : ResourceType,
    pub resource_id : ResourceId,
    pub collect_resource : CollectResource,
}

impl Resource {
    pub fn new(resource_type : ResourceType, resource_id : ResourceId, collect_resource : CollectResource) -> Resource {
        Resource {
            resource_type: resource_type,
            resource_id: resource_id,
            collect_resource: collect_resource,
        }
    }
}
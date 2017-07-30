pub mod resource;
use resource::*;

pub struct Village {
    pub worker_count : u32,
    resources : Vec<Resource>,
}

impl Village {
    pub fn new(worker_count: u32) -> Village {
        Village {
            worker_count: worker_count,
            resources: vec!(),
        }
    }

    pub fn add_resource(&mut self, resource_type: ResourceType) {
        for resource in self.resources.iter() {
            if resource.resource_type == resource_type {
                return;
            }
        }

        self.resources.push(Resource {
            resource_type: resource_type,
            worker_count: 0
        });
    }

    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }
}

#[test]
fn disallow_adding_existing_resource() {
    let mut v = Village::new(1);
    v.add_resource(ResourceType::Gold);
    v.add_resource(ResourceType::Gold);

    assert_eq!(1, v.resources().len());
}
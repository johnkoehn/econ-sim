pub mod resource;
use resource::*;
use std::collections::HashMap;

pub struct Village {
    pub worker_count : u32,
    resources : Vec<Resource>,
    pub stockpile : HashMap<ResourceType, u32>,
}

impl Village {
    pub fn new(worker_count: u32) -> Village {
        let mut village = Village {
            worker_count: worker_count,
            resources: vec!(),
            stockpile: HashMap::new(),
        };

        //add each resource type to the stockpile
        for resource_type in ResourceType::iterator() {
            village.stockpile.insert(*resource_type, 0);
        }
        village
    }

    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
    
    pub fn assign_workers(&mut self, assigned_resource: &Resource, number_of_workers: u32) {
        if self.idle_worker_count() >= number_of_workers {
            for resource in self.resources.iter_mut() {
                if resource == assigned_resource
                {
                    resource.worker_count += number_of_workers;
                }
            }
        }
    }

    // //TODO: Needs to be modified. An empty collection if none
    // pub fn resource(&self, resource_type: ResourceType) -> Option<&Resource> {
    //     self.resources.iter().find(|r| r.resource_type == resource_type)
    // }

    //TODO: set of resource types
    pub fn all_resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    pub fn idle_worker_count(&self) -> u32 {
        let assigned_count : u32 = self.resources.iter()
            .map(|r| r.worker_count)
            .sum();

        self.worker_count - assigned_count
    }

    pub fn collect_resources(&mut self) {
        for resource in self.resources.iter_mut() {
            //update the resource count
            *self.stockpile.get_mut(&resource.resource_type).unwrap() += resource.collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use village::*;

    #[test]
    fn disallow_adding_existing_resource() {
        let mut v = Village::new(1);
        v.add_resource(Resource {
            resource_type: ResourceType::Gold,
            worker_count: 0,
        });
        v.add_resource(Resource {
            resource_type: ResourceType::Gold,
            worker_count: 0,
        });

        assert_eq!(2, v.all_resources().len());
    }

    // #[test]
    // fn assign_workers_to_resource() {
    //     let mut v = Village::new(4);
    //     v.add_resource(ResourceType::Wood);
    //     v.assign_workers(ResourceType::Wood, 1);

    //     assert_eq!(1, v.resource(ResourceType::Wood).unwrap().worker_count);
    // }

    // #[test]
    // fn assign_workers_to_resource_up_to_limit() {
    //     let mut v = Village::new(2);
    //     v.add_resource(ResourceType::Stone);
    //     v.assign_workers(ResourceType::Stone, 1);
    //     v.assign_workers(ResourceType::Stone, 1);

    //     assert_eq!(2, v.resource(ResourceType::Stone).unwrap().worker_count);
    // }

    // #[test]
    // fn assign_workers_to_different_resources() {
    //     let mut v = Village::new(4);
    //     v.add_resource(ResourceType::Wood);
    //     v.add_resource(ResourceType::Gold);
    //     v.assign_workers(ResourceType::Wood, 2);
    //     v.assign_workers(ResourceType::Gold, 2);

    //     assert_eq!(2, v.resource(ResourceType::Wood).unwrap().worker_count);
    //     assert_eq!(2, v.resource(ResourceType::Gold).unwrap().worker_count);
    // }

    // #[test]
    // fn idle_worker_count() {
    //     let v = Village::new(70);

    //     assert_eq!(70, v.idle_worker_count());
    // }

    // #[test]
    // fn idle_worker_count_after_assigning() {
    //     let mut v = Village::new(5);
    //     v.add_resource(ResourceType::Gold);
    //     v.assign_workers(ResourceType::Gold, 1);

    //     assert_eq!(4, v.idle_worker_count());
    // }

    // #[test]
    // fn stockpile() {
    //     let v = Village::new(5);
    //     let value = v.stockpile.get(&ResourceType::Food);
    //     assert_eq!(0, *value.unwrap());
    // }

    // #[test]
    // fn stockpile_collection() {
    //     let mut v = Village::new(5);
    //     v.add_resource(ResourceType::Wood);
    //     v.assign_workers(ResourceType::Wood, 5);
    //     v.collect_resources();

    //     assert_eq!(5, *v.stockpile.get(&ResourceType::Wood).unwrap());
    // }
}
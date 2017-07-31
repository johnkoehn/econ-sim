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

    pub fn assign_worker(&mut self, resource_type: ResourceType) {
        self.assign_worker_multiple(resource_type, 1);
    }

    pub fn assign_worker_multiple(&mut self, resource_type: ResourceType, number_of_workers: u32) {
        if(self.idle_worker_count() >= number_of_workers)
        {
            for resource in self.resources.iter_mut() {
                if resource.resource_type == resource_type {
                    resource.worker_count += number_of_workers;

                    return;
                }
            }
        }
    }

    pub fn resource(& self, resource_type: ResourceType) -> Option<&Resource> {
        self.resources.iter().find(|r| r.resource_type == resource_type)
    }

    pub fn resource_types(&self) -> Vec<ResourceType> {
        self.resources.iter()
            .map(|r| r.resource_type)
            .collect()
    }

    pub fn idle_worker_count(& self) -> u32 {
        let assigned_count : u32 = self.resources.iter()
            .map(|r| r.worker_count)
            .sum();

        self.worker_count - assigned_count
    }
}

#[cfg(test)]
mod tests {
    use village::*;
    use resource::*;

    #[test]
    fn disallow_adding_existing_resource() {
        let mut v = Village::new(1);
        v.add_resource(ResourceType::Gold);
        v.add_resource(ResourceType::Gold);

        assert_eq!(1, v.resource_types().len());
    }

    #[test]
    fn assign_worker_to_resource() {
        let mut v = Village::new(4);
        v.add_resource(ResourceType::Wood);
        v.assign_worker(ResourceType::Wood);

        assert_eq!(1, v.resource(ResourceType::Wood).unwrap().worker_count);

        //check that you can't assign more workers then available
        v.assign_worker(ResourceType::Wood);
        v.assign_worker(ResourceType::Wood);
        v.assign_worker(ResourceType::Wood);
        v.assign_worker(ResourceType::Wood);
        assert_eq!(4, v.resource(ResourceType::Wood).unwrap().worker_count);
    }

    #[test]
    fn assign_worker_to_resource_multiple()
    {
        let mut v = Village::new(4);
        v.add_resource(ResourceType::Wood);
        v.add_resource(ResourceType::Gold);
        
        v.assign_worker_multiple(ResourceType::Wood, 2);
        assert_eq!(2, v.resource(ResourceType::Wood).unwrap().worker_count);

        v.assign_worker_multiple(ResourceType::Wood, 3);
        assert_eq!(2, v.resource(ResourceType::Wood).unwrap().worker_count);

        v.assign_worker_multiple(ResourceType::Gold, 2);
        assert_eq!(2, v.resource(ResourceType::Gold).unwrap().worker_count);
    }

    #[test]
    fn idle_worker_count() {
        let v = Village::new(70);

        assert_eq!(70, v.idle_worker_count());
    }

    #[test]
    fn idle_worker_count_after_assigning() {
        let mut v = Village::new(5);
        v.add_resource(ResourceType::Gold);
        v.assign_worker(ResourceType::Gold);

        assert_eq!(4, v.idle_worker_count());
    }
}
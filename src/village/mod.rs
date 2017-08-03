pub mod resource;
use resource::*;
use std::collections::HashMap;

pub struct Village {
    pub worker_count : u32,
    resources : Vec<Resource>,
    pub stockpile : HashMap<ResourceType, u32>,
    number_of_resource_sites : u32,
}

impl Village {
    pub fn new(worker_count: u32) -> Village {
        let mut village = Village {
            worker_count: worker_count,
            resources: vec!(),
            stockpile: HashMap::new(),
            number_of_resource_sites: 0, 
        };

        //add each resource type to the stockpile
        for resource_type in ResourceType::iterator() {
            village.stockpile.insert(*resource_type, 0);
        }
        village
    }

    pub fn create_resource(&mut self, resource_type: ResourceType) {
        self.resources.push(Resource {
            resource_type: resource_type,
            worker_count: 0,
            resource_id: self.number_of_resource_sites,
        });
        self.number_of_resource_sites += 1;
    }
    
    pub fn assign_workers(&mut self, resource_id: u32, number_of_workers: u32) {
        if self.idle_worker_count() >= number_of_workers && resource_id < self.number_of_resource_sites {
            self.resources[resource_id as usize].worker_count += number_of_workers;
        }
    }

    pub fn remove_workers(&mut self, resource_id: u32, number_of_workers: u32) {
        if resource_id < self.number_of_resource_sites && self.resources[resource_id as usize].worker_count >= number_of_workers {
            self.resources[resource_id as usize].worker_count -= number_of_workers;
        }
    }

    pub fn resources(&self, resource_type: ResourceType) -> Vec<&Resource> {
        self.resources.iter().filter(|r| r.resource_type == resource_type).collect::<Vec<_>>()
    }

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
    fn add_resources() {
        let mut v = Village::new(1);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);

        assert_eq!(3, v.all_resources().len());
    }

    #[test]
    fn assign_workers_to_resource() {
        let mut v = Village::new(4);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);
        let mut resource_assignments: Vec<(u32, u32)> = Vec::new();
        {
            //get the list of resources and extract the ids of the resources you want to assign workers to
            let x = v.all_resources();
            resource_assignments.push((x[0].resource_id, 2));
            resource_assignments.push((x[1].resource_id, 1));
        }
        v.assign_workers(resource_assignments[0].0, resource_assignments[0].1);
        v.assign_workers(resource_assignments[1].0, resource_assignments[1].1);

        let x = v.all_resources();
        assert_eq!(2, x[0].worker_count);
        assert_eq!(1, x[1].worker_count);
        assert_eq!(0, x[2].worker_count);
    }

    #[test]
    fn remove_workers_from_resource()
    {
        let mut v = Village::new(4);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);
        v.assign_workers(0, 3);
        v.assign_workers(1, 1);
        v.remove_workers(0, 2);
        v.remove_workers(1, 1);

        let x = v.all_resources();
        assert_eq!(1, x[0].worker_count);
        assert_eq!(0, x[1].worker_count);
    }

    #[test]
    fn assign_workers_to_resource_up_to_limit() {
        let mut v = Village::new(2);
        v.create_resource(ResourceType::Stone);
        v.assign_workers(0, 1);
        v.assign_workers(0, 1);
        v.assign_workers(0, 1);

        assert_eq!(2, v.all_resources()[0].worker_count);
    }

    #[test]
    fn idle_worker_count() {
        let v = Village::new(70);

        assert_eq!(70, v.idle_worker_count());
    }

    #[test]
    fn idle_worker_count_after_assigning() {
        let mut v = Village::new(5);
        v.create_resource(ResourceType::Gold);
        v.assign_workers(0, 2);

        assert_eq!(3, v.idle_worker_count());
    }

    #[test]
    fn stockpile() {
        let v = Village::new(5);
        let value = v.stockpile.get(&ResourceType::Food);
        assert_eq!(0, *value.unwrap());
    }

    #[test]
    fn stockpile_collection() {
        let mut v = Village::new(5);
        v.create_resource(ResourceType::Wood);
        v.assign_workers(0, 5);
        v.collect_resources();

        assert_eq!(5, *v.stockpile.get(&ResourceType::Wood).unwrap());
    }

    #[test]
    fn get_resources_by_type()
    {
        let mut v = Village::new(5);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);
        v.create_resource(ResourceType::Wood);
        v.create_resource(ResourceType::Wood);
        v.create_resource(ResourceType::Stone);

        assert_eq!(2, v.resources(ResourceType::Gold).len());
        assert_eq!(3, v.resources(ResourceType::Wood).len());
        assert_eq!(1, v.resources(ResourceType::Stone).len());
        assert_eq!(0, v.resources(ResourceType::Food).len());
    }
}
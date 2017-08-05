pub mod resource;
use resource::*;
use std::collections::HashMap;

pub struct Village {
    pub worker_count : u32,
    pub stockpile : HashMap<ResourceType, u32>,

    resources : Vec<Resource>,
    resource_id_counter: u32,
}

impl Village {
    pub fn new(worker_count: u32) -> Village {
        let mut village = Village {
            worker_count: worker_count,
            resources: vec!(),
            stockpile: HashMap::new(),
            resource_id_counter: 0,
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
            resource_id: self.resource_id_counter,
        });
        self.resource_id_counter += 1;
    }
    
    pub fn assign_workers(&mut self, resource_id: u32, number_of_workers: u32) {
        if self.idle_worker_count() >= number_of_workers {
            if let Some(r) = self.resources.iter_mut().find(|r| r.resource_id == resource_id) {
                r.worker_count += number_of_workers;
            }
        }
    }

    pub fn remove_workers(&mut self, resource_id: u32, number_of_workers: u32) {
        if let Some(r) = self.resources.iter_mut().find(|r| r.resource_id == resource_id) {
            if r.worker_count >= number_of_workers {
                r.worker_count -= number_of_workers
            }
        }
    }

    pub fn resources_of_type(&self, resource_type: ResourceType) -> Vec<&Resource> {
        self.resources.iter()
            .filter(|r| r.resource_type == resource_type)
            .collect()
    }

    pub fn resources(&self) -> &Vec<Resource> {
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
            *self.stockpile.get_mut(&resource.resource_type).unwrap() += resource.collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use village::*;

    #[test]
    fn create_resources() {
        let mut v = Village::new(1);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);

        assert_eq!(3, v.resources().len());
    }

    #[test]
    fn assign_workers_to_resource() {
        let mut v = Village::new(4);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);

        v.assign_workers(1, 2);

        let x = v.resources();
        assert_eq!(2, x[1].worker_count);
    }

    #[test]
    fn remove_workers_from_resource() {
        let mut v = Village::new(4);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);
        v.assign_workers(0, 3);
        v.assign_workers(1, 1);
        v.remove_workers(0, 2);
        v.remove_workers(1, 1);

        let x = v.resources();
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

        assert_eq!(2, v.resources()[0].worker_count);
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
    fn get_resources_by_type() {
        let mut v = Village::new(5);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);
        v.create_resource(ResourceType::Wood);

        assert_eq!(1, v.resources_of_type(ResourceType::Gold).len());
        assert_eq!(2, v.resources_of_type(ResourceType::Wood).len());
        assert_eq!(0, v.resources_of_type(ResourceType::Stone).len());
        assert_eq!(0, v.resources_of_type(ResourceType::Food).len());
    }
}
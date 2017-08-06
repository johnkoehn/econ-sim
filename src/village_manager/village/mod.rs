pub mod resource;
pub mod worker;

use village_manager::village::resource::*;
use village_manager::village::worker::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type VillageRef = Rc<RefCell<Village>>;

pub struct Village {
    pub stockpile: HashMap<ResourceType, u32>,

    workers: Vec<Worker>,
    worker_id_counter: u32,

    resources: Vec<Resource>,
    resource_id_counter: u32,
}

impl Village {
    pub fn new() -> Village {
        let mut village = Village {
            stockpile: HashMap::new(),
            workers: vec!(),
            worker_id_counter: 0,
            resources: vec!(),
            resource_id_counter: 0,
        };

        //add each resource type to the stockpile
        for resource_type in ResourceType::iterator() {
            village.stockpile.insert(*resource_type, 0);
        }

        village
    }

    /// Adds a new Worker instance to this Village.
    /// Worker id values start at 1 and auto increment in subsequent invocations
    /// Returns the id of the Worker instance
    pub fn create_worker(&mut self) -> WorkerId {
        self.worker_id_counter += 1;

        self.workers.push(Worker {
            worker_id: self.worker_id_counter,
            assigned_resource: 0,
        });

        self.worker_id_counter
    }

    /// Adds a new Resource instance to this Village.
    /// Resource id values start at 1 and auto increment in subsequent invocations
    /// Returns the id of the Resource instance
    pub fn create_resource(&mut self, resource_type: ResourceType) -> ResourceId {
        self.resource_id_counter += 1;

        self.resources.push(Resource {
            resource_type: resource_type,
            resource_id: self.resource_id_counter,
        });

        self.resource_id_counter
    }

    /// Assigns a worker to an object
    /// Passing in a resource id of value 0 will cause the worker to be ideal
    pub fn assign_worker(&mut self, worker_id: WorkerId, resource_id: ResourceId) {
        if let Some(w) = self.workers.iter_mut().find(|w| w.worker_id == worker_id) {
            if resource_id == 0 {
                w.assigned_resource = 0;
            } else if let Some(r) = self.resources.iter().find(|r| r.resource_id == resource_id) {
                w.assigned_resource = r.resource_id;
            }
        }
    }

    pub fn resource(&self, resource_id: ResourceId) -> Option<&Resource> {
        self.resources.iter().find(|r| r.resource_id == resource_id)
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
        self.workers.iter().filter(|w| w.assigned_resource == 0).count() as u32
    }

    pub fn collect_resources(&mut self) {
        for worker in self.workers.iter() {
            let resource_type : ResourceType;

            if let Some(r) = self.resource(worker.assigned_resource) {
                resource_type = r.resource_type;
            } else {
                continue;
            }

            *self.stockpile.get_mut(&resource_type).unwrap() += 1;
        }
    }

    pub fn workers_on_resource(&self, resource_id: u32) -> Vec<&Worker> {
        self.workers.iter().filter(|w| w.assigned_resource == resource_id).collect()
    }
}

#[cfg(test)]
mod tests {
    use village_manager::village::*;

    #[test]
    fn create_resources() {
        let mut v = Village::new();
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);

        assert_eq!(3, v.resources().len());
    }

    #[test]
    fn assign_worker_to_resource() {
        let mut v = Village::new();
        let w1 = v.create_worker();
        let r1 = v.create_resource(ResourceType::Gold);

        v.assign_worker(w1, r1);

        assert_eq!(1, v.workers_on_resource(r1).len());
        assert_eq!(0, v.idle_worker_count());
    }

    #[test]
    fn unassign_worker_from_resource() {
        let mut v = Village::new();
        let w1 = v.create_worker();
        let r1 = v.create_resource(ResourceType::Gold);

        v.assign_worker(w1, r1);
        v.assign_worker(w1, 0);

        assert_eq!(0, v.workers_on_resource(r1).len());
        assert_eq!(1, v.idle_worker_count());
    }

    #[test]
    fn stockpile() {
        let v = Village::new();
        let value = v.stockpile.get(&ResourceType::Food);
        assert_eq!(0, *value.unwrap());
    }

    #[test]
    fn stockpile_collection() {
        let mut v = Village::new();
        let r1 = v.create_resource(ResourceType::Wood);
        let w1 = v.create_worker();
        let w2 = v.create_worker();

        v.assign_worker(w1, r1);
        v.assign_worker(w2, r1);
        v.collect_resources();

        assert_eq!(2, *v.stockpile.get(&ResourceType::Wood).unwrap());
    }

    #[test]
    fn get_resources_by_type() {
        let mut v = Village::new();
        v.create_resource(ResourceType::Gold);
        v.create_resource(ResourceType::Wood);
        v.create_resource(ResourceType::Wood);

        assert_eq!(1, v.resources_of_type(ResourceType::Gold).len());
        assert_eq!(2, v.resources_of_type(ResourceType::Wood).len());
        assert_eq!(0, v.resources_of_type(ResourceType::Stone).len());
        assert_eq!(0, v.resources_of_type(ResourceType::Food).len());
    }
}
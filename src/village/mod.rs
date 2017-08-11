pub mod resource;
pub mod worker;

use village::resource::*;
use village::worker::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type VillageRef = Rc<RefCell<Village>>;

pub type CheckForWorkerDeath = fn(&Worker) -> bool;

pub struct Village {
    pub stockpile: HashMap<ResourceType, f64>,

    workers: Vec<Worker>,
    worker_id_counter: u32,
    resources: Vec<Resource>,
    resource_id_counter: u32,

    check_for_worker_death: CheckForWorkerDeath,
}

impl Village {
    pub fn new(check_for_worker_death: CheckForWorkerDeath) -> Village {
        let mut village = Village {
            stockpile: HashMap::new(),
            workers: vec!(),
            worker_id_counter: 0,
            resources: vec!(),
            resource_id_counter: 0,
            check_for_worker_death: check_for_worker_death,
        };

        //add each resource type to the stockpile
        for resource_type in ResourceType::iterator() {
            village.stockpile.insert(*resource_type, 0 as f64);
        }

        village
    }

    /// Adds a new Worker instance to this Village.
    /// Worker id values start at 1 and auto increment in subsequent invocations
    /// Returns the id of the Worker instance
    pub fn create_worker(&mut self, power: u32) -> WorkerId {
        self.worker_id_counter += 1;
        self.workers.push(Worker::new(self.worker_id_counter, power));

        self.worker_id_counter
    }

    /// Adds a new Resource instance to this Village.
    /// Resource id values start at 1 and auto increment in subsequent invocations
    /// Returns the id of the Resource instance
    pub fn create_resource(&mut self, resource_type: ResourceType, collect_resource: CollectResource) -> ResourceId {
        self.resource_id_counter += 1;

        self.resources.push(Resource {
            resource_type: resource_type,
            resource_id: self.resource_id_counter,
            collect_resource: collect_resource,
        });

        self.resource_id_counter
    }

    /// Assigns a worker to an object
    /// Passing in a resource id of value 0 will cause the worker to be ideal
    pub fn assign_worker(&mut self, worker_id: WorkerId, resource_id: ResourceId) -> Result<(), &'static str> {
        if let Some(w) = self.workers.iter_mut().find(|w| w.worker_id == worker_id) {
            if resource_id == 0 {
                w.assigned_resource = 0;
            } else if let Some(r) = self.resources.iter().find(|r| r.resource_id == resource_id) {
                w.assigned_resource = r.resource_id;
            } else {
               return Err("Invalid Resource ID");
            }
        }
        else {
            return Err("Invalid Worker ID");
        }
        Ok(())
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

    pub fn worker(&self, worker_id: WorkerId) -> Option<&Worker> {
        self.workers.iter().find(|w| w.worker_id == worker_id)
    }

    pub fn workers_on_resource(&self, resource_id: u32) -> Vec<&Worker> {
        self.workers.iter().filter(|w| w.assigned_resource == resource_id).collect()
    }

    pub fn power_on_resource(&self, resource_id: u32) -> u32 {
        let mut power = 0;
        for worker in self.workers_on_resource(resource_id).iter() {
            power += worker.power;
        }
        power
    }

    pub fn simulate(&mut self) {
        for resource in self.resources.iter() {
            let power = self.power_on_resource(resource.resource_id);
            *self.stockpile.get_mut(&resource.resource_type).unwrap() += (resource.collect_resource)(power);
        }

        for worker in self.workers.iter_mut() {
            worker.age += 1;
            worker.is_alive = !(self.check_for_worker_death)(&worker);
        }

        // remove workers not alive
        self.workers.retain(|ref w| w.is_alive);
    }
}

#[cfg(test)]
mod tests {
    use village::*;
    use village::worker::*;

    fn default_village() -> Village {
        Village::new(|w: &Worker| false)
    }

    fn default_worker(village: &mut Village) -> WorkerId {
        village.create_worker(1)
    }

    fn default_collect_resource() -> fn(u32) -> f64 {
        move |x| x as f64
    }

    #[test]
    fn create_resources() {
        let mut v = default_village();
        v.create_resource(ResourceType::Gold, default_collect_resource());
        v.create_resource(ResourceType::Gold, default_collect_resource());
        v.create_resource(ResourceType::Wood, default_collect_resource());

        assert_eq!(3, v.resources().len());
    }

    #[test]
    fn assign_worker_to_resource() {
        let mut v = default_village();
        let w1 = default_worker(&mut v);
        let r1 = v.create_resource(ResourceType::Gold, default_collect_resource());

        v.assign_worker(w1, r1);

        assert_eq!(1, v.workers_on_resource(r1).len());
        assert_eq!(0, v.idle_worker_count());
    }

    #[test]
    fn unassign_worker_from_resource() {
        let mut v = default_village();
        let w1 = default_worker(&mut v);
        let r1 = v.create_resource(ResourceType::Gold, default_collect_resource());

        v.assign_worker(w1, r1);
        v.assign_worker(w1, 0);

        assert_eq!(0, v.workers_on_resource(r1).len());
        assert_eq!(1, v.idle_worker_count());
    }

    #[test]
    fn assign_worker_to_resource_invalid_worker_id() {
        let mut v = default_village();
        let r1 = v.create_resource(ResourceType::Gold, default_collect_resource());

        assert!(v.assign_worker(1, r1).is_err());
    }

    #[test]
    fn assign_worker_to_resource_invalid_resource_id() {
        let mut v = default_village();
        let w1 = default_worker(&mut v);

        assert!(v.assign_worker(w1, 1).is_err());
    }

    #[test]
    fn stockpile_starts_empty() {
        let v = default_village();
        let value = v.stockpile.get(&ResourceType::Food);

        assert_eq!(0, *value.unwrap() as u32);
    }

    #[test]
    fn simulate_collect_resources() {
        let mut v = default_village();
        let r1 = v.create_resource(ResourceType::Wood, default_collect_resource());
        let w1 = default_worker(&mut v);
        let w2 = default_worker(&mut v);

        v.assign_worker(w1, r1);
        v.assign_worker(w2, r1);
        v.simulate();

        assert_eq!(2, *v.stockpile.get(&ResourceType::Wood).unwrap() as u32);
    }

    #[test]
    fn simulate_increase_worker_age() {
        let mut v = default_village();
        let w1 = default_worker(&mut v);

        v.simulate();

        assert_eq!(1, v.worker(w1).unwrap().age);
    }

    #[test]
    fn simulate_increase_worker_age_multiple() {
        let mut v = default_village();
        let w1 = default_worker(&mut v);
        let w2 = default_worker(&mut v);

        v.simulate();
        v.simulate();

        assert_eq!(2, v.worker(w1).unwrap().age);
        assert_eq!(2, v.worker(w2).unwrap().age);
    }

    #[test]
    fn simulate_worker_death() {
        let mut v = Village::new(|w| w.worker_id == 1);
        let w1 = default_worker(&mut v);
        let w2 = default_worker(&mut v);

        v.simulate();

        assert!(v.worker(w1).is_none());
        assert!(v.worker(w2).is_some());
    }

    #[test]
    fn get_resources_by_type() {
        let mut v = default_village();
        v.create_resource(ResourceType::Gold, default_collect_resource());
        v.create_resource(ResourceType::Wood, default_collect_resource());
        v.create_resource(ResourceType::Wood, default_collect_resource());

        assert_eq!(1, v.resources_of_type(ResourceType::Gold).len());
        assert_eq!(2, v.resources_of_type(ResourceType::Wood).len());
        assert_eq!(0, v.resources_of_type(ResourceType::Stone).len());
        assert_eq!(0, v.resources_of_type(ResourceType::Food).len());
    }

    #[test]
    fn simulate_resource_collect() {
        let mut v = default_village();
        let r1 = v.create_resource(ResourceType::Wood, |x| 2 as f64);
        v.simulate();

        assert_eq!(2, *v.stockpile.get(&ResourceType::Wood).unwrap() as u32);
    }
}
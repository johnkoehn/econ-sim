pub type WorkerId = u32;

pub struct Worker {
    pub worker_id: WorkerId,
    pub assigned_resource: u32,
    pub age: u32,
    pub is_alive: bool,
    pub power: u32,
}

impl Worker {
    pub fn new(worker_id: WorkerId, power: u32) -> Worker {
        Worker {
            worker_id: worker_id,
            assigned_resource: 0,
            age: 0,
            is_alive: true,
            power: power,
        }
    }
}
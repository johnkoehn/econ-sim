pub type WorkerId = u32;

pub struct Worker {
    pub worker_id: WorkerId,
    pub assigned_resource: u32,
}
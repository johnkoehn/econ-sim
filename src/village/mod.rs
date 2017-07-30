pub mod resource;

pub struct Village {
    pub worker_count : u32,
    pub resources : Vec<resource::Resource>,
}
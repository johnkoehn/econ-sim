mod village;

use village::*;

fn main() {
    let v1 = Village {
        worker_count: 20,
        resources: vec!(resource::Resource { resource_type: resource::ResourceType::Gold })
    };
}
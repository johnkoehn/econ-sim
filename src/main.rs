mod village;

use village::*;

fn main() {
    let mut v1 = Village::new(20);
    v1.add_resource(resource::ResourceType::Wood);
}
extern crate embed_resource;

fn main() {
    let _ = embed_resource::compile("resources.rc", embed_resource::NONE);
}

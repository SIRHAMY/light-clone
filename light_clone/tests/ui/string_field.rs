use light_clone::LightClone;

#[derive(LightClone)]
struct BadStruct {
    name: String, // String does not implement LightClone
}

fn main() {}

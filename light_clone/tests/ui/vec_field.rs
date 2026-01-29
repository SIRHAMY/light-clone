use light_clone::LightClone;

#[derive(LightClone)]
struct BadStruct {
    items: Vec<i32>, // Vec does not implement LightClone
}

fn main() {}

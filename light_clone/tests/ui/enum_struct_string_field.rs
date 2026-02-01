use light_clone::LightClone;

#[derive(Clone, LightClone)]
enum BadEnum {
    Good { x: i32 },
    Bad { s: String }, // String does not implement LightClone
}

fn main() {}

use light_clone::LightClone;

#[derive(LightClone)]
enum BadEnum {
    Good { x: i32 },
    Bad { s: String }, // String does not implement LightClone
}

fn main() {}

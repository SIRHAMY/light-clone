use light_clone::LightClone;

// LightClone without Clone should fail - Clone is required
#[derive(LightClone)]
struct NoClone {
    value: i32,
}

fn main() {}

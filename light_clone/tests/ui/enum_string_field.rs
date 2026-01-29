use light_clone::LightClone;

#[derive(LightClone)]
enum BadEnum {
    Good(i32),
    Bad(String), // String does not implement LightClone
}

fn main() {}

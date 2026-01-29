use light_clone::LightClone;

#[derive(LightClone)]
union MyUnion {
    a: i32,
    b: f32,
}

fn main() {}

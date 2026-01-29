use light_clone::LightClone;

#[derive(LightClone)]
enum MyEnum {
    A,
    B(i32),
}

fn main() {}

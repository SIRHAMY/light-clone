use light_clone::LightClone;

#[derive(Clone, LightClone)]
struct Container<T: Clone + LightClone> {
    value: T,
}

fn main() {
    // String does not implement LightClone, so this should fail
    let _c: Container<String> = Container {
        value: String::from("hello"),
    };
}

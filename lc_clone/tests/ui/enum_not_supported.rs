use lc_clone::LcClone;

#[derive(LcClone)]
enum MyEnum {
    A,
    B(i32),
}

fn main() {}

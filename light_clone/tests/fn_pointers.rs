use light_clone::LightClone;

fn zero_args() -> i32 {
    42
}

fn one_arg(x: i32) -> i32 {
    x * 2
}

fn six_args(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    a + b + c + d + e + f
}

#[allow(clippy::too_many_arguments)]
fn twelve_args(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
    i: i32,
    j: i32,
    k: i32,
    l: i32,
) -> i32 {
    a + b + c + d + e + f + g + h + i + j + k + l
}

#[test]
fn test_fn_pointer_zero_args() {
    let f: fn() -> i32 = zero_args;
    let cloned = f.light_clone();
    assert_eq!(f(), cloned());
    assert_eq!(cloned(), 42);
}

#[test]
fn test_fn_pointer_one_arg() {
    let f: fn(i32) -> i32 = one_arg;
    let cloned = f.light_clone();
    assert_eq!(f(21), cloned(21));
    assert_eq!(cloned(5), 10);
}

#[test]
fn test_fn_pointer_six_args() {
    let f: fn(i32, i32, i32, i32, i32, i32) -> i32 = six_args;
    let cloned = f.light_clone();
    assert_eq!(f(1, 2, 3, 4, 5, 6), cloned(1, 2, 3, 4, 5, 6));
    assert_eq!(cloned(1, 2, 3, 4, 5, 6), 21);
}

#[test]
#[allow(clippy::type_complexity)]
fn test_fn_pointer_twelve_args() {
    let f: fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32 = twelve_args;
    let cloned = f.light_clone();
    assert_eq!(
        f(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        cloned(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
    );
    assert_eq!(cloned(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12), 78);
}

#[test]
fn test_fn_pointer_lc_shorthand() {
    let f: fn(i32) -> i32 = one_arg;
    let cloned = f.lc();
    assert_eq!(cloned(10), 20);
}

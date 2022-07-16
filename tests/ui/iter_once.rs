// run-rustfix
#![warn(clippy::iter_once)]
#![allow(clippy::iter_next_slice, clippy::redundant_clone)]

fn array() {
    assert_eq!([123].into_iter().next(), Some(123));
    assert_eq!([123].iter_mut().next(), Some(&mut 123));
    assert_eq!([123].iter().next(), Some(&123));
    assert_eq!(Some(123).into_iter().next(), Some(123));
    assert_eq!(Some(123).iter_mut().next(), Some(&mut 123));
    assert_eq!(Some(123).iter().next(), Some(&123));

    // Don't trigger on non-iter methods
    let _: Option<String> = Some("test".to_string()).clone();
    let _: [String; 1] = ["test".to_string()].clone();
}

macro_rules! in_macros {
    () => {
        assert_eq!([123].into_iter().next(), Some(123));
        assert_eq!([123].iter_mut().next(), Some(&mut 123));
        assert_eq!([123].iter().next(), Some(&123));
        assert_eq!(Some(123).into_iter().next(), Some(123));
        assert_eq!(Some(123).iter_mut().next(), Some(&mut 123));
        assert_eq!(Some(123).iter().next(), Some(&123));
    };
}

fn main() {
    array();
    in_macros!();
}

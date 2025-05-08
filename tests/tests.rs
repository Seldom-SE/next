use core::f32;

use next::Next;

#[test]
fn next_enum() {
    mod next_module {
        pub use next::Next;
    }

    #[repr(u8)]
    #[derive(Next, PartialEq, Debug)]
    #[next(path = next_module::Next)]
    enum Foo {
        C { first: f32, second: u8 } = 2,
        A = 0,
        B(bool),
    }

    assert_eq!(Foo::MIN, Foo::A);
    assert_eq!(Foo::A.next(), Some(Foo::B(false)));
    assert_eq!(Foo::B(false).next(), Some(Foo::B(true)));
    assert_eq!(
        Foo::B(true).next(),
        Some(Foo::C {
            first: f32::NEG_INFINITY,
            second: 0
        }),
    );
    assert_eq!(
        Foo::C {
            first: f32::NEG_INFINITY,
            second: 0
        }
        .next(),
        Some(Foo::C {
            first: f32::NEG_INFINITY,
            second: 1,
        }),
    );
    assert_eq!(
        Foo::C {
            first: f32::NEG_INFINITY,
            second: u8::MAX,
        }
        .next(),
        Some(Foo::C {
            first: f32::MIN,
            second: 0,
        }),
    );
    assert_eq!(
        Foo::C {
            first: f32::INFINITY,
            second: u8::MAX,
        }
        .next(),
        None,
    );
}

#[test]
fn next_struct() {
    #[derive(Next, PartialEq, Debug)]
    struct Foo {
        a: (),
    }

    assert_eq!(Foo::MIN, Foo { a: () });
    assert_eq!(Foo { a: () }.next(), None);
}

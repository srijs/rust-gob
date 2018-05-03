#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! test_builtin {
    ($name:ident, $go_typ:ident, $go_value:expr, $typ:ident, $value:expr) => {
        de_test!(
            $name
            go_decls "",
            go_value $go_typ $go_value,
            rust_decls {},
            validate v: $typ {
                assert_eq!(v, $value);
            }
        );
    };
}

mod bool {
    test_builtin!(test_true, bool, true, bool, true);
    test_builtin!(test_false, bool, false, bool, false);
}

mod uint {
    mod max {
        mod unsigned {

            test_builtin!(u8, uint64, u8::max_value(), u8, u8::max_value());
            test_builtin!(u16, uint64, u16::max_value(), u16, u16::max_value());
            test_builtin!(u32, uint64, u32::max_value(), u32, u32::max_value());
            test_builtin!(u64, uint64, u64::max_value(), u64, u64::max_value());
            test_builtin!(usize, uint, usize::max_value(), usize, usize::max_value());
        }

        mod signed {
            test_builtin!(i8, uint64, i8::max_value(), i8, i8::max_value());
            test_builtin!(i16, uint64, i16::max_value(), i16, i16::max_value());
            test_builtin!(i32, uint64, i32::max_value(), i32, i32::max_value());
            test_builtin!(i64, uint64, i64::max_value(), i64, i64::max_value());
            test_builtin!(isize, uint, isize::max_value(), isize, isize::max_value());
        }
    }
}

mod int {
    mod max {
        mod unsigned {
            test_builtin!(u8, int64, i8::max_value() as u8, u8, i8::max_value() as u8);
            test_builtin!(
                u16,
                int64,
                i16::max_value() as u16,
                u16,
                i16::max_value() as u16
            );
            test_builtin!(
                u32,
                int64,
                i32::max_value() as u32,
                u32,
                i32::max_value() as u32
            );
            test_builtin!(
                u64,
                int64,
                i64::max_value() as u64,
                u64,
                i64::max_value() as u64
            );
            test_builtin!(
                usize,
                int,
                isize::max_value() as usize,
                usize,
                isize::max_value() as usize
            );
        }

        mod signed {
            test_builtin!(i8, int64, i8::max_value(), i8, i8::max_value());
            test_builtin!(i16, int64, i16::max_value(), i16, i16::max_value());
            test_builtin!(i32, int64, i32::max_value(), i32, i32::max_value());
            test_builtin!(i64, int64, i64::max_value(), i64, i64::max_value());
            test_builtin!(isize, int, isize::max_value(), isize, isize::max_value());
        }
    }
}

mod float {
    const PI32: f32 = ::std::f32::consts::PI;
    const PI64: f64 = ::std::f64::consts::PI;

    test_builtin!(f32, float32, PI32, f32, PI32);
    test_builtin!(f64, float64, PI64, f64, PI64);
}

mod string {
    const GO_DATA: &str = "\"hello world\"";
    const DATA: &str = "hello world";
    test_builtin!(String, string, GO_DATA, String, DATA.to_string());
}

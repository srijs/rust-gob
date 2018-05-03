#[macro_export]
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! de_test {
    ($test_name:ident
        go_decls $go_decls:expr,
        go_value $go_typ:ident $go_value:expr,
        rust_decls { $($decls:item)* },
        validate $val_name:ident : $typ:ident { $($val_exprs:tt)* }
    ) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            use std::env;
            use gob;
            use serde::Deserialize;

            #[warn(non_snake_case)]
            {
                let code = format!(r#"
                    package main

                    import (
                        "fmt"
                        "os"
                        "encoding/gob"
                    )

                    {decls}

                    func main() {{
                        enc := gob.NewEncoder(os.Stdout)
                        err := enc.Encode(value())

                        if err != nil {{
                            fmt.Fprintf(os.Stderr, "%v", err)
                            return
                        }}
                    }}

                    func value() {typ} {{
                        return {value}
                    }}
                "#,
                    typ = stringify!($go_typ),
                    value = $go_value,
                    decls = $go_decls,
                );

                if let Ok(_) = env::var("GOB_CODE_PANIC") {
                    panic!("{}", code);
                }

                let output = ::utils::go::run(&code).expect("runs go binary");
                let stderr = ::std::string::String::from_utf8(output.stderr).unwrap();
                let stdout = output.stdout.as_slice();

                if !stderr.is_empty() {
                    panic!("{}", stderr);
                }

                let deserializer = gob::Deserializer::from_slice(stdout);

                $(#[allow(non_snake_case)] $decls)*

                let $val_name = <$typ>::deserialize(deserializer).unwrap();

                $($val_exprs)*;
            }
        }
    }
}

#[macro_export]
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! ser_test {
    ($test_name:ident
        go_decls $go_decls:expr,
        go_value $go_typ:ident $go_value:expr,
        rust_decls { $($decls:item)* },
        validate $val_name:ident : $typ:ident { $($val_exprs:tt)* }
    ) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            use std::env;
            use gob;

            #[warn(non_snake_case)]
            {
                let code = format!(r#"
                    package main

                    import (
                        "fmt"
                        "os"
                        "encoding/gob"
                    )

                    {decls}

                    func main() {{
                        var decoded = new ({typ})
                        codec := gob.NewDecoder(os.Stdin)
                        err := codec.Decode(decoded)
                        if err != nil {{
                            fmt.Fprintf(os.Stderr, "%v", err)
                            return
                        }}

                        var expected = value()

                        if *decoded != expected {{
                            fmt.Fprintf(os.Stderr, "\nGolang Expected:\n")
                            fmt.Fprintf(os.Stderr, "%+v\n", expected)
                            fmt.Fprintf(os.Stderr, "Golang Decoded:\n")
                            fmt.Fprintf(os.Stderr, "%+v\n", decoded)
                            return
                        }}

                    }}

                    func value() {typ} {{
                        return {value}
                    }}
                "#,
                    typ = stringify!($go_typ),
                    value = $go_value,
                    decls = $go_decls,
                );

                // Rust type declarations.
                $(#[allow(non_snake_case)] $decls)*

                // Create a default instance.
                let $val_name: $typ = $typ::default();
                //panic!("{:#?}", $val_name);

                // Serialize and write to a buffer.
                let mut buf = Vec::new();
                {
                    let mut stream = gob::StreamSerializer::new(&mut buf);
                    stream.serialize(&$val_name).expect("serialization works");
                }

                if let Ok(_) = env::var("GOB_CODE_PANIC") {
                    panic!("{}", code);
                }

                // Run `go`, passing in the serialized data on `stdin`
                let output = ::utils::go::run_with_input(&code, Some(buf)).expect("runs go binary");

                // Make sure there was no output and no error.
                let stderr = ::std::string::String::from_utf8(output.stderr).unwrap();
                if !stderr.is_empty() {
                    panic!("{}", stderr);
                }
                $($val_exprs)*;
            }
        }
    }
}

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const LOWER_LIMIT: usize = 16;

fn main() {
    let limit = if cfg!(feature="limit_2048") {
        2048
    } else if cfg!(feature="limit_1024") {
        1024
    } else if cfg!(feature="limit_512") {
        512
    } else if cfg!(feature="limit_256") {
        256
    } else if cfg!(feature="limit_128") {
        128
    } else {
        64
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lib.rs");
    let mut f = File::create(&dest_path).unwrap();

    let mut output = String::new();

    output.push_str(r#"
/// Unroll the given for loop
///
/// Example:
///
/// ```ignore
/// unroll! {
///   for i in 0..5 {
///     println!("Iteration {}", i);
///   }
/// }
/// ```
///
/// will expand into:
///
/// ```ignore
/// { println!("Iteration {}", 0); }
/// { println!("Iteration {}", 1); }
/// { println!("Iteration {}", 2); }
/// { println!("Iteration {}", 3); }
/// { println!("Iteration {}", 4); }
/// ```
#[macro_export]
macro_rules! unroll {
    (for $v:ident in 0..0 $c:block) => {};

    (for $v:ident in 0..$b:tt {$($c:tt)*}) => {
        #[allow(non_upper_case_globals)]
        { unroll!(@$v, 0, $b, {$($c)*}); }
    };

"#);

    for i in 0..limit + 1 {
        output.push_str(format!("    (@$v:ident, $a:expr, {}, $c:block) => {{\n", i).as_str());

        if i <= LOWER_LIMIT {
            output.push_str(format!("        {{ const $v: usize = $a; $c }}\n").as_str());

            for a in 1..i {
                output.push_str(format!("        {{ const $v: usize = $a + {}; $c }}\n", a).as_str());
            }
        } else {
            let half = i / 2;

            if i % 2 == 0 {
                output.push_str(format!("        unroll!(@$v, $a, {0}, $c);\n", half).as_str());
                output.push_str(format!("        unroll!(@$v, $a + {0}, {0}, $c);\n", half).as_str());
            } else {
                if half > 1 {
                    output.push_str(format!("        unroll!(@$v, $a, {}, $c);\n", i - 1).as_str())
                }

                output.push_str(format!("        {{ const $v: usize = $a + {}; $c }}\n", i - 1).as_str());
            }
        }

        output.push_str("    };\n\n");
    }

    output.push_str("}\n\n");

    output.push_str(format!(r"
#[cfg(test)]
mod tests {{
    #[test]
    fn test_all() {{
        {{
            let a: Vec<usize> = vec![];
            unroll! {{
                for i in 0..0 {{
                    a.push(i);
                }}
            }}
            assert_eq!(a, (0..0).collect::<Vec<usize>>());
        }}
        {{
            let mut a: Vec<usize> = vec![];
            unroll! {{
                for i in 0..1 {{
                    a.push(i);
                }}
            }}
            assert_eq!(a, (0..1).collect::<Vec<usize>>());
        }}
        {{
            let mut a: Vec<usize> = vec![];
            unroll! {{
                for i in 0..{0} {{
                    a.push(i);
                }}
            }}
            assert_eq!(a, (0..{0}).collect::<Vec<usize>>());
        }}
    }}
}}
", limit).as_str());

    f.write_all(output.as_bytes()).unwrap();
}
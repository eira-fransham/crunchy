#!/usr/bin/env python3
import sys

if len(sys.argv) != 2:
    print('Usage: gen-code.py LIMIT')
    sys.exit(1)

LIMIT = int(sys.argv[1])

print(
    r'''#[macro_export]
macro_rules! unroll {
    (for $v:ident in 0..0 $c:block) => {};
    (for $v:ident in 0..$b:tt {$($c:tt)*}) => {
        #[allow(non_upper_case_globals)] { unroll!(@$v, 0, $b, {$($c)*}); }
    };'''
)

for i in range(1, LIMIT + 1):
    print(
        r'''    (@$v:ident, $a:expr, %d, $c:block) => {''' % i
    )
    limit = i
    half = limit // 2
    if 2*half == limit:
        for a in ('', ' + %d' % half):
            print(
                r'''        unroll!(@$v, $a%s, %d, $c);''' % (a, half)
            )
    else:
        if half > 1:
            print(
                r'''        unroll!(@$v, $a, %d, $c);''' % (limit - 1)
            )
        print(
            r'''        { const $v: usize = $a + %d; $c }''' % (limit - 1)
        )
    print(
        r'''    };'''
    )

print(
    r'''}'''
)

print(
    r'''
#[test]
fn test() {
    {
        let a: Vec<usize> = vec![];
        unroll! {
            for i in 0..0 {
                a.push(i);
            }
        }
        assert_eq!(a, (0..0).collect::<Vec<usize>>());
    }
    {
        let mut a: Vec<usize> = vec![];
        unroll! {
            for i in 0..1 {
                a.push(i);
            }
        }
        assert_eq!(a, (0..1).collect::<Vec<usize>>());
    }
    {
        let mut a: Vec<usize> = vec![];
        unroll! {
            for i in 0..%d {
                a.push(i);
            }
        }
        assert_eq!(a, (0..%d).collect::<Vec<usize>>());
    }
}
''' % (LIMIT, LIMIT)
)

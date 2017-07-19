#!/bin/bash

set -euo pipefail

echo '#[macro_export]'
echo "
  macro_rules! unroll {
    (for \$var:ident in 0..\$to:tt \$blk:block) => {
         unroll! { @for \$var \$to \$to \$blk }
    };
    (@for \$var:ident \$to:tt 0 \$blk:block) => {};
    $(
      for i in $(seq 1 $1); do
        echo "
        (@for \$var:ident \$to:tt $i \$blk:block) => {
          $(
            echo '#[allow(non_upper_case_globals)]{'
            echo "            { const \$var: usize = \$to - $i; \$blk }"
            echo "            unroll! { @for \$var \$to $((i - 1)) \$blk }"
            echo '          }'
          )
        };
        "
      done
    )
  }
"
echo '
#[test]
fn test() {
    let mut a = vec![];

    unroll! {
        for i in 0..5 {
            a.push(i);
        }
    }

    assert_eq!(a, vec![0, 1, 2, 3, 4]);
}
'

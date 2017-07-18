#!/bin/bash

set -euo pipefail

echo '#[macro_export]'
echo "
  macro_rules! unroll {
    (for \$var:ident in 0..0 \$blk:block) => {};
    $(
      for i in $(seq 1 $1); do
        echo "
        (for \$var:ident in 0..$i \$blk:block) => {
          $(
            echo '#[allow(non_upper_case_globals)]{'
            for j in $(seq 0 $((i - 1))); do
              echo "            { const \$var: usize = $j; \$blk }"
            done
            echo '          }'
          )
        };
        "
      done
    )
  }
"

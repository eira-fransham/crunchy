#[macro_export]

  macro_rules! unroll {
    (for $var:ident in 0..$to:tt $blk:block) => {
         unroll! { @for $var $to $to $blk }
    };
    (@for $var:ident $to:tt 0 $blk:block) => {};
    
        (@for $var:ident $to:tt 1 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 1; $blk }
            unroll! { @for $var $to 0 $blk }
          }
        };
        

        (@for $var:ident $to:tt 2 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 2; $blk }
            unroll! { @for $var $to 1 $blk }
          }
        };
        

        (@for $var:ident $to:tt 3 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 3; $blk }
            unroll! { @for $var $to 2 $blk }
          }
        };
        

        (@for $var:ident $to:tt 4 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 4; $blk }
            unroll! { @for $var $to 3 $blk }
          }
        };
        

        (@for $var:ident $to:tt 5 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 5; $blk }
            unroll! { @for $var $to 4 $blk }
          }
        };
        

        (@for $var:ident $to:tt 6 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 6; $blk }
            unroll! { @for $var $to 5 $blk }
          }
        };
        

        (@for $var:ident $to:tt 7 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 7; $blk }
            unroll! { @for $var $to 6 $blk }
          }
        };
        

        (@for $var:ident $to:tt 8 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 8; $blk }
            unroll! { @for $var $to 7 $blk }
          }
        };
        

        (@for $var:ident $to:tt 9 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 9; $blk }
            unroll! { @for $var $to 8 $blk }
          }
        };
        

        (@for $var:ident $to:tt 10 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 10; $blk }
            unroll! { @for $var $to 9 $blk }
          }
        };
        

        (@for $var:ident $to:tt 11 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 11; $blk }
            unroll! { @for $var $to 10 $blk }
          }
        };
        

        (@for $var:ident $to:tt 12 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 12; $blk }
            unroll! { @for $var $to 11 $blk }
          }
        };
        

        (@for $var:ident $to:tt 13 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 13; $blk }
            unroll! { @for $var $to 12 $blk }
          }
        };
        

        (@for $var:ident $to:tt 14 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 14; $blk }
            unroll! { @for $var $to 13 $blk }
          }
        };
        

        (@for $var:ident $to:tt 15 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 15; $blk }
            unroll! { @for $var $to 14 $blk }
          }
        };
        

        (@for $var:ident $to:tt 16 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 16; $blk }
            unroll! { @for $var $to 15 $blk }
          }
        };
        

        (@for $var:ident $to:tt 17 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 17; $blk }
            unroll! { @for $var $to 16 $blk }
          }
        };
        

        (@for $var:ident $to:tt 18 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 18; $blk }
            unroll! { @for $var $to 17 $blk }
          }
        };
        

        (@for $var:ident $to:tt 19 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 19; $blk }
            unroll! { @for $var $to 18 $blk }
          }
        };
        

        (@for $var:ident $to:tt 20 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 20; $blk }
            unroll! { @for $var $to 19 $blk }
          }
        };
        

        (@for $var:ident $to:tt 21 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 21; $blk }
            unroll! { @for $var $to 20 $blk }
          }
        };
        

        (@for $var:ident $to:tt 22 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 22; $blk }
            unroll! { @for $var $to 21 $blk }
          }
        };
        

        (@for $var:ident $to:tt 23 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 23; $blk }
            unroll! { @for $var $to 22 $blk }
          }
        };
        

        (@for $var:ident $to:tt 24 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 24; $blk }
            unroll! { @for $var $to 23 $blk }
          }
        };
        

        (@for $var:ident $to:tt 25 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 25; $blk }
            unroll! { @for $var $to 24 $blk }
          }
        };
        

        (@for $var:ident $to:tt 26 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 26; $blk }
            unroll! { @for $var $to 25 $blk }
          }
        };
        

        (@for $var:ident $to:tt 27 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 27; $blk }
            unroll! { @for $var $to 26 $blk }
          }
        };
        

        (@for $var:ident $to:tt 28 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 28; $blk }
            unroll! { @for $var $to 27 $blk }
          }
        };
        

        (@for $var:ident $to:tt 29 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 29; $blk }
            unroll! { @for $var $to 28 $blk }
          }
        };
        

        (@for $var:ident $to:tt 30 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 30; $blk }
            unroll! { @for $var $to 29 $blk }
          }
        };
        

        (@for $var:ident $to:tt 31 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 31; $blk }
            unroll! { @for $var $to 30 $blk }
          }
        };
        

        (@for $var:ident $to:tt 32 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 32; $blk }
            unroll! { @for $var $to 31 $blk }
          }
        };
        

        (@for $var:ident $to:tt 33 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 33; $blk }
            unroll! { @for $var $to 32 $blk }
          }
        };
        

        (@for $var:ident $to:tt 34 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 34; $blk }
            unroll! { @for $var $to 33 $blk }
          }
        };
        

        (@for $var:ident $to:tt 35 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 35; $blk }
            unroll! { @for $var $to 34 $blk }
          }
        };
        

        (@for $var:ident $to:tt 36 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 36; $blk }
            unroll! { @for $var $to 35 $blk }
          }
        };
        

        (@for $var:ident $to:tt 37 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 37; $blk }
            unroll! { @for $var $to 36 $blk }
          }
        };
        

        (@for $var:ident $to:tt 38 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 38; $blk }
            unroll! { @for $var $to 37 $blk }
          }
        };
        

        (@for $var:ident $to:tt 39 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 39; $blk }
            unroll! { @for $var $to 38 $blk }
          }
        };
        

        (@for $var:ident $to:tt 40 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 40; $blk }
            unroll! { @for $var $to 39 $blk }
          }
        };
        

        (@for $var:ident $to:tt 41 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 41; $blk }
            unroll! { @for $var $to 40 $blk }
          }
        };
        

        (@for $var:ident $to:tt 42 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 42; $blk }
            unroll! { @for $var $to 41 $blk }
          }
        };
        

        (@for $var:ident $to:tt 43 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 43; $blk }
            unroll! { @for $var $to 42 $blk }
          }
        };
        

        (@for $var:ident $to:tt 44 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 44; $blk }
            unroll! { @for $var $to 43 $blk }
          }
        };
        

        (@for $var:ident $to:tt 45 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 45; $blk }
            unroll! { @for $var $to 44 $blk }
          }
        };
        

        (@for $var:ident $to:tt 46 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 46; $blk }
            unroll! { @for $var $to 45 $blk }
          }
        };
        

        (@for $var:ident $to:tt 47 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 47; $blk }
            unroll! { @for $var $to 46 $blk }
          }
        };
        

        (@for $var:ident $to:tt 48 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 48; $blk }
            unroll! { @for $var $to 47 $blk }
          }
        };
        

        (@for $var:ident $to:tt 49 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 49; $blk }
            unroll! { @for $var $to 48 $blk }
          }
        };
        

        (@for $var:ident $to:tt 50 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 50; $blk }
            unroll! { @for $var $to 49 $blk }
          }
        };
        

        (@for $var:ident $to:tt 51 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 51; $blk }
            unroll! { @for $var $to 50 $blk }
          }
        };
        

        (@for $var:ident $to:tt 52 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 52; $blk }
            unroll! { @for $var $to 51 $blk }
          }
        };
        

        (@for $var:ident $to:tt 53 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 53; $blk }
            unroll! { @for $var $to 52 $blk }
          }
        };
        

        (@for $var:ident $to:tt 54 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 54; $blk }
            unroll! { @for $var $to 53 $blk }
          }
        };
        

        (@for $var:ident $to:tt 55 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 55; $blk }
            unroll! { @for $var $to 54 $blk }
          }
        };
        

        (@for $var:ident $to:tt 56 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 56; $blk }
            unroll! { @for $var $to 55 $blk }
          }
        };
        

        (@for $var:ident $to:tt 57 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 57; $blk }
            unroll! { @for $var $to 56 $blk }
          }
        };
        

        (@for $var:ident $to:tt 58 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 58; $blk }
            unroll! { @for $var $to 57 $blk }
          }
        };
        

        (@for $var:ident $to:tt 59 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 59; $blk }
            unroll! { @for $var $to 58 $blk }
          }
        };
        

        (@for $var:ident $to:tt 60 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 60; $blk }
            unroll! { @for $var $to 59 $blk }
          }
        };
        

        (@for $var:ident $to:tt 61 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 61; $blk }
            unroll! { @for $var $to 60 $blk }
          }
        };
        

        (@for $var:ident $to:tt 62 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 62; $blk }
            unroll! { @for $var $to 61 $blk }
          }
        };
        

        (@for $var:ident $to:tt 63 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 63; $blk }
            unroll! { @for $var $to 62 $blk }
          }
        };
        

        (@for $var:ident $to:tt 64 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 64; $blk }
            unroll! { @for $var $to 63 $blk }
          }
        };
        

        (@for $var:ident $to:tt 65 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 65; $blk }
            unroll! { @for $var $to 64 $blk }
          }
        };
        

        (@for $var:ident $to:tt 66 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 66; $blk }
            unroll! { @for $var $to 65 $blk }
          }
        };
        

        (@for $var:ident $to:tt 67 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 67; $blk }
            unroll! { @for $var $to 66 $blk }
          }
        };
        

        (@for $var:ident $to:tt 68 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 68; $blk }
            unroll! { @for $var $to 67 $blk }
          }
        };
        

        (@for $var:ident $to:tt 69 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 69; $blk }
            unroll! { @for $var $to 68 $blk }
          }
        };
        

        (@for $var:ident $to:tt 70 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 70; $blk }
            unroll! { @for $var $to 69 $blk }
          }
        };
        

        (@for $var:ident $to:tt 71 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 71; $blk }
            unroll! { @for $var $to 70 $blk }
          }
        };
        

        (@for $var:ident $to:tt 72 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 72; $blk }
            unroll! { @for $var $to 71 $blk }
          }
        };
        

        (@for $var:ident $to:tt 73 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 73; $blk }
            unroll! { @for $var $to 72 $blk }
          }
        };
        

        (@for $var:ident $to:tt 74 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 74; $blk }
            unroll! { @for $var $to 73 $blk }
          }
        };
        

        (@for $var:ident $to:tt 75 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 75; $blk }
            unroll! { @for $var $to 74 $blk }
          }
        };
        

        (@for $var:ident $to:tt 76 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 76; $blk }
            unroll! { @for $var $to 75 $blk }
          }
        };
        

        (@for $var:ident $to:tt 77 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 77; $blk }
            unroll! { @for $var $to 76 $blk }
          }
        };
        

        (@for $var:ident $to:tt 78 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 78; $blk }
            unroll! { @for $var $to 77 $blk }
          }
        };
        

        (@for $var:ident $to:tt 79 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 79; $blk }
            unroll! { @for $var $to 78 $blk }
          }
        };
        

        (@for $var:ident $to:tt 80 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 80; $blk }
            unroll! { @for $var $to 79 $blk }
          }
        };
        

        (@for $var:ident $to:tt 81 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 81; $blk }
            unroll! { @for $var $to 80 $blk }
          }
        };
        

        (@for $var:ident $to:tt 82 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 82; $blk }
            unroll! { @for $var $to 81 $blk }
          }
        };
        

        (@for $var:ident $to:tt 83 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 83; $blk }
            unroll! { @for $var $to 82 $blk }
          }
        };
        

        (@for $var:ident $to:tt 84 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 84; $blk }
            unroll! { @for $var $to 83 $blk }
          }
        };
        

        (@for $var:ident $to:tt 85 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 85; $blk }
            unroll! { @for $var $to 84 $blk }
          }
        };
        

        (@for $var:ident $to:tt 86 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 86; $blk }
            unroll! { @for $var $to 85 $blk }
          }
        };
        

        (@for $var:ident $to:tt 87 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 87; $blk }
            unroll! { @for $var $to 86 $blk }
          }
        };
        

        (@for $var:ident $to:tt 88 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 88; $blk }
            unroll! { @for $var $to 87 $blk }
          }
        };
        

        (@for $var:ident $to:tt 89 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 89; $blk }
            unroll! { @for $var $to 88 $blk }
          }
        };
        

        (@for $var:ident $to:tt 90 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 90; $blk }
            unroll! { @for $var $to 89 $blk }
          }
        };
        

        (@for $var:ident $to:tt 91 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 91; $blk }
            unroll! { @for $var $to 90 $blk }
          }
        };
        

        (@for $var:ident $to:tt 92 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 92; $blk }
            unroll! { @for $var $to 91 $blk }
          }
        };
        

        (@for $var:ident $to:tt 93 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 93; $blk }
            unroll! { @for $var $to 92 $blk }
          }
        };
        

        (@for $var:ident $to:tt 94 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 94; $blk }
            unroll! { @for $var $to 93 $blk }
          }
        };
        

        (@for $var:ident $to:tt 95 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 95; $blk }
            unroll! { @for $var $to 94 $blk }
          }
        };
        

        (@for $var:ident $to:tt 96 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 96; $blk }
            unroll! { @for $var $to 95 $blk }
          }
        };
        

        (@for $var:ident $to:tt 97 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 97; $blk }
            unroll! { @for $var $to 96 $blk }
          }
        };
        

        (@for $var:ident $to:tt 98 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 98; $blk }
            unroll! { @for $var $to 97 $blk }
          }
        };
        

        (@for $var:ident $to:tt 99 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 99; $blk }
            unroll! { @for $var $to 98 $blk }
          }
        };
        

        (@for $var:ident $to:tt 100 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 100; $blk }
            unroll! { @for $var $to 99 $blk }
          }
        };
        

        (@for $var:ident $to:tt 101 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 101; $blk }
            unroll! { @for $var $to 100 $blk }
          }
        };
        

        (@for $var:ident $to:tt 102 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 102; $blk }
            unroll! { @for $var $to 101 $blk }
          }
        };
        

        (@for $var:ident $to:tt 103 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 103; $blk }
            unroll! { @for $var $to 102 $blk }
          }
        };
        

        (@for $var:ident $to:tt 104 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 104; $blk }
            unroll! { @for $var $to 103 $blk }
          }
        };
        

        (@for $var:ident $to:tt 105 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 105; $blk }
            unroll! { @for $var $to 104 $blk }
          }
        };
        

        (@for $var:ident $to:tt 106 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 106; $blk }
            unroll! { @for $var $to 105 $blk }
          }
        };
        

        (@for $var:ident $to:tt 107 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 107; $blk }
            unroll! { @for $var $to 106 $blk }
          }
        };
        

        (@for $var:ident $to:tt 108 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 108; $blk }
            unroll! { @for $var $to 107 $blk }
          }
        };
        

        (@for $var:ident $to:tt 109 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 109; $blk }
            unroll! { @for $var $to 108 $blk }
          }
        };
        

        (@for $var:ident $to:tt 110 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 110; $blk }
            unroll! { @for $var $to 109 $blk }
          }
        };
        

        (@for $var:ident $to:tt 111 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 111; $blk }
            unroll! { @for $var $to 110 $blk }
          }
        };
        

        (@for $var:ident $to:tt 112 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 112; $blk }
            unroll! { @for $var $to 111 $blk }
          }
        };
        

        (@for $var:ident $to:tt 113 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 113; $blk }
            unroll! { @for $var $to 112 $blk }
          }
        };
        

        (@for $var:ident $to:tt 114 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 114; $blk }
            unroll! { @for $var $to 113 $blk }
          }
        };
        

        (@for $var:ident $to:tt 115 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 115; $blk }
            unroll! { @for $var $to 114 $blk }
          }
        };
        

        (@for $var:ident $to:tt 116 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 116; $blk }
            unroll! { @for $var $to 115 $blk }
          }
        };
        

        (@for $var:ident $to:tt 117 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 117; $blk }
            unroll! { @for $var $to 116 $blk }
          }
        };
        

        (@for $var:ident $to:tt 118 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 118; $blk }
            unroll! { @for $var $to 117 $blk }
          }
        };
        

        (@for $var:ident $to:tt 119 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 119; $blk }
            unroll! { @for $var $to 118 $blk }
          }
        };
        

        (@for $var:ident $to:tt 120 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 120; $blk }
            unroll! { @for $var $to 119 $blk }
          }
        };
        

        (@for $var:ident $to:tt 121 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 121; $blk }
            unroll! { @for $var $to 120 $blk }
          }
        };
        

        (@for $var:ident $to:tt 122 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 122; $blk }
            unroll! { @for $var $to 121 $blk }
          }
        };
        

        (@for $var:ident $to:tt 123 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 123; $blk }
            unroll! { @for $var $to 122 $blk }
          }
        };
        

        (@for $var:ident $to:tt 124 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 124; $blk }
            unroll! { @for $var $to 123 $blk }
          }
        };
        

        (@for $var:ident $to:tt 125 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 125; $blk }
            unroll! { @for $var $to 124 $blk }
          }
        };
        

        (@for $var:ident $to:tt 126 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 126; $blk }
            unroll! { @for $var $to 125 $blk }
          }
        };
        

        (@for $var:ident $to:tt 127 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 127; $blk }
            unroll! { @for $var $to 126 $blk }
          }
        };
        

        (@for $var:ident $to:tt 128 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 128; $blk }
            unroll! { @for $var $to 127 $blk }
          }
        };
        

        (@for $var:ident $to:tt 129 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 129; $blk }
            unroll! { @for $var $to 128 $blk }
          }
        };
        

        (@for $var:ident $to:tt 130 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 130; $blk }
            unroll! { @for $var $to 129 $blk }
          }
        };
        

        (@for $var:ident $to:tt 131 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 131; $blk }
            unroll! { @for $var $to 130 $blk }
          }
        };
        

        (@for $var:ident $to:tt 132 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 132; $blk }
            unroll! { @for $var $to 131 $blk }
          }
        };
        

        (@for $var:ident $to:tt 133 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 133; $blk }
            unroll! { @for $var $to 132 $blk }
          }
        };
        

        (@for $var:ident $to:tt 134 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 134; $blk }
            unroll! { @for $var $to 133 $blk }
          }
        };
        

        (@for $var:ident $to:tt 135 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 135; $blk }
            unroll! { @for $var $to 134 $blk }
          }
        };
        

        (@for $var:ident $to:tt 136 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 136; $blk }
            unroll! { @for $var $to 135 $blk }
          }
        };
        

        (@for $var:ident $to:tt 137 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 137; $blk }
            unroll! { @for $var $to 136 $blk }
          }
        };
        

        (@for $var:ident $to:tt 138 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 138; $blk }
            unroll! { @for $var $to 137 $blk }
          }
        };
        

        (@for $var:ident $to:tt 139 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 139; $blk }
            unroll! { @for $var $to 138 $blk }
          }
        };
        

        (@for $var:ident $to:tt 140 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 140; $blk }
            unroll! { @for $var $to 139 $blk }
          }
        };
        

        (@for $var:ident $to:tt 141 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 141; $blk }
            unroll! { @for $var $to 140 $blk }
          }
        };
        

        (@for $var:ident $to:tt 142 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 142; $blk }
            unroll! { @for $var $to 141 $blk }
          }
        };
        

        (@for $var:ident $to:tt 143 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 143; $blk }
            unroll! { @for $var $to 142 $blk }
          }
        };
        

        (@for $var:ident $to:tt 144 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 144; $blk }
            unroll! { @for $var $to 143 $blk }
          }
        };
        

        (@for $var:ident $to:tt 145 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 145; $blk }
            unroll! { @for $var $to 144 $blk }
          }
        };
        

        (@for $var:ident $to:tt 146 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 146; $blk }
            unroll! { @for $var $to 145 $blk }
          }
        };
        

        (@for $var:ident $to:tt 147 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 147; $blk }
            unroll! { @for $var $to 146 $blk }
          }
        };
        

        (@for $var:ident $to:tt 148 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 148; $blk }
            unroll! { @for $var $to 147 $blk }
          }
        };
        

        (@for $var:ident $to:tt 149 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 149; $blk }
            unroll! { @for $var $to 148 $blk }
          }
        };
        

        (@for $var:ident $to:tt 150 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 150; $blk }
            unroll! { @for $var $to 149 $blk }
          }
        };
        

        (@for $var:ident $to:tt 151 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 151; $blk }
            unroll! { @for $var $to 150 $blk }
          }
        };
        

        (@for $var:ident $to:tt 152 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 152; $blk }
            unroll! { @for $var $to 151 $blk }
          }
        };
        

        (@for $var:ident $to:tt 153 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 153; $blk }
            unroll! { @for $var $to 152 $blk }
          }
        };
        

        (@for $var:ident $to:tt 154 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 154; $blk }
            unroll! { @for $var $to 153 $blk }
          }
        };
        

        (@for $var:ident $to:tt 155 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 155; $blk }
            unroll! { @for $var $to 154 $blk }
          }
        };
        

        (@for $var:ident $to:tt 156 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 156; $blk }
            unroll! { @for $var $to 155 $blk }
          }
        };
        

        (@for $var:ident $to:tt 157 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 157; $blk }
            unroll! { @for $var $to 156 $blk }
          }
        };
        

        (@for $var:ident $to:tt 158 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 158; $blk }
            unroll! { @for $var $to 157 $blk }
          }
        };
        

        (@for $var:ident $to:tt 159 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 159; $blk }
            unroll! { @for $var $to 158 $blk }
          }
        };
        

        (@for $var:ident $to:tt 160 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 160; $blk }
            unroll! { @for $var $to 159 $blk }
          }
        };
        

        (@for $var:ident $to:tt 161 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 161; $blk }
            unroll! { @for $var $to 160 $blk }
          }
        };
        

        (@for $var:ident $to:tt 162 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 162; $blk }
            unroll! { @for $var $to 161 $blk }
          }
        };
        

        (@for $var:ident $to:tt 163 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 163; $blk }
            unroll! { @for $var $to 162 $blk }
          }
        };
        

        (@for $var:ident $to:tt 164 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 164; $blk }
            unroll! { @for $var $to 163 $blk }
          }
        };
        

        (@for $var:ident $to:tt 165 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 165; $blk }
            unroll! { @for $var $to 164 $blk }
          }
        };
        

        (@for $var:ident $to:tt 166 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 166; $blk }
            unroll! { @for $var $to 165 $blk }
          }
        };
        

        (@for $var:ident $to:tt 167 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 167; $blk }
            unroll! { @for $var $to 166 $blk }
          }
        };
        

        (@for $var:ident $to:tt 168 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 168; $blk }
            unroll! { @for $var $to 167 $blk }
          }
        };
        

        (@for $var:ident $to:tt 169 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 169; $blk }
            unroll! { @for $var $to 168 $blk }
          }
        };
        

        (@for $var:ident $to:tt 170 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 170; $blk }
            unroll! { @for $var $to 169 $blk }
          }
        };
        

        (@for $var:ident $to:tt 171 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 171; $blk }
            unroll! { @for $var $to 170 $blk }
          }
        };
        

        (@for $var:ident $to:tt 172 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 172; $blk }
            unroll! { @for $var $to 171 $blk }
          }
        };
        

        (@for $var:ident $to:tt 173 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 173; $blk }
            unroll! { @for $var $to 172 $blk }
          }
        };
        

        (@for $var:ident $to:tt 174 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 174; $blk }
            unroll! { @for $var $to 173 $blk }
          }
        };
        

        (@for $var:ident $to:tt 175 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 175; $blk }
            unroll! { @for $var $to 174 $blk }
          }
        };
        

        (@for $var:ident $to:tt 176 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 176; $blk }
            unroll! { @for $var $to 175 $blk }
          }
        };
        

        (@for $var:ident $to:tt 177 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 177; $blk }
            unroll! { @for $var $to 176 $blk }
          }
        };
        

        (@for $var:ident $to:tt 178 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 178; $blk }
            unroll! { @for $var $to 177 $blk }
          }
        };
        

        (@for $var:ident $to:tt 179 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 179; $blk }
            unroll! { @for $var $to 178 $blk }
          }
        };
        

        (@for $var:ident $to:tt 180 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 180; $blk }
            unroll! { @for $var $to 179 $blk }
          }
        };
        

        (@for $var:ident $to:tt 181 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 181; $blk }
            unroll! { @for $var $to 180 $blk }
          }
        };
        

        (@for $var:ident $to:tt 182 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 182; $blk }
            unroll! { @for $var $to 181 $blk }
          }
        };
        

        (@for $var:ident $to:tt 183 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 183; $blk }
            unroll! { @for $var $to 182 $blk }
          }
        };
        

        (@for $var:ident $to:tt 184 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 184; $blk }
            unroll! { @for $var $to 183 $blk }
          }
        };
        

        (@for $var:ident $to:tt 185 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 185; $blk }
            unroll! { @for $var $to 184 $blk }
          }
        };
        

        (@for $var:ident $to:tt 186 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 186; $blk }
            unroll! { @for $var $to 185 $blk }
          }
        };
        

        (@for $var:ident $to:tt 187 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 187; $blk }
            unroll! { @for $var $to 186 $blk }
          }
        };
        

        (@for $var:ident $to:tt 188 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 188; $blk }
            unroll! { @for $var $to 187 $blk }
          }
        };
        

        (@for $var:ident $to:tt 189 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 189; $blk }
            unroll! { @for $var $to 188 $blk }
          }
        };
        

        (@for $var:ident $to:tt 190 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 190; $blk }
            unroll! { @for $var $to 189 $blk }
          }
        };
        

        (@for $var:ident $to:tt 191 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 191; $blk }
            unroll! { @for $var $to 190 $blk }
          }
        };
        

        (@for $var:ident $to:tt 192 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 192; $blk }
            unroll! { @for $var $to 191 $blk }
          }
        };
        

        (@for $var:ident $to:tt 193 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 193; $blk }
            unroll! { @for $var $to 192 $blk }
          }
        };
        

        (@for $var:ident $to:tt 194 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 194; $blk }
            unroll! { @for $var $to 193 $blk }
          }
        };
        

        (@for $var:ident $to:tt 195 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 195; $blk }
            unroll! { @for $var $to 194 $blk }
          }
        };
        

        (@for $var:ident $to:tt 196 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 196; $blk }
            unroll! { @for $var $to 195 $blk }
          }
        };
        

        (@for $var:ident $to:tt 197 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 197; $blk }
            unroll! { @for $var $to 196 $blk }
          }
        };
        

        (@for $var:ident $to:tt 198 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 198; $blk }
            unroll! { @for $var $to 197 $blk }
          }
        };
        

        (@for $var:ident $to:tt 199 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 199; $blk }
            unroll! { @for $var $to 198 $blk }
          }
        };
        

        (@for $var:ident $to:tt 200 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 200; $blk }
            unroll! { @for $var $to 199 $blk }
          }
        };
        

        (@for $var:ident $to:tt 201 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 201; $blk }
            unroll! { @for $var $to 200 $blk }
          }
        };
        

        (@for $var:ident $to:tt 202 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 202; $blk }
            unroll! { @for $var $to 201 $blk }
          }
        };
        

        (@for $var:ident $to:tt 203 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 203; $blk }
            unroll! { @for $var $to 202 $blk }
          }
        };
        

        (@for $var:ident $to:tt 204 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 204; $blk }
            unroll! { @for $var $to 203 $blk }
          }
        };
        

        (@for $var:ident $to:tt 205 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 205; $blk }
            unroll! { @for $var $to 204 $blk }
          }
        };
        

        (@for $var:ident $to:tt 206 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 206; $blk }
            unroll! { @for $var $to 205 $blk }
          }
        };
        

        (@for $var:ident $to:tt 207 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 207; $blk }
            unroll! { @for $var $to 206 $blk }
          }
        };
        

        (@for $var:ident $to:tt 208 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 208; $blk }
            unroll! { @for $var $to 207 $blk }
          }
        };
        

        (@for $var:ident $to:tt 209 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 209; $blk }
            unroll! { @for $var $to 208 $blk }
          }
        };
        

        (@for $var:ident $to:tt 210 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 210; $blk }
            unroll! { @for $var $to 209 $blk }
          }
        };
        

        (@for $var:ident $to:tt 211 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 211; $blk }
            unroll! { @for $var $to 210 $blk }
          }
        };
        

        (@for $var:ident $to:tt 212 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 212; $blk }
            unroll! { @for $var $to 211 $blk }
          }
        };
        

        (@for $var:ident $to:tt 213 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 213; $blk }
            unroll! { @for $var $to 212 $blk }
          }
        };
        

        (@for $var:ident $to:tt 214 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 214; $blk }
            unroll! { @for $var $to 213 $blk }
          }
        };
        

        (@for $var:ident $to:tt 215 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 215; $blk }
            unroll! { @for $var $to 214 $blk }
          }
        };
        

        (@for $var:ident $to:tt 216 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 216; $blk }
            unroll! { @for $var $to 215 $blk }
          }
        };
        

        (@for $var:ident $to:tt 217 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 217; $blk }
            unroll! { @for $var $to 216 $blk }
          }
        };
        

        (@for $var:ident $to:tt 218 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 218; $blk }
            unroll! { @for $var $to 217 $blk }
          }
        };
        

        (@for $var:ident $to:tt 219 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 219; $blk }
            unroll! { @for $var $to 218 $blk }
          }
        };
        

        (@for $var:ident $to:tt 220 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 220; $blk }
            unroll! { @for $var $to 219 $blk }
          }
        };
        

        (@for $var:ident $to:tt 221 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 221; $blk }
            unroll! { @for $var $to 220 $blk }
          }
        };
        

        (@for $var:ident $to:tt 222 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 222; $blk }
            unroll! { @for $var $to 221 $blk }
          }
        };
        

        (@for $var:ident $to:tt 223 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 223; $blk }
            unroll! { @for $var $to 222 $blk }
          }
        };
        

        (@for $var:ident $to:tt 224 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 224; $blk }
            unroll! { @for $var $to 223 $blk }
          }
        };
        

        (@for $var:ident $to:tt 225 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 225; $blk }
            unroll! { @for $var $to 224 $blk }
          }
        };
        

        (@for $var:ident $to:tt 226 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 226; $blk }
            unroll! { @for $var $to 225 $blk }
          }
        };
        

        (@for $var:ident $to:tt 227 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 227; $blk }
            unroll! { @for $var $to 226 $blk }
          }
        };
        

        (@for $var:ident $to:tt 228 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 228; $blk }
            unroll! { @for $var $to 227 $blk }
          }
        };
        

        (@for $var:ident $to:tt 229 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 229; $blk }
            unroll! { @for $var $to 228 $blk }
          }
        };
        

        (@for $var:ident $to:tt 230 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 230; $blk }
            unroll! { @for $var $to 229 $blk }
          }
        };
        

        (@for $var:ident $to:tt 231 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 231; $blk }
            unroll! { @for $var $to 230 $blk }
          }
        };
        

        (@for $var:ident $to:tt 232 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 232; $blk }
            unroll! { @for $var $to 231 $blk }
          }
        };
        

        (@for $var:ident $to:tt 233 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 233; $blk }
            unroll! { @for $var $to 232 $blk }
          }
        };
        

        (@for $var:ident $to:tt 234 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 234; $blk }
            unroll! { @for $var $to 233 $blk }
          }
        };
        

        (@for $var:ident $to:tt 235 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 235; $blk }
            unroll! { @for $var $to 234 $blk }
          }
        };
        

        (@for $var:ident $to:tt 236 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 236; $blk }
            unroll! { @for $var $to 235 $blk }
          }
        };
        

        (@for $var:ident $to:tt 237 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 237; $blk }
            unroll! { @for $var $to 236 $blk }
          }
        };
        

        (@for $var:ident $to:tt 238 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 238; $blk }
            unroll! { @for $var $to 237 $blk }
          }
        };
        

        (@for $var:ident $to:tt 239 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 239; $blk }
            unroll! { @for $var $to 238 $blk }
          }
        };
        

        (@for $var:ident $to:tt 240 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 240; $blk }
            unroll! { @for $var $to 239 $blk }
          }
        };
        

        (@for $var:ident $to:tt 241 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 241; $blk }
            unroll! { @for $var $to 240 $blk }
          }
        };
        

        (@for $var:ident $to:tt 242 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 242; $blk }
            unroll! { @for $var $to 241 $blk }
          }
        };
        

        (@for $var:ident $to:tt 243 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 243; $blk }
            unroll! { @for $var $to 242 $blk }
          }
        };
        

        (@for $var:ident $to:tt 244 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 244; $blk }
            unroll! { @for $var $to 243 $blk }
          }
        };
        

        (@for $var:ident $to:tt 245 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 245; $blk }
            unroll! { @for $var $to 244 $blk }
          }
        };
        

        (@for $var:ident $to:tt 246 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 246; $blk }
            unroll! { @for $var $to 245 $blk }
          }
        };
        

        (@for $var:ident $to:tt 247 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 247; $blk }
            unroll! { @for $var $to 246 $blk }
          }
        };
        

        (@for $var:ident $to:tt 248 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 248; $blk }
            unroll! { @for $var $to 247 $blk }
          }
        };
        

        (@for $var:ident $to:tt 249 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 249; $blk }
            unroll! { @for $var $to 248 $blk }
          }
        };
        

        (@for $var:ident $to:tt 250 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 250; $blk }
            unroll! { @for $var $to 249 $blk }
          }
        };
        

        (@for $var:ident $to:tt 251 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 251; $blk }
            unroll! { @for $var $to 250 $blk }
          }
        };
        

        (@for $var:ident $to:tt 252 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 252; $blk }
            unroll! { @for $var $to 251 $blk }
          }
        };
        

        (@for $var:ident $to:tt 253 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 253; $blk }
            unroll! { @for $var $to 252 $blk }
          }
        };
        

        (@for $var:ident $to:tt 254 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 254; $blk }
            unroll! { @for $var $to 253 $blk }
          }
        };
        

        (@for $var:ident $to:tt 255 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 255; $blk }
            unroll! { @for $var $to 254 $blk }
          }
        };
        

        (@for $var:ident $to:tt 256 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 256; $blk }
            unroll! { @for $var $to 255 $blk }
          }
        };
        

        (@for $var:ident $to:tt 257 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 257; $blk }
            unroll! { @for $var $to 256 $blk }
          }
        };
        

        (@for $var:ident $to:tt 258 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 258; $blk }
            unroll! { @for $var $to 257 $blk }
          }
        };
        

        (@for $var:ident $to:tt 259 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 259; $blk }
            unroll! { @for $var $to 258 $blk }
          }
        };
        

        (@for $var:ident $to:tt 260 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 260; $blk }
            unroll! { @for $var $to 259 $blk }
          }
        };
        

        (@for $var:ident $to:tt 261 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 261; $blk }
            unroll! { @for $var $to 260 $blk }
          }
        };
        

        (@for $var:ident $to:tt 262 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 262; $blk }
            unroll! { @for $var $to 261 $blk }
          }
        };
        

        (@for $var:ident $to:tt 263 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 263; $blk }
            unroll! { @for $var $to 262 $blk }
          }
        };
        

        (@for $var:ident $to:tt 264 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 264; $blk }
            unroll! { @for $var $to 263 $blk }
          }
        };
        

        (@for $var:ident $to:tt 265 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 265; $blk }
            unroll! { @for $var $to 264 $blk }
          }
        };
        

        (@for $var:ident $to:tt 266 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 266; $blk }
            unroll! { @for $var $to 265 $blk }
          }
        };
        

        (@for $var:ident $to:tt 267 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 267; $blk }
            unroll! { @for $var $to 266 $blk }
          }
        };
        

        (@for $var:ident $to:tt 268 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 268; $blk }
            unroll! { @for $var $to 267 $blk }
          }
        };
        

        (@for $var:ident $to:tt 269 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 269; $blk }
            unroll! { @for $var $to 268 $blk }
          }
        };
        

        (@for $var:ident $to:tt 270 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 270; $blk }
            unroll! { @for $var $to 269 $blk }
          }
        };
        

        (@for $var:ident $to:tt 271 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 271; $blk }
            unroll! { @for $var $to 270 $blk }
          }
        };
        

        (@for $var:ident $to:tt 272 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 272; $blk }
            unroll! { @for $var $to 271 $blk }
          }
        };
        

        (@for $var:ident $to:tt 273 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 273; $blk }
            unroll! { @for $var $to 272 $blk }
          }
        };
        

        (@for $var:ident $to:tt 274 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 274; $blk }
            unroll! { @for $var $to 273 $blk }
          }
        };
        

        (@for $var:ident $to:tt 275 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 275; $blk }
            unroll! { @for $var $to 274 $blk }
          }
        };
        

        (@for $var:ident $to:tt 276 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 276; $blk }
            unroll! { @for $var $to 275 $blk }
          }
        };
        

        (@for $var:ident $to:tt 277 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 277; $blk }
            unroll! { @for $var $to 276 $blk }
          }
        };
        

        (@for $var:ident $to:tt 278 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 278; $blk }
            unroll! { @for $var $to 277 $blk }
          }
        };
        

        (@for $var:ident $to:tt 279 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 279; $blk }
            unroll! { @for $var $to 278 $blk }
          }
        };
        

        (@for $var:ident $to:tt 280 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 280; $blk }
            unroll! { @for $var $to 279 $blk }
          }
        };
        

        (@for $var:ident $to:tt 281 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 281; $blk }
            unroll! { @for $var $to 280 $blk }
          }
        };
        

        (@for $var:ident $to:tt 282 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 282; $blk }
            unroll! { @for $var $to 281 $blk }
          }
        };
        

        (@for $var:ident $to:tt 283 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 283; $blk }
            unroll! { @for $var $to 282 $blk }
          }
        };
        

        (@for $var:ident $to:tt 284 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 284; $blk }
            unroll! { @for $var $to 283 $blk }
          }
        };
        

        (@for $var:ident $to:tt 285 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 285; $blk }
            unroll! { @for $var $to 284 $blk }
          }
        };
        

        (@for $var:ident $to:tt 286 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 286; $blk }
            unroll! { @for $var $to 285 $blk }
          }
        };
        

        (@for $var:ident $to:tt 287 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 287; $blk }
            unroll! { @for $var $to 286 $blk }
          }
        };
        

        (@for $var:ident $to:tt 288 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 288; $blk }
            unroll! { @for $var $to 287 $blk }
          }
        };
        

        (@for $var:ident $to:tt 289 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 289; $blk }
            unroll! { @for $var $to 288 $blk }
          }
        };
        

        (@for $var:ident $to:tt 290 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 290; $blk }
            unroll! { @for $var $to 289 $blk }
          }
        };
        

        (@for $var:ident $to:tt 291 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 291; $blk }
            unroll! { @for $var $to 290 $blk }
          }
        };
        

        (@for $var:ident $to:tt 292 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 292; $blk }
            unroll! { @for $var $to 291 $blk }
          }
        };
        

        (@for $var:ident $to:tt 293 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 293; $blk }
            unroll! { @for $var $to 292 $blk }
          }
        };
        

        (@for $var:ident $to:tt 294 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 294; $blk }
            unroll! { @for $var $to 293 $blk }
          }
        };
        

        (@for $var:ident $to:tt 295 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 295; $blk }
            unroll! { @for $var $to 294 $blk }
          }
        };
        

        (@for $var:ident $to:tt 296 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 296; $blk }
            unroll! { @for $var $to 295 $blk }
          }
        };
        

        (@for $var:ident $to:tt 297 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 297; $blk }
            unroll! { @for $var $to 296 $blk }
          }
        };
        

        (@for $var:ident $to:tt 298 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 298; $blk }
            unroll! { @for $var $to 297 $blk }
          }
        };
        

        (@for $var:ident $to:tt 299 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 299; $blk }
            unroll! { @for $var $to 298 $blk }
          }
        };
        

        (@for $var:ident $to:tt 300 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 300; $blk }
            unroll! { @for $var $to 299 $blk }
          }
        };
        

        (@for $var:ident $to:tt 301 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 301; $blk }
            unroll! { @for $var $to 300 $blk }
          }
        };
        

        (@for $var:ident $to:tt 302 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 302; $blk }
            unroll! { @for $var $to 301 $blk }
          }
        };
        

        (@for $var:ident $to:tt 303 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 303; $blk }
            unroll! { @for $var $to 302 $blk }
          }
        };
        

        (@for $var:ident $to:tt 304 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 304; $blk }
            unroll! { @for $var $to 303 $blk }
          }
        };
        

        (@for $var:ident $to:tt 305 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 305; $blk }
            unroll! { @for $var $to 304 $blk }
          }
        };
        

        (@for $var:ident $to:tt 306 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 306; $blk }
            unroll! { @for $var $to 305 $blk }
          }
        };
        

        (@for $var:ident $to:tt 307 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 307; $blk }
            unroll! { @for $var $to 306 $blk }
          }
        };
        

        (@for $var:ident $to:tt 308 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 308; $blk }
            unroll! { @for $var $to 307 $blk }
          }
        };
        

        (@for $var:ident $to:tt 309 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 309; $blk }
            unroll! { @for $var $to 308 $blk }
          }
        };
        

        (@for $var:ident $to:tt 310 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 310; $blk }
            unroll! { @for $var $to 309 $blk }
          }
        };
        

        (@for $var:ident $to:tt 311 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 311; $blk }
            unroll! { @for $var $to 310 $blk }
          }
        };
        

        (@for $var:ident $to:tt 312 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 312; $blk }
            unroll! { @for $var $to 311 $blk }
          }
        };
        

        (@for $var:ident $to:tt 313 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 313; $blk }
            unroll! { @for $var $to 312 $blk }
          }
        };
        

        (@for $var:ident $to:tt 314 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 314; $blk }
            unroll! { @for $var $to 313 $blk }
          }
        };
        

        (@for $var:ident $to:tt 315 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 315; $blk }
            unroll! { @for $var $to 314 $blk }
          }
        };
        

        (@for $var:ident $to:tt 316 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 316; $blk }
            unroll! { @for $var $to 315 $blk }
          }
        };
        

        (@for $var:ident $to:tt 317 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 317; $blk }
            unroll! { @for $var $to 316 $blk }
          }
        };
        

        (@for $var:ident $to:tt 318 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 318; $blk }
            unroll! { @for $var $to 317 $blk }
          }
        };
        

        (@for $var:ident $to:tt 319 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 319; $blk }
            unroll! { @for $var $to 318 $blk }
          }
        };
        

        (@for $var:ident $to:tt 320 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 320; $blk }
            unroll! { @for $var $to 319 $blk }
          }
        };
        

        (@for $var:ident $to:tt 321 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 321; $blk }
            unroll! { @for $var $to 320 $blk }
          }
        };
        

        (@for $var:ident $to:tt 322 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 322; $blk }
            unroll! { @for $var $to 321 $blk }
          }
        };
        

        (@for $var:ident $to:tt 323 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 323; $blk }
            unroll! { @for $var $to 322 $blk }
          }
        };
        

        (@for $var:ident $to:tt 324 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 324; $blk }
            unroll! { @for $var $to 323 $blk }
          }
        };
        

        (@for $var:ident $to:tt 325 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 325; $blk }
            unroll! { @for $var $to 324 $blk }
          }
        };
        

        (@for $var:ident $to:tt 326 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 326; $blk }
            unroll! { @for $var $to 325 $blk }
          }
        };
        

        (@for $var:ident $to:tt 327 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 327; $blk }
            unroll! { @for $var $to 326 $blk }
          }
        };
        

        (@for $var:ident $to:tt 328 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 328; $blk }
            unroll! { @for $var $to 327 $blk }
          }
        };
        

        (@for $var:ident $to:tt 329 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 329; $blk }
            unroll! { @for $var $to 328 $blk }
          }
        };
        

        (@for $var:ident $to:tt 330 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 330; $blk }
            unroll! { @for $var $to 329 $blk }
          }
        };
        

        (@for $var:ident $to:tt 331 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 331; $blk }
            unroll! { @for $var $to 330 $blk }
          }
        };
        

        (@for $var:ident $to:tt 332 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 332; $blk }
            unroll! { @for $var $to 331 $blk }
          }
        };
        

        (@for $var:ident $to:tt 333 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 333; $blk }
            unroll! { @for $var $to 332 $blk }
          }
        };
        

        (@for $var:ident $to:tt 334 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 334; $blk }
            unroll! { @for $var $to 333 $blk }
          }
        };
        

        (@for $var:ident $to:tt 335 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 335; $blk }
            unroll! { @for $var $to 334 $blk }
          }
        };
        

        (@for $var:ident $to:tt 336 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 336; $blk }
            unroll! { @for $var $to 335 $blk }
          }
        };
        

        (@for $var:ident $to:tt 337 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 337; $blk }
            unroll! { @for $var $to 336 $blk }
          }
        };
        

        (@for $var:ident $to:tt 338 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 338; $blk }
            unroll! { @for $var $to 337 $blk }
          }
        };
        

        (@for $var:ident $to:tt 339 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 339; $blk }
            unroll! { @for $var $to 338 $blk }
          }
        };
        

        (@for $var:ident $to:tt 340 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 340; $blk }
            unroll! { @for $var $to 339 $blk }
          }
        };
        

        (@for $var:ident $to:tt 341 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 341; $blk }
            unroll! { @for $var $to 340 $blk }
          }
        };
        

        (@for $var:ident $to:tt 342 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 342; $blk }
            unroll! { @for $var $to 341 $blk }
          }
        };
        

        (@for $var:ident $to:tt 343 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 343; $blk }
            unroll! { @for $var $to 342 $blk }
          }
        };
        

        (@for $var:ident $to:tt 344 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 344; $blk }
            unroll! { @for $var $to 343 $blk }
          }
        };
        

        (@for $var:ident $to:tt 345 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 345; $blk }
            unroll! { @for $var $to 344 $blk }
          }
        };
        

        (@for $var:ident $to:tt 346 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 346; $blk }
            unroll! { @for $var $to 345 $blk }
          }
        };
        

        (@for $var:ident $to:tt 347 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 347; $blk }
            unroll! { @for $var $to 346 $blk }
          }
        };
        

        (@for $var:ident $to:tt 348 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 348; $blk }
            unroll! { @for $var $to 347 $blk }
          }
        };
        

        (@for $var:ident $to:tt 349 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 349; $blk }
            unroll! { @for $var $to 348 $blk }
          }
        };
        

        (@for $var:ident $to:tt 350 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 350; $blk }
            unroll! { @for $var $to 349 $blk }
          }
        };
        

        (@for $var:ident $to:tt 351 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 351; $blk }
            unroll! { @for $var $to 350 $blk }
          }
        };
        

        (@for $var:ident $to:tt 352 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 352; $blk }
            unroll! { @for $var $to 351 $blk }
          }
        };
        

        (@for $var:ident $to:tt 353 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 353; $blk }
            unroll! { @for $var $to 352 $blk }
          }
        };
        

        (@for $var:ident $to:tt 354 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 354; $blk }
            unroll! { @for $var $to 353 $blk }
          }
        };
        

        (@for $var:ident $to:tt 355 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 355; $blk }
            unroll! { @for $var $to 354 $blk }
          }
        };
        

        (@for $var:ident $to:tt 356 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 356; $blk }
            unroll! { @for $var $to 355 $blk }
          }
        };
        

        (@for $var:ident $to:tt 357 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 357; $blk }
            unroll! { @for $var $to 356 $blk }
          }
        };
        

        (@for $var:ident $to:tt 358 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 358; $blk }
            unroll! { @for $var $to 357 $blk }
          }
        };
        

        (@for $var:ident $to:tt 359 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 359; $blk }
            unroll! { @for $var $to 358 $blk }
          }
        };
        

        (@for $var:ident $to:tt 360 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 360; $blk }
            unroll! { @for $var $to 359 $blk }
          }
        };
        

        (@for $var:ident $to:tt 361 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 361; $blk }
            unroll! { @for $var $to 360 $blk }
          }
        };
        

        (@for $var:ident $to:tt 362 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 362; $blk }
            unroll! { @for $var $to 361 $blk }
          }
        };
        

        (@for $var:ident $to:tt 363 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 363; $blk }
            unroll! { @for $var $to 362 $blk }
          }
        };
        

        (@for $var:ident $to:tt 364 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 364; $blk }
            unroll! { @for $var $to 363 $blk }
          }
        };
        

        (@for $var:ident $to:tt 365 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 365; $blk }
            unroll! { @for $var $to 364 $blk }
          }
        };
        

        (@for $var:ident $to:tt 366 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 366; $blk }
            unroll! { @for $var $to 365 $blk }
          }
        };
        

        (@for $var:ident $to:tt 367 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 367; $blk }
            unroll! { @for $var $to 366 $blk }
          }
        };
        

        (@for $var:ident $to:tt 368 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 368; $blk }
            unroll! { @for $var $to 367 $blk }
          }
        };
        

        (@for $var:ident $to:tt 369 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 369; $blk }
            unroll! { @for $var $to 368 $blk }
          }
        };
        

        (@for $var:ident $to:tt 370 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 370; $blk }
            unroll! { @for $var $to 369 $blk }
          }
        };
        

        (@for $var:ident $to:tt 371 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 371; $blk }
            unroll! { @for $var $to 370 $blk }
          }
        };
        

        (@for $var:ident $to:tt 372 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 372; $blk }
            unroll! { @for $var $to 371 $blk }
          }
        };
        

        (@for $var:ident $to:tt 373 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 373; $blk }
            unroll! { @for $var $to 372 $blk }
          }
        };
        

        (@for $var:ident $to:tt 374 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 374; $blk }
            unroll! { @for $var $to 373 $blk }
          }
        };
        

        (@for $var:ident $to:tt 375 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 375; $blk }
            unroll! { @for $var $to 374 $blk }
          }
        };
        

        (@for $var:ident $to:tt 376 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 376; $blk }
            unroll! { @for $var $to 375 $blk }
          }
        };
        

        (@for $var:ident $to:tt 377 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 377; $blk }
            unroll! { @for $var $to 376 $blk }
          }
        };
        

        (@for $var:ident $to:tt 378 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 378; $blk }
            unroll! { @for $var $to 377 $blk }
          }
        };
        

        (@for $var:ident $to:tt 379 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 379; $blk }
            unroll! { @for $var $to 378 $blk }
          }
        };
        

        (@for $var:ident $to:tt 380 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 380; $blk }
            unroll! { @for $var $to 379 $blk }
          }
        };
        

        (@for $var:ident $to:tt 381 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 381; $blk }
            unroll! { @for $var $to 380 $blk }
          }
        };
        

        (@for $var:ident $to:tt 382 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 382; $blk }
            unroll! { @for $var $to 381 $blk }
          }
        };
        

        (@for $var:ident $to:tt 383 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 383; $blk }
            unroll! { @for $var $to 382 $blk }
          }
        };
        

        (@for $var:ident $to:tt 384 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 384; $blk }
            unroll! { @for $var $to 383 $blk }
          }
        };
        

        (@for $var:ident $to:tt 385 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 385; $blk }
            unroll! { @for $var $to 384 $blk }
          }
        };
        

        (@for $var:ident $to:tt 386 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 386; $blk }
            unroll! { @for $var $to 385 $blk }
          }
        };
        

        (@for $var:ident $to:tt 387 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 387; $blk }
            unroll! { @for $var $to 386 $blk }
          }
        };
        

        (@for $var:ident $to:tt 388 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 388; $blk }
            unroll! { @for $var $to 387 $blk }
          }
        };
        

        (@for $var:ident $to:tt 389 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 389; $blk }
            unroll! { @for $var $to 388 $blk }
          }
        };
        

        (@for $var:ident $to:tt 390 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 390; $blk }
            unroll! { @for $var $to 389 $blk }
          }
        };
        

        (@for $var:ident $to:tt 391 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 391; $blk }
            unroll! { @for $var $to 390 $blk }
          }
        };
        

        (@for $var:ident $to:tt 392 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 392; $blk }
            unroll! { @for $var $to 391 $blk }
          }
        };
        

        (@for $var:ident $to:tt 393 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 393; $blk }
            unroll! { @for $var $to 392 $blk }
          }
        };
        

        (@for $var:ident $to:tt 394 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 394; $blk }
            unroll! { @for $var $to 393 $blk }
          }
        };
        

        (@for $var:ident $to:tt 395 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 395; $blk }
            unroll! { @for $var $to 394 $blk }
          }
        };
        

        (@for $var:ident $to:tt 396 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 396; $blk }
            unroll! { @for $var $to 395 $blk }
          }
        };
        

        (@for $var:ident $to:tt 397 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 397; $blk }
            unroll! { @for $var $to 396 $blk }
          }
        };
        

        (@for $var:ident $to:tt 398 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 398; $blk }
            unroll! { @for $var $to 397 $blk }
          }
        };
        

        (@for $var:ident $to:tt 399 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 399; $blk }
            unroll! { @for $var $to 398 $blk }
          }
        };
        

        (@for $var:ident $to:tt 400 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 400; $blk }
            unroll! { @for $var $to 399 $blk }
          }
        };
        

        (@for $var:ident $to:tt 401 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 401; $blk }
            unroll! { @for $var $to 400 $blk }
          }
        };
        

        (@for $var:ident $to:tt 402 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 402; $blk }
            unroll! { @for $var $to 401 $blk }
          }
        };
        

        (@for $var:ident $to:tt 403 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 403; $blk }
            unroll! { @for $var $to 402 $blk }
          }
        };
        

        (@for $var:ident $to:tt 404 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 404; $blk }
            unroll! { @for $var $to 403 $blk }
          }
        };
        

        (@for $var:ident $to:tt 405 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 405; $blk }
            unroll! { @for $var $to 404 $blk }
          }
        };
        

        (@for $var:ident $to:tt 406 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 406; $blk }
            unroll! { @for $var $to 405 $blk }
          }
        };
        

        (@for $var:ident $to:tt 407 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 407; $blk }
            unroll! { @for $var $to 406 $blk }
          }
        };
        

        (@for $var:ident $to:tt 408 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 408; $blk }
            unroll! { @for $var $to 407 $blk }
          }
        };
        

        (@for $var:ident $to:tt 409 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 409; $blk }
            unroll! { @for $var $to 408 $blk }
          }
        };
        

        (@for $var:ident $to:tt 410 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 410; $blk }
            unroll! { @for $var $to 409 $blk }
          }
        };
        

        (@for $var:ident $to:tt 411 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 411; $blk }
            unroll! { @for $var $to 410 $blk }
          }
        };
        

        (@for $var:ident $to:tt 412 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 412; $blk }
            unroll! { @for $var $to 411 $blk }
          }
        };
        

        (@for $var:ident $to:tt 413 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 413; $blk }
            unroll! { @for $var $to 412 $blk }
          }
        };
        

        (@for $var:ident $to:tt 414 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 414; $blk }
            unroll! { @for $var $to 413 $blk }
          }
        };
        

        (@for $var:ident $to:tt 415 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 415; $blk }
            unroll! { @for $var $to 414 $blk }
          }
        };
        

        (@for $var:ident $to:tt 416 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 416; $blk }
            unroll! { @for $var $to 415 $blk }
          }
        };
        

        (@for $var:ident $to:tt 417 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 417; $blk }
            unroll! { @for $var $to 416 $blk }
          }
        };
        

        (@for $var:ident $to:tt 418 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 418; $blk }
            unroll! { @for $var $to 417 $blk }
          }
        };
        

        (@for $var:ident $to:tt 419 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 419; $blk }
            unroll! { @for $var $to 418 $blk }
          }
        };
        

        (@for $var:ident $to:tt 420 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 420; $blk }
            unroll! { @for $var $to 419 $blk }
          }
        };
        

        (@for $var:ident $to:tt 421 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 421; $blk }
            unroll! { @for $var $to 420 $blk }
          }
        };
        

        (@for $var:ident $to:tt 422 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 422; $blk }
            unroll! { @for $var $to 421 $blk }
          }
        };
        

        (@for $var:ident $to:tt 423 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 423; $blk }
            unroll! { @for $var $to 422 $blk }
          }
        };
        

        (@for $var:ident $to:tt 424 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 424; $blk }
            unroll! { @for $var $to 423 $blk }
          }
        };
        

        (@for $var:ident $to:tt 425 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 425; $blk }
            unroll! { @for $var $to 424 $blk }
          }
        };
        

        (@for $var:ident $to:tt 426 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 426; $blk }
            unroll! { @for $var $to 425 $blk }
          }
        };
        

        (@for $var:ident $to:tt 427 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 427; $blk }
            unroll! { @for $var $to 426 $blk }
          }
        };
        

        (@for $var:ident $to:tt 428 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 428; $blk }
            unroll! { @for $var $to 427 $blk }
          }
        };
        

        (@for $var:ident $to:tt 429 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 429; $blk }
            unroll! { @for $var $to 428 $blk }
          }
        };
        

        (@for $var:ident $to:tt 430 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 430; $blk }
            unroll! { @for $var $to 429 $blk }
          }
        };
        

        (@for $var:ident $to:tt 431 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 431; $blk }
            unroll! { @for $var $to 430 $blk }
          }
        };
        

        (@for $var:ident $to:tt 432 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 432; $blk }
            unroll! { @for $var $to 431 $blk }
          }
        };
        

        (@for $var:ident $to:tt 433 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 433; $blk }
            unroll! { @for $var $to 432 $blk }
          }
        };
        

        (@for $var:ident $to:tt 434 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 434; $blk }
            unroll! { @for $var $to 433 $blk }
          }
        };
        

        (@for $var:ident $to:tt 435 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 435; $blk }
            unroll! { @for $var $to 434 $blk }
          }
        };
        

        (@for $var:ident $to:tt 436 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 436; $blk }
            unroll! { @for $var $to 435 $blk }
          }
        };
        

        (@for $var:ident $to:tt 437 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 437; $blk }
            unroll! { @for $var $to 436 $blk }
          }
        };
        

        (@for $var:ident $to:tt 438 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 438; $blk }
            unroll! { @for $var $to 437 $blk }
          }
        };
        

        (@for $var:ident $to:tt 439 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 439; $blk }
            unroll! { @for $var $to 438 $blk }
          }
        };
        

        (@for $var:ident $to:tt 440 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 440; $blk }
            unroll! { @for $var $to 439 $blk }
          }
        };
        

        (@for $var:ident $to:tt 441 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 441; $blk }
            unroll! { @for $var $to 440 $blk }
          }
        };
        

        (@for $var:ident $to:tt 442 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 442; $blk }
            unroll! { @for $var $to 441 $blk }
          }
        };
        

        (@for $var:ident $to:tt 443 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 443; $blk }
            unroll! { @for $var $to 442 $blk }
          }
        };
        

        (@for $var:ident $to:tt 444 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 444; $blk }
            unroll! { @for $var $to 443 $blk }
          }
        };
        

        (@for $var:ident $to:tt 445 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 445; $blk }
            unroll! { @for $var $to 444 $blk }
          }
        };
        

        (@for $var:ident $to:tt 446 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 446; $blk }
            unroll! { @for $var $to 445 $blk }
          }
        };
        

        (@for $var:ident $to:tt 447 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 447; $blk }
            unroll! { @for $var $to 446 $blk }
          }
        };
        

        (@for $var:ident $to:tt 448 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 448; $blk }
            unroll! { @for $var $to 447 $blk }
          }
        };
        

        (@for $var:ident $to:tt 449 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 449; $blk }
            unroll! { @for $var $to 448 $blk }
          }
        };
        

        (@for $var:ident $to:tt 450 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 450; $blk }
            unroll! { @for $var $to 449 $blk }
          }
        };
        

        (@for $var:ident $to:tt 451 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 451; $blk }
            unroll! { @for $var $to 450 $blk }
          }
        };
        

        (@for $var:ident $to:tt 452 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 452; $blk }
            unroll! { @for $var $to 451 $blk }
          }
        };
        

        (@for $var:ident $to:tt 453 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 453; $blk }
            unroll! { @for $var $to 452 $blk }
          }
        };
        

        (@for $var:ident $to:tt 454 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 454; $blk }
            unroll! { @for $var $to 453 $blk }
          }
        };
        

        (@for $var:ident $to:tt 455 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 455; $blk }
            unroll! { @for $var $to 454 $blk }
          }
        };
        

        (@for $var:ident $to:tt 456 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 456; $blk }
            unroll! { @for $var $to 455 $blk }
          }
        };
        

        (@for $var:ident $to:tt 457 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 457; $blk }
            unroll! { @for $var $to 456 $blk }
          }
        };
        

        (@for $var:ident $to:tt 458 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 458; $blk }
            unroll! { @for $var $to 457 $blk }
          }
        };
        

        (@for $var:ident $to:tt 459 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 459; $blk }
            unroll! { @for $var $to 458 $blk }
          }
        };
        

        (@for $var:ident $to:tt 460 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 460; $blk }
            unroll! { @for $var $to 459 $blk }
          }
        };
        

        (@for $var:ident $to:tt 461 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 461; $blk }
            unroll! { @for $var $to 460 $blk }
          }
        };
        

        (@for $var:ident $to:tt 462 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 462; $blk }
            unroll! { @for $var $to 461 $blk }
          }
        };
        

        (@for $var:ident $to:tt 463 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 463; $blk }
            unroll! { @for $var $to 462 $blk }
          }
        };
        

        (@for $var:ident $to:tt 464 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 464; $blk }
            unroll! { @for $var $to 463 $blk }
          }
        };
        

        (@for $var:ident $to:tt 465 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 465; $blk }
            unroll! { @for $var $to 464 $blk }
          }
        };
        

        (@for $var:ident $to:tt 466 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 466; $blk }
            unroll! { @for $var $to 465 $blk }
          }
        };
        

        (@for $var:ident $to:tt 467 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 467; $blk }
            unroll! { @for $var $to 466 $blk }
          }
        };
        

        (@for $var:ident $to:tt 468 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 468; $blk }
            unroll! { @for $var $to 467 $blk }
          }
        };
        

        (@for $var:ident $to:tt 469 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 469; $blk }
            unroll! { @for $var $to 468 $blk }
          }
        };
        

        (@for $var:ident $to:tt 470 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 470; $blk }
            unroll! { @for $var $to 469 $blk }
          }
        };
        

        (@for $var:ident $to:tt 471 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 471; $blk }
            unroll! { @for $var $to 470 $blk }
          }
        };
        

        (@for $var:ident $to:tt 472 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 472; $blk }
            unroll! { @for $var $to 471 $blk }
          }
        };
        

        (@for $var:ident $to:tt 473 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 473; $blk }
            unroll! { @for $var $to 472 $blk }
          }
        };
        

        (@for $var:ident $to:tt 474 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 474; $blk }
            unroll! { @for $var $to 473 $blk }
          }
        };
        

        (@for $var:ident $to:tt 475 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 475; $blk }
            unroll! { @for $var $to 474 $blk }
          }
        };
        

        (@for $var:ident $to:tt 476 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 476; $blk }
            unroll! { @for $var $to 475 $blk }
          }
        };
        

        (@for $var:ident $to:tt 477 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 477; $blk }
            unroll! { @for $var $to 476 $blk }
          }
        };
        

        (@for $var:ident $to:tt 478 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 478; $blk }
            unroll! { @for $var $to 477 $blk }
          }
        };
        

        (@for $var:ident $to:tt 479 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 479; $blk }
            unroll! { @for $var $to 478 $blk }
          }
        };
        

        (@for $var:ident $to:tt 480 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 480; $blk }
            unroll! { @for $var $to 479 $blk }
          }
        };
        

        (@for $var:ident $to:tt 481 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 481; $blk }
            unroll! { @for $var $to 480 $blk }
          }
        };
        

        (@for $var:ident $to:tt 482 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 482; $blk }
            unroll! { @for $var $to 481 $blk }
          }
        };
        

        (@for $var:ident $to:tt 483 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 483; $blk }
            unroll! { @for $var $to 482 $blk }
          }
        };
        

        (@for $var:ident $to:tt 484 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 484; $blk }
            unroll! { @for $var $to 483 $blk }
          }
        };
        

        (@for $var:ident $to:tt 485 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 485; $blk }
            unroll! { @for $var $to 484 $blk }
          }
        };
        

        (@for $var:ident $to:tt 486 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 486; $blk }
            unroll! { @for $var $to 485 $blk }
          }
        };
        

        (@for $var:ident $to:tt 487 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 487; $blk }
            unroll! { @for $var $to 486 $blk }
          }
        };
        

        (@for $var:ident $to:tt 488 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 488; $blk }
            unroll! { @for $var $to 487 $blk }
          }
        };
        

        (@for $var:ident $to:tt 489 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 489; $blk }
            unroll! { @for $var $to 488 $blk }
          }
        };
        

        (@for $var:ident $to:tt 490 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 490; $blk }
            unroll! { @for $var $to 489 $blk }
          }
        };
        

        (@for $var:ident $to:tt 491 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 491; $blk }
            unroll! { @for $var $to 490 $blk }
          }
        };
        

        (@for $var:ident $to:tt 492 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 492; $blk }
            unroll! { @for $var $to 491 $blk }
          }
        };
        

        (@for $var:ident $to:tt 493 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 493; $blk }
            unroll! { @for $var $to 492 $blk }
          }
        };
        

        (@for $var:ident $to:tt 494 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 494; $blk }
            unroll! { @for $var $to 493 $blk }
          }
        };
        

        (@for $var:ident $to:tt 495 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 495; $blk }
            unroll! { @for $var $to 494 $blk }
          }
        };
        

        (@for $var:ident $to:tt 496 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 496; $blk }
            unroll! { @for $var $to 495 $blk }
          }
        };
        

        (@for $var:ident $to:tt 497 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 497; $blk }
            unroll! { @for $var $to 496 $blk }
          }
        };
        

        (@for $var:ident $to:tt 498 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 498; $blk }
            unroll! { @for $var $to 497 $blk }
          }
        };
        

        (@for $var:ident $to:tt 499 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 499; $blk }
            unroll! { @for $var $to 498 $blk }
          }
        };
        

        (@for $var:ident $to:tt 500 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 500; $blk }
            unroll! { @for $var $to 499 $blk }
          }
        };
        

        (@for $var:ident $to:tt 501 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 501; $blk }
            unroll! { @for $var $to 500 $blk }
          }
        };
        

        (@for $var:ident $to:tt 502 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 502; $blk }
            unroll! { @for $var $to 501 $blk }
          }
        };
        

        (@for $var:ident $to:tt 503 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 503; $blk }
            unroll! { @for $var $to 502 $blk }
          }
        };
        

        (@for $var:ident $to:tt 504 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 504; $blk }
            unroll! { @for $var $to 503 $blk }
          }
        };
        

        (@for $var:ident $to:tt 505 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 505; $blk }
            unroll! { @for $var $to 504 $blk }
          }
        };
        

        (@for $var:ident $to:tt 506 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 506; $blk }
            unroll! { @for $var $to 505 $blk }
          }
        };
        

        (@for $var:ident $to:tt 507 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 507; $blk }
            unroll! { @for $var $to 506 $blk }
          }
        };
        

        (@for $var:ident $to:tt 508 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 508; $blk }
            unroll! { @for $var $to 507 $blk }
          }
        };
        

        (@for $var:ident $to:tt 509 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 509; $blk }
            unroll! { @for $var $to 508 $blk }
          }
        };
        

        (@for $var:ident $to:tt 510 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 510; $blk }
            unroll! { @for $var $to 509 $blk }
          }
        };
        

        (@for $var:ident $to:tt 511 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 511; $blk }
            unroll! { @for $var $to 510 $blk }
          }
        };
        

        (@for $var:ident $to:tt 512 $blk:block) => {
          #[allow(non_upper_case_globals)]{
            { const $var: usize = $to - 512; $blk }
            unroll! { @for $var $to 511 $blk }
          }
        };
        
  }


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


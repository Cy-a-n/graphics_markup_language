#[allow(unused)]
macro_rules! finite_automata_match_input {
    ($value:expr, $($pattern:pat => $next_state:expr,)+) => {
        match $value {
            $(
                $pattern => $next_state,
            )+
            _ => {
                panic!("Expected values: {}", stringify!($($pattern),+));
            }
        }
    };
}

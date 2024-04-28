/// # Transition
/// Represents a transition in the parser.
macro_rules! transition {
    ($tokens:expr, $( $expected_input:pat => $next_state:expr ),* $(,)?) => {
        {
            match $tokens.next() {
                None => UnexpectedEnd(vec![$($crate::token::TokenValue::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*]),
                Some((i, token)) => {
                   match token.value() {
                   $( $expected_input => $next_state, )*
                   _ => UnexpectedToken(vec![$($crate::token::TokenValue::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*], i),
                   }
                }
            }
        }
    };
}

/// # Transition
/// Represents a transition in the parser that calls `tokens.peek()` instead of `tokens.next()`.
/// Useful if another function called in transition will advance `tokens`.
macro_rules! transition_peek {
    ($tokens:expr, $( $expected_input:pat => $next_state:expr ),* $(,)?) => {
        {
            match $tokens.peek() {
                None => UnexpectedEnd(vec![$($crate::token::TokenValue::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*]),
                Some((i, token)) => {
                   match token.value() {
                    $( $expected_input => $next_state, )*
                   _ => UnexpectedToken(vec![$($crate::token::TokenValue::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*], *i),
                   }
                }
            }
        }
    };
}

/// # Transition
/// Represents a transition in the parser. If `$input` is matches on the catchall `_` this macro return an `Return` state instead of an `UnexpectedToken` state.
/// Useful if the finite automate has optional characters in this state.
macro_rules! transition_return_on_unexpected {
    ($tokens:expr, $value: expr, $( $expected_input:pat => $next_state:expr ),* $(,)?) => {
        {
            match $tokens.peek() {
                None => UnexpectedEnd(vec![$($crate::token::TokenValue::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*]),
                Some((_, token)) => {
                   match token.value() {
                    $(
                        $expected_input => {
                            $tokens.next();
                            $next_state
                        }
                    )*
                   _ => Return($value),
                   }
                }
            }
        }
    };
}

pub(crate) use {transition, transition_peek, transition_return_on_unexpected};

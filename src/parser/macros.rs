macro_rules! sub_parser_branch {
    ($tokens:expr, $( $expected_input:pat => $next_state:expr ),* $(,)?) => {
        {
            match $tokens.peek() {
                None => TokensUnexpectedEnd(vec![$($crate::token::Value::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*]),
                Some((_, token)) => {
                   match token.value() {
                   $(
                       $expected_input => {
                            $tokens.next();
                            $next_state
                       },
                   )*
                   _ => UnexpectedToken(vec![$($crate::token::Value::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*]),
                   }
                }
            }
        }
    };
}

macro_rules! sub_parser_return_branch {
    ($tokens:expr, $value: expr, $( $expected_input:pat => $next_state:expr ),* $(,)?) => {
        {
            match $tokens.peek() {
                None => TokensUnexpectedEnd(vec![$($crate::token::Value::from_str(stringify!($expected_input)).unwrap_or_else(|invalid_token| panic!("BUG: Could not convert pattern {invalid_token} to token."))),*]),
                Some((_, token)) => {
                   match token.value() {
                   $(
                       $expected_input => {
                           $tokens.next();
                           $next_state
                       },
                   )*
                   _ => End($value),
                   }
                }
            }
        }
    };
}

pub(crate) use {sub_parser_branch, sub_parser_return_branch};

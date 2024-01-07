use crate::token::Token;
use crate::token::TokenValue;
use crate::token::TokenValue::*;
use State::*;

pub fn to_tokens(actual_tokens_iter: &str) -> Vec<Token> {
    let actual_tokens = actual_tokens_iter.lines();
    let mut tokens = vec![];
    let mut current_state = Start;

    for (line, actual_tokens_iter) in actual_tokens.enumerate() {
        for (offset, char) in actual_tokens_iter.chars().enumerate() {
            let (next_state, token_value) = current_state.next_state(line, offset, &char);

            if let Some(token_value) = token_value {
                tokens.push(Token::new(
                    line,
                    offset + 1 - token_value.length(),
                    token_value,
                ));
            }

            current_state = next_state;
        }
    }

    // Subtract the length of the tokens from their offset

    tokens
}

fn lexer_panic(expected: Vec<char>, state: &State, char: &char, line: &usize, offset: &usize) -> ! {
    panic!(
        "Lexer error: Expected `{:?}` in state `{:?}`, got `{}` in line {}, offset {}.",
        expected, state, char, &line, &offset
    )
}

#[derive(Debug)]
enum State {
    Start,
    #[allow(non_camel_case_types)]
    String_r,
    #[allow(non_camel_case_types)]
    String_re,
    #[allow(non_camel_case_types)]
    String_g,
    #[allow(non_camel_case_types)]
    String_gr,
    #[allow(non_camel_case_types)]
    String_gre,
    #[allow(non_camel_case_types)]
    String_gree,
    #[allow(non_camel_case_types)]
    String_b,
    #[allow(non_camel_case_types)]
    String_bl,
    #[allow(non_camel_case_types)]
    String_blu,
    #[allow(non_camel_case_types)]
    String_p,
    #[allow(non_camel_case_types)]
    String_po,
    #[allow(non_camel_case_types)]
    String_pos,
    #[allow(non_camel_case_types)]
    String_posi,
    #[allow(non_camel_case_types)]
    String_posit,
    #[allow(non_camel_case_types)]
    String_positi,
    #[allow(non_camel_case_types)]
    String_positio,
    #[allow(non_camel_case_types)]
    String_ro,
    #[allow(non_camel_case_types)]
    String_rot,
    #[allow(non_camel_case_types)]
    String_rota,
    #[allow(non_camel_case_types)]
    String_rotat,
    #[allow(non_camel_case_types)]
    String_rotati,
    #[allow(non_camel_case_types)]
    String_rotatio,
    #[allow(non_camel_case_types)]
    String_w,
    #[allow(non_camel_case_types)]
    String_wi,
    #[allow(non_camel_case_types)]
    String_wid,
    #[allow(non_camel_case_types)]
    String_widt,
    #[allow(non_camel_case_types)]
    String_bo,
    #[allow(non_camel_case_types)]
    String_bor,
    #[allow(non_camel_case_types)]
    String_bord,
    #[allow(non_camel_case_types)]
    String_borde,
    #[allow(non_camel_case_types)]
    String_border,
    #[allow(non_camel_case_types)]
    String_border_,
    #[allow(non_camel_case_types)]
    String_border_c,
    #[allow(non_camel_case_types)]
    String_border_co,
    #[allow(non_camel_case_types)]
    String_border_col,
    #[allow(non_camel_case_types)]
    String_border_colo,
    #[allow(non_camel_case_types)]
    String_f,
    #[allow(non_camel_case_types)]
    String_fi,
    #[allow(non_camel_case_types)]
    String_fil,
    #[allow(non_camel_case_types)]
    String_fill,
    #[allow(non_camel_case_types)]
    String_fill_,
    #[allow(non_camel_case_types)]
    String_fill_c,
    #[allow(non_camel_case_types)]
    String_fill_co,
    #[allow(non_camel_case_types)]
    String_fill_col,
    #[allow(non_camel_case_types)]
    String_fill_colo,
    #[allow(non_camel_case_types)]
    String_v,
    #[allow(non_camel_case_types)]
    String_ve,
    #[allow(non_camel_case_types)]
    String_ver,
    #[allow(non_camel_case_types)]
    String_vert,
    #[allow(non_camel_case_types)]
    String_verti,
    #[allow(non_camel_case_types)]
    String_vertic,
    #[allow(non_camel_case_types)]
    String_vertice,
    #[allow(non_camel_case_types)]
    String_vi,
    #[allow(non_camel_case_types)]
    String_vis,
    #[allow(non_camel_case_types)]
    String_visi,
    #[allow(non_camel_case_types)]
    String_visib,
    #[allow(non_camel_case_types)]
    String_visibl,
    #[allow(non_camel_case_types)]
    String_visible,
    #[allow(non_camel_case_types)]
    String_visible_,
    #[allow(non_camel_case_types)]
    String_visible_e,
    #[allow(non_camel_case_types)]
    String_visible_ex,
    #[allow(non_camel_case_types)]
    String_visible_ext,
    #[allow(non_camel_case_types)]
    String_visible_exte,
    #[allow(non_camel_case_types)]
    String_visible_exten,
    #[allow(non_camel_case_types)]
    String_ba,
    #[allow(non_camel_case_types)]
    String_bac,
    #[allow(non_camel_case_types)]
    String_back,
    #[allow(non_camel_case_types)]
    String_backg,
    #[allow(non_camel_case_types)]
    String_backgr,
    #[allow(non_camel_case_types)]
    String_backgro,
    #[allow(non_camel_case_types)]
    String_backgrou,
    #[allow(non_camel_case_types)]
    String_backgroun,
    #[allow(non_camel_case_types)]
    String_background,
    #[allow(non_camel_case_types)]
    String_background_,
    #[allow(non_camel_case_types)]
    String_background_c,
    #[allow(non_camel_case_types)]
    String_background_co,
    #[allow(non_camel_case_types)]
    String_background_col,
    #[allow(non_camel_case_types)]
    String_background_colo,
    #[allow(non_camel_case_types)]
    String_s,
    #[allow(non_camel_case_types)]
    String_sh,
    #[allow(non_camel_case_types)]
    String_sha,
    #[allow(non_camel_case_types)]
    String_shap,
    #[allow(non_camel_case_types)]
    String_shape,
}

impl State {
    fn next_state(&self, line: usize, offset: usize, char: &char) -> (Self, Option<TokenValue>) {
        match self {
            Start => match char {
                c if c.is_whitespace() => (Start, None),
                '=' => (Start, Some(PrimitiveValue)),
                '-' => (Start, Some(NegativeValue)),
                '+' => (Start, Some(PositiveValue)),
                '0' => (Start, Some(Zero)),
                '1' => (Start, Some(One)),
                '{' => (Start, Some(StructStart)),
                '}' => (Start, Some(StructEnd)),
                '[' => (Start, Some(ArrayStart)),
                ']' => (Start, Some(ArrayEnd)),
                'r' => (String_r, None),
                'g' => (String_g, None),
                'b' => (String_b, None),
                'x' => (Start, Some(AttributX)),
                'y' => (Start, Some(AttributY)),
                'p' => (String_p, None),
                'w' => (String_w, None),
                'f' => (String_f, None),
                'v' => (String_v, None),
                's' => (String_s, None),
                _ => lexer_panic(
                    vec![
                        ' ', '=', '-', '+', '0', '1', '{', '}', '[', ']', 'r', 'g', 'b', 'x', 'y',
                        'p', 'w', 'f', 'v', 's',
                    ],
                    self,
                    char,
                    &line,
                    &offset,
                ),
            },
            String_r => match char {
                'e' => (String_re, None),
                'o' => (String_ro, None),
                _ => lexer_panic(vec!['e', 'o'], &self, char, &line, &offset),
            },
            String_re => match char {
                'd' => (Start, Some(AttributRed)),
                _ => lexer_panic(vec!['d'], &self, char, &line, &offset),
            },
            String_g => match char {
                'r' => (String_gr, None),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_gr => match char {
                'e' => (String_gre, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_gre => match char {
                'e' => (String_gree, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_gree => match char {
                'n' => (Start, Some(AttributGreen)),
                _ => lexer_panic(vec!['n'], &self, char, &line, &offset),
            },
            String_b => match char {
                'l' => (String_bl, None),
                'o' => (String_bo, None),
                'a' => (String_ba, None),
                _ => lexer_panic(vec!['l', 'o', 'a'], &self, char, &line, &offset),
            },
            String_bl => match char {
                'u' => (String_blu, None),
                _ => lexer_panic(vec!['u'], &self, char, &line, &offset),
            },
            String_blu => match char {
                'e' => (Start, Some(AttributBlue)),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_p => match char {
                'o' => (String_po, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_po => match char {
                's' => (String_pos, None),
                _ => lexer_panic(vec!['s'], &self, char, &line, &offset),
            },
            String_pos => match char {
                'i' => (String_posi, None),
                _ => lexer_panic(vec!['i'], &self, char, &line, &offset),
            },
            String_posi => match char {
                't' => (String_posit, None),
                _ => lexer_panic(vec!['t'], &self, char, &line, &offset),
            },
            String_posit => match char {
                'i' => (String_positi, None),
                _ => lexer_panic(vec!['i'], &self, char, &line, &offset),
            },
            String_positi => match char {
                'o' => (String_positio, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_positio => match char {
                'n' => (Start, Some(AttributPosition)),
                _ => lexer_panic(vec!['n'], &self, char, &line, &offset),
            },
            String_ro => match char {
                't' => (String_rot, None),
                _ => lexer_panic(vec!['t'], &self, char, &line, &offset),
            },
            String_rot => match char {
                'a' => (String_rota, None),
                _ => lexer_panic(vec!['a'], &self, char, &line, &offset),
            },
            String_rota => match char {
                't' => (String_rotat, None),
                _ => lexer_panic(vec!['t'], &self, char, &line, &offset),
            },
            String_rotat => match char {
                'i' => (String_rotati, None),
                _ => lexer_panic(vec!['i'], &self, char, &line, &offset),
            },
            String_rotati => match char {
                'o' => (String_rotatio, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_rotatio => match char {
                'n' => (Start, Some(AttributRotation)),
                _ => lexer_panic(vec!['n'], &self, char, &line, &offset),
            },
            String_w => match char {
                'i' => (String_wi, None),
                _ => lexer_panic(vec!['i'], &self, char, &line, &offset),
            },
            String_wi => match char {
                'd' => (String_wid, None),
                _ => lexer_panic(vec!['d'], &self, char, &line, &offset),
            },
            String_wid => match char {
                't' => (String_widt, None),
                _ => lexer_panic(vec!['t'], &self, char, &line, &offset),
            },
            String_widt => match char {
                'h' => (Start, Some(AttributWidth)),
                _ => lexer_panic(vec!['h'], &self, char, &line, &offset),
            },
            String_bo => match char {
                'r' => (String_bor, None),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_bor => match char {
                'd' => (String_bord, None),
                _ => lexer_panic(vec!['d'], &self, char, &line, &offset),
            },
            String_bord => match char {
                'e' => (String_borde, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_borde => match char {
                'r' => (String_border, None),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_border => match char {
                '_' => (String_border_, None),
                _ => lexer_panic(vec!['_'], &self, char, &line, &offset),
            },
            String_border_ => match char {
                'c' => (String_border_c, None),
                _ => lexer_panic(vec!['c'], &self, char, &line, &offset),
            },
            String_border_c => match char {
                'o' => (String_border_co, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_border_co => match char {
                'l' => (String_border_col, None),
                _ => lexer_panic(vec!['l'], &self, char, &line, &offset),
            },
            String_border_col => match char {
                'o' => (String_border_colo, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_border_colo => match char {
                'r' => (Start, Some(AttributBorderColor)),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_f => match char {
                'i' => (String_fi, None),
                _ => lexer_panic(vec!['i'], &self, char, &line, &offset),
            },
            String_fi => match char {
                'l' => (String_fil, None),
                _ => lexer_panic(vec!['l'], &self, char, &line, &offset),
            },
            String_fil => match char {
                'l' => (String_fill, None),
                _ => lexer_panic(vec!['l'], &self, char, &line, &offset),
            },
            String_fill => match char {
                '_' => (String_fill_, None),
                _ => lexer_panic(vec!['_'], &self, char, &line, &offset),
            },
            String_fill_ => match char {
                'c' => (String_fill_c, None),
                _ => lexer_panic(vec!['c'], &self, char, &line, &offset),
            },
            String_fill_c => match char {
                'o' => (String_fill_co, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_fill_co => match char {
                'l' => (String_fill_col, None),
                _ => lexer_panic(vec!['l'], &self, char, &line, &offset),
            },
            String_fill_col => match char {
                'o' => (String_fill_colo, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_fill_colo => match char {
                'r' => (Start, Some(AttributFillColor)),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_v => match char {
                'e' => (String_ve, None),
                'i' => (String_vi, None),
                _ => lexer_panic(vec!['e', 'i'], &self, char, &line, &offset),
            },
            String_ve => match char {
                'r' => (String_ver, None),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_ver => match char {
                't' => (String_vert, None),
                _ => lexer_panic(vec!['t'], &self, char, &line, &offset),
            },
            String_vert => match char {
                'i' => (String_verti, None),
                _ => lexer_panic(vec!['i'], &self, char, &line, &offset),
            },
            String_verti => match char {
                'c' => (String_vertic, None),
                _ => lexer_panic(vec!['c'], &self, char, &line, &offset),
            },
            String_vertic => match char {
                'e' => (String_vertice, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_vertice => match char {
                's' => (Start, Some(AttributVertices)),
                _ => lexer_panic(vec!['s'], &self, char, &line, &offset),
            },
            String_vi => match char {
                's' => (String_vis, None),
                _ => lexer_panic(vec!['s'], &self, char, &line, &offset),
            },
            String_vis => match char {
                'i' => (String_visi, None),
                _ => lexer_panic(vec!['i'], &self, char, &line, &offset),
            },
            String_visi => match char {
                'b' => (String_visib, None),
                _ => lexer_panic(vec!['b'], &self, char, &line, &offset),
            },
            String_visib => match char {
                'l' => (String_visibl, None),
                _ => lexer_panic(vec!['l'], &self, char, &line, &offset),
            },
            String_visibl => match char {
                'e' => (String_visible, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_visible => match char {
                '_' => (String_visible_, None),
                _ => lexer_panic(vec!['_'], &self, char, &line, &offset),
            },
            String_visible_ => match char {
                'e' => (String_visible_e, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_visible_e => match char {
                'x' => (String_visible_ex, None),
                _ => lexer_panic(vec!['x'], &self, char, &line, &offset),
            },
            String_visible_ex => match char {
                't' => (String_visible_ext, None),
                _ => lexer_panic(vec!['t'], &self, char, &line, &offset),
            },
            String_visible_ext => match char {
                'e' => (String_visible_exte, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_visible_exte => match char {
                'n' => (String_visible_exten, None),
                _ => lexer_panic(vec!['n'], &self, char, &line, &offset),
            },
            String_visible_exten => match char {
                't' => (Start, Some(AttributVisibleExtent)),
                _ => lexer_panic(vec!['t'], &self, char, &line, &offset),
            },
            String_ba => match char {
                'c' => (String_bac, None),
                _ => lexer_panic(vec!['c'], &self, char, &line, &offset),
            },
            String_bac => match char {
                'k' => (String_back, None),
                _ => lexer_panic(vec!['k'], &self, char, &line, &offset),
            },
            String_back => match char {
                'g' => (String_backg, None),
                _ => lexer_panic(vec!['g'], &self, char, &line, &offset),
            },
            String_backg => match char {
                'r' => (String_backgr, None),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_backgr => match char {
                'o' => (String_backgro, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_backgro => match char {
                'u' => (String_backgrou, None),
                _ => lexer_panic(vec!['u'], &self, char, &line, &offset),
            },
            String_backgrou => match char {
                'n' => (String_backgroun, None),
                _ => lexer_panic(vec!['n'], &self, char, &line, &offset),
            },
            String_backgroun => match char {
                'd' => (String_background, None),
                _ => lexer_panic(vec!['d'], &self, char, &line, &offset),
            },
            String_background => match char {
                '_' => (String_background_, None),
                _ => lexer_panic(vec!['c'], &self, char, &line, &offset),
            },
            String_background_ => match char {
                'c' => (String_background_c, None),
                _ => lexer_panic(vec!['c'], &self, char, &line, &offset),
            },
            String_background_c => match char {
                'o' => (String_background_co, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_background_co => match char {
                'l' => (String_background_col, None),
                _ => lexer_panic(vec!['l'], &self, char, &line, &offset),
            },
            String_background_col => match char {
                'o' => (String_background_colo, None),
                _ => lexer_panic(vec!['o'], &self, char, &line, &offset),
            },
            String_background_colo => match char {
                'r' => (Start, Some(AttributBackgroundColor)),
                _ => lexer_panic(vec!['r'], &self, char, &line, &offset),
            },
            String_s => match char {
                'h' => (String_sh, None),
                _ => lexer_panic(vec!['h'], &self, char, &line, &offset),
            },
            String_sh => match char {
                'a' => (String_sha, None),
                _ => lexer_panic(vec!['a'], &self, char, &line, &offset),
            },
            String_sha => match char {
                'p' => (String_shap, None),
                _ => lexer_panic(vec!['p'], &self, char, &line, &offset),
            },
            String_shap => match char {
                'e' => (String_shape, None),
                _ => lexer_panic(vec!['e'], &self, char, &line, &offset),
            },
            String_shape => match char {
                's' => (Start, Some(AttributShapes)),
                _ => lexer_panic(vec!['s'], &self, char, &line, &offset),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenValue;

    use super::*;
    use crate::utils::is_same_variant;

    fn err_actual_tokens_iter_ends(
        expected_token: &TokenValue,
        index: &usize,
        actual_tokens: &Vec<Token>,
        expected_tokens: &Vec<TokenValue>,
    ) -> Result<(), String> {
        Result::Err(format!(
            "`actual_tokens_iter` ended before `expected_tokens_iter`.\n`expected_token`: `{:?}`,\n`index`: {},\n`actual_tokens`: `{:?}`,\n`expected_tokens`: `{:?}`.",
            expected_token,
            index,
            actual_tokens,
            expected_tokens
        ))
    }

    fn err_expected_tokens_iter_ends(
        actual_token: &Token,
        index: &usize,
        actual_tokens: &Vec<Token>,
        expected_tokens: &Vec<TokenValue>,
    ) -> Result<(), String> {
        Result::Err(format!(
            "`expected_tokens_iter` ended before `actual_tokens_iter`.\n`actual_token`: `{:?}`,\n`index`: {},\n`actual_tokens`: `{:?}`,\n`expected_tokens`: `{:?}`.",
            &actual_token,
            index,
            actual_tokens,
            expected_tokens
        ))
    }

    fn err_token_mismatch(
        actual_token: &Token,
        expected_token: &TokenValue,
        index: &usize,
        actual_tokens: &Vec<Token>,
        expected_tokens: &Vec<TokenValue>,
    ) -> Result<(), String> {
        Result::Err(format!("`actual_token` does not equal `current_token`.\n `actual_token`: `{:?}`, `expected_token`: `{:?}`,\n`index`: {},\n`actual_tokens`: `{:?}`,\n`expected_tokens`: `{:?}`.", actual_token, expected_token,
        index,
        actual_tokens,
        expected_tokens))
    }

    #[test]
    fn minimal_source_code() -> Result<(), String> {
        let actual_tokens = to_tokens("{visible_extentbackground_colorshapes}");
        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            AttributBackgroundColor,
            AttributShapes,
            StructEnd,
        ];

        let mut actual_tokens_iter = actual_tokens.iter();
        let mut expected_tokens_iter = expected_tokens.iter();

        let mut index: usize = 0;
        loop {
            let actual_token = actual_tokens_iter.next();
            let expected_token = expected_tokens_iter.next();

            match (actual_token, expected_token) {
                (None, None) => return Ok(()),
                (None, Some(expected_token)) => {
                    return err_actual_tokens_iter_ends(
                        expected_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), None) => {
                    return err_expected_tokens_iter_ends(
                        actual_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), Some(expected_token)) => {
                    if !is_same_variant(actual_token.value(), &expected_token) {
                        return err_token_mismatch(
                            actual_token,
                            expected_token,
                            &index,
                            &actual_tokens,
                            &expected_tokens,
                        );
                    }
                }
            };
            index += 1;
        }
    }

    #[test]
    fn custom_visible_extent() -> Result<(), String> {
        let actual_tokens =
            to_tokens("{visible_extent{x=+10001111y=-001010101111111}background_colorshapes}");
        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            One,
            Zero,
            Zero,
            Zero,
            One,
            One,
            One,
            One,
            AttributY,
            PrimitiveValue,
            NegativeValue,
            Zero,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
            AttributBackgroundColor,
            AttributShapes,
            StructEnd,
        ];

        let mut actual_tokens_iter = actual_tokens.iter();
        let mut expected_tokens_iter = expected_tokens.iter();

        let mut index: usize = 0;
        loop {
            let actual_token = actual_tokens_iter.next();
            let expected_token = expected_tokens_iter.next();

            match (actual_token, expected_token) {
                (None, None) => return Ok(()),
                (None, Some(expected_token)) => {
                    return err_actual_tokens_iter_ends(
                        expected_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), None) => {
                    return err_expected_tokens_iter_ends(
                        actual_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), Some(expected_token)) => {
                    if !is_same_variant(actual_token.value(), &expected_token) {
                        return err_token_mismatch(
                            actual_token,
                            expected_token,
                            &index,
                            &actual_tokens,
                            &expected_tokens,
                        );
                    }
                }
            };
            index += 1;
        }
    }

    #[test]
    fn custom_background_color() -> Result<(), String> {
        let actual_tokens =
            to_tokens("{visible_extentbackground_color{red=11111111green=00000000blue=0}shapes}");
        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            AttributBackgroundColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            AttributBlue,
            PrimitiveValue,
            Zero,
            StructEnd,
            AttributShapes,
            StructEnd,
        ];

        let mut actual_tokens_iter = actual_tokens.iter();
        let mut expected_tokens_iter = expected_tokens.iter();

        let mut index: usize = 0;
        loop {
            let actual_token = actual_tokens_iter.next();
            let expected_token = expected_tokens_iter.next();

            match (actual_token, expected_token) {
                (None, None) => return Ok(()),
                (None, Some(expected_token)) => {
                    return err_actual_tokens_iter_ends(
                        expected_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), None) => {
                    return err_expected_tokens_iter_ends(
                        actual_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), Some(expected_token)) => {
                    if !is_same_variant(actual_token.value(), &expected_token) {
                        return err_token_mismatch(
                            actual_token,
                            expected_token,
                            &index,
                            &actual_tokens,
                            &expected_tokens,
                        );
                    }
                }
            };
            index += 1;
        }
    }

    #[test]
    fn custom_full() -> Result<(), String> {
        let actual_tokens = to_tokens(
            r#"
{
    visible_extent {
        x = -10
        y = 11111
    }
    background_color {
        red = 1
        green = 0
        blue = 11
    }
    shapes [
        {
            position
            rotation
            width
            border_color
            fill_color
            vertices
        }
        {
            position {
                x = +0
                y = -0
            }
            rotation = 0
            shapes [ 
                { 
                    position { 
                        x = 1 
                        y = 0
                    }
                    rotation = 11
                    width = -111111111111111
                    border_color { 
                        red = 1
                        green = 0
                        blue = 11
                    }
                    fill_color {
                        red = 11111111
                        green = 0
                        blue = 10
                    }
                    vertices [ 
                        {
                            x = -10
                            y = 10
                        }
                        {
                            x = -1
                            y=1
                        }
                    ]
                }
                {
                    position
                    rotation
                    width
                    border_color
                    fill_color
                    vertices
                }
            ]
        }
        {
            position {
                x = +0
                y = -1
            }
            rotation = 1
            width = -10
            border_color {
                red = 1
                green = 0
                blue = 101
            }
            fill_color {
                red = 0
                green = 1
                blue = 10
            }
            vertices [
                {
                    
                    x = 1010
                    y = 10101
                }
                {
                    x = 101010
                    y = 1010101
                }
            ]
        }
        {
            position
            rotation
            shapes
        }
    ]
}"#,
        );

        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativeValue,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
            AttributBackgroundColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            One,
            StructEnd,
            AttributShapes,
            ArrayStart,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributWidth,
            AttributBorderColor,
            AttributFillColor,
            AttributVertices,
            StructEnd,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            Zero,
            AttributY,
            PrimitiveValue,
            NegativeValue,
            Zero,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            Zero,
            AttributShapes,
            ArrayStart,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            AttributY,
            PrimitiveValue,
            Zero,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            One,
            One,
            AttributWidth,
            PrimitiveValue,
            NegativeValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributBorderColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            One,
            StructEnd,
            AttributFillColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            AttributVertices,
            ArrayStart,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativeValue,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativeValue,
            One,
            AttributY,
            PrimitiveValue,
            One,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributWidth,
            AttributBorderColor,
            AttributFillColor,
            AttributVertices,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            Zero,
            AttributY,
            PrimitiveValue,
            NegativeValue,
            One,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            One,
            AttributWidth,
            PrimitiveValue,
            NegativeValue,
            One,
            Zero,
            AttributBorderColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            One,
            StructEnd,
            AttributFillColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            Zero,
            AttributGreen,
            PrimitiveValue,
            One,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            AttributVertices,
            ArrayStart,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            StructEnd,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributShapes,
            StructEnd,
            ArrayEnd,
            StructEnd,
        ];

        let mut actual_tokens_iter = actual_tokens.iter();
        let mut expected_tokens_iter = expected_tokens.iter();

        let mut index: usize = 0;
        loop {
            let actual_token = actual_tokens_iter.next();
            let expected_token = expected_tokens_iter.next();

            match (actual_token, expected_token) {
                (None, None) => return Ok(()),
                (None, Some(expected_token)) => {
                    return err_actual_tokens_iter_ends(
                        expected_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), None) => {
                    return err_expected_tokens_iter_ends(
                        actual_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), Some(expected_token)) => {
                    if !is_same_variant(actual_token.value(), &expected_token) {
                        return err_token_mismatch(
                            actual_token,
                            expected_token,
                            &index,
                            &actual_tokens,
                            &expected_tokens,
                        );
                    }
                }
            };
            index += 1;
        }
    }
}

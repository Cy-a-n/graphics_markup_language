use crate::token::Token;
use crate::token::Token::*;
use State::*;

pub fn to_tokens(actual_tokens: &str) -> Vec<Token> {
    let actual_tokens = actual_tokens.lines();
    let mut tokens: Vec<Token> = vec![];
    let mut current_state = Start;

    for (line, actual_tokens) in actual_tokens.enumerate() {
        for (offset, char) in actual_tokens.chars().enumerate() {
            let (next_state, output) = current_state.next_state(line, offset, &char);

            if let Err(expected) = next_state {
                panic!(
                    "Lexer error: Expected {:?} in state {:?}, got {}.",
                    expected, current_state, char
                )
            }

            if let Some(token) = output {
                tokens.push(token);
            }

            current_state = next_state;
        }
    }

    tokens
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
    Err(Vec<char>),
}

impl State {
    fn next_state(&self, line: usize, offset: usize, char: &char) -> (Self, Option<Token>) {
        match self {
            Err(_) => {
                panic!("The `next_state`-method should never be called on State::Err")
            }
            Start => match char {
                c if c.is_whitespace() => (Start, None),
                '=' => (Start, Some(PrimitiveValue(line, offset))),
                '-' => (Start, Some(NegativeValue(line, offset))),
                '+' => (Start, Some(PositiveValue(line, offset))),
                '0' => (Start, Some(Zero(line, offset))),
                '1' => (Start, Some(One(line, offset))),
                '{' => (Start, Some(StructStart(line, offset))),
                '}' => (Start, Some(StructEnd(line, offset))),
                '[' => (Start, Some(ArrayStart(line, offset))),
                ']' => (Start, Some(ArrayEnd(line, offset))),
                'r' => (String_r, None),
                'g' => (String_g, None),
                'b' => (String_b, None),
                'x' => (Start, Some(AttributX(line, offset))),
                'y' => (Start, Some(AttributY(line, offset))),
                'p' => (String_p, None),
                'w' => (String_w, None),
                'f' => (String_f, None),
                'v' => (String_v, None),
                's' => (String_s, None),
                _ => (
                    Err(vec![
                        ' ', '=', '-', '+', '0', '1', '{', '}', '[', ']', 'r', 'g', 'b', 'x', 'y',
                        'p', 'w', 'f', 'v', 's',
                    ]),
                    None,
                ),
            },
            String_r => match char {
                'e' => (String_re, None),
                'o' => (String_ro, None),
                _ => (Err(vec!['e', 'o']), None),
            },
            String_re => match char {
                'd' => (Start, Some(AttributRed(line, offset))),
                _ => (Err(vec!['d']), None),
            },
            String_g => match char {
                'r' => (String_gr, None),
                _ => (Err(vec!['r']), None),
            },
            String_gr => match char {
                'e' => (String_gre, None),
                _ => (Err(vec!['e']), None),
            },
            String_gre => match char {
                'e' => (String_gree, None),
                _ => (Err(vec!['e']), None),
            },
            String_gree => match char {
                'n' => (Start, Some(AttributGreen(line, offset))),
                _ => (Err(vec!['n']), None),
            },
            String_b => match char {
                'l' => (String_bl, None),
                'o' => (String_bo, None),
                'a' => (String_ba, None),
                _ => (Err(vec!['l', 'o', 'a']), None),
            },
            String_bl => match char {
                'u' => (String_blu, None),
                _ => (Err(vec!['u']), None),
            },
            String_blu => match char {
                'e' => (Start, Some(AttributBlue(line, offset))),
                _ => (Err(vec!['e']), None),
            },
            String_p => match char {
                'o' => (String_po, None),
                _ => (Err(vec!['o']), None),
            },
            String_po => match char {
                's' => (String_pos, None),
                _ => (Err(vec!['s']), None),
            },
            String_pos => match char {
                'i' => (String_posi, None),
                _ => (Err(vec!['i']), None),
            },
            String_posi => match char {
                't' => (String_posit, None),
                _ => (Err(vec!['t']), None),
            },
            String_posit => match char {
                'i' => (String_positi, None),
                _ => (Err(vec!['i']), None),
            },
            String_positi => match char {
                'o' => (String_positio, None),
                _ => (Err(vec!['o']), None),
            },
            String_positio => match char {
                'n' => (Start, Some(AttributPosition(line, offset))),
                _ => (Err(vec!['n']), None),
            },
            String_ro => match char {
                't' => (String_rot, None),
                _ => (Err(vec!['t']), None),
            },
            String_rot => match char {
                'a' => (String_rota, None),
                _ => (Err(vec!['a']), None),
            },
            String_rota => match char {
                't' => (String_rotat, None),
                _ => (Err(vec!['t']), None),
            },
            String_rotat => match char {
                'i' => (String_rotati, None),
                _ => (Err(vec!['i']), None),
            },
            String_rotati => match char {
                'o' => (String_rotatio, None),
                _ => (Err(vec!['o']), None),
            },
            String_rotatio => match char {
                'n' => (Start, Some(AttributRotation(line, offset))),
                _ => (Err(vec!['n']), None),
            },
            String_w => match char {
                'i' => (String_wi, None),
                _ => (Err(vec!['i']), None),
            },
            String_wi => match char {
                'd' => (String_wid, None),
                _ => (Err(vec!['d']), None),
            },
            String_wid => match char {
                't' => (String_widt, None),
                _ => (Err(vec!['t']), None),
            },
            String_widt => match char {
                'h' => (Start, Some(AttributWidth(line, offset))),
                _ => (Err(vec!['h']), None),
            },
            String_bo => match char {
                'r' => (String_bor, None),
                _ => (Err(vec!['r']), None),
            },
            String_bor => match char {
                'd' => (String_bord, None),
                _ => (Err(vec!['d']), None),
            },
            String_bord => match char {
                'e' => (String_borde, None),
                _ => (Err(vec!['e']), None),
            },
            String_borde => match char {
                'r' => (String_border, None),
                _ => (Err(vec!['r']), None),
            },
            String_border => match char {
                '_' => (String_border_, None),
                _ => (Err(vec!['_']), None),
            },
            String_border_ => match char {
                'c' => (String_border_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_border_c => match char {
                'o' => (String_border_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_border_co => match char {
                'l' => (String_border_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_border_col => match char {
                'o' => (String_border_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_border_colo => match char {
                'r' => (Start, Some(AttributBorderColor(line, offset))),
                _ => (Err(vec!['r']), None),
            },
            String_f => match char {
                'i' => (String_fi, None),
                _ => (Err(vec!['i']), None),
            },
            String_fi => match char {
                'l' => (String_fil, None),
                _ => (Err(vec!['l']), None),
            },
            String_fil => match char {
                'l' => (String_fill, None),
                _ => (Err(vec!['l']), None),
            },
            String_fill => match char {
                '_' => (String_fill_, None),
                _ => (Err(vec!['_']), None),
            },
            String_fill_ => match char {
                'c' => (String_fill_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_fill_c => match char {
                'o' => (String_fill_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_fill_co => match char {
                'l' => (String_fill_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_fill_col => match char {
                'o' => (String_fill_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_fill_colo => match char {
                'r' => (Start, Some(AttributFillColor(line, offset))),
                _ => (Err(vec!['r']), None),
            },
            String_v => match char {
                'e' => (String_ve, None),
                'i' => (String_vi, None),
                _ => (Err(vec!['e', 'i']), None),
            },
            String_ve => match char {
                'r' => (String_ver, None),
                _ => (Err(vec!['r']), None),
            },
            String_ver => match char {
                't' => (String_vert, None),
                _ => (Err(vec!['t']), None),
            },
            String_vert => match char {
                'i' => (String_verti, None),
                _ => (Err(vec!['i']), None),
            },
            String_verti => match char {
                'c' => (String_vertic, None),
                _ => (Err(vec!['c']), None),
            },
            String_vertic => match char {
                'e' => (String_vertice, None),
                _ => (Err(vec!['e']), None),
            },
            String_vertice => match char {
                's' => (Start, Some(AttributVertices(line, offset))),
                _ => (Err(vec!['s']), None),
            },
            String_vi => match char {
                's' => (String_vis, None),
                _ => (Err(vec!['s']), None),
            },
            String_vis => match char {
                'i' => (String_visi, None),
                _ => (Err(vec!['i']), None),
            },
            String_visi => match char {
                'b' => (String_visib, None),
                _ => (Err(vec!['b']), None),
            },
            String_visib => match char {
                'l' => (String_visibl, None),
                _ => (Err(vec!['l']), None),
            },
            String_visibl => match char {
                'e' => (String_visible, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible => match char {
                '_' => (String_visible_, None),
                _ => (Err(vec!['_']), None),
            },
            String_visible_ => match char {
                'e' => (String_visible_e, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible_e => match char {
                'x' => (String_visible_ex, None),
                _ => (Err(vec!['x']), None),
            },
            String_visible_ex => match char {
                't' => (String_visible_ext, None),
                _ => (Err(vec!['t']), None),
            },
            String_visible_ext => match char {
                'e' => (String_visible_exte, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible_exte => match char {
                'n' => (String_visible_exten, None),
                _ => (Err(vec!['n']), None),
            },
            String_visible_exten => match char {
                't' => (Start, Some(AttributVisibleExtent(line, offset))),
                _ => (Err(vec!['t']), None),
            },
            String_ba => match char {
                'c' => (String_bac, None),
                _ => (Err(vec!['c']), None),
            },
            String_bac => match char {
                'k' => (String_back, None),
                _ => (Err(vec!['k']), None),
            },
            String_back => match char {
                'g' => (String_backg, None),
                _ => (Err(vec!['g']), None),
            },
            String_backg => match char {
                'r' => (String_backgr, None),
                _ => (Err(vec!['r']), None),
            },
            String_backgr => match char {
                'o' => (String_backgro, None),
                _ => (Err(vec!['o']), None),
            },
            String_backgro => match char {
                'u' => (String_backgrou, None),
                _ => (Err(vec!['u']), None),
            },
            String_backgrou => match char {
                'n' => (String_backgroun, None),
                _ => (Err(vec!['n']), None),
            },
            String_backgroun => match char {
                'd' => (String_background, None),
                _ => (Err(vec!['d']), None),
            },
            String_background => match char {
                '_' => (String_background_, None),
                _ => (Err(vec!['c']), None),
            },
            String_background_ => match char {
                'c' => (String_background_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_background_c => match char {
                'o' => (String_background_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_background_co => match char {
                'l' => (String_background_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_background_col => match char {
                'o' => (String_background_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_background_colo => match char {
                'r' => (Start, Some(AttributBackgroundColor(line, offset))),
                _ => (Err(vec!['r']), None),
            },
            String_s => match char {
                'h' => (String_sh, None),
                _ => (Err(vec!['h']), None),
            },
            String_sh => match char {
                'a' => (String_sha, None),
                _ => (Err(vec!['a']), None),
            },
            String_sha => match char {
                'p' => (String_shap, None),
                _ => (Err(vec!['p']), None),
            },
            String_shap => match char {
                'e' => (String_shape, None),
                _ => (Err(vec!['e']), None),
            },
            String_shape => match char {
                's' => (Start, Some(AttributShapes(line, offset))),
                _ => (Err(vec!['s']), None),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_source_code() -> Result<(), String> {
        let mut actual_tokens = to_tokens("{visible_extentbackground_colorshapes}").into_iter();
        let mut expected_tokens = vec![
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            AttributBackgroundColor(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter();

        let previous_actual_token: Option<Token> = None;
        let previous_expected_token: Option<Token> = None;

        loop {
            let actual_token = actual_tokens.next();
            let expected_token = expected_tokens.next();

            match (actual_token, expected_token) {
                (None, None) => return Ok(()),
                (None, Some(expected_token)) => return Result::Err(format!("`actual_tokens` ended before `expected_tokens`.\n `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", expected_token, previous_actual_token, previous_expected_token)),
                (Some(actual_token), None) => return Result::Err(format!("`expected_tokens` ended before `actual_tokens`.\n `actual_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", actual_token, previous_actual_token, previous_expected_token)),
                (Some(actual_token), Some(expected_token)) => {if !crate::is_same_variant(&actual_token, &expected_token) {
                    return Result::Err(format!("`actual_token` does not equal `current_token`.\n `actual_token`: `{:?}`, `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`.", actual_token, expected_token, previous_actual_token, previous_expected_token));
                }},
            };
        }
    }

    #[test]
    fn custom_visible_extent() -> Result<(), String> {
        let mut actual_tokens =
            to_tokens("{visible_extent{x=+10001111y=-001010101111111}background_colorshapes}")
                .into_iter();
        let mut expected_tokens = vec![
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            PositiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributBackgroundColor(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter();

        let previous_actual_token: Option<Token> = None;
        let previous_expected_token: Option<Token> = None;

        loop {
            let actual_token = actual_tokens.next();
            let expected_token = expected_tokens.next();

            match (actual_token, expected_token) {
                (None, None) => return Ok(()),
                (None, Some(expected_token)) => return Result::Err(format!("`actual_tokens` ended before `expected_tokens`.\n `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", expected_token, previous_actual_token, previous_expected_token)),
                (Some(actual_token), None) => return Result::Err(format!("`expected_tokens` ended before `actual_tokens`.\n `actual_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", actual_token, previous_actual_token, previous_expected_token)),
                (Some(actual_token), Some(expected_token)) => {if !crate::is_same_variant(&actual_token, &expected_token) {
                    return Result::Err(format!("`actual_token` does not equal `current_token`.\n `actual_token`: `{:?}`, `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`.", actual_token, expected_token, previous_actual_token, previous_expected_token));
                }},
            };
        }
    }

    #[test]
    fn custom_background_color() -> Result<(), String> {
        let mut actual_tokens =
            to_tokens("{visible_extentbackground_color{red=11111111green=00000000blue=0}shapes}")
                .into_iter();
        let mut expected_tokens = vec![
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            AttributBackgroundColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter();

        let previous_actual_token: Option<Token> = None;
        let previous_expected_token: Option<Token> = None;

        loop {
            let actual_token = actual_tokens.next();
            let expected_token = expected_tokens.next();

            match (actual_token, expected_token) {
                    (None, None) => return Ok(()),
                    (None, Some(expected_token)) => return Result::Err(format!("`actual_tokens` ended before `expected_tokens`.\n `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", expected_token, previous_actual_token, previous_expected_token)),
                    (Some(actual_token), None) => return Result::Err(format!("`expected_tokens` ended before `actual_tokens`.\n `actual_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", actual_token, previous_actual_token, previous_expected_token)),
                    (Some(actual_token), Some(expected_token)) => {if !crate::is_same_variant(&actual_token, &expected_token) {
                        return Result::Err(format!("`actual_token` does not equal `current_token`.\n `actual_token`: `{:?}`, `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`.", actual_token, expected_token, previous_actual_token, previous_expected_token));
                    }},
                };
        }
    }

    #[test]
    fn custom_full() -> Result<(), String> {
        let mut actual_tokens = to_tokens(
            r#"{
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
                                blue = 
                                11
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
        )
        .into_iter();

        let mut expected_tokens = vec![
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributBackgroundColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributShapes(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            AttributRotation(0, 0),
            AttributWidth(0, 0),
            AttributBorderColor(0, 0),
            AttributFillColor(0, 0),
            AttributVertices(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            PositiveValue(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributRotation(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributShapes(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributRotation(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            AttributWidth(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributBorderColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributFillColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributVertices(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            AttributRotation(0, 0),
            AttributWidth(0, 0),
            AttributBorderColor(0, 0),
            AttributFillColor(0, 0),
            AttributVertices(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            PositiveValue(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributRotation(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributWidth(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributBorderColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributFillColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributVertices(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            AttributRotation(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter();

        let previous_actual_token: Option<Token> = None;
        let previous_expected_token: Option<Token> = None;

        loop {
            let actual_token = actual_tokens.next();
            let expected_token = expected_tokens.next();

            match (actual_token, expected_token) {
                    (None, None) => return Ok(()),
                    (None, Some(expected_token)) => return Result::Err(format!("`actual_tokens` ended before `expected_tokens`.\n `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", expected_token, previous_actual_token, previous_expected_token)),
                    (Some(actual_token), None) => return Result::Err(format!("`expected_tokens` ended before `actual_tokens`.\n `actual_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`", actual_token, previous_actual_token, previous_expected_token)),
                    (Some(actual_token), Some(expected_token)) => {if !crate::is_same_variant(&actual_token, &expected_token) {
                        return Result::Err(format!("`actual_token` does not equal `current_token`.\n `actual_token`: `{:?}`, `expected_token`: `{:?}`, `previous_actual_token`: `{:?}`, `previous_expected_token`: `{:?}`.", actual_token, expected_token, previous_actual_token, previous_expected_token));
                    }},
                };
        }
    }
}

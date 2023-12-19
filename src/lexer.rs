use crate::token::Token;
use crate::token::Token::*;
use State::*;

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_state = Start;

    for input in source_code.chars() {
        let (next_state, output) = current_state.next_state(&input);

        if let Err(expected) = next_state {
            panic!(
                "Lexer error: Expected {:?} in state {:?}, got {}.",
                expected, current_state, input
            )
        }

        if let Some(token) = output {
            tokens.push(token);
        }

        current_state = next_state;
    }

    tokens
}

#[derive(Debug)]
enum State {
    Start,
    String_r,
    String_re,
    String_g,
    String_gr,
    String_gre,
    String_gree,
    String_b,
    String_bl,
    String_blu,
    String_p,
    String_po,
    String_pos,
    String_posi,
    String_posit,
    String_positi,
    String_positio,
    String_ro,
    String_rot,
    String_rota,
    String_rotat,
    String_rotati,
    String_rotatio,
    String_w,
    String_wi,
    String_wid,
    String_widt,
    String_bo,
    String_bor,
    String_bord,
    String_borde,
    String_border,
    String_border_,
    String_border_c,
    String_border_co,
    String_border_col,
    String_border_colo,
    String_f,
    String_fi,
    String_fil,
    String_fill,
    String_fill_,
    String_fill_c,
    String_fill_co,
    String_fill_col,
    String_fill_colo,
    String_v,
    String_ve,
    String_ver,
    String_vert,
    String_verti,
    String_vertic,
    String_vertice,
    String_vi,
    String_vis,
    String_visi,
    String_visib,
    String_visibl,
    String_visible,
    String_visible_,
    String_visible_e,
    String_visible_ex,
    String_visible_ext,
    String_visible_exte,
    String_visible_exten,
    String_ba,
    String_bac,
    String_back,
    String_backg,
    String_backgr,
    String_backgro,
    String_backgrou,
    String_backgroun,
    String_background,
    String_background_,
    String_background_c,
    String_background_co,
    String_background_col,
    String_background_colo,
    String_s,
    String_sh,
    String_sha,
    String_shap,
    String_shape,
    Err(Vec<char>),
}

use std::f32::consts::E;

impl State {
    fn next_state(&self, input: &char) -> (State, Option<Token>) {
        match self {
            Err(_) => {
                panic!("The `next_state`-method should never be called on State::Err")
            }
            Start => match input {
                ' ' => (Start, None),
                '=' => (Start, Some(PrimitiveValue)),
                '-' => (Start, Some(NegativValue)),
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
                _ => (
                    Err(vec![
                        ' ', '=', '-', '+', '0', '1', '{', '}', '[', ']', 'r', 'g', 'b', 'x', 'y',
                        'p', 'w', 'f', 'v', 's',
                    ]),
                    None,
                ),
            },
            String_r => match input {
                'e' => (String_re, None),
                'o' => (String_ro, None),
                _ => (Err(vec!['e', 'o']), None),
            },
            String_re => match input {
                'd' => (Start, Some(AttributRed)),
                _ => (Err(vec!['d']), None),
            },
            String_g => match input {
                'r' => (String_gr, None),
                _ => (Err(vec!['r']), None),
            },
            String_gr => match input {
                'e' => (String_gre, None),
                _ => (Err(vec!['e']), None),
            },
            String_gre => match input {
                'e' => (String_gree, None),
                _ => (Err(vec!['e']), None),
            },
            String_gree => match input {
                'n' => (Start, Some(AttributGreen)),
                _ => (Err(vec!['n']), None),
            },
            String_b => match input {
                'l' => (String_bl, None),
                'o' => (String_bo, None),
                'a' => (String_ba, None),
                _ => (Err(vec!['l', 'o', 'a']), None),
            },
            String_bl => match input {
                'u' => (String_blu, None),
                _ => (Err(vec!['u']), None),
            },
            String_blu => match input {
                'e' => (Start, Some(AttributBlue)),
                _ => (Err(vec!['e']), None),
            },
            String_p => match input {
                'o' => (String_po, None),
                _ => (Err(vec!['o']), None),
            },
            String_po => match input {
                's' => (String_pos, None),
                _ => (Err(vec!['s']), None),
            },
            String_pos => match input {
                'i' => (String_posi, None),
                _ => (Err(vec!['i']), None),
            },
            String_posi => match input {
                't' => (String_posit, None),
                _ => (Err(vec!['t']), None),
            },
            String_posit => match input {
                'i' => (String_positi, None),
                _ => (Err(vec!['i']), None),
            },
            String_positi => match input {
                'o' => (String_positio, None),
                _ => (Err(vec!['o']), None),
            },
            String_positio => match input {
                'n' => (Start, Some(AttributPosition)),
                _ => (Err(vec!['n']), None),
            },
            String_ro => match input {
                't' => (String_rot, None),
                _ => (Err(vec!['t']), None),
            },
            String_rot => match input {
                'a' => (String_rota, None),
                _ => (Err(vec!['a']), None),
            },
            String_rota => match input {
                't' => (String_rotat, None),
                _ => (Err(vec!['t']), None),
            },
            String_rotat => match input {
                'i' => (String_rotati, None),
                _ => (Err(vec!['i']), None),
            },
            String_rotati => match input {
                'o' => (String_rotatio, None),
                _ => (Err(vec!['o']), None),
            },
            String_rotatio => match input {
                'n' => (Start, Some(AttributRotation)),
                _ => (Err(vec!['n']), None),
            },
            String_w => match input {
                'i' => (String_wi, None),
                _ => (Err(vec!['i']), None),
            },
            String_wi => match input {
                'd' => (String_wid, None),
                _ => (Err(vec!['d']), None),
            },
            String_wid => match input {
                't' => (String_widt, None),
                _ => (Err(vec!['t']), None),
            },
            String_widt => match input {
                'h' => (Start, Some(AttributWidth)),
                _ => (Err(vec!['h']), None),
            },
            String_bo => match input {
                'r' => (String_bor, None),
                _ => (Err(vec!['r']), None),
            },
            String_bor => match input {
                'd' => (String_bord, None),
                _ => (Err(vec!['d']), None),
            },
            String_bord => match input {
                'e' => (String_borde, None),
                _ => (Err(vec!['e']), None),
            },
            String_borde => match input {
                'r' => (String_border, None),
                _ => (Err(vec!['r']), None),
            },
            String_border => match input {
                '_' => (String_border_, None),
                _ => (Err(vec!['_']), None),
            },
            String_border_ => match input {
                'c' => (String_border_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_border_c => match input {
                'o' => (String_border_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_border_co => match input {
                'l' => (String_border_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_border_col => match input {
                'o' => (String_border_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_border_colo => match input {
                'r' => (Start, Some(AttributBorderColor)),
                _ => (Err(vec!['r']), None),
            },
            String_f => match input {
                'i' => (String_fi, None),
                _ => (Err(vec!['i']), None),
            },
            String_fi => match input {
                'l' => (String_fil, None),
                _ => (Err(vec!['l']), None),
            },
            String_fil => match input {
                'l' => (String_fill, None),
                _ => (Err(vec!['l']), None),
            },
            String_fill => match input {
                '_' => (String_fill_, None),
                _ => (Err(vec!['_']), None),
            },
            String_fill_ => match input {
                'c' => (String_fill_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_fill_c => match input {
                'o' => (String_fill_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_fill_co => match input {
                'l' => (String_fill_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_fill_col => match input {
                'o' => (String_fill_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_fill_colo => match input {
                'r' => (Start, Some(AttributFillColor)),
                _ => (Err(vec!['r']), None),
            },
            String_v => match input {
                'e' => (String_ve, None),
                'i' => (String_vi, None),
                _ => (Err(vec!['e', 'i']), None),
            },
            String_ve => match input {
                'r' => (String_ver, None),
                _ => (Err(vec!['r']), None),
            },
            String_ver => match input {
                't' => (String_vert, None),
                _ => (Err(vec!['t']), None),
            },
            String_vert => match input {
                'i' => (String_verti, None),
                _ => (Err(vec!['i']), None),
            },
            String_verti => match input {
                'c' => (String_vertic, None),
                _ => (Err(vec!['c']), None),
            },
            String_vertic => match input {
                'e' => (String_vertice, None),
                _ => (Err(vec!['e']), None),
            },
            String_vertice => match input {
                's' => (Start, Some(AttributVertices)),
                _ => (Err(vec!['s']), None),
            },
            String_vi => match input {
                's' => (String_vis, None),
                _ => (Err(vec!['s']), None),
            },
            String_vis => match input {
                'i' => (String_visi, None),
                _ => (Err(vec!['i']), None),
            },
            String_visi => match input {
                'b' => (String_visib, None),
                _ => (Err(vec!['b']), None),
            },
            String_visib => match input {
                'l' => (String_visibl, None),
                _ => (Err(vec!['l']), None),
            },
            String_visibl => match input {
                'e' => (String_visible, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible => match input {
                '_' => (String_visible_, None),
                _ => (Err(vec!['_']), None),
            },
            String_visible_ => match input {
                'e' => (String_visible_e, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible_e => match input {
                'x' => (String_visible_ex, None),
                _ => (Err(vec!['x']), None),
            },
            String_visible_ex => match input {
                't' => (String_visible_ext, None),
                _ => (Err(vec!['t']), None),
            },
            String_visible_ext => match input {
                'e' => (String_visible_exte, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible_exte => match input {
                'n' => (String_visible_exten, None),
                _ => (Err(vec!['n']), None),
            },
            String_visible_exten => match input {
                't' => (Start, Some(AttributVisibleExtent)),
                _ => (Err(vec!['t']), None),
            },
            String_ba => match input {
                'c' => (String_bac, None),
                _ => (Err(vec!['c']), None),
            },
            String_bac => match input {
                'k' => (String_back, None),
                _ => (Err(vec!['k']), None),
            },
            String_back => match input {
                'g' => (String_backg, None),
                _ => (Err(vec!['g']), None),
            },
            String_backg => match input {
                'r' => (String_backgr, None),
                _ => (Err(vec!['r']), None),
            },
            String_backgr => match input {
                'o' => (String_backgro, None),
                _ => (Err(vec!['o']), None),
            },
            String_backgro => match input {
                'u' => (String_backgrou, None),
                _ => (Err(vec!['u']), None),
            },
            String_backgrou => match input {
                'n' => (String_backgroun, None),
                _ => (Err(vec!['n']), None),
            },
            String_backgroun => match input {
                'd' => (String_background, None),
                _ => (Err(vec!['d']), None),
            },
            String_background => match input {
                '_' => (String_background_, None),
                _ => (Err(vec!['c']), None),
            },
            String_background_ => match input {
                'c' => (String_background_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_background_c => match input {
                'o' => (String_background_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_background_co => match input {
                'l' => (String_background_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_background_col => match input {
                'o' => (String_background_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_background_colo => match input {
                'r' => (Start, Some(AttributBackgroundColor)),
                _ => (Err(vec!['r']), None),
            },
            String_s => match input {
                'h' => (String_sh, None),
                _ => (Err(vec!['h']), None),
            },
            String_sh => match input {
                'a' => (String_sha, None),
                _ => (Err(vec!['a']), None),
            },
            String_sha => match input {
                'p' => (String_shap, None),
                _ => (Err(vec!['p']), None),
            },
            String_shap => match input {
                'e' => (String_shape, None),
                _ => (Err(vec!['e']), None),
            },
            String_shape => match input {
                's' => (Start, Some(AttributShapes)),
                _ => (Err(vec!['s']), None),
            },
        }
    }
}

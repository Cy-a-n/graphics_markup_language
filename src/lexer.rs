pub fn tokenize(source_code: &str) {}

enum Token {
    PrimitiveValue,
    NegativValue,
    PositivValue,
    Zero,
    One,
    StructStart,
    StructEnd,
    ArrayStart,
    ArrayEnd,
    Attribut_red,
    Attribut_green,
    Attribut_blue,
    Attribut_x,
    Attribut_y,
    Attribut_position,
    Attribut_rotation,
    Attribut_width,
    Attribut_border_color,
    Attribut_fill_color,
    Attribut_vertices,
    Attribut_visible_extent,
    Attribut_background_color,
    Attribut_shapes,
}

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
    Err(Box<State>, Vec<char>, char),
}

impl State {
    fn next_state(current_state: State, input: char) -> (State, Option<Token>) {
        match current_state {
            State::Err(_, _, _) => {
                panic!("The `next_state`-method should never be called on State::Err")
            }
            State::Start => match input {
                ' ' => (State::Start, None),
                '=' => (State::Start, Some(Token::PrimitiveValue)),
                '-' => (State::Start, Some(Token::NegativValue)),
                '+' => (State::Start, Some(Token::PositivValue)),
                '0' => (State::Start, Some(Token::Zero)),
                '1' => (State::Start, Some(Token::One)),
                '{' => (State::Start, Some(Token::StructStart)),
                '}' => (State::Start, Some(Token::StructEnd)),
                '[' => (State::Start, Some(Token::ArrayStart)),
                ']' => (State::Start, Some(Token::ArrayEnd)),
                'r' => (State::String_r, None),
                'g' => (State::String_g, None),
                'b' => (State::String_b, None),
                _ => (
                    State::Err(
                        Box::new(State::Start),
                        vec![
                            ' ', '=', '-', '+', '0', '1', '{', '}', '[', ']', 'r', 'g', 'b', 'x',
                            'y', 'p', 'w', 'f', 'v', 's',
                        ],
                        input,
                    ),
                    None,
                ),
            },
            State::String_r => todo!(),
            State::String_re => todo!(),
            State::String_g => todo!(),
            State::String_gr => todo!(),
            State::String_gre => todo!(),
            State::String_gree => todo!(),
            State::String_b => todo!(),
            State::String_bl => todo!(),
            State::String_blu => todo!(),
            State::String_p => todo!(),
            State::String_po => todo!(),
            State::String_pos => todo!(),
            State::String_posi => todo!(),
            State::String_posit => todo!(),
            State::String_positi => todo!(),
            State::String_positio => todo!(),
            State::String_ro => todo!(),
            State::String_rot => todo!(),
            State::String_rota => todo!(),
            State::String_rotat => todo!(),
            State::String_rotati => todo!(),
            State::String_rotatio => todo!(),
            State::String_w => todo!(),
            State::String_wi => todo!(),
            State::String_wid => todo!(),
            State::String_widt => todo!(),
            State::String_bo => todo!(),
            State::String_bor => todo!(),
            State::String_bord => todo!(),
            State::String_borde => todo!(),
            State::String_border => todo!(),
            State::String_border_ => todo!(),
            State::String_border_c => todo!(),
            State::String_border_co => todo!(),
            State::String_border_col => todo!(),
            State::String_border_colo => todo!(),
            State::String_f => todo!(),
            State::String_fi => todo!(),
            State::String_fil => todo!(),
            State::String_fill => todo!(),
            State::String_fill_ => todo!(),
            State::String_fill_c => todo!(),
            State::String_fill_co => todo!(),
            State::String_fill_col => todo!(),
            State::String_fill_colo => todo!(),
            State::String_v => todo!(),
            State::String_ve => todo!(),
            State::String_ver => todo!(),
            State::String_vert => todo!(),
            State::String_verti => todo!(),
            State::String_vertic => todo!(),
            State::String_vertice => todo!(),
            State::String_vi => todo!(),
            State::String_vis => todo!(),
            State::String_visi => todo!(),
            State::String_visib => todo!(),
            State::String_visibl => todo!(),
            State::String_visible => todo!(),
            State::String_visible_ => todo!(),
            State::String_visible_e => todo!(),
            State::String_visible_ex => todo!(),
            State::String_visible_ext => todo!(),
            State::String_visible_exte => todo!(),
            State::String_visible_exten => todo!(),
            State::String_ba => todo!(),
            State::String_bac => todo!(),
            State::String_back => todo!(),
            State::String_backg => todo!(),
            State::String_backgr => todo!(),
            State::String_backgro => todo!(),
            State::String_backgrou => todo!(),
            State::String_backgroun => todo!(),
            State::String_background => todo!(),
            State::String_background_ => todo!(),
            State::String_background_c => todo!(),
            State::String_background_co => todo!(),
            State::String_background_col => todo!(),
            State::String_background_colo => todo!(),
            State::String_s => todo!(),
            State::String_sh => todo!(),
            State::String_sha => todo!(),
            State::String_shap => todo!(),
            State::String_shape => todo!(),
        }
    }
}

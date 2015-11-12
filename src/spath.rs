
/*
 * Interprets SVG paths.
 * Sequential M commands are turned into a LineTo
 * Z is turned into a LineTo
 * H and V are turned into LineTo
 * S is turned into CurveTo
 * T is turned into QuadraticTo
 *
 * All values are absolute.
 */

use std::str::FromStr;
use std::borrow::Borrow;

#[derive(Debug)]
pub enum PathElem {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64 },
    QuadraticTo { x1: f64, y1: f64, x: f64, y: f64 },
    ArcTo { rx: f64, ry: f64, x_rotation: f64, lrg_arc: bool, sweep: bool, x: f64, y: f64 }
}

enum PathToken {
    M(bool),
    Z, // upper and lowercase are identical
    L(bool), H(bool), V(bool),
    C(bool), S(bool),
    Q(bool), T(bool),
    A(bool),
}

enum PathParams {
    MLTParam(f64, f64),
    HVParam(f64),
    CParam(f64, f64, f64, f64, f64, f64),
    SQParam(f64, f64, f64, f64),
    AParam(f64, f64, f64, bool, bool, f64, f64)
}

fn get_cmd_char(c: char) -> Option<PathToken> {
    match c.to_uppercase().next().unwrap() {
        'M' => Some(PathToken::M(c.is_uppercase())),
        'Z' => Some(PathToken::Z),
        'L' => Some(PathToken::L(c.is_uppercase())),
        'H' => Some(PathToken::H(c.is_uppercase())),
        'V' => Some(PathToken::V(c.is_uppercase())),
        'C' => Some(PathToken::C(c.is_uppercase())),
        'S' => Some(PathToken::S(c.is_uppercase())),
        'Q' => Some(PathToken::Q(c.is_uppercase())),
        'T' => Some(PathToken::T(c.is_uppercase())),
        'A' => Some(PathToken::A(c.is_uppercase())),
        _ => None
    }
}

fn switch_commas_with_spaces(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    for c in s.chars() {
        r.push(if c == ',' { ' ' } else { c });
    }
    r
}

fn get_cmd(s: &str) -> Option<(PathToken, &str)> {
    let (cmd, rest) = s.trim_left().split_at(1);
    cmd.chars().next().and_then(|cmd_char| get_cmd_char(cmd_char).map(|cmd| (cmd, rest)))
}

fn split_num(s: &str) -> (&str, &str) {
    s.split_at(s.find(|c: char| c != '.' && c != '-' && !c.is_numeric()).unwrap_or(s.len()))
}

fn get_f64(s: &str) -> Option<(f64, &str)> {
    let (num_str, rest) = split_num(s.trim_left());
    FromStr::from_str(num_str).ok().map(|val: f64| (val, rest))
}

fn get_bool(s: &str) -> Option<(bool, &str)> {
    let (num_str, rest) = split_num(s.trim_left());
    FromStr::from_str(num_str).ok().map(|val: u8| (val > 0, rest))
}


fn get_path_params<'a, F>(s: &'a str, get_vals: F) -> Option<(Vec<PathParams>, &'a str)>
    where F: Fn(&'a str) -> Option<(PathParams, &'a str)> {
    let mut v = Vec::<PathParams>::new();
    let mut val = get_vals(s);
    if val.is_some() {
        loop {
            let (data, rest) = val.unwrap();
            v.push(data);
            val = get_vals(rest);
            if val.is_none() {
                return Some((v, rest))
            }
        }
    } else {
        None
    }
}

fn get_mlt<'a>(s: &'a str) -> Option<(PathParams, &'a str)> {
    get_f64(s).and_then(|(x, s2)| get_f64(s2).map(|(y, s3)| (PathParams::MLTParam(x, y), s3)))
}

fn get_mlts<'a>(s: &'a str) -> Option<(Vec<PathParams>, &'a str)> {
    get_path_params(s, get_mlt)
}

fn get_hv<'a>(s: &'a str) -> Option<(PathParams, &'a str)> {
    get_f64(s).map(|(hv, s2)| (PathParams::HVParam(hv), s2))
}

fn get_hvs(s: &str) -> Option<(Vec<PathParams>, &str)> {
    get_path_params(s, get_hv)
}

fn get_c<'a>(s: &'a str) -> Option<(PathParams, &'a str)> {
    get_f64(s)
        .and_then(|(a1, s1)| get_f64(s1)
        .and_then(|(a2, s2)| get_f64(s2)
        .and_then(|(a3, s3)| get_f64(s3)
        .and_then(|(a4, s4)| get_f64(s4)
        .and_then(|(a5, s5)| get_f64(s5)
        .map(|(a6, s6)| (PathParams::CParam(a1, a2, a3, a4, a5, a6), s6))
        )))))
}

fn get_cs(s: &str) -> Option<(Vec<PathParams>, &str)> {
    get_path_params(s, get_c)
}

fn get_sq<'a>(s: &'a str) -> Option<(PathParams, &'a str)> {
    get_f64(s)
        .and_then(|(a1, s1)| get_f64(s1)
        .and_then(|(a2, s2)| get_f64(s2)
        .and_then(|(a3, s3)| get_f64(s3)
        .map(|(a4, s4)| (PathParams::SQParam(a1, a2, a3, a4), s4))
        )))
}

fn get_sqs(s: &str) -> Option<(Vec<PathParams>, &str)> {
    get_path_params(s, get_sq)
}

fn get_a<'a>(s: &'a str) -> Option<(PathParams, &'a str)> {
    get_f64(s)
        .and_then(|(a1, s1)| get_f64(s1)
        .and_then(|(a2, s2)| get_f64(s2)
        .and_then(|(a3, s3)| get_bool(s3)
        .and_then(|(a4, s4)| get_bool(s4)
        .and_then(|(a5, s5)| get_f64(s5)
        .and_then(|(a6, s6)| get_f64(s6)
        .map(|(a7, s7)| (PathParams::AParam(a1, a2, a3, a4, a5, a6, a7), s7))
        ))))))
}

fn get_as(s: &str) -> Option<(Vec<PathParams>, &str)> {
    get_path_params(s, get_a)
}

fn get_cmd_with_params(s: &str) -> Option<(PathToken, Vec<PathParams>, &str)> {
    get_cmd(s).and_then(|(cmd, rest)| match cmd {
        PathToken::M(_) | PathToken::L(_) | PathToken::T(_) =>
            get_mlts(rest).map(|(mlts, others)| (cmd, mlts, others)),
        PathToken::Z => Some((cmd, Vec::new(), rest)),
        PathToken::H(_) | PathToken::V(_) =>
            get_hvs(rest).map(|(hvs, others)| (cmd, hvs, others)),
        PathToken::C(_) => get_cs(rest).map(|(cs, others)| (cmd, cs, others)),
        PathToken::S(_) | PathToken::Q(_) =>
            get_sqs(rest).map(|(sqs, others)| (cmd, sqs, others)),
        PathToken::A(_) => get_as(rest).map(|(a, others)| (cmd, a, others))
    })
}

fn tokenize(si: &str) -> Vec<(PathToken, Vec<PathParams>)> {
    let s: String = switch_commas_with_spaces(si);
    let mut sb: &str = s.borrow();
    let mut so = sb.trim_left();
    let mut v = Vec::<(PathToken, Vec<PathParams>)>::new();
    while !so.is_empty() {
        let maybe_cmd = get_cmd_with_params(so);
        if maybe_cmd.is_none() {
            println!("Giving up at {}", so);
            return v;
        }
        let (token, params, nexts) = maybe_cmd.unwrap();
        v.push((token, params));
        so = nexts.trim_left();
    }
    v
}

fn move_to(pt: Option<(f64, f64)>, params: &PathParams) -> Option<PathElem> {
    match *params {
        PathParams::MLTParam(x, y) => Some(pt.map_or_else(|| PathElem::MoveTo { x: x, y: y },
        |(cx, cy)| PathElem::MoveTo { x: x + cx, y: y + cy })),
        _ => None
    }
}

fn line_to(pt: Option<(f64, f64)>, params: &PathParams) -> Option<PathElem> {
    match *params {
        PathParams::MLTParam(x, y) => Some(pt.map_or_else(|| PathElem::LineTo { x: x, y: y },
        |(cx, cy)| PathElem::LineTo { x: x + cx, y: y + cy })),
        _ => None
    }
}

fn line_to_h(abs: bool, p: (f64, f64), params: &PathParams) -> Option<PathElem> {
    match *params {
        PathParams::HVParam(x) => Some(if abs {
            PathElem::LineTo { x: x, y: p.1 }
        } else {
            PathElem::LineTo { x: x + p.0, y: p.1 }
        }),
        _ => None
    }
}

fn line_to_v(abs: bool, p: (f64, f64), params: &PathParams) -> Option<PathElem> {
    match *params {
        PathParams::HVParam(y) => Some(if abs {
            PathElem::LineTo { x: p.0, y: y }
        } else {
            PathElem::LineTo { x: p.0, y: y + p.1 }
        }),
        _ => None
    }
}

fn curve_to(pt: Option<(f64, f64)>, params: &PathParams) -> Option<PathElem> {
    match *params {
        PathParams::CParam(x1, y1, x2, y2, x, y) => Some(pt.map_or_else(|| PathElem::CurveTo {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            x: x,
            y: y
        },
        |(cx, cy)| PathElem::CurveTo {
            x1: x1 + cx,
            y1: y1 + cy,
            x2: x2 + cx,
            y2: y2 + cy,
            x: x + cx,
            y: y + cy
        })),
        _ => None
    }
}

fn reflect(p: (f64, f64), r: (f64, f64)) -> (f64, f64) {
        (2.0*p.0 - r.0, 2.0*p.1 - r.1)
}

fn smooth_curve_to(abs: bool,
                   cp: Option<(f64, f64)>,
                   p: (f64, f64),
                   params: &PathParams) -> Option<PathElem> {
    let (cp1x, cp1y) = cp.map_or_else(|| p, |r| reflect(p, r));
    match *params {
        PathParams::SQParam(x2, y2, x, y) => Some(if abs {
            PathElem::CurveTo {
                x1: cp1x,
                y1: cp1y,
                x2: x2,
                y2: y2,
                x: x,
                y: y
            }
        } else {
            PathElem::CurveTo {
                x1: cp1x,
                y1: cp1y,
                x2: x2 + p.0,
                y2: y2 + p.1,
                x: x + p.0,
                y: y + p.1
            }
        }),
        _ => None
    }
}

fn quad_to(pt: Option<(f64, f64)>, params: &PathParams) -> Option<PathElem> {
    match *params {
        PathParams::SQParam(x1, y1, x, y) => Some(pt.map_or_else(|| PathElem::QuadraticTo {
            x1: x1,
            y1: y1,
            x: x,
            y: y
        }, |(cx, cy)| PathElem::QuadraticTo {
            x1: x1 + cx,
            y1: y1 + cy,
            x: x + cx,
            y: y + cy,
        })),
        _ => None
    }
}

fn t_quad_to(abs: bool,
             cp: Option<(f64, f64)>,
             p: (f64, f64),
             params: &PathParams) -> Option<PathElem> {
    let (cpx, cpy) = cp.map_or_else(|| p, |r| reflect(p, r));
    match *params {
        PathParams::MLTParam(x, y) => Some(if abs {
            PathElem::QuadraticTo {
                x1: cpx,
                y1: cpy,
                x: x,
                y: y
            }
        } else {
            PathElem::QuadraticTo {
                x1: cpx,
                y1: cpy,
                x: x + p.0,
                y: y + p.1,
            }
        }),
        _ => None
    }
}

fn arc_to(pt: Option<(f64, f64)>, params: &PathParams) -> Option<PathElem> {
    match *params {
        PathParams::AParam(rx, ry, x_rotation, lrg_arc, sweep, x, y) =>
            Some(pt.map_or_else(|| PathElem::ArcTo {
                rx: rx,
                ry: ry,
                x_rotation: x_rotation,
                lrg_arc: lrg_arc,
                sweep: sweep,
                x: x,
                y: y
            }, |(cx, cy)| PathElem::ArcTo {
                rx: rx,
                ry: ry,
                x_rotation: x_rotation,
                lrg_arc: lrg_arc,
                sweep: sweep,
                x: x + cx,
                y: y + cy,
            })),
        _ => None
    }
}

struct PathState {
    v: Vec<PathElem>,
}

impl PathState {
    fn new() -> PathState {
        PathState {
            v: Vec::<PathElem>::new()
        }
    }

    fn point_for(elem: &PathElem) -> (f64, f64) {
        match *elem {
            PathElem::MoveTo { x, y } => (x, y),
            PathElem::LineTo { x, y } => (x, y),
            PathElem::CurveTo { x, y, .. } => (x, y),
            PathElem::QuadraticTo { x, y, .. } => (x, y),
            PathElem::ArcTo { x, y, .. } => (x, y)
        }
    }

    fn last_curve_cp(&self) -> Option<(f64, f64)> {
        self.v.last().and_then(|elem| match *elem {
            PathElem::CurveTo { x2, y2, .. } => Some((x2, y2)),
            _ => None
        })
    }

    fn last_quad_cp(&self) -> Option<(f64, f64)> {
        self.v.last().and_then(|elem| match *elem {
            PathElem::QuadraticTo { x1, y1, .. } => Some((x1, y1)),
            _ => None
        })
    }

    fn initial_pt(&self) -> Option<(f64, f64)> {
        self.v.get(0).map(PathState::point_for)
    }
    
    fn last_pt(&self) -> Option<(f64, f64)> {
        self.v.last().map(PathState::point_for)
    }

    fn update(&mut self, elem: PathElem) {
        self.v.push(elem);
    }
}

fn convert_token(token: PathToken, mut params: Vec<PathParams>,
                 mut s: PathState) -> PathState {
    let origin = (0 as f64, 0 as f64);
    match token {
        PathToken::M(abs) => {
            let lpt1 = if abs { None } else { s.last_pt() };
            let move_loc = params.remove(0);
            let mt = move_to(lpt1, &move_loc).unwrap();
            s.update(mt);
            for p in params {
                let lpt = if abs { None } else { s.last_pt() };
                let elem = line_to(lpt, &p).unwrap();
                s.update(elem);
            }
        },
        PathToken::Z => {
            let (ix, iy) = s.initial_pt().unwrap_or(origin);
            let (px, py) = s.last_pt().unwrap_or(origin);
            s.update(PathElem::LineTo { x: ix, y: iy });
            s.update(PathElem::MoveTo { x: px, y: py });
        },
        PathToken::L(abs) => {
            let lpt = if abs { None } else { s.last_pt() };
            for p in params {
                let elem = line_to(lpt, &p).unwrap();
                s.update(elem);
            }
        },
        PathToken::H(abs) => for p in params {
            let elem = line_to_h(abs, s.last_pt().unwrap_or(origin), &p).unwrap();
            s.update(elem);
        },
        PathToken::V(abs) => for p in params {
            let elem = line_to_v(abs, s.last_pt().unwrap_or(origin), &p).unwrap();
            s.update(elem);
        },
        PathToken::C(abs) => {
            let lpt = if abs { None } else { s.last_pt() };
            for p in params {
                let elem = curve_to(lpt, &p).unwrap();
                s.update(elem);
            }
        },
        PathToken::S(abs) => for p in params {
            let elem = smooth_curve_to(abs, s.last_curve_cp(),
                                     s.last_pt().unwrap_or(origin), &p).unwrap();
            s.update(elem);
        },
        PathToken::Q(abs) => {
            let lpt = if abs { None } else { s.last_pt() };
            for p in params {
                let elem = quad_to(lpt, &p).unwrap();
                s.update(elem);
            }
        },
        PathToken::T(abs) => for p in params {
            let elem = t_quad_to(abs, s.last_quad_cp(),
                s.last_pt().unwrap_or(origin), &p).unwrap();
            s.update(elem);
        },
        PathToken::A(abs) => {
            let lpt = if abs { None } else { s.last_pt() };
            for p in params {
                let elem = arc_to(lpt, &p).unwrap();
                s.update(elem);
            }
        }
    };

    s
}

pub fn read_path(s: &str) -> Vec<PathElem> {
    let tokens = tokenize(s);
    let mut state = PathState::new();
    for (token, params) in tokens {
        state = convert_token(token, params, state);
    }
    state.v
}

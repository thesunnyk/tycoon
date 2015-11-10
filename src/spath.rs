
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

pub enum PathElem {
    Move { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64 },
    QuadraticTo { x2: f64, y2: f64, x: f64, y: f64 },
    ArcTo { rx: f64, ry: f64, x_rotation: f64, lrg_arc: bool, sweep: bool, x: f64, y: f64 }
}

enum PathToken {
    MU, ML,
    Z, // upper and lowercase are identical
    LU, LL, HU, HL, VU, VL,
    CU, CL, SU, SL,
    QU, QL, TU, TL,
    AU, AL,
}

enum PathParams {
    MLTParam(f64, f64),
    HVParam(f64),
    CParam(f64, f64, f64, f64, f64, f64),
    SQParam(f64, f64, f64, f64),
    AParam(f64, f64, f64, bool, bool, f64, f64)
}

fn get_cmd_char(c: char) -> Option<PathToken> {
    match c {
        'M' => Some(PathToken::MU),
        'm' => Some(PathToken::ML),
        'z' => Some(PathToken::Z),
        'Z' => Some(PathToken::Z),
        'L' => Some(PathToken::LU),
        'l' => Some(PathToken::LL),
        'H' => Some(PathToken::HU),
        'h' => Some(PathToken::HL),
        'V' => Some(PathToken::VU),
        'v' => Some(PathToken::VL),
        'C' => Some(PathToken::CU),
        'c' => Some(PathToken::CL),
        'S' => Some(PathToken::SU),
        's' => Some(PathToken::SL),
        'Q' => Some(PathToken::QU),
        'q' => Some(PathToken::QL),
        'T' => Some(PathToken::TU),
        't' => Some(PathToken::TL),
        'A' => Some(PathToken::AU),
        'a' => Some(PathToken::AL),
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
    let (cmd, rest) = s.trim_left().split_at(0);
    cmd.chars().next().and_then(|cmd_char| get_cmd_char(cmd_char).map(|cmd| (cmd, rest)))
}

fn split_num(s: &str) -> (&str, &str) {
    s.split_at(s.find(|c: char| c != '.' && !c.is_numeric()).unwrap_or(s.len()))
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
        .map(|(a7, s7)| (PathParams::AParam(a1, a2, a3, a4, a5, a6, a7), s6))
        ))))))
}

fn get_as(s: &str) -> Option<(Vec<PathParams>, &str)> {
    get_path_params(s, get_a)
}

fn get_cmd_with_params(s: &str) -> Option<(PathToken, Vec<PathParams>, &str)> {
    get_cmd(s).and_then(|(cmd, rest)| match cmd {
        PathToken::MU | PathToken::ML | PathToken::LU | PathToken::LL | PathToken::TU |
            PathToken::TL =>
            get_mlts(rest).map(|(mlts, others)| (cmd, mlts, others)),
        PathToken::Z => Some((cmd, Vec::new(), rest)),
        PathToken::HU | PathToken::HL | PathToken::VU | PathToken::VL =>
            get_hvs(rest).map(|(hvs, others)| (cmd, hvs, others)),
        PathToken::CU | PathToken::CL => get_cs(rest).map(|(cs, others)| (cmd, cs, others)),
        PathToken::SU | PathToken::SL | PathToken::QU | PathToken::QL =>
            get_sqs(rest).map(|(sqs, others)| (cmd, sqs, others)),
        PathToken::AU | PathToken::AL => get_as(rest).map(|(a, others)| (cmd, a, others))
    })
}

fn tokenize(si: &str) -> Vec<(PathToken, Vec<PathParams>)> {
    let s: String = switch_commas_with_spaces(si);
    let mut so: &str = s.borrow();
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

// pub fn read_path(s: &str) -> Vec<PathElem> {
// }


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

fn get_mlts(s: &str) -> Option<(Vec<PathParams>, &str)> {
    None
}

fn get_hvs(s: &str) -> Option<(Vec<PathParams>, &str)> {
    None
}

fn get_cs(s: &str) -> Option<(Vec<PathParams>, &str)> {
    None
}

fn get_sqs(s: &str) -> Option<(Vec<PathParams>, &str)> {
    None
}

fn get_as(s: &str) -> Option<(Vec<PathParams>, &str)> {
    None
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

// fn tokenize(si: &str) -> Vec<PathToken> {
//     let s: String = FromStr::from_str(si).unwrap();

// }

// pub fn read_path(s: &str) -> Vec<PathElem> {
// }

#[path = "./scanner.rs"]
mod scanner;
pub use crate::scanner::Scanner;

#[path = "./math_tokenizer.rs"]
mod math_tokenizer;
pub use crate::math_tokenizer::{MathToken, MathTokenizer};

#[path = "./parser.rs"]
mod parser;
pub use crate::parser::{precedence, RPNExpr, ShuntingParser};

#[path = "./rpneval.rs"]
mod rpneval;
pub use crate::rpneval::MathContext;

pub use num_complex::Complex32;
pub use std::f32::consts::{E, PI};
pub use std::f32::{INFINITY, NEG_INFINITY};
pub use std::vec::Vec;
pub use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(input: &str, n: i16, graph_type: u8) -> Vec<f32> {
    let expr = ShuntingParser::parse_str(input).unwrap();
    let ctx = MathContext::new();
    let mut pos = Vec::new();
    let mut color = Vec::new();
    let saturation = 0.75;
    let num = n * 10;
    for i in -num..=num {
        for j in -num..=num {
            let re = i as f32 / n as f32;
            let im = j as f32 / n as f32;
            pos.push(re);
            pos.push(im);
            let result = ctx.eval(&expr, Complex32::new(re, im)).unwrap();
            // settles height
            match graph_type {
                0 | 1 => {
                    pos.push(remove_inf(result.re));
                }
                2 | 3 => pos.push(remove_inf(result.im)),
                _ => pos.push(remove_inf(result.norm())),
            }
            // settles color
            match graph_type {
                0 => {
                    let mut color_vec = hsvcolor(sig(result.im) * 2. * PI, saturation, 1.);
                    color.append(&mut color_vec);
                }
                1 => {
                    let sig_im = sig(result.im);
                    color.push(sig_im);
                    color.push(sig_im);
                    color.push(sig_im);
                }
                2 => {
                    let mut color_vec = hsvcolor(sig(result.re) * 2. * PI, saturation, 1.);
                    color.append(&mut color_vec);
                }
                3 => {
                    let sig_re = sig(result.re);
                    color.push(sig_re);
                    color.push(sig_re);
                    color.push(sig_re);
                }
                _ => {
                    let mut color_vec = hsvcolor(result.arg(), saturation, 1.);
                    color.append(&mut color_vec);
                }
            }
        }
    }
    pos.append(&mut color);
    return pos;
}

fn remove_inf(val: f32) -> f32 {
    match val {
        f32::NAN => return 1E+5,
        INFINITY => return 1E+5,
        NEG_INFINITY => return -1E+5,
        _ => return val,
    }
}
// accepts h in radians
fn hsvcolor(mut h: f32, s: f32, v: f32) -> Vec<f32> {
    // neg check
    if h < 0. {
        h = h % (2. * PI);
        h += 2. * PI;
    }
    // convert to degrees
    h = h * 180. / PI;
    let c = s * v;
    let i = h / 60.;
    let x = c * (1. - (i % 2. - 1.).abs());

    let r1: f32;
    let g1: f32;
    let b1: f32;

    match i.floor() as i8 {
        0 => {
            r1 = c;
            g1 = x;
            b1 = 0.;
        }
        1 => {
            r1 = x;
            g1 = c;
            b1 = 0.;
        }
        2 => {
            r1 = 0.;
            g1 = c;
            b1 = x;
        }
        3 => {
            r1 = 0.;
            g1 = x;
            b1 = c;
        }
        4 => {
            r1 = x;
            g1 = 0.;
            b1 = c;
        }
        5 => {
            r1 = c;
            g1 = 0.;
            b1 = x;
        }
        _ => {
            r1 = 0.;
            g1 = 0.;
            b1 = 0.;
        }
    }

    let m = v - c;
    let mut vec = Vec::new();
    vec.push(r1 + m);
    vec.push(g1 + m);
    vec.push(b1 + m);
    return vec;
}

fn sig(x: f32) -> f32 {
    return 1. / (1. + E.powf(-x));
}

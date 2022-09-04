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
pub use std::vec::Vec;
pub use wasm_bindgen::prelude::*;

// ports console.log
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn evaluate(input: &str, n: i16, graph_type: u8, log_height: bool) -> Vec<f32> {
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
                // Re-Im, height = real component
                0 | 1 => {
                    let height = remove_inf(result.re);
                    if log_height {
                        if height == 0. {
                            pos.push(0.);
                        } else if height > 0. {
                            pos.push((1. + height).ln());
                        } else {
                            pos.push(-(1. - height).ln());
                        }
                    } else {
                        pos.push(height);
                    }
                }
                // Im-Re, height = imaginary component
                2 | 3 => {
                    let height = remove_inf(result.im);
                    if log_height {
                        if height == 0. {
                            pos.push(0.);
                        } else if height > 0. {
                            pos.push((1. + height).ln());
                        } else {
                            pos.push(-(1. - height).ln());
                        }
                    } else {
                        pos.push(height);
                    }
                }
                // Mod-Arg, height = modulus
                _ => {
                    let height = remove_inf(result.norm());
                    if log_height {
                        pos.push((1.0 + height).ln());
                    } else {
                        pos.push(height);
                    }
                }
            }
            // settles color
            match graph_type {
                // Re-Im, color
                0 => {
                    //log(format!("{:?}", result.im).as_str());
                    let mut color_vec = gradient(result.im);
                    color.append(&mut color_vec);
                }
                // Re-Im, BW
                1 => {
                    let sig_im = sig(result.im);
                    color.push(sig_im);
                    color.push(sig_im);
                    color.push(sig_im);
                }
                // Im-Re, color
                2 => {
                    let mut color_vec = gradient(result.re);
                    color.append(&mut color_vec);
                }
                // Im-Re, BW
                3 => {
                    let sig_re = sig(result.re);
                    color.push(sig_re);
                    color.push(sig_re);
                    color.push(sig_re);
                }
                // Mod-Arg
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
    if val.is_nan() {
        return 1E+38; // handles NaN
    } else if val.is_finite() {
        return val; // handles normal values
    } else if val.is_sign_positive() {
        return 1E+38; // handles positive infinity
    }
    return -1E+38; // handles negative infinity
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

fn gradient(y: f32) -> Vec<f32> {
    let gradient = colorous::INFERNO;
    let color = gradient.eval_continuous(sig(y) as f64);
    let mut vec = Vec::new();
    vec.push((color.r as f32) / 256.);
    vec.push((color.g as f32) / 256.);
    vec.push((color.b as f32) / 256.);
    return vec;
}

fn sig(x: f32) -> f32 {
    return 1. / (1. + E.powf(-x / 5.));
}

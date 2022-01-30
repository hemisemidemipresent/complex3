use crate::math_tokenizer::MathToken;
use crate::parser::RPNExpr;
use num_complex::{Complex32, Complex64};
use spfunc::gamma::*;
// use spfunc::zeta::{zeta, zetah};
use std::collections::HashMap;
use std::f32::consts::PI;
pub use std::f32::{INFINITY, NEG_INFINITY};

// its a secret tool that will help us later
// // a shorthand for checking number of arguments before eval_fn
macro_rules! nargs {
    ($argcheck:expr, $ifok:expr) => {
        if $argcheck {
            $ifok
        } else {
            Err(format!("Wrong number of arguments"))
        }
    };
}

// yeah this is the base of like every eval
#[derive(Debug, Clone)]
pub struct MathContext(pub HashMap<String, Complex32>);

impl MathContext {
    pub fn new() -> MathContext {
        use std::f32::consts;
        let mut cx = HashMap::new();
        cx.insert(format!("pi"), Complex32::new(consts::PI, 0.));
        cx.insert(format!("e"), Complex32::new(consts::E, 0.));
        MathContext(cx)
    }

    pub fn setvar(&mut self, var: &str, val: Complex32) {
        self.0.insert(var.to_string(), val);
    }

    pub fn eval(&self, rpn: &RPNExpr, z: Complex32) -> Result<Complex32, String> {
        let mut operands = Vec::new();

        for token in rpn.0.iter() {
            match *token {
                MathToken::Number(num) => operands.push(Complex32::new(num, 0.)),
                MathToken::Imaginary(num) => operands.push(Complex32::new(0., num)),
                MathToken::Variable(_) => operands.push(z),
                MathToken::BOp(ref op) => {
                    let r = operands.pop().ok_or(format!("Wrong number of arguments"))?;
                    let l = operands.pop().ok_or(format!("Wrong number of arguments"))?;
                    println!("{}", r);
                    println!("{}", l);

                    match &op[..] {
                        "+" => operands.push(l + r),
                        "-" => operands.push(l - r),
                        "*" => operands.push(l * r),
                        "/" => {
                            let quotient = l / r;
                            if quotient.is_normal() {
                                operands.push(quotient);
                            } else {
                                operands.push(l.fdiv(r));
                            }
                        }
                        "^" => {
                            if r.im == 0. {
                                operands.push(l.powf(r.re));
                            } else if l.re == 0. && l.im == 0. {
                                operands.push(Complex32::new(0., 0.));
                            } else {
                                operands.push(l.powc(r));
                            }
                        }
                        _ => return Err(format!("Bad Token: {}", op.clone())),
                    }
                }
                MathToken::UOp(ref op) => {
                    let o = operands.pop().ok_or(format!("Wrong number of arguments"))?;
                    match &op[..] {
                        "-" => operands.push(-o),
                        _ => return Err(format!("Bad Token: {}", op.clone())),
                    }
                }
                MathToken::Function(ref fname, arity) => {
                    if arity > operands.len() {
                        return Err(format!("Wrong number of arguments"));
                    }
                    let cut = operands.len() - arity;
                    let args = operands.split_off(cut);
                    operands.push(Self::eval_fn(fname, args)?)
                }
                _ => return Err(format!("Bad Token: {:?}", *token)),
            }
        }
        operands.pop().ok_or(format!("Wrong number of arguments"))
    }

    fn eval_fn(fname: &str, args: Vec<Complex32>) -> Result<Complex32, String> {
        match fname {
            // trigonometric
            "sin" => nargs!(args.len() == 1, Ok(args[0].sin())),
            "cos" => nargs!(args.len() == 1, Ok(args[0].cos())),
            "tan" => nargs!(args.len() == 1, Ok(args[0].tan())),
            "csc" => nargs!(args.len() == 1, Ok(args[0].sin().finv())),
            "sec" => nargs!(args.len() == 1, Ok(args[0].cos().finv())),
            "cot" => nargs!(args.len() == 1, Ok(args[0].tan().finv())),
            // hyperbolic trigonometric
            "sinh" => nargs!(args.len() == 1, Ok(args[0].sinh())),
            "cosh" => nargs!(args.len() == 1, Ok(args[0].cosh())),
            "tanh" => nargs!(args.len() == 1, Ok(args[0].tanh())),
            "csch" => nargs!(args.len() == 1, Ok(args[0].sinh().finv())),
            "sech" => nargs!(args.len() == 1, Ok(args[0].cosh().finv())),
            "coth" => nargs!(args.len() == 1, Ok(args[0].tanh().finv())),
            // inverse trigonometric
            "asin" => nargs!(args.len() == 1, Ok(args[0].asin())),
            "acos" => nargs!(args.len() == 1, Ok(args[0].acos())),
            "atan" => nargs!(args.len() == 1, Ok(args[0].atan())),
            "acsc" => nargs!(args.len() == 1, Ok(args[0].asin().finv())),
            "asec" => nargs!(args.len() == 1, Ok(args[0].acos().finv())),
            "acot" => nargs!(args.len() == 1, Ok(args[0].atan().finv())),
            // inverse hyperbolic trigonometric
            "asinh" => nargs!(args.len() == 1, Ok(args[0].asinh())),
            "acosh" => nargs!(args.len() == 1, Ok(args[0].acosh())),
            "atanh" => nargs!(args.len() == 1, Ok(args[0].atanh())),
            "acsch" => nargs!(args.len() == 1, Ok(args[0].asinh().finv())),
            "asech" => nargs!(args.len() == 1, Ok(args[0].acosh().finv())),
            "acoth" => nargs!(args.len() == 1, Ok(args[0].atanh().finv())),
            // misc
            "exp" => nargs!(args.len() == 1, Ok(args[0].exp())),
            "ln" => nargs!(args.len() == 1, Ok(args[0].ln())),
            "log" => nargs!(args.len() == 2, Ok(args[0].log(args[1].norm()))),
            "conj" => nargs!(args.len() == 1, Ok(args[0].conj())),
            "inv" => nargs!(args.len() == 1, Ok(args[0].inv())),
            "sqrt" => nargs!(args.len() == 1, Ok(args[0].sqrt())),
            "cbrt" => nargs!(args.len() == 1, Ok(args[0].cbrt())),
            // util?
            "Re" => nargs!(args.len() == 1, Ok(Complex32::new(args[0].re, 0.))),
            "Im" => nargs!(args.len() == 1, Ok(Complex32::new(args[0].im, 0.))),
            "norm" | "mod" => nargs!(args.len() == 1, Ok(Complex32::new(args[0].norm(), 0.))),
            "arg" => nargs!(args.len() == 1, Ok(Complex32::new(args[0].arg(), 0.))),
            // special fns
            "zeta" => nargs!(args.len() == 1, Ok(zeta3(args[0], 25))),
            "zetac" => nargs!(args.len() == 2, Ok(zeta3(args[0], args[1].norm() as i32))),

            "gamma" => nargs!(args.len() == 1, Ok(gamma(args[0]))),
            "lngamma" => nargs!(args.len() == 1, Ok(gamma_ln(args[0]))),
            "digamma" => nargs!(args.len() == 1, Ok(digamma(args[0]))),
            "trigamma" => nargs!(args.len() == 1, Ok(polygamma(args[0], 1))),
            "polygamma" => nargs!(
                args.len() == 2,
                Ok(polygamma(args[0], (args[1].norm() as u8).into()))
            ),
            "lambertw" => nargs!(args.len() == 1, Ok(lambertw(args[0], 0))),
            "lambertwb" => nargs!(
                args.len() == 2,
                Ok(lambertw(args[0], args[1].norm() as i32))
            ),

            // final
            _ => nargs!(args.len() == 1, Ok(args[0])),
        }
    }
}

pub fn gamma(z: Complex32) -> Complex32 {
    let g = 7; // 15 max
    let gamma_p = vec![
        0.99999999999999709182,
        57.156235665862923517,
        -59.597960355475491248,
        14.136097974741747174,
        -0.49191381609762019978,
        0.33994649984811888699E-4,
        0.46523628927048575665E-4,
        -0.98374475304879564677E-4,
        0.15808870322491248884E-3,
        -0.21026444172410488319E-3,
        0.21743961811521264320E-3,
        -0.16431810653676389022E-3,
        0.84418223983852743293E-4,
        -0.26190838401581408670E-4,
        0.36899182659531622704E-5,
    ];
    let gamma_g = 4.7421875;
    let sqrt_2_pi = 2.5066282746310002; // sqrt(2*PI)
    if z.re < 0.5 {
        // t is name for 1 - z
        let t = Complex32::new(1., 0.) - z;
        let sin_pi_z = Complex32::new(z.re * PI, z.im * PI).sin();
        return Complex32::new(PI, 0.) / sin_pi_z / gamma(t);
    }
    // n = z - 1
    let n = Complex32::new(z.re - 1., z.im);
    // x = gamma_pval[0]
    let mut x = Complex32::new(gamma_p[0], 0.);
    // gamma_p length is 15
    // for (i, gamma_pval) in enumerate(gamma_p):
    for i in 1..g {
        // x += gamma_pval / (n + i)
        // let gamma_pval = Complex32::new(gamma_p[i], 0.);
        // x += gamma_pval / (n + Complex32::new(i as f32, 0.))
        x += Complex32::new(gamma_p[i], 0.) / (n + Complex32::new(i as f32, 0.))
    }
    // t = n + gamma_g + 0.5
    let t = Complex32::new(n.re + gamma_g + 0.5, n.im);
    // y = sqrt(2 * pi) * t ** (n + 0.5) * exp(-t) * x
    // let tpow = t.powc(Complex32::new(n.re + 0.5, n.im));
    // let exp_t = (-t).exp();
    return x
        * t.powc(Complex32::new(n.re + 0.5, n.im))
        * (-t).exp()
        * Complex32::new(sqrt_2_pi, 0.);
}

// pub fn zeta1(z: Complex32, t: i32) -> Complex32 {
//     if z.re == 1. && z.im == 0. {
//         return Complex32::new(INFINITY, 0.);
//     }
//     let mut result = Complex32::new(0., 0.);
//     for n in 0..t {
//         let mut res = Complex32::new(0., 0.);
//         for k in 0..=n {
//             let k1 = Complex32::new((k + 1) as f32, 0.).powc(-z);
//             let binom = binom(n as i128, k as i128);
//             let f = sign(k) * binom as f32;
//             let idk = k1 * Complex32::new(f, 0.);
//             res += idk;
//         }
//         let j = (2 as i128).pow((n + 1) as u32) as f32;
//         let resj = Complex32::new(res.re / j, res.im / j); // res/j
//         result += resj;
//     }
//     return result
//         / (-Complex32::new(2., 0.).powc(-z + Complex32::new(1., 0.)) + Complex32::new(1., 0.));
// }

// zeta fn
pub fn zeta3(z: Complex32, t: i32) -> Complex32 {
    if z.re > 10.0 {
        return Complex32::new(1.0, 0.0); // very rough approximation but this prevents overflow causing an err
    }
    // trivial zeroes
    if z.im == 0.0 && z.re < 0.0 && z.re % 2.0 == 0.0 {
        return Complex32::new(0.0, 0.0);
    }
    // the pole
    if z.re == 1. && z.im == 0. {
        return Complex32::new(INFINITY, 0.);
    }
    let negz = -Complex64::new(z.re as f64, z.im as f64);
    let mut result = Complex64::new(0., 0.);
    for n in 0..t {
        let mut res = Complex64::new(0., 0.);
        for k in 0..=n {
            let k1 = Complex64::new((k + 1) as f64, 0.).powc(negz);
            let binom = binom(n as i128, k as i128);
            let f = sign(k) as f64 * binom as f64;
            let idk = k1 * Complex64::new(f, 0.);
            res += idk;
        }
        let j = (2 as i128).pow((n + 1) as u32) as f64;
        let resj = Complex64::new(res.re / j, res.im / j); // res/j
        result += resj;
    }
    let returned = result
        / (-Complex64::new(2., 0.).powc(negz + Complex64::new(1., 0.)) + Complex64::new(1., 0.));
    return Complex32::new(returned.re as f32, returned.im as f32);
}

fn sign(k: i32) -> f32 {
    if k % 2 == 0 {
        return 1.0;
    } else {
        return -1.0;
    }
}

fn binom(n: i128, k: i128) -> i128 {
    (0..n + 1).rev().zip(1..k + 1).fold(1, |mut r, (n, d)| {
        r *= n;
        r /= d;
        r
    })
}

// lambertw
pub fn zexpz(z: Complex32) -> Complex32 {
    return z * z.exp();
}
//The derivative of z * exp(z) = exp(z) + z * exp(z)
pub fn zexpz_d(z: Complex32) -> Complex32 {
    return z.exp() + z * z.exp();
}
//The second derivative of z * exp(z) = 2. * exp(z) + z * exp(z)

pub fn zexpz_dd(z: Complex32) -> Complex32 {
    return Complex32::new(2., 0.) * z.exp() + z * z.exp();
}

pub fn init_point(z: Complex32, k: i32) -> Complex32 {
    let i = Complex32::new(0., 1.);
    let two_pi_k_i = Complex32::new(0., 2. * PI * k as f32);
    let mut ip = z.ln() + two_pi_k_i - (z.ln() + two_pi_k_i).ln(); // initial point coming from the general asymptotic approximation
    let p = (Complex32::new(2., 0.) * (z.exp() + Complex32::new(1., 0.))).sqrt(); // used when we are close to the branch cut around zero and when k=0,-1

    if (z - (-f32::exp(-1.))).norm() <= 1. {
        //we are close to the branch cut, the initial point must be chosen carefully
        if k == 0 {
            ip = Complex32::new(-1., 0.) + p - Complex32::new(1. / 3., 0.) * p.powi(2)
                + Complex32::new(11. / 72., 0.) * p.powi(3);
        }
        if k == 1 && (z.im < 0. || z.im > 0.) {
            ip = Complex32::new(-1., 0.) - p - Complex32::new(1. / 3., 0.) * p.powi(2)
                + Complex32::new(11. / 72., 0.) * p.powi(3);
        }
    }
    let one = Complex32::new(1., 0.);
    let two = Complex32::new(2., 0.);

    if k == 0 && (z - Complex32::new(0.5, 0.)).norm() <= 0.5 {
        let c1 = Complex32::new(0.35173371, 0.);
        let c2 = Complex32::new(0.1237166, 0.);
        let c3 = Complex32::new(7.061302897, 0.);
        let c4 = Complex32::new(0.827184, 0.0);
        ip = (c1 * (c2 + c3 * z)) / (two + c4 * (one + two * z))
    }
    if k == -1 && (z - Complex32::new(0.5, 0.)).norm() <= 0.5 {
        // (1,1) Pade approximant for W(-1,a)
        let c1 = Complex32::new(2.2591588985, 4.22096);
        let c2 = Complex32::new(-14.073271, -33.767687754);
        let c3 = Complex32::new(12.7127, 19.071643);
        let c4 = Complex32::new(17.23103, 10.629721);
        ip = -((c1 * (c2 * z - c3 * (one + two * z))) / (two - c4 * (one + two * z)));
    }
    return ip;
}

pub fn lambertw(z: Complex32, k: i32) -> Complex32 {
    if z.re == 0. && z.im == 0. {
        if k == 0 {
            return Complex32::new(0., 0.);
        } else {
            return Complex32::new(NEG_INFINITY, 0.);
        }
    }
    // if z.re == -f32::exp(-1.) && (k == 0 || k == -1) {
    //     return Complex32::new(-1., 0.);
    // }
    // if z.re == f32::exp(-1.) && k == 0 {
    //     return Complex32::new(1., 0.);
    // }
    let mut w = init_point(z, k);
    println!("{}", w);

    let mut wprev = w;
    let maxiter = 30;
    let mut iter = 0;
    let prec = 1E-30;
    let two = Complex32::new(2., 0.);

    loop {
        wprev = w;
        w -= two * ((zexpz(w) - z) * zexpz_d(w))
            / (two * zexpz_d(w).powi(2) - (zexpz(w) - z) * zexpz_dd(w));
        if (w - wprev).norm() < prec || iter > maxiter {
            break;
        }
        iter += 1;
    }
    return w;
}

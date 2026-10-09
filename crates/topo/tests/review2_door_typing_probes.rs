//! PR 4415 second review: probe families for the door's in-band typing.
//! Each row PRINTS its outcomes (`R2 ...` lines); run with
//! `--nocapture` and read them. `R2_RUN=1` turns the rows on.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanError, ShellClassifyError, ValidationError};

use crate::common;

type V3 = [f64; 3];
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

fn on() -> bool {
    std::env::var("R2_RUN").is_ok()
}

fn at_rest(b: topo::Body<f64>, tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(b, tol).expect("at rest")
}

/// The witness pose (`notch307 nt e0 a<a> d<d>`), every coordinate moved
/// `o` along x.
fn witness(a: u32, d: f64, o: f64, tol: Tol) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let profile = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)].map(|(x, y)| (x + o, y));
    let prism = at_rest(common::prism::<f64>(&profile, 1.0, tol).body, tol);
    let v: V3 = [2.0 + o, 1.0, 1.0];
    let e = unit([2.0, 1.0, 0.0]);
    let (p1, p2) = basis(e);
    let al = std::f64::consts::TAU * (f64::from(a) + 0.25) / 16.0;
    let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
    let m = unit(unit(add(base, scale(e, d))));
    let (u, w) = basis(m);
    let cube = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
            let p = add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
            Point3::new(p[0], p[1], p[2])
        },
        tol,
    );
    (prism, at_rest(cube, tol))
}

fn describe(r: &Result<topo::BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(res) => match res.body() {
            Some(b) => {
                let solids: Vec<usize> = b.body.solids().map(|(k, _)| b.body.shells_of_solid(k).map_or(0, |s| s.len())).collect();
                format!("OK shells-per-solid={solids:?}")
            }
            None => "OK empty".into(),
        },
        Err(BooleanError::Escalated { decision, diag }) => {
            format!("Escalated {decision:?} margin={:?} terminal={}", diag.margin, diag.terminal_sliver)
        }
        Err(BooleanError::ResultInvalid { errors }) => {
            let e: Vec<String> = errors
                .iter()
                .map(|e| match e {
                    ValidationError::ShellRoleUndecided { error, .. } => match error {
                        ShellClassifyError::Escalated { source, sliver, .. } => format!(
                            "ShellRoleUndecided/Escalated walk={:?} sliver={:?}",
                            source.margin,
                            sliver.as_ref().map(|s| s.reading().margin)
                        ),
                        other => format!("ShellRoleUndecided/{other:?}"),
                    },
                    other => format!("{other:?}").chars().take(160).collect(),
                })
                .collect();
            format!("ResultInvalid {e:?}")
        }
        Err(other) => format!("{other:?}").chars().take(200).collect(),
    }
}

/// The witness ∩ far from the world origin: the walk's world-origin
/// error grows with `o`, the certified reading about the body's corner
/// does not.
#[test]
fn r2_witness_far_from_the_origin() {
    if !on() {
        return;
    }
    let tol = Tol::witness();
    for d in [1e-8, 2e-8, 3e-8] {
        for o in [0.0, 1.0, 1e1, 1e2, 1e3, 3e3, 1e4, 1e5] {
            let (p, c) = witness(3, d, o, tol);
            let r = topo::intersect(&p, &c, tol);
            println!("R2 FAR d={d:e} o={o:e}: {}", describe(&r));
        }
    }
}

/// A box with a unit-square cavity `h` deep, `o` along x: the cavity's
/// `V/A` is `-h/(2+4h)`.
fn thin_void(h: f64, o: f64, tol: Tol) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let a = at_rest(common::brick::<f64>((o, o + 3.0), (0.0, 3.0), (0.0, 3.0), tol), tol);
    let b = at_rest(common::brick::<f64>((o + 1.0, o + 2.0), (1.0, 2.0), (1.0, 2.0), tol), tol);
    let hollow = topo::subtract(&a, &b, tol).expect("hollow").body().unwrap().body.clone();
    let c = at_rest(common::brick::<f64>((o + 0.5, o + 2.5), (0.5, 2.5), (0.5, 2.0 - h), tol), tol);
    topo::union(&hollow, &c, tol)
}

/// The thin void across both edges of the band, near and far from the
/// origin.
#[test]
fn r2_thin_void_across_the_band() {
    if !on() {
        return;
    }
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    // |V/A| = h/(2+4h) ≈ h/2: the zero edge at h ≈ 2ε, the escalate edge at h ≈ 2Kε.
    let mut hs: Vec<f64> = (0..=8).map(|i| k * eps * (1.02 + 0.1 * f64::from(i))).collect();
    hs.extend((0..=24).map(|i| k * eps * (1.988 + 0.001 * f64::from(i))));
    for o in [0.0, 1e2, 1e3, 1e4] {
        for &h in &hs {
            let want = -h / (2.0 + 4.0 * h);
            println!("R2 VOID o={o:e} h={h:e} V/A={want:e}: {}", describe(&thin_void(h, o, tol)));
        }
    }
}

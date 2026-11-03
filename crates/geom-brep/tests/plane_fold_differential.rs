//! **The plane folded into the 1×2 arm, read beside its ℝ⁴ reading**:
//! every fixture here is read whole by `PlaneSection` (the plane's own
//! ℝ⁴ lane) and by `ImplicitSection` with the plane as its partner (the
//! residual × chart trace, C3's 1×2), on the same plane, wall, domain and
//! band. The two traces sample differently, so their carriers differ in
//! bits; what must agree is the section: whether it is decided, how many
//! branches it has, which sides each open branch ends on, and how many
//! boundary regions it reports. No fixture decided by the ℝ⁴ reading may
//! be refused by the 1×2 one.
//!
//! The fixtures are the plane × NURBS lane's own: the certifiable and
//! the inflected wall under the tilted cutting plane, a dome's level
//! loops, and the far wall the ε-rounding rows place from 20 m to 520 m
//! off the origin, each at the run's ε (the suite runs 1e-9, 1e-6 and
//! 1e-12).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::ssi::BranchEnd;
use geom_brep::{ImplicitSection, PlaneSection, SsiDomain, SsiError, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

use crate::shared::tol::band;

/// The wall of `m5_pr7_ssi`: a cubic in `u` through four columns, linear
/// in `v`, extruded `0.8` in `z`.
fn wall_from_cols(cols: [(f64, f64); 4]) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

const CERTIFIABLE: [(f64, f64); 4] = [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)];
const INFLECTED: [(f64, f64); 4] = [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)];

/// The tilted plane at mid height, shifted `x` along the first axis.
fn cutting_plane(x: f64) -> Surface<f64> {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    Surface::Plane {
        origin: Point3::new(x, 0.0, 0.4),
        normal: n,
        u_ref: u,
    }
}

fn wall_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: 1.0,
    }
}

/// A biquadratic dome whose middle control point stands at `y = −1`.
fn dome() -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|n| {
            let (i, j) = (f64::from(n / 3), f64::from(n % 3));
            let y = if n == 4 { -1.0 } else { 0.0 };
            Point3::new(i / 2.0, y, j / 2.0)
        })
        .collect();
    NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap()
}

fn level(y: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.5, y, 0.5),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The flat wall `z = α·x` over `y ∈ [0, ½]` whose chart path bends
/// (`m5_pr7_ssi`'s `bent_path_flat_wall`): `x = X·[(s − ½) − κh(s − ½)² −
/// μh]`, `h = 4t(1 − t)`, biquadratic, cut by `z = 0` in the line
/// `x = z = 0`.
fn bent_path_flat_wall(alpha: f64, width: f64, kappa: f64, mu: f64) -> NurbsSurface<f64> {
    let lin = [-0.5, 0.0, 0.5];
    let square = [0.25, -0.25, 0.25];
    let hump = [0.0, 2.0, 0.0];
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::with_capacity(9);
    for i in 0..3 {
        for (j, h) in hump.iter().enumerate() {
            let x = width * (lin[i] - kappa * square[i] * h - mu * h);
            control.push(Point3::new(x, 0.25 * f64::from(j as u8), alpha * x));
        }
    }
    NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap()
}

fn dome_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, -0.2, 0.5),
        half_extent: 2.0,
        extent: 1.0,
        floor_scale: 1.0,
    }
}

/// What the section is, as the two readings must agree on it: decided or
/// refused (by the refusal's kind), and if decided, its branches' ends
/// (sorted) and its regions' count.
#[derive(Debug, PartialEq)]
enum Read {
    Decided { ends: Vec<String>, regions: usize },
    Refused(String),
}

fn read(r: Result<SsiOutcome, SsiError>) -> Read {
    match r {
        Ok(o) => {
            let mut ends: Vec<String> = o
                .branches
                .iter()
                .map(|b| match b.end {
                    BranchEnd::Closed => "loop".to_owned(),
                    BranchEnd::Rim { from, to } => {
                        let mut s = [from, to].map(|e| format!("{:?}", e.side));
                        s.sort();
                        format!("rim {} {}", s[0], s[1])
                    }
                    BranchEnd::Cut => "cut".to_owned(),
                })
                .collect();
            ends.sort();
            Read::Decided {
                ends,
                regions: o.boundary.len(),
            }
        }
        Err(e) => {
            let d = format!("{e:?}");
            let cut = d.find(['(', '{', ' ']).unwrap_or(d.len());
            Read::Refused(d[..cut].to_owned())
        }
    }
}

/// **No plane fixture the ℝ⁴ reading decides is refused by the 1×2
/// one, and where both decide they read one section.** Red against a
/// plane partner whose form, exclusion, side reading or march differs
/// from the plane's in what it decides.
#[test]
fn the_plane_reads_one_section_on_both_traces() {
    let b = band();
    let mut fixtures: Vec<(String, Surface<f64>, NurbsSurface<f64>, SsiDomain)> = vec![
        (
            "certifiable wall".into(),
            cutting_plane(0.0),
            wall_from_cols(CERTIFIABLE),
            wall_domain(),
        ),
        (
            "inflected wall".into(),
            cutting_plane(0.0),
            wall_from_cols(INFLECTED),
            wall_domain(),
        ),
    ];
    for y in [-0.125, -0.05, -0.2] {
        fixtures.push((format!("dome level {y}"), level(y), dome(), dome_domain()));
    }
    for x in [20.0, 60.0, 500.0, 520.0] {
        fixtures.push((
            format!("far wall at {x} m"),
            cutting_plane(x),
            wall_from_cols(CERTIFIABLE.map(|(u, v)| (u + x, v))),
            wall_domain(),
        ));
    }
    fixtures.push((
        "flat wall whose chart path bends".into(),
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        bent_path_flat_wall(1e-6, 0.01, 0.8, 0.2),
        SsiDomain {
            center: Point3::new(0.0, 0.0, 0.0),
            half_extent: 1.0,
            extent: 1.0,
            floor_scale: 1.0,
        },
    ));
    let mut table = String::new();
    let mut moved = Vec::new();
    for (what, plane, wall, domain) in &fixtures {
        let r4 = read(PlaneSection::of(plane, wall, *domain, b).and_then(PlaneSection::all));
        let full = ImplicitSection::of(plane, wall, *domain, b).and_then(ImplicitSection::all);
        let detail = full.as_ref().err().map(|e| format!("{e:?}"));
        let r2 = read(full);
        table.push_str(&format!("{what}: ℝ⁴ {r4:?} | 1×2 {r2:?}\n"));
        if let (Read::Decided { .. }, Some(d)) = (&r4, &detail) {
            table.push_str(&format!("    the 1×2 refusal: {d}\n"));
        }
        match (&r4, &r2) {
            (Read::Decided { .. }, Read::Refused(_)) => moved.push(what.clone()),
            (Read::Decided { .. }, Read::Decided { .. }) if r4 != r2 => moved.push(what.clone()),
            _ => {}
        }
    }
    eprintln!("ε {:e}\n{table}", b.zero());
    assert!(
        moved.is_empty(),
        "decided by the ℝ⁴ reading and read otherwise by the 1×2 one at ε {:e}: {moved:?}\n{table}",
        b.zero()
    );
}

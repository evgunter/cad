//! Reviewer probes for PR 5333 (cone through the periodic-chart closure,
//! per-term whole-turn charge). Each probe prints its verdict at f64 and
//! at the interval scalar, at the flux lane and the material sign.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, boundary_material_sign_loops, curved_face_loops};
use geom_core::{CertifiedEnclosure, Decide, Interval, Real};
use std::f64::consts::{PI, TAU};

const HA: f64 = 0.5;

fn cone<T: Real>() -> Surface<T> {
    Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: T::from_f64(HA),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// Rim at signed slant `v` (negative: the other nappe), azimuth u0 -> u1.
fn rim<T: Real>(v: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    let (s, c) = HA.sin_cos();
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, v * c),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(v.abs() * s),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn gen_<T: Real>(u: f64, v0: f64, v1: f64) -> LoopEdge<T> {
    let (s, c) = HA.sin_cos();
    edge(
        Curve3::Line {
            origin: p3(0.0, 0.0, 0.0),
            dir: v3(s * u.cos(), s * u.sin(), c),
        },
        v0,
        v1,
        0,
        0,
    )
}

/// A window [v0,v1]x[u0,u1], ccw in (u, v) when `with`.
fn window<T: Real>(v0: f64, v1: f64, u0: f64, u1: f64, with: bool) -> Vec<LoopEdge<T>> {
    let k = if v0 < 0.0 { PI } else { 0.0 };
    let l = vec![
        rim::<T>(v0, u0, u1),
        gen_::<T>(u1 + k, v0, v1),
        rim::<T>(v1, u1, u0),
        gen_::<T>(u0 + k, v1, v0),
    ];
    if with { l } else { reverse(l) }
}

fn reverse<T: Real>(l: Vec<LoopEdge<T>>) -> Vec<LoopEdge<T>> {
    let mut r: Vec<LoopEdge<T>> = l
        .into_iter()
        .map(|mut e| {
            e.forward = !e.forward;
            core::mem::swap(&mut e.start, &mut e.end);
            e
        })
        .collect();
    r.reverse();
    r
}

fn run<T: Decide + CertifiedEnclosure>(loops: &[Vec<LoopEdge<T>>]) -> String {
    let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    let flux = match curved_face_loops(&cone::<T>(), &refs, true, band()) {
        Ok(fc) => {
            let (lo, hi) = fc.area.certified_bracket().unwrap();
            let (flo, fhi) = fc.flux.certified_bracket().unwrap();
            format!("area {:.12} flux {:.12}", 0.5 * (lo + hi), 0.5 * (flo + fhi))
        }
        Err(e) => format!("ERR {e:?}"),
    };
    let side = match boundary_material_sign_loops(&cone::<T>(), &refs, band()) {
        Ok(s) => format!("{s:?}"),
        Err(e) => format!("ERR {e:?}"),
    };
    format!("flux[{flux}] side[{side}]")
}

fn cases<T: Decide + CertifiedEnclosure>() -> Vec<(&'static str, Vec<Vec<LoopEdge<T>>>)> {
    let up = |v: f64| vec![rim::<T>(v, 0.0, TAU)];
    let dn = |v: f64| vec![rim::<T>(v, TAU, 0.0)];
    let e = eps();
    vec![
        ("P01 band +1@1 -1@2", vec![up(1.0), dn(2.0)]),
        ("P02 band -1@1 +1@2 (hole orientation)", vec![dn(1.0), up(2.0)]),
        ("P03 band outer stored second", vec![dn(2.0), up(1.0)]),
        ("P04 band opposite nappes +1@1 -1@-1", vec![up(1.0), dn(-1.0)]),
        ("P05 band opposite nappes +1@1 +1@-1", vec![up(1.0), up(-1.0)]),
        ("P06 band opposite nappes +1@2 -1@-1", vec![up(2.0), dn(-1.0)]),
        ("P07 band + hole (against)", vec![up(1.0), dn(2.0), window(1.3, 1.6, 1.0, 2.0, false)]),
        ("P08 band + hole (with)", vec![up(1.0), dn(2.0), window(1.3, 1.6, 1.0, 2.0, true)]),
        ("P09 band + hole outside band", vec![up(1.0), dn(2.0), window(3.0, 3.3, 1.0, 2.0, false)]),
        ("P10 band split rim", vec![vec![rim::<T>(1.0, 0.0, PI), rim::<T>(1.0, PI, TAU)], dn(2.0)]),
        ("P11 band split rim, 3 arcs, seam off", vec![vec![rim::<T>(1.0, 0.3, 2.0), rim::<T>(1.0, 2.0, 4.0), rim::<T>(1.0, 4.0, TAU + 0.3)], dn(2.0)]),
        ("P12 cap +1@2", vec![up(2.0)]),
        ("P13 cap -1@2", vec![dn(2.0)]),
        ("P14 cap on lower nappe +1@-2", vec![up(-2.0)]),
        ("P15 cap + winding ring other nappe", vec![up(2.0), dn(-1.0)]),
        ("P16 cap + hole (against)", vec![up(2.0), window(1.0, 1.5, 1.0, 2.0, false)]),
        ("P17 hole first, cap second", vec![window(1.0, 1.5, 1.0, 2.0, false), up(2.0)]),
        ("P18 hole(with) first, cap second", vec![window(1.0, 1.5, 1.0, 2.0, true), up(2.0)]),
        ("P19 +1 +1 and a 0", vec![up(1.0), up(2.0), window(1.3, 1.6, 1.0, 2.0, false)]),
        ("P20 0 first then +1 +1", vec![window(1.3, 1.6, 1.0, 2.0, false), up(1.0), up(2.0)]),
        ("P21 +1 -1 +1", vec![up(1.0), dn(2.0), up(3.0)]),
        ("P22 +1 -1 +1 -1", vec![up(1.0), dn(2.0), up(3.0), dn(4.0)]),
        ("P23 same rim twice opposite", vec![up(1.0), dn(1.0)]),
        ("P24 slit cap", vec![vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, TAU), gen_::<T>(TAU, 2.0, 0.0)]]),
        ("P25 slit cap + winding ring", vec![vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, TAU), gen_::<T>(TAU, 2.0, 0.0)], dn(1.0)]),
        ("P25b slit cap + winding ring up", vec![vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, TAU), gen_::<T>(TAU, 2.0, 0.0)], up(1.0)]),
        ("P25c slit cap rev + winding ring", vec![reverse(vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, TAU), gen_::<T>(TAU, 2.0, 0.0)]), dn(1.0)]),
        ("P25d slit cap rev + winding ring up", vec![reverse(vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, TAU), gen_::<T>(TAU, 2.0, 0.0)]), up(1.0)]),
        ("P26 slit cap + ring other nappe", vec![vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, TAU), gen_::<T>(TAU, 2.0, 0.0)], dn(-1.0)]),
        ("P27 slit cap + hole", vec![vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, TAU), gen_::<T>(TAU, 2.0, 0.0)], window(1.0, 1.5, 1.0, 2.0, false)]),
        ("P28 sector through apex", vec![vec![gen_::<T>(0.0, 0.0, 2.0), rim::<T>(2.0, 0.0, 1.0), gen_::<T>(1.0, 2.0, 0.0)]]),
        ("P29 band, rim 1.5 turns + half arc back?", vec![vec![rim::<T>(1.0, 0.0, TAU + PI), rim::<T>(1.0, TAU + PI, TAU)], dn(2.0)]),
        ("P30 band, rims a period off", vec![vec![rim::<T>(1.0, TAU, 2.0 * TAU)], vec![rim::<T>(2.0, 2.0 * TAU, TAU)]]),
        ("P31 band, rims stored span 2 turns each", vec![vec![rim::<T>(1.0, 0.0, 2.0 * TAU)], vec![rim::<T>(2.0, 2.0 * TAU, 0.0)]]),
        ("P32 rect at seam", vec![vec![rim::<T>(1.0, TAU - 1.0, TAU), gen_::<T>(TAU, 1.0, 2.0), rim::<T>(2.0, TAU, TAU - 1.0), gen_::<T>(TAU - 1.0, 2.0, 1.0)]]),
        ("P33 rect, gen 2eps up", vec![vec![rim::<T>(1.0, TAU - 1.0, TAU), gen_::<T>(TAU, 1.0 + 2.0 * e, 2.0), rim::<T>(2.0, TAU, TAU - 1.0), gen_::<T>(TAU - 1.0, 2.0, 1.0)]]),
        ("P34 rect, gen 3.5eps up", vec![vec![rim::<T>(1.0, TAU - 1.0, TAU), gen_::<T>(TAU, 1.0 + 3.5 * e, 2.0), rim::<T>(2.0, TAU, TAU - 1.0), gen_::<T>(TAU - 1.0, 2.0, 1.0)]]),
        ("P35 rect, gen 10eps up", vec![vec![rim::<T>(1.0, TAU - 1.0, TAU), gen_::<T>(TAU, 1.0 + 10.0 * e, 2.0), rim::<T>(2.0, TAU, TAU - 1.0), gen_::<T>(TAU - 1.0, 2.0, 1.0)]]),
        ("P36 cap rim not closing by 2eps (az)", vec![vec![rim::<T>(2.0, 0.0, TAU - 2.0 * e)]]),
        ("P37 cap rim not closing by 10eps (az)", vec![vec![rim::<T>(2.0, 0.0, TAU - 10.0 * e)]]),
        ("P38 cap rim wrong sense in middle (+ then - arc)", vec![vec![rim::<T>(2.0, 0.0, PI), rim::<T>(2.0, PI, 0.0)]]),
        ("P39 lower nappe band", vec![up(-1.0), dn(-2.0)]),
        ("P40 lower nappe cap+hole", vec![up(-2.0), window(-1.5, -1.0, 1.0, 2.0, false)]),
        ("P41 lower nappe cap+hole(other)", vec![up(-2.0), window(-1.5, -1.0, 1.0, 2.0, true)]),
        ("P42 band with hole spanning seam", vec![up(1.0), dn(2.0), window(1.3, 1.6, TAU - 0.5, TAU + 0.5, false)]),
        ("P43 band big hole (against) larger than band? no: wide", vec![up(1.0), dn(2.0), window(1.1, 1.9, 0.1, TAU - 0.1, false)]),
        ("P44 band, hole bigger than band outside", vec![up(1.0), dn(2.0), window(3.0, 6.0, 0.1, TAU - 0.1, false)]),
        ("P45 band, hole bigger than band outside (with)", vec![up(1.0), dn(2.0), window(3.0, 6.0, 0.1, TAU - 0.1, true)]),
        ("P46 cap, hole outside cap", vec![up(2.0), window(3.0, 3.5, 1.0, 2.0, false)]),
        ("P47 cap, hole(with) outside cap", vec![up(2.0), window(3.0, 3.5, 1.0, 2.0, true)]),
        ("P48 window alone", vec![window(1.0, 2.0, 1.0, 2.0, true)]),
        ("P49 window + hole outside it", vec![window(1.0, 2.0, 1.0, 2.0, true), window(1.2, 1.4, 3.0, 3.5, false)]),
        ("P50 hole(with) first, cap -1 second", vec![window(1.0, 1.5, 1.0, 2.0, false), dn(2.0)]),
    ]
}

#[test]
fn review_5333_probe_table() {
    for (name, loops) in cases::<f64>() {
        println!("F64 {name:<55} {}", run(&loops));
    }
    for (name, loops) in cases::<Interval>() {
        println!("IV  {name:<55} {}", run(&loops));
    }
}

fn circ<T: Real>(radius: f64, z: f64, (u0, u1): (f64, f64)) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, z),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(radius),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

fn cyl<T: Real>(radius: f64) -> Surface<T> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: T::from_f64(radius),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// Claim 8: the cylinder's `props_chart_axis_inside` reads the SURFACE
/// radius, not the rim's: rims the axis class admits at 10.4ε on a
/// cylinder of radius 9.6ε.
#[test]
fn review_5333_cylinder_axis_inside_reads_the_surface_radius() {
    let e = eps();
    for (rs, rc) in [(9.6, 10.4), (9.95, 10.5), (10.5, 10.5), (10.2, 10.9)] {
        fn go<T: Decide + CertifiedEnclosure>(rs: f64, rc: f64) -> String {
            let lower = vec![circ::<T>(rc, 0.0, (0.0, TAU))];
            let upper = vec![circ::<T>(rc, 1.0, (TAU, 0.0))];
            match curved_face_loops(&cyl::<T>(rs), &[&lower, &upper], true, band()) {
                Ok(fc) => format!("area {:?}", fc.area.certified_bracket()),
                Err(e) => format!("ERR {e:?}"),
            }
        }
        println!("CYL R={rs}e rc={rc}e f64 {}", go::<f64>(rs * e, rc * e));
        println!("CYL R={rs}e rc={rc}e iv  {}", go::<Interval>(rs * e, rc * e));
    }
}

/// Near-apex cone faces: a band whose inner rim is `k·ε` from the axis
/// and outer rim at radius ~1, and a cap with a near-apex hole, f64 and
/// interval.
#[test]
fn review_5333_near_apex_cone_faces() {
    let e = eps();
    let s = HA.sin();
    for k in [10.5, 11.5, 1e2, 1e3, 1e4] {
        let v = k * e / s;
        fn go<T: Decide + CertifiedEnclosure>(v: f64) -> [String; 2] {
            let band = vec![vec![rim::<T>(v, 0.0, TAU)], vec![rim::<T>(2.0, TAU, 0.0)]];
            let arcs = vec![
                vec![rim::<T>(v, 0.0, 2.0), rim::<T>(v, 2.0, 4.0), rim::<T>(v, 4.0, TAU)],
                vec![rim::<T>(2.0, TAU, 0.0)],
            ];
            [run(&band), run(&arcs)]
        }
        let [a, b] = go::<f64>(v);
        println!("NEAR k={k} f64 band {a}\nNEAR k={k} f64 arcs {b}");
        let [a, b] = go::<Interval>(v);
        println!("NEAR k={k} iv  band {a}\nNEAR k={k} iv  arcs {b}");
    }
}

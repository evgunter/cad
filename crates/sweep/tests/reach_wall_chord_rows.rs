//! **Bars whose edges are chords of a cylinder wall, and cubes touching
//! a drum at a corner: every op answers its closed form or names the
//! door that stops it.**
//!
//! These are poses the pierce lane's sector-side curvature charge
//! refused before it was read at its peak (`boolean::sectors::side_code`).
//! Two families:
//!
//! - **Bars straddling a cap.** A bar crossing a cylinder's top (or
//!   bottom) cap, its long edges inside the cap's slab: an edge at
//!   lateral offset `|y| < r` is a CHORD of the wall, pierced twice, and
//!   between its two pierces the bar's floor meets the wall in an arc
//!   with the chord beside it — a two-edge face. The ∩ is the disc's
//!   band under the bar, times the depth. These poses once returned ∩
//!   bodies missing that face (a wrong volume, negative on some, tier 3
//!   red): the join took the chord for the section segment and never
//!   minted the arc. REACH first fixed it with a geometric test of the
//!   chord against the wall; the join's adjacency skip now reads the
//!   segment's locus instead (JOIN-1), and the section segment here
//!   lies inside the bar's floor, so no edge is ever taken for it.
//! - **Cubes touching a drum's wall at a corner**, their main diagonal
//!   along the wall's normal, inside or outside: each op is the cube's
//!   volume combined with the drum's, exactly.
//!
//! Each row runs ∪, both ∖ and ∩; a body must hold its closed-form
//! volume and pass tier 3, and a refusal must be the door the row
//! names. The bars' rows all build: a cut that notches the cap's wall
//! measures, and one whose section closes inside the wall joins
//! through the pierce rings.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, PI};

use crate::common::germ_pair::cyl;
use crate::common::operands::{framed_bar, three_arc_cylinder};
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use sweep::test_support::brick;
use topo::{Body, BooleanError, BooleanResult};

/// What one op must answer.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// A body of this volume (0 for an empty result).
    Volume(f64),
}

fn check(what: &str, r: Result<BooleanResult<f64>, BooleanError>, want: Want) {
    let tol = Tol::witness();
    match (r, want) {
        (Ok(r), Want::Volume(v)) => {
            let got = match r.body() {
                Some(b) => {
                    topo::validate_geometric(&b.body, tol)
                        .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
                    topo::mass_properties(&b.body, tol).unwrap().volume
                }
                None => 0.0,
            };
            assert!(
                (got - v).abs() <= 1e-9 * v.abs().max(1e-3),
                "{what}: volume {got}, closed form {v}"
            );
        }
        (got, want) => panic!(
            "{what}: wanted {want:?}, got {:?}",
            got.map(|r| r.body().is_some())
        ),
    }
}

/// The four ops of `a` with `b`, against `[∪, A∖B, B∖A, ∩]`.
fn four(what: &str, a: &Body<f64>, b: &Body<f64>, want: [Want; 4]) {
    let tol = Tol::witness();
    check(&format!("{what} ∪"), topo::union(a, b, tol), want[0]);
    check(&format!("{what} A∖B"), topo::subtract(a, b, tol), want[1]);
    check(&format!("{what} B∖A"), topo::subtract(b, a, tol), want[2]);
    check(&format!("{what} ∩"), topo::intersect(a, b, tol), want[3]);
}

/// `∫ 2·sqrt(1 − y²) dy` over `[y0, y1] ∩ [−1, 1]`: the area of the
/// unit disc's band between two chords.
fn band_area(y0: f64, y1: f64) -> f64 {
    let f = |y: f64| y * (1.0 - y * y).sqrt() + y.asin();
    let (a, b) = (y0.max(-1.0), y1.min(1.0));
    if b <= a { 0.0 } else { f(b) - f(a) }
}

/// **The reviewed scan.** A bar of width `0.404` along `(1, 1, 0)/√2`,
/// its centreline `c` off the axis of the unit cylinder `z ∈ [0, 2]`,
/// sunk `depth` into the top cap. Its near floor edge, at lateral
/// `c − w/2`, is a chord of the wall, clipped by the bar's own ends.
/// Every op builds, the deeper poses at `c = 0.9` through the pierce
/// rings their sections close on.
#[test]
fn a_diagonal_bar_sunk_into_a_cylinder_cap_answers_its_closed_form() {
    let (w, t0, t1) = (
        0.404_001_346_965_559_2,
        -0.688_250_512_587_890_8,
        1.738_194_969_430_420_4,
    );
    let s = FRAC_1_SQRT_2;
    let a = three_arc_cylinder(Point2::new(0.0, 0.0), 1.0, 0.0, 2.0, 0.0);
    let (vcyl, vbar) = (2.0 * PI, w * w * (t1 - t0));
    for c in [0.9_f64, 1.047, 1.15] {
        let lo = c - w / 2.0;
        let half = (1.0 - lo * lo).sqrt();
        let f = |x: f64| 0.5 * (x * (1.0 - x * x).sqrt() + x.asin()) - lo * x;
        let area = f(half.min(t1)) - f((-half).max(t0));
        for depth in [0.002, 0.0078, 0.03, 0.1, 0.3] {
            let o = Point3::new(c * s, -c * s, 2.0 - depth + w / 2.0);
            let b = framed_bar(o, Vec3::new(s, s, 0.0), t0, t1, w);
            let i = area * depth;
            let what = format!("c {c}, depth {depth}:");
            four(
                &what,
                &a,
                &b,
                [
                    Want::Volume(vcyl + vbar - i),
                    Want::Volume(vcyl - i),
                    Want::Volume(vbar - i),
                    Want::Volume(i),
                ],
            );
        }
    }
}

/// **The same class along `x`, both caps.** A bar spanning the whole
/// disc along `x`, lateral band `y ∈ [c − h, c + h]`, sunk `0.1` into
/// the top cap or raised `0.1` into the bottom one: ∩ is the disc's
/// band times the depth, whether both floor edges are chords, one is,
/// or the band runs off the disc.
#[test]
fn an_x_bar_sunk_into_either_cap_answers_its_closed_form() {
    let (w, depth) = (0.4_f64, 0.1);
    let h = w / 2.0;
    let a = three_arc_cylinder(Point2::new(0.0, 0.0), 1.0, 0.0, 2.0, 0.0);
    let (vcyl, vbar) = (2.0 * PI, w * w * 6.0);
    for c in [0.3, -0.95, 1.1] {
        let i = band_area(c - h, c + h) * depth;
        for (cap, zc) in [("top", 2.0 - depth + h), ("bottom", depth - h)] {
            let b = framed_bar(
                Point3::new(0.0, c, zc),
                Vec3::new(1.0, 0.0, 0.0),
                -3.0,
                3.0,
                w,
            );
            four(
                &format!("x-bar c {c}, {cap} cap:"),
                &a,
                &b,
                [
                    Want::Volume(vcyl + vbar - i),
                    Want::Volume(vcyl - i),
                    Want::Volume(vbar - i),
                    Want::Volume(i),
                ],
            );
        }
    }
}

/// The rigid motion taking the cube `[0, l]³`'s corner at the origin to
/// `p`, its main diagonal to `diag`, spun `spin` about it.
fn cube_at(p: Point3<f64>, diag: Vec3<f64>, spin: f64, l: f64) -> Body<f64> {
    let tol = Tol::witness();
    let u1 = Vec3::new(1.0, 1.0, 1.0).normalize();
    let u2 = Vec3::new(1.0, -1.0, 0.0).normalize();
    let u3 = u1.cross(u2);
    let a = diag.normalize();
    let b0 = a.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    let c0 = a.cross(b0);
    let b = b0 * spin.cos() + c0 * spin.sin();
    let c = a.cross(b);
    let col = |j: usize| {
        let pick = |u: Vec3<f64>| [u.x, u.y, u.z][j];
        a * pick(u1) + b * pick(u2) + c * pick(u3)
    };
    let m = Mat3::from_cols(col(0), col(1), col(2));
    let cube = brick((0.0, l), (0.0, l), (0.0, l), tol);
    topo::transform_rigid(&cube, &Affine3::from_parts(m, p - Point3::origin()), tol)
        .expect("the cube moves")
}

/// **A cube touching a drum's wall at one corner.** Its main diagonal
/// runs along the wall's normal, so every edge leaves the corner at
/// `d̂·n̂ = ∓1/√3`: inward, the cube lies inside the drum (its corners
/// say so, and the drum is convex); outward, it lies beyond the wall's
/// tangent plane.
#[test]
fn a_cube_touching_a_drum_at_a_corner_answers_its_closed_form() {
    let drum = cyl(1.0, 2.0);
    let vd = 4.0 * PI;
    let phi = 0.7_f64;
    let p = Point3::new(phi.cos(), phi.sin(), 0.3);
    let n = Vec3::new(phi.cos(), phi.sin(), 0.0);
    for l in [0.65, 0.8] {
        for spin in [0.0, 0.4] {
            let cube3 = l * l * l;
            let inner = cube_at(p, -n, spin, l);
            assert!(
                inner
                    .vertices()
                    .map(|(_, v)| *inner.get_point(v.point).unwrap())
                    .filter(|q| (*q - p).norm() > 1e-9)
                    .all(|q| q.x.hypot(q.y) < 1.0 && q.z.abs() < 2.0),
                "l {l} spin {spin}: the inner cube's corners are inside the drum"
            );
            four(
                &format!("inner cube l {l} spin {spin}:"),
                &drum,
                &inner,
                [
                    Want::Volume(vd),
                    Want::Volume(vd - cube3),
                    Want::Volume(0.0),
                    Want::Volume(cube3),
                ],
            );
            four(
                &format!("outer cube l {l} spin {spin}:"),
                &drum,
                &cube_at(p, n, spin, l),
                [
                    Want::Volume(vd + cube3),
                    Want::Volume(vd),
                    Want::Volume(cube3),
                    Want::Volume(0.0),
                ],
            );
        }
    }
}

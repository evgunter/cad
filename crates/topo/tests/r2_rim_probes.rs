//! **Reviewer probes for the rim door** ([`topo::query::rim_of`], PR
//! 1821 review lane r2) — the rows that can go RED when the door's
//! ORDER claim breaks.
//!
//! `rim_of`'s doc states two things about order: the answer starts at
//! the seed, and it runs "in the direction `edge`'s carrier parameter
//! increases". Every rim in the shipped corpus is ONE or TWO arcs
//! (measured: ten rims, `{1: 4, 2: 6}`), and at two arcs both claims
//! are satisfied by every possible answer — `[a, b]` and `[b, a]` are
//! rotations of each other, and the only non-seed arc is the second
//! one whichever way the walk runs. So no shipped row can distinguish
//! the stated direction from its reverse.
//!
//! This fixture is a THREE-arc rim, where the two directions give
//! different `Vec`s, and the row names the one the doc promises.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::query::rim_of;
use topo::{Body, EdgeKey, FaceSurface, MefSite, MevSite};

const RIM_Z: f64 = 0.5;

fn rim_r() -> f64 {
    (1.0 - RIM_Z * RIM_Z).sqrt()
}

fn at(theta: f64) -> Point3<f64> {
    let r = rim_r();
    Point3::new(r * theta.cos(), r * theta.sin(), RIM_Z)
}

fn rim_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, RIM_Z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: rim_r(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **A spherical cap whose rim is split into THREE arcs**, at
/// parameters `0 → τ/3 → 2τ/3 → τ`, each arc stated on the SAME
/// circle value bit for bit and each `he_plus`-forward in the
/// direction that parameter increases.
///
/// Returns the arcs in the order the construction laid them down,
/// which is the carrier's positive direction.
fn three_arc_rim() -> (Body<f64>, [EdgeKey; 3]) {
    let tol = Tol::witness();
    let third = TAU / 3.0;
    let mut body = Body::<f64>::new();

    let seed = body.mvfs(at(0.0)).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();

    let a = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(third),
            EdgeCurveSpec::arc_of_circle(rim_circle(), 0.0, third).unwrap(),
            tol,
        )
        .unwrap()
        .edge;

    // Grow the second arc as a strut off the far vertex: the empty-run
    // `Fan` site, whose `he1 == he2` starts at that vertex.
    let a_back = body.get_edge(a).unwrap().he_minus;
    let b = body
        .mev(
            MevSite::Fan {
                he1: a_back,
                he2: a_back,
            },
            at(2.0 * third),
            EdgeCurveSpec::arc_of_circle(rim_circle(), third, 2.0 * third).unwrap(),
            tol,
        )
        .unwrap()
        .edge;

    // Close the rim: join the third vertex back to the first, minting
    // the disc that gives every arc a SECOND surface.
    let b_back = body.get_edge(b).unwrap().he_minus;
    let a_fwd = body.get_edge(a).unwrap().he_plus;
    let c = body
        .mef(
            MefSite::Chords {
                he1: b_back,
                he2: a_fwd,
            },
            EdgeCurveSpec::arc_of_circle(rim_circle(), 2.0 * third, TAU).unwrap(),
            FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, 0.0, RIM_Z),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
            tol,
        )
        .unwrap()
        .edge;

    (body, [a, b, c])
}

/// **A three-arc rim is answered in the carrier's positive direction,
/// from every one of its arcs.**
///
/// The whole of the door's order contract, on the smallest fixture
/// that can falsify it: the seed first, then the arc the seed's
/// parameter runs INTO, then the next. At three arcs the reverse walk
/// is a different `Vec` — `[a, c, b]` — so this row goes red on a
/// direction flip, which no corpus row does.
#[test]
fn a_three_arc_rim_is_ordered_in_the_carriers_positive_direction_from_every_seed() {
    let (body, [a, b, c]) = three_arc_rim();

    assert_eq!(rim_of(&body, a).unwrap(), vec![a, b, c], "seeded at arc 0");
    assert_eq!(rim_of(&body, b).unwrap(), vec![b, c, a], "seeded at arc 1");
    assert_eq!(rim_of(&body, c).unwrap(), vec![c, a, b], "seeded at arc 2");
}

/// **Determinism (D9): the same body and seed answer identically on
/// repeat**, at a rim big enough for the answer to have a choice.
#[test]
fn a_three_arc_rims_answer_repeats() {
    let (body, [a, _, _]) = three_arc_rim();
    let first = rim_of(&body, a).unwrap();
    assert_eq!(rim_of(&body, a).unwrap(), first);
    assert_eq!(first.len(), 3, "not vacuous: the rim really is three arcs");
}

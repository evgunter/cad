//! **R2 review probes for MESH-8** (issue 868, which moved three
//! input-quality detectors out of the walk). Independent instruments,
//! written to falsify the PR's claims rather than to re-assert them.
//!
//! Every row here PRINTS what it measured; the assertions are only on
//! the parts a reviewer is confident about, so a row that goes red is
//! a finding and not a re-baselining chore.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

// `tests/common/` is named because `witness_bodies` and the shared tolerance live
// there. A marker's own file is implicit; a sibling helper module is not.
test_utils::gated_to![
    "crates/mesh/src/",
    "crates/topo/src/splitting/",
    "crates/mesh/tests/common/",
];

use crate::common;
use common::*;
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::Body;
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite, MevSite};

// ---------------------------------------------------------------
// D9: an INDEPENDENT byte digest (claim 4)
// ---------------------------------------------------------------

/// A different mixer from `r2_bytes`' FNV-1a, on purpose: two
/// instruments agreeing is evidence, one instrument run twice is a
/// tautology. 64-bit xorshift-multiply over every position bit
/// pattern, every index and every boundary id, order-sensitive.
fn mixdigest(words: impl IntoIterator<Item = u64>) -> u64 {
    let mut h: u64 = 0x243f_6a88_85a3_08d3;
    for w in words {
        h ^= w.wrapping_add(0x9e37_79b9_7f4a_7c15);
        h = h.rotate_left(23).wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 29;
    }
    h
}

fn body_tour() -> Vec<(&'static str, Body<f64>)> {
    vec![
        ("ball", ball()),
        ("cone", cone()),
        ("l_prism", l_prism()),
        ("washer", washer()),
        ("donut", donut()),
        ("sphere_wedge", sphere_wedge(2.0)),
        ("cone_wedge", cone_wedge(2.0, 1.3)),
        ("rounded_prism", rounded_prism()),
        ("holed_prism", holed_prism()),
        ("wedge", wedge()),
        ("axis_wedge", axis_wedge()),
    ]
}

/// **The D9 leg, independently.** Print one line per (body, δ): the
/// counts and a digest over the mesh's bytes. The detectors gated
/// nothing, so every line must be identical at the merge base and at
/// the head — diff the two runs' output.
#[test]
fn r2r_independent_byte_digest() {
    println!("# eps = {:e}", Tol::witness().eps());
    for (name, b) in body_tour() {
        for d in [0.1f64, 0.02, 0.004] {
            let m = mesh::tessellate(&b, d, Tol::witness()).unwrap();
            let mut words: Vec<u64> = Vec::new();
            for p in &m.positions {
                words.push(p.x.to_bits());
                words.push(p.y.to_bits());
                words.push(p.z.to_bits());
            }
            let mut tris = 0usize;
            for patch in &m.patches {
                tris += patch.triangles.len();
                for t in &patch.triangles {
                    words.extend(t.iter().map(|i| u64::from(*i)));
                }
            }
            for pl in &m.boundaries {
                words.push(pl.points.len() as u64);
                words.extend(pl.points.iter().map(|i| u64::from(*i)));
            }
            println!(
                "DIGEST {name} d={d} pos={} tri={tris} bnd={} h={:016x}",
                m.positions.len(),
                m.boundaries.len(),
                mixdigest(words)
            );
        }
    }
}

// ---------------------------------------------------------------
// Issue 1571's body through `tessellate` — the BEFORE/AFTER row
// (claims 5 and 8)
// ---------------------------------------------------------------

/// **Issue 1571's witness, re-derived here.** A unit sphere with a rim
/// at `z = 0.5` and ONE great-circle arc from the rim's `u = π` end
/// OVER the north pole to its `u = 0` end.
///
/// The body is declared as a `Body` here and in
/// `mesh/tests/common/witness_bodies.rs` (the shared home, which
/// `mesh7r1_probes` and `mesh11_arc_branch` both use), plus twice as
/// hand-built `LoopEdge` loops for the props doors
/// (`geom-brep/tests/cert1_sphere_polar.rs`,
/// `geom-brep/tests/mesh11_arc_branch.rs`), which cannot share the
/// `Body` home. This copy stays separate because an independent
/// instrument that re-derives its subject is evidence in a way one that
/// imports it is not.
fn pole_crossing_half_cap() -> Body<f64> {
    let tol = Tol::witness();
    let rim_r = (1.0f64 - 0.25).sqrt();
    let a = Point3::new(rim_r, 0.0, 0.5);
    let b = Point3::new(-rim_r, 0.0, 0.5);
    let sphere = Surface::Sphere {
        frame: geom::Frame::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        radius: 1.0,
    };
    let rim = Curve3::circle(
        Point3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        rim_r,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let great = |axis: Vec3<f64>| {
        Curve3::circle(
            Point3::new(0.0, 0.0, 0.0),
            axis,
            1.0,
            Vec3::new(-rim_r, 0.0, 0.5),
        )
    };
    let mut g = great(Vec3::new(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < 0.5 {
        g = great(Vec3::new(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: sphere,
            sense: true,
        },
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_rim.he_minus,
            he2: e_rim.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(g, 0.0, t_end).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// **What `tessellate` does on the π-rad witness, at both δ rows.**
///
/// At the merge base this row prints the `closing_column` panic and
/// its numbers at δ = 0.5; at this head it must print whatever the
/// deletion leaves — an `Ok` mesh, a typed refusal, or a panic from a
/// DIFFERENT guard (the issue-897 census). Run it with debug
/// assertions on and off.
#[test]
fn r2r_the_1571_body_through_tessellate() {
    let tol = Tol::witness();
    let body = pole_crossing_half_cap();
    println!("DEBUG_ASSERTIONS = {}", cfg!(debug_assertions));
    for d in [0.5f64, 0.1] {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            mesh::tessellate(&body, d, tol)
        }));
        std::panic::set_hook(hook);
        let says = match out {
            Ok(Ok(m)) => {
                let watertight = mesh::validate::check_mesh(&m).is_ok();
                format!(
                    "Ok({} tris, watertight={watertight})",
                    m.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
                )
            }
            Ok(Err(e)) => format!("Err({e:?})"),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
                    .unwrap_or_default();
                format!("PANIC({msg})")
            }
        };
        println!("W1571 delta={d} eps={:e} {says}", tol.eps());
    }
}

//! R2 review probes for CERT-9 / issue 303. Authored by R2; adopted
//! onto the branch by the fix pass as the permanent record of the
//! review's measurements (each probe attacks one claim of PR 1361).
//! Fix-pass edit (disclosed in PR 1361): P2 tolerates typed upstream
//! refusals, which its non-dyadic offset fixtures hit at the
//! ε = 1e-12 CI band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use mesh::Mesh;
use mesh::tessellate;
use mesh::validate::{check_mesh, signed_volume};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

/// The PRE-FIX spelling, verbatim: fold anchored at the world origin.
fn origin_fold(m: &Mesh) -> f64 {
    let o = Point3::origin();
    let mut six_v = 0.0;
    for patch in &m.patches {
        for tri in &patch.triangles {
            let a = m.positions[tri[0] as usize] - o;
            let b = m.positions[tri[1] as usize] - o;
            let c = m.positions[tri[2] as usize] - o;
            six_v += a.dot(b.cross(c));
        }
    }
    six_v / 6.0
}

fn prism_on(plane: SketchPlane<f64>, poly: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| Point2::new(x, y)));
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("profile validation");
    extrude(&vp, Extrusion::Distance(h), Tol::witness())
        .expect("extrude")
        .body
}

fn plane_at(o: f64) -> SketchPlane<f64> {
    SketchPlane::from_frame(
        Point3::new(o, o, o),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

fn mesh_of(plane: SketchPlane<f64>, poly: &[(f64, f64)], h: f64, delta: f64) -> Mesh {
    tessellate(&prism_on(plane, poly, h), delta, Tol::witness()).expect("tessellate")
}

/// Like `mesh_of`, but reports an upstream TYPED refusal instead of
/// panicking — the e2e consumer's actual experience.
fn try_mesh_of(plane: SketchPlane<f64>, poly: &[(f64, f64)], h: f64, delta: f64) -> Option<Mesh> {
    let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| Point2::new(x, y)));
    let vp = match Profile::new(plane, vec![lp]).validate(Tol::witness()) {
        Ok(v) => v,
        Err(e) => {
            println!("    REFUSED at profile validation: {e:?}");
            return None;
        }
    };
    let body = match extrude(&vp, Extrusion::Distance(h), Tol::witness()) {
        Ok(x) => x.body,
        Err(e) => {
            println!("    REFUSED at extrude: {e:?}");
            return None;
        }
    };
    match tessellate(&body, delta, Tol::witness()) {
        Ok(m) => Some(m),
        Err(e) => {
            println!("    REFUSED at tessellate: {e:?}");
            None
        }
    }
}

/// P1 — CLAIM 1's premise. The exactness argument needs a CLOSED mesh.
/// Drop one triangle: the mesh now has boundary, the total area vector
/// is nonzero, and the anchor MATTERS. Does the recentring change the
/// open-mesh answer? Print both spellings.
#[test]
fn p1_open_mesh_answer_changed_by_the_recentring() {
    for off in [0.0, 1.0e3] {
        let mut m = mesh_of(
            plane_at(off),
            &[(0.0, 0.0), (2.7, 0.0), (2.7, 1.3), (0.0, 1.3)],
            0.9,
            1e-2,
        );
        let closed_new = signed_volume(&m);
        let closed_old = origin_fold(&m);
        assert_eq!(check_mesh(&m), Ok(()), "precondition: closed");
        // Open it.
        let dropped = m.patches[0].triangles.pop().expect("a triangle");
        assert!(matches!(
            check_mesh(&m),
            Err(mesh::validate::MeshError::BoundaryEdge { .. })
        ));
        let open_new = signed_volume(&m);
        let open_old = origin_fold(&m);
        println!(
            "off={off:e} dropped={dropped:?}\n  CLOSED new={closed_new:.17e} old={closed_old:.17e}\n\
             \x20 OPEN   new={open_new:.17e} old={open_old:.17e}  delta={:.6e}",
            open_new - open_old
        );
    }
}

/// P2 — CLAIM 2's non-dyadic reasoning. A dyadic axis-aligned box is
/// said to cancel pairwise-exactly and hide the defect. Measure the
/// PRE-FIX drift for a dyadic box beside the PR's non-dyadic prism.
#[test]
fn p2_dyadic_box_hides_the_prefix_defect() {
    for (label, poly, h) in [
        (
            "dyadic  2x1x0.5",
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)][..],
            0.5,
        ),
        (
            "nondyad 2.7x1.3x0.9",
            &[(0.0, 0.0), (2.7, 0.0), (2.7, 1.3), (0.0, 1.3)][..],
            0.9,
        ),
    ] {
        let Some(m0) = try_mesh_of(SketchPlane::xy(), poly, h, 1e-2) else {
            continue;
        };
        let (b0_old, b0_new) = (origin_fold(&m0), signed_volume(&m0));
        for off in [1.0e3, 1.0e6, 1.0e8] {
            let Some(m) = try_mesh_of(plane_at(off), poly, h, 1e-2) else {
                continue;
            };
            let old = origin_fold(&m);
            let new = signed_volume(&m);
            println!(
                "{label} off={off:e}: PRE-FIX drift {:.5e}   POST-FIX drift {:.5e}",
                ((old - b0_old) / b0_old).abs(),
                ((new - b0_new) / b0_new).abs()
            );
        }
    }
}

/// P3 — CLAIM 3, the anchor's own arithmetic. `o = lo + (hi−lo)*0.5`.
/// Translate a real mesh's positions to extreme placements and to a
/// mixed-sign span, and see what the anchor does.
#[test]
fn p3_anchor_arithmetic_at_extreme_placements() {
    let base = mesh_of(
        SketchPlane::xy(),
        &[(0.0, 0.0), (2.7, 0.0), (2.7, 1.3), (0.0, 1.3)],
        0.9,
        1e-2,
    );
    let truth = signed_volume(&base);
    for shift in [1.0e8_f64, 1.0e150, 1.0e300, 1.0e308] {
        let mut m = base.clone();
        for p in &mut m.positions {
            *p = Point3::new(p.x + shift, p.y + shift, p.z + shift);
        }
        println!(
            "shift={shift:e}: new={:.17e} (rel {:.3e})  old={:.17e}",
            signed_volume(&m),
            ((signed_volume(&m) - truth) / truth).abs(),
            origin_fold(&m)
        );
    }
    // Mixed-sign span wider than f64 max: does `hi − lo` overflow?
    let mut m = base.clone();
    for (i, p) in m.positions.iter_mut().enumerate() {
        let s = if i % 2 == 0 { -1.0e308 } else { 1.0e308 };
        *p = Point3::new(p.x + s, p.y + s, p.z + s);
    }
    println!(
        "mixed-sign +-1e308: new={:.6e} old={:.6e}",
        signed_volume(&m),
        origin_fold(&m)
    );
}

/// P4 — E2E, and CLAIM 1's "body-interior anchor" wording. An L-shaped
/// (non-convex) body: the bbox centre is NOT in the material. Read the
/// volume through the public tessellation door at a near and a far
/// placement, as a consumer would.
#[test]
fn p4_e2e_l_shaped_body_near_and_far() {
    // L: 3x3 square with a 2x2 bite out of the +x+y corner. Area 5.
    let l = [
        (0.0, 0.0),
        (3.0, 0.0),
        (3.0, 1.0),
        (1.0, 1.0),
        (1.0, 3.0),
        (0.0, 3.0),
    ];
    let rect = [(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)];
    for (label, poly, exact) in [
        ("L-nonconvex", &l[..], 5.0 * 0.7),
        ("rect-control", &rect[..], 3.0 * 0.7),
    ] {
        let mut near = f64::NAN;
        for off in [0.0, 1.0e3, 1.0e5, 1.0e7, 1.0e8] {
            println!("  {label} off={off:e}:");
            let Some(m) = try_mesh_of(plane_at(off), poly, 0.7, 1e-2) else {
                continue;
            };
            let closed = check_mesh(&m);
            let v = signed_volume(&m);
            if off == 0.0 {
                near = v;
            }
            println!(
                "    v={v:.17e} closed={closed:?} rel-vs-exact {:.3e} rel-vs-near {:.3e} \
                 pre-fix would read {:.6e}",
                ((v - exact) / exact).abs(),
                ((v - near) / near).abs(),
                origin_fold(&m)
            );
        }
    }
}

/// P5 — CLAIM 4's blind spot: a differently-shaped sweep. Not a test of
/// behavior; it records what my own grep found (see report).
#[test]
fn p5_sweep_placeholder() {}

/// P6 — CLAIM 2's headline digit: reproduce the huge-offset red value
/// (PR body says 33.333333333333336 against a true 1e-9) with the
/// pre-fix spelling, independently of the PR's own row.
#[test]
fn p6_reproduce_the_huge_offset_red_digit() {
    let m = mesh_of(
        plane_at(1.0e8),
        &[(0.0, 0.0), (1.0e-3, 0.0), (1.0e-3, 1.0e-3), (0.0, 1.0e-3)],
        1.0e-3,
        1e-6,
    );
    let (old, new) = (origin_fold(&m), signed_volume(&m));
    println!(
        "huge-offset 1e-3 cube @1e8: PRE-FIX={old:.17e} (rel {:.3e})  \
         POST-FIX={new:.17e} (rel {:.3e})",
        ((old - 1e-9) / 1e-9).abs(),
        ((new - 1e-9) / 1e-9).abs()
    );
}

/// P7 — the new early return. A mesh with triangles but NO positions
/// is corrupt: pre-fix it panicked on the index, post-fix it returns a
/// quiet 0.0. Same class as P3's 1e300 NaN→0.0.
#[test]
fn p7_early_return_swallows_a_corrupt_mesh() {
    let mut m = mesh_of(
        SketchPlane::xy(),
        &[(0.0, 0.0), (2.7, 0.0), (2.7, 1.3), (0.0, 1.3)],
        0.9,
        1e-2,
    );
    m.positions.clear();
    assert!(!m.patches.iter().all(|p| p.triangles.is_empty()));
    println!(
        "positions cleared, {} patches kept: signed_volume={:?}  (check_mesh={:?})",
        m.patches.len(),
        signed_volume(&m),
        check_mesh(&m)
    );
}

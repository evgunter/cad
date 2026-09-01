//! R2 REVIEW PROBES for MATE-3 (PR 1423), sweep side. Reviewer branch
//! only.
//!
//! Two things the PR asserts but does not commit a row for:
//!
//! 1. **Reachability honesty** (§3): `.cusp()` profile → `validate`
//!    accepts → `extrude` BUILDS the cusp solid → `validate_geometric`
//!    refuses typed `UndeclaredCusp`. The PR ran this as an
//!    uncommitted probe ("`crates/sweep` source is fenced out of this
//!    unit" — but `crates/sweep/tests` was NOT: the unit already moves
//!    a row in `tests/m9_3_zip.rs`). Executed here.
//!
//! 2. **The exactness of the reversal** (§2): "`Dir::reversed`, never
//!    `ang + π`". This probe measures the residual the distinction
//!    controls, so the claim is observable.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Open, Profile, SketchPlane, Start};
use sweep::{Extrusion, extrude};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The lune: the cross-section of D1's kissing-cylinders figure,
/// authored through the new door. Circles (0,1) r 1 and (0,2) r 2 are
/// internally tangent at the origin; the region kept is the x ≥ 0 lip.
fn lune() -> profile::ClosedLoop<f64> {
    let tol = Tol::witness();
    Open.at(p2(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(p2(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
}

/// **The reachability chain, executed.** Nothing on it may proceed
/// silently, and nothing may emit a declaration the author never made.
#[test]
fn r2_cusp_profile_extrudes_to_a_solid_that_refuses_typed_at_rest() {
    let tol = Tol::witness();
    let loops = vec![profile::ProfileLoop::from(lune())];
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let validated = Profile::new(plane, loops)
        .validate(tol)
        .expect("the .cusp() profile validates: the joint is DECLARED");
    let ext =
        extrude(&validated, Extrusion::Distance(1.0), tol).expect("extrude BUILDS the cusp solid");
    let body = ext.body;
    println!(
        "R2 chain: v/e/f = {}/{}/{}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    // Structural tiers pass; the at-rest geometric gate is what catches
    // it, and it must be TYPED, naming the cusp end.
    assert_eq!(topo::validate_closed(&body), Ok(()));
    let errs = topo::validate_geometric(&body, tol)
        .expect_err("the extruded cusp solid must refuse at rest");
    println!("R2 chain: at-rest verdict {errs:?}");
    assert!(
        errs.iter().any(|e| matches!(
            e,
            topo::ValidationError::UndeclaredCusp {
                wedge: geom_brep::MaterialWedge::Cusp,
                ..
            }
        )),
        "the refusal must be the typed UndeclaredCusp: {errs:?}"
    );
    // And the declaration the author DID make (on the profile joint) is
    // NOT smuggled into the body: the wall pair carries no contact
    // record, so declaring it takes an explicit act by the caller.
    let cusp_edges: Vec<_> = errs
        .iter()
        .filter_map(|e| match e {
            topo::ValidationError::UndeclaredCusp { edge, .. } => Some(*edge),
            _ => None,
        })
        .collect();
    assert_eq!(cusp_edges.len(), 1, "exactly one cusp edge: {cusp_edges:?}");
    let e = body.get_edge(cusp_edges[0]).unwrap();
    let face_of = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        body.get_loop(l).unwrap().face
    };
    let decl = [topo::DeclaredContact {
        a: face_of(e.he_plus),
        b: face_of(e.he_minus),
        class: topo::ContactClass::Tangent,
    }];
    let after = topo::validate_geometric_declared(&body, &decl, tol);
    println!("R2 chain: declared verdict {after:?}");
    assert_eq!(after, Ok(()), "declared, the extruded cusp solid is legal");
}

/// **The reversal's exactness, measured.** The departure ray must be
/// the incoming ray NEGATED. `Dir` is private, so this reads the
/// consequence the negation controls: the two arcs meeting at the kiss
/// have exactly opposite unit tangents there, reconstructed from the
/// stored vertices and bulges.
///
/// Printed rather than asserted at a threshold, so the number is the
/// evidence: with the exact negation it is 0; with `ang + π` it is a
/// few ulp, which every band in the project accepts — i.e. the
/// distinction the PR headlines is real but unobserved by any row.
#[test]
fn r2_the_cusp_reversal_residual_is_measured() {
    let lp = lune();
    let raw = profile::ProfileLoop::from(lp);
    let v: Vec<profile::ProfileVertex<f64>> = raw.vertices().to_vec();
    let n = v.len();
    // The kiss is vertex 2 (the loop is: (0,4) → (0,2) → (0,0) kiss →
    // back). Incoming arc is v[1]→v[2]; outgoing arc is v[2]→v[3 % n].
    let tang_end = |a: usize, b: usize| {
        let p0 = v[a].pos();
        let p1 = v[b].pos();
        let bulge = v[a].bulge();
        // End tangent of a circular arc: chord direction rotated by
        // +2*atan(bulge) ... at the END the rotation is +Δ where
        // tan(Δ/2) = bulge.
        let d = p1 - p0;
        let delta: f64 = bulge.atan();
        let (s, c) = (2.0f64 * delta).sin_cos();
        let t = geom_core::Vec2::new(d.x * c - d.y * s, d.x * s + d.y * c);
        t / t.norm_squared().sqrt()
    };
    let tang_start = |a: usize, b: usize| {
        let p0 = v[a].pos();
        let p1 = v[b].pos();
        let bulge = v[a].bulge();
        let d = p1 - p0;
        let delta: f64 = bulge.atan();
        let (s, c) = (-2.0f64 * delta).sin_cos();
        let t = geom_core::Vec2::new(d.x * c - d.y * s, d.x * s + d.y * c);
        t / t.norm_squared().sqrt()
    };
    let incoming = tang_end(1, 2);
    let outgoing = tang_start(2, 3 % n);
    let sum = incoming + outgoing;
    println!(
        "R2 exactness: incoming {incoming:?} outgoing {outgoing:?} |sum| {:e}",
        sum.norm_squared().sqrt()
    );
}

//! **A closed tangent chain of mixed links fillets whole.**
//!
//! A rounded outline — lines and arcs meeting tangentially, its links on
//! a plane–plane pair and a plane–cylinder pair by turns — is one closed
//! chain on the face every link shares. The band is the rolling ball's
//! along the whole chain: a cylinder over each line, a torus over each
//! arc, consecutive bands meeting G1 on the section they share at each
//! junction (`fillet3_chain_g1`'s band reading).
//!
//! The oracle is Pappus. A convex round of radius `r` removes the
//! section `A = r²(1 − π/4)` between the square corner and the ball,
//! whose centroid sits `d = r(10 − 3π)/(3(4 − π))` in from both faces.
//! The pieces abut at the junctions' section planes, so a rim removes
//! `A·(P − 2πd)` for an outline of perimeter `P` turning once: the
//! lines' prisms `A·L` and each arc's wedge `A·θ·(R − d)`. A concave
//! round adds the same section on the same path where the outline
//! bounds the material inside it (a pocket's floor).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::common::interval::iv;
use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point2, Point3, Tol};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, chamfer_edges, fillet_edges};
use sweep::test_support::{assert_naming_totality, at_rest, extruded};
use topo::{Body, EdgeKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The rounded rectangle's half-extents and corner radius.
const HX: f64 = 1.0;
const HY: f64 = 0.6;
const R: f64 = 0.3;
const H: f64 = 0.5;

/// The outline's perimeter: four lines and a whole turn of radius `R`.
fn perimeter() -> f64 {
    4.0 * (HX - R) + 4.0 * (HY - R) + 2.0 * PI * R
}

/// The removed or added section of a round of radius `r`, and its
/// centroid's inset from each face.
fn section(r: f64) -> (f64, f64) {
    (
        r * r * (1.0 - PI / 4.0),
        r * (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI)),
    )
}

/// **The plate**: `[−1, 1] × [−0.6, 0.6]` about `(x0, y0)`, corner
/// radius `0.3`, from `z0` up `h`, at any scalar.
fn plate<T: Decide + topo::AtRestPolicy>(x0: f64, y0: f64, z0: f64, h: f64) -> Body<T> {
    let q = T::from_f64((FRAC_PI_2 / 4.0).tan());
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    let (ax, ay) = (HX - R, HY - R);
    let plane = SketchPlane::xy_at(Point3::new(
        T::from_f64(x0),
        T::from_f64(y0),
        T::from_f64(z0),
    ));
    extruded(
        plane,
        vec![bulge_loop(vec![
            (p(-ax, -HY), T::zero()),
            (p(ax, -HY), q),
            (p(HX, -ay), T::zero()),
            (p(HX, ay), q),
            (p(ax, HY), T::zero()),
            (p(-ax, HY), q),
            (p(-HX, ay), T::zero()),
            (p(-HX, -ay), q),
        ])],
        T::from_f64(h),
        tol(),
    )
}

fn every_edge<T: Decide>(body: &Body<T>) -> Vec<EdgeKey> {
    let mut all = topo::query::all_edges(body);
    all.sort_unstable();
    all
}

/// The edges of `body` whose two ends lie at height `z`.
fn rim_at<T: Decide + Bounds>(body: &Body<T>, z: f64) -> Vec<EdgeKey> {
    let mid = |x: T| 0.5 * (x.lo() + x.hi());
    every_edge(body)
        .into_iter()
        .filter(|&e| {
            let at = |v| {
                let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                (mid(p.z) - z).abs() < 1e-9
            };
            let (s, t) = body.edge_vertices(e).unwrap();
            at(s) && at(t)
        })
        .collect()
}

/// A body's volume and the half-width of its certified enclosure.
fn volume<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(body: &Body<T>) -> (f64, f64) {
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("certified props");
    let (lo, hi) = (p.enclosure.volume.lo(), p.enclosure.volume.hi());
    (0.5 * (lo + hi), 0.5 * (hi - lo) + p.enclosure.volume_pad)
}

/// Fillet every edge of the plate at `r` and check the carve: tier 3,
/// naming total, one band face and one section per link of each rim,
/// and the volume removed at Pappus's closed form.
fn plate_row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    r: f64,
    size: T,
) {
    let body = plate::<T>(0.0, 0.0, 0.0, H);
    let edges = every_edge(&body);
    let out = fillet_edges(&at_rest(&body, tol()), &edges, size, tol())
        .unwrap_or_else(|e| panic!("{what}: the rounded plate fillets whole, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    let requested: Vec<EdgeKey> = rim_at(&body, 0.0)
        .into_iter()
        .chain(rim_at(&body, H))
        .collect();
    assert_naming_totality(&body, &out, &requested, what);
    let rec = out.naming.as_ref().expect("the carve's birth records");
    assert_eq!(
        rec.blends.len(),
        16,
        "{what}: one band face per link of both rims"
    );
    assert_eq!(
        rec.sections.len(),
        16,
        "{what}: one section per junction of both rims"
    );
    assert!(
        rec.bands.is_empty() && rec.slits.is_empty(),
        "{what}: a mixed rim's bands are per link, with no slit"
    );
    let (a, d) = section(r);
    let truth = 2.0 * a * (perimeter() - 2.0 * PI * d);
    let (v0, w0) = volume(&body);
    let (v1, w1) = volume(&out.body);
    let removed = v0 - v1;
    assert!(
        (removed - truth).abs() <= w0 + w1 + 1e-12,
        "{what}: ΔV {removed} is Pappus's {truth} (enclosure ±{})",
        w0 + w1
    );
    assert!(
        w0 + w1 < 1e-7,
        "{what}: the enclosure is a claim: ±{}",
        w0 + w1
    );
}

/// **The rounded plate filleted all round builds** at the pin's radius
/// and near the validity bound `r < R/2`, where an arc's torus band
/// would have a spine radius `R − r` no larger than its tube, at `f64`.
#[test]
fn a_rounded_plate_filleted_all_round_removes_pappuss_volume() {
    for r in [0.05, 0.14] {
        plate_row::<f64>(&format!("r = {r}"), r, r);
    }
}

/// The same plate at the certified scalar.
#[test]
fn a_rounded_plate_filleted_all_round_removes_pappuss_volume_at_interval() {
    for r in [0.05, 0.14] {
        plate_row::<Interval>(&format!("Interval, r = {r}"), r, iv(r));
    }
}

/// **The chamfer does not reach a rounded outline**: its one arm is
/// plane–plane, so each arc link refuses before any chain is carved.
#[test]
fn a_rounded_plate_chamfered_all_round_refuses_at_its_arcs() {
    let body = plate::<f64>(0.0, 0.0, 0.0, H);
    let got =
        chamfer_edges(&at_rest(&body, tol()), &every_edge(&body), 0.05, tol()).map_err(|r| r.error);
    assert!(
        matches!(got, Err(BlendError::ChamferArmUnsupported { .. })),
        "a chamfer has no band over a plane and a cylinder, got {got:?}"
    );
}

/// **A pocket's floor**: a `3 × 2 × 1` block less the rounded prism
/// from `z = 0.5` up through its top, centred. The floor's rim is the outline, its whole
/// outer cycle; a concave round there adds `A·(P − 2πd)`.
fn pocket() -> Body<f64> {
    use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
    let block = sweep::test_support::block::<f64>(3.0, 2.0, 1.0, tol());
    let cut =
        sweep::test_support::finished("the rounded prism", plate::<f64>(1.5, 1.0, 0.5, 1.0), tol());
    boolean_op_with(
        BooleanOp::Subtract,
        &sweep::test_support::finished("the block", block, tol()),
        &cut,
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the pocket builds: {e:?}"))
    .body()
    .expect("the pocket is a body")
    .body
    .clone()
    .into_body()
}

#[test]
fn a_rounded_pockets_floor_fillets_whole_adding_pappuss_volume() {
    let body = pocket();
    let floor = rim_at(&body, 0.5);
    assert_eq!(floor.len(), 8, "the floor's rim: four lines and four arcs");
    for r in [0.05, 0.14] {
        let what = format!("pocket floor, r = {r}");
        let out = fillet_edges(&at_rest(&body, tol()), &floor, r, tol())
            .unwrap_or_else(|e| panic!("{what}: the floor fillets whole, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
        assert_naming_totality(&body, &out, &floor, &what);
        let (a, d) = section(r);
        let truth = a * (perimeter() - 2.0 * PI * d);
        let (v0, w0) = volume(&body);
        let (v1, w1) = volume(&out.body);
        assert!(
            w0 + w1 < 1e-7,
            "{what}: the enclosure is a claim: ±{}",
            w0 + w1
        );
        let added = v1 - v0;
        assert!(
            (added - truth).abs() <= w0 + w1 + 1e-12,
            "{what}: ΔV {added} is Pappus's {truth}"
        );
    }
}

/// **A pocket's mouth**: the outline is a RING of the block's top, the
/// shared host, and a convex round there removes the section on the
/// outline's far side, `A·(P + 2πd)`.
#[test]
fn a_rounded_pockets_mouth_fillets_whole_removing_pappuss_volume() {
    let body = pocket();
    let mouth: Vec<EdgeKey> = rim_at(&body, 1.0)
        .into_iter()
        .filter(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = body
                .get_point(
                    body.get_vertex(body.get_half_edge(he).unwrap().start)
                        .unwrap()
                        .point,
                )
                .unwrap();
            (p.x - 1.5).abs() <= HX + 1e-9 && (p.y - 1.0).abs() <= HY + 1e-9
        })
        .collect();
    assert_eq!(mouth.len(), 8, "the mouth: four lines and four arcs");
    for r in [0.05, 0.14] {
        let what = format!("pocket mouth, r = {r}");
        let out = fillet_edges(&at_rest(&body, tol()), &mouth, r, tol())
            .unwrap_or_else(|e| panic!("{what}: the mouth fillets whole, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
        assert_naming_totality(&body, &out, &mouth, &what);
        let (a, d) = section(r);
        let truth = a * (perimeter() + 2.0 * PI * d);
        let (v0, w0) = volume(&body);
        let (v1, w1) = volume(&out.body);
        assert!(
            w0 + w1 < 1e-7,
            "{what}: the enclosure is a claim: ±{}",
            w0 + w1
        );
        let removed = v0 - v1;
        assert!(
            (removed - truth).abs() <= w0 + w1 + 1e-12,
            "{what}: ΔV {removed} is Pappus's {truth}"
        );
    }
}

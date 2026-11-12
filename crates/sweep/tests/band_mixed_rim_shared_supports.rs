//! **A mixed rim beside other cycles on its supports.** A rounded
//! outline's rim (`band_closed_mixed_chain`) carves when a mate carries
//! a ring of its own — a bore cross-drilled through the plate's long
//! walls — and when its host carries a ladder rim of the same call — a
//! round hole through the plate. Each builds at the closed form, and at
//! the volume of the sequential calls that build it without the shared
//! support; the ring stays on its plane, and a ring moved into a strip
//! refuses at the clearance meter. An annulus rim around a mixed rim
//! refuses, and its sequential recourse builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point2, Point3, RigidMotion, Tol};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{at_rest, extruded, finished};
use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, EdgeKey, FinishedBody as _, validate_geometric};

use crate::band_closed_mixed_chain_shapes::{
    Outline, assert_touch_escalation, every_edge, holed_plate, prism, rect, rim_where,
    tightest_interval, volume,
};

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.05;

/// What a convex round of radius `r` of a circular rim of radius `rho`
/// removes, the ball outside the circle (a hole's mouth) or inside it
/// (a disc's edge): Pappus over the section `r²(1 − π/4)`, whose
/// centroid sits `r(10 − 3π)/(3(4 − π))` in from each face.
fn circle_rim(rho: f64, ball_outside: bool, r: f64) -> f64 {
    let a = r * r * (1.0 - PI / 4.0);
    let d = r * (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI));
    2.0 * PI * a * if ball_outside { rho + d } else { rho - d }
}

fn outline() -> Outline {
    rect(1.0, 0.6, [0.3; 4])
}

fn subtract<T: Decide + Bounds + topo::AtRestPolicy>(a: Body<T>, b: Body<T>) -> Body<T> {
    boolean_op_with(
        BooleanOp::Subtract,
        &finished("the stock", a, tol()),
        &finished("the cutter", b, tol()),
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the cut builds: {e:?}"))
    .body()
    .expect("the cut is a body")
    .body
    .clone()
    .into_body()
}

/// A bore of radius `0.08` along `y` through `y ∈ [−0.7, 0.7]`, its axis
/// at `x = 0`, `z = bz`.
fn bore<T: Decide + topo::AtRestPolicy>(bz: f64) -> Body<T> {
    let plane = SketchPlane::from_frame(RigidMotion::axes_zx(Point3::new(
        T::zero(),
        T::from_f64(-0.7),
        T::zero(),
    )));
    let p = |z: f64| Point2::new(T::from_f64(z), T::zero());
    extruded(
        plane,
        vec![bulge_loop(vec![
            (p(bz - 0.08), T::one()),
            (p(bz + 0.08), T::one()),
        ])],
        T::from_f64(1.4),
        tol(),
    )
}

/// The rounded plate, `z ∈ [0, 0.5]`, cross-drilled at height `bz`.
fn cross_drilled<T: Decide + Bounds + topo::AtRestPolicy>(bz: f64) -> Body<T> {
    subtract(prism::<T>(&outline(), 0.0, 0.5), bore::<T>(bz))
}

/// The outline's rim at height `z`.
fn outer_rim<T: Decide + Bounds>(body: &Body<T>, z: f64) -> Vec<EdgeKey> {
    let o = outline();
    rim_where(body, z, |p| o.sd((p.x, p.y)).abs() < 1e-9)
}

/// Each face carrying rings, as its plane's normal (componentwise
/// magnitudes) and its ring count; a ring on a face that is not a plane
/// panics.
fn ring_hosts<T: Decide + Bounds>(body: &Body<T>) -> Vec<(Point3<f64>, usize)> {
    let mid = |x: T| 0.5 * (x.lo() + x.hi());
    body.faces()
        .filter(|(_, fd)| !fd.rings.is_empty())
        .map(|(f, fd)| match body.face_surface(f) {
            Some(Surface::Plane { normal, .. }) => (
                Point3::new(
                    mid(normal.x).abs(),
                    mid(normal.y).abs(),
                    mid(normal.z).abs(),
                ),
                fd.rings.len(),
            ),
            other => panic!("a ring lies on a face that is not a plane: {other:?}"),
        })
        .collect()
}

const Y: Point3<f64> = Point3::new(0.0, 1.0, 0.0);
const Z: Point3<f64> = Point3::new(0.0, 0.0, 1.0);

/// Fillets `edges` of `body` at `R` and checks the carve: tier 3, the
/// rings on two planes of normal `±rings_on` one each, and the volume
/// removed at `truth`. Returns the result's volume, or `None` where the
/// tightest Interval row refuses with the section's touch escalation.
fn carve<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    body: &Body<T>,
    edges: &[EdgeKey],
    truth: f64,
    rings_on: Point3<f64>,
) -> Option<f64> {
    let out = match fillet_edges(&at_rest(body, tol()), edges, T::from_f64(R), tol()) {
        Ok(out) => {
            assert!(
                !tightest_interval::<T>(),
                "{what}: built where its touch escalates"
            );
            out
        }
        Err(e) if tightest_interval::<T>() => {
            assert_touch_escalation(what, &e.error);
            return None;
        }
        Err(e) => panic!("{what}: builds in one call, got {:?}", e.error),
    };
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    let hosts = ring_hosts(&out.body);
    assert_eq!(hosts.len(), 2, "{what}: two faces carry rings, {hosts:?}");
    for &(n, count) in &hosts {
        assert!(
            count == 1 && (n - rings_on).norm() < 1e-9,
            "{what}: one ring on each plane of normal ±{rings_on:?}, {hosts:?}"
        );
    }
    let ((v0, w0), (v1, w1)) = (volume(body), volume(&out.body));
    assert!(
        w0 + w1 < 1e-7,
        "{what}: the enclosure is a claim: ±{}",
        w0 + w1
    );
    assert!(
        ((v0 - v1) - truth).abs() <= w0 + w1 + 1e-12,
        "{what}: ΔV {} is the closed form {truth}",
        v0 - v1
    );
    Some(v1)
}

/// **A cross-drilled plate's top rim fillets in one call**: the bore's
/// rings on the two long walls stay on them, `0.25` below the trim, and
/// the rim removes Pappus's volume — the volume of filleting first and
/// drilling after.
#[test]
fn a_cross_drilled_plates_top_rim_fillets_with_the_bore_on_its_mates() {
    fn row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(what: &str) -> Option<f64> {
        let body = cross_drilled::<T>(0.2);
        let rim = outer_rim(&body, 0.5);
        assert_eq!(rim.len(), 8, "{what}: the top rim");
        carve(what, &body, &rim, outline().pappus(R, true), Y)
    }
    let one = row::<f64>("cross-drilled").expect("f64 builds");
    row::<Interval>("cross-drilled at Interval");

    let plate = prism::<f64>(&outline(), 0.0, 0.5);
    let rounded = fillet_edges(&at_rest(&plate, tol()), &outer_rim(&plate, 0.5), R, tol())
        .unwrap_or_else(|e| panic!("the undrilled plate's top rim: {:?}", e.error));
    let (vs, ws) = volume(&subtract(Body::clone(&rounded.body), bore::<f64>(0.2)));
    assert!(
        (one - vs).abs() <= ws + 1e-12,
        "drilled then filleted {one} is filleted then drilled {vs}"
    );
}

/// **A bore rising toward the trim refuses at a clearance meter**:
/// the wall's strip runs `0.45 ≤ z ≤ 0.5`, so a bore of radius `0.08`
/// clears it below `bz = 0.37` and reaches into it above.
#[test]
fn a_bore_reaching_a_mates_strip_refuses_at_the_meter() {
    for (bz, clear) in [(0.36, true), (0.38, false)] {
        let what = format!("bore at z = {bz}");
        let body = cross_drilled::<f64>(bz);
        let rim = outer_rim(&body, 0.5);
        if clear {
            carve(&what, &body, &rim, outline().pappus(R, true), Y);
            continue;
        }
        let Err(e) = fillet_edges(&at_rest(&body, tol()), &rim, R, tol()) else {
            panic!("{what}: reaches the strip and built");
        };
        let (BlendError::RingClearance { margin, .. }
        | BlendError::FaceClearanceUncertified { margin, .. }) = e.error
        else {
            panic!("{what}: refuses at a clearance meter, got {:?}", e.error);
        };
        let read = margin
            .reading
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a definite reading");
        assert!(
            (read + 0.01).abs() < 1e-9,
            "{what}: the meter reads the bore 0.01 into the strip, got {read}"
        );
    }
}

/// **The holed plate cut by a Boolean carves its ladder rims first**:
/// a slab with the hole already through it, intersected with the
/// rounded prism, keys the hole's rims before the outline's, so the
/// mixed carves read caps the ladder carves already cut.
#[test]
fn the_holed_plate_intersected_carves_its_ladders_first_at_the_closed_form() {
    let square: Vec<_> = [(-1.5, -1.5), (1.5, -1.5), (1.5, 1.5), (-1.5, 1.5)]
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect();
    let hole = vec![(Point2::new(-0.2, 0.0), -1.0), (Point2::new(0.2, 0.0), -1.0)];
    let slab = extruded(
        SketchPlane::xy_at(Point3::new(0.0, 0.0, -0.1)),
        vec![bulge_loop(square), bulge_loop(hole)],
        0.7,
        tol(),
    );
    let body = boolean_op_with(
        BooleanOp::Intersect,
        &finished("the holed slab", slab, tol()),
        &finished("the rounded prism", prism::<f64>(&outline(), 0.0, 0.5), tol()),
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the intersection builds: {e:?}"))
    .body()
    .expect("the intersection is a body")
    .body
    .clone()
    .into_body();
    let truth = 2.0 * outline().pappus(R, true) + 2.0 * circle_rim(0.2, true, R);
    carve("intersected holed plate", &body, &every_edge(&body), truth, Z);
}

/// **A rounded plate with a round hole fillets every edge in one call**:
/// each cap hosts the outer mixed rim and the hole's ladder rim, and the
/// call removes both rims' closed forms on both caps — the volume of the
/// outer rims first and the hole's in a second call.
#[test]
fn a_rounded_plate_with_a_round_hole_fillets_every_edge_in_one_call() {
    fn row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
        what: &str,
        truth: f64,
    ) -> Option<f64> {
        let (_, body) = holed_plate::<T>((0.0, 0.0), 0.2);
        carve(what, &body, &every_edge(&body), truth, Z)
    }
    let truth = 2.0 * outline().pappus(R, true) + 2.0 * circle_rim(0.2, true, R);
    let one = row::<f64>("holed plate", truth).expect("f64 builds");
    row::<Interval>("holed plate at Interval", truth);

    let (_, body) = holed_plate::<f64>((0.0, 0.0), 0.2);
    let outer: Vec<EdgeKey> = outer_rim(&body, 0.0)
        .into_iter()
        .chain(outer_rim(&body, 0.5))
        .collect();
    let first = fillet_edges(&at_rest(&body, tol()), &outer, R, tol())
        .unwrap_or_else(|e| panic!("the outer rims: {:?}", e.error));
    let b1 = Body::clone(&first.body);
    let on_hole = |p: Point3<f64>| (p.x.hypot(p.y) - 0.2).abs() < 1e-9;
    let hole: Vec<EdgeKey> = rim_where(&b1, 0.0, on_hole)
        .into_iter()
        .chain(rim_where(&b1, 0.5, on_hole))
        .collect();
    assert_eq!(hole.len(), 4, "the hole's two rims of two arcs");
    let second = fillet_edges(&at_rest(&b1, tol()), &hole, R, tol())
        .unwrap_or_else(|e| panic!("the hole's rims: {:?}", e.error));
    let (vs, ws) = volume(&second.body);
    assert!(
        (one - vs).abs() <= ws + 1e-12,
        "one call {one} is the two calls' {vs}"
    );
}

/// **A pocket's mouth beside a round hole**: both rims are rings of the
/// block's top, a mixed one and a ladder, and one call removes both
/// closed forms.
#[test]
fn a_pockets_mouth_and_a_hole_beside_it_fillet_in_one_call() {
    let o = outline();
    let block = sweep::test_support::block::<f64>(3.0, 2.0, 1.0, tol());
    let pin = extruded(
        SketchPlane::xy_at(Point3::new(0.0, 0.0, -0.1)),
        vec![bulge_loop(vec![
            (Point2::new(0.12, 1.0), 1.0),
            (Point2::new(0.32, 1.0), 1.0),
        ])],
        1.2,
        tol(),
    );
    let pocket = extruded(
        SketchPlane::xy_at(Point3::new(1.5, 1.0, 0.5)),
        vec![bulge_loop(
            o.chain
                .iter()
                .map(|&(q, b)| (Point2::new(q.0, q.1), b))
                .collect(),
        )],
        1.0,
        tol(),
    );
    let body = subtract(subtract(block, pin), pocket);
    let mouth = rim_where(&body, 1.0, |p| {
        (p.x - 1.5).abs() <= 1.0 + 1e-9 && (p.y - 1.0).abs() <= 0.6 + 1e-9
    });
    let hole = rim_where(&body, 1.0, |p| {
        ((p.x - 0.22).hypot(p.y - 1.0) - 0.1).abs() < 1e-9
    });
    assert_eq!((mouth.len(), hole.len()), (8, 2), "the mouth and the hole");
    let both: Vec<EdgeKey> = mouth.iter().chain(&hole).copied().collect();
    let out = fillet_edges(&at_rest(&body, tol()), &both, R, tol())
        .unwrap_or_else(|e| panic!("the mouth and the hole fillet together: {:?}", e.error));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    let mut hosts = ring_hosts(&out.body);
    hosts.sort_by_key(|&(_, count)| count);
    assert!(
        hosts.len() == 2
            && hosts.iter().all(|&(n, _)| (n - Z).norm() < 1e-9)
            && hosts.iter().map(|&(_, c)| c).eq([1, 2]),
        "the top keeps the mouth's and the hole's rings, the bottom the hole's: {hosts:?}"
    );
    let truth = o.pappus(R, false) + circle_rim(0.1, true, R);
    let ((v0, w0), (v1, w1)) = (volume(&body), volume(&out.body));
    assert!(
        ((v0 - v1) - truth).abs() <= w0 + w1 + 1e-12,
        "ΔV {} is the closed form {truth}",
        v0 - v1
    );

    let first = fillet_edges(&at_rest(&body, tol()), &mouth, R, tol())
        .unwrap_or_else(|e| panic!("the mouth alone: {:?}", e.error));
    let b1 = Body::clone(&first.body);
    let hole = rim_where(&b1, 1.0, |p| {
        ((p.x - 0.22).hypot(p.y - 1.0) - 0.1).abs() < 1e-9
    });
    let second = fillet_edges(&at_rest(&b1, tol()), &hole, R, tol())
        .unwrap_or_else(|e| panic!("the hole after the mouth: {:?}", e.error));
    let (vs, ws) = volume(&second.body);
    assert!(
        (v1 - vs).abs() <= w1 + ws + 1e-12,
        "one call {v1} is the two calls' {vs}"
    );
}

/// **A hole moved toward a straight link refuses at a clearance
/// meter**: the right link's strip on the cap runs `0.95 ≤ x ≤ 1`, and
/// the hole's own trim circle has radius `0.25`, so the hole clears it
/// below `x = 0.7`. There the hole's trim passes within `0.05` of the
/// corner arcs' centres, inside their whole trim circles and clear of
/// the quarter each link cuts.
#[test]
fn a_hole_reaching_a_links_strip_refuses_at_the_meter() {
    for (x, clear) in [(0.69, true), (0.71, false)] {
        let what = format!("hole at x = {x}");
        let (o, body) = holed_plate::<f64>((x, 0.0), 0.2);
        let truth = 2.0 * o.pappus(R, true) + 2.0 * circle_rim(0.2, true, R);
        if clear {
            carve(&what, &body, &every_edge(&body), truth, Z);
            continue;
        }
        let Err(e) = fillet_edges(&at_rest(&body, tol()), &every_edge(&body), R, tol()) else {
            panic!("{what}: reaches the strip and built");
        };
        let (BlendError::RingClearance { margin, .. }
        | BlendError::FaceClearanceUncertified { margin, .. }) = e.error
        else {
            panic!("{what}: refuses at a clearance meter, got {:?}", e.error);
        };
        let read = margin
            .reading
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a definite reading");
        assert!(
            (read + 0.01).abs() < 1e-9,
            "{what}: the meter reads the hole's trim 0.01 into the strip, got {read}"
        );
    }
}

/// **Holes near a corner arc that the meter cannot clear refuse with
/// the sequential recourse, which builds**: each hole's trim circle
/// passes the corner arc's wedge or a straight link's slab end, where
/// the ranged read straddles the strip though the circle clears it. The
/// outline's rims first, then the hole's, build at the closed form.
#[test]
fn holes_beside_a_corner_refuse_with_the_sequential_recourse_which_builds() {
    for (at, hr) in [
        ((0.559, 0.159), 0.1),
        ((0.6, 0.2), 0.1),
        ((0.845, 0.25), 0.05),
        ((0.849, 0.25), 0.05),
    ] {
        let what = format!("hole r {hr} at {at:?}");
        let (o, body) = holed_plate::<f64>(at, hr);
        let Err(e) = fillet_edges(&at_rest(&body, tol()), &every_edge(&body), R, tol()) else {
            panic!("{what}: built in one call; this row pins the meter's unclear read");
        };
        assert!(
            matches!(&e.error, BlendError::UnsupportedChain { detail, .. }
                if detail.contains("ladder ring beside a mixed rim")
                    && detail.contains("SEQUENTIAL calls")),
            "{what}: refuses with the sequential recourse, got {:?}",
            e.error
        );
        let outer: Vec<EdgeKey> = outer_rim(&body, 0.0)
            .into_iter()
            .chain(outer_rim(&body, 0.5))
            .collect();
        let first = fillet_edges(&at_rest(&body, tol()), &outer, R, tol())
            .unwrap_or_else(|e| panic!("{what}: the outline's rims: {:?}", e.error));
        let b1 = Body::clone(&first.body);
        let on_hole = |p: Point3<f64>| ((p.x - at.0).hypot(p.y - at.1) - hr).abs() < 1e-9;
        let hole: Vec<EdgeKey> = rim_where(&b1, 0.0, on_hole)
            .into_iter()
            .chain(rim_where(&b1, 0.5, on_hole))
            .collect();
        assert_eq!(hole.len(), 4, "{what}: the hole's two rims of two arcs");
        let second = fillet_edges(&at_rest(&b1, tol()), &hole, R, tol())
            .unwrap_or_else(|e| panic!("{what}: the hole's rims after: {:?}", e.error));
        validate_geometric(&second.body, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
        let truth = 2.0 * o.pappus(R, true) + 2.0 * circle_rim(hr, true, R);
        let ((v0, w0), (v1, w1)) = (volume(&body), volume(&second.body));
        assert!(
            ((v0 - v1) - truth).abs() <= w0 + w1 + 1e-12,
            "{what}: ΔV {} is the closed form {truth}",
            v0 - v1
        );
    }
}

/// **An annulus rim around a mixed rim refuses in one call, and its
/// recourse builds**: a disc of radius `1.5` authored as eight arcs,
/// with the rounded outline through it. The disc's rims are hostless
/// annuli on the caps whose rings are the outline's mixed rims
/// (`work/band/an-annulus-rim-around-a-mixed-rim-refuses-in-one-call.md`).
/// The outline's rims carve first, then the disc's.
#[test]
fn an_annulus_rim_around_a_mixed_rim_refuses_and_composes_sequentially() {
    let o = outline();
    let n = o.chain.len();
    // The outline run backwards: each vertex in turn back, each arc's
    // bulge negated with its run.
    let hole: Vec<_> = (0..n)
        .map(|i| {
            let (q, _) = o.chain[(n - i) % n];
            let (_, b) = o.chain[(2 * n - i - 1) % n];
            (Point2::new(q.0, q.1), -b)
        })
        .collect();
    let disc: Vec<_> = (0..8)
        .map(|i| {
            let t = f64::from(i) * PI / 4.0;
            (Point2::new(1.5 * t.cos(), 1.5 * t.sin()), (PI / 16.0).tan())
        })
        .collect();
    let body = extruded(
        SketchPlane::xy(),
        vec![bulge_loop(disc), bulge_loop(hole)],
        0.5,
        tol(),
    );
    let Err(e) = fillet_edges(&at_rest(&body, tol()), &every_edge(&body), R, tol()) else {
        panic!("an annulus around a mixed rim built in one call");
    };
    assert!(
        matches!(&e.error, BlendError::UnsupportedChain { detail, .. }
            if detail.contains("an annulus and a ladder or mixed rim share a support")
                && detail.contains("SEQUENTIAL calls")),
        "refuses at the shared-support gate, got {:?}",
        e.error
    );

    let inner: Vec<EdgeKey> = outer_rim(&body, 0.0)
        .into_iter()
        .chain(outer_rim(&body, 0.5))
        .collect();
    let first = fillet_edges(&at_rest(&body, tol()), &inner, R, tol())
        .unwrap_or_else(|e| panic!("the outline's rims: {:?}", e.error));
    let b1 = Body::clone(&first.body);
    let on_disc = |p: Point3<f64>| (p.x.hypot(p.y) - 1.5).abs() < 1e-9;
    let disc_rims: Vec<EdgeKey> = rim_where(&b1, 0.0, on_disc)
        .into_iter()
        .chain(rim_where(&b1, 0.5, on_disc))
        .collect();
    assert_eq!(disc_rims.len(), 16, "the disc's two rims of eight arcs");
    let second = fillet_edges(&at_rest(&b1, tol()), &disc_rims, R, tol())
        .unwrap_or_else(|e| panic!("the disc's rims: {:?}", e.error));
    validate_geometric(&second.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    let truth = 2.0 * circle_rim(1.5, false, R) + 2.0 * o.pappus(R, false);
    let ((v0, w0), (v1, w1)) = (volume(&body), volume(&second.body));
    assert!(
        ((v0 - v1) - truth).abs() <= w0 + w1 + 1e-12,
        "ΔV {} is the closed form {truth}",
        v0 - v1
    );
}

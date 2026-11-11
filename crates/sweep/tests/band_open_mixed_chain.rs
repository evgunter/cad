//! **An open tangent chain of mixed links fillets whole.**
//!
//! A run of an outline's lines and arcs meeting tangentially, ending at
//! a cut-off or a turn, is one open chain: a cylinder band over each
//! line and a torus band over each arc between them, consecutive bands
//! meeting G1 on the section they share at each junction, one band face
//! per link.
//!
//! The oracle is Pappus by pieces. A round of radius `r` at a square
//! edge removes, or adds, the section `A = r²(1 − π/4)`, whose centroid
//! sits `d = r(10 − 3π)/(3(4 − π))` in from both faces. A prism cut
//! square or by a mitre plane holds `A` times its centroid line's
//! length, and an arc's piece `A·θ·ρ` for a centroid at radius `ρ`; the
//! pieces abut at the junctions' section planes and at the mitre.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::band_closed_mixed_chain_shapes::{assert_touch_escalation, tightest_interval};
use crate::common::interval::iv;
use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point2, Point3, Tol};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, chamfer_edges, fillet_edges};
use sweep::test_support::{assert_naming_totality, at_rest, extruded};
use topo::{Body, EdgeKey, validate_geometric};

pub(crate) fn tol() -> Tol {
    Tol::witness()
}

/// The section a round of radius `r` removes or adds, and its
/// centroid's inset from each face.
pub(crate) fn section(r: f64) -> (f64, f64) {
    (
        r * r * (1.0 - PI / 4.0),
        r * (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI)),
    )
}

/// An outline of `(x, y, bulge)` vertices extruded from `z0` up `h`, at
/// any scalar.
pub(crate) fn outline<T: Decide + topo::AtRestPolicy>(
    o: &[(f64, f64, f64)],
    z0: f64,
    h: f64,
) -> Body<T> {
    let plane = SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), T::from_f64(z0)));
    extruded(
        plane,
        vec![bulge_loop(
            o.iter()
                .map(|&(x, y, q)| (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(q)))
                .collect(),
        )],
        T::from_f64(h),
        tol(),
    )
}

/// The rounded corner's radius.
pub(crate) const R: f64 = 0.3;

/// **The L-plate**: an L of arms 2 and 1 wide, its outer corner at
/// `(2, 0)` rounded at `R`, `z ∈ [0, 0.5]`. The run is the top's line
/// along `y = 0`, the arc, and the line along `x = 2`, cut off square at
/// the walls `x = 0` and `y = 1`.
pub(crate) fn l_plate<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let q = (FRAC_PI_2 / 4.0).tan();
    outline(
        &[
            (0.0, 0.0, 0.0),
            (2.0 - R, 0.0, q),
            (2.0, R, 0.0),
            (2.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
            (1.0, 2.0, 0.0),
            (0.0, 2.0, 0.0),
        ],
        0.0,
        0.5,
    )
}

/// The teardrop: two lines `√3` long from a 60° point at the origin,
/// tangent to a 240° arc of radius 1 about `(2, 0)`, `z ∈ [0, 1]`.
fn teardrop<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let s3 = 3f64.sqrt();
    outline(
        &[(0.0, 0.0, 0.0), (1.5, -s3 / 2.0, s3), (1.5, s3 / 2.0, 0.0)],
        0.0,
        1.0,
    )
}

pub(crate) fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// The edges of `body` whose two ends lie at height `z` and whose
/// midpoint `keep` admits.
pub(crate) fn edges_at<T: Decide + Bounds>(
    body: &Body<T>,
    z: f64,
    keep: impl Fn(f64, f64) -> bool,
) -> Vec<EdgeKey> {
    let mut all = topo::query::all_edges(body);
    all.sort_unstable();
    all.into_iter()
        .filter(|&e| {
            let (s, t) = body.edge_vertices(e).unwrap();
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (a, b) = (p(s), p(t));
            (mid(a.z) - z).abs() < 1e-9
                && (mid(b.z) - z).abs() < 1e-9
                && keep(0.5 * (mid(a.x) + mid(b.x)), 0.5 * (mid(a.y) + mid(b.y)))
        })
        .collect()
}

/// A body's volume and the half-width of its certified enclosure.
pub(crate) fn volume<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    body: &Body<T>,
) -> (f64, f64) {
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("certified props");
    let (lo, hi) = (p.enclosure.volume.lo(), p.enclosure.volume.hi());
    (0.5 * (lo + hi), 0.5 * (hi - lo) + p.enclosure.volume_pad)
}

/// **A slanted section's shared row at `Interval` at the tightest ε**:
/// where a junction's straight link runs off the axes, the section's
/// admission as the shared row of its two bands does not certify at
/// the certified scalar at ε = 1e-12, before its touch is read, on a
/// closed rim as on an open one
/// (`work/band/every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12.md`).
pub(crate) fn assert_shared_row_unadmitted(what: &str, e: &BlendError) {
    assert!(
        matches!(e, BlendError::UnsupportedGeometry { detail, .. }
            if *detail == "a junction's section does not certify as the shared row of its two bands"),
        "{what}: at Interval at ε 1e-12 a slanted section's shared row does not admit, got {e:?}"
    );
}

/// Fillet `edges` of `body` at `r` and check the carve: tier 3, naming
/// total, one band face per link and one section per junction, and the
/// volume moved (`sign` −1 removed, +1 added) at `truth`. At `Interval`
/// at ε = 1e-12 the row refuses as `tightest` says instead
/// ([`tightest_interval`]).
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    body: &Body<T>,
    edges: &[EdgeKey],
    r: f64,
    size: T,
    (links, junctions): (usize, usize),
    (sign, truth): (f64, f64),
    tightest: fn(&str, &BlendError),
) {
    let out = match fillet_edges(&at_rest(body, tol()), edges, size, tol()) {
        Ok(out) => {
            assert!(
                !tightest_interval::<T>(),
                "{what}: built where its touch escalates"
            );
            out
        }
        Err(e) if tightest_interval::<T>() => {
            tightest(what, &e.error);
            return;
        }
        Err(e) => panic!("{what}: the open mixed chain fillets whole at r = {r}, got {e:?}"),
    };
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    let rec = out.naming.as_ref().expect("the carve's birth records");
    assert_eq!(rec.blends.len(), links, "{what}: one band face per link");
    assert_eq!(
        out.blend_faces.len(),
        links,
        "{what}: one blend face per link"
    );
    assert_eq!(
        rec.sections.len(),
        junctions,
        "{what}: one section per junction"
    );
    let (v0, w0) = volume(body);
    let (v1, w1) = volume(&out.body);
    assert!(
        w0 + w1 < 1e-7,
        "{what}: the enclosure is a claim: ±{}",
        w0 + w1
    );
    let moved = sign * (v1 - v0);
    assert!(
        (moved - truth).abs() <= w0 + w1 + 1e-12,
        "{what}: ΔV {moved} is Pappus's {truth} (enclosure ±{})",
        w0 + w1
    );
}

/// The L-plate's run: the two lines' prisms, `2 − R` and `1 − R` long,
/// and the quarter arc's piece at centroid radius `R − d`.
fn l_plate_row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    r: f64,
    size: T,
) {
    let body = l_plate::<T>();
    let run = edges_at(&body, 0.5, |x, y| y.abs() < 1e-9 || (x > 1.5 && y < 0.9));
    assert_eq!(run.len(), 3, "{what}: the line, the arc and the line");
    let (a, d) = section(r);
    let truth = a * ((2.0 - R) + (1.0 - R) + FRAC_PI_2 * (R - d));
    let tightest = assert_touch_escalation;
    run_row(what, &body, &run, r, size, (3, 2), (-1.0, truth), tightest);
}

/// **The L-plate's rounded outer corner run, cut off square at both
/// ends**, removes Pappus's volume by pieces, at `f64` and near the
/// validity bound `r < R/2`.
#[test]
fn an_l_plates_rounded_corner_run_fillets_whole_removing_pappuss_volume() {
    for r in [0.1, 0.14] {
        l_plate_row::<f64>(&format!("L-plate, r = {r}"), r, r);
    }
}

/// The same run at the certified scalar.
#[test]
fn an_l_plates_rounded_corner_run_fillets_whole_at_interval() {
    for r in [0.1, 0.14] {
        l_plate_row::<Interval>(&format!("L-plate, Interval, r = {r}"), r, iv(r));
    }
}

/// The teardrop's rim: each line's centroid runs `√3 − d·cot 30°` from
/// its junction to the mitre plane, and the arc's `240°` at `1 − d`.
fn teardrop_row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    r: f64,
    size: T,
) {
    let body = teardrop::<T>();
    let rim = edges_at(&body, 1.0, |_, _| true);
    assert_eq!(rim.len(), 3, "{what}: two lines and the arc");
    let (a, d) = section(r);
    let s3 = 3f64.sqrt();
    let truth = a * (1.0 - d) * (2.0 * s3 + 4.0 * PI / 3.0);
    let tightest = assert_shared_row_unadmitted;
    run_row(what, &body, &rim, r, size, (3, 2), (-1.0, truth), tightest);
}

/// **The teardrop's rim, which turns once, breaks at its turn into one
/// open mixed chain** whose two lines meet in the mitre, and it removes
/// Pappus's volume by pieces.
#[test]
fn a_teardrops_rim_fillets_whole_through_its_turn_removing_pappuss_volume() {
    teardrop_row::<f64>("teardrop", 0.1, 0.1);
}

/// The same rim at the certified scalar.
#[test]
fn a_teardrops_rim_fillets_whole_at_interval() {
    teardrop_row::<Interval>("teardrop, Interval", 0.1, iv(0.1));
}

/// **A notch's floor**: a `3 × 2 × 1` block less the prism over
/// `x ≥ 1, y ≤ 1` from `z = 0.5` up, its inner corner at `(1, 1)`
/// rounded at `R`. The floor's rim is the line along `x = 1`, the arc
/// and the line along `y = 1`, cut off square at the walls `y = 0` and
/// `x = 3`; a concave round there adds the section along it.
fn notch() -> Body<f64> {
    use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
    let q = (FRAC_PI_2 / 4.0).tan();
    let block = sweep::test_support::block::<f64>(3.0, 2.0, 1.0, tol());
    let cut = outline::<f64>(
        &[
            (1.0, -1.0, 0.0),
            (4.0, -1.0, 0.0),
            (4.0, 1.0, 0.0),
            (1.0 + R, 1.0, q),
            (1.0, 1.0 - R, 0.0),
        ],
        0.5,
        1.0,
    );
    boolean_op_with(
        BooleanOp::Subtract,
        &sweep::test_support::finished("the block", block, tol()),
        &sweep::test_support::finished("the notch", cut, tol()),
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the notch builds: {e:?}"))
    .body()
    .expect("the notch is a body")
    .body
    .clone()
    .into_body()
}

/// **A notch's floor, a concave open mixed chain**, adds Pappus's
/// volume by pieces: the lines' prisms `1 − R` and `2 − R` long and the
/// quarter arc's piece at `R − d` from the corner's axis, the fill lying
/// on the axis side of the concave wall.
#[test]
fn a_notchs_rounded_floor_run_fillets_whole_adding_pappuss_volume() {
    let body = notch();
    let floor = edges_at(&body, 0.5, |x, y| y > 1e-9 && x < 3.0 - 1e-9);
    assert_eq!(
        floor.len(),
        3,
        "the floor's rim: the line, the arc and the line"
    );
    for r in [0.1, 0.14] {
        let (a, d) = section(r);
        let truth = a * ((1.0 - R) + (2.0 - R) + FRAC_PI_2 * (R - d));
        run_row(
            &format!("notch floor, r = {r}"),
            &body,
            &floor,
            r,
            r,
            (3, 2),
            (1.0, truth),
            assert_touch_escalation,
        );
    }
}

/// **The chamfer does not reach an open mixed chain**: its one arm is
/// plane–plane, so the arc refuses before any chain is carved.
#[test]
fn an_open_mixed_chain_chamfered_refuses_at_its_arc() {
    let body = l_plate::<f64>();
    let run = edges_at(&body, 0.5, |x, y| y.abs() < 1e-9 || (x > 1.5 && y < 0.9));
    let got = chamfer_edges(&at_rest(&body, tol()), &run, 0.05, tol()).map_err(|r| r.error);
    assert!(
        matches!(got, Err(BlendError::ChamferArmUnsupported { .. })),
        "a chamfer has no band over a plane and a cylinder, got {got:?}"
    );
}

/// **A torus link at an open chain's end refuses at the open door**:
/// a half-disc's arc, closed by two lines meeting below its centre,
/// requested alone ends at two sharp corners, cut off in walls that do
/// not hold its axis; even a wall that does, whose section of the torus
/// is a circle, has no built cut-off
/// (`work/band/an-end-torus-link-cut-off-in-its-meridian-plane-is-unbuilt.md`).
#[test]
fn a_torus_link_at_an_open_chains_end_refuses_at_the_open_door() {
    let body = outline::<f64>(
        &[(1.0, 0.0, 1.0), (-1.0, 0.0, 0.0), (0.0, -1.0, 0.0)],
        0.0,
        0.5,
    );
    let arc = edges_at(&body, 0.5, |x, y| x.abs() < 0.1 && y > -0.1);
    assert_eq!(arc.len(), 1, "the D's arc");
    let got = fillet_edges(&at_rest(&body, tol()), &arc, 0.1, tol()).map_err(|r| r.error);
    assert!(
        matches!(
            &got,
            Err(BlendError::UnsupportedChain { detail, .. })
                if detail.contains("a torus between two junctions")
        ),
        "a torus at an open chain's end has no built end, got {got:?}"
    );
}

/// **A smooth outline filleted in part stays refused**, at the end
/// where the request stops while its edge runs on smoothly into one it
/// does not name, whatever the arm of the link that stops there
/// (`work/band/a-smooth-outline-filleted-in-part-stops-at-a-tangent-junction.md`):
/// the rounded plate's run of line, arc and line requested alone, and
/// the L-plate's line alone, line and arc, and arc alone. The recourse
/// names the whole smooth run.
#[test]
fn a_smooth_outline_filleted_in_part_refuses_naming_the_whole_run() {
    let q = (FRAC_PI_2 / 4.0).tan();
    let (hx, hy) = (1.0, 0.6);
    let (ax, ay) = (hx - R, hy - R);
    let plate = outline::<f64>(
        &[
            (-ax, -hy, 0.0),
            (ax, -hy, q),
            (hx, -ay, 0.0),
            (hx, ay, q),
            (ax, hy, 0.0),
            (-ax, hy, q),
            (-hx, ay, 0.0),
            (-hx, -ay, q),
        ],
        0.0,
        0.5,
    );
    let l = l_plate::<f64>();
    let rows = [
        (
            "rounded plate's run",
            &plate,
            edges_at(&plate, 0.5, |x, y| {
                y < -hy + 1e-9 || x > hx - 1e-9 || (x > 0.5 && y < -0.2)
            }),
            3,
        ),
        (
            "L-plate's line",
            &l,
            edges_at(&l, 0.5, |_, y| y.abs() < 1e-9),
            1,
        ),
        (
            "L-plate's line and arc",
            &l,
            edges_at(&l, 0.5, |x, y| {
                y.abs() < 1e-9 || (x > 1.5 && x < 1.99 && y < 0.9)
            }),
            2,
        ),
        (
            "L-plate's arc",
            &l,
            edges_at(&l, 0.5, |x, y| x > 1.5 && x < 1.99 && y > 1e-9 && y < 0.9),
            1,
        ),
    ];
    for (what, body, run, n) in rows {
        assert_eq!(run.len(), n, "{what}: the request");
        let got = fillet_edges(&at_rest(body, tol()), &run, 0.1, tol()).map_err(|r| r.error);
        let Err(e @ BlendError::UnsupportedRunOut { detail, .. }) = &got else {
            panic!("{what}: a part of a smooth outline refuses at its tangent end, got {got:?}");
        };
        assert_eq!(
            *detail,
            sweep::blend::battery::PARTIAL_TANGENT_RUN,
            "{what}: the refusal names the tangent run requested in part"
        );
        let shown = e.to_string();
        assert!(
            shown.contains("every edge it runs on into smoothly"),
            "{what}: {shown}"
        );
    }
}

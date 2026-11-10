#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
//! Rows for [`super`]: a spline face's region, on charts that are affine
//! along every iso line (degree 1 both ways), so a face's sides are
//! straight edges whose images are iso lines.

use super::{Caps, Place, Reading, Trim, bezier_cells, read, read_capped};
use crate::body::Body;
use crate::boolean::contain::{CurvedPlacement, FaceContainment, curved_face_placement};
use crate::entity::FaceKey;
use crate::euler::{FaceSurface, MefSite, MevSite};
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A degree-1 net over `x ∈ [−1, 1]` (one span along `u`) through the
/// `(y, z)` rows `rows` along `v`, its interior `v` knots evenly spaced,
/// each a C⁰ crease.
pub(crate) fn ruled(rows: &[(f64, f64)]) -> NurbsSurface<f64> {
    let n = rows.len();
    let mut kv = vec![0.0, 0.0];
    kv.extend((1..n - 1).map(|i| i as f64 / (n - 1) as f64));
    kv.extend([1.0, 1.0]);
    let control = [-1.0, 1.0]
        .into_iter()
        .flat_map(|x| rows.iter().map(move |&(y, z)| Point3::new(x, y, z)))
        .collect();
    NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(kv, 1).unwrap(),
        control,
        vec![1.0; 2 * n],
    )
    .unwrap()
}

/// The twisted bilinear patch `S(u, v) = (u, v, c·u·v)` on `[0, 1]²`:
/// curved, and a straight line along every iso line.
pub(crate) fn twisted(c: f64) -> NurbsSurface<f64> {
    let kv = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, c),
    ];
    NurbsSurface::new(kv(), kv(), control, vec![1.0; 4]).unwrap()
}

/// **A sheet face on `chart` bounded by the chart polygon `uv`**, each
/// side a straight edge whose image is the line in the chart it lies on
/// (minted at rest). Every side must stay where the chart is affine
/// along it. A chart closed in `u` takes coordinates past its domain's
/// end: a side wholly there is stated a period down, in the principal
/// window, and the mint's joint elements lift it back.
pub(crate) fn sheet(chart: &NurbsSurface<f64>, uv: &[(f64, f64)]) -> (Body<f64>, FaceKey) {
    holed_sheet(chart, uv, None)
}

/// [`sheet`] with a hole, the chart polygon `ring`, planted through a
/// strut from the outer polygon's third corner to the ring's first and
/// killed (`kemr`), then closed round with its own sides (`mef`).
fn holed_sheet(
    chart: &NurbsSurface<f64>,
    uv: &[(f64, f64)],
    ring: Option<&[(f64, f64)]>,
) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let (u_lo, u_hi) = chart.knot_values_u().domain();
    let at = |(u, v): (f64, f64)| chart.eval(u_lo + (u - u_lo).rem_euclid(u_hi - u_lo), v);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(uv[0]), true).unwrap();
    let key = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Nurbs(std::sync::Arc::new(chart.clone())),
                sense: true,
            },
        )
        .unwrap();
    // The iso lane's line limb reads the whole row or column, so a side
    // along a creased one, or off both iso directions, states its image
    // as a general degree-1 curve, which certifies over its own span.
    let creased = |k: &geom_core::spline::Knots<f64>| k.runs().count() > 2;
    let (creased_u, creased_v) = (
        creased(chart.knot_values_u()),
        creased(chart.knot_values_v()),
    );
    let side = |a: (f64, f64), b: (f64, f64)| {
        let line = EdgeCurveSpec::line_between(at(a), at(b));
        let (a, b) = if a.0.min(b.0) >= u_hi {
            let p = u_hi - u_lo;
            ((a.0 - p, a.1), (b.0 - p, b.1))
        } else {
            (a, b)
        };
        let len = line.param_end;
        #[allow(clippy::float_cmp)]
        let iso = (a.0 == b.0 && !creased_v) || (a.1 == b.1 && !creased_u);
        let image = if iso {
            Pcurve::IsoLine {
                p0: Point2::new(a.0, a.1),
                pl: Vec2::new((b.0 - a.0) / len, (b.1 - a.1) / len),
            }
        } else {
            Pcurve::General(std::sync::Arc::new(
                geom::NurbsCurve2::new(
                    KnotVector::clamped(vec![0.0, 0.0, len, len], 1).unwrap(),
                    vec![Point2::new(a.0, a.1), Point2::new(b.0, b.1)],
                    vec![1.0; 2],
                )
                .unwrap(),
            ))
        };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(key, image),
            ..line
        }
    };
    // A polygon round the empty loop `lp` at `poly[0]`, closed into a
    // new face by its last side: the vertices in order.
    let polygon = |body: &mut Body<f64>, lp, poly: &[(f64, f64)]| {
        let first = body
            .mev(
                MevSite::Lone { r#loop: lp },
                at(poly[1]),
                side(poly[0], poly[1]),
                tol,
            )
            .unwrap();
        let mut made = vec![first];
        for k in 2..poly.len() {
            let last = made[made.len() - 1];
            made.push(
                body.mev(
                    MevSite::Fan {
                        he1: last.he_minus,
                        he2: last.he_minus,
                    },
                    at(poly[k]),
                    side(poly[k - 1], poly[k]),
                    tol,
                )
                .unwrap(),
            );
        }
        let (last, before) = (made[made.len() - 1], made[made.len() - 2]);
        let owner = body.get_loop(lp).unwrap().face;
        let he = body
            .find_half_edge(owner, last.vertex, before.vertex)
            .unwrap();
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: first.he_plus,
                },
                side(poly[poly.len() - 1], poly[0]),
                FaceSurface::Shared { key, sense: true },
                tol,
            )
            .unwrap()
            .face;
        (face, made.iter().map(|m| m.vertex).collect::<Vec<_>>())
    };
    let (face, corners) = polygon(&mut body, seed.r#loop, uv);
    if let Some(ring) = ring {
        // `corners[k]` is `uv[k + 1]`: the strut leaves `uv[2]`.
        let from = corners[1];
        let he = [corners[2], corners[0]]
            .into_iter()
            .find_map(|to| body.find_half_edge(face, from, to))
            .unwrap();
        let strut = body
            .mev(
                MevSite::Fan { he1: he, he2: he },
                at(ring[0]),
                side(uv[2], ring[0]),
                tol,
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        polygon(&mut body, kill.ring, ring);
    }
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    (body, face)
}

/// The rectangle `[u0, u1] × [v0, v1]`, counterclockwise.
pub(crate) fn rect((u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> Vec<(f64, f64)> {
    vec![(u0, v0), (u1, v0), (u1, v1), (u0, v1)]
}

/// The unit normal of the twisted patch at `(u, v)`.
fn twisted_normal(c: f64, u: f64, v: f64) -> Vec3<f64> {
    Vec3::new(-c * v, -c * u, 1.0).normalize()
}

fn place(body: &Body<f64>, face: FaceKey, q: Point3<f64>) -> CurvedPlacement {
    curved_face_placement(body, face, q, band()).unwrap()
}

/// **A point of a curved spline face is placed by its trim**: inside
/// the trim in, on the carrier outside it out, off the carrier by more
/// than the band off it, and within the band of the carrier but not
/// decidably on it escalated. Each read `Trim(None)` before the spline
/// arm.
#[test]
fn a_point_of_a_spline_face_is_placed_in_or_out_by_its_trim() {
    let c = 0.6;
    let chart = twisted(c);
    let (body, face) = sheet(&chart, &rect((0.25, 0.75), (0.25, 0.75)));
    let b = band();
    let n = twisted_normal(c, 0.5, 0.5);
    let on = chart.eval(0.5, 0.5);
    assert_eq!(
        place(&body, face, on),
        CurvedPlacement::Trim(Some(FaceContainment::In)),
        "a point of the face"
    );
    assert_eq!(
        place(&body, face, on + n * (b.zero() * 0.5)),
        CurvedPlacement::Trim(Some(FaceContainment::In)),
        "a point within the band's zero of the face"
    );
    assert_eq!(
        place(&body, face, chart.eval(0.9, 0.4)),
        CurvedPlacement::Trim(Some(FaceContainment::Out)),
        "a point of the carrier past the trim"
    );
    assert_eq!(
        place(&body, face, chart.eval(0.1, 0.05)),
        CurvedPlacement::Trim(Some(FaceContainment::Out)),
        "a point of the carrier past the trim, by a corner"
    );
    assert_eq!(
        place(&body, face, on + n * 0.1),
        CurvedPlacement::OffCarrier,
        "a point off the carrier"
    );
    assert_eq!(
        place(&body, face, chart.eval(1.0, 1.0) + Vec3::new(0.2, 0.0, 0.0)),
        CurvedPlacement::OffCarrier,
        "a point off the chart's end"
    );
    let in_band = curved_face_placement(&body, face, on + n * ((b.zero() + b.escalate()) / 2.0), b);
    let Err(crate::boolean::ContainError::Escalated { diag, .. }) = in_band else {
        panic!("a point in the band of the carrier escalates: {in_band:?}");
    };
    assert_eq!(
        diag.predicate,
        Some(super::SPLINE_REGION_CARRIER),
        "{diag:?}"
    );
}

/// **A point on the trim's boundary within the band** is the boundary
/// walk's (an edge's point is `OnEdge`, one in its band escalates); and
/// the chart reading, asked directly at a chart point within the band of
/// the trim along the chart, reads it near and never a side.
#[test]
fn a_point_within_the_band_of_the_trim_is_never_given_a_side() {
    let c = 0.6;
    let chart = twisted(c);
    let (body, face) = sheet(&chart, &rect((0.25, 0.75), (0.25, 0.75)));
    let b = band();
    let edge_point = chart.eval(0.75, 0.5);
    assert!(
        matches!(
            place(&body, face, edge_point),
            CurvedPlacement::Trim(Some(FaceContainment::OnEdge(_)))
        ),
        "a point of an edge is on it"
    );
    let cells = bezier_cells(&chart).unwrap();
    let trim = Trim::of(&body, face, &chart, &cells, Caps::READ, b).unwrap();
    // `|S_u| ≥ 1` on this chart, so a chart step of `δ` along `u` is at
    // least `δ` metres along the chart: halfway into the band.
    let step = (b.zero() + b.escalate()) / 2.0;
    for u in [0.75 - step, 0.75 + step] {
        let near = trim.place(&cells, super::Rect::point(u, 0.5), b);
        assert!(
            matches!(near, Place::Near(Some(ref d)) if d.predicate == Some(super::SPLINE_REGION_TRIM)),
            "a chart point within the band of the trim at u = {u}: {near:?}"
        );
    }
    let deep = trim.place(&cells, super::Rect::point(0.75 - 1e-3, 0.5), b);
    assert!(matches!(deep, Place::In), "past the band inside: {deep:?}");
    let past = trim.place(&cells, super::Rect::point(0.75 + 1e-3, 0.5), b);
    assert!(
        matches!(past, Place::Out),
        "past the band outside: {past:?}"
    );
}

/// **Across a crease**: a roof chart, its two planar halves meeting at a
/// C⁰ knot run, a face spanning the crease. A point on the crease, either
/// side of it, and past the trim beside it are placed with no derivative
/// read anywhere.
#[test]
fn a_point_at_a_crease_is_placed_without_a_derivative_across_it() {
    let chart = ruled(&[(0.0, 0.0), (1.0, 0.0), (2.0, 1.0)]);
    let (body, face) = sheet(
        &chart,
        &[
            (0.25, 0.25),
            (0.75, 0.25),
            (0.75, 0.5),
            (0.75, 0.75),
            (0.25, 0.75),
            (0.25, 0.5),
        ],
    );
    for v in [0.5, 0.5 - 1e-9, 0.5 + 1e-9, 0.4, 0.6] {
        assert_eq!(
            place(&body, face, chart.eval(0.5, v)),
            CurvedPlacement::Trim(Some(FaceContainment::In)),
            "v = {v}"
        );
    }
    for (u, v) in [(0.9, 0.5), (0.1, 0.5 + 1e-9), (0.9, 0.5 - 1e-9)] {
        assert_eq!(
            place(&body, face, chart.eval(u, v)),
            CurvedPlacement::Trim(Some(FaceContainment::Out)),
            "({u}, {v})"
        );
    }
    // Inside the fold of the roof, off both halves by more than the band.
    let q = chart.eval(0.5, 0.5) + Vec3::new(0.0, -0.01, 0.01);
    assert_eq!(place(&body, face, q), CurvedPlacement::OffCarrier);
}

/// The hairpin: ruled along `x`, degree 1 in `v` with its C⁰ knot at
/// `v = ½`, through the `(y, z)` rows `(1, 0)`, `(0, 0)`, `(1, h)` with
/// `h = ε/100`. The half `v < ½` is the floor strip; the half `v > ½`
/// folds back over it, rising `h` over the unit run.
fn hairpin() -> NurbsSurface<f64> {
    let h = Tol::witness().eps() / 100.0;
    ruled(&[(1.0, 0.0), (0.0, 0.0), (1.0, h)])
}

/// **The hairpin's upper-half foot is out of a face on the lower half**
/// (the second witness of `a-point-has-no-place-in-a-nurbs-faces-trim`):
/// the projection of `p = (1/7, 3/7, 3h/7)`, a point of the upper half
/// `3h/7` off the face's edge line `y = 3/7`, lands on the upper half,
/// and the trim reading puts that foot out of the face. A point of the
/// upper half over the face's interior falls to the face half and reads
/// in; one over the lower half past the trim reads out.
#[test]
fn a_hairpin_carriers_upper_foot_is_out_of_a_lower_half_face() {
    let chart = hairpin();
    let h = Tol::witness().eps() / 100.0;
    // `y = 1 − 2v` on the lower half: the face is `y ∈ [3/7, 6/7]`,
    // `x ∈ [−0.8, 0.8]`.
    let (body, face) = sheet(&chart, &rect((0.1, 0.9), (1.0 / 14.0, 2.0 / 7.0)));
    let b = band();
    let p = Point3::new(1.0 / 7.0, 3.0 / 7.0, 3.0 * h / 7.0);
    let foot = chart.project(p).unwrap();
    assert!(
        foot.v > 0.5,
        "the projection's foot is the upper half's: {foot:?}"
    );
    let cells = bezier_cells(&chart).unwrap();
    let trim = Trim::of(&body, face, &chart, &cells, Caps::READ, b).unwrap();
    let placed = trim.place(&cells, super::Rect::point(foot.u, foot.v), b);
    assert!(
        matches!(placed, Place::Out),
        "the upper-half foot: {placed:?}"
    );
    // Over the face's interior: `y = 0.6` is `v = 0.2` below, `v = 0.8`
    // above.
    let over = chart.eval(0.5, 0.8);
    assert_eq!(
        place(&body, face, over),
        CurvedPlacement::Trim(Some(FaceContainment::In)),
        "a point of the upper half over the face"
    );
    // Over the lower half past the trim: `y = 0.2`.
    let past = chart.eval(0.5, 0.6);
    assert_eq!(
        place(&body, face, past),
        CurvedPlacement::Trim(Some(FaceContainment::Out)),
        "a point of the upper half over the lower half past the trim"
    );
    assert!(
        matches!(read(&body, face, &chart, p, b), Reading::Unsettled(_)),
        "the door's own point, by its edge line"
    );
}

// ---- review lane r1 (PR 5805) counterexample rows ----

/// A point of the carrier half a band outside the trim, read past the
/// boundary walk: a point of the face (the edge's) is within the band, so
/// Out is unsound here; the reading must stay unsettled. Goes red if the
/// Out rule stops requiring every kept cell to be decided out.
#[test]
fn r1_a_carrier_point_in_the_band_outside_the_trim_is_not_out() {
    let c = 0.6;
    let chart = twisted(c);
    let (body, face) = sheet(&chart, &rect((0.25, 0.75), (0.25, 0.75)));
    let b = band();
    let step = (b.zero() + b.escalate()) / 2.0;
    for u in [0.75 + step, 0.75 + b.zero() * 0.5] {
        let r = read(&body, face, &chart, chart.eval(u, 0.5), b);
        assert!(matches!(r, Reading::Unsettled(_)), "u = {u}: {r:?}");
    }
}

/// The hairpin door's own point lies within `3h/7` of the face's edge, so
/// Out (no face point within the band) is false there.
#[test]
fn r1_the_hairpin_door_point_is_not_out() {
    let chart = hairpin();
    let h = Tol::witness().eps() / 100.0;
    let (body, face) = sheet(&chart, &rect((0.1, 0.9), (1.0 / 14.0, 2.0 / 7.0)));
    let p = Point3::new(1.0 / 7.0, 3.0 / 7.0, 3.0 * h / 7.0);
    let r = read(&body, face, &chart, p, band());
    assert!(matches!(r, Reading::Unsettled(_)), "{r:?}");
}

/// The ray from a query on the crease row passes exactly through the
/// trim's vertex `(0.75, 0.5)`: the half-open rule counts it once.
#[test]
fn r1_a_ray_through_a_trim_vertex_counts_once() {
    let chart = ruled(&[(0.0, 0.0), (1.0, 0.0), (2.0, 1.0)]);
    let (body, face) = sheet(
        &chart,
        &[
            (0.25, 0.25),
            (0.75, 0.25),
            (0.75, 0.5),
            (0.75, 0.75),
            (0.25, 0.75),
            (0.25, 0.5),
        ],
    );
    let b = band();
    let cells = bezier_cells(&chart).unwrap();
    let trim = Trim::of(&body, face, &chart, &cells, Caps::READ, b).unwrap();
    let at = trim.place(&cells, super::Rect::point(0.5, 0.5), b);
    assert!(matches!(at, Place::In), "{at:?}");
}

/// On a chart closed in `u` (period 1), an outer loop lifted from a
/// member in the principal window runs over the seam, `u ∈ [0.8, 1.4]`,
/// while a hole inside it, whose members all lie in the principal
/// window, is lifted at `u ∈ [0.1, 0.3]` (its true place in the outer's
/// cover is `[1.1, 1.3]`). A query in the hole is out of the face.
#[test]
fn r1_a_hole_lifted_a_period_from_its_outer_loop_is_still_a_hole() {
    let chart = twisted(0.6);
    let cells = bezier_cells(&chart).unwrap();
    let square = |(u0, u1): (f64, f64), (v0, v1): (f64, f64), ccw: bool| {
        let mut p = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
        if !ccw {
            p.reverse();
        }
        (0..4)
            .map(|k| {
                super::Piece::segment(
                    super::Piece::vertex(p[k], [0.0; 2]),
                    super::Piece::vertex(p[(k + 1) % 4], [0.0; 2]),
                )
            })
            .collect::<Vec<_>>()
    };
    let b = band();
    let outer = Trim::from_pieces(
        square((0.8, 1.4), (0.2, 0.8), true),
        super::Rect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        },
        [Some(1.0), None],
        Caps::READ,
    )
    .unwrap();
    let trim = outer
        .with_rings(&cells, vec![square((0.1, 0.3), (0.4, 0.6), false)], b)
        .expect("the hole's place in the outer loop's cover is decided");
    // Sanity: in the face but not in the hole.
    let face_point = trim.place(&cells, super::Rect::point(0.9, 0.5), b);
    assert!(matches!(face_point, Place::In), "{face_point:?}");
    let hole = trim.place(&cells, super::Rect::point(0.2, 0.5), b);
    assert!(matches!(hole, Place::Out), "a point of the hole: {hole:?}");
}

/// **Review r2: a chart point of the carrier within the band of the trim
/// is never read out (nor in)**, asked of [`read`] directly so the
/// boundary walk does not answer first. A face point lies within the
/// band of either, so the only honest reading is unsettled; the Out rule
/// (every kept cell wholly outside) is what keeps the outer one from Out.
#[test]
fn review_r2_a_carrier_point_within_the_band_of_the_trim_is_unsettled() {
    let c = 0.6;
    let chart = twisted(c);
    let (body, face) = sheet(&chart, &rect((0.25, 0.75), (0.25, 0.75)));
    let b = band();
    let step = (b.zero() + b.escalate()) / 2.0;
    for u in [0.75 - step, 0.75 + step] {
        let r = read(&body, face, &chart, chart.eval(u, 0.5), b);
        assert!(matches!(r, Reading::Unsettled(_)), "u = {u}: {r:?}");
    }
}

/// **Review r2: ray parity through a trim vertex and along a trim side.**
/// The crease fixture's sides meet at `(0.25, ½)` and `(0.75, ½)`, so a
/// `+u` ray at `v = ½` passes through vertices; an L-shaped face has a
/// side lying along the ray.
#[test]
fn review_r2_parity_through_a_vertex_and_along_a_side() {
    let chart = ruled(&[(0.0, 0.0), (1.0, 0.0), (2.0, 1.0)]);
    let b = band();
    let cells = bezier_cells(&chart).unwrap();
    let (body, face) = sheet(
        &chart,
        &[
            (0.25, 0.25),
            (0.75, 0.25),
            (0.75, 0.5),
            (0.75, 0.75),
            (0.25, 0.75),
            (0.25, 0.5),
        ],
    );
    let trim = Trim::of(&body, face, &chart, &cells, Caps::READ, b).unwrap();
    for (u, want_in) in [(0.5, true), (0.1, false), (0.9, false)] {
        let p = trim.place(&cells, super::Rect::point(u, 0.5), b);
        assert!(
            if want_in {
                matches!(p, Place::In)
            } else {
                matches!(p, Place::Out)
            },
            "through vertices at u = {u}: {p:?}"
        );
    }
    let (body, face) = sheet(
        &chart,
        &[
            (0.25, 0.25),
            (0.75, 0.25),
            (0.75, 0.5),
            (0.5, 0.5),
            (0.5, 0.75),
            (0.25, 0.75),
            (0.25, 0.5),
        ],
    );
    let trim = Trim::of(&body, face, &chart, &cells, Caps::READ, b).unwrap();
    for (u, want_in) in [(0.3, true), (0.1, false)] {
        let p = trim.place(&cells, super::Rect::point(u, 0.5), b);
        assert!(
            if want_in {
                matches!(p, Place::In)
            } else {
                matches!(p, Place::Out)
            },
            "along a side at u = {u}: {p:?}"
        );
    }
}

/// A square tube closed in `u` (period 1), its seam mid-side on `x = 1`,
/// straight in `z` along `v`.
fn square_tube() -> NurbsSurface<f64> {
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (-1.0, 1.0),
        (-1.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let control = ring
        .iter()
        .flat_map(|&(x, y)| [0.0, 1.0].map(move |z| Point3::new(x, y, z)))
        .collect();
    NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.125, 0.375, 0.625, 0.875, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        vec![1.0; 12],
    )
    .unwrap()
}

/// **A face across a closed chart's seam, with and without a hole, at
/// rest**: on the square tube, the face `u ∈ [0.9, 1.1]` (its sides at
/// `u ≥ 1` stated in the principal window and lifted back by the mint's
/// joint elements) reads in on both sides of the seam and out across the
/// tube. A hole beside the seam, on either side of it, is a hole: each
/// loop's lift starts on one of its own rows, so the ring is lifted into
/// the outer loop's cover before the parity is read. The hole at
/// `u ∈ [0.92, 0.96]` read in before that.
#[test]
fn a_face_across_a_closed_charts_seam_reads_its_region_and_its_hole() {
    let chart = square_tube();
    let outer = [
        (0.9, 0.2),
        (1.0, 0.2),
        (1.1, 0.2),
        (1.1, 0.8),
        (1.0, 0.8),
        (0.9, 0.8),
    ];
    let into = CurvedPlacement::Trim(Some(FaceContainment::In));
    let out = CurvedPlacement::Trim(Some(FaceContainment::Out));
    let (body, face) = sheet(&chart, &outer);
    for (u, v, want) in [
        (0.95, 0.5, into),
        (0.05, 0.5, into),
        (0.5, 0.5, out),
        (0.2, 0.5, out),
    ] {
        assert_eq!(
            place(&body, face, chart.eval(u, v)),
            want,
            "no hole: ({u}, {v})"
        );
    }
    for (name, hole, inside) in [
        (
            "past the seam",
            [(1.02, 0.4), (1.06, 0.4), (1.06, 0.6), (1.02, 0.6)],
            0.04,
        ),
        (
            "before the seam",
            [(0.92, 0.4), (0.96, 0.4), (0.96, 0.6), (0.92, 0.6)],
            0.94,
        ),
    ] {
        let (body, face) = holed_sheet(&chart, &outer, Some(&hole));
        assert_eq!(
            body.get_face(face).unwrap().rings.len(),
            1,
            "{name}: the hole is a ring"
        );
        for (u, v, want) in [
            (inside, 0.5, out),
            (0.95, 0.3, into),
            (0.05, 0.3, into),
            (0.5, 0.5, out),
        ] {
            assert_eq!(
                place(&body, face, chart.eval(u, v)),
                want,
                "{name}: ({u}, {v})"
            );
        }
    }
}

/// **A reading past its work caps stands unsettled**, never a verdict:
/// a point of the carrier past the trim reads out at the caps every
/// caller reads at, and unsettled with one kept cell allowed; the trim
/// reading of its foot reads out, and near with one piece allowed.
#[test]
fn a_reading_past_its_work_caps_stands_unsettled() {
    let chart = twisted(0.6);
    let (body, face) = sheet(&chart, &rect((0.25, 0.75), (0.25, 0.75)));
    let b = band();
    let q = chart.eval(0.9, 0.4);
    assert!(matches!(read(&body, face, &chart, q, b), Reading::Out));
    let one_cell = Caps {
        cells: 1,
        ..Caps::READ
    };
    let capped = read_capped(&body, face, &chart, q, b, one_cell);
    assert!(
        matches!(capped, Reading::Unsettled(None)),
        "one cell: {capped:?}"
    );
    let cells = bezier_cells(&chart).unwrap();
    let at = super::Rect::point(0.9, 0.4);
    let trim = Trim::of(&body, face, &chart, &cells, Caps::READ, b).unwrap();
    assert!(matches!(trim.place(&cells, at, b), Place::Out));
    let one_piece = Caps {
        pieces: 1,
        ..Caps::READ
    };
    let trim = Trim::of(&body, face, &chart, &cells, one_piece, b).unwrap();
    let capped = trim.place(&cells, at, b);
    assert!(matches!(capped, Place::Near(None)), "one piece: {capped:?}");
}

/// **A way out of the window within the band keeps a query near.** On a
/// chart `L = 5·escalate` across in `v` below `v = ½` and collapsed above
/// it (its upper rows coincide), the trim square `v ∈ [0.6, 0.9]` is
/// `0.15·L` along the chart from the query `(0.5, 0.35)`: within the
/// band. The window round the query keeps below the collapse (the
/// collapsed cell has no arm), and its way out along `v` is `L/16`, in
/// the band, so the query reads near. The gaps alone, metered by the
/// window's arms (`0.25·L`), clear every piece, and would read it out.
#[test]
fn a_way_out_of_the_window_within_the_band_keeps_a_query_near() {
    let b = band();
    let reach = 2.5 * b.escalate();
    let chart = ruled(&[(0.0, 0.0), (reach, 0.0), (reach, 0.0)]);
    let cells = bezier_cells(&chart).unwrap();
    let p = [[0.25, 0.6], [0.75, 0.6], [0.75, 0.9], [0.25, 0.9]];
    let square = (0..4)
        .map(|k| {
            super::Piece::segment(
                super::Piece::vertex(p[k], [0.0; 2]),
                super::Piece::vertex(p[(k + 1) % 4], [0.0; 2]),
            )
        })
        .collect();
    let unit = super::Rect {
        u: (0.0, 1.0),
        v: (0.0, 1.0),
    };
    let trim = Trim::from_pieces(square, unit, [None, None], Caps::READ).unwrap();
    let near = trim.place(&cells, super::Rect::point(0.5, 0.35), b);
    assert!(
        matches!(near, Place::Near(Some(ref d)) if d.predicate == Some(super::SPLINE_REGION_TRIM)),
        "the way out of the window is in the band: {near:?}"
    );
}

// ---- the interior arm's chart placement ----

/// **A meeting's chart point is placed by the trim on its own sheet**
/// ([`super::spline_chart_placement`]): on the twisted sheet, a chart
/// point inside the trim is in, one past it out, and one within the band
/// of it along the chart is given no side. On the hairpin, a meeting on
/// the upper half over the face's interior is out of a face on the lower
/// half, where its point in space reads in through the lower sheet.
#[test]
fn a_meetings_chart_point_is_placed_on_its_own_sheet() {
    use super::spline_chart_placement;
    let b = band();
    let chart = twisted(0.6);
    let (body, face) = sheet(&chart, &rect((0.25, 0.75), (0.25, 0.75)));
    let at = |uv| spline_chart_placement(&body, face, uv, b);
    assert!(
        matches!(at((0.5, 0.5)), Ok(Some(FaceContainment::In))),
        "inside the trim"
    );
    assert!(
        matches!(at((0.9, 0.4)), Ok(Some(FaceContainment::Out))),
        "past the trim"
    );
    let step = (b.zero() + b.escalate()) / 2.0;
    let near = at((0.75 + step, 0.5));
    assert!(
        matches!(near, Err(_) | Ok(None)),
        "within the band of the trim along the chart: {near:?}"
    );

    let chart = hairpin();
    let (body, face) = sheet(&chart, &rect((0.1, 0.9), (1.0 / 14.0, 2.0 / 7.0)));
    assert_eq!(
        place(&body, face, chart.eval(0.5, 0.8)),
        CurvedPlacement::Trim(Some(FaceContainment::In)),
        "the upper-half point, in space, reads in through the lower sheet"
    );
    let upper = spline_chart_placement(&body, face, (0.5, 0.8), b);
    assert!(
        matches!(upper, Ok(Some(FaceContainment::Out))),
        "the upper-half meeting is out of the lower-half face: {upper:?}"
    );
    let lower = spline_chart_placement(&body, face, (0.5, 0.2), b);
    assert!(
        matches!(lower, Ok(Some(FaceContainment::In))),
        "the lower-half meeting under it is in: {lower:?}"
    );
}

/// **A spline face's trim reads past a placeholder, and past no other
/// rowless half**: the sheet with a null edge fanned into its outer
/// loop (`region::with_null_edge`, the sweep's crossing-vertex
/// scaffolding) places every point as the sheet without it does, its
/// lift past placeholders skipping the null edge's halves and lifting
/// the rest exactly as the bare sheet's, while the strict lift gaps at
/// a null half. What is no placeholder gaps under both lifts: a null
/// edge whose ends have parted (a straight segment would otherwise
/// close the gap unseen), a certified half with its row detached, and
/// a closed certified edge, whose halves start on one point.
#[test]
fn a_spline_trim_reads_past_a_placeholder_and_no_other_rowless_half() {
    use crate::pcurves::{LiftGap, loop_lift_past_placeholders};
    let c = 0.6;
    let chart = twisted(c);
    let uv = rect((0.25, 0.75), (0.25, 0.75));
    let (plain, plain_face) = sheet(&chart, &uv);
    let (mut nulled, face) = sheet(&chart, &uv);
    super::super::region::with_null_edge(&mut nulled, face);
    let outer = |body: &Body<f64>, face: FaceKey| body.get_face(face).unwrap().outer;
    let nulls: Vec<_> = nulled
        .edges()
        .filter(|&(e, d)| nulled.edge_curve_linked(e, d).null_scaffold().is_some())
        .map(|(e, _)| e)
        .collect();
    let [null] = nulls[..] else {
        panic!("one null edge: {nulls:?}");
    };
    assert!(
        crate::null::is_placeholder(&nulled, null),
        "a fresh null edge is one point"
    );
    let halves_of = |body: &Body<f64>, e| {
        let d = body.get_edge(e).unwrap();
        [d.he_plus, d.he_minus]
    };
    let null_halves = halves_of(&nulled, null);

    let lifted = |body: &Body<f64>, face| -> Vec<_> {
        loop_lift_past_placeholders(body, outer(body, face))
            .unwrap()
            .into_iter()
            .map(|r| (r.half_edge, r.lift, r.pcurve.eval(r.row.params().0)))
            .collect()
    };
    let (skipped, bare) = (lifted(&nulled, face), lifted(&plain, plain_face));
    assert_eq!(skipped.len(), bare.len(), "the null halves are left out");
    for ((_, lift, start), (_, bare_lift, bare_start)) in skipped.iter().zip(&bare) {
        assert_eq!(lift, bare_lift, "each row lifts as on the bare sheet");
        assert_eq!(
            (start.x, start.y),
            (bare_start.x, bare_start.y),
            "each row starts as on the bare sheet"
        );
    }
    let Err(LiftGap { half_edge }) = nulled.loop_lift(outer(&nulled, face)) else {
        panic!("the strict lift refuses the null edge");
    };
    assert!(
        null_halves.contains(&half_edge),
        "it gaps at a null half: {half_edge:?}"
    );
    for q in [
        chart.eval(0.5, 0.5),
        chart.eval(0.9, 0.4),
        chart.eval(0.3, 0.6),
    ] {
        let got = place(&nulled, face, q);
        assert!(
            matches!(got, CurvedPlacement::Trim(Some(_))),
            "the nulled sheet places {q:?}: {got:?}"
        );
        assert_eq!(
            got,
            place(&plain, plain_face, q),
            "as the bare sheet at {q:?}"
        );
    }

    // Each shape that is no placeholder gaps under both lifts, at one of
    // the halves named.
    let gaps_at = |body: &Body<f64>, lk, at: &[crate::HalfEdgeKey], what: &str| {
        for (lift, got) in [
            ("strict", body.loop_lift(lk).map(|_| ())),
            (
                "past placeholders",
                loop_lift_past_placeholders(body, lk).map(|_| ()),
            ),
        ] {
            let Err(LiftGap { half_edge }) = got else {
                panic!("{what}: the {lift} lift gaps, got {got:?}");
            };
            assert!(
                at.contains(&half_edge),
                "{what}: the {lift} lift gaps at {half_edge:?}"
            );
        }
    };
    let unread = |body: &Body<f64>, what: &str| {
        assert_eq!(
            place(body, face, chart.eval(0.5, 0.5)),
            CurvedPlacement::Trim(None),
            "{what} leaves the trim unread"
        );
    };

    // A null edge whose far end has parted from its near one.
    let mut parted = nulled.clone();
    let far = parted.get_half_edge(null_halves[1]).unwrap().start;
    let moved = parted.add_point(chart.eval(0.3, 0.3));
    parted.get_vertex_mut(far).unwrap().point = moved;
    assert!(
        !crate::null::is_placeholder(&parted, null),
        "a parted null edge is no placeholder"
    );
    gaps_at(
        &parted,
        outer(&parted, face),
        &null_halves,
        "a parted null edge",
    );
    unread(&parted, "a parted null edge");

    // A certified half with its row detached.
    let (dropped, _, _) = skipped[1];
    let mut torn = nulled.clone();
    torn.detach_pcurve(dropped).unwrap();
    gaps_at(
        &torn,
        outer(&torn, face),
        &[dropped],
        "a rowless certified half",
    );
    unread(&torn, "a rowless certified half");

    // A closed certified edge: a circle planted in a box's top face, one
    // edge whose halves both start on its one vertex, rowless on the
    // plane.
    let tol = Tol::witness();
    let square = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let prism = crate::test_support_fixtures::prism_z::<f64>(&square, 0.0, 1.0, tol);
    let mut disc = prism.body;
    let crate::LoopBoundary::Cycle { first } = disc
        .get_loop(disc.get_face(prism.top_face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("the top's outer loop is a cycle");
    };
    let planted = crate::test_support_fixtures::plant_disc_face(
        &mut disc,
        first,
        Point3::new(1.0, 1.0, 1.0),
        0.2,
        tol,
    );
    let circle = halves_of(&disc, planted.edge);
    let d = disc.get_edge(planted.edge).unwrap();
    assert!(
        disc.edge_curve_linked(planted.edge, d)
            .certified()
            .is_some()
            && disc.get_half_edge(circle[0]).unwrap().start
                == disc.get_half_edge(circle[1]).unwrap().start,
        "the circle is certified and closes on its one vertex"
    );
    assert!(
        !crate::null::is_placeholder(&disc, planted.edge),
        "a closed certified edge is no placeholder"
    );
    gaps_at(&disc, planted.r#loop, &circle, "a closed certified edge");
}

/// **A placeholder at any joint of a face across a closed chart's seam,
/// outer loop and hole alike, is read past.** Each half of each loop in turn gets a null strut fanned
/// at its start (the seam-crossing joints among them); the trim lifts
/// every other half as the bare face does and places each probe point
/// as the bare face does.
#[test]
fn a_placeholder_at_each_joint_across_a_closed_charts_seam_reads_as_the_bare_face() {
    let chart = square_tube();
    let outer = [
        (0.9, 0.2),
        (1.0, 0.2),
        (1.1, 0.2),
        (1.1, 0.8),
        (1.0, 0.8),
        (0.9, 0.8),
    ];
    let hole = [(1.02, 0.4), (1.06, 0.4), (1.06, 0.6), (1.02, 0.6)];
    let probes = [
        (0.95, 0.5),
        (0.05, 0.5),
        (0.5, 0.5),
        (0.2, 0.5),
        (0.04, 0.5),
        (0.95, 0.3),
        (0.05, 0.3),
        (0.99, 0.21),
        (0.01, 0.79),
    ];
    for ring in [None, Some(&hole[..])] {
        let (bare, face) = holed_sheet(&chart, &outer, ring);
        let loops: Vec<_> = bare.proven_face(face).loops().map(|(lk, ..)| lk).collect();
        let want: Vec<_> = probes
            .iter()
            .map(|&(u, v)| place(&bare, face, chart.eval(u, v)))
            .collect();
        let rows = |b: &Body<f64>, lk| -> Vec<_> {
            crate::pcurves::loop_lift_past_placeholders(b, lk)
                .unwrap()
                .into_iter()
                .map(|r| (r.half_edge, r.lift, r.pcurve.eval(r.row.params().0)))
                .collect()
        };
        for &lk in &loops {
            for he in crate::pcurves::loop_rows(&bare, lk) {
                let mut nulled = bare.clone();
                nulled
                    .mev_null(
                        crate::construction::MevSite::Fan { he1: he, he2: he },
                        crate::construction::NewVertexSide::Above,
                    )
                    .unwrap();
                for &l in &loops {
                    let (got, base) = (rows(&nulled, l), rows(&bare, l));
                    assert_eq!(got.len(), base.len(), "ring {ring:?} at {he:?}");
                    for (g, b) in got.iter().zip(&base) {
                        assert_eq!(g.0, b.0, "same halves, same order at {he:?}");
                        assert_eq!(g.1, b.1, "same deck at {he:?}");
                        assert_eq!((g.2.x, g.2.y), (b.2.x, b.2.y), "same start at {he:?}");
                    }
                }
                for (&(u, v), w) in probes.iter().zip(&want) {
                    assert_eq!(
                        place(&nulled, face, chart.eval(u, v)),
                        *w,
                        "ring {ring:?}, placeholder at {he:?}: ({u}, {v})"
                    );
                }
            }
        }
    }
}

// ---- the f64 cut's charge ----

/// The band of the tightest ε the kernel runs at (the `1e-12` row), where
/// an `f64` cut's rounding is a band's size: the charge's rows read here
/// whatever the run's ε, each fixture sized so its rounding passes this
/// band's escalation.
fn tight_band() -> Band {
    Band::new(1e-12, 1e-11).unwrap()
}

fn lift(x: f64) -> geom_core::Interval {
    <geom_core::Interval as geom_core::Real>::from_f64(x)
}

/// **A point on a far rational sheet is not read off its carrier**
/// ([`super::carrier_walk`], the walk [`read_capped`] reads, every cell
/// and foot placed in): a knotted rational chart `1.8e5` across, `4e4`
/// out, weights three decades apart, whose `v = 0` row is the straight
/// line `y = 3e4, z = 2e4`; the point is on that line, inside the row's
/// span, so its distance from the chart is zero. Uncharged, the cut's
/// rounding puts every cell's box past the band and the walk reads it
/// off the carrier (`SPLICE_SPLINE_REGION_MUTANT=spline-region-charge-dropped`,
/// and the uncentred, uncharged walk this one replaced).
#[test]
fn a_point_on_a_far_rational_sheet_is_not_read_off_its_carrier() {
    let (s, o) = (177_827.941_003_892_28, [-2e4, 3e4, 2e4]);
    let zs = [0.125, -0.25, -0.125, -0.25];
    let mut control = Vec::new();
    for (i, z) in zs.into_iter().enumerate() {
        let x = i as f64 / 3.0 * s + o[0];
        control.push(Point3::new(x, o[1], o[2]));
        control.push(Point3::new(x, o[1] + s, o[2] + z * s));
    }
    let chart = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        vec![
            12.625, 0.0625, 0.140625, 0.03125, 2.265625, 0.015625, 0.328125, 0.625,
        ],
    )
    .unwrap();
    let q = Point3::new(o[0] + 0.59375 * s, o[1], o[2]);
    let cells = bezier_cells(&chart).unwrap();
    let reading = super::carrier_walk(&chart, &cells, q, tight_band(), Caps::READ, |_| Place::In);
    assert!(
        !matches!(reading, Reading::OffCarrier),
        "a point on the sheet reads off its carrier: {reading:?}"
    );
}

/// A double-double `hi + lo`: the cut's reference arithmetic, near
/// enough exact that its own rounding is a few units of `u²`.
#[derive(Clone, Copy, Debug)]
struct Dd(f64, f64);

impl Dd {
    fn of(x: f64) -> Self {
        Self(x, 0.0)
    }

    fn two_sum(a: f64, b: f64) -> Self {
        let s = a + b;
        let bb = s - a;
        Self(s, (a - (s - bb)) + (b - bb))
    }

    fn add(self, o: Self) -> Self {
        let s = Self::two_sum(self.0, o.0);
        Self::two_sum(s.0, s.1 + self.1 + o.1)
    }

    fn sub(self, o: Self) -> Self {
        self.add(Self(-o.0, -o.1))
    }

    fn mul(self, o: Self) -> Self {
        let p = self.0 * o.0;
        Self::two_sum(p, self.0.mul_add(o.0, -p) + self.0 * o.1 + self.1 * o.0)
    }

    fn div(self, o: Self) -> Self {
        let q = self.0 / o.0;
        let r = self.sub(o.mul(Self::of(q)));
        Self::two_sum(q, r.0 / o.0)
    }
}

/// A single-span net and its cut, beside the same net cut in
/// double-double: the chart's net moved about the cell's centre into
/// homogeneous form, as [`bezier_cells`] holds it.
fn dd_net(chart: &NurbsSurface<f64>, centre: geom_core::Vec3<f64>) -> Vec<[Dd; 4]> {
    chart
        .control()
        .iter()
        .zip(chart.weights())
        .map(|(p, &w)| {
            let w = Dd::of(w);
            let m = |x: f64, c: f64| Dd::of(x).sub(Dd::of(c)).mul(w);
            [m(p.x, centre.x), m(p.y, centre.y), m(p.z, centre.z), w]
        })
        .collect()
}

/// `net` split at the fraction `s` along `u` (`along_u`) or `v`, the
/// half kept by `left`, in double-double (the reference of
/// [`super::Patch::split`]).
fn dd_split(
    net: &[[Dd; 4]],
    (pu, pv): (usize, usize),
    along_u: bool,
    s: f64,
    left: bool,
) -> Vec<[Dd; 4]> {
    let (n, m) = if along_u { (pu, pv) } else { (pv, pu) };
    let at = |i: usize, k: usize| {
        if along_u {
            i * (pv + 1) + k
        } else {
            k * (pv + 1) + i
        }
    };
    let t = Dd::of(s);
    let mut out = net.to_vec();
    for k in 0..=m {
        let mut row: Vec<[Dd; 4]> = (0..=n).map(|i| net[at(i, k)]).collect();
        let (mut lo, mut hi) = (Vec::new(), Vec::new());
        for level in 0..=n {
            lo.push(row[0]);
            hi.push(row[n - level]);
            for i in 0..n - level {
                let (a, b) = (row[i], row[i + 1]);
                row[i] = core::array::from_fn(|c| a[c].add(b[c].sub(a[c]).mul(t)));
            }
        }
        hi.reverse();
        for i in 0..=n {
            out[at(i, k)] = if left { lo[i] } else { hi[i] };
        }
    }
    out
}

/// The worst, over the cell's points and coordinates, of each point's
/// drift from its double-double reference over its rounding bound
/// ([`super::point_rounding`]).
fn drift_over_bound(cell: &super::Patch<f64>, reference: &[[Dd; 4]]) -> f64 {
    let mut worst = 0.0f64;
    for (control, exact) in cell.net.iter().zip(reference) {
        let bound = super::point_rounding(control);
        for c in 0..3 {
            let drift = Dd::of(control.0[c] / control.0[3])
                .sub(exact[c].div(exact[3]))
                .0
                .abs();
            worst = worst.max(drift / bound[c]);
        }
    }
    worst
}

/// **The cut's rounding bound holds the cut's drift**: single-span
/// rational patches of degree one to four, `1` to `1e6` across, placed
/// up to `1e6` away, weights up to eight decades apart, each cut along a
/// random path of six quarterings and one split at a random fraction
/// (as [`super::Patch::restricted`] cuts), against the same cut in
/// double-double. No point drifts past its bound. The insertion's bound
/// is `geom_brep`'s spline incidence's, measured there against a
/// knotted cut (`the_cut_rounding_bound_holds_a_knotted_cuts_drift`).
#[test]
fn the_cut_rounding_bound_holds_the_cut_drift() {
    let mut rng = test_utils::fuzz::start("spline-region-cut-drift");
    let mut worst = 0.0f64;
    for _ in 0..test_utils::fuzz::scaled(1000) {
        let (pu, pv) = (1 + rng.below(4), 1 + rng.below(4));
        let k = |p: usize| {
            KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).unwrap()
        };
        let (scale, off) = (
            10f64.powf(rng.range(0.0, 6.0)),
            10f64.powf(rng.range(0.0, 6.0)),
        );
        let control = (0..(pu + 1) * (pv + 1))
            .map(|_| {
                Point3::new(
                    rng.range(-1.0, 1.0) * scale + off,
                    rng.range(-1.0, 1.0) * scale - off,
                    rng.range(-1.0, 1.0) * scale,
                )
            })
            .collect();
        let spread = rng.range(0.0, 4.0);
        let weights = (0..(pu + 1) * (pv + 1))
            .map(|_| 10f64.powf(rng.range(-spread, spread)))
            .collect();
        let chart = NurbsSurface::new(k(pu), k(pv), control, weights).unwrap();
        let mut cell = bezier_cells(&chart).unwrap().remove(0);
        let mut exact = dd_net(&chart, cell.centre);
        worst = worst.max(drift_over_bound(&cell, &exact));
        for step in 0..7 {
            let (along_u, s, left) = if step == 3 {
                (rng.below(2) == 0, rng.range(0.05, 0.95), rng.below(2) == 0)
            } else {
                (step % 2 == 0, 0.5, rng.below(2) == 0)
            };
            let (l, r) = cell.split(along_u, s);
            cell = if left { l } else { r };
            exact = dd_split(&exact, (pu, pv), along_u, s, left);
            worst = worst.max(drift_over_bound(&cell, &exact));
        }
    }
    eprintln!("[spline-region-cut-drift] worst drift / bound {worst}");
    assert!(
        worst <= 1.0,
        "a cut point drifts {worst} times its rounding bound; {}",
        test_utils::fuzz::replay()
    );
}

/// **The cut's rounding bound is not loose by half**: one bilinear
/// rational patch whose net, moved about its centre into homogeneous
/// form and back, drifts more than half its bound in `x`. Halved
/// (`SPLICE_SPLINE_REGION_MUTANT=spline-region-bound-halved`), the bound
/// no longer holds it.
#[test]
fn the_cut_rounding_bound_comes_within_half_of_a_points_drift() {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let chart = NurbsSurface::new(
        k(),
        k(),
        vec![
            Point3::new(-238.77494180773263, -75.44209369912775, 24.927673166782743),
            Point3::new(38.653231319340016, 402.1064266784137, 21.6381793477302),
            Point3::new(469.3367931984738, 441.01805823091445, -45.592431086187766),
            Point3::new(418.6754132603401, -77.29399775704272, -117.47042168432851),
        ],
        vec![
            0.34391945461948203,
            41.40593693073497,
            27.63484388360674,
            237.43592290619594,
        ],
    )
    .unwrap();
    let cell = bezier_cells(&chart).unwrap().remove(0);
    let ratio = drift_over_bound(&cell, &dd_net(&chart, cell.centre));
    assert!(
        (0.5..=1.0).contains(&ratio),
        "the drift is {ratio} of its bound, outside (½, 1]"
    );
}

/// **A query on a far trim curve is given no side**, at `f64` and at
/// `Interval` ([`Trim::place`]): the chart is flat, `1e3` across, its
/// `v` domain `[999, 1001]` (each unit of `v` 500 m), and the trim's
/// lower side is a rational quadratic image along `v = 1000` exactly
/// (weights three decades apart), closed by segments. The query lies on
/// that side. Uncharged, its pieces' boxes, cut in `f64` (and at
/// `Interval` collapsed to their value channel), sit ulps of `1000` off
/// the line, decided past the band once metered, and the query is read
/// in (`SPLICE_SPLINE_REGION_MUTANT=spline-region-trim-uncharged`).
#[test]
fn a_query_on_a_far_trim_curve_is_given_no_side() {
    use super::{Piece, bezier_pieces};
    let (c, a) = (1e3, 1e3);
    let chart = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![c - 1.0, c - 1.0, c + 1.0, c + 1.0], 1).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, a, 0.0),
            Point3::new(a, 0.0, 0.0),
            Point3::new(a, a, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let image = geom::NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        [0.2, 0.4, 0.6, 0.8]
            .into_iter()
            .map(|u| Point2::new(u, c))
            .collect(),
        vec![0.09375, 32.40625, 0.078125, 68.78125],
    )
    .unwrap();
    let domain = super::Rect {
        u: (0.0, 1.0),
        v: (c - 1.0, c + 1.0),
    };
    let trim = |mut run: Vec<Piece>| {
        let v = |u: f64, v: f64| Piece::vertex([u, v], [0.0; 2]);
        run.push(Piece::segment(run.last().unwrap().last(), v(0.8, c + 0.5)));
        run.push(Piece::segment(v(0.8, c + 0.5), v(0.2, c + 0.5)));
        run.push(Piece::segment(v(0.2, c + 0.5), run[0].first()));
        Trim::from_pieces(run, domain, [None, None], Caps::READ).unwrap()
    };
    let q = super::Rect::point(0.59375, c);
    let at = trim(bezier_pieces(&image, 0.0, 1.0).unwrap()).place(
        &bezier_cells(&chart).unwrap(),
        q,
        tight_band(),
    );
    assert!(matches!(at, Place::Near(_)), "f64: {at:?}");
    let at = trim(bezier_pieces(&image.map_scalar(lift), 0.0, 1.0).unwrap()).place(
        &bezier_cells(&chart.map_scalar(lift)).unwrap(),
        q,
        tight_band(),
    );
    assert!(matches!(at, Place::Near(_)), "Interval: {at:?}");
}

/// Knotted cuts against a double-double Boehm reference: the cells'
/// insertions and the trim's pieces.
mod knotted {
    use super::super::{Piece, bezier_cells, bezier_pieces};
    use super::Dd;
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::{Point2, Point3, Vec3};

    fn knots(rng: &mut test_utils::fuzz::Rng, p: usize) -> KnotVector {
        let mut interior: Vec<f64> = (0..rng.below(5)).map(|_| rng.range(0.05, 0.95)).collect();
        interior.sort_by(f64::total_cmp);
        let mut v = vec![0.0; p + 1];
        for x in interior {
            v.extend(core::iter::repeat_n(x, 1 + rng.below(p)));
        }
        v.extend(core::iter::repeat_n(1.0, p + 1));
        KnotVector::clamped(v, p).unwrap()
    }

    /// Boehm's insertion in double-double, every interior knot (and each
    /// of `extra`) raised to `p`.
    fn dd_refine<const N: usize>(
        knots: &[f64],
        p: usize,
        row: Vec<[Dd; N]>,
        extra: &[f64],
    ) -> Vec<[Dd; N]> {
        let (mut knots, mut row) = (knots.to_vec(), row);
        let (first, last) = (knots[0], knots[knots.len() - 1]);
        let mut interior: Vec<f64> = knots
            .iter()
            .copied()
            .chain(extra.iter().copied())
            .filter(|&x| x != first && x != last)
            .collect();
        interior.sort_by(f64::total_cmp);
        interior.dedup();
        for x in interior {
            while knots.iter().filter(|&&k| k == x).count() < p {
                let s = knots.iter().filter(|&&k| k == x).count();
                let k = knots.iter().rposition(|&u| u <= x).unwrap();
                let mut out = Vec::with_capacity(row.len() + 1);
                for i in 0..=row.len() {
                    if i + p <= k {
                        out.push(row[i]);
                    } else if i + s > k {
                        out.push(row[i - 1]);
                    } else {
                        let a = Dd::of(x)
                            .sub(Dd::of(knots[i]))
                            .div(Dd::of(knots[i + p]).sub(Dd::of(knots[i])));
                        let (lo, hi) = (row[i - 1], row[i]);
                        out.push(core::array::from_fn(|c| lo[c].add(hi[c].sub(lo[c]).mul(a))));
                    }
                }
                knots.insert(k + 1, x);
                row = out;
            }
        }
        row
    }

    fn drift(x: f64, exact: Dd) -> f64 {
        let d = Dd::of(x).sub(exact);
        (d.0 + d.1).abs()
    }

    /// **The cut's rounding bound holds a knotted cut's drift**: knotted
    /// rational charts, any interior multiplicity, weights up to eight
    /// decades apart, placed up to `1e5` of their size away, each
    /// Bézier cell [`bezier_cells`] cuts (then split at random
    /// fractions) against Boehm's insertion in double-double.
    #[test]
    fn the_cut_rounding_bound_holds_a_knotted_cuts_drift() {
        let rng = &mut test_utils::fuzz::start("spline-region-knotted-drift");
        let mut worst = 0.0f64;
        for case in 0..test_utils::fuzz::scaled(600) {
            let (pu, pv) = (1 + rng.below(4), 1 + rng.below(4));
            let (ku, kv) = (knots(rng, pu), knots(rng, pv));
            let (nu, nv) = (ku.control_count(), kv.control_count());
            let scale = 10f64.powf(rng.range(-2.0, 4.0));
            let o = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            ) * scale
                * [0.0, 10.0, 1e3, 1e5][rng.below(4)];
            let control: Vec<Point3<f64>> = (0..nu * nv)
                .map(|_| {
                    Point3::origin()
                        + Vec3::new(
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                        ) * scale
                        + o
                })
                .collect();
            let decades = [0.0, 2.0, 4.0, 8.0][rng.below(4)];
            let weights: Vec<f64> = (0..nu * nv)
                .map(|_| 10f64.powf(rng.range(-decades / 2.0, decades / 2.0)))
                .collect();
            let chart = NurbsSurface::new(ku, kv, control.clone(), weights.clone()).unwrap();
            let cells = bezier_cells(&chart).unwrap();
            let c = cells[0].centre;
            let at = |k: usize| {
                let w = Dd::of(weights[k]);
                let m = |x: f64, c: f64| Dd::of(x).sub(Dd::of(c)).mul(w);
                let p = control[k];
                [m(p.x, c.x), m(p.y, c.y), m(p.z, c.z), w]
            };
            let (u, v) = (
                chart.knot_values_u().values(),
                chart.knot_values_v().values(),
            );
            let rows: Vec<Vec<[Dd; 4]>> = (0..nu)
                .map(|i| dd_refine(v, pv, (0..nv).map(|j| at(i * nv + j)).collect(), &[]))
                .collect();
            let columns: Vec<Vec<[Dd; 4]>> = (0..rows[0].len())
                .map(|j| dd_refine(u, pu, rows.iter().map(|r| r[j]).collect(), &[]))
                .collect();
            let (cu, cv) = ((columns[0].len() - 1) / pu, (columns.len() - 1) / pv);
            assert_eq!(
                cells.len(),
                cu * cv,
                "case {case}: the cells are the reference's"
            );
            for i in 0..cu {
                for j in 0..cv {
                    let mut exact = Vec::new();
                    for a in 0..=pu {
                        for b in 0..=pv {
                            exact.push(columns[j * pv + b][i * pu + a]);
                        }
                    }
                    let mut cell = cells[i * cv + j].clone();
                    for step in 0..rng.below(6) {
                        let (along_u, s, left) = (
                            rng.below(2) == 0,
                            if step % 2 == 0 {
                                0.5
                            } else {
                                rng.range(0.05, 0.95)
                            },
                            rng.below(2) == 0,
                        );
                        let (l, r) = cell.split(along_u, s);
                        cell = if left { l } else { r };
                        exact = super::dd_split(&exact, (pu, pv), along_u, s, left);
                    }
                    for (got, ex) in cell.net.iter().zip(&exact) {
                        let bound = super::super::point_rounding(got);
                        for k in 0..3 {
                            let d = drift(got.0[k] / got.0[3], ex[k].div(ex[3]));
                            worst = worst.max(d / bound[k]);
                            assert!(
                                d <= bound[k],
                                "case {case}: drift {d:e} past bound {:e}; {}",
                                bound[k],
                                test_utils::fuzz::replay()
                            );
                        }
                    }
                }
            }
        }
        eprintln!("[spline-region-knotted-drift] worst drift / bound {worst}");
    }

    /// **The trim's pieces hold their drift**: random rational knotted
    /// image curves, far from the origin in the chart, cut by
    /// [`bezier_pieces`] over a random `[a, b]` (inserting knots that
    /// are not breaks), each piece then halved, against double-double.
    #[test]
    fn the_trim_pieces_rounding_bound_holds_their_drift() {
        let rng = &mut test_utils::fuzz::start("spline-region-piece-drift");
        let mut worst = 0.0f64;
        for case in 0..test_utils::fuzz::scaled(1500) {
            let p = 1 + rng.below(4);
            let kv = knots(rng, p);
            let n = kv.control_count();
            let scale = 10f64.powf(rng.range(-3.0, 1.0));
            let off = 10f64.powf(rng.range(0.0, 6.0)) * [1.0, -1.0][rng.below(2)];
            let control: Vec<Point2<f64>> = (0..n)
                .map(|_| {
                    Point2::new(
                        rng.range(-1.0, 1.0) * scale + off,
                        rng.range(-1.0, 1.0) * scale - off,
                    )
                })
                .collect();
            let decades = [0.0, 2.0, 4.0, 8.0][rng.below(4)];
            let weights: Vec<f64> = (0..n)
                .map(|_| 10f64.powf(rng.range(-decades / 2.0, decades / 2.0)))
                .collect();
            let image =
                geom::NurbsCurve2::new(kv.clone(), control.clone(), weights.clone()).unwrap();
            let (mut a, mut b) = (rng.range(0.0, 1.0), rng.range(0.0, 1.0));
            if a > b {
                core::mem::swap(&mut a, &mut b);
            }
            if rng.below(3) == 0 {
                a = 0.0;
            }
            if rng.below(3) == 0 {
                b = 1.0;
            }
            if b - a < 1e-3 {
                continue;
            }
            let Some(pieces) = bezier_pieces(&image, a, b) else {
                panic!("case {case}: no pieces");
            };
            let row: Vec<[Dd; 3]> = control
                .iter()
                .zip(&weights)
                .map(|(c, &w)| {
                    let w = Dd::of(w);
                    [Dd::of(c.x).mul(w), Dd::of(c.y).mul(w), w]
                })
                .collect();
            let exact = dd_refine(kv.knots(), p, row, &[a, b]);
            // The pieces are the reference's cells between a and b.
            let mut bk: Vec<f64> = kv.knots().iter().copied().chain([a, b]).collect();
            bk.sort_by(f64::total_cmp);
            bk.dedup();
            let firsts: Vec<usize> = (0..bk.len() - 1)
                .filter(|&k| bk[k] >= a && bk[k + 1] <= b)
                .collect();
            assert_eq!(firsts.len(), pieces.len(), "case {case}");
            let check = |piece: &Piece, ex: &[[Dd; 3]], worst: &mut f64| {
                for (got, e) in piece.net.iter().zip(ex) {
                    let bound = super::super::point_rounding(got);
                    for k in 0..2 {
                        let d = drift(got.0[k] / got.0[2], e[k].div(e[2]));
                        *worst = worst.max(d / bound[k]);
                        assert!(
                            d <= bound[k],
                            "case {case}: piece drift {d:e} past bound {:e}; {}",
                            bound[k],
                            test_utils::fuzz::replay()
                        );
                    }
                }
            };
            for (piece, &k) in pieces.iter().zip(&firsts) {
                let mut ex: Vec<[Dd; 3]> = exact[k * p..=k * p + p].to_vec();
                let mut piece = piece.clone();
                check(&piece, &ex, &mut worst);
                for _ in 0..rng.below(8) {
                    let left = rng.below(2) == 0;
                    let (l, r) = piece.halves();
                    piece = if left { l } else { r };
                    // de Casteljau at 1/2 in double-double.
                    let mut rowd = ex.clone();
                    let (mut lo, mut hi) = (Vec::new(), Vec::new());
                    for level in 0..=p {
                        lo.push(rowd[0]);
                        hi.push(rowd[p - level]);
                        for i in 0..p - level {
                            let (x, y) = (rowd[i], rowd[i + 1]);
                            rowd[i] =
                                core::array::from_fn(|c| x[c].add(y[c].sub(x[c]).mul(Dd::of(0.5))));
                        }
                    }
                    hi.reverse();
                    ex = if left { lo } else { hi };
                    check(&piece, &ex, &mut worst);
                }
            }
        }
        eprintln!("[spline-region-piece-drift] worst drift / bound {worst}");
    }
}

/// **A query on a wide `Interval` trim image is given no side**: the far
/// trim row's image at `Interval`, every control's `v` a bracket
/// `[c − 4e-11, c + 4e-11]`, so the line `v = c + 3e-11` is one of the
/// curves it stands for; a query on that line is within the band of the
/// trim. The piece held at its brackets' middles is off by their
/// half-width, and only that charge keeps the query near (the half-width
/// dropped, the query reads in).
#[test]
fn a_query_on_a_wide_interval_trim_image_is_given_no_side() {
    use super::{Piece, bezier_pieces};
    let (c, a) = (1e3, 1e3);
    let chart = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![c - 1.0, c - 1.0, c + 1.0, c + 1.0], 1).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, a, 0.0),
            Point3::new(a, 0.0, 0.0),
            Point3::new(a, a, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let w = 4e-11;
    let image = geom::NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        [0.2, 0.4, 0.6, 0.8]
            .into_iter()
            .map(|u| Point2::new(u, c))
            .collect(),
        vec![0.09375, 32.40625, 0.078125, 68.78125],
    )
    .unwrap()
    .map_scalar(|x| {
        if (x - c).abs() < 1.0 {
            geom_core::Interval::from_bounds(x - w, x + w)
        } else {
            lift(x)
        }
    });
    let domain = super::Rect {
        u: (0.0, 1.0),
        v: (c - 1.0, c + 1.0),
    };
    let mut run = bezier_pieces(&image, 0.0, 1.0).unwrap();
    let v = |u: f64, v: f64| Piece::vertex([u, v], [0.0; 2]);
    run.push(Piece::segment(run.last().unwrap().last(), v(0.8, c + 0.5)));
    run.push(Piece::segment(v(0.8, c + 0.5), v(0.2, c + 0.5)));
    run.push(Piece::segment(v(0.2, c + 0.5), run[0].first()));
    let trim = Trim::from_pieces(run, domain, [None, None], Caps::READ).unwrap();
    let cells = bezier_cells(&chart.map_scalar(lift)).unwrap();
    let mut sided = 0;
    for k in 0..64 {
        let u = 0.25 + 0.5 * (k as f64 + 0.5) / 64.0;
        let q = super::Rect::point(u, c + 3e-11);
        if !matches!(trim.place(&cells, q, tight_band()), Place::Near(_)) {
            sided += 1;
        }
    }
    assert_eq!(
        sided, 0,
        "{sided} of 64 queries on a member of the trim got a side"
    );
}

/// **A piece whose rounding bound is NaN has no box**
/// ([`super::Piece::bbox`]): a NaN bound refuses, and is not dropped by
/// the per-channel fold to leave the box unwidened.
#[test]
fn a_piece_with_a_nan_bound_has_no_box() {
    use super::Piece;
    let piece = Piece {
        net: vec![
            ([0.0, 0.0, 1.0], [0.0, f64::NAN, 0.0]),
            ([1.0, 1.0, 1.0], [0.0; 3]),
        ],
    };
    let b = piece.bbox();
    assert!(b.is_none(), "a NaN bound gives the box {b:?}");
}

/// **A cell whose rounding bound is NaN has no floor past zero**
/// ([`super::Patch::floor`]): a NaN bound poisons the floor, and is not
/// dropped by the per-channel fold to leave the floor at the box gap.
#[test]
fn a_cell_with_a_nan_bound_has_no_floor() {
    let net = vec![
        ([0.0, 0.0, 0.0, 1.0], [f64::NAN, 0.0, 0.0, 0.0]),
        ([1.0, 0.0, 0.0, 1.0], [0.0; 4]),
        ([0.0, 1.0, 0.0, 1.0], [0.0; 4]),
        ([1.0, 1.0, 0.0, 1.0], [0.0; 4]),
    ];
    let cell = super::Patch {
        rect: super::Rect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        },
        pu: 1,
        pv: 1,
        net,
        centre: Vec3::new(0.0, 0.0, 0.0),
    };
    let floor = cell.floor(Point3::new(0.5, 0.5, 1.0));
    assert!(floor.is_nan(), "a NaN bound leaves the floor {floor}");
}

/// A unit square cell at `z = 0`, held about `centre`, each point
/// carrying the rounding bound `e` on its `z` channel.
fn square_cell(centre: Vec3<f64>, e: f64) -> super::Patch<f64> {
    let net = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| ([x, y, 0.0, 1.0], [0.0, 0.0, e, 0.0]))
        .to_vec();
    super::Patch {
        rect: super::Rect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        },
        pu: 1,
        pv: 1,
        net,
        centre,
    }
}

/// **A cell's floor is lowered by the rounding its points carry and by
/// the read's** ([`super::Patch::floor`]), each term alone: no fixture
/// makes either decide alone (each covers the other's drift wherever
/// `q` meets a box face, the site argues), so the composition is
/// pinned here, and the bounds themselves by the drift rows. A cell
/// whose points carry `1e-6` in `z`, `2e-6` below it, about the origin,
/// floors at about `1e-6`; the same cell carrying nothing, held `1e8`
/// away with `q` `2e-6` below it in the cell's frame, floors at the gap
/// less the read's `16u·(reach + |q − c| + 1e8)`.
#[test]
fn a_cells_floor_is_lowered_by_its_cut_and_its_read() {
    let below = 2e-6;
    let cut = square_cell(Vec3::new(0.0, 0.0, 0.0), 1e-6);
    let floor = cut.floor(Point3::new(0.5, 0.5, -below));
    assert!(
        floor <= below - 1e-6 + 1e-12,
        "the cut's bound is charged: floor {floor:e}"
    );
    let far = 1e8;
    let read = square_cell(Vec3::new(far, 0.0, 0.0), 0.0);
    let floor = read.floor(Point3::new(far + 0.5, 0.5, -below));
    let charge = geom_core::rounding_charge(2.0 * (2f64.sqrt() + 0.5f64.hypot(0.5) + far));
    assert!(
        floor <= below - 0.9 * charge && charge > 0.0,
        "the read is charged: floor {floor:e}, charge {charge:e}"
    );
}

/// **A query on a far trim curve along `v` is given no side**, the
/// transpose of [`a_query_on_a_far_trim_curve_is_given_no_side`]: the
/// image lies along `u = 1000` exactly, so it is the box's widening in
/// `u` that keeps the query near
/// (`SPLICE_SPLINE_REGION_MUTANT=spline-region-u-unwidened`).
#[test]
fn a_query_on_a_far_trim_curve_along_v_is_given_no_side() {
    use super::{Piece, bezier_pieces};
    let (c, a) = (1e3, 1e3);
    let chart = NurbsSurface::new(
        KnotVector::clamped(vec![c - 1.0, c - 1.0, c + 1.0, c + 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, a, 0.0),
            Point3::new(a, 0.0, 0.0),
            Point3::new(a, a, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let image = geom::NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        [0.2, 0.4, 0.6, 0.8]
            .into_iter()
            .map(|v| Point2::new(c, v))
            .collect(),
        vec![0.09375, 32.40625, 0.078125, 68.78125],
    )
    .unwrap();
    let domain = super::Rect {
        u: (c - 1.0, c + 1.0),
        v: (0.0, 1.0),
    };
    let mut run = bezier_pieces(&image, 0.0, 1.0).unwrap();
    let v = |u: f64, v: f64| Piece::vertex([u, v], [0.0; 2]);
    run.push(Piece::segment(run.last().unwrap().last(), v(c - 0.5, 0.8)));
    run.push(Piece::segment(v(c - 0.5, 0.8), v(c - 0.5, 0.2)));
    run.push(Piece::segment(v(c - 0.5, 0.2), run[0].first()));
    let trim = Trim::from_pieces(run, domain, [None, None], Caps::READ).unwrap();
    let cells = bezier_cells(&chart).unwrap();
    let sided = (0..64)
        .filter(|k| {
            let q = super::Rect::point(c, 0.25 + 0.5 * (f64::from(*k) + 0.5) / 64.0);
            !matches!(trim.place(&cells, q, tight_band()), Place::Near(_))
        })
        .count();
    assert_eq!(sided, 0, "{sided} of 64 queries on the trim got a side");
}

/// **A shifted piece's rounding bound holds its drift**
/// ([`super::Piece::shifted`], a ring moved whole periods): rational
/// control points up to `1e4` out in the chart, weights up to eight
/// decades apart, moved by up to `1e4` in each channel, against the move
/// in double-double. No point drifts past its bound
/// (`SPLICE_SPLINE_REGION_MUTANT=spline-region-shift-untracked`: the move
/// charged nothing, the drift passes the bound).
#[test]
fn a_shifted_pieces_rounding_bound_holds_its_drift() {
    let mut rng = test_utils::fuzz::start("spline-region-shift-drift");
    let mut worst = 0.0f64;
    for _ in 0..test_utils::fuzz::scaled(4000) {
        let w = 10f64.powf(rng.range(-4.0, 4.0));
        let p = [rng.range(-1e4, 1e4), rng.range(-1e4, 1e4)];
        let h = [p[0] * w, p[1] * w, w];
        let piece = super::Piece {
            net: vec![(h, [0.0; 3]), (h, [0.0; 3])],
        };
        let d = [rng.range(-1e4, 1e4), rng.range(-1e4, 1e4)];
        let moved = piece.shifted(d[0], d[1]).net[0];
        let bound = super::point_rounding(&moved);
        for c in 0..2 {
            let exact = Dd::of(h[c]).add(Dd::of(d[c]).mul(Dd::of(w))).div(Dd::of(w));
            let drift = Dd::of(moved.0[c] / moved.0[2]).sub(exact).0.abs();
            worst = worst.max(drift / bound[c]);
        }
    }
    eprintln!("[spline-region-shift-drift] worst drift / bound {worst}");
    assert!(
        worst <= 1.0,
        "a shifted point drifts {worst} times its bound; {}",
        test_utils::fuzz::replay()
    );
}

/// **An iso image's end holds its evaluation's rounding**
/// ([`super::image_pieces`] on a [`Pcurve::IsoLine`]): lines `p0 + pl·t`
/// with `p0` up to `1e4` out in the chart and `pl·t` of its size, each
/// end against `p0 + pl·t` in double-double. No end drifts past its
/// bound (`SPLICE_SPLINE_REGION_MUTANT=spline-region-iso-end-uncharged`:
/// the end charged nothing, the drift passes the bound).
#[test]
fn an_iso_images_ends_hold_their_evaluations_rounding() {
    let mut rng = test_utils::fuzz::start("spline-region-iso-end-drift");
    let mut worst = 0.0f64;
    for _ in 0..test_utils::fuzz::scaled(4000) {
        let p0 = Point2::new(rng.range(-1e4, 1e4), rng.range(-1e4, 1e4));
        let pl = Vec2::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
        let (t0, t1) = (rng.range(0.0, 1e4), rng.range(1e4, 2e4));
        let line = Pcurve::IsoLine { p0, pl };
        let scale = p0.x.abs().max(p0.y.abs());
        let run = super::image_pieces(&line, t0, t1, scale).unwrap();
        for (end, t) in [(run[0].first(), t0), (run[0].last(), t1)] {
            let bound = super::point_rounding(&end);
            for (c, (a, b)) in [(p0.x, pl.x), (p0.y, pl.y)].into_iter().enumerate() {
                let exact = Dd::of(a).add(Dd::of(b).mul(Dd::of(t)));
                let drift = Dd::of(end.0[c] / end.0[2]).sub(exact).0.abs();
                worst = worst.max(drift / bound[c]);
            }
        }
    }
    eprintln!("[spline-region-iso-end-drift] worst drift / bound {worst}");
    assert!(
        worst <= 1.0,
        "an iso end drifts {worst} times its bound; {}",
        test_utils::fuzz::replay()
    );
}

/// **A spline face's stations read on its own chart, never off its
/// carrier** (`crate::cell_region`'s held door). The twisted bilinear
/// patch `(u, v, u·v)`, a sheet over `[0.1, 0.9]²` of it: every station
/// of a 21 × 21 grid over the whole chart is placed by the curved door,
/// and none reads `OffCarrier`. A station is a point of the chart, and
/// the spline arm reads it by its own search, which `Held::check` does
/// not guard. `cell_region::station_region` answers `In` well inside the
/// sheet and `Out` well outside it, and panics nowhere.
#[test]
fn a_spline_faces_stations_read_on_its_own_chart() {
    let (body, face) = sheet(
        &twisted(1.0),
        &[(0.1, 0.1), (0.9, 0.1), (0.9, 0.9), (0.1, 0.9)],
    );
    let surface = body.face(face).unwrap().surface().clone();
    let n = 20;
    for i in 0..=n {
        for j in 0..=n {
            let (u, v) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
            let at = curved_face_placement(&body, face, surface.eval(u, v), band());
            assert!(
                !matches!(at, Ok(CurvedPlacement::OffCarrier)),
                "the station ({u}, {v}) reads off its own carrier: {at:?}"
            );
            let _ = crate::cell_region::station_region(&body, face, (u, v), band());
        }
    }
    let region = |uv| crate::cell_region::station_region(&body, face, uv, band());
    assert_eq!(region((0.5, 0.5)), Some(FaceContainment::In));
    assert_eq!(region((0.02, 0.5)), Some(FaceContainment::Out));
}

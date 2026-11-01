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
    assert_eq!(diag.predicate, Some(super::CARRIER), "{diag:?}");
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
            matches!(near, Place::Near(Some(ref d)) if d.predicate == Some(super::TRIM)),
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
            .map(|k| super::Piece::segment(p[k], p[(k + 1) % 4]))
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
        .map(|k| super::Piece::segment(p[k], p[(k + 1) % 4]))
        .collect();
    let unit = super::Rect {
        u: (0.0, 1.0),
        v: (0.0, 1.0),
    };
    let trim = Trim::from_pieces(square, unit, [None, None], Caps::READ).unwrap();
    let near = trim.place(&cells, super::Rect::point(0.5, 0.35), b);
    assert!(
        matches!(near, Place::Near(Some(ref d)) if d.predicate == Some(super::TRIM)),
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

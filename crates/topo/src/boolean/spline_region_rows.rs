#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
//! Rows for [`super`]: a spline face's region, on charts that are affine
//! along every iso line (degree 1 both ways), so a face's sides are
//! straight edges whose images are iso lines.

use super::{Place, Reading, Trim, bezier_cells, read};
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
fn ruled(rows: &[(f64, f64)]) -> NurbsSurface<f64> {
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
fn twisted(c: f64) -> NurbsSurface<f64> {
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
/// side a straight edge whose image is the iso line it lies on. Every
/// side must stay in one Bézier cell along its own iso direction, where
/// the chart is affine.
fn sheet(chart: &NurbsSurface<f64>, uv: &[(f64, f64)]) -> (Body<f64>, FaceKey) {
    let (mut body, face) = sheet_unminted(chart, uv);
    crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
    (body, face)
}

fn sheet_unminted(chart: &NurbsSurface<f64>, uv: &[(f64, f64)]) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let at = |(u, v): (f64, f64)| chart.eval(u, v);
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
    // The iso lane's line limb reads the whole column, so a side along a
    // creased column states its image as a general degree-1 curve, which
    // certifies over its own span.
    let creased_v = chart.knot_values_v().runs().count() > 2;
    let side = |a: (f64, f64), b: (f64, f64)| {
        let line = EdgeCurveSpec::line_between(at(a), at(b));
        let len = line.param_end;
        #[allow(clippy::float_cmp)]
        let image = if creased_v && a.0 == b.0 {
            Pcurve::General(std::sync::Arc::new(
                geom::NurbsCurve2::new(
                    KnotVector::clamped(vec![0.0, 0.0, len, len], 1).unwrap(),
                    vec![Point2::new(a.0, a.1), Point2::new(b.0, b.1)],
                    vec![1.0; 2],
                )
                .unwrap(),
            ))
        } else {
            Pcurve::IsoLine {
                p0: Point2::new(a.0, a.1),
                pl: Vec2::new((b.0 - a.0) / len, (b.1 - a.1) / len),
            }
        };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(key, image),
            ..line
        }
    };
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(uv[1]),
            side(uv[0], uv[1]),
            tol,
        )
        .unwrap();
    let mut last = first;
    let mut before = None;
    for k in 2..uv.len() {
        let next = body
            .mev(
                MevSite::Fan {
                    he1: last.he_minus,
                    he2: last.he_minus,
                },
                at(uv[k]),
                side(uv[k - 1], uv[k]),
                tol,
            )
            .unwrap();
        before = Some(last);
        last = next;
    }
    let he = body
        .find_half_edge(seed.face, last.vertex, before.unwrap().vertex)
        .unwrap();
    let face = body
        .mef(
            MefSite::Chords {
                he1: he,
                he2: first.he_plus,
            },
            side(uv[uv.len() - 1], uv[0]),
            FaceSurface::Shared { key, sense: true },
            tol,
        )
        .unwrap()
        .face;
    (body, face)
}

/// The rectangle `[u0, u1] × [v0, v1]`, counterclockwise.
fn rect((u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> Vec<(f64, f64)> {
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
    let trim = Trim::of(&body, face, &chart, b).unwrap();
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
    let b = band();
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
    let _ = b;
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
    let trim = Trim::of(&body, face, &chart, b).unwrap();
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
        matches!(
            read(&body, face, &chart, p, b),
            Reading::Out | Reading::Unsettled(_)
        ),
        "the door's own point, by its edge line"
    );
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
    let trim = Trim::of(&body, face, &chart, b).unwrap();
    for (u, want_in) in [(0.5, true), (0.1, false), (0.9, false)] {
        let p = trim.place(&cells, super::Rect::point(u, 0.5), b);
        assert!(
            if want_in { matches!(p, Place::In) } else { matches!(p, Place::Out) },
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
    let trim = Trim::of(&body, face, &chart, b).unwrap();
    for (u, want_in) in [(0.3, true), (0.1, false)] {
        let p = trim.place(&cells, super::Rect::point(u, 0.5), b);
        assert!(
            if want_in { matches!(p, Place::In) } else { matches!(p, Place::Out) },
            "along a side at u = {u}: {p:?}"
        );
    }
}

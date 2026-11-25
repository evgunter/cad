//! **Which NURBS faces the clearance engine admits a window for, and
//! how a trimmed one's window is read**, on bilinear affine charts:
//!
//! - the square lamina whose four edges are the chart's four boundary
//!   isos is the whole patch, and both its faces are admitted;
//! - the triangle lamina (its third edge the diagonal), the same square
//!   on a chart twice as wide (its isos run round half the domain), and
//!   the bored block's top cap re-charted onto a patch (its holes inner
//!   loops) are admitted on the knot domain, a superset;
//! - each trimmed window holds its face's chart images on every side of
//!   the domain;
//! - on that superset the cells' regions read the face: no cell dropped
//!   holds a face point, and an interior cell reads inside;
//! - so the walk reads the face alone: a separation and an embedding
//!   verdict that the chart's unused half would change.
//!
//! Every coordinate is dyadic, so each scalar lifts the fixture exactly.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::Band;
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Interval, Point3, Real, Tol};
use topo::clearance::{
    Budgets, DEFAULT_MAX_CELL_DEPTH, Embedding, EngineRefusal, Side, Which, embedded,
    min_separation,
};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{AtRestPolicy, Body, FaceKey};

/// An affine chart `(u, v) ↦ o + u·a + v·b`, its three vectors.
type Chart = [(f64, f64, f64); 3];

/// The identity chart, lying in `z = 0`.
const FLAT: Chart = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];

/// The lamina through `corners` on the identity chart over
/// `[0, width] × [0, 1]` at `T`, in a body of its own.
fn lamina<T: AtRestPolicy>(width: f64, corners: &[(f64, f64)]) -> (Body<T>, Vec<FaceKey>) {
    let mut body = Body::<T>::new();
    let faces = lamina_in(&mut body, FLAT, width, corners);
    (body, faces)
}

/// The lamina through the chart points `corners` on the affine `chart`
/// over `[0, width] × [0, 1]`, added to `body`: `mvfs`, a `mev_line` to
/// each later corner, and a `mef_chord` back to the first, every edge's
/// pcurve minted. Returns its two faces.
fn lamina_in<T: AtRestPolicy>(
    body: &mut Body<T>,
    [o, a, b]: Chart,
    width: f64,
    corners: &[(f64, f64)],
) -> Vec<FaceKey> {
    let tol = Tol::witness();
    let at = |(u, v): (f64, f64)| {
        (
            o.0 + u * a.0 + v * b.0,
            o.1 + u * a.1 + v * b.1,
            o.2 + u * a.2 + v * b.2,
        )
    };
    let pt =
        |(x, y, z): (f64, f64, f64)| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    let seed = body.mvfs(pt(at(corners[0])), true).unwrap();
    let span = |w: f64| KnotVector::clamped(vec![0.0, 0.0, w, w], 1).unwrap();
    let net = NurbsSurface::new(
        Knots::from(span(width)).lift(Real::from_f64),
        Knots::from(span(1.0)).lift(Real::from_f64),
        [(0.0, 0.0), (0.0, 1.0), (width, 0.0), (width, 1.0)]
            .into_iter()
            .map(|c| pt(at(c)))
            .collect(),
        vec![Real::one(); 4],
    )
    .unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Nurbs(Arc::new(net)),
            sense: true,
        },
    )
    .unwrap();
    let mut prev = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(at(corners[1])),
            tol,
        )
        .unwrap();
    let first = prev.he_plus;
    let mut before = prev.vertex;
    for &corner in &corners[2..] {
        before = prev.vertex;
        prev = body
            .mev_line(
                MevSite::Fan {
                    he1: prev.he_minus,
                    he2: prev.he_minus,
                },
                pt(at(corner)),
                tol,
            )
            .unwrap();
    }
    let back = body.find_half_edge(seed.face, prev.vertex, before).unwrap();
    let closed = body
        .mef_chord(
            MefSite::Chords {
                he1: back,
                he2: first,
            },
            tol,
        )
        .unwrap();
    let faces = vec![seed.face, closed.face];
    topo::mint_pcurves_of(body, &faces, tol).unwrap();
    faces
}

fn windows<T: AtRestPolicy + geom_core::CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<(), EngineRefusal> {
    Side {
        body,
        faces: vec![face],
    }
    .windows(Which::First)
    .map(|_| ())
}

fn admitted<T: AtRestPolicy + geom_core::CertifiedBounds>(body: &Body<T>, faces: &[FaceKey]) {
    for &face in faces {
        assert_eq!(
            windows(body, face),
            Ok(()),
            "{face:?} at {} has a window",
            T::NAME
        );
    }
}

/// The square's four edges are the chart's four boundary isos: both
/// faces of the lamina are the whole patch.
fn the_iso_square_is_admitted<T: AtRestPolicy + geom_core::CertifiedBounds>() {
    let (body, faces) = lamina::<T>(1.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    admitted(&body, &faces);
}

/// The triangle's diagonal is no iso of the chart: its window is the
/// knot domain, a superset.
fn the_triangle_is_admitted<T: AtRestPolicy + geom_core::CertifiedBounds>() {
    let (body, faces) = lamina::<T>(1.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
    admitted(&body, &faces);
}

/// The square `[0, 1]²` on the chart over `[0, 2] × [0, 1]`: its edges
/// are all isos of the chart, and its right edge, `u = 1`, is an
/// interior column, so the loop runs round half the domain.
fn the_half_square_is_admitted<T: AtRestPolicy + geom_core::CertifiedBounds>() {
    let (body, faces) = lamina::<T>(2.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    admitted(&body, &faces);
}

#[test]
fn an_iso_square_lamina_is_admitted_at_f64_and_interval() {
    the_iso_square_is_admitted::<f64>();
    the_iso_square_is_admitted::<Interval>();
}

#[test]
fn a_triangle_lamina_is_admitted_at_f64_and_interval() {
    the_triangle_is_admitted::<f64>();
    the_triangle_is_admitted::<Interval>();
}

#[test]
fn a_square_on_half_its_chart_is_admitted_at_f64_and_interval() {
    the_half_square_is_admitted::<f64>();
    the_half_square_is_admitted::<Interval>();
}

/// **A NURBS face with inner loops is admitted**: the bored block's top
/// cap, re-charted onto the patch `(u, v) ↦ (3u, 2v, 2)`, keeps its
/// holes as inner loops.
#[test]
fn a_cap_with_holes_on_a_patch_is_admitted() {
    let tol = Tol::witness();
    let mut body = topo::test_support::holed_block::<f64>(3.0, &[0.75, 2.25], tol);
    topo::test_support::describe_as_intersections(&mut body, tol);
    let top = body
        .faces()
        .find(|(fk, f)| {
            !f.rings.is_empty()
                && matches!(
                    body.face_surface(*fk),
                    Ok(Surface::Plane { frame, .. }) if frame.axis().z > 0.5
                )
        })
        .map(|(k, _)| k)
        .expect("the top cap holds the holes");
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let patch = NurbsSurface::new(
        unit(),
        unit(),
        vec![
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 2.0, 2.0),
            Point3::new(3.0, 0.0, 2.0),
            Point3::new(3.0, 2.0, 2.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    body.set_face_surface_unvouched_for_tests(
        top,
        FaceSurface::New {
            surface: Surface::Nurbs(Arc::new(patch)),
            sense: true,
        },
    )
    .unwrap();
    admitted(&body, &[top]);
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn budgets() -> Budgets {
    Budgets {
        max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
        max_cell_pairs: 1 << 16,
    }
}

/// **The window holds the face and the walk drops the chart's rest.**
/// The square `[0, 1]²` on the flat chart over `[0, 2] × [0, 1]`, and,
/// `0.125` above, the strip `x ∈ [1.5, 2]` of another lamina's chart
/// over `[1.5, 2.5]`: the faces are `√(0.5² + 0.125²) ≈ 0.515` apart,
/// while the first chart's unused half lies `0.125` under the strip.
/// The bracket holds the distance, and its lower end clears `0.125`:
/// the cells of the unused half were read outside the face and dropped.
#[test]
fn the_separation_of_two_trimmed_spline_faces_reads_their_faces_alone() {
    let (a, fa) = lamina::<Interval>(2.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    let mut b = Body::<Interval>::new();
    let above = [(1.5, 0.0, 0.125), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)];
    let fb = lamina_in(
        &mut b,
        above,
        1.0,
        &[(0.0, 0.0), (0.5, 0.0), (0.5, 1.0), (0.0, 1.0)],
    );
    let m = min_separation(
        &Side {
            body: &a,
            faces: vec![fa[0]],
        },
        &Side {
            body: &b,
            faces: vec![fb[0]],
        },
        false,
        band(),
        Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: 1 << 9,
        },
    )
    .unwrap();
    let truth = (0.5f64 * 0.5 + 0.125 * 0.125).sqrt();
    assert!(m.lo() <= truth, "the lower end {} holds {truth}", m.lo());
    assert!(
        m.hi().is_some_and(|hi| truth <= hi),
        "the upper end {:?} holds {truth}",
        m.hi()
    );
    assert!(
        m.lo() > 0.25,
        "the lower end {} clears twice the unused half's 0.125: its cells were dropped",
        m.lo()
    );
}

/// One face of the half-square lamina in `z = 0`, and one face of a
/// whole-patch lamina standing in the plane `x = at`, `y ∈ [-0.25,
/// 0.75]`, `z ∈ [-0.5, 0.5]`, in one body: the verdict door over the
/// two.
fn standing_through(at: f64) -> (Embedding, [FaceKey; 2]) {
    let mut body = Body::<Interval>::new();
    let lying = lamina_in(
        &mut body,
        FLAT,
        2.0,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
    )[0];
    let standing = [(at, -0.25, -0.5), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)];
    let standing = lamina_in(
        &mut body,
        standing,
        1.0,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
    )[0];
    let side = Side {
        body: &body,
        faces: vec![lying, standing],
    };
    (
        embedded(&side, band(), budgets()).verdict,
        [lying, standing],
    )
}

/// **A plane through a trimmed spline face crosses it**: the standing
/// lamina at `x = 0.5` passes through the lying square along
/// `y ∈ [0, 0.75]`. The door must not read it embedded.
#[test]
fn a_lamina_through_a_trimmed_spline_face_crosses_it() {
    let (verdict, [lying, standing]) = standing_through(0.5);
    match verdict {
        Embedding::Crosses(c) => {
            let mut faces = [c.faces.0, c.faces.1];
            faces.sort();
            let mut want = [lying, standing];
            want.sort();
            assert_eq!(faces, want, "the crossing is between the two laminas");
        }
        other => panic!("the crossing reads {other:?}"),
    }
}

/// **A plane through the chart's unused half meets nothing**: at
/// `x = 1.5` the standing lamina crosses the lying face's chart but not
/// the face, so the pair is embedded, which the walk reads only by
/// dropping the cells of the chart off the face.
#[test]
fn a_lamina_through_the_unused_half_of_a_trimmed_spline_chart_is_embedded() {
    let (verdict, _) = standing_through(1.5);
    assert_eq!(verdict, Embedding::Embedded);
}

/// **Every trimmed spline face's window holds its chart images**: each
/// face's stored images, sampled at 65 parameters over each half-edge's
/// span, lie in its window to `1e-12`. The faces trim the chart at
/// every side of its knot domain: the half-square (`u` from the domain's
/// low end, all of `v`), its mirror on the domain's high half, a strip
/// over all of `u` and half of `v`, and the triangle, whose two isos lie
/// on the domain's edges `v = 0` and `u = 1`. Red where a window is
/// narrower than its face on any side, which leaves part of the face
/// outside the cells the walk reads.
#[test]
fn every_trimmed_spline_window_holds_its_chart_images() {
    let square = |a: f64, b: f64| vec![(a, 0.0), (b, 0.0), (b, 1.0), (a, 1.0)];
    type Fixture = (&'static str, f64, Vec<(f64, f64)>);
    let fixtures: [Fixture; 4] = [
        ("the low half-square", 2.0, square(0.0, 1.0)),
        ("the high half-square", 2.0, square(1.0, 2.0)),
        (
            "the strip",
            1.0,
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 0.5), (0.0, 0.5)],
        ),
        (
            "the triangle",
            1.0,
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)],
        ),
    ];
    let mut read = 0;
    for (what, width, corners) in fixtures {
        let (body, faces) = lamina::<f64>(width, &corners);
        let rects = Side {
            body: &body,
            faces: faces.clone(),
        }
        .windows(Which::First)
        .unwrap_or_else(|e| panic!("{what}: every face has a window: {e:?}"))
        .rects();
        for (face, u, v) in rects {
            for (he, row) in body.pcurves() {
                if body.face_of_half_edge(he) != Some(face) {
                    continue;
                }
                let (t0, t1) = row.params();
                for i in 0..=64 {
                    let p = row.pcurve().eval(t0 + (t1 - t0) * f64::from(i) / 64.0);
                    read += 1;
                    assert!(
                        u.0 <= p.x + 1e-12
                            && p.x <= u.1 + 1e-12
                            && v.0 <= p.y + 1e-12
                            && p.y <= v.1 + 1e-12,
                        "{what}, face {face:?}: the window {u:?} × {v:?} holds the image point \
                         ({}, {})",
                        p.x,
                        p.y
                    );
                }
            }
        }
    }
    assert!(read > 1000, "the image points read: {read}");
}

/// **No cell of a trimmed spline face is dropped holding a face
/// point**: every dyadic cell of the triangle lamina's chart to depth 4
/// that `cell_region` reads `Outside` misses the closed triangle
/// `v ≤ u`, at `f64` and `Interval`, and some are dropped at `f64`.
fn no_outside_cell_holds_the_triangle<T: AtRestPolicy + geom_core::CertifiedBounds>() -> usize {
    use topo::cell_region::{CellRegion, cell_region};
    let (body, faces) = lamina::<T>(1.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
    let mut dropped = 0;
    for depth in 1..=4u32 {
        let n = 1u32 << depth;
        let side = |i: u32| (f64::from(i) / f64::from(n), f64::from(i + 1) / f64::from(n));
        for (i, j) in (0..n).flat_map(|i| (0..n).map(move |j| (i, j))) {
            let (u, v) = (side(i), side(j));
            if cell_region(&body, faces[0], u, v, band()) == CellRegion::Outside {
                dropped += 1;
                // The cell meets `v ≤ u` iff its low v is at most its high u.
                assert!(
                    v.0 > u.1,
                    "{}: {u:?} × {v:?} dropped, holds the face",
                    T::NAME
                );
            }
        }
    }
    dropped
}

#[test]
fn no_cell_of_a_trimmed_spline_face_is_dropped_holding_a_face_point() {
    assert!(
        no_outside_cell_holds_the_triangle::<f64>() > 0,
        "the door drops cells"
    );
    no_outside_cell_holds_the_triangle::<Interval>();
}

/// **An interior cell of a trimmed spline face reads `Inside`**: on the
/// half-square lamina, the cell `[0.25, 0.5]²` lies a quarter clear of
/// the face's boundary, and `cell_region` reads it inside at `f64` and
/// `Interval`. Red where the cell's ball does not narrow with the cell
/// on a spline chart, which keeps every cell undecided.
#[test]
fn an_interior_cell_of_a_trimmed_spline_face_reads_inside() {
    use topo::cell_region::{CellRegion, cell_region};
    fn inside<T: AtRestPolicy + geom_core::CertifiedBounds>() -> CellRegion {
        let (body, faces) = lamina::<T>(2.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        cell_region(&body, faces[0], (0.25, 0.5), (0.25, 0.5), band())
    }
    assert_eq!(inside::<f64>(), CellRegion::Inside, "at f64");
    assert_eq!(inside::<Interval>(), CellRegion::Inside, "at Interval");
}

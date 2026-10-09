//! **Ring re-homing on a cylinder's chart reads a run vertex or a run
//! row on the ray by the half-open rule** ([`super::chart_ring_side`]):
//! a run vertex the ray passes through counts once by the rows on either
//! side of it, a ring vertex on a run row along the ray says nothing,
//! and a window of exactly one period reads a vertex at the seam's
//! azimuth alike on either edge.

use super::sibling_escalation_rows::{band, rim_arc, ring_at, tol};
use super::*;
use crate::euler::{MefSite, MevSite};
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};

fn at(u: f64, v: f64) -> Point3<f64> {
    Point3::new(u.cos(), u.sin(), v)
}

/// The unit wall `[0.2, 1.4] × [0, 1]` holding the island
/// `[0.5, 1] × [0.3, 0.7]` walled off by its closing ruling at 0.5,
/// its high rim split at azimuth 0.75: the run's vertex `(0.75, 0.7)`
/// stands on no ruling. Returns the body, the wall, its surface and the
/// island's face.
fn split_rim_island() -> (Body<f64>, FaceKey, geom::Surface<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.2, 1.4),
        (0.0, 1.0),
        tol(),
    );
    let cyl = body.get_face(face).unwrap().surface;
    let surface = body.get_surface(cyl).unwrap().clone();
    let corners = [
        at(0.5, 0.3),
        at(1.0, 0.3),
        at(1.0, 0.7),
        at(0.75, 0.7),
        at(0.5, 0.7),
    ];
    let island = ring_at(&mut body, face, corners[0], &[]);
    let low = rim_arc(&mut body, cyl, 0.3, (0.5, 1.0));
    let e0 = body
        .mev(MevSite::Lone { r#loop: island }, corners[1], low, tol())
        .unwrap();
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    let e1 = body.mev_line(fan(e0.he_minus), corners[2], tol()).unwrap();
    let high = rim_arc(&mut body, cyl, 0.7, (1.0, 0.75));
    let e2 = body.mev(fan(e1.he_minus), corners[3], high, tol()).unwrap();
    let high = rim_arc(&mut body, cyl, 0.7, (0.75, 0.5));
    let e3 = body.mev(fan(e2.he_minus), corners[4], high, tol()).unwrap();
    let after = body.get_half_edge(e3.he_plus).unwrap().next;
    let made = body
        .mef_chord(
            MefSite::Chords {
                he1: e0.he_plus,
                he2: after,
            },
            tol(),
        )
        .unwrap();
    (body, face, surface, made.face)
}

/// **A ray through a run vertex off every ruling crosses the run there
/// once.** Below the high rim's vertex at azimuth 0.75 the ray from
/// inside the island leaves it through that vertex (In), and from below
/// the island it enters through the low rim and leaves through the
/// vertex (Out); above, it meets nothing (Out). Dropping the rows that
/// end at the ray's azimuth reads the first two the other way round.
#[test]
fn a_ray_through_a_run_vertex_crosses_the_run_there_once() {
    let (mut body, face, surface, run) = split_rim_island();
    for (v, want) in [
        (0.5, RingSide::In),
        (0.2, RingSide::Out),
        (0.9, RingSide::Out),
    ] {
        let ring = ring_at(&mut body, face, at(0.75, v), &[]);
        assert_eq!(
            chart_ring_side(&body, &surface, run, ring, band()).unwrap(),
            want,
            "the ring vertex at (0.75, {v})"
        );
    }
}

/// **A ring vertex on a run row along the ray says nothing; one beyond
/// the row reads past it.** On the island's ruling at 1, on its closing
/// ruling at 0.5, at the run's corner `(1, 0.3)` and at the run vertex
/// `(0.75, 0.7)` the vertex is on the run (`Undecided`); below or above
/// either ruling it is outside.
#[test]
fn a_ring_vertex_on_a_run_row_along_the_ray_says_nothing() {
    let (mut body, face, surface, run) = split_rim_island();
    for (u, v, want) in [
        (1.0, 0.5, RingSide::Undecided),
        (0.5, 0.5, RingSide::Undecided),
        (1.0, 0.3, RingSide::Undecided),
        (0.75, 0.7, RingSide::Undecided),
        (1.0, 0.2, RingSide::Out),
        (1.0, 0.8, RingSide::Out),
        (0.5, 0.2, RingSide::Out),
        (0.5, 0.8, RingSide::Out),
    ] {
        let ring = ring_at(&mut body, face, at(u, v), &[]);
        assert_eq!(
            chart_ring_side(&body, &surface, run, ring, band()).unwrap(),
            want,
            "the ring vertex at ({u}, {v})"
        );
    }
}

/// **A run whose window is one whole period reads a vertex at the
/// seam's azimuth alike on either edge.** The unit wall `[0, τ] × [0, 1]`
/// is its own run, closed along its seam. A vertex at the seam's azimuth,
/// at it or a hair inside the band either side (so it lands at `lo` or
/// at `hi`), reads on the seam (`Undecided`) between the rims and
/// outside below or above them; one a little either side of the seam,
/// and one opposite it, is inside.
#[test]
fn a_full_period_run_reads_the_seam_alike_on_either_edge() {
    let tau = core::f64::consts::TAU;
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.0, tau),
        (0.0, 1.0),
        tol(),
    );
    let cyl = body.get_face(face).unwrap().surface;
    let surface = body.get_surface(cyl).unwrap().clone();
    let hair = 0.5 * band().zero();
    for du in [-hair, 0.0, hair] {
        for (v, want) in [
            (0.5, RingSide::Undecided),
            (-0.5, RingSide::Out),
            (1.5, RingSide::Out),
        ] {
            let ring = ring_at(&mut body, face, at(du, v), &[]);
            assert_eq!(
                chart_ring_side(&body, &surface, face, ring, band()).unwrap(),
                want,
                "the ring vertex at ({du:e}, {v})"
            );
        }
    }
    for u in [1e-3, tau - 1e-3, 0.5 * tau] {
        let ring = ring_at(&mut body, face, at(u, 0.5), &[]);
        assert_eq!(
            chart_ring_side(&body, &surface, face, ring, band()).unwrap(),
            RingSide::In,
            "the ring vertex at ({u}, 0.5)"
        );
    }
}

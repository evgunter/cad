//! **A fillet band whose meridian points fall between its rows** meshes,
//! and every triangle beside them reaches no further than its own lattice
//! and its meridian's chords allow: the curved lane's CDT runs in its
//! lattice's metric (`curved`'s module docs, "The lattice metric").
//!
//! The band is a strip of a thin torus, built by two public doors: a
//! revolve of a plate's section whose corner at the hole's rim is rounded
//! (a whole turn, one face with one slit, the seam; or a partial turn, one
//! face with two meridian edges and no slit), and `sweep`'s fillet of a
//! holed plate's rim (one face with one slit). Its lattice is about
//! `√(R/r)` times taller than wide in UV, so a meridian point between two
//! rows had, in raw UV, an empty circle several columns wide.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};
use std::collections::HashSet;

use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

use crate::common::{filleted_plate, sampled_deviation};

/// How far, relative to its span, a chart coordinate read back from a
/// mesh position (`atan2` of `f64` coordinates) may sit from the lattice
/// line it was laid on: rounding, many orders below any step these bands
/// lay (each above 1e-3 of its span).
const CHART_READ: f64 = 1e-9;

/// The plate section `[hole, 1] × [0, 1]` with its corner at the hole's
/// bottom rim rounded at `r`, revolved about `y` by `turn`: the rounded
/// corner sweeps a band of the torus `(hole + r, r)`.
fn revolved_band(hole: f64, r: f64, turn: Revolution<f64>) -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(hole, r), -(PI / 8.0).tan()),
        (Point2::new(hole + r, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(hole, 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the section validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, turn, Tol::witness())
        .expect("the section revolves")
        .body
        .into_body()
}

/// A point's torus chart coordinates `(u, v)`, each in `(−π, π]`.
fn chart(surface: &Surface<f64>, p: Point3<f64>) -> [f64; 2] {
    let Surface::Torus {
        frame,
        major_radius,
        ..
    } = *surface
    else {
        panic!("the band is a torus strip");
    };
    let w = p - frame.origin();
    let (axis, x) = (frame.axis(), frame.u_ref());
    let y = axis.cross(x);
    let along = w.dot(axis);
    let radial = (w - axis * along).norm();
    [w.dot(y).atan2(w.dot(x)), along.atan2(radial - major_radius)]
}

/// What the band's patch shows of its lattice, its boundary and the
/// triangles beside it.
struct BandReading {
    /// The lattice's rows (interior rows plus one).
    rows: usize,
    /// The fewest chords on one of the band's meridian edges.
    meridian_chords: usize,
    /// Meridian points strictly between two of the lattice's rows.
    off_row_meridian_points: usize,
    /// The largest triangle `u` extent over the column width, and `v`
    /// extent over the row gap.
    reach: [f64; 2],
    /// The largest sampled deviation of a band triangle from the torus.
    deviation: f64,
}

fn distinct(mut xs: Vec<f64>, span: f64) -> Vec<f64> {
    xs.sort_by(f64::total_cmp);
    xs.dedup_by(|a, b| (*a - *b).abs() <= CHART_READ * span);
    xs
}

fn extent(xs: impl Iterator<Item = f64> + Clone) -> f64 {
    xs.clone().fold(f64::NEG_INFINITY, f64::max) - xs.fold(f64::INFINITY, f64::min)
}

/// Reads the band's patch of `mesh`. Its lattice is laid off its interior
/// points (columns and rows strictly inside); its meridians are the
/// boundary polylines on it whose `v` varies, its rims those whose `v`
/// does not.
fn read_band(body: &Body<f64>, mesh: &mesh::Mesh) -> BandReading {
    let patch = mesh
        .patches
        .iter()
        .find(|q| matches!(body.face_surface(q.face), Some(Surface::Torus { .. })))
        .expect("the body has its band");
    let surface = body.face_surface(patch.face).unwrap();
    let ids: HashSet<u32> = patch.triangles.iter().flatten().copied().collect();
    let first = *ids.iter().min().unwrap();
    // `u` and `v` unwrapped about the first point's, so a strip across
    // the chart's `±π` cut reads one window; a whole turn's seam reads at
    // both ends of its turn.
    let [u_ref, v_ref] = chart(surface, mesh.positions[first as usize]);
    let near = |x: f64, about: f64| about + (x - about + PI).rem_euclid(TAU) - PI;
    let at = |i: u32| {
        let [u, v] = chart(surface, mesh.positions[i as usize]);
        [near(u, u_ref), near(v, v_ref)]
    };
    let polylines: Vec<&mesh::BoundaryPolyline> = mesh
        .boundaries
        .iter()
        .filter(|b| b.points.iter().all(|i| ids.contains(i)))
        .collect();
    let on_boundary: HashSet<u32> = polylines.iter().flat_map(|b| b.points.clone()).collect();
    let vs: Vec<f64> = ids.iter().map(|&i| at(i)[1]).collect();
    let vspan = extent(vs.iter().copied());
    let (meridians, rims): (Vec<_>, Vec<_>) = polylines
        .into_iter()
        .partition(|b| extent(b.points.iter().map(|&i| at(i)[1])) > CHART_READ * vspan);
    assert!(
        !meridians.is_empty() && !rims.is_empty(),
        "the band has meridians and rims"
    );
    let interior: Vec<[f64; 2]> = ids
        .iter()
        .filter(|i| !on_boundary.contains(i))
        .map(|&i| at(i))
        .collect();
    let rows = distinct(interior.iter().map(|p| p[1]).collect(), vspan);
    // A whole turn's columns: its interior columns are `nu − 1` of the
    // turn's; a partial turn's are read off its meridians' ends.
    let whole = meridians.len() == 1;
    let uspan = if whole {
        TAU
    } else {
        extent(ids.iter().map(|&i| at(i)[0]))
    };
    let columns = distinct(interior.iter().map(|p| p[0]).collect(), uspan);
    assert!(
        !rows.is_empty() && columns.len() > 2,
        "the band lays a lattice: rows {rows:?}, {} columns",
        columns.len()
    );
    #[allow(clippy::cast_precision_loss)]
    let (gap, width) = (
        vspan / (rows.len() + 1) as f64,
        uspan / (columns.len() + 1) as f64,
    );
    let on =
        |x: f64, lines: &[f64], span: f64| lines.iter().any(|l| (x - l).abs() <= CHART_READ * span);
    let inside = |b: &mesh::BoundaryPolyline| b.points[1..b.points.len() - 1].to_vec();
    let off_row_meridian_points = meridians
        .iter()
        .flat_map(|b| inside(b))
        .filter(|&i| !on(at(i)[1], &rows, vspan))
        .count();
    let meridian_chords = meridians.iter().map(|b| b.points.len() - 1).min().unwrap();
    let mut reach = [0.0f64; 2];
    let mut deviation = 0.0f64;
    for t in &patch.triangles {
        let c = t.map(&at);
        // A whole turn's triangle beside the seam has corners a turn apart.
        let du = [c[1][0] - c[0][0], c[2][0] - c[0][0]].map(|d| (d + PI).rem_euclid(TAU) - PI);
        reach[0] = reach[0].max(extent([0.0, du[0], du[1]].into_iter()) / width);
        reach[1] = reach[1].max(extent(c.iter().map(|p| p[1])) / gap);
        let tri = t.map(|i| mesh.positions[i as usize]);
        deviation = deviation.max(sampled_deviation(surface, tri, 8));
    }
    BandReading {
        rows: rows.len() + 1,
        meridian_chords,
        off_row_meridian_points,
        reach,
        deviation,
    }
}

/// The band's relation at `delta`: it meshes, watertight; every band
/// triangle lies within δ of the torus by sampling, which reads no
/// certificate; a meridian carries a point between two rows (the
/// witness is in the class); and a triangle spans at most one lattice column and at
/// most `max(1, nv/m)` row gaps, `nv` the band's rows and `m` the fewest
/// chords on one of its meridians: a meridian chord spans `nv/m` gaps,
/// and the triangles beside it reach no further (`curved`'s module docs,
/// "What is measured and not proved").
fn assert_band(what: &str, body: &Body<f64>, delta: f64) {
    let mesh = mesh::tessellate(body, delta, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}, δ = {delta}: the band meshes: {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&mesh),
        Ok(()),
        "{what}, δ = {delta}: watertight"
    );
    let band = read_band(body, &mesh);
    assert!(
        band.off_row_meridian_points > 0,
        "{what}, δ = {delta}: a meridian carries a chord point between two rows"
    );
    assert!(
        band.deviation <= delta,
        "{what}, δ = {delta}: a band triangle samples {:e} off the torus",
        band.deviation
    );
    #[allow(clippy::cast_precision_loss)]
    let rows_allowed = (band.rows as f64 / band.meridian_chords as f64).max(1.0);
    let [u, v] = band.reach;
    assert!(
        u <= 1.0 + CHART_READ && v <= rows_allowed * (1.0 + CHART_READ),
        "{what}, δ = {delta}: a band triangle reaches {u} columns and {v} rows; the lattice \
         allows one column and {rows_allowed} rows ({} rows, {} meridian chords)",
        band.rows,
        band.meridian_chords
    );
}

/// **Revolved bands, at the δ their raw-UV CDT refused**: a whole turn
/// (one slit) and two partial turns (two meridian edges, no slit) of a
/// quarter arc of radius 5e-3 on a hole of 0.6. Each meridian, a quarter
/// arc, takes a chord count other than the band's rows
/// (`a-quarter-arcs-chord-count-sits-on-the-capped-steps-ceiling-edge`).
/// The section builds at every ε row.
#[test]
fn a_revolved_band_meshes_with_its_meridian_points_between_rows() {
    for (what, turn) in [
        ("a whole turn", Revolution::Full),
        ("a quarter turn", Revolution::Partial(PI / 2.0)),
        ("two radians", Revolution::Partial(2.0)),
    ] {
        let body = revolved_band(0.6, 5e-3, turn);
        for delta in [3e-3, 1e-3] {
            assert_band(what, &body, delta);
        }
    }
}

/// **The fillet door's band on a hole of three and of five arcs**: the
/// slit takes two chords where the band lays three rows.
#[test]
fn a_filleted_holes_one_slit_band_meshes_with_its_slit_points_between_rows() {
    let three = filleted_plate(0.3, 1e-3, 3);
    for delta in [1e-3, 3e-4] {
        assert_band("three arcs, r = 1e-3", &three, delta);
    }
    let five = filleted_plate(0.3, 5e-3, 5);
    for delta in [3e-3, 1e-3] {
        assert_band("five arcs, r = 5e-3", &five, delta);
    }
}

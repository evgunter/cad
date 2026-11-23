//! **A fillet band that is one ring with one slit** meshes where its
//! slit's chord points fall between the band's rows, and every triangle
//! beside them stays inside one lattice cell: the curved lane's CDT runs
//! in the lattice metric (`curved`'s module docs, "The lattice metric").
//!
//! The band is a torus strip a whole turn round whose meridian side is
//! one seam edge, built by two public doors: a full revolve of a plate's
//! section with its hole's rim rounded, and `sweep`'s fillet of a holed
//! plate's rim. Its lattice is `√(R/r)` times taller than wide in UV, so
//! a slit point between two rows had, in raw UV, an empty circle several
//! columns wide.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{
    ExtrudeSide, Extrusion, Revolution, RevolveAxis, blend::fillet_edges, extrude, revolve,
};
use topo::Body;

use crate::common::sampled_deviation;

/// The plate section `[hole, 1] × [0, 1]` with its corner at the hole's
/// bottom rim rounded at `r`, revolved a full turn about `y`: the rounded
/// corner sweeps a band of the torus `(hole + r, r)`, one face whose
/// slit is the revolve's seam.
fn revolved_band(hole: f64, r: f64) -> Option<Body<f64>> {
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
    match revolve(&vp, axis, Revolution::Full, Tol::witness()) {
        Ok(built) => Some(built.body.into_body()),
        // The thin torus's certification refuses at ε = 1e-12, upstream
        // of the mesh
        // (`work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`).
        Err(e) => {
            let eps = Tol::witness().eps();
            assert!(
                eps < 1e-9 && format!("{e:?}").contains("KernelLimit"),
                "the section revolves at ε = {eps:e}: {e:?}"
            );
            println!(
                "SKIPPED at ε = {eps:e}: the band's revolve refuses KernelLimit upstream of \
                 the mesh, so this run asserts nothing of its mesh"
            );
            None
        }
    }
}

/// The plate `[−1, 1]² × [0, 1]` with a hole of radius `hole` drawn as
/// `arcs` equal arcs, its bottom rim filleted at `r` through
/// `sweep::blend::fillet_edges`: the band is one face with one slit.
fn filleted_plate(hole: f64, r: f64, arcs: usize) -> Body<f64> {
    let square = bulge_loop(vec![
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(-1.0, 1.0), 0.0),
    ]);
    #[allow(clippy::cast_precision_loss)]
    let n = arcs as f64;
    let rim = bulge_loop(
        (0..arcs)
            .map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let a = TAU * k as f64 / n;
                (
                    Point2::new(hole * a.cos(), -hole * a.sin()),
                    -(PI / (2.0 * n)).tan(),
                )
            })
            .collect(),
    );
    let vp = Profile::new(SketchPlane::xy(), vec![square, rim])
        .validate(Tol::witness())
        .expect("the holed plate validates");
    let extrusion = Extrusion::Distance {
        depth: 1.0,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&vp, extrusion, Tol::witness())
        .expect("the plate extrudes")
        .body
        .into_body();
    let bottom_rim: Vec<topo::EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.carrier()),
                Some(geom::Curve3::Circle(c)) if c.frame.origin().z == 0.0
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(bottom_rim.len(), arcs, "the bottom rim is the hole's arcs");
    let at_rest = sweep::test_support::at_rest(&body, Tol::witness());
    fillet_edges(&at_rest, &bottom_rim, r, Tol::witness())
        .expect("the rim fillets")
        .body
        .into_body()
}

/// A band triangle's corners in the torus chart, `u` unwrapped about the
/// first corner's.
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

/// What the band's patch shows of its lattice and of the triangles
/// beside its boundary.
struct BandReading {
    /// Boundary points on the slit strictly between two of the lattice's
    /// rows.
    off_row_slit_points: usize,
    /// Rim points strictly between two of the lattice's columns.
    off_column_rim_points: usize,
    /// The largest triangle `u` extent over the column width, and `v`
    /// extent over the row gap.
    reach: [f64; 2],
    /// The largest sampled deviation of a band triangle from the torus.
    deviation: f64,
}

fn distinct(mut xs: Vec<f64>, span: f64) -> Vec<f64> {
    xs.sort_by(f64::total_cmp);
    xs.dedup_by(|a, b| (*a - *b).abs() <= 1e-9 * span);
    xs
}

/// Reads the band's patch of `mesh`: its lattice is laid off its
/// interior points (columns and rows strictly inside), its boundary off
/// the rest.
fn read_band(body: &Body<f64>, mesh: &mesh::Mesh) -> BandReading {
    let patch = mesh
        .patches
        .iter()
        .find(|q| matches!(body.face_surface(q.face), Some(Surface::Torus { .. })))
        .expect("the body has its band");
    let surface = body.face_surface(patch.face).unwrap();
    let on_boundary: std::collections::HashSet<u32> = mesh
        .boundaries
        .iter()
        .flat_map(|b| b.points.iter().copied())
        .collect();
    let mut ids: Vec<u32> = patch.triangles.iter().flatten().copied().collect();
    ids.sort_unstable();
    ids.dedup();
    // `u` on `[0, 2π)`; `v` unwrapped about the first point's, so a strip
    // across the chart's `±π` cut reads one window.
    let v_ref = chart(surface, mesh.positions[ids[0] as usize])[1];
    let at = |i: u32| {
        let [u, v] = chart(surface, mesh.positions[i as usize]);
        let u = u.rem_euclid(TAU);
        [
            if TAU - u <= 1e-9 { 0.0 } else { u },
            v_ref + (v - v_ref + PI).rem_euclid(TAU) - PI,
        ]
    };
    let vs: Vec<f64> = ids.iter().map(|&i| at(i)[1]).collect();
    let (v0, v1) = (
        vs.iter().copied().fold(f64::INFINITY, f64::min),
        vs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    );
    let vspan = v1 - v0;
    let interior: Vec<[f64; 2]> = ids
        .iter()
        .filter(|i| !on_boundary.contains(i))
        .map(|&i| at(i))
        .collect();
    let rows = distinct(interior.iter().map(|p| p[1]).collect(), vspan);
    let columns = distinct(interior.iter().map(|p| p[0]).collect(), TAU);
    assert!(
        !rows.is_empty() && columns.len() > 2,
        "the band lays a lattice: rows {rows:?}, {} columns",
        columns.len()
    );
    #[allow(clippy::cast_precision_loss)]
    let (gap, width) = (
        vspan / (rows.len() + 1) as f64,
        TAU / (columns.len() + 1) as f64,
    );
    let near = |x: f64, set: &[f64], scale: f64| set.iter().any(|s| (x - s).abs() <= 1e-9 * scale);
    let mut off_row_slit_points = 0;
    let mut off_column_rim_points = 0;
    for &i in ids.iter().filter(|i| on_boundary.contains(i)) {
        let [u, v] = at(i);
        let inside_v = v > v0 + 1e-9 * vspan && v < v1 - 1e-9 * vspan;
        if inside_v && u.abs() <= 1e-9 && !near(v, &rows, vspan) {
            off_row_slit_points += 1;
        }
        if !inside_v && !near(u, &columns, TAU) && u.abs() > 1e-9 {
            off_column_rim_points += 1;
        }
    }
    let mut reach = [0.0f64; 2];
    let mut deviation = 0.0f64;
    for t in &patch.triangles {
        let c = t.map(&at);
        let du = [c[1][0] - c[0][0], c[2][0] - c[0][0]].map(|d| (d + PI).rem_euclid(TAU) - PI);
        let us = [0.0, du[0], du[1]];
        let extent = |xs: [f64; 3]| {
            xs.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                - xs.iter().copied().fold(f64::INFINITY, f64::min)
        };
        reach[0] = reach[0].max(extent(us) / width);
        reach[1] = reach[1].max(extent([c[0][1], c[1][1], c[2][1]]) / gap);
        let tri = t.map(|i| mesh.positions[i as usize]);
        deviation = deviation.max(sampled_deviation(surface, tri, 8));
    }
    BandReading {
        off_row_slit_points,
        off_column_rim_points,
        reach,
        deviation,
    }
}

/// The band's relation at `delta`: it meshes; every band triangle lies
/// within δ of the torus by sampling, which reads no certificate; and a
/// triangle reaches at most one lattice column and one and a half rows
/// (the containment `curved`'s "The lattice metric" states as measured).
/// `slit` and `rim` name the off-lattice points the witness must carry.
fn assert_band(what: &str, body: &Body<f64>, delta: f64, slit: bool, rim: bool) {
    let mesh = mesh::tessellate(body, delta, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}, δ = {delta}: the band meshes: {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&mesh),
        Ok(()),
        "{what}, δ = {delta}: watertight"
    );
    let band = read_band(body, &mesh);
    assert!(
        !slit || band.off_row_slit_points > 0,
        "{what}, δ = {delta}: the slit carries a chord point between two rows"
    );
    assert!(
        !rim || band.off_column_rim_points > 0,
        "{what}, δ = {delta}: a rim carries a chord point between two columns"
    );
    assert!(
        band.deviation <= delta,
        "{what}, δ = {delta}: a band triangle samples {:e} off the torus",
        band.deviation
    );
    let [u, v] = band.reach;
    assert!(
        u <= 1.0 + 1e-9 && v <= 1.5 + 1e-9,
        "{what}, δ = {delta}: a band triangle reaches {u} columns and {v} rows"
    );
}

/// **The revolved band, at the three δ its raw-UV CDT refused**: the
/// slit, one quarter arc of radius 5e-4, takes three chords where the
/// band lays two rows (`a-quarter-arcs-chord-count-sits-on-the-capped-steps-ceiling-edge`),
/// and the triangles beside its points stay in one column.
#[test]
fn a_revolved_one_slit_band_meshes_with_its_slit_points_between_rows() {
    let Some(body) = revolved_band(0.3, 5e-4) else {
        return;
    };
    for delta in [1e-3, 3e-4, 1e-4] {
        assert_band("the revolved band", &body, delta, true, false);
    }
}

/// **The fillet door's band on a hole of three and of five arcs**: the
/// slit takes two chords where the band lays three rows.
#[test]
fn a_filleted_holes_one_slit_band_meshes_with_its_slit_points_between_rows() {
    let three = filleted_plate(0.3, 1e-3, 3);
    for delta in [1e-3, 3e-4] {
        assert_band("three arcs, r = 1e-3", &three, delta, true, false);
    }
    let five = filleted_plate(0.3, 5e-3, 5);
    for delta in [3e-3, 1e-3] {
        assert_band("five arcs, r = 5e-3", &five, delta, true, false);
    }
}

/// **An off-lattice point by another source than the slit's count**: a
/// hole of two arcs makes the band's foot two edges, each chorded on its
/// own, so the foot holds more chords than the band has columns. On the
/// band's short axis, and it stays in one cell.
#[test]
fn a_band_whose_foot_is_two_arcs_meshes_with_its_rim_points_between_columns() {
    let body = filleted_plate(0.3, 1e-3, 2);
    for delta in [3e-3, 1e-3] {
        assert_band("two arcs, r = 1e-3", &body, delta, false, true);
    }
}

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

/// **The filleted holes' meshes are one value at every ε row**: their
/// bytes at the δ the rows above read, against one pinned digest.
/// Nothing that chooses the mesh reads ε (`[mesh.winding]`,
/// `[tolerance.chordal]`).
#[test]
fn the_filleted_holes_meshes_are_byte_identical_at_every_eps() {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (body, delta) in [
        (filleted_plate(0.3, 1e-3, 3), 1e-3),
        (filleted_plate(0.3, 1e-3, 3), 3e-4),
        (filleted_plate(0.3, 5e-3, 5), 1e-3),
        (filleted_plate(0.3, 1e-3, 2), 3e-3),
    ] {
        let m = mesh::tessellate(&body, delta, Tol::witness()).expect("the band meshes");
        for p in &m.positions {
            for x in [p.x, p.y, p.z] {
                fnv(&mut h, x.to_bits());
            }
        }
        for q in &m.patches {
            for t in &q.triangles {
                for &i in t {
                    fnv(&mut h, u64::from(i));
                }
            }
        }
    }
    assert_eq!(
        h,
        0x527e_a70e_d171_1f85,
        "the filleted holes' bytes at ε = {:e}",
        Tol::witness().eps()
    );
}

/// The sphere zone between latitudes `lo` and `hi` (degrees) of the
/// unit sphere, revolved a full turn about `y`.
fn zone(lo: f64, hi: f64) -> Body<f64> {
    let (a, b) = (lo.to_radians(), hi.to_radians());
    let lp = bulge_loop(vec![
        (Point2::new(a.cos(), a.sin()), ((b - a) / 4.0).tan()),
        (Point2::new(b.cos(), b.sin()), 0.0),
        (Point2::new(0.0, b.sin()), 0.0),
        (Point2::new(0.0, a.sin()), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the zone's section validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .expect("the section revolves")
        .body
        .into_body()
}

/// **A curved sphere face off the equator, in its metric**: the zone at
/// 50°–80° has lattice metric 2 (`1/cos 65°`, rounded to a power of
/// two), and every triangle lies within δ of the sphere by sampling.
#[test]
fn an_off_equator_sphere_zone_meshes_within_delta_in_its_metric() {
    let body = zone(50.0, 80.0);
    for delta in [1e-2, 1e-3] {
        let m = mesh::tessellate(&body, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("δ = {delta}: the zone meshes: {e:?}"));
        let mut spheres = 0;
        for q in &m.patches {
            let surface = body.face_surface(q.face).unwrap();
            if !matches!(surface, Surface::Sphere { .. }) {
                continue;
            }
            spheres += 1;
            for t in &q.triangles {
                let d = sampled_deviation(surface, t.map(|i| m.positions[i as usize]), 8);
                assert!(
                    d <= delta,
                    "δ = {delta}: a sphere triangle samples {d:e} off"
                );
            }
        }
        assert!(spheres > 0, "δ = {delta}: the zone has its sphere faces");
    }
}

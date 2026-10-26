//! **A plane across a one-face closed wall.** A transverse plane cuts
//! such a wall — a one-segment circle's extruded cylinder, a full
//! revolve's tube — in a conic that crosses the wall's one wrap edge
//! once, so the section loop has one site and its one segment runs
//! from that site round the whole conic back to it. Every row runs ∪,
//! ∩ and both differences, each in both operand orders
//! (`differential::every_op_both_orders`), so the planar face is the
//! first operand in half the runs and the second in the rest. Each run
//! is read through `differential::outcome` (tiers 2 and 3′, the
//! certificate, the legal operand, the closed-form volume) and its
//! result meshed and `check_mesh`ed.
//!
//! - the CLEAVE tube: a full-revolved tube under a box;
//! - a slab and a blind pocket across a one-segment cylinder's wall;
//! - a tilted plane across that wall;
//! - a tilted plane passing close by the wrap edge's end vertex;
//! - the tube under a box whose side face cuts its outer wall, so one
//!   one-site loop lies beside matched segments in the same faces.
//!
//! A box less a tube leaves a rod in the bore, two solids the census
//! clears on the rod's depth about the bore's axis. A piece of the wall
//! under a tilted plane is a legal operand: a ray from the far brick
//! that meets nothing reads the side at infinity off a ray aimed at the
//! piece.
//!
//! The one-site loop the join does not build keeps
//! `Join(SingleSiteSectionLoop)`: a plane tangent to the wall's cap at
//! the wrap edge's end. A circle edge lying in the partner's face beside
//! a transverse loop builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{Body, BooleanError, SplitJoinError};

use crate::common::differential::{every_op_both_orders, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle of radius `r` about `(cx, cy)`, its one vertex
/// at azimuth `az`, turning `sweep`.
fn circle(cx: f64, cy: f64, r: f64, az: f64, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r * az.cos(), cy + r * az.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep,
        }),
    )])
}

/// `loops` extruded from `z = z0` up by `h`.
fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol()), tol()).unwrap()
}

/// The one-segment circle of radius 1 about the z axis, its vertex at
/// `(1, 0)`, extruded over `z ∈ [z0, z0 + 2]`: one wall whose seam strut
/// is its wrap edge, on the meridian through `(1, 0, z)`. `2π`.
fn seam_cylinder(z0: f64) -> Body<f64> {
    extruded(vec![circle(0.0, 0.0, 1.0, 0.0, TAU)], z0, 2.0)
}

/// The tube `r ∈ [ri, 0.5]`, `y ∈ [−0.5, 1.5]`, revolved a full turn
/// about `y`: two walls, each with its one seam.
fn tube(ri: f64) -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved_about_y(
        vec![p(ri, -0.5), p(0.5, -0.5), p(0.5, 1.5), p(ri, 1.5)],
        Revolution::Full,
        tol(),
    )
}

/// The half-space below the plane through `at` with upward normal
/// `R·ẑ`, as a box 8 wide and 8 deep under it: every row's body lies
/// well inside its other faces.
fn below(at: Point3<f64>, turn: Affine3<f64>) -> Body<f64> {
    let raw = brick((-4.0, 4.0), (-4.0, 4.0), (-8.0, 0.0), tol());
    let to = Affine3::translation(at - Point3::origin());
    topo::transform_rigid(
        &raw,
        &geom_core::test_support::rigid(&(to * turn), tol()),
        tol(),
    )
    .unwrap()
}

/// Every op in both orders between `a` and `b` (volumes `va`, `vb`,
/// overlapping in `vab`), each read through `outcome`: `SOUND`; and meshed,
/// unless the result holds a window in a curved face, which the mesher
/// refuses typed (`TessellateError::RingOnCurvedFace`,
/// `work/tess/a-notched-or-ringed-cylinder-wall-does-not-tessellate.md`).
fn every_op(what: &str, a: Body<f64>, b: Body<f64>, v: (f64, f64, f64)) {
    let (a, b) = (fin("A", a), fin("B", b));
    for (op, r, want) in every_op_both_orders(&a, &b, v, tol()) {
        let row = format!("{what}: {op}");
        let res = r.as_ref().unwrap_or_else(|e| panic!("{row}: {e:?}"));
        let body = &res.body().unwrap_or_else(|| panic!("{row}: empty")).body;
        match mesh::tessellate(body, 5e-3, tol()) {
            Ok(m) => mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{row}: mesh {e:?}")),
            Err(mesh::TessellateError::RingOnCurvedFace { .. }) => {}
            Err(e) => panic!("{row}: {e:?}"),
        }
        // The volume against the closed form within the quadrature's
        // own certified pad, which grows with ε; `outcome`'s fixed 1e-7
        // holds at the default ε only.
        let m = topo::mass_properties(body, tol(), topo::test_support::pinned_resolution(tol()))
            .unwrap();
        assert!(
            (m.enclosure.volume - want).abs() <= m.enclosure.volume_pad + 1e-9 * want.max(1.0),
            "{row}: volume {} ± {}, closed form {want}",
            m.enclosure.volume,
            m.enclosure.volume_pad
        );
        // `operand=` reads only whether the union builds. A legal
        // operand's union with the far brick also holds the brick's
        // volume.
        let far = fin(
            "the far brick",
            brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()),
        );
        let joined = topo::union(body, &far, tol())
            .unwrap_or_else(|e| panic!("{row}: ∪ a far brick: {e:?}"));
        let j = topo::mass_properties(
            &joined.body().expect("not empty").body,
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .unwrap();
        assert!(
            (j.enclosure.volume - (m.enclosure.volume + 1.0)).abs()
                <= j.enclosure.volume_pad + m.enclosure.volume_pad + 1e-9,
            "{row}: ∪ a far brick: volume {} vs {}",
            j.enclosure.volume,
            m.enclosure.volume + 1.0
        );
        let line = outcome(r, m.enclosure.volume, tol());
        for f in ["t2=true", "t3p=true", "cert=true", "operand=true"] {
            assert!(line.contains(f), "{row}: want {f}: {line}");
        }
    }
}

fn fin(what: &str, body: Body<f64>) -> topo::AtRestBody<f64> {
    topo::test_support::finished(what, body, tol())
}

/// **The CLEAVE tube**: the box `(−1.5, 1.5) × (0, 1) × (−1.5, 1.5)`
/// and the tube `r ∈ [0.3, 0.5]`. The box's faces `y = 0` and `y = 1`
/// cut each tube wall across its one seam: four one-site loops. Tube
/// `0.32π`, box 9, overlap `0.16π`.
#[test]
fn a_box_across_a_full_revolved_tube() {
    let block = brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol());
    every_op(
        "the tube under a box",
        tube(0.3),
        block,
        (0.32 * PI, 9.0, 0.16 * PI),
    );
}

/// **A slab across the one-segment cylinder**: `z ∈ [0.5, 1]` over the
/// whole disc, two planes each cutting the wall in a circle through its
/// seam once. Slab 18, overlap `π/2`.
#[test]
fn a_slab_across_a_one_segment_cylinder() {
    let slab = brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol());
    every_op("the slab", seam_cylinder(0.0), slab, (TAU, 18.0, 0.5 * PI));
}

/// **A blind pocket**: the one-segment cylinder raised to
/// `z ∈ [0.5, 2.5]` sunk into the plate `z ∈ [0, 1]`, whose top face
/// cuts the cylinder's wall across its seam; the pocket is the plate
/// less the cylinder. Plate 36, overlap `π/2`.
#[test]
fn a_blind_pocket_through_a_plates_top_face() {
    let plate = brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol());
    every_op(
        "the pocket",
        plate,
        seam_cylinder(0.5),
        (36.0, TAU, 0.5 * PI),
    );
}

/// **A tilted plane**: `z = 1 + y·tan 20°`, turned about `x`, an ellipse
/// through the seam at `z = 1`. Below it the disc holds
/// `∫(1 + y·tan 20°) dA = π`; the box is 512.
#[test]
fn a_tilted_plane_across_a_one_segment_cylinder() {
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        20f64.to_radians(),
    );
    let cutter = below(Point3::new(0.0, 0.0, 1.0), turn);
    every_op(
        "the tilted plane",
        seam_cylinder(0.0),
        cutter,
        (TAU, 512.0, PI),
    );
}

/// **A far point's side on the tilted plane's wall piece is read by a
/// ray aimed at the piece**: every ray from `(50, 50, 50)` misses the
/// piece, so the side is the side at infinity, and a ray aimed at one of
/// the piece's faces reads it: the wall is read from its boundary edges
/// (`topo::boolean::ruled_region`), so a ray crossing it is answered and
/// the volume's sign is never asked. The enclosure that sign is
/// certified on (`bool_point_in_solid_infinity_enclosure`) is then not
/// read at all.
#[test]
fn a_far_points_side_on_the_wall_piece_is_read_by_an_aimed_ray() {
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        20f64.to_radians(),
    );
    let (a, b) = (
        fin("A", seam_cylinder(0.0)),
        fin("B", below(Point3::new(0.0, 0.0, 1.0), turn)),
    );
    let piece = topo::intersect(&a, &b, tol()).expect("A ∩ B builds");
    let piece = &piece.body().expect("not empty").body;
    let band = geom_core::Band::linear(tol()).expect("the run's band");
    let bracket = geom_core::k_stats::Bracket::open();
    let side = topo::point_in_solid(piece, Point3::new(50.0, 50.0, 50.0), band, tol());
    let recorded = bracket.finish();
    assert_eq!(
        side,
        Ok(topo::SolidContainment::Out),
        "the far point's side"
    );
    let certified = recorded
        .verdicts
        .iter()
        .filter(|v| v.predicate == "bool_point_in_solid_infinity_enclosure")
        .map(|v| v.sign)
        .collect::<Vec<_>>();
    assert_eq!(
        certified,
        [],
        "an aimed ray reads the side, and the enclosure is not asked: {recorded:?}"
    );
}

/// **A plane close by the wrap edge's end**: `z = c + m(1 − x)`, turned
/// about `y`, meets the seam at `z = c`, just above the bottom cap's
/// one vertex `(1, 0, 0)`, and rises to `c + 2m` across the disc. Below
/// it the disc holds `π(c + m)`.
#[test]
fn a_plane_close_by_the_wrap_edges_end() {
    let m: f64 = 0.5;
    for c in [0.02, 1e-3] {
        let turn =
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), m.atan());
        every_op(
            &format!("the plane {c} above the end"),
            seam_cylinder(0.0),
            below(Point3::new(1.0, 0.0, c), turn),
            (TAU, 512.0, PI * (c + m)),
        );
    }
}

/// **A one-site loop beside matched ones**: the CLEAVE tube under the box
/// `(−1.5, 0.4) × (0, 1) × (−1.5, 1.5)`. Its face `x = 0.4` misses the
/// inner wall and cuts the outer one along two rulings, so in the faces
/// `y = 0` and `y = 1` the inner wall's whole circle, through its seam,
/// lies beside the outer wall's arc, whose ends are on the box's edges.
/// Box 5.7; overlap the annulus `r ∈ [0.3, 0.5]` less the outer disc's
/// segment beyond `x = 0.4`, `0.25·acos 0.8 − 0.4·0.3`.
#[test]
fn a_one_site_loop_beside_matched_ones() {
    let block = brick((-1.5, 0.4), (0.0, 1.0), (-1.5, 1.5), tol());
    let segment = 0.25 * 0.8f64.acos() - 0.4 * 0.3;
    every_op(
        "the tube beside the box's edge",
        tube(0.3),
        block,
        (0.32 * PI, 5.7, 0.16 * PI - segment),
    );
}

/// Every op in both orders refuses `Join(SingleSiteSectionLoop { count })`.
fn every_op_refuses(what: &str, a: Body<f64>, b: Body<f64>, count: usize) {
    let (a, b) = (fin("A", a), fin("B", b));
    for (op, r, _) in every_op_both_orders(&a, &b, (0.0, 0.0, 0.0), tol()) {
        assert!(
            matches!(
                r,
                Err(BooleanError::Join(ref j))
                    if matches!(j.error(), SplitJoinError::SingleSiteSectionLoop { count: c } if *c == count)
            ),
            "{what}: {op}: {:?}",
            r.map(|_| "a body")
        );
    }
}

/// **A circle edge lying in the partner's face joins beside a
/// transverse loop.** A bored tube whose outer wall turns from the
/// cylinder `ρ = 0.5` (`y ≤ 0`) to a sphere at the circle `y = 0`, an
/// edge whose one vertex lies in the box face `y = 0`: its section loop
/// is that closed edge, one site
/// (`a_closed_conic_in_the_partners_face.rs`). The inner wall `ρ = 0.3`
/// crosses the same face across its seam. Either side of the face: the
/// box above holds the dome, the one below the cylinder's piece.
#[test]
fn a_circle_edge_in_the_partners_face_builds() {
    // The sphere about (0, −0.2) through (0.5, 0) meets the bore ρ = 0.3
    // at y = top.
    let top = (0.29f64 - 0.09).sqrt() - 0.2;
    let (a0, a1) = (0.2f64.atan2(0.5), (top + 0.2).atan2(0.3));
    let tube = revolved_about_y(
        vec![
            (Point2::new(0.3, -0.5), 0.0),
            (Point2::new(0.5, -0.5), 0.0),
            (Point2::new(0.5, 0.0), ((a1 - a0) / 4.0).tan()),
            (Point2::new(0.3, top), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    // π∫₀^top (0.29 − (y + 0.2)² − 0.09) dy.
    let dome = PI * (0.2 * top - ((top + 0.2).powi(3) - 0.008) / 3.0);
    let shank = PI * (0.25 - 0.09) * 0.5;
    for (y, shared) in [((0.0, 2.0), dome), ((-2.0, 0.0), shank)] {
        let block = brick((-1.5, 1.5), y, (-1.5, 1.5), tol());
        every_op(
            &format!("the in-face circle, box y = {y:?}"),
            tube.clone(),
            block,
            (dome + shank, 18.0, shared),
        );
    }
}

/// **A plane tangent to the cap at the wrap edge's end keeps the
/// refusal**: `z = m(1 − x)` touches the bottom rim at its one vertex
/// `(1, 0, 0)`, so its section's one site is that vertex, where the
/// rim, not the wrap edge alone, meets the plane.
#[test]
fn a_plane_tangent_at_the_wrap_edges_end_refuses() {
    let m: f64 = 0.5;
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), m.atan());
    every_op_refuses(
        "the plane tangent at the end vertex",
        seam_cylinder(0.0),
        below(Point3::new(1.0, 0.0, 0.0), turn),
        1,
    );
}

/// **A fully revolved tube answers `point_in_solid` at every point of a
/// grid round it.** The profile `(0.25, 0) → (0.5, 0) → (0.5, 1) →
/// (0.25, 1)` revolved a full turn about `y`: each wall is one face
/// whose region is read from its rims, so no wall reads a window or a
/// period. Inside exactly where `0.25 < ρ < 0.5` and `0 < y < 1`, over
/// a 9³ grid on `[−0.6, 0.6] × [−0.1, 1.1] × [−0.6, 0.6]`, none of whose
/// points lies on the tube's boundary.
#[test]
fn a_full_revolved_tube_answers_every_grid_point() {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let tube = revolved_about_y(
        vec![p(0.25, 0.0), p(0.5, 0.0), p(0.5, 1.0), p(0.25, 1.0)],
        Revolution::Full,
        tol(),
    );
    let band = geom_core::Band::linear(tol()).expect("the run's band");
    let mut wrong = Vec::new();
    for (i, j, k) in (0..9).flat_map(|i| (0..9).flat_map(move |j| (0..9).map(move |k| (i, j, k)))) {
        let q = Point3::new(
            -0.6 + 0.15 * f64::from(i),
            -0.1 + 0.15 * f64::from(j),
            -0.6 + 0.15 * f64::from(k),
        );
        let rho = q.x.hypot(q.z);
        let inside = rho > 0.25 && rho < 0.5 && q.y > 0.0 && q.y < 1.0;
        let want = if inside {
            topo::SolidContainment::In
        } else {
            topo::SolidContainment::Out
        };
        match topo::point_in_solid(&tube, q, band, tol()) {
            Ok(got) if got == want => {}
            got => wrong.push(format!("{q:?}: {got:?}, want {want:?}")),
        }
    }
    assert!(
        wrong.is_empty(),
        "{} of 729:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

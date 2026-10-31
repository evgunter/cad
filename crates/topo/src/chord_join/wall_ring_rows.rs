//! **Ring re-homing on a cylinder wall reads a path's parity across the
//! run** ([`super::path_ring_side`]): a ring vertex beside a run vertex,
//! along a ruling of the run or on a run arc's plane reads its side, one
//! on the run says nothing, a wall closed along its seam reads a vertex on the seam as
//! any other, and an island bounded by spline edges cut from a cylinder
//! × sphere section re-homes its bystanders as membership in the ball
//! says.

test_utils::gated_to![
    "crates/topo/src/chord_join.rs",
    "crates/topo/src/ring_path.rs",
    "crates/geom-brep/src/curve_pair.rs",
    "crates/geom-brep/src/curve_pair/",
    "crates/geom-brep/src/pair_section.rs",
];

use super::sibling_escalation_rows::{band, rim_arc, ring_at, tol};
use super::*;
use crate::euler::{MefSite, MevSite};
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
use test_utils::fuzz::{self, Rng};

fn at(u: f64, v: f64) -> Point3<f64> {
    Point3::new(u.cos(), u.sin(), v)
}

/// [`path_ring_side`] of `ring` against `newf`'s outer, walled off
/// `oldf`, on `oldf`'s surface.
pub(super) fn wall_side(
    body: &Body<f64>,
    (oldf, newf): (FaceKey, FaceKey),
    ring: LoopKey,
) -> Result<RingSide, SplitJoinError> {
    let surface = body
        .get_face(oldf)
        .and_then(|f| body.get_surface(f.surface))
        .unwrap()
        .clone();
    let quadric = Quadric::of(&surface).unwrap();
    path_ring_side(body, (&surface, &quadric), (oldf, newf), ring, band())
}

/// The unit wall `[u0, u1] × [0, 1]` holding the island
/// `[0.5, 1] × [0.3, 0.7]` walled off by its closing ruling at 0.5,
/// its high rim split at azimuth 0.75: the run's vertex `(0.75, 0.7)`
/// stands on no ruling. Returns the body, the wall and the island's
/// face.
fn split_rim_island((u0, u1): (f64, f64)) -> (Body<f64>, FaceKey, FaceKey) {
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        (u0, u1),
        (0.0, 1.0),
        tol(),
    );
    let cyl = body.get_face(face).unwrap().surface;
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
    (body, face, made.face)
}

/// **A ring vertex at a run vertex's azimuth off every ruling reads its
/// side.** Below the high rim's vertex at azimuth 0.75, inside the
/// island, it is In; below the island and above it, Out.
#[test]
fn a_vertex_at_a_run_vertexs_azimuth_reads_its_side() {
    let (mut body, face, run) = split_rim_island((0.2, 1.4));
    for (v, want) in [
        (0.5, RingSide::In),
        (0.2, RingSide::Out),
        (0.9, RingSide::Out),
    ] {
        let ring = ring_at(&mut body, face, at(0.75, v), &[]);
        assert_eq!(
            wall_side(&body, (face, run), ring).unwrap(),
            want,
            "the ring vertex at (0.75, {v})"
        );
    }
}

/// **A ring vertex on the run says nothing; one on a run ruling's line
/// beyond the run reads outside.** On the island's ruling at 1, on its
/// closing ruling at 0.5, at the run's corner `(1, 0.3)` and at the run
/// vertex `(0.75, 0.7)` the vertex is on the run (`Undecided`); below
/// or above either ruling it is outside.
#[test]
fn a_ring_vertex_on_the_run_says_nothing() {
    let (mut body, face, run) = split_rim_island((0.2, 1.4));
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
            wall_side(&body, (face, run), ring).unwrap(),
            want,
            "the ring vertex at ({u}, {v})"
        );
    }
}

/// The slope of [`tilted_island`]'s rims: their planes are tilted
/// `atan(TILT)` about `y`, so each cuts the unit cylinder in an ellipse
/// whose semi-axes differ by more than the island is wide.
const TILT: f64 = 2.5;

/// The height at azimuth `u` of the ellipse the plane through
/// `(0, 0, h)` tilted by [`TILT`] cuts the unit cylinder in.
fn tilted(h: f64, u: f64) -> f64 {
    h + TILT * u.cos()
}

/// The arc of [`tilted`]`(h, ·)` from azimuth `u0` to `u1` (either way
/// round), its parameter the azimuth, described as the cylinder cut by
/// its plane.
fn tilted_arc(
    body: &mut Body<f64>,
    cyl: SurfaceKey,
    h: f64,
    (u0, u1): (f64, f64),
) -> EdgeCurveSpec<f64> {
    let (ellipse, normal) = crate::ring_path::tilted_section(h, TILT);
    let plane = body.add_surface(geom::Surface::Plane {
        origin: Point3::new(0.0, 0.0, h),
        normal,
        u_ref: Vec3::unit_y(),
    });
    let (carrier, t0, t1) = if u1 > u0 {
        (ellipse, u0, u1)
    } else {
        (ellipse.reversed().unwrap(), -u0, -u1)
    };
    EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(cyl, plane),
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
        },
        carrier,
        param_start: t0,
        param_end: t1,
    }
}

/// The unit wall `[0.2, 1.4] × [0, 4]` holding the island between the
/// rulings at azimuths 0.5 and 1 and the tilted rims
/// [`tilted`]`(0.3, ·)` and [`tilted`]`(0.7, ·)`, walled off by its
/// closing ruling at 0.5. Returns the body, the wall and the island's
/// face.
fn tilted_island() -> (Body<f64>, FaceKey, FaceKey) {
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        (0.2, 1.4),
        (0.0, 4.0),
        tol(),
    );
    let cyl = body.get_face(face).unwrap().surface;
    let corners = [
        at(0.5, tilted(0.3, 0.5)),
        at(1.0, tilted(0.3, 1.0)),
        at(1.0, tilted(0.7, 1.0)),
        at(0.5, tilted(0.7, 0.5)),
    ];
    let island = ring_at(&mut body, face, corners[0], &[]);
    let low = tilted_arc(&mut body, cyl, 0.3, (0.5, 1.0));
    let e0 = body
        .mev(MevSite::Lone { r#loop: island }, corners[1], low, tol())
        .unwrap();
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    let e1 = body.mev_line(fan(e0.he_minus), corners[2], tol()).unwrap();
    let high = tilted_arc(&mut body, cyl, 0.7, (1.0, 0.5));
    let e2 = body.mev(fan(e1.he_minus), corners[3], high, tol()).unwrap();
    let after = body.get_half_edge(e2.he_plus).unwrap().next;
    let made = body
        .mef_chord(
            MefSite::Chords {
                he1: e0.he_plus,
                he2: after,
            },
            tol(),
        )
        .unwrap();
    (body, face, made.face)
}

/// **A ring vertex on a run arc's plane, off the arc, reads its side.**
/// On each tilted rim's plane either side of the island, the vertex is
/// outside, every path from it starting on that plane out of the rim's
/// span; between the rims, inside the island, it is In.
#[test]
fn a_vertex_on_a_run_arcs_plane_off_the_arc_reads_its_side() {
    let (mut body, face, run) = tilted_island();
    for (u, h) in [(1.2, 0.7), (0.3, 0.7), (1.2, 0.3), (0.3, 0.3)] {
        let ring = ring_at(&mut body, face, at(u, tilted(h, u)), &[]);
        assert_eq!(
            wall_side(&body, (face, run), ring).unwrap(),
            RingSide::Out,
            "the ring vertex at azimuth {u} on the plane through height {h}"
        );
    }
    let ring = ring_at(&mut body, face, at(0.75, tilted(0.5, 0.75)), &[]);
    assert_eq!(
        wall_side(&body, (face, run), ring).unwrap(),
        RingSide::In,
        "the ring vertex inside the island"
    );
}

/// **A run along the seam twice reads through it.** A full-turn sheet's
/// whole boundary is the run (both rims, the seam once each way),
/// re-homed against a taller full-turn sheet's references: a vertex on
/// the seam, or a hair inside the band either side of it, is on the run
/// between the rims and outside below or above them; one a little
/// either side of the seam, or opposite it, is inside, the paths
/// crossing the seam's two arcs alike.
#[test]
fn a_run_along_the_seam_twice_reads_through_it() {
    let tau = core::f64::consts::TAU;
    let mut body = Body::<f64>::new();
    let frame = CylFrame::canonical(1.0);
    let oldf = cyl_wall_sheet(&mut body, frame, (0.0, tau), (-3.0, 2.0), tol());
    let newf = cyl_wall_sheet(&mut body, frame, (0.0, tau), (0.0, 1.0), tol());
    let hair = 0.5 * band().zero();
    for du in [-hair, 0.0, hair] {
        for (v, want) in [
            (0.5, RingSide::Undecided),
            (-0.5, RingSide::Out),
            (1.5, RingSide::Out),
        ] {
            let ring = ring_at(&mut body, oldf, at(du, v), &[]);
            assert_eq!(
                wall_side(&body, (oldf, newf), ring).unwrap(),
                want,
                "the ring vertex at ({du:e}, {v})"
            );
        }
    }
    for u in [1e-3, tau - 1e-3, 0.5 * tau, 3.0] {
        let ring = ring_at(&mut body, oldf, at(u, 0.5), &[]);
        assert_eq!(
            wall_side(&body, (oldf, newf), ring).unwrap(),
            RingSide::In,
            "the ring vertex at ({u}, 0.5)"
        );
    }
}

/// **Random wall patches re-home as membership says** (a counterexample
/// search). A patch `[u0, u0 + w] × [v0, v1]` of the unit cylinder,
/// `w` a whole turn in a quarter of the draws and a hair to 0.5 short of
/// one in another quarter, is the run, re-homed against a taller
/// full-turn sheet's references; each bystander is drawn anywhere, or
/// 1e-7 to 1e-1 off one of the patch's edges, and the oracle is its
/// azimuth modulo a turn and its height. A decided side wrong past
/// 1e-9, an undecided reading past 1e-3 of every edge, or an untyped
/// refusal fails.
#[test]
fn random_wall_patches_re_home_as_membership_says() {
    let tau = core::f64::consts::TAU;
    let mut rng = fuzz::start("chord_join::wall_ring_rows::wall_patches");
    let frame = CylFrame::canonical(1.0);
    let mut fails = Vec::new();
    for _ in 0..fuzz::scaled(60) {
        let u0 = rng.range(-4.0, 4.0);
        let w = match rng.below(4) {
            0 => tau,
            1 => tau - rng.log_uniform(1e-4, 0.5),
            _ => rng.range(0.05, tau),
        };
        let (v0, v1) = (rng.range(0.0, 0.4), rng.range(0.6, 1.5));
        let s0 = rng.range(-4.0, 4.0);
        let mut body = Body::<f64>::new();
        let oldf = cyl_wall_sheet(&mut body, frame, (s0, s0 + tau), (-3.0, 2.0), tol());
        let newf = cyl_wall_sheet(&mut body, frame, (u0, u0 + w), (v0, v1), tol());
        for _ in 0..8 {
            let edge = |e: f64, rng: &mut Rng| {
                let off = rng.log_uniform(1e-7, 1e-1);
                if rng.below(2) == 0 { e + off } else { e - off }
            };
            let (u, v) = match rng.below(3) {
                0 => (rng.range(-4.0, 4.0), rng.range(-1.0, 2.0)),
                1 => {
                    let e = if rng.below(2) == 0 { u0 } else { u0 + w };
                    (edge(e, &mut rng), rng.range(v0 - 0.2, v1 + 0.2))
                }
                _ => {
                    let e = if rng.below(2) == 0 { v0 } else { v1 };
                    (rng.range(-4.0, 4.0), edge(e, &mut rng))
                }
            };
            let du = (u - u0).rem_euclid(tau);
            let margin = du
                .min(tau - du)
                .min((w - du).abs())
                .min((v - v0).abs())
                .min((v - v1).abs());
            let inside = (w >= tau || du < w) && v > v0 && v < v1;
            let ring = ring_at(&mut body, oldf, at(u, v), &[]);
            match wall_side(&body, (oldf, newf), ring) {
                Ok(s @ (RingSide::In | RingSide::Out))
                    if (s == RingSide::In) != inside && margin > 1e-9 =>
                {
                    fails.push(format!(
                        "({u}, {v}) in patch ({u0}, {w}, {v0}..{v1}): {s:?}"
                    ));
                }
                Ok(RingSide::Undecided | RingSide::OnRun) if margin > 1e-3 => {
                    fails.push(format!(
                        "({u}, {v}) in patch ({u0}, {w}, {v0}..{v1}) undecided"
                    ));
                }
                Err(SplitJoinError::Escalated { .. }) | Ok(_) => {}
                Err(e) => fails.push(format!("({u}, {v}): {e:?}")),
            }
        }
    }
    assert!(fails.is_empty(), "{fails:#?}; {}", fuzz::replay());
}

/// The certified spec of the spline `piece` of a cylinder × sphere
/// section, run forward or (`back`) from its end to its start, described
/// as the two surfaces' intersection.
fn section_spec(
    pair: geom_brep::SurfacePair,
    piece: geom::NurbsCurve3<f64>,
    back: bool,
) -> EdgeCurveSpec<f64> {
    let (t0, t1) = piece.domain();
    let (piece, t0, t1) = if back {
        (geom_brep::reversed_column(&piece), -t1, -t0)
    } else {
        (piece, t0, t1)
    };
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(piece));
    EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair,
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
        },
        carrier,
        param_start: t0,
        param_end: t1,
    }
}

/// A point of the wall `|azimuth| < 1.2`, `|z| < 0.9` of the radius-0.5
/// cylinder about `z`, and whether it lies inside the ball (`centre`,
/// `big`): a clustered distance off the section loop `c`, or anywhere on
/// the wall; `None` where membership is within its rounding.
fn bystander(
    rng: &mut Rng,
    c: &geom_brep::CarriedComponent<f64>,
    (centre, big): (Point3<f64>, f64),
) -> Option<(Point3<f64>, bool)> {
    let r = 0.5;
    let (theta, z) = if rng.below(4) == 0 {
        (rng.range(-1.1, 1.1), rng.range(-0.85, 0.85))
    } else {
        let p = c.carrier.eval(rng.range(c.span.0, c.span.1));
        let off = rng.log_uniform(1e-5, 5e-2);
        let turn = rng.range(-core::f64::consts::PI, core::f64::consts::PI);
        (
            p.y.atan2(p.x) + turn.cos() * off / r,
            p.z + turn.sin() * off,
        )
    };
    let q = Point3::new(r * theta.cos(), r * theta.sin(), z);
    let margin = (q - centre).norm() - big;
    (margin.abs() > 1e-8).then_some((q, margin < 0.0))
}

/// The section loop a ball (`centre`, radius `big`) cuts the
/// radius-0.5 cylinder about `z` in, carried, and the cylinder's
/// frame.
fn carried_loop(centre: Point3<f64>, big: f64) -> (geom_brep::CarriedComponent<f64>, CylFrame) {
    let frame = CylFrame::canonical(0.5);
    let geom_brep::SectionRead::Built(section) =
        geom_brep::pair_section(&frame.surface(), &ball(centre, big), None, band()).unwrap()
    else {
        panic!("the value builds cylinder × sphere");
    };
    let carried = section
        .carry(Some(geom_brep::SsiLane::marched()), band())
        .unwrap();
    let [c] = carried.components() else {
        panic!("one loop for a ball crossing the wall short of its far side");
    };
    (c.clone(), frame)
}

/// The wall of [`carried_loop`]'s cylinder over `|azimuth| < 1.2`,
/// `|z| < 0.9`, holding a lone-vertex ring at each of `points` and the
/// loop `c` (the ball's, `sphere`) as a ring cut at `cuts` into spline
/// edges, each the certified intersection of the cylinder and the
/// sphere, its last piece the closing chord; after the chord's `mef`,
/// [`ChordJoiner::rehome_rings`]. Whether each point's ring moved to the
/// island.
fn spline_island_rehome(
    (c, frame): (&geom_brep::CarriedComponent<f64>, CylFrame),
    sphere: &geom::Surface<f64>,
    cuts: &[f64],
    points: &[Point3<f64>],
) -> Result<Vec<bool>, SplitJoinError> {
    let geom::Curve3::Nurbs(net) = &c.carrier else {
        panic!("a fitted component's carrier is a spline");
    };
    let mut rest = (**net).clone();
    let mut pieces = Vec::new();
    for &u in cuts {
        let (head, tail) = rest.split_at(u).unwrap();
        pieces.push(head);
        rest = tail;
    }
    pieces.push(rest);
    let n = pieces.len();
    let corners: Vec<Point3<f64>> = pieces.iter().map(|p| p.eval(p.domain().0)).collect();
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(&mut body, frame, (-1.2, 1.2), (-0.9, 0.9), tol());
    let cyl = body.get_face(face).unwrap().surface;
    let rings: Vec<LoopKey> = points
        .iter()
        .map(|&q| ring_at(&mut body, face, q, &[]))
        .collect();
    let island = ring_at(&mut body, face, corners[0], &[]);
    let sph = body.add_surface(sphere.clone());
    let pair = geom_brep::SurfacePair::new(cyl, sph);
    let mut tip: Option<crate::construction::MevCreated> = None;
    let mut first = None;
    for (k, piece) in pieces.iter().take(n - 1).enumerate() {
        let site = match &tip {
            None => MevSite::Lone { r#loop: island },
            Some(h) => MevSite::Fan {
                he1: h.he_minus,
                he2: h.he_minus,
            },
        };
        let made = body
            .mev(
                site,
                corners[k + 1],
                section_spec(pair, piece.clone(), false),
                tol(),
            )
            .unwrap();
        first.get_or_insert(made.he_plus);
        tip = Some(made);
    }
    let last = tip.unwrap().he_plus;
    let after = body.get_half_edge(last).unwrap().next;
    // The chord runs from the run's first vertex to its last.
    let made = body
        .mef(
            MefSite::Chords {
                he1: first.unwrap(),
                he2: after,
            },
            section_spec(pair, pieces[n - 1].clone(), true),
            FaceSurface::Shared {
                key: cyl,
                sense: true,
            },
            tol(),
        )
        .unwrap();
    let remainder = body.get_half_edge(after).unwrap().parent_loop;
    ChordJoiner::new(band()).rehome_rings(&mut body, face, made.face, remainder)?;
    Ok(rings
        .into_iter()
        .map(|ring| body.get_loop(ring).unwrap().face == made.face)
        .collect())
}

/// The sphere of the ball about `centre` of radius `big`.
fn ball(centre: Point3<f64>, big: f64) -> geom::Surface<f64> {
    geom::Surface::Sphere {
        center: centre,
        radius: big,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// **A spline-bounded wall island decides its bystanders** (a fixed
/// witness): the loop of the ball of radius 0.3 centred on the wall at
/// azimuth 0, cut at the middle of its span into two spline edges, sends
/// the bystanders at the ball's centre and at `(0.2, 0.1)` (azimuth,
/// height) to the island and keeps those at `(−0.9, 0.5)` and
/// `(0.3, 0.5)`, at every ε row.
#[test]
fn a_spline_bounded_wall_island_decides_its_bystanders() {
    let (centre, big) = (Point3::new(0.5, 0.0, 0.0), 0.3);
    let (c, frame) = carried_loop(centre, big);
    let on = |u: f64, z: f64| Point3::new(0.5 * u.cos(), 0.5 * u.sin(), z);
    let points = [on(0.0, 0.0), on(0.2, 0.1), on(-0.9, 0.5), on(0.3, 0.5)];
    let mid = 0.5 * (c.span.0 + c.span.1);
    let moved = spline_island_rehome((&c, frame), &ball(centre, big), &[mid], &points).unwrap();
    assert_eq!(moved, [true, true, false, false], "each bystander's face");
}

/// **An island bounded by spline edges re-homes its bystanders as
/// membership in the ball says** (a counterexample search: every sampled
/// bystander, the side the ball gives or a typed refusal). The wall of
/// the radius-0.5 cylinder about `z` over `|azimuth| < 1.2`, `|z| < 0.9`,
/// holds a ring: the loop a ball crossing it cuts, in one to three
/// cuts' pieces, each the certified intersection of the cylinder and the
/// sphere, its last piece the closing chord. After the chord's `mef`,
/// each bystander ring — a lone vertex a clustered distance off the loop,
/// or anywhere on the wall — moves to the island where it lies in the
/// ball and stays where it does not.
#[test]
fn a_spline_bounded_wall_island_re_homes_its_bystanders_as_the_ball_says() {
    let mut rng = fuzz::start("chord_join::wall_ring_rows::spline_island");
    let poses = [
        (Point3::new(0.5, 0.0, 0.0), 0.3),
        (Point3::new(0.35, 0.0, 0.1), 0.3),
        (Point3::new(0.7, 0.1, -0.2), 0.3),
    ];
    let (mut asked, mut refused, mut failures) = (0usize, 0usize, Vec::new());
    for (centre, big) in poses {
        let (c, frame) = carried_loop(centre, big);
        for _ in 0..fuzz::scaled(1) {
            let (lo, hi) = c.span;
            let mut cuts: Vec<f64> = (0..=rng.below(3))
                .map(|_| rng.range(lo + 0.05 * (hi - lo), hi - 0.05 * (hi - lo)))
                .collect();
            cuts.sort_by(f64::total_cmp);
            cuts.dedup_by(|a, b| (*a - *b).abs() < 0.05 * (hi - lo));
            let drawn: Vec<(Point3<f64>, bool)> = (0..4)
                .filter_map(|_| bystander(&mut rng, &c, (centre, big)))
                .collect();
            let points: Vec<Point3<f64>> = drawn.iter().map(|&(q, _)| q).collect();
            asked += drawn.len();
            match spline_island_rehome((&c, frame), &ball(centre, big), &cuts, &points) {
                Err(SplitJoinError::Escalated { .. }) => refused += drawn.len(),
                Err(e) => failures.push(format!("centre {centre:?}, cuts {cuts:?}: {e:?}")),
                Ok(moved) => {
                    for ((q, inside), moved) in drawn.into_iter().zip(moved) {
                        if moved != inside {
                            failures.push(format!(
                                "centre {centre:?}, cuts {cuts:?}: {q:?} in the ball {inside}, \
                                 moved {moved}"
                            ));
                        }
                    }
                }
            }
        }
    }
    println!("[fuzz] spline island: {asked} bystanders, {refused} escalated");
    assert!(
        failures.is_empty(),
        "asked {asked}, escalated {refused}: {:?}; {}",
        &failures[..failures.len().min(10)],
        fuzz::replay()
    );
}

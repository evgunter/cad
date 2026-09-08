//! **R1 review probes for SHELL-5 (PR #2159).**
//!
//! Instruments, not acceptance. Each row prints what it measures; a
//! row asserts only where the PR makes a load-bearing claim, and a row
//! that pins a WRONG-but-current behaviour says so on the page so it
//! goes red the day the behaviour is fixed (self-retiring).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use geom_core::{Affine3, Point2, Point3, Sign, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{
    Body, FaceKey, LoopBoundary, ShellError, ShellKey, ShellRole, SolidKey, VoidContainment,
    VoidEvidence, insert_void,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

/// A `w × d × h` box with its min corner at `(x0, y0, z0)`.
fn boxy_at(x0: f64, y0: f64, z0: f64, w: f64, d: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(x0, y0), 0.0),
        ProfileVertex::new(p2(x0 + w, y0), 0.0),
        ProfileVertex::new(p2(x0 + w, y0 + d), 0.0),
        ProfileVertex::new(p2(x0, y0 + d), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(tol())
        .expect("a rectangle is a valid profile");
    extrude(&profile, Extrusion::Distance(h), tol())
        .expect("a rectangle extrudes")
        .body
}

fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("a polygon is a valid profile");
    extrude(&profile, Extrusion::Distance(h), tol())
        .expect("a polygon extrudes")
        .body
}

/// A meridian polyline revolved a full turn about the sketch's `+y`
/// axis through `(axis_x, 0)`, on the xy plane translated by `z0`.
fn revolved_at(pts: &[(f64, f64)], axis_x: f64, z0: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(tol())
        .expect("the meridian is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(axis_x, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    topo::subtract(a, b, tol())
        .expect("the subtraction runs")
        .body()
        .expect("a body")
        .body
        .clone()
}

/// `insert_void` with the carried-positive evidence the shell verb
/// itself hands the door: the cavity is strictly inside by
/// construction of the fixture, and this is a review instrument.
fn with_void(mut dst: Body<f64>, cavity: Body<f64>) -> Body<f64> {
    let (solid, _) = dst.solids().next().expect("one solid");
    let evidence = VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: Sign::Positive,
                    },
                )
            })
            .collect(),
    };
    insert_void(&mut dst, solid, cavity, &evidence, tol()).expect("the void inserts");
    dst
}

/// `(origin, OUTWARD normal)` of a planar face.
fn plane_of(body: &Body<f64>, face: FaceKey) -> (Point3<f64>, Vec3<f64>) {
    let data = body.get_face(face).expect("the face");
    let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(data.surface) else {
        panic!("{face:?} is not planar")
    };
    (*origin, if data.sense { *normal } else { -*normal })
}

/// The plane constant `n_out · x` of a planar face.
fn plane_const(body: &Body<f64>, face: FaceKey) -> f64 {
    let (o, n) = plane_of(body, face);
    o.x * n.x + o.y * n.y + o.z * n.z
}

/// The planar face of `shell` whose OUTWARD normal is `n` and whose
/// plane constant is `c`.
fn face_on(body: &Body<f64>, shell: ShellKey, n: (f64, f64, f64), c: f64) -> FaceKey {
    let hits: Vec<FaceKey> = body
        .get_shell(shell)
        .expect("the shell")
        .faces
        .iter()
        .copied()
        .filter(|&f| {
            matches!(
                body.get_surface(body.get_face(f).unwrap().surface),
                Some(geom::Surface::Plane { .. })
            )
        })
        .filter(|&f| {
            let (_, nn) = plane_of(body, f);
            (nn.x - n.0).abs() < 1e-9
                && (nn.y - n.1).abs() < 1e-9
                && (nn.z - n.2).abs() < 1e-9
                && (plane_const(body, f) - c).abs() < 1e-9
        })
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "one face with outward {n:?} at {c}: {hits:?}"
    );
    hits[0]
}

/// Every boundary point of a face.
fn face_points(body: &Body<f64>, face: FaceKey) -> Vec<Point3<f64>> {
    let data = body.get_face(face).expect("the face");
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("the loop").boundary else {
            continue;
        };
        for he in body.loop_cycle(first).expect("the cycle") {
            let v = body.get_half_edge(he).expect("he").start;
            let p = body.get_vertex(v).expect("v").point;
            out.push(*body.get_point(p).expect("point"));
        }
    }
    out
}

fn extent(pts: &[Point3<f64>], axis: fn(&Point3<f64>) -> f64) -> (f64, f64) {
    let lo = pts.iter().map(axis).fold(f64::INFINITY, f64::min);
    let hi = pts.iter().map(axis).fold(f64::NEG_INFINITY, f64::max);
    (lo, hi)
}

fn roles_by_solid(body: &Body<f64>) -> Vec<(SolidKey, Vec<ShellRole>)> {
    let roles = topo::classify_shells(body, tol()).expect("the shells classify");
    body.solids()
        .map(|(solid, _)| {
            let mut kinds: Vec<ShellRole> = roles
                .iter()
                .filter(|c| c.solid == solid)
                .map(|c| c.role)
                .collect();
            kinds.sort_by_key(|r| format!("{r:?}"));
            (solid, kinds)
        })
        .collect()
}

fn roles(body: &Body<f64>) -> (ShellKey, Vec<ShellKey>) {
    let rows = topo::classify_shells(body, tol()).expect("classifies");
    let outer: Vec<ShellKey> = rows
        .iter()
        .filter(|c| c.role == ShellRole::Outer)
        .map(|c| c.shell)
        .collect();
    assert_eq!(outer.len(), 1);
    let voids = rows
        .iter()
        .filter(|c| c.role == ShellRole::Void)
        .map(|c| c.shell)
        .collect();
    (outer[0], voids)
}

fn v(w: f64, d: f64, h: f64) -> f64 {
    w * d * h
}

// ---------------------------------------------------------------------
// Claim 3: the clearance gate reads the OPERAND's footprints, and a
// void's dilated faces grow by `t` on every side.
// ---------------------------------------------------------------------

/// **Two voids offset diagonally.** Facing walls `g = 0.2` apart whose
/// footprints are disjoint by `δ = 0.1` in `y`; at `t = 0.15` both are
/// below `2t = 0.3`, so the dilated twins cross — but the gate tests
/// the operand's footprints, which do not overlap, and never looks.
#[test]
fn r1p3_diagonal_voids_cross_silently() {
    let outer = boxy_at(0.0, 0.0, 0.0, 6.0, 4.0, 4.0);
    let v1 = boxy_at(1.0, 1.0, 1.0, 1.2, 1.0, 2.0); // x [1, 2.2], y [1, 2]
    let v2 = boxy_at(2.4, 2.1, 1.0, 1.2, 1.0, 2.0); // x [2.4, 3.6], y [2.1, 3.1]
    let body = subtract(&subtract(&outer, &v1), &v2);
    assert_eq!(body.solids().count(), 1);
    assert_eq!(body.shells().count(), 3);
    let (_, voids) = roles(&body);
    assert_eq!(voids.len(), 2);
    let t = 0.15;
    // The two facing void walls: void 1's +x wall at x = 2.2 (its
    // outward normal points OUT of the material, i.e. into the cavity:
    // −x) and void 2's −x wall at x = 2.4 (outward +x).
    let a = voids
        .iter()
        .find_map(|&s| {
            body.get_shell(s).unwrap().faces.iter().copied().find(|&f| {
                let (_, n) = plane_of(&body, f);
                n.x < -0.5 && (plane_const(&body, f) + 2.2).abs() < 1e-9
            })
        })
        .expect("void 1's +x wall");
    let b = voids
        .iter()
        .find_map(|&s| {
            body.get_shell(s).unwrap().faces.iter().copied().find(|&f| {
                let (_, n) = plane_of(&body, f);
                n.x > 0.5 && (plane_const(&body, f) - 2.4).abs() < 1e-9
            })
        })
        .expect("void 2's −x wall");
    let ya = extent(&face_points(&body, a), |p| p.y);
    let yb = extent(&face_points(&body, b), |p| p.y);
    println!("[measured] operand footprints in y: a {ya:?}, b {yb:?} (disjoint by 0.1)");

    match topo::shell(&body, t, tol()) {
        Err(e) => println!("[measured] diagonal voids at t={t}: refuses {e}"),
        Ok(s) => {
            let out = &s.body;
            let tier3 = topo::validate_geometric(out, tol());
            let props = topo::mass_properties(out, tol()).expect("props");
            // The sum of the three walls as if they did not cross; the
            // twins' overlap `0.1 × 0.2 × 2.3` is counted twice in it.
            let naive =
                (v(6.0, 4.0, 4.0) - v(5.7, 3.7, 3.7)) + 2.0 * (v(1.5, 1.3, 2.3) - v(1.2, 1.0, 2.0));
            let ta = s.naming.inner_of(a).expect("twin of a");
            let tb = s.naming.inner_of(b).expect("twin of b");
            let (oa, _) = plane_of(out, ta);
            let (ob, _) = plane_of(out, tb);
            let tya = extent(&face_points(out, ta), |p| p.y);
            let tyb = extent(&face_points(out, tb), |p| p.y);
            println!(
                "[measured] diagonal voids at t={t}: BUILDS; solids={} shells={} tier3={tier3:?} volume={} naive-sum={naive}",
                out.solids().count(),
                out.shells().count(),
                props.volume
            );
            println!(
                "[measured] twin of a at x={} (y {tya:?}); twin of b at x={} (y {tyb:?}) — crossed: {}",
                oa.x,
                ob.x,
                oa.x > ob.x && tya.1 > tyb.0
            );
            // Pinned as measured: the twins cross (void 1's twin wall
            // sits at x = 2.35, void 2's at 2.25, overlapping in y on
            // [1.95, 2.15]) and nothing refuses. Goes red when a gate
            // reads the OFFSET footprints (or SHELL-4's certificate
            // lands); rewrite it as the refusal then.
            assert!(oa.x > ob.x, "the dilated twins cross in x");
            assert!(tya.1 > tyb.0 && tyb.1 > tya.0, "and overlap in y");
            assert_eq!(tier3, Ok(()), "tier 3 does not see it");
            assert!(
                (props.volume - naive).abs() < 1e-9,
                "the props sum the crossing walls"
            );
        }
    }
}

/// **The same class on an OUTER shell, before this PR:** an S-bend
/// prism whose two risers (x = 1 and x = 0.8) face each other across
/// `0.2` with footprints disjoint by `0.1` in `y`, and whose two
/// shelves (y = 0.3 and y = 0.2) face each other across `0.1` with
/// footprints disjoint by `0.2` in `x`. At `t = 0.12` (`2t = 0.24`)
/// every pair is below the wall and none is gated. Compiled and run at
/// the merge base too (see the R1 report).
#[test]
fn r1p3_outer_shell_s_bend_crosses_silently_pre_existing() {
    let s_bend = prism(
        &[
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 0.2),
            (2.0, 0.2),
            (2.0, 0.5),
            (0.8, 0.5),
            (0.8, 0.3),
            (0.0, 0.3),
        ],
        1.0,
    );
    let (shell, _) = s_bend.shells().next().unwrap();
    let riser_r = face_on(&s_bend, shell, (1.0, 0.0, 0.0), 1.0); // x = 1, y ∈ [0, 0.2]
    let riser_l = face_on(&s_bend, shell, (-1.0, 0.0, 0.0), -0.8); // x = 0.8, y ∈ [0.3, 0.5]
    for t in [0.09, 0.12] {
        match topo::shell(&s_bend, t, tol()) {
            Err(e) => println!("[measured] S-bend at t={t}: refuses {e}"),
            Ok(s) => {
                let out = &s.body;
                let tier3 = topo::validate_geometric(out, tol());
                let props = topo::mass_properties(out, tol()).expect("props");
                let (o_r, _) = plane_of(out, s.naming.inner_of(riser_r).unwrap());
                let (o_l, _) = plane_of(out, s.naming.inner_of(riser_l).unwrap());
                println!(
                    "[measured] S-bend at t={t}: BUILDS tier3={tier3:?} volume={} ; riser twins at x={} (from x=1) and x={} (from x=0.8) — crossed: {}",
                    props.volume,
                    o_r.x,
                    o_l.x,
                    o_r.x < o_l.x
                );
                if t > 0.1 {
                    assert!(o_r.x < o_l.x, "the column's two offsets cross");
                    assert_eq!(tier3, Ok(()), "tier 3 does not see it");
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// Claim 2: the curved window on a hollow operand.
// ---------------------------------------------------------------------

/// **A hollow vessel whose CYLINDER wall is thinner than `2t`** while
/// its planar walls are thick: `vessel(1, 2)` with a `r = 0.9`,
/// `y ∈ [0.5, 1.5]` cylindrical cavity (wall 0.1 radially, 0.5
/// axially) shelled at `t = 0.08`. The planar gate has nothing to say;
/// the per-face reach decides are vacuous on a dilating cylinder.
/// SELF-RETIRING: pins the current silent build; goes red the day
/// SHELL-4's certificate refuses it.
#[test]
fn r1p2_thin_cylinder_wall_hollow_vessel_builds_silently() {
    let vessel = revolved_at(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)], 0.0, 0.0);
    let cavity = revolved_at(&[(0.0, 0.5), (0.9, 0.5), (0.9, 1.5), (0.0, 1.5)], 0.0, 0.0);
    let hollow = with_void(vessel, cavity);
    assert_eq!(hollow.shells().count(), 2);
    assert_eq!(topo::validate_geometric(&hollow, tol()), Ok(()));
    let radius_of = |body: &Body<f64>, shell: ShellKey| -> f64 {
        body.get_shell(shell)
            .unwrap()
            .faces
            .iter()
            .find_map(
                |&f| match body.get_surface(body.get_face(f).unwrap().surface) {
                    Some(geom::Surface::Cylinder { radius, .. }) => Some(*radius),
                    _ => None,
                },
            )
            .expect("a cylinder wall")
    };
    let (outer, voids) = roles(&hollow);
    println!(
        "[measured] hollow vessel: outer r={} void r={}",
        radius_of(&hollow, outer),
        radius_of(&hollow, voids[0])
    );
    let t = 0.08;
    match topo::shell(&hollow, t, tol()) {
        Err(e) => println!("[measured] thin-cylinder hollow at t={t}: refuses {e}"),
        Ok(s) => {
            let out = &s.body;
            let tier3 = topo::validate_geometric(out, tol());
            let props = topo::mass_properties(out, tol()).expect("props");
            let pi = core::f64::consts::PI;
            let naive = pi * ((1.0 * 2.0 - 0.92 * 0.92 * 1.84) + (0.98 * 0.98 * 1.16 - 0.81 * 1.0));
            let twins: Vec<(ShellKey, f64)> = out
                .shells()
                .filter(|(k, _)| *k != outer && *k != voids[0])
                .map(|(k, _)| (k, radius_of(out, k)))
                .collect();
            println!(
                "[measured] thin-cylinder hollow at t={t}: BUILDS solids={} shells={} tier3={tier3:?} volume={} naive-sum={naive} twin radii={twins:?}",
                out.solids().count(),
                out.shells().count(),
                props.volume
            );
            // The eroded outer twin is the cavity of the operand's solid.
            let r_out_twin = radius_of(
                out,
                out.get_solid(s.naming.thickened[0].0)
                    .unwrap()
                    .shells
                    .iter()
                    .copied()
                    .find(|&k| k != outer)
                    .unwrap(),
            );
            let r_void_twin = radius_of(
                out,
                s.naming
                    .inner_of(face_of_cyl(&hollow, voids[0]))
                    .map(|f| out.get_face(f).unwrap().shell)
                    .unwrap(),
            );
            println!(
                "[measured] eroded outer twin r={r_out_twin}, dilated void twin r={r_void_twin} — crossed: {}",
                r_void_twin > r_out_twin
            );
            assert!((r_out_twin - 0.92).abs() < 1e-12);
            assert!((r_void_twin - 0.98).abs() < 1e-12);
            assert!(r_void_twin > r_out_twin, "the two cylinder twins cross");
            assert_eq!(tier3, Ok(()), "tier 3 does not see it");
        }
    }
}

/// The cylinder face of a shell.
fn face_of_cyl(body: &Body<f64>, shell: ShellKey) -> FaceKey {
    body.get_shell(shell)
        .unwrap()
        .faces
        .iter()
        .copied()
        .find(|&f| {
            matches!(
                body.get_surface(body.get_face(f).unwrap().surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .expect("a cylinder face")
}

// ---------------------------------------------------------------------
// Claim 1: a curved void wall through the PER-CHART door.
// ---------------------------------------------------------------------

/// **A box with a cylindrical cavity**: neither all-planar nor axial
/// (the box's side planes are parallel to the cavity's axis), so the
/// per-chart door moves every chart, the void's cylinder included —
/// a DILATION (`d = +t`) on a reversed cylinder face. Closed form:
/// `[4³ − 3.8³] + π[1.1²·2.2 − 1²·2]`.
#[test]
fn r1p1_cylindrical_void_in_a_box_through_the_per_chart_door() {
    let cube = boxy_at(0.0, 0.0, 0.0, 4.0, 4.0, 4.0);
    // Axis: the line x = 2, z = 2 along y; r = 1; y ∈ [1, 3].
    let cavity = revolved_at(&[(2.0, 1.0), (3.0, 1.0), (3.0, 3.0), (2.0, 3.0)], 2.0, 2.0);
    match topo::subtract(&cube, &cavity, tol()) {
        Ok(r) => match r.body() {
            Some(b) => println!(
                "[measured] the boolean builds box − cylinder: solids={} shells={}",
                b.body.solids().count(),
                b.body.shells().count()
            ),
            None => println!("[measured] the boolean returned no body for box − cylinder"),
        },
        Err(e) => println!("[measured] the boolean refuses box − cylinder: {e}"),
    }
    let hollow = with_void(cube, cavity);
    assert_eq!(topo::validate_geometric(&hollow, tol()), Ok(()));
    let t = 0.1;
    match topo::shell(&hollow, t, tol()) {
        Err(e) => println!("[measured] box with cylindrical void at t={t}: refuses {e}"),
        Ok(s) => {
            let out = &s.body;
            let tier3 = topo::validate_geometric(out, tol());
            let props = topo::mass_properties(out, tol()).expect("props");
            let pi = core::f64::consts::PI;
            let want = (64.0 - 3.8f64.powi(3)) + pi * (1.1 * 1.1 * 2.2 - 2.0);
            println!(
                "[measured] box with cylindrical void at t={t}: BUILDS solids={} shells={} tier3={tier3:?} roles={:?} volume={} (pad {}) want={want}",
                out.solids().count(),
                out.shells().count(),
                roles_by_solid(out),
                props.volume,
                props.volume_pad
            );
            assert_eq!(tier3, Ok(()));
            assert_eq!(out.solids().count(), 2);
            for (_, kinds) in roles_by_solid(out) {
                assert_eq!(kinds, vec![ShellRole::Outer, ShellRole::Void]);
            }
            assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
        }
    }
}

// ---------------------------------------------------------------------
// Claim 5: is `OperandOuterShells` reachable?
// ---------------------------------------------------------------------

/// The PR's own §5 fixture — a hollow 2³ box subtracted from a 6³ box
/// — is one solid with two `Outer` shells; the verb refuses it typed.
#[test]
fn r1p5_operand_outer_shells_is_reachable_through_the_boolean() {
    let big = boxy_at(0.0, 0.0, 0.0, 6.0, 6.0, 6.0);
    let small = topo::shell(&boxy_at(2.0, 2.0, 2.0, 2.0, 2.0, 2.0), 0.25, tol())
        .expect("the small box shells")
        .body;
    let body = subtract(&big, &small);
    let rows = topo::classify_shells(&body, tol()).expect("classifies");
    println!(
        "[measured] big − hollow small: solids={} shells={} roles={:?}",
        body.solids().count(),
        body.shells().count(),
        rows.iter()
            .map(|c| (c.shell, c.role, c.volume))
            .collect::<Vec<_>>()
    );
    let e = topo::shell(&body, 0.1, tol()).expect_err("two outer shells refuse");
    println!("[measured] shell of it: {e}");
    assert!(
        matches!(e, ShellError::OperandOuterShells { outer: 2 }),
        "{e}"
    );
}

// ---------------------------------------------------------------------
// Claim 6: the opened arm on a void face WITH A HOLE (unmeasured).
// ---------------------------------------------------------------------

/// A holed box (a square pillar through it) subtracted from a bigger
/// box: the cavity has a pillar through it and its ceiling carries a
/// ring. Designate that ceiling.
#[test]
fn r1p6_open_a_void_ceiling_with_a_pillar_through_it() {
    let outer_loop = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.0, 1.0), 0.0),
        ProfileVertex::new(p2(3.0, 1.0), 0.0),
        ProfileVertex::new(p2(3.0, 3.0), 0.0),
        ProfileVertex::new(p2(1.0, 3.0), 0.0),
    ]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.8, 1.8), 0.0),
        ProfileVertex::new(p2(1.8, 2.2), 0.0),
        ProfileVertex::new(p2(2.2, 2.2), 0.0),
        ProfileVertex::new(p2(2.2, 1.8), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 1.0)));
    let profile = match Profile::new(plane, vec![outer_loop, hole]).validate(tol()) {
        Ok(p) => p,
        Err(e) => {
            println!("[measured] the holed profile refuses: {e:?}");
            return;
        }
    };
    let holed = extrude(&profile, Extrusion::Distance(2.0), tol())
        .expect("the holed box extrudes")
        .body;
    println!(
        "[measured] holed box: faces={} rings={}",
        holed.faces().count(),
        holed.faces().map(|(_, f)| f.rings.len()).sum::<usize>()
    );
    let cube = boxy_at(0.0, 0.0, 0.0, 4.0, 4.0, 4.0);
    let body = match topo::subtract(&cube, &holed, tol()) {
        Ok(r) => match r.body() {
            Some(b) => b.body.clone(),
            None => {
                println!("[measured] the boolean returned no body for box − holed box");
                return;
            }
        },
        Err(e) => {
            println!("[measured] the boolean refuses box − holed box: {e}");
            return;
        }
    };
    println!(
        "[measured] box − holed box: solids={} shells={} tier3={:?}",
        body.solids().count(),
        body.shells().count(),
        topo::validate_geometric(&body, tol())
    );
    let (_, voids) = roles(&body);
    assert_eq!(voids.len(), 1);
    // Outward points out of the material: into the cavity, −z.
    let ceiling = face_on(&body, voids[0], (0.0, 0.0, -1.0), -3.0);
    let rings = body.get_face(ceiling).unwrap().rings.len();
    println!("[measured] the void's ceiling {ceiling:?} carries {rings} ring(s)");
    let t = 0.1;
    match topo::shell_open(&body, t, &[ceiling], tol()) {
        Err(e) => println!("[measured] open the holed void ceiling at t={t}: refuses {e}"),
        Ok(s) => {
            let out = &s.body;
            let tier3 = topo::validate_geometric(out, tol());
            let props = topo::mass_properties(out, tol());
            // A = 4³ − 3.8³; B = V′ − V where V = 2²·2 − 0.4²·2 and
            // V′ = 2.2³ − 0.2²·2.2; the removed slab is the counterpart's
            // footprint (2.2² − 0.2²) × t.
            let a = 64.0 - 3.8f64.powi(3);
            let b = (2.2f64.powi(3) - 0.04 * 2.2) - (8.0 - 0.16 * 2.0);
            let slab = (2.2 * 2.2 - 0.04) * t;
            let want = a + b - slab;
            println!(
                "[measured] open the holed void ceiling at t={t}: BUILDS solids={} shells={} tier3={tier3:?} volume={:?} want={want} rims={} holes={} dead.faces={:?} dead.shells={:?}",
                out.solids().count(),
                out.shells().count(),
                props.as_ref().map(|p| p.volume),
                s.naming.rims.len(),
                s.naming.rims.first().map_or(0, |r| r.holes.len()),
                s.naming.dead.faces,
                s.naming.dead.shells
            );
            if let Some(r) = s.naming.rims.first() {
                let (_, n) = plane_of(out, r.rim);
                println!("[measured] rim {:?} outward normal {n:?}", r.rim);
                for h in &r.holes {
                    let (_, n) = plane_of(out, h.face);
                    println!(
                        "[measured] hole rim {:?} outward normal {n:?} rings={}",
                        h.face,
                        out.get_face(h.face).unwrap().rings.len()
                    );
                }
            }
            let mesh = mesh::tessellate(out, 0.01, tol());
            println!(
                "[measured] tessellates: {:?}",
                mesh.as_ref().map(|m| m.patches.len())
            );
        }
    }
}

// ---------------------------------------------------------------------
// The end-to-end exercise: a user hollows a part, hollows it again,
// then opens the inner wall.
// ---------------------------------------------------------------------

#[test]
fn r1_e2e_hollow_twice_then_open_the_inner_wall() {
    let part = boxy_at(0.0, 0.0, 0.0, 2.0, 3.0, 4.0);
    let (shell0, _) = part.shells().next().unwrap();
    let top = face_on(&part, shell0, (0.0, 0.0, 1.0), 4.0);

    // 1. hollow it.
    let first = topo::shell(&part, 0.25, tol()).expect("hollow");
    println!(
        "[e2e] hollow: solids={} shells={} thickened={:?}",
        first.body.solids().count(),
        first.body.shells().count(),
        first.naming.thickened
    );
    // The inner wall's ceiling is the top's cavity twin — the record
    // is the only way to name it without probing geometry.
    let ceiling = first.naming.inner_of(top).expect("the top's twin");

    // 2. hollow it again.
    let twice = topo::shell(&first.body, 0.05, tol()).expect("hollow twice");
    println!(
        "[e2e] hollow twice: solids={} shells={} thickened={:?} roles={:?}",
        twice.body.solids().count(),
        twice.body.shells().count(),
        twice.naming.thickened,
        roles_by_solid(&twice.body)
    );
    let props = topo::mass_properties(&twice.body, tol()).expect("props");
    println!(
        "[e2e] hollow twice volume={} area={}",
        props.volume, props.surface_area
    );

    // 3a. open the inner wall ON THE RESULT: what a user would try first.
    match topo::shell_open(&twice.body, 0.05, &[ceiling], tol()) {
        Ok(_) => println!("[e2e] opening the twice-hollowed body: Ok (!)"),
        Err(e) => println!("[e2e] opening the twice-hollowed body: refuses {e}"),
    }
    // 3b. the way that works: fold the opening into the second shell.
    let opened =
        topo::shell_open(&first.body, 0.05, &[ceiling], tol()).expect("open the inner wall");
    let out = &opened.body;
    println!(
        "[e2e] opened inner wall: solids={} shells={} tier3={:?} roles={:?}",
        out.solids().count(),
        out.shells().count(),
        topo::validate_geometric(out, tol()),
        roles_by_solid(out)
    );
    let props = topo::mass_properties(out, tol()).expect("props");
    let want = (v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9)) + (v(1.6, 2.6, 3.6) - v(1.5, 2.5, 3.5))
        - 1.6 * 2.6 * 0.05;
    println!(
        "[e2e] opened volume={} want={want} rim={:?} ring_edges={}",
        props.volume,
        opened.naming.rims[0].rim,
        opened.naming.rims[0].ring_edges.len()
    );
    assert!((props.volume - want).abs() < 1e-12);

    // 4. mesh it.
    match mesh::tessellate(out, 0.01, tol()) {
        Ok(m) => println!(
            "[e2e] mesh: positions={} patches={} triangles={}",
            m.positions.len(),
            m.patches.len(),
            m.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
        ),
        Err(e) => println!("[e2e] mesh refuses: {e}"),
    }
    // 5. export it (STEP is the only exporter in the workspace).
    match step_export::step_string(
        out,
        &step_export::StepOptions {
            product_name: "opened inner wall".into(),
            ..Default::default()
        },
        tol(),
    ) {
        Ok(s) => println!(
            "[e2e] STEP: {} bytes, MANIFOLD_SOLID_BREP × {}",
            s.len(),
            s.matches("MANIFOLD_SOLID_BREP").count()
        ),
        Err(e) => println!("[e2e] STEP refuses: {e}"),
    }
    match step_export::step_string(
        &first.body,
        &step_export::StepOptions {
            product_name: "hollow once".into(),
            ..Default::default()
        },
        tol(),
    ) {
        Ok(s) => println!(
            "[e2e] STEP (hollow once, pre-existing shape): {} bytes",
            s.len()
        ),
        Err(e) => println!("[e2e] STEP (hollow once, pre-existing shape) refuses: {e}"),
    }
    match step_export::step_string(
        &twice.body,
        &step_export::StepOptions {
            product_name: "hollow twice".into(),
            ..Default::default()
        },
        tol(),
    ) {
        Ok(s) => println!(
            "[e2e] STEP (sealed, 2 solids): {} bytes, MANIFOLD_SOLID_BREP × {}",
            s.len(),
            s.matches("MANIFOLD_SOLID_BREP").count()
        ),
        Err(e) => println!("[e2e] STEP (sealed, 2 solids) refuses: {e}"),
    }
}

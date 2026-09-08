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
    Body, FaceKey, ShellError, ShellKey, ShellRole, VoidContainment, VoidEvidence, insert_void,
};

use crate::verbs_shell::{brick, prism, roles_by_solid, v};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

/// A `w × d × h` box with its min corner at `(x0, y0, z0)`.
fn boxy_at(x0: f64, y0: f64, z0: f64, w: f64, d: f64, h: f64) -> Body<f64> {
    brick(x0, x0 + w, y0, y0 + d, z0, z0 + h)
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

// ---------------------------------------------------------------------
// Claim 3: the clearance gate reads the OPERAND's footprints, and a
// void's dilated faces grow by `t` on every side.
// ---------------------------------------------------------------------

/// **Two voids offset diagonally.** Facing walls `g = 0.2` apart whose
/// operand footprints are disjoint by `δ = 0.1` in `y`; at `t = 0.15`
/// both are below `2t = 0.3`, so the dilated twins would cross. The
/// gate grows each footprint by `t` before the separation test — the
/// footprint an offset has past a concave edge — so the pair is read
/// as facing and refused. (R1's row; written when the gate read the
/// operand's footprints and built this silently, crossing twins at
/// x = 2.35 and x = 2.25.)
#[test]
fn r1p3_diagonal_voids_refuse_at_the_grown_footprint_gate() {
    let outer = boxy_at(0.0, 0.0, 0.0, 6.0, 4.0, 4.0);
    let v1 = boxy_at(1.0, 1.0, 1.0, 1.2, 1.0, 2.0); // x [1, 2.2], y [1, 2]
    let v2 = boxy_at(2.4, 2.1, 1.0, 1.2, 1.0, 2.0); // x [2.4, 3.6], y [2.1, 3.1]
    let body = subtract(&subtract(&outer, &v1), &v2);
    assert_eq!(body.solids().count(), 1);
    assert_eq!(body.shells().count(), 3);
    let (_, voids) = roles(&body);
    assert_eq!(voids.len(), 2);
    let t = 0.15;
    let e = topo::shell(&body, t, tol()).expect_err("the diagonal pair is read as facing");
    let ShellError::WallClearance {
        face,
        other,
        gap,
        needed,
    } = e
    else {
        panic!("expected the wall-clearance gate, got {e}");
    };
    assert!(
        (gap - 0.2).abs() < 1e-9,
        "the facing walls are 0.2 apart, got {gap}"
    );
    assert!((needed - 0.3).abs() < 1e-12);
    let shell_of = |f: FaceKey| body.get_face(f).expect("an operand face").shell;
    assert!(voids.contains(&shell_of(face)) && voids.contains(&shell_of(other)));
    assert_ne!(shell_of(face), shell_of(other), "one face of each void");
    // Below the wall the same body builds: at t = 0.09 the walls need
    // 0.18 < 0.2 and the grown footprints (disjoint by 0.1 − 0.18 < 0)
    // overlap, but the gap decide passes.
    let s = topo::shell(&body, 0.09, tol()).expect("clear of every wall");
    assert_eq!(topo::validate_geometric(&s.body, tol()), Ok(()));
    assert_eq!(s.body.solids().count(), 3);
}

/// **The same class on an OUTER shell, pre-existing:** an S-bend prism
/// whose two risers (x = 1 and x = 0.8) face each other across `0.2`
/// with footprints disjoint by `0.1` in `y`, and whose two shelves
/// (y = 0.3 and y = 0.2) face each other across `0.1` with footprints
/// disjoint by `0.2` in `x`. At `t = 0.12` (`2t = 0.24`) the risers'
/// grown footprints overlap and the pair refuses; at `t = 0.09`
/// (`2t = 0.18`) the risers clear their gap and the shelves' grown
/// footprints (`0.2 − 0.18 = 0.02` apart) stay separated — and the
/// body builds with its riser twins UNCROSSED, which is what the
/// growth by exactly `t` buys over a coarser gate. (R1's row; at the
/// merge base the `t = 0.12` body built with the twins crossed.)
#[test]
fn r1p3_outer_shell_s_bend_refuses_above_the_wall_and_builds_below_it() {
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

    let e = topo::shell(&s_bend, 0.12, tol()).expect_err("the risers' offsets would cross");
    let ShellError::WallClearance {
        face, other, gap, ..
    } = e
    else {
        panic!("expected the wall-clearance gate, got {e}");
    };
    assert!(
        (gap - 0.2).abs() < 1e-9,
        "the risers are 0.2 apart, got {gap}"
    );
    let mut named = vec![face, other];
    named.sort();
    let mut risers = vec![riser_r, riser_l];
    risers.sort();
    assert_eq!(named, risers, "the refusal names the two risers");

    let s = topo::shell(&s_bend, 0.09, tol()).expect("below the wall it builds");
    let out = &s.body;
    assert_eq!(topo::validate_geometric(out, tol()), Ok(()));
    let (o_r, _) = plane_of(out, s.naming.inner_of(riser_r).unwrap());
    let (o_l, _) = plane_of(out, s.naming.inner_of(riser_l).unwrap());
    assert!(
        o_r.x > o_l.x,
        "the riser twins do not cross: {} from x = 1, {} from x = 0.8",
        o_r.x,
        o_l.x
    );
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
// Claim 6: the opened arm on a void face WITH A HOLE (unmeasured).
// ---------------------------------------------------------------------

/// A holed box (a square pillar through it) subtracted from a bigger
/// box: the cavity has a pillar through it and its ceiling carries a
/// ring. Designate that ceiling — the hole path of the void-side glue,
/// closed form and all (the row that closed
/// `work/shell/shell-open-on-a-void-face-with-a-hole.md`; R1's row,
/// R2 built the same fixture at 6×6×4 and measured the same shape).
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
            // Pinned (the residue file's missing row): tier 3, the closed
            // form, one rim and one hole rim both facing UP into the gap,
            // the designated face dead, the operand void fused away.
            assert_eq!(tier3, Ok(()));
            let volume = props.expect("props").volume;
            assert!((volume - want).abs() <= 1e-12, "got {volume}, want {want}");
            let r = &s.naming.rims[0];
            assert_eq!(r.holes.len(), 1);
            assert!(plane_of(out, r.rim).1.z > 0.5, "the rim faces the gap");
            assert!(
                plane_of(out, r.holes[0].face).1.z > 0.5,
                "the hole rim faces the gap"
            );
            assert!(out.get_face(ceiling).is_none());
            assert_eq!(s.naming.dead.shells, vec![voids[0]]);
            assert!(mesh.is_ok());
        }
    }
}

// ---------------------------------------------------------------------
// The end-to-end exercise: a user hollows a part, hollows it again,
// then opens the inner wall.
// ---------------------------------------------------------------------

/// R1's end-to-end row (R2 wrote the same exercise; its dead-shell
/// finding lives in `verbs_shell`'s void-ceiling row now).
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
    // Refused: the verb is single-solid, and a twice-hollowed body has
    // two (`work/shell/shell-open-on-a-multi-solid-body.md`).
    let e = topo::shell_open(&twice.body, 0.05, &[ceiling], tol())
        .expect_err("a multi-solid body has no single solid to open");
    println!("[e2e] opening the twice-hollowed body: refuses {e}");
    assert!(matches!(e, ShellError::NotOneSolid { solids: 2 }), "{e}");
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

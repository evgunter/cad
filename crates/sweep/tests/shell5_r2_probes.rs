//! **SHELL-5 review probes (R2).** Falsification rows for the claims
//! the unit makes about a hollow operand: that the clearance gate is
//! sufficient on planar operands, and that the curved window is
//! documented as exactly what it is.
//!
//! Rows here assert MEASURED behaviour, including behaviour that is
//! wrong. Each such row says so on its face and names what would have
//! to land for it to go red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, LoopBoundary, ShellKey, ShellRole};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A `w x d x h` box at the origin.
fn boxy(w: f64, d: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(w, 0.0), 0.0),
        ProfileVertex::new(p2(w, d), 0.0),
        ProfileVertex::new(p2(0.0, d), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a rectangle is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a rectangle extrudes")
        .body
}

/// An axis-aligned box `[x0,x1] x [y0,y1] x [z0,z1]` as a body.
fn brick(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(x0, y0), 0.0),
        ProfileVertex::new(p2(x1, y0), 0.0),
        ProfileVertex::new(p2(x1, y1), 0.0),
        ProfileVertex::new(p2(x0, y1), 0.0),
    ]);
    let plane = SketchPlane::new(geom_core::Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(tol)
        .expect("a rectangle is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .expect("a rectangle extrudes")
        .body
}

fn cut(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    topo::subtract(a, b, Tol::witness())
        .expect("the subtraction runs")
        .body()
        .expect("a body")
        .body
        .clone()
}

fn void_shells(body: &Body<f64>) -> Vec<ShellKey> {
    topo::classify_shells(body, Tol::witness())
        .expect("classifies")
        .iter()
        .filter(|c| c.role == ShellRole::Void)
        .map(|c| c.shell)
        .collect()
}

/// The axis-aligned extent of a shell's vertices.
fn shell_box(body: &Body<f64>, shell: ShellKey) -> [(f64, f64); 3] {
    let mut out = [(f64::INFINITY, f64::NEG_INFINITY); 3];
    for &face in &body.get_shell(shell).expect("a shell").faces {
        let data = body.get_face(face).expect("a face");
        for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
            let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("a loop").boundary else {
                continue;
            };
            for he in body.loop_cycle(first).expect("a cycle") {
                let start = body.get_half_edge(he).expect("a half-edge").start;
                let vertex = body.get_vertex(start).expect("a vertex");
                let pt = *body.get_point(vertex.point).expect("a point");
                for (i, c) in [pt.x, pt.y, pt.z].into_iter().enumerate() {
                    out[i].0 = out[i].0.min(c);
                    out[i].1 = out[i].1.max(c);
                }
            }
        }
    }
    out
}

fn overlap(a: [(f64, f64); 3], b: [(f64, f64); 3]) -> f64 {
    let mut vol = 1.0;
    for i in 0..3 {
        let lo = a[i].0.max(b[i].0);
        let hi = a[i].1.min(b[i].1);
        if hi <= lo {
            return 0.0;
        }
        vol *= hi - lo;
    }
    vol
}

// ---------------------------------------------------------------------
// Claim 3: `wall_clearance` is NOT sufficient on planar hollow operands
// ---------------------------------------------------------------------

/// **Two voids offset DIAGONALLY.** Facing walls are `g` apart with
/// `g < 2t`, but the facing faces' projected footprints are disjoint on
/// the operand, so `footprints_may_overlap` skips both pairs and the
/// gate says nothing. The dilated twins nevertheless grow by `t` on
/// EVERY side and interpenetrate. Measured: the verb builds, tier 3 is
/// green, and two thin solids of the result claim the same material.
///
/// This row asserts the DEFECT. It goes red the day a clearance
/// certificate reads the moved body (issue #1055 / SHELL-4).
#[test]
fn r2_diagonal_voids_slip_the_clearance_gate_and_cross() {
    let tol = Tol::witness();
    let outer = boxy(6.0, 4.0, 4.0);
    // A: x 1.0..2.5, y 0.8..1.8   B: x 2.9..4.4, y 2.3..3.3, both z 1..3.
    let one = cut(&outer, &brick(1.0, 2.5, 0.8, 1.8, 1.0, 3.0));
    let body = cut(&one, &brick(2.9, 4.4, 2.3, 3.3, 1.0, 3.0));
    assert_eq!(body.solids().count(), 1, "one solid");
    assert_eq!(body.shells().count(), 3, "outer plus two voids");
    let voids = void_shells(&body);
    assert_eq!(voids.len(), 2);

    // The facing walls are 0.4 apart in x and 0.5 apart in y; 2t = 0.6
    // exceeds both. The gate does not fire.
    let t = 0.3;
    let shelled = topo::shell(&body, t, tol).expect("MEASURED: the gate lets this through");
    let out = &shelled.body;
    assert_eq!(
        topo::validate_geometric(out, tol),
        Ok(()),
        "tier 3 is green"
    );
    assert_eq!(out.solids().count(), 3);
    assert_eq!(out.shells().count(), 6);

    // The two dilated twins occupy a common box.
    let twins: Vec<ShellKey> = shelled
        .naming
        .thickened
        .iter()
        .filter(|(_, s)| voids.contains(s))
        .map(|(solid, _)| {
            let shells = &out.get_solid(*solid).expect("a thin solid").shells;
            shells[0]
        })
        .collect();
    assert_eq!(twins.len(), 2, "one twin per void");
    let (a, b) = (shell_box(out, twins[0]), shell_box(out, twins[1]));
    let common = overlap(a, b);
    assert!(
        common > 1e-9,
        "MEASURED DEFECT: the twins are disjoint after all — a={a:?} b={b:?}"
    );
    assert!(
        (common - 0.2 * 0.1 * 2.6).abs() < 1e-9,
        "the crossing region is 0.2 x 0.1 x 2.6, got {common}"
    );

    // The reported volume is the naive sum of three walls: the crossing
    // region is counted twice.
    let props = topo::mass_properties(out, tol).expect("props");
    let naive = (6.0 * 4.0 * 4.0 - 5.4 * 3.4 * 3.4)
        + (2.1 * 1.6 * 2.6 - 1.5 * 1.0 * 2.0)
        + (2.1 * 1.6 * 2.6 - 1.5 * 1.0 * 2.0);
    assert!(
        (props.volume - naive).abs() < 1e-9,
        "got {}, naive sum {naive}",
        props.volume
    );
    assert!(
        (props.volume - (naive - common)).abs() > 1e-3,
        "the double count is real"
    );
}

/// **The same class WITHOUT a void.** Two notches cut in from opposite
/// sides of one box, offset diagonally: a single-shell, non-convex
/// operand. The concave faces grow inward exactly as a void's do, the
/// footprint test skips the same two pairs, and the offset planes
/// cross. Placed here so the finding above reads as a PRE-EXISTING gate
/// defect this unit makes generic, not as one this unit introduced.
#[test]
fn r2_the_same_gate_hole_exists_on_a_single_shell_notched_operand() {
    let tol = Tol::witness();
    let outer = boxy(6.0, 4.0, 4.0);
    let one = cut(&outer, &brick(1.0, 2.5, -1.0, 1.8, 1.0, 3.0));
    let body = cut(&one, &brick(2.9, 4.4, 2.3, 5.0, 1.0, 3.0));
    assert_eq!(body.solids().count(), 1);
    assert_eq!(body.shells().count(), 1, "notches, not voids");

    let t = 0.3;
    let shelled = topo::shell(&body, t, tol).expect("MEASURED: the gate lets this through too");
    let out = &shelled.body;
    assert_eq!(topo::validate_geometric(out, tol), Ok(()), "tier 3 green");
    let props = topo::mass_properties(out, tol).expect("props");
    // Erode every plane by t. The two concave end faces (y = 1.8 -> 2.1
    // and y = 2.3 -> 2.0) and the two concave side faces (x = 2.5 ->
    // 2.8 and x = 2.9 -> 2.6) cross over the box
    // x 2.6..2.8, y 2.0..2.1, z 0.7..3.3 — volume 0.052 — which the
    // eroded body subtracts TWICE.
    let operand = 96.0 - 1.5 * 1.8 * 2.0 - 1.5 * 1.7 * 2.0;
    let eroded_naive = 5.4 * 3.4 * 3.4 - 2.1 * 1.8 * 2.6 - 2.1 * 1.7 * 2.6;
    let crossing = 0.2 * 0.1 * 2.6;
    assert!(
        (props.volume - (operand - eroded_naive)).abs() < 1e-9,
        "MEASURED DEFECT: got {}, naive {} (true set {})",
        props.volume,
        operand - eroded_naive,
        operand - eroded_naive - crossing
    );
}

// ---------------------------------------------------------------------
// Claim 2: the curved window on a hollow operand
// ---------------------------------------------------------------------

/// A cylinder of radius `r` spanning `z0..z1`, coaxial with `z`.
fn can(r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, z0), 0.0),
        ProfileVertex::new(p2(r, z0), 0.0),
        ProfileVertex::new(p2(r, z1), 0.0),
        ProfileVertex::new(p2(0.0, z1), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// **A CURVED void wall thinner than `2t`, with every planar wall
/// thicker.** A centred box with a coaxial cylindrical void, grafted
/// through the same door the shell verb uses. `wall_clearance` walks
/// PLANAR faces only, so the 0.2 radial wall between the void's
/// cylinder and the box's sides is never read: the dilated void
/// cylinder (r = 0.95) ends up OUTSIDE the eroded box wall (x = 0.85)
/// and the two thin solids cross.
///
/// This row asserts the DEFECT the module docs describe ("on a hollow
/// operand the window is the whole moved clone"). It goes red when
/// #1055 / SHELL-4's certificate lands.
#[test]
fn r2_a_thin_curved_wall_shells_silently_into_crossing_walls() {
    let tol = Tol::witness();
    let mut body = can(1.0, 0.0, 3.0);
    let solid = body.solids().next().expect("one solid").0;
    let cavity = can(0.8, 1.0, 2.0);
    let evidence = topo::boolean::VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    topo::boolean::VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    };
    topo::boolean::insert_void(&mut body, solid, cavity, &evidence, tol).expect("the void grafts");
    assert_eq!(body.solids().count(), 1);
    assert_eq!(body.shells().count(), 2, "outer plus one cylindrical void");

    let t = 0.15; // 2t = 0.30 > 0.20, the radial wall.
    let shelled = topo::shell(&body, t, tol).expect("MEASURED: it builds silently");
    let out = &shelled.body;
    assert_eq!(topo::validate_geometric(out, tol), Ok(()), "tier 3 green");
    assert_eq!(out.solids().count(), 2);

    let mut radii: Vec<f64> = out
        .faces()
        .filter_map(|(_, f)| match out.get_surface(f.surface) {
            Some(geom::Surface::Cylinder { radius, .. }) => Some(*radius),
            _ => None,
        })
        .collect();
    radii.sort_by(|a, b| a.partial_cmp(b).unwrap());
    radii.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    assert_eq!(radii.len(), 4, "four cylinder radii, got {radii:?}");
    assert!(
        (radii[0] - 0.8).abs() < 1e-12
            && (radii[1] - 0.85).abs() < 1e-12
            && (radii[2] - 0.95).abs() < 1e-12
            && (radii[3] - 1.0).abs() < 1e-12,
        "MEASURED DEFECT: the dilated void wall (0.95) is OUTSIDE the \
         eroded outer wall (0.85), got {radii:?}"
    );

    // The dilated void reaches r = 0.95, past the eroded outer r = 0.85.
    let voids = void_shells(&body);
    let twin_solid = shelled
        .naming
        .thickened
        .iter()
        .find(|(_, s)| voids.contains(s))
        .expect("a thin solid for the void")
        .0;
    let twin = out.get_solid(twin_solid).expect("the thin solid").shells[0];
    let tb = shell_box(out, twin);
    assert!(
        tb[0].1 > 0.85 + 1e-9,
        "MEASURED DEFECT: the dilated void reaches r = {}, past the eroded wall 0.85",
        tb[0].1
    );

    let props = topo::mass_properties(out, tol).expect("props");
    println!("thin-curved-wall volume = {}", props.volume);
}

// ---------------------------------------------------------------------
// Claim 5: is `OperandOuterShells { outer != 1 }` reachable?
// ---------------------------------------------------------------------

/// **The PR's own §5 body, handed to the verb.** `subtract(box 6³,
/// shell(box 2³, 0.25))` yields ONE solid with THREE shells that
/// classify to two `Outer` and one `Void` — the island inside B's
/// cavity filed as a shell of A's solid. The verb refuses it typed,
/// so the variant is reachable from the public doors and is not dead
/// code.
#[test]
fn r2_the_hollow_b_subtraction_reaches_operand_outer_shells() {
    let tol = Tol::witness();
    let inner = topo::shell(&brick(2.0, 4.0, 2.0, 4.0, 2.0, 4.0), 0.25, tol)
        .expect("the small box shells")
        .body;
    let body = cut(&brick(0.0, 6.0, 0.0, 6.0, 0.0, 6.0), &inner);
    assert_eq!(body.solids().count(), 1, "one solid");
    assert_eq!(body.shells().count(), 3, "three shells in it");
    let roles = topo::classify_shells(&body, tol).expect("classifies");
    let outer = roles.iter().filter(|c| c.role == ShellRole::Outer).count();
    assert_eq!(outer, 2, "two Outer shells under one solid");

    let e = topo::shell(&body, 0.05, tol).expect_err("the verb refuses");
    assert!(
        matches!(e, topo::ShellError::OperandOuterShells { outer: 2 }),
        "expected OperandOuterShells {{ outer: 2 }}, got {e}"
    );
}

// ---------------------------------------------------------------------
// The end-to-end exercise: hollow, hollow again, open the inner wall
// ---------------------------------------------------------------------

/// A consumer's seat: hollow a part, hollow it again, then open the
/// inner wall — and measure, mesh and export what comes back.
#[test]
fn r2_e2e_hollow_twice_then_open_the_inner_wall() {
    let tol = Tol::witness();
    let part = boxy(2.0, 3.0, 4.0);
    let once = topo::shell(&part, 0.25, tol).expect("first hollow").body;
    let twice = topo::shell(&once, 0.05, tol).expect("second hollow");
    let body = &twice.body;
    println!(
        "sealed: {} solids, {} shells, volume {}",
        body.solids().count(),
        body.shells().count(),
        topo::mass_properties(body, tol).expect("props").volume
    );

    // Designate the void's ceiling — the inner wall's top.
    let voids = void_shells(&once);
    assert_eq!(voids.len(), 1);
    let ceiling = once
        .get_shell(voids[0])
        .expect("the void")
        .faces
        .iter()
        .copied()
        .find(|f| {
            matches!(
                once.get_surface(once.get_face(*f).expect("a face").surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - 3.75).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
            )
        })
        .expect("the void ceiling at z = 3.75");
    let opened = topo::shell_open(&once, 0.05, &[ceiling], tol).expect("the inner wall opens");
    let out = &opened.body;
    assert_eq!(topo::validate_geometric(out, tol), Ok(()), "tier 3");
    println!(
        "opened: {} solids, {} shells, volume {}, rims {}, thickened {:?}",
        out.solids().count(),
        out.shells().count(),
        topo::mass_properties(out, tol).expect("props").volume,
        opened.naming.rims.len(),
        opened.naming.thickened.len()
    );

    // What can a consumer do with it?
    let meshed = mesh::tessellate(out, 0.05, tol);
    println!(
        "mesh: {:?}",
        meshed
            .as_ref()
            .map(|m| m.patches.len())
            .map_err(|e| format!("{e}"))
    );
    let step = step_export::step_string(
        out,
        &step_export::StepOptions {
            product_name: "twice-hollowed part".into(),
            ..Default::default()
        },
        tol,
    );
    println!(
        "step: {:?}",
        step.as_ref().map(|s| s.len()).map_err(|e| format!("{e}"))
    );
}

// ---------------------------------------------------------------------
// Claim 4: the grouping is pinned; is the PAIRING?
// ---------------------------------------------------------------------

/// **The V ↔ V′ pairing, pinned.** The spec calls the pairing
/// STRUCTURAL (the graft map). Nothing in the acceptance rows reads it:
/// a two-void body whose voids are paired with each OTHER's twin has
/// the same solid count, the same shell count, the same `[Outer, Void]`
/// per solid, the same volume (signed volumes sum the same however the
/// shells are grouped) and the same tier-3 verdict.
///
/// This row reads the pairing itself, through the public record: every
/// face of a thin solid's OTHER shell must be an `inner` twin whose
/// SOURCE face belongs to that solid's own operand void.
#[test]
fn r2_each_thin_solid_pairs_its_own_voids_twin() {
    let tol = Tol::witness();
    let outer = boxy(6.0, 4.0, 4.0);
    let one = cut(&outer, &brick(1.0, 2.2, 1.0, 3.0, 1.0, 3.0));
    let body = cut(&one, &brick(3.8, 5.0, 1.0, 3.0, 1.0, 3.0));
    assert_eq!(body.shells().count(), 3, "outer plus two voids");
    let voids = void_shells(&body);
    assert_eq!(voids.len(), 2);

    let shelled = topo::shell(&body, 0.15, tol).expect("well clear of every wall");
    let out = &shelled.body;
    assert_eq!(out.solids().count(), 3);
    // `naming.inner` rows are (RESULT twin, SOURCE operand face).
    let inner = &shelled.naming.inner;

    for &(solid, shell) in &shelled.naming.thickened {
        if !voids.contains(&shell) {
            continue;
        }
        let own: Vec<topo::FaceKey> = body
            .get_shell(shell)
            .expect("the operand void")
            .faces
            .clone();
        let shells = &out.get_solid(solid).expect("a thin solid").shells;
        assert_eq!(shells.len(), 2, "a thin solid is twin plus void");
        let twin = *shells
            .iter()
            .find(|s| **s != shell)
            .expect("the twin shell");
        for &face in &out.get_shell(twin).expect("the twin").faces {
            let source = inner
                .iter()
                .find(|(t, _)| *t == face)
                .map(|(_, s)| *s)
                .expect("every twin face names a source");
            assert!(
                own.contains(&source),
                "MUTANT-KILLER: thin solid {solid:?} carries a twin face whose source \
                 {source:?} is on another void, not on {shell:?}"
            );
        }
    }
}

// ---------------------------------------------------------------------
// Claim 6: the hole path on a VOID face (the residue file's case)
// ---------------------------------------------------------------------

/// **A void with a pillar through it**, so the void's ceiling is a
/// holed face — the case `work/shell/shell-open-on-a-void-face-with-a-
/// hole.md` records as unmeasured. Built by subtracting a holed slab.
#[test]
fn r2_open_a_holed_void_ceiling() {
    let tol = Tol::witness();
    let outer_lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.0, 1.0), 0.0),
        ProfileVertex::new(p2(5.0, 1.0), 0.0),
        ProfileVertex::new(p2(5.0, 5.0), 0.0),
        ProfileVertex::new(p2(1.0, 5.0), 0.0),
    ]);
    let hole_lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.5, 2.5), 0.0),
        ProfileVertex::new(p2(2.5, 3.5), 0.0),
        ProfileVertex::new(p2(3.5, 3.5), 0.0),
        ProfileVertex::new(p2(3.5, 2.5), 0.0),
    ]);
    let plane = SketchPlane::new(geom_core::Affine3::translation(Vec3::new(0.0, 0.0, 1.0)));
    let profile = Profile::new(plane, vec![outer_lp, hole_lp])
        .validate(tol)
        .expect("a holed rectangle is a valid profile");
    let tool = extrude(&profile, Extrusion::Distance(2.0), tol)
        .expect("the holed rectangle extrudes")
        .body;
    let body = cut(&boxy(6.0, 6.0, 4.0), &tool);
    println!(
        "holed-void operand: {} solids, {} shells",
        body.solids().count(),
        body.shells().count()
    );
    let voids = void_shells(&body);
    println!("voids: {}", voids.len());
    assert_eq!(voids.len(), 1, "one void, with a pillar through it");

    // The void's ceiling at z = 3, holed by the pillar.
    let ceiling = body
        .get_shell(voids[0])
        .expect("the void")
        .faces
        .iter()
        .copied()
        .find(|f| {
            let d = body.get_face(*f).expect("a face");
            !d.rings.is_empty()
                && matches!(
                    body.get_surface(d.surface),
                    Some(geom::Surface::Plane { origin, normal, .. })
                        if (origin.z - 3.0).abs() < 1e-9
                            && normal.x.abs() < 1e-9
                            && normal.y.abs() < 1e-9
                )
        })
        .expect("a holed ceiling at z = 3");

    match topo::shell_open(&body, 0.2, &[ceiling], tol) {
        Ok(opened) => {
            let out = &opened.body;
            println!(
                "MEASURED Ok: {} solids, {} shells, tier3 {:?}, volume {}, holes {}",
                out.solids().count(),
                out.shells().count(),
                topo::validate_geometric(out, tol),
                topo::mass_properties(out, tol).expect("props").volume,
                opened.naming.rims.first().map_or(0, |r| r.holes.len())
            );
        }
        Err(e) => println!("MEASURED refusal: {e}"),
    }
}

// ---------------------------------------------------------------------
// Claim 7: is the OLD (single-shell) domain byte-identical?
// ---------------------------------------------------------------------

/// **A body-level differential over a wider single-shell corpus.** The
/// PR cites `verbs_shell::r2_probe_other_two_passes_dump`, which prints
/// `validate_pseudomanifold` / `contact_marks` VERDICTS over six
/// bodies — not the shelled bodies themselves. This row writes a
/// bit-faithful dump of every shelled body over a corpus that reaches
/// all three offset doors (all-planar, axial, per-chart) and both arms
/// (sealed, opened), so the claim can actually be diffed.
///
/// Unarmed without `BITDUMP_DIR`, exactly as `bitdump.rs`'s rows.
#[test]
fn r2_bitdump_single_shell_shell_corpus() {
    let Some(dir) = std::env::var("BITDUMP_DIR").ok().filter(|d| !d.is_empty()) else {
        return;
    };
    let tol = Tol::witness();
    let mut text = String::new();
    let mut row = |name: &str, body: &Body<f64>, t: f64, open: &[usize]| {
        use std::fmt::Write as _;
        let faces: Vec<topo::FaceKey> = body.faces().map(|(k, _)| k).collect();
        let picked: Vec<topo::FaceKey> = open.iter().map(|&i| faces[i]).collect();
        let _ = writeln!(text, "== {name} t={t} open={open:?} ==");
        match topo::shell_open(body, t, &picked, tol) {
            Ok(s) => {
                let _ = writeln!(text, "{}", crate::bitdump::dump(&s.body));
                let _ = writeln!(
                    text,
                    "naming outer={} inner={} rims={} dead_faces={}",
                    s.naming.outer.len(),
                    s.naming.inner.len(),
                    s.naming.rims.len(),
                    s.naming.dead.faces.len()
                );
                let _ = writeln!(
                    text,
                    "props {:?}",
                    topo::mass_properties(&s.body, tol).map(|p| p.volume)
                );
                let _ = writeln!(text, "tier3 {:?}", topo::validate_geometric(&s.body, tol));
            }
            Err(e) => {
                let _ = writeln!(text, "REFUSED {e}");
            }
        }
    };
    // all-planar door, sealed and opened
    row("boxy", &boxy(2.0, 3.0, 4.0), 0.25, &[]);
    row("boxy_open0", &boxy(2.0, 3.0, 4.0), 0.25, &[0]);
    row("boxy_open01", &boxy(2.0, 3.0, 4.0), 0.2, &[0, 1]);
    row("brick_thin", &brick(0.0, 1.0, 0.0, 1.0, 0.0, 5.0), 0.1, &[]);
    // an L prism — a concave outer shell, the per-chart corner class
    let l = {
        let pts = [
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 1.5),
            (1.5, 1.5),
            (1.5, 4.0),
            (0.0, 4.0),
        ];
        let lp = ProfileLoop::new(
            pts.iter()
                .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("an L is a valid profile");
        extrude(&profile, Extrusion::Distance(2.0), tol)
            .expect("an L extrudes")
            .body
    };
    row("l_prism", &l, 0.2, &[]);
    row("l_prism_open0", &l, 0.2, &[0]);
    // an oblique prism — the simultaneous planar door's own class
    let oblique = {
        let pts = [(0.0, 0.0), (3.0, 0.0), (2.0, 2.0), (0.4, 1.6)];
        let lp = ProfileLoop::new(
            pts.iter()
                .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("valid");
        extrude(&profile, Extrusion::Distance(2.0), tol)
            .expect("extrudes")
            .body
    };
    row("oblique", &oblique, 0.15, &[]);
    // axial door
    row("vessel", &can(1.0, 0.0, 2.0), 0.2, &[]);
    row("vessel_open0", &can(1.0, 0.0, 2.0), 0.2, &[0]);
    row("vessel_thin", &can(0.4, 0.0, 3.0), 0.05, &[]);
    std::fs::create_dir_all(&dir).expect("the dump dir");
    std::fs::write(format!("{dir}/shell5_r2_single_shell.txt"), &text).expect("write");
}

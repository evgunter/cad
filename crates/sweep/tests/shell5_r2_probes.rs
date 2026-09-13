//! **SHELL-5 review probes (R2).** Falsification rows for the claims
//! the unit makes about a hollow operand: that the clearance gate is
//! sufficient on planar operands, and that the curved window is
//! documented as exactly what it is.
//!
//! Rows here assert MEASURED behaviour, including behaviour that is
//! wrong. Each such row says so on its face and names what would have
//! to land for it to go red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, LoopBoundary, ShellError, ShellKey, ShellRole};

use crate::verbs_shell::{boxy, brick, cut, two_void_box};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
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

// ---------------------------------------------------------------------
// Claim 3: `wall_clearance` is NOT sufficient on planar hollow operands
// ---------------------------------------------------------------------

/// **Two voids offset DIAGONALLY.** Facing walls are `g` apart with
/// `g < 2t`, and the facing faces' projected footprints are disjoint
/// on the OPERAND — but the dilated twins grow by `t` on every side
/// and would interpenetrate, so the gate grows each footprint by `t`
/// before the separation test and refuses the pair. (R2's row; written
/// when the gate read the operand's footprints, built this, and
/// counted the `0.2 × 0.1 × 2.6` crossing twice in the volume.)
#[test]
fn r2_diagonal_voids_refuse_at_the_grown_footprint_gate() {
    let tol = Tol::witness();
    let outer = boxy(6.0, 4.0, 4.0);
    // A: x 1.0..2.5, y 0.8..1.8   B: x 2.9..4.4, y 2.3..3.3, both z 1..3.
    let one = cut(&outer, &brick(1.0, 2.5, 0.8, 1.8, 1.0, 3.0));
    let body = cut(&one, &brick(2.9, 4.4, 2.3, 3.3, 1.0, 3.0));
    assert_eq!(body.solids().count(), 1, "one solid");
    assert_eq!(body.shells().count(), 3, "outer plus two voids");
    let voids = void_shells(&body);
    assert_eq!(voids.len(), 2);

    // The facing walls are 0.4 apart in x with footprints 0.5 apart in
    // y; 2t = 0.6 exceeds both, so the grown footprints overlap and
    // the gap is short.
    let t = 0.3;
    let e = topo::shell(&body, t, tol).expect_err("the diagonal pair is read as facing");
    let ShellError::WallClearance {
        face, other, gap, ..
    } = e
    else {
        panic!("expected the wall-clearance gate, got {e}");
    };
    assert!(
        (gap - 0.4).abs() < 1e-9,
        "the facing walls are 0.4 apart, got {gap}"
    );
    let shell_of = |f: topo::FaceKey| body.get_face(f).expect("an operand face").shell;
    assert!(voids.contains(&shell_of(face)) && voids.contains(&shell_of(other)));
    assert_ne!(shell_of(face), shell_of(other), "one face of each void");
}

/// **The same class WITHOUT a void.** Two notches cut in from opposite
/// sides of one box, offset diagonally: a single-shell, non-convex
/// operand. The concave faces grow inward exactly as a void's do, so
/// the grown-footprint gate reads the same two pairs as facing and
/// refuses. Placed here so the class reads as a PRE-EXISTING gate
/// defect this unit made generic and then closed, not one it
/// introduced. (R2's row; at the merge base this built and subtracted
/// the `0.052` crossing twice.)
#[test]
fn r2_the_same_gate_hole_is_closed_on_a_single_shell_notched_operand() {
    let tol = Tol::witness();
    let outer = boxy(6.0, 4.0, 4.0);
    let one = cut(&outer, &brick(1.0, 2.5, -1.0, 1.8, 1.0, 3.0));
    let body = cut(&one, &brick(2.9, 4.4, 2.3, 5.0, 1.0, 3.0));
    assert_eq!(body.solids().count(), 1);
    assert_eq!(body.shells().count(), 1, "notches, not voids");

    let t = 0.3;
    let e = topo::shell(&body, t, tol).expect_err("the notches' concave faces are read as facing");
    assert!(matches!(e, ShellError::WallClearance { .. }), "got {e}");
    // Below the wall it builds: at t = 0.1 the notch walls (0.4 apart
    // in x, footprints 0.5 apart in y) clear.
    let s = topo::shell(&body, 0.1, tol).expect("clear of every wall");
    assert_eq!(topo::validate_geometric(&s.body, tol), Ok(()));
    let props = topo::mass_properties(&s.body, tol).expect("props");
    // Erode every plane by t: the box to 5.8 × 3.8 × 3.8, each notch
    // grown by t on its three material sides and running from the
    // eroded outer face it opens on (notch 1: x 0.9..2.6, y 0.1..1.9;
    // notch 2: x 2.8..4.5, y 2.2..3.9; both z 0.9..3.1).
    let operand = 96.0 - 1.5 * 1.8 * 2.0 - 1.5 * 1.7 * 2.0;
    let eroded = 5.8 * 3.8 * 3.8 - 1.7 * 1.8 * 2.2 - 1.7 * 1.7 * 2.2;
    assert!(
        (props.volume - (operand - eroded)).abs() < 1e-9,
        "got {}, want {}",
        props.volume,
        operand - eroded
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
/// operand the window is the whole moved clone") and is SELF-RETIRING:
/// it goes red the day SHELL-4's clearance certificate (#1055) refuses
/// a curved thin wall, and is rewritten as that refusal then. R2's
/// row; R1 measured the same class on a revolved vessel with a
/// cylindrical cavity (`r = 0.9`, `t = 0.08`: twins at 0.92 and 0.98,
/// crossed, tier 3 green).
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
/// code. (R2's row; R1's `r1p5` measured the same body.)
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
        matches!(e, topo::ShellError::OperandOuterShells { outer: 2, .. }),
        "expected OperandOuterShells {{ outer: 2 }}, got {e}"
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
/// SOURCE face belongs to that solid's own operand void. MUTANT-KILLER
/// (R2's row): pairing each void with the NEXT void's twin passes every
/// other row in the tree and reds here.
#[test]
fn r2_each_thin_solid_pairs_its_own_voids_twin() {
    let tol = Tol::witness();
    let (body, _) = two_void_box();
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
// Claim 8: a precondition the new door does NOT check
// ---------------------------------------------------------------------

/// **`move_shells_to_new_solid` will mint a solid with NO outer
/// boundary**, and nothing downstream objects. Its five preconditions
/// are all structural (resolve, one solid, non-empty remainder); none
/// is about the shells forming a coherent piece of material. Moving a
/// hollow box's VOID out on its own leaves one solid whose only shell
/// has negative signed volume, and tier 3 passes it.
///
/// The shell verb never does this — it always moves the pair — but the
/// door is `pub` on `Body`, and its own docs claim only "tier-1
/// preservation". Measured here so the boundary of what it guarantees
/// is on the page.
#[test]
fn r2_the_new_door_mints_a_solid_with_no_outer_shell() {
    let tol = Tol::witness();
    let mut body = topo::shell(&boxy(2.0, 3.0, 4.0), 0.25, tol)
        .expect("the box hollows")
        .body;
    let voids = void_shells(&body);
    assert_eq!(voids.len(), 1);
    let minted = body
        .move_shells_to_new_solid(&[voids[0]])
        .expect("MEASURED: the door accepts a lone void");
    assert_eq!(body.solids().count(), 2);
    assert_eq!(
        body.get_solid(minted).expect("the minted solid").shells,
        vec![voids[0]]
    );
    assert_eq!(
        topo::validate_geometric(&body, tol),
        Ok(()),
        "MEASURED: tier 3 passes a solid whose only shell is a cavity"
    );
    let roles = topo::classify_shells(&body, tol).expect("classifies");
    let minted_roles: Vec<ShellRole> = roles
        .iter()
        .filter(|c| c.solid == minted)
        .map(|c| c.role)
        .collect();
    assert_eq!(
        minted_roles,
        vec![ShellRole::Void],
        "the minted solid is all cavity and no material"
    );
}

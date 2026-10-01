//! **SHELL-8 review R2 — execution probes.** Each row falsifies (or
//! fails to falsify) one claim of PR #2207 by running it. Rows print
//! `[r2]` lines; the assertions are the verdicts.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use core::f64::consts::PI;

use geom_core::k_stats::Bracket;
use geom_core::{Affine3, Point3, Sign, Tol, Vec3};
use sweep::test_support::{block, brick};
use topo::ShellNaming;
use topo::{Body, FaceKey, ShellError, ShellRole, SolidKey, VoidContainment, VoidEvidence};

use crate::common::approx::band;
use crate::common::charts::{charts_of, moves_inward};
use crate::common::oracles::box_volume;
use crate::common::shell_operands::{hollow_box, outer_and_void, vessel};
use crate::shell8_common::{beside, beside_raw, cap, faces_of, solid_of, tol, volume, wearers};

fn points(body: &Body<f64>) -> Vec<(topo::VertexKey, (u64, u64, u64))> {
    body.vertices()
        .map(|(k, v)| {
            let p = body.get_point(v.point).unwrap();
            (k, (p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
        })
        .collect()
}

fn roles(body: &Body<f64>) -> Vec<(topo::ShellKey, ShellRole)> {
    topo::classify_shells(body, tol())
        .expect("classifies")
        .into_iter()
        .map(|c| (c.shell, c.role))
        .collect()
}

/// Compare the bit patterns of every vertex of `solid` between two
/// bodies that share the key space (a clone and its moved copy).
fn bitwise_solid(before: &Body<f64>, after: &Body<f64>, solid: SolidKey) -> (usize, usize) {
    let a = points(before);
    let b = points(after);
    assert_eq!(a.len(), b.len(), "no vertex minted or killed");
    let owners = topo::SolidOwners::of(before);
    let (mut same, mut moved) = (0, 0);
    for ((k, pa), (k2, pb)) in a.iter().zip(b.iter()) {
        assert_eq!(k, k2, "arena order kept");
        if owners.vertex(*k).expect("every vertex has an owning solid") != solid {
            continue;
        }
        if pa == pb {
            same += 1;
        } else {
            moved += 1;
        }
    }
    (same, moved)
}

// ---------------------------------------------------------------------
// Claim 1 — a scoped simultaneous door reads and writes nothing outside
// the named solids: the AXIAL door, and a two-of-three scope.
// ---------------------------------------------------------------------

#[test]
fn r2_axial_door_scoped_to_the_vessel_leaves_the_box_bitwise() {
    let t = 0.05;
    let pair = beside(
        &block(2.0, 3.0, 4.0, Tol::witness()),
        &vessel(1.0, 2.0),
        10.0,
    );
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let (bx, vs) = (solids[0], solids[1]);

    // Whole-body reading: not axial (the box is in it).
    assert!(!topo::is_axial(&pair, band()).unwrap());

    // The vessel's charts only, through the axial door.
    let mut work = pair.clone();
    topo::offset_charts_together(
        &mut work,
        &moves_inward(&pair, charts_of(&pair, vs), t),
        band(),
        tol(),
    )
    .expect("the vessel's charts move through the axial door while the box stands by");
    let (same, moved) = bitwise_solid(&pair, &work, bx);
    println!("[r2] axial door scoped to vessel: box vertices same={same} moved={moved}");
    assert_eq!(moved, 0, "the box moved");
    assert_eq!(same, 8);
    let (_, vmoved) = bitwise_solid(&pair, &work, vs);
    assert!(vmoved > 0, "the vessel did not move");
    assert_eq!(topo::validate_geometric(&work, tol()), Ok(()));

    // The box's charts only, through the planar door — the vessel's
    // curved faces are OUT of scope and must not trip `TogetherNonPlanar`.
    let mut work = pair.clone();
    let r = topo::offset_planes_together(
        &mut work,
        &moves_inward(&pair, charts_of(&pair, bx), t),
        band(),
        tol(),
    );
    println!("[r2] planar door scoped to box beside a vessel: {r:?}");
    r.expect("the box's charts move through the planar door while the vessel stands by");
    let (same, moved) = bitwise_solid(&pair, &work, vs);
    assert_eq!(moved, 0, "the vessel moved: same={same}");
    assert_eq!(topo::validate_geometric(&work, tol()), Ok(()));
}

#[test]
fn r2_scope_naming_two_of_three_solids_leaves_the_third_bitwise() {
    let t = 0.05;
    let two = beside(
        &block(2.0, 3.0, 4.0, Tol::witness()),
        &block(2.0, 3.0, 4.0, Tol::witness()),
        10.0,
    );
    let three = beside(&two, &block(2.0, 3.0, 4.0, Tol::witness()), 20.0);
    let solids: Vec<SolidKey> = three.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 3);
    let mut moves = moves_inward(&three, charts_of(&three, solids[0]), t);
    moves.extend(moves_inward(&three, charts_of(&three, solids[2]), t));
    let mut work = three.clone();
    topo::offset_planes_together(&mut work, &moves, band(), tol()).expect("two of three move");
    let mid = bitwise_solid(&three, &work, solids[1]);
    let a = bitwise_solid(&three, &work, solids[0]);
    let c = bitwise_solid(&three, &work, solids[2]);
    println!("[r2] two of three: first={a:?} middle={mid:?} third={c:?}");
    assert_eq!(mid, (8, 0));
    assert_eq!(a, (0, 8));
    assert_eq!(c, (0, 8));

    // Two of three named, one of them in part: refuses naming the face.
    let mut partial = moves_inward(&three, charts_of(&three, solids[0]), t);
    partial.extend(moves_inward(&three, charts_of(&three, solids[2]), t));
    partial.pop();
    let mut work = three.clone();
    let e = topo::offset_planes_together(&mut work, &partial, band(), tol()).unwrap_err();
    assert!(
        matches!(e, topo::ReplaceFaceError::TogetherPartialSet { face } if solid_of(&three, face) == solids[2]),
        "{e}"
    );
}

// ---------------------------------------------------------------------
// Claim 2 — cross-solid pairs never gate: a solid INSIDE another solid's
// void, close to the void wall.
// ---------------------------------------------------------------------

#[test]
fn r2_a_solid_inside_anothers_void_shells_and_never_gates() {
    let t = 0.05;
    let hollow = hollow_box(); // void [0.25,1.75]×[0.25,2.75]×[0.25,3.75]
    // A 0.5-cube whose top sits 0.02 (< 2t) below the void's ceiling.
    let inner = block(0.5, 0.5, 0.5, Tol::witness());
    let (body, inner_solid) = beside_raw(&hollow, &inner, Vec3::new(0.75, 1.25, 3.75 - 0.02 - 0.5));
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Ok(()),
        "a solid inside a void is a valid body"
    );
    println!("[r2] roles of the nested operand: {:?}", roles(&body));
    let s = topo::shell(&body, t, tol())
        .expect("both shell; the void wall and the cube face never gate");
    let want = (box_volume(2.0, 3.0, 4.0) - box_volume(1.9, 2.9, 3.9))
        + (box_volume(1.6, 2.6, 3.6) - box_volume(1.5, 2.5, 3.5))
        + (box_volume(0.5, 0.5, 0.5) - box_volume(0.4, 0.4, 0.4));
    println!(
        "[r2] nested: solids={} shells={} volume={} want={want} thickened={:?}",
        s.body.solids().count(),
        s.body.shells().count(),
        volume(&s.body),
        s.naming.thickened
    );
    assert_eq!(topo::validate_geometric(&s.body, tol()), Ok(()));
    assert_eq!(s.body.solids().count(), 3);
    assert_eq!(s.body.shells().count(), 6);
    assert!((volume(&s.body) - want).abs() < 1e-12);
    assert!(
        s.naming
            .thickened
            .iter()
            .any(|(owner, _)| *owner == inner_solid)
    );

    // Same-solid control: a cube of the SAME size standing beside the
    // hollow box 0.02 from its outer wall never gates either (cross
    // solid) — but a 0.02 wall INSIDE one solid does.
    let (thin, _) = beside_raw(
        &block(2.0, 3.0, 4.0, Tol::witness()),
        &block(0.5, 0.5, 0.5, Tol::witness()),
        Vec3::new(2.02, 0.0, 0.0),
    );
    topo::shell(&thin, t, tol()).expect("cross-solid 0.02 gap builds");
}

// ---------------------------------------------------------------------
// Claim 3 — the roles read: once per HOLLOW solid over that solid's own
// shells, nothing for a plain one; the outer count is the solid's own.
// ---------------------------------------------------------------------

#[test]
fn r2_roles_are_read_per_hollow_solid_and_never_for_a_plain_one() {
    let pair = beside(&hollow_box(), &block(2.0, 3.0, 4.0, Tol::witness()), 10.0);
    let bracket = Bracket::open();
    topo::shell(&pair, 0.05, tol()).expect("shells");
    let verdicts = bracket.finish().verdicts;
    let signs = verdicts
        .iter()
        .filter(|v| v.predicate == "chk_shell_volume_sign")
        .count();
    println!("[r2] hollow+plain: chk_shell_volume_sign verdicts = {signs}");
    assert_eq!(
        signs, 2,
        "only the HOLLOW solid's two shells are classified; the plain \
         neighbour's is its boundary by arity and is never read"
    );

    let plain = beside(
        &block(2.0, 3.0, 4.0, Tol::witness()),
        &block(2.0, 3.0, 4.0, Tol::witness()),
        10.0,
    );
    let bracket = Bracket::open();
    topo::shell(&plain, 0.05, tol()).expect("shells");
    let verdicts = bracket.finish().verdicts;
    let signs = verdicts
        .iter()
        .filter(|v| v.predicate == "chk_shell_volume_sign")
        .count();
    println!("[r2] plain+plain: chk_shell_volume_sign verdicts = {signs}");
    assert_eq!(
        signs, 0,
        "a body of only single-shell solids reads no roles"
    );
}

#[test]
fn r2_operand_outer_shells_names_the_offending_solids_own_count() {
    // A solid with TWO outer shells: a positively oriented cube inserted
    // through the void door (which trusts carried evidence and reverts
    // the cavity — so a pre-reverted cavity lands positive).
    let mut host = block(2.0, 3.0, 4.0, Tol::witness());
    let host_solid = host.solids().next().unwrap().0;
    let cube = topo::transform_rigid(
        &block(0.5, 0.5, 0.5, Tol::witness()),
        &Affine3::translation(Vec3::new(0.75, 1.25, 1.75)),
        tol(),
    )
    .unwrap();
    let pre_reverted = cube.revert().expect("reverts");
    let evidence = VoidEvidence {
        shells: pre_reverted
            .shells()
            .map(|(s, _)| {
                (
                    s,
                    VoidContainment::Carried {
                        sign: Sign::Positive,
                    },
                )
            })
            .collect(),
    };
    topo::insert_void(&mut host, host_solid, pre_reverted, &evidence).expect("inserts");
    println!("[r2] two-outer host roles: {:?}", roles(&host));
    let (body, _) = beside_raw(
        &host,
        &block(2.0, 3.0, 4.0, Tol::witness()),
        Vec3::new(10.0, 0.0, 0.0),
    );
    let e = topo::shell(&body, 0.05, tol()).expect_err("two outer shells in one solid refuse");
    println!("[r2] two-outer beside plain: {e}");
    // **The count is that SOLID's own, and the refusal names it.** The
    // roles are read per hollow solid, so the plain neighbour's shell
    // is never classified: a whole-body read would have counted three
    // outer shells here and named none of them.
    let host_solid_in_body = body.solids().next().unwrap().0;
    assert!(
        matches!(
            e,
            ShellError::OperandOuterShells { solid, outer: 2 } if solid == host_solid_in_body
        ),
        "{e}"
    );
}

// ---------------------------------------------------------------------
// Claim 5 — a public producer puts one chart on two components: a
// boolean that cuts one slab in two keeps the operand's surface keys on
// both components and files them under ONE solid as two outer shells
// (the ownership door then files them as two solids, below).
// ---------------------------------------------------------------------

/// The un-moved split slab is one solid with two OUTER shells, which
/// the shell door refuses whole and opened, upstream of any chart
/// grouping — whichever of the shared top's wearers is named.
#[test]
fn r2_a_disconnecting_subtract_files_two_outer_shells_the_shell_door_refuses() {
    let body = split_slab(false);
    let solid = body.solids().next().unwrap().0;
    let outer_shells = |e: &ShellError<f64>| matches!(e, ShellError::OperandOuterShells { solid: s, outer: 2 } if *s == solid);
    let e = topo::shell(&body, 0.05, tol()).unwrap_err();
    assert!(outer_shells(&e), "closed: {e}");
    let tops = plane_face_at_z(&body, solid, 1.0);
    assert_eq!(tops.len(), 2, "one top wearer per component");
    for open in [&tops[..1], &tops[1..], &tops[..]] {
        let e = topo::shell_open(&body, 0.05, open, tol()).unwrap_err();
        assert!(outer_shells(&e), "opened at {open:?}: {e}");
    }
}

// ---------------------------------------------------------------------
// Claim 6 — the lift's door is the designated face's solid's, read on
// the result: the hollow vessel opened at its void ceiling, alone and
// beside a box.
// ---------------------------------------------------------------------

fn hollow_vessel_want(t1: f64, t: f64, r: f64, h: f64, lid: bool) -> f64 {
    let cyl = |r: f64, h: f64| PI * r * r * h;
    let a = cyl(r, h) - cyl(r - t, h - 2.0 * t);
    let (rv, hv) = (r - t1, h - 2.0 * t1);
    let b = cyl(rv + t, hv + 2.0 * t) - cyl(rv, hv);
    a + b - if lid { cyl(rv + t, t) } else { 0.0 }
}

#[test]
fn r2_lift_on_the_vessels_void_ceiling_alone_and_beside_a_box() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let (t1, t) = (0.1, 0.02);
    let hv = topo::shell(&vessel(1.0, 2.0), t1, tol()).unwrap().body;
    let (_, void) = outer_and_void(&hv);
    let ceiling = cap(&hv, void, y, 2.0 - t1);

    let alone = topo::shell_open(&hv, t, &ceiling, tol()).expect("the void ceiling opens");
    let props = topo::mass_properties(&alone.body, tol()).unwrap();
    let want = hollow_vessel_want(t1, t, 1.0, 2.0, true);
    println!(
        "[r2] hollow vessel opened on its void ceiling: solids={} shells={} volume={} want={want} pad={}",
        alone.body.solids().count(),
        alone.body.shells().count(),
        props.volume,
        props.volume_pad
    );
    assert_eq!(topo::validate_geometric(&alone.body, tol()), Ok(()));
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
    assert_eq!(alone.body.solids().count(), 2);
    assert_eq!(alone.body.shells().count(), 3);

    // Beside a box: the lift's door must be the vessel's, and the box
    // must be bitwise the sealed result's box.
    let pair = beside(&block(2.0, 3.0, 4.0, Tol::witness()), &hv, 10.0);
    let box_solid = pair.solids().next().unwrap().0;
    let (_, void) = {
        // The vessel's void in the pair: the shell with a Void role.
        let rs = roles(&pair);
        let voids: Vec<_> = rs
            .iter()
            .filter(|(_, r)| *r == ShellRole::Void)
            .map(|(s, _)| *s)
            .collect();
        assert_eq!(voids.len(), 1);
        (rs, voids[0])
    };
    let ceiling = cap(&pair, void, y, 2.0 - t1);
    let sealed = topo::shell(&pair, t, tol()).expect("sealed");
    let opened =
        topo::shell_open(&pair, t, &ceiling, tol()).expect("opened on the vessel's void ceiling");
    // By OPERAND key: the box's entities survive under their keys in
    // both results, while the rim surgery kills vertices elsewhere.
    let owners = topo::SolidOwners::of(&pair);
    let (mut same, mut moved) = (0, 0);
    for (k, _) in pair.vertices() {
        if owners.vertex(k).expect("every vertex has an owning solid") != box_solid {
            continue;
        }
        let p = |b: &Body<f64>| {
            let pt = b.get_point(b.get_vertex(k).unwrap().point).unwrap();
            (pt.x.to_bits(), pt.y.to_bits(), pt.z.to_bits())
        };
        if p(&sealed.body) == p(&opened.body) {
            same += 1
        } else {
            moved += 1
        }
    }
    println!("[r2] box beside opened vessel: box vertices same={same} moved={moved}");
    assert_eq!((same, moved), (8, 0));
    let props = topo::mass_properties(&opened.body, tol()).unwrap();
    let want = (box_volume(2.0, 3.0, 4.0)
        - box_volume(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t))
        + hollow_vessel_want(t1, t, 1.0, 2.0, true);
    println!(
        "[r2] box beside opened vessel: solids={} shells={} volume={} want={want}",
        opened.body.solids().count(),
        opened.body.shells().count(),
        props.volume
    );
    assert_eq!(topo::validate_geometric(&opened.body, tol()), Ok(()));
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
    assert_eq!(opened.body.solids().count(), 3);
    assert_eq!(opened.body.shells().count(), 5);
    assert_eq!(opened.naming.rims.len(), 1);
    assert_eq!(opened.naming.rims[0].side, topo::RimShell::Void);
}

// ---------------------------------------------------------------------
// Claim 9 — NoSolid carries no count and is the only refusal on arity.
// ---------------------------------------------------------------------

#[test]
fn r2_no_solid_is_only_the_empty_operand() {
    let empty: Body<f64> = Body::new();
    let e = topo::shell_open(&empty, 0.05, &[], tol()).unwrap_err();
    assert!(matches!(e, ShellError::NoSolid));
    println!("[r2] NoSolid display: {e}");
    // Three solids build.
    let two = beside(
        &block(2.0, 3.0, 4.0, Tol::witness()),
        &block(2.0, 3.0, 4.0, Tol::witness()),
        10.0,
    );
    let three = beside(&two, &block(2.0, 3.0, 4.0, Tol::witness()), 20.0);
    let s = topo::shell(&three, 0.05, tol()).expect("three build");
    assert_eq!(s.body.solids().count(), 3);
}

// ---------------------------------------------------------------------
// Claim 4 — the solid-order assertion on a body whose solid arena has a
// freed slot, and after a graft that reuses it.
// ---------------------------------------------------------------------

#[test]
fn r2_solid_order_assertion_on_a_body_with_a_freed_solid_slot() {
    let mut body = block(2.0, 3.0, 4.0, Tol::witness());
    let lone = body
        .mvfs(Point3::new(50.0, 50.0, 50.0), true)
        .expect("a lone-vertex solid");
    let (mut body, third) = beside_raw(
        &body,
        &block(2.0, 3.0, 4.0, Tol::witness()),
        Vec3::new(10.0, 0.0, 0.0),
    );
    let killed = body.kvfs(lone.solid).expect("the lone solid dies");
    assert_eq!(killed.killed_solid, lone.solid);
    let order: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    println!("[r2] freed slot: solid order {order:?} (third = {third:?})");
    assert_eq!(order.len(), 2);
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let s = topo::shell(&body, 0.05, tol()).expect("a freed slot in the arena is not a reorder");
    assert_eq!(s.body.solids().count(), 2);
    let one_wall = box_volume(2.0, 3.0, 4.0) - box_volume(1.9, 2.9, 3.9);
    assert!((volume(&s.body) - 2.0 * one_wall).abs() < 1e-12);

    // A graft that REUSES the freed slot (a key with version 2 in the
    // middle of the arena): still one order, still builds.
    let (body, reused) = beside_raw(
        &body,
        &block(2.0, 3.0, 4.0, Tol::witness()),
        Vec3::new(20.0, 0.0, 0.0),
    );
    let order: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    println!("[r2] reused slot: solid order {order:?} (reused = {reused:?})");
    let s = topo::shell(&body, 0.05, tol()).expect("builds");
    assert_eq!(s.body.solids().count(), 3);
    assert!((volume(&s.body) - 3.0 * one_wall).abs() < 1e-12);
}

// ---------------------------------------------------------------------
// The end-to-end exercise, from a consumer's seat: public doors only.
// ---------------------------------------------------------------------

/// The result face that `source` became in `naming` (the inner twin),
/// found by scanning the rows — there is no reverse index.
fn twin_of(naming: &ShellNaming, source: FaceKey) -> FaceKey {
    naming
        .inner
        .iter()
        .find(|(_, s)| *s == source)
        .map(|(t, _)| *t)
        .expect("the source face has an inner twin")
}

/// The planar face of `solid` normal to `axis` at `value` — the
/// consumer's own way to name a wall, since the record has no
/// solid-scoped face query.
fn wall(body: &Body<f64>, solid: SolidKey, axis: Vec3<f64>, value: f64) -> FaceKey {
    for face in faces_of(body, solid) {
        let f = body.get_face(face).unwrap();
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        if normal.cross(axis).norm() < 1e-9
            && (Vec3::new(origin.x, origin.y, origin.z).dot(axis) - value).abs() < 1e-9
        {
            return face;
        }
    }
    panic!("no wall of {solid:?} normal to {axis:?} at {value}")
}

fn tess(label: &str, body: &Body<f64>) {
    let m = mesh::tessellate(body, 1e-3, tol()).expect("tessellates");
    let tris: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
    println!(
        "[r2] e2e {label}: tessellated positions={} patches={} triangles={tris}",
        m.positions.len(),
        m.patches.len()
    );
    assert!(tris > 0);
}

#[test]
fn r2_e2e_consumer_seat() {
    let (z, y) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0));
    let one_wall = |t: f64| {
        box_volume(2.0, 3.0, 4.0) - box_volume(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t)
    };

    // ---- 1. Two parts in one body, hollowed in one call. ----
    let part_a = block(2.0, 3.0, 4.0, Tol::witness());
    let part_b = vessel(1.0, 2.0);
    let mut assembly = part_a.clone();
    let placed = topo::transform_rigid(
        &part_b,
        &Affine3::translation(Vec3::new(10.0, 0.0, 0.0)),
        tol(),
    )
    .unwrap();
    let b_solid = topo::graft_disjoint(&mut assembly, &placed).expect("placed");
    let a_solid = assembly.solids().next().unwrap().0;
    let hollowed = topo::shell(&assembly, 0.05, tol()).expect("both parts hollow in one call");
    let want = one_wall(0.05) + PI * (1.0 * 2.0 - 0.95 * 0.95 * 1.9);
    let props = topo::mass_properties(&hollowed.body, tol()).unwrap();
    println!(
        "[r2] e2e 1: solids={} shells={} volume={} want={want}",
        hollowed.body.solids().count(),
        hollowed.body.shells().count(),
        props.volume
    );
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
    let rs = roles(&hollowed.body);
    println!(
        "[r2] e2e 1: roles {rs:?}; thickened {:?}",
        hollowed.naming.thickened
    );
    assert_eq!(rs.iter().filter(|(_, r)| *r == ShellRole::Void).count(), 2);
    tess("1 (two hollow parts)", &hollowed.body);

    // ---- 2. Hollow, hollow, open on a box. ----
    let first = topo::shell(&part_a, 0.25, tol()).unwrap();
    let second = topo::shell(&first.body, 0.05, tol()).unwrap();
    // Naming "the ceiling of the innermost cavity": the operand's top
    // face → its inner twin after the first hollowing (a void face,
    // whose key SURVIVES the second hollowing under the same key).
    let top = wall(&part_a, part_a.solids().next().unwrap().0, z, 4.0);
    let inner_ceiling = twin_of(&first.naming, top);
    assert!(
        second.body.get_face(inner_ceiling).is_some(),
        "the void face survives under its key"
    );
    // Which thin solid holds it now, through the record: thickened is
    // keyed by OPERAND SHELL, so face → shell → row.
    let its_shell = first.body.get_face(inner_ceiling).unwrap().shell;
    let (owner, _) = second
        .naming
        .thickened
        .iter()
        .find(|(_, s)| *s == its_shell)
        .unwrap();
    println!(
        "[r2] e2e 2: the inner ceiling {inner_ceiling:?} sits in thin solid {owner:?} of {:?}",
        second.body.solids().map(|(k, _)| k).collect::<Vec<_>>()
    );
    let third = topo::shell_open(&second.body, 0.01, &[inner_ceiling], tol())
        .expect("hollow, hollow, open");
    let want = (box_volume(2.0, 3.0, 4.0) - box_volume(1.98, 2.98, 3.98))
        + (box_volume(1.92, 2.92, 3.92) - box_volume(1.9, 2.9, 3.9))
        + (box_volume(1.6, 2.6, 3.6) - box_volume(1.58, 2.58, 3.58))
        + (box_volume(1.52, 2.52, 3.52) - box_volume(1.5, 2.5, 3.5))
        - 1.52 * 2.52 * 0.01;
    println!(
        "[r2] e2e 2: solids={} shells={} volume={} want={want}",
        third.body.solids().count(),
        third.body.shells().count(),
        volume(&third.body)
    );
    assert!((volume(&third.body) - want).abs() < 1e-12);
    assert_eq!(topo::validate_geometric(&third.body, tol()), Ok(()));
    tess("2 (hollow, hollow, open)", &third.body);
    // The OTHER way to say "inner wall": the dilated twin of that
    // ceiling — the OUTER face of the innermost thin solid.
    let outer_of_innermost = twin_of(&second.naming, inner_ceiling);
    let alt = topo::shell_open(&second.body, 0.01, &[outer_of_innermost], tol())
        .expect("opens on the outer side too");
    // That twin is the ceiling of S2's OUTER shell (1.6 × 2.6 × 3.6), so
    // this is an OUTER designation and the lid is the cavity footprint.
    let want_alt = want + 1.52 * 2.52 * 0.01 - 1.58 * 2.58 * 0.01;
    println!(
        "[r2] e2e 2': opened on the twin instead: volume={} want={want_alt}",
        volume(&alt.body)
    );
    assert!((volume(&alt.body) - want_alt).abs() < 1e-12);

    // ---- 3. Vessel beside a box, hollowed, then opened on the
    // vessel's inner wall. ----
    // FRICTION, measured: the vessel's cap is a full revolve's — two
    // half-disc faces on one chart — and designating ONE of them refuses
    // `OpenFaceChartPartial`. The consumer has to know to widen a face
    // to its chart and map each face through the record.
    let b_cap = wall(&assembly, b_solid, y, 2.0);
    let one_face_only = twin_of(&hollowed.naming, b_cap);
    let e = topo::shell_open(&hollowed.body, 0.02, &[one_face_only], tol()).unwrap_err();
    println!("[r2] e2e 3: one half-disc of the vessel's inner ceiling: {e}");
    assert!(matches!(e, ShellError::OpenFaceChartPartial { .. }));
    let chart = assembly.get_face(b_cap).unwrap().surface;
    let b_inner_ceiling: Vec<FaceKey> = wearers(&assembly, b_solid, chart)
        .into_iter()
        .map(|k| twin_of(&hollowed.naming, k))
        .collect();
    let opened = topo::shell_open(&hollowed.body, 0.02, &b_inner_ceiling, tol())
        .expect("opened on the vessel's inner wall");
    // BOTH solids shell again: the box's thin solid becomes two more.
    let want = (box_volume(2.0, 3.0, 4.0) - box_volume(1.96, 2.96, 3.96))
        + (box_volume(1.94, 2.94, 3.94) - box_volume(1.9, 2.9, 3.9))
        + hollow_vessel_want(0.05, 0.02, 1.0, 2.0, true);
    let props = topo::mass_properties(&opened.body, tol()).unwrap();
    println!(
        "[r2] e2e 3: solids={} shells={} volume={} want={want} rims={}",
        opened.body.solids().count(),
        opened.body.shells().count(),
        props.volume,
        opened.naming.rims.len()
    );
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
    assert_eq!(topo::validate_geometric(&opened.body, tol()), Ok(()));
    let per_solid: Vec<(SolidKey, usize)> = opened
        .body
        .solids()
        .map(|(k, s)| (k, s.shells.len()))
        .collect();
    println!(
        "[r2] e2e 3: shells per solid {per_solid:?}; roles {:?}",
        roles(&opened.body)
    );
    tess("3 (vessel beside box, opened)", &opened.body);
    let _ = a_solid;
}

/// The 6×1×1 slab cut in half by a subtract. The subtract files both
/// components under ONE solid (ZIP's separate defect), each cut operand
/// face's fragments keeping the operand's surface key, so four charts —
/// the top, bottom, front and back planes — are each worn by a face of
/// both components. `moved` files the components as two solids through
/// the public ownership door.
fn split_slab(moved: bool) -> Body<f64> {
    let slab = brick((0.0, 6.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let wall = brick((2.5, 3.5), (-1.0, 2.0), (-1.0, 2.0), Tol::witness());
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&slab, &wall, tol()) else {
        panic!("no body")
    };
    let mut body = b.body;
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    assert_eq!(shells.len(), 2, "the subtract files two shells");
    assert_eq!(body.solids().count(), 1, "under one solid");
    if moved {
        body.move_shells_to_new_solid(&[shells[1]])
            .expect("one component moves to its own solid");
        assert_eq!(body.solids().count(), 2);
    }
    body
}

/// Face pairs of `body` wearing one surface key that `apart` separates.
fn sharing(body: &Body<f64>, apart: &dyn Fn(FaceKey, FaceKey) -> bool) -> Vec<(FaceKey, FaceKey)> {
    let mut pairs = Vec::new();
    for (fa, a) in body.faces() {
        for (fb, bb) in body.faces() {
            if fa < fb && a.surface == bb.surface && apart(fa, fb) {
                pairs.push((fa, fb));
            }
        }
    }
    pairs
}

/// The planar face of `solid` whose chart is the plane `z = z0`.
fn plane_face_at_z(body: &Body<f64>, solid: SolidKey, z0: f64) -> Vec<FaceKey> {
    faces_of(body, solid)
        .into_iter()
        .filter(|&f| {
            matches!(
                body.get_surface(body.get_face(f).unwrap().surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z0).abs() < 1e-12 && normal.z.abs() > 0.5
            )
        })
        .collect()
}

/// **A chart is body-wide, and the mover keeps it.** The split slab's
/// cut operand faces share their operand's key across the two shells;
/// `move_shells_to_new_solid` re-partitions ownership only, so the
/// same four charts are worn by both solids afterwards and the surface
/// arena is unchanged by the move. The shell door groups each solid's
/// OWN faces by key, so it thickens the pair solid by solid.
#[test]
fn r2_move_shells_to_new_solid_keeps_every_chart_and_the_slab_thickens() {
    let mut body = split_slab(false);
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    let across_shells = sharing(&body, &|fa, fb| {
        body.get_face(fa).unwrap().shell != body.get_face(fb).unwrap().shell
    });
    // The top, bottom, front and back planes, each cut in two.
    assert_eq!(across_shells.len(), 4, "shared charts across the shells");
    let charts_before: Vec<topo::SurfaceKey> = body.surfaces().map(|(k, _)| k).collect();

    body.move_shells_to_new_solid(&[shells[1]])
        .expect("one component moves to its own solid");

    assert_eq!(body.solids().count(), 2);
    let charts_after: Vec<topo::SurfaceKey> = body.surfaces().map(|(k, _)| k).collect();
    assert_eq!(
        charts_after, charts_before,
        "the move mints and drops no chart"
    );
    let across_solids = sharing(&body, &|fa, fb| solid_of(&body, fa) != solid_of(&body, fb));
    assert_eq!(
        across_solids, across_shells,
        "the same four charts are worn by both solids"
    );
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));

    let shelled = topo::shell(&body, 0.05, tol()).expect("the split slab as two solids shells");
    assert_eq!(
        shelled.body.solids().count(),
        2,
        "one thin solid per component"
    );
    assert_volume(&shelled.body, 2.0 * (2.5 - 2.4 * 0.9 * 0.9), "shelled");
    assert_eq!(topo::validate_geometric(&shelled.body, tol()), Ok(()));
}

/// **`replace_faces_offset` groups within the group's solid.** On the
/// split slab the top plane is one chart worn by a face of each solid.
/// Offsetting one solid's wearer moves it onto a fresh key and leaves
/// the other solid's face on the old one: no edge joins the two.
#[test]
fn r2_replace_faces_offset_on_one_solids_wearers_leaves_the_other_on_the_old_key() {
    let mut body = split_slab(true);
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let mine = plane_face_at_z(&body, solids[0], 1.0);
    let theirs = plane_face_at_z(&body, solids[1], 1.0);
    assert_eq!((mine.len(), theirs.len()), (1, 1), "one top face per solid");
    let old = body.get_face(mine[0]).unwrap().surface;
    assert_eq!(
        body.get_face(theirs[0]).unwrap().surface,
        old,
        "the two top faces wear one chart"
    );
    let before = volume(&body);

    topo::replace_faces_offset(&mut body, &mine, -0.1, tol())
        .expect("one solid's wearers of a shared chart offset together");

    assert_ne!(
        body.get_face(mine[0]).unwrap().surface,
        old,
        "the offset wearer is on a fresh key"
    );
    assert_eq!(
        body.get_face(theirs[0]).unwrap().surface,
        old,
        "the other solid's wearer keeps the old key"
    );
    assert!(
        body.get_surface(old).is_some(),
        "the old chart still resolves"
    );
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let want = before - 2.5 * 0.1;
    assert!(
        (volume(&body) - want).abs() <= 1e-9,
        "volume {} want {want}",
        volume(&body)
    );
}

/// **Point-in-solid chooses its group representative within the
/// selection.** A ball cut through its equator by a slab leaves two caps
/// whose sphere faces keep the ball's one sphere key; filed as two
/// solids, that key is worn by both. Each solid's query answers on its
/// own faces: its own cap's interior in, the other's out.
#[test]
fn r2_point_in_solid_of_one_cap_answers_beside_a_shared_sphere_chart() {
    let ball = sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let slab = brick((-2.0, 2.0), (-2.0, 2.0), (-0.2, 0.2), Tol::witness());
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&ball, &slab, tol()) else {
        panic!("no body")
    };
    let mut body = b.body;
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    assert_eq!(shells.len(), 2, "the subtract files two caps");
    body.move_shells_to_new_solid(&[shells[1]])
        .expect("one cap moves to its own solid");
    let is_sphere = |f: FaceKey| {
        matches!(
            body.get_surface(body.get_face(f).unwrap().surface),
            Some(geom::Surface::Sphere { .. })
        )
    };
    let across = sharing(&body, &|fa, fb| {
        is_sphere(fa) && solid_of(&body, fa) != solid_of(&body, fb)
    });
    assert!(
        !across.is_empty(),
        "the sphere chart is worn by both solids"
    );
    let band = geom_core::Band::linear(tol()).unwrap();
    let (up, down) = (Point3::new(0.0, 0.0, 0.6), Point3::new(0.0, 0.0, -0.6));
    let mut read = Vec::new();
    for (solid, _) in body.solids() {
        let at =
            |q| topo::point_in_solid_of(&body, solid, q, band, tol()).map_err(|e| e.to_string());
        read.push((at(up), at(down)));
    }
    use topo::SolidContainment::{In, Out};
    assert!(
        read == vec![(Ok(In), Ok(Out)), (Ok(Out), Ok(In))]
            || read == vec![(Ok(Out), Ok(In)), (Ok(In), Ok(Out))],
        "each solid holds its own cap and not the other's: {read:?}"
    );
}

/// Two unit bricks side by side as two solids, the upper one's bottom
/// face re-charted onto the lower one's top plane (one locus, `z = 1`,
/// opposite material sides) through the public attach door. Answers
/// `(body, lower solid, upper solid, lower's top, upper's bottom)`.
fn opposed_pair() -> (Body<f64>, SolidKey, SolidKey, FaceKey, FaceKey) {
    let lower = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let upper = brick((0.0, 1.0), (0.0, 1.0), (1.0, 2.0), Tol::witness());
    let (mut body, up) = beside_raw(&lower, &upper, Vec3::new(3.0, 0.0, 0.0));
    let down = body.solids().map(|(k, _)| k).find(|&k| k != up).unwrap();
    let top = plane_face_at_z(&body, down, 1.0)[0];
    let bottom = plane_face_at_z(&body, up, 1.0)[0];
    let key = body.get_face(top).unwrap().surface;
    let Some(geom::Surface::Plane { normal: n_top, .. }) = body.get_surface(key).cloned() else {
        panic!("top is planar")
    };
    let bottom_data = body.get_face(bottom).unwrap();
    let Some(geom::Surface::Plane {
        normal: n_bottom, ..
    }) = body.get_surface(bottom_data.surface).cloned()
    else {
        panic!("bottom is planar")
    };
    // The bottom's outward direction, stated against the shared chart.
    let outward = if bottom_data.sense {
        n_bottom
    } else {
        -n_bottom
    };
    let sense = outward.dot(n_top) > 0.0;
    // Its boundary edges name its own chart: each is re-described as the
    // intersection of its side wall with the shared chart.
    let rims: Vec<(topo::EdgeKey, topo::SurfaceKey, Point3<f64>, Point3<f64>)> = body
        .edges()
        .filter_map(|(e, d)| {
            let (plus, minus) = (d.he_plus, d.he_minus);
            let face = |he| body.face_of_half_edge(he).unwrap();
            let side = match (face(plus) == bottom, face(minus) == bottom) {
                (true, false) => face(minus),
                (false, true) => face(plus),
                _ => return None,
            };
            let at = |he| {
                let v = body.get_half_edge(he).unwrap().start;
                *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
            };
            Some((e, body.get_face(side).unwrap().surface, at(plus), at(minus)))
        })
        .collect();
    assert_eq!(rims.len(), 4, "the bottom's four edges");
    let specs: Vec<_> = rims
        .into_iter()
        .map(|(edge, wall, p, q)| {
            let len = p.distance(q);
            let spec = geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1: wall,
                    s2: key,
                    witness: p + (q - p) * 0.5,
                },
                carrier: geom::Curve3::Line {
                    origin: p,
                    dir: (q - p) / len,
                },
                param_start: 0.0,
                param_end: len,
            };
            (edge, spec)
        })
        .collect();
    body.set_face_surfaces_describing(
        vec![topo::Rechart::shared(key, bottom, sense)],
        &specs,
        tol(),
    )
    .expect("the bottom moves onto the shared chart with its edges");
    assert_ne!(
        body.get_face(top).unwrap().sense,
        body.get_face(bottom).unwrap().sense,
        "one chart, opposed senses, two solids"
    );
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    (body, down, up, top, bottom)
}

/// `body`'s volume is `want` to the exact-volume pad.
fn assert_volume(body: &Body<f64>, want: f64, what: &str) {
    let props = topo::mass_properties(body, tol()).unwrap();
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "{what}: volume {} want {want}",
        props.volume
    );
}

/// Point-in-solid of `solid` at `q`.
fn in_solid(body: &Body<f64>, solid: SolidKey, q: Point3<f64>) -> topo::SolidContainment {
    let band = geom_core::Band::linear(tol()).unwrap();
    topo::point_in_solid_of(body, solid, q, band, tol()).expect("point-in-solid answers")
}

/// **Two solids on one chart with opposed senses shell independently.**
/// The chart's sense is mixed across the body and uniform inside each
/// solid, and the shell door's sense gate reads each solid's own
/// wearers.
#[test]
fn r2_two_solids_on_one_chart_with_opposed_senses_shell_independently() {
    let (body, ..) = opposed_pair();
    let shelled = topo::shell(&body, 0.1, tol()).expect("each solid thickens on its own");
    assert_eq!(shelled.body.solids().count(), 2);
    assert_volume(&shelled.body, 2.0 * (1.0 - 0.8 * 0.8 * 0.8), "shelled");
    assert_eq!(topo::validate_geometric(&shelled.body, tol()), Ok(()));
}

/// **Opened on a chart two solids share, whichever solid is named.** On
/// the opposed pair and on the same-sense split slab, opening one
/// solid's wearer opens that solid only, and naming both — in either
/// order, or beside another chart of the second solid — opens both. The
/// thickness is uniform, so each thin solid's volume is closed form:
/// `outer − (outer shrunk by 2t, and by t only along an open axis)`.
#[test]
fn r2_open_on_a_chart_two_solids_share_opens_only_the_named_solids() {
    let (pair, _, _, top, bottom) = opposed_pair();
    let t = 0.1;
    let (closed, open) = (1.0 - 0.8 * 0.8 * 0.8, 1.0 - 0.8 * 0.8 * 0.9);
    for (faces, want) in [
        (vec![top], open + closed),
        (vec![bottom], closed + open),
        (vec![top, bottom], 2.0 * open),
        (vec![bottom, top], 2.0 * open),
    ] {
        let s = topo::shell_open(&pair, t, &faces, tol())
            .unwrap_or_else(|e| panic!("opposed pair opened at {faces:?}: {e}"));
        assert_eq!(s.body.solids().count(), 2, "opposed pair at {faces:?}");
        assert_volume(&s.body, want, &format!("opposed pair opened at {faces:?}"));
        assert_eq!(topo::validate_geometric(&s.body, tol()), Ok(()));
    }

    let slab = split_slab(true);
    let solids: Vec<SolidKey> = slab.solids().map(|(k, _)| k).collect();
    let a = plane_face_at_z(&slab, solids[0], 1.0);
    let b = plane_face_at_z(&slab, solids[1], 1.0);
    let b_bottom = plane_face_at_z(&slab, solids[1], 0.0);
    let t = 0.05;
    let (closed, open) = (2.5 - 2.4 * 0.9 * 0.9, 2.5 - 2.4 * 0.9 * 0.95);
    use topo::SolidContainment::{In, Out};
    for (faces, want) in [
        (a.clone(), open + closed),
        (b.clone(), open + closed),
        ([&a[..], &b[..]].concat(), 2.0 * open),
        ([&b[..], &a[..]].concat(), 2.0 * open),
        ([&a[..], &b_bottom[..]].concat(), 2.0 * open),
    ] {
        let s = topo::shell_open(&slab, t, &faces, tol())
            .unwrap_or_else(|e| panic!("split slab opened at {faces:?}: {e}"));
        assert_eq!(s.body.solids().count(), 2, "split slab at {faces:?}");
        assert_volume(&s.body, want, &format!("split slab opened at {faces:?}"));
        assert_eq!(topo::validate_geometric(&s.body, tol()), Ok(()));
        // Each thin solid holds its own component's wall point, not the
        // other's, and not the hollow.
        let (wall_a, wall_b, hollow) = (
            Point3::new(1.0, 0.025, 0.5),
            Point3::new(5.0, 0.025, 0.5),
            Point3::new(1.0, 0.5, 0.5),
        );
        let hits: Vec<_> = s
            .body
            .solids()
            .map(|(k, _)| {
                (
                    in_solid(&s.body, k, wall_a),
                    in_solid(&s.body, k, wall_b),
                    in_solid(&s.body, k, hollow),
                )
            })
            .collect();
        assert!(
            hits.contains(&(In, Out, Out)) && hits.contains(&(Out, In, Out)),
            "split slab at {faces:?}: {hits:?}"
        );
    }
}

/// **The axial lift moves the lift solid's own wearers only.** A ball
/// cut by two slabs into a top cap, a zone and a bottom cap, all
/// wearing the ball's one sphere key, filed as three solids. Every
/// lift through `ChartsTogether` names the sphere chart at distance
/// zero, and the zone takes TWO rims, the second lifted after the
/// first's surgery. Oracle: an opening is a solid's own change, so the
/// volume deltas against the closed shell add over solids, in any
/// designation order, and the caps and the zone's two rims are
/// mirror-symmetric.
#[test]
fn r2_three_solids_on_one_sphere_chart_open_additively() {
    let tol = tol();
    let ball = sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol);
    let upper = brick((-2.0, 2.0), (-2.0, 2.0), (0.3, 0.5), Tol::witness());
    let lower = brick((-2.0, 2.0), (-2.0, 2.0), (-0.5, -0.3), Tol::witness());
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&ball, &upper, tol) else {
        panic!("the first cut builds")
    };
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&b.body, &lower, tol) else {
        panic!("the second cut builds")
    };
    let mut body = b.body;
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    assert_eq!(shells.len(), 3, "a top cap, a zone and a bottom cap");
    for &shell in &shells[1..] {
        body.move_shells_to_new_solid(&[shell])
            .expect("each piece moves to its own solid");
    }
    assert_eq!(topo::validate_geometric(&body, tol), Ok(()));
    let sphere_keys: std::collections::BTreeSet<topo::SurfaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { .. })
            )
        })
        .map(|(_, f)| f.surface)
        .collect();
    assert_eq!(sphere_keys.len(), 1, "one sphere chart across three solids");
    let at = |z: f64| -> FaceKey {
        let found: Vec<FaceKey> = body
            .solids()
            .flat_map(|(s, _)| plane_face_at_z(&body, s, z))
            .collect();
        assert_eq!(found.len(), 1, "one plane face at z = {z}");
        found[0]
    };
    let (top_cap, zone_up, zone_down, bottom_cap) = (at(0.5), at(0.3), at(-0.3), at(-0.5));
    let t = 0.05;
    let closed = topo::shell(&body, t, tol).expect("the three pieces shell");
    assert_eq!(topo::validate_geometric(&closed.body, tol), Ok(()));
    let closed = volume(&closed.body);
    let delta = |faces: &[FaceKey]| -> f64 {
        let s = topo::shell_open(&body, t, faces, tol)
            .unwrap_or_else(|e| panic!("opened at {faces:?}: {e}"));
        assert_eq!(topo::validate_geometric(&s.body, tol), Ok(()));
        assert_eq!(s.body.solids().count(), 3, "opened at {faces:?}");
        volume(&s.body) - closed
    };
    let close = |got: f64, want: f64, what: &str| {
        assert!((got - want).abs() <= 1e-7, "{what}: {got} want {want}");
    };
    let d_cap = delta(&[top_cap]);
    close(delta(&[bottom_cap]), d_cap, "the caps are mirror twins");
    let d_up = delta(&[zone_up]);
    close(
        delta(&[zone_down]),
        d_up,
        "the zone's rims are mirror twins",
    );
    let d_zone = delta(&[zone_up, zone_down]);
    close(
        delta(&[zone_down, zone_up]),
        d_zone,
        "the zone's two rims, either order",
    );
    close(
        delta(&[zone_up, top_cap, zone_down]),
        d_zone + d_cap,
        "additive over solids",
    );
    close(
        delta(&[top_cap, zone_down, zone_up]),
        d_zone + d_cap,
        "additive, reordered",
    );
    close(
        delta(&[bottom_cap, zone_up, top_cap, zone_down]),
        d_zone + 2.0 * d_cap,
        "every rim open",
    );
    assert!(
        d_cap < 0.0 && d_zone < d_up && d_up < 0.0,
        "an opening removes material: cap {d_cap}, one rim {d_up}, two {d_zone}"
    );
}

/// **`replace_faces_offset`'s scope is the solid, not the shell.** On
/// the un-moved split slab one solid wears the top chart on two shells:
/// naming one shell's wearer refuses `SharedSurfaceKey`, and naming both
/// moves them. Across solids — the moved slab — one call moves both
/// solids' wearers, and on the opposed pair the lower solid's top moves
/// alone.
#[test]
fn r2_replace_faces_offset_scopes_to_the_solid_across_shells_and_solids() {
    let mut one = split_slab(false);
    let solid = one.solids().next().unwrap().0;
    let tops = plane_face_at_z(&one, solid, 1.0);
    let e = topo::replace_faces_offset(&mut one.clone(), &tops[..1], -0.1, tol()).unwrap_err();
    assert!(
        matches!(e, topo::ReplaceFaceError::SharedSurfaceKey { .. }),
        "one shell's wearer of the solid's chart: {e}"
    );
    let before = volume(&one);
    topo::replace_faces_offset(&mut one, &tops, -0.1, tol()).expect("both of the solid's wearers");
    assert_eq!(topo::validate_geometric(&one, tol()), Ok(()));
    assert_volume(&one, before - 2.0 * 2.5 * 0.1, "one solid, both tops");

    let mut two = split_slab(true);
    let solids: Vec<SolidKey> = two.solids().map(|(k, _)| k).collect();
    let both = [
        plane_face_at_z(&two, solids[0], 1.0),
        plane_face_at_z(&two, solids[1], 1.0),
    ]
    .concat();
    let before = volume(&two);
    topo::replace_faces_offset(&mut two, &both, -0.1, tol()).expect("both solids' wearers");
    assert_eq!(topo::validate_geometric(&two, tol()), Ok(()));
    assert_volume(&two, before - 2.0 * 2.5 * 0.1, "two solids, both tops");

    let (mut pair, down, up, top, _) = opposed_pair();
    let before = volume(&pair);
    topo::replace_faces_offset(&mut pair, &[top], -0.1, tol()).expect("the lower's top alone");
    assert_eq!(topo::validate_geometric(&pair, tol()), Ok(()));
    assert_volume(&pair, before - 0.1, "opposed pair: the lower shrinks only");
    use topo::SolidContainment::{In, Out};
    assert_eq!(
        in_solid(&pair, down, Point3::new(0.5, 0.5, 0.95)),
        Out,
        "lower, above its new top"
    );
    assert_eq!(
        in_solid(&pair, up, Point3::new(3.5, 0.5, 1.05)),
        In,
        "upper, unmoved"
    );
}

/// **Point-in-solid on one sphere chart worn across two shells of ONE
/// solid.** A ball cut through its equator by a slab leaves two caps
/// on the ball's one sphere key, both filed under one solid. The
/// solid's query reads both caps; after the move, the whole-body query
/// does.
#[test]
fn r2_point_in_solid_reads_one_sphere_chart_across_two_shells_of_one_solid() {
    let ball = sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let slab = brick((-2.0, 2.0), (-2.0, 2.0), (-0.2, 0.2), Tol::witness());
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&ball, &slab, tol()) else {
        panic!("no body")
    };
    let mut body = b.body;
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let solid = body.solids().next().unwrap().0;
    use topo::SolidContainment::{In, Out};
    for (q, want) in [
        (Point3::new(0.0, 0.0, 0.6), In),
        (Point3::new(0.0, 0.0, -0.6), In),
        (Point3::new(0.6, 0.0, -0.6), In),
        (Point3::new(0.0, 0.0, 0.0), Out),
        (Point3::new(0.0, 0.0, 1.5), Out),
        (Point3::new(0.75, 0.0, 0.75), Out),
    ] {
        assert_eq!(in_solid(&body, solid, q), want, "one solid at {q:?}");
    }
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    body.move_shells_to_new_solid(&[shells[1]])
        .expect("one cap moves to its own solid");
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let band = geom_core::Band::linear(tol()).unwrap();
    for (q, want) in [
        (Point3::new(0.0, 0.0, 0.6), In),
        (Point3::new(0.0, 0.0, -0.6), In),
        (Point3::new(0.0, 0.0, 0.0), Out),
        (Point3::new(0.0, 0.0, 1.5), Out),
    ] {
        assert_eq!(
            topo::point_in_solid(&body, q, band, tol()).unwrap(),
            want,
            "whole body at {q:?}"
        );
    }
}

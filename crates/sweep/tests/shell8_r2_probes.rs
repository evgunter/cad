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
use geom_core::{Affine3, Band, Point3, Sign, Tol, Vec3};
use topo::ShellNaming;
use topo::{Body, FaceKey, ShellError, ShellRole, SolidKey, VoidContainment, VoidEvidence};

use crate::verbs_shell::{boxy, hollow_box, v, vessel};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("a band")
}

/// `other` placed `d` along `+x` beside `body` as a second solid, through
/// the public disjoint-graft door. No validity assertion — some rows
/// build operands tier 3 would not bless on purpose.
fn beside_raw(body: &Body<f64>, other: &Body<f64>, d: Vec3<f64>) -> (Body<f64>, SolidKey) {
    let mut out = body.clone();
    let placed = topo::transform_rigid(other, &Affine3::translation(d), tol()).expect("rigid");
    let k = topo::graft_disjoint(&mut out, &placed, tol()).expect("grafts");
    (out, k)
}

fn beside(body: &Body<f64>, other: &Body<f64>, dx: f64) -> Body<f64> {
    let (out, _) = beside_raw(body, other, Vec3::new(dx, 0.0, 0.0));
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "operand valid"
    );
    out
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).expect("props").volume
}

fn points(body: &Body<f64>) -> Vec<(topo::VertexKey, (u64, u64, u64))> {
    body.vertices()
        .map(|(k, v)| {
            let p = body.get_point(v.point).unwrap();
            (k, (p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
        })
        .collect()
}

fn solid_of(body: &Body<f64>, face: FaceKey) -> SolidKey {
    let shell = body.get_face(face).unwrap().shell;
    body.get_shell(shell).unwrap().solid
}

fn solid_of_vertex(body: &Body<f64>, vertex: topo::VertexKey) -> SolidKey {
    let he = body.get_vertex(vertex).unwrap().emanating.unwrap();
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    let face = body.get_loop(lp).unwrap().face;
    solid_of(body, face)
}

fn faces_of(body: &Body<f64>, solid: SolidKey) -> Vec<FaceKey> {
    body.faces()
        .filter(|(k, _)| solid_of(body, *k) == solid)
        .map(|(k, _)| k)
        .collect()
}

fn charts_of(body: &Body<f64>, solid: SolidKey) -> Vec<Vec<FaceKey>> {
    let mut out: Vec<(topo::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for face in faces_of(body, solid) {
        let key = body.get_face(face).unwrap().surface;
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => v.push(face),
            None => out.push((key, vec![face])),
        }
    }
    out.into_iter().map(|(_, v)| v).collect()
}

/// Inward moves for every chart of `solid` at wall `t` (the verb's own
/// `inward` rule: sense true → −t).
fn inward_moves(body: &Body<f64>, solid: SolidKey, t: f64) -> Vec<topo::ChartMove<f64>> {
    charts_of(body, solid)
        .into_iter()
        .map(|faces| {
            let sense = body.get_face(faces[0]).unwrap().sense;
            topo::ChartMove {
                faces,
                distance: if sense { -t } else { t },
            }
        })
        .collect()
}

/// The chart of `shell` whose plane is normal to `axis` at `value`.
fn cap(body: &Body<f64>, shell: topo::ShellKey, axis: Vec3<f64>, value: f64) -> Vec<FaceKey> {
    for &face in &body.get_shell(shell).unwrap().faces {
        let f = body.get_face(face).unwrap();
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        if normal.cross(axis).norm() > 1e-9 {
            continue;
        }
        if (Vec3::new(origin.x, origin.y, origin.z).dot(axis) - value).abs() < 1e-9 {
            let chart = f.surface;
            return body
                .faces()
                .filter(|(_, g)| g.surface == chart)
                .map(|(k, _)| k)
                .collect();
        }
    }
    panic!("no cap of {shell:?} normal to {axis:?} at {value}")
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
    let (mut same, mut moved) = (0, 0);
    for ((k, pa), (k2, pb)) in a.iter().zip(b.iter()) {
        assert_eq!(k, k2, "arena order kept");
        if solid_of_vertex(before, *k) != solid {
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
    let pair = beside(&boxy(2.0, 3.0, 4.0), &vessel(1.0, 2.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let (bx, vs) = (solids[0], solids[1]);

    // Whole-body reading: not axial (the box is in it).
    assert!(!topo::is_axial(&pair, band()).unwrap());

    // The vessel's charts only, through the axial door.
    let mut work = pair.clone();
    topo::offset_charts_together(&mut work, &inward_moves(&pair, vs, t), band(), tol())
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
    let r = topo::offset_planes_together(&mut work, &inward_moves(&pair, bx, t), band(), tol());
    println!("[r2] planar door scoped to box beside a vessel: {r:?}");
    r.expect("the box's charts move through the planar door while the vessel stands by");
    let (same, moved) = bitwise_solid(&pair, &work, vs);
    assert_eq!(moved, 0, "the vessel moved: same={same}");
    assert_eq!(topo::validate_geometric(&work, tol()), Ok(()));
}

#[test]
fn r2_scope_naming_two_of_three_solids_leaves_the_third_bitwise() {
    let t = 0.05;
    let two = beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let three = beside(&two, &boxy(2.0, 3.0, 4.0), 20.0);
    let solids: Vec<SolidKey> = three.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 3);
    let mut moves = inward_moves(&three, solids[0], t);
    moves.extend(inward_moves(&three, solids[2], t));
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
    let mut partial = inward_moves(&three, solids[0], t);
    partial.extend(inward_moves(&three, solids[2], t));
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
    let inner = boxy(0.5, 0.5, 0.5);
    let (body, inner_solid) = beside_raw(&hollow, &inner, Vec3::new(0.75, 1.25, 3.75 - 0.02 - 0.5));
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Ok(()),
        "a solid inside a void is a valid body"
    );
    println!("[r2] roles of the nested operand: {:?}", roles(&body));
    let s = topo::shell(&body, t, tol())
        .expect("both shell; the void wall and the cube face never gate");
    let want = (v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9))
        + (v(1.6, 2.6, 3.6) - v(1.5, 2.5, 3.5))
        + (v(0.5, 0.5, 0.5) - v(0.4, 0.4, 0.4));
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
        &boxy(2.0, 3.0, 4.0),
        &boxy(0.5, 0.5, 0.5),
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
    let pair = beside(&hollow_box(), &boxy(2.0, 3.0, 4.0), 10.0);
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
        "only the HOLLOW solid's two shells are classified; the plain          neighbour's is its boundary by arity and is never read"
    );

    let plain = beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), 10.0);
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
    let mut host = boxy(2.0, 3.0, 4.0);
    let host_solid = host.solids().next().unwrap().0;
    let cube = topo::transform_rigid(
        &boxy(0.5, 0.5, 0.5),
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
    topo::insert_void(&mut host, host_solid, pre_reverted, &evidence, tol()).expect("inserts");
    println!("[r2] two-outer host roles: {:?}", roles(&host));
    let (body, _) = beside_raw(&host, &boxy(2.0, 3.0, 4.0), Vec3::new(10.0, 0.0, 0.0));
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
// Claim 5 — is `ChartSpansSolids` reachable through a public producer?
// A boolean that cuts one slab into two components: do the two
// components' faces keep the operand's surface keys, and do they land in
// two solids?
// ---------------------------------------------------------------------

#[test]
fn r2_chart_spans_solids_reachability_through_a_disconnecting_subtract() {
    let slab = crate::verbs_shell::brick(0.0, 6.0, 0.0, 1.0, 0.0, 1.0);
    let wall = crate::verbs_shell::brick(2.5, 3.5, -1.0, 2.0, -1.0, 2.0);
    let r = topo::subtract(&slab, &wall, tol());
    let body = match r {
        Ok(topo::BooleanResult::Body(b)) => {
            println!(
                "[r2] disconnecting subtract: kind={:?} solids={} shells={}",
                b.kind,
                b.body.solids().count(),
                b.body.shells().count()
            );
            b.body
        }
        other => {
            println!("[r2] disconnecting subtract did not give a body: {other:?}");
            return;
        }
    };
    // Surface keys shared across solids?
    let mut shared = 0usize;
    for (fa, a) in body.faces() {
        for (fb, b) in body.faces() {
            if fa < fb && a.surface == b.surface && solid_of(&body, fa) != solid_of(&body, fb) {
                shared += 1;
            }
        }
    }
    println!("[r2] cross-solid face pairs sharing a surface key: {shared}");
    match topo::shell(&body, 0.05, tol()) {
        Ok(s) => println!(
            "[r2] shell of the split slab BUILDS: solids={} shells={} volume={} want={}",
            s.body.solids().count(),
            s.body.shells().count(),
            volume(&s.body),
            2.0 * (2.5 - 0.9 * 0.9 * 2.4)
        ),
        Err(e) => println!("[r2] shell of the split slab refuses: {e}"),
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
    let (_, void) = crate::verbs_shell::outer_and_void(&hv);
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
    let pair = beside(&boxy(2.0, 3.0, 4.0), &hv, 10.0);
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
    let (mut same, mut moved) = (0, 0);
    for (k, _) in pair.vertices() {
        if solid_of_vertex(&pair, k) != box_solid {
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
    let want = (v(2.0, 3.0, 4.0) - v(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t))
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
    let two = beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let three = beside(&two, &boxy(2.0, 3.0, 4.0), 20.0);
    let s = topo::shell(&three, 0.05, tol()).expect("three build");
    assert_eq!(s.body.solids().count(), 3);
}

// ---------------------------------------------------------------------
// Claim 4 — the solid-order assertion on a body whose solid arena has a
// freed slot, and after a graft that reuses it.
// ---------------------------------------------------------------------

#[test]
fn r2_solid_order_assertion_on_a_body_with_a_freed_solid_slot() {
    let mut body = boxy(2.0, 3.0, 4.0);
    let lone = body
        .mvfs(Point3::new(50.0, 50.0, 50.0))
        .expect("a lone-vertex solid");
    let (mut body, third) = beside_raw(&body, &boxy(2.0, 3.0, 4.0), Vec3::new(10.0, 0.0, 0.0));
    let killed = body.kvfs(lone.solid).expect("the lone solid dies");
    assert_eq!(killed.killed_solid, lone.solid);
    let order: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    println!("[r2] freed slot: solid order {order:?} (third = {third:?})");
    assert_eq!(order.len(), 2);
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    let s = topo::shell(&body, 0.05, tol()).expect("a freed slot in the arena is not a reorder");
    assert_eq!(s.body.solids().count(), 2);
    let one_wall = v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9);
    assert!((volume(&s.body) - 2.0 * one_wall).abs() < 1e-12);

    // A graft that REUSES the freed slot (a key with version 2 in the
    // middle of the arena): still one order, still builds.
    let (body, reused) = beside_raw(&body, &boxy(2.0, 3.0, 4.0), Vec3::new(20.0, 0.0, 0.0));
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
    let one_wall = |t: f64| v(2.0, 3.0, 4.0) - v(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t);

    // ---- 1. Two parts in one body, hollowed in one call. ----
    let part_a = boxy(2.0, 3.0, 4.0);
    let part_b = vessel(1.0, 2.0);
    let mut assembly = part_a.clone();
    let placed = topo::transform_rigid(
        &part_b,
        &Affine3::translation(Vec3::new(10.0, 0.0, 0.0)),
        tol(),
    )
    .unwrap();
    let b_solid = topo::graft_disjoint(&mut assembly, &placed, tol()).expect("placed");
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
    let want = (v(2.0, 3.0, 4.0) - v(1.98, 2.98, 3.98))
        + (v(1.92, 2.92, 3.92) - v(1.9, 2.9, 3.9))
        + (v(1.6, 2.6, 3.6) - v(1.58, 2.58, 3.58))
        + (v(1.52, 2.52, 3.52) - v(1.5, 2.5, 3.5))
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
    let b_inner_ceiling: Vec<FaceKey> = assembly
        .faces()
        .filter(|(_, f)| f.surface == chart)
        .map(|(k, _)| twin_of(&hollowed.naming, k))
        .collect();
    let opened = topo::shell_open(&hollowed.body, 0.02, &b_inner_ceiling, tol())
        .expect("opened on the vessel's inner wall");
    // BOTH solids shell again: the box's thin solid becomes two more.
    let want = (v(2.0, 3.0, 4.0) - v(1.96, 2.96, 3.96))
        + (v(1.94, 2.94, 3.94) - v(1.9, 2.9, 3.9))
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

/// **Is `ChartSpansSolids` reachable through public doors?** A
/// disconnecting subtract files both components under ONE solid; if
/// their fragments share a surface key, `Body::move_shells_to_new_solid`
/// (public) yields a two-solid body whose chart spans both.
#[test]
fn r2_chart_spans_solids_through_subtract_then_move_shells() {
    let slab = crate::verbs_shell::brick(0.0, 6.0, 0.0, 1.0, 0.0, 1.0);
    let wall = crate::verbs_shell::brick(2.5, 3.5, -1.0, 2.0, -1.0, 2.0);
    let Ok(topo::BooleanResult::Body(b)) = topo::subtract(&slab, &wall, tol()) else {
        panic!("no body")
    };
    let mut body = b.body;
    let shells: Vec<topo::ShellKey> = body.shells().map(|(k, _)| k).collect();
    assert_eq!(shells.len(), 2);
    let mut shared_across_shells = 0usize;
    for (fa, a) in body.faces() {
        for (fb, bb) in body.faces() {
            if fa < fb && a.surface == bb.surface && a.shell != bb.shell {
                shared_across_shells += 1;
            }
        }
    }
    println!(
        "[r2] split slab: face pairs on different shells sharing a surface key = {shared_across_shells}"
    );
    let minted = body
        .move_shells_to_new_solid(&[shells[1]])
        .expect("one component moves to its own solid");
    println!(
        "[r2] after move_shells_to_new_solid: solids={} minted={minted:?} tier3={:?}",
        body.solids().count(),
        topo::validate_geometric(&body, tol())
    );
    match topo::shell(&body, 0.05, tol()) {
        Ok(s) => println!(
            "[r2] the split slab as two solids BUILDS: solids={} volume={} want={}",
            s.body.solids().count(),
            volume(&s.body),
            2.0 * (2.5 - 2.4 * 0.9 * 0.9)
        ),
        Err(e) => {
            println!("[r2] the split slab as two solids refuses: {e}");
            assert!(matches!(e, ShellError::ChartSpansSolids { .. }), "{e}");
        }
    }
}

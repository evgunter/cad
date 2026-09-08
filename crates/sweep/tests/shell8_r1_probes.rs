//! **SHELL-8 review R1 — probe rows.**
//!
//! Falsification attempts against PR #2207's claims. Every row here is
//! an EXECUTED measurement, not an assertion of intent; the rows that
//! pass are evidence for the claim, the rows that were written to go
//! red and did not are recorded as such in the review report.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use geom_core::{Affine3, Band, Tol, Vec3};
use topo::{Body, FaceKey, ShellKey, SolidKey};

use crate::verbs_shell::{boxy, hollow_box, v, vessel};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("a band")
}

fn beside(body: &Body<f64>, other: &Body<f64>, dx: f64) -> Body<f64> {
    let mut out = body.clone();
    let placed =
        topo::transform_rigid(other, &Affine3::translation(Vec3::new(dx, 0.0, 0.0)), tol())
            .expect("a rigid map");
    topo::graft_disjoint(&mut out, &placed, tol()).expect("the placed copy grafts");
    out
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).expect("props").volume
}

fn solid_of(body: &Body<f64>, face: FaceKey) -> SolidKey {
    let shell = body.get_face(face).unwrap().shell;
    body.get_shell(shell).unwrap().solid
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

fn face_of_he(body: &Body<f64>, he: topo::HalfEdgeKey) -> FaceKey {
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    body.get_loop(lp).unwrap().face
}

/// A DEEP per-solid dump: every face's surface, every edge's carrier,
/// parameters and description, every vertex's point — the reading the
/// PR's own bitwise row does not take (it compares vertex POINTS only).
fn deep_dump(body: &Body<f64>, solid: SolidKey) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mine = faces_of(body, solid);
    for &f in &mine {
        let d = body.get_face(f).unwrap();
        out.push(format!(
            "face sense={} rings={} surface={:?}",
            d.sense,
            d.rings.len(),
            body.get_surface(d.surface)
        ));
    }
    for (k, e) in body.edges() {
        let fa = face_of_he(body, e.he_plus);
        if !mine.contains(&fa) {
            continue;
        }
        let c = body
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        out.push(format!(
            "edge {k:?} carrier={:?} params={:?} description={:?}",
            c.carrier(),
            c.params(),
            c.description()
        ));
    }
    for (k, vx) in body.vertices() {
        let Some(em) = body.get_vertex(k).unwrap().emanating else {
            continue;
        };
        if !mine.contains(&face_of_he(body, em)) {
            continue;
        }
        let p = body.get_point(vx.point).unwrap();
        out.push(format!(
            "vertex bits=({:x},{:x},{:x})",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        ));
    }
    out.sort();
    out
}

// ---------------------------------------------------------------------
// Claim 1 — the AXIAL door, scoped
// ---------------------------------------------------------------------

/// **The axial simultaneous door on a vessel beside a box: the box is
/// untouched DEEPER than the pinned row measures.** The unit's own STOP
/// row is the PLANAR door and compares vertex points only. This one
/// runs `offset_charts_together` — whose write phase mints surfaces,
/// re-authors edge carriers and then runs `mint_pcurves` over the WHOLE
/// clone — and compares the out-of-scope solid's faces, surfaces, edge
/// carriers, parameters, descriptions AND vertex bit patterns.
#[test]
fn r1_axial_door_leaves_the_other_solid_deep_identical() {
    let pair = beside(&vessel(1.0, 2.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 2, "a vessel beside a box");
    let (ves, bx) = (solids[0], solids[1]);

    let before = deep_dump(&pair, bx);
    let moves: Vec<topo::ChartMove<f64>> = charts_of(&pair, ves)
        .into_iter()
        .map(|faces| topo::ChartMove {
            faces,
            distance: -0.05,
        })
        .collect();
    let mut work = pair.clone();
    topo::offset_charts_together(&mut work, &moves, band(), tol())
        .expect("the vessel's charts move together while the box stands aside");
    let after = deep_dump(&work, bx);
    println!(
        "[r1] axial scoped: {} dump rows for the untouched box",
        before.len()
    );
    assert_eq!(before, after, "the box is deep-identical, not just bitwise");
}

/// **The axial frame's EXTENT is the scope's.** `axial_frame` levers
/// every alignment margin by the radial extent of the vertices it
/// reads. If it read the BODY's, a box parked a million metres away
/// would lever the vessel's own margins by `1e6` and the door would
/// stop deciding. Scoped, the vessel shells at the same volume it does
/// alone — and that equality is the measurement.
#[test]
fn r1_a_distant_box_does_not_lever_the_vessels_margins() {
    let t = 0.05;
    let (r, h) = (1.0, 2.0);
    let alone = topo::shell(&vessel(r, h), t, tol())
        .expect("the vessel alone")
        .body;
    let far = topo::shell(
        &beside(&vessel(r, h), &boxy(2.0, 3.0, 4.0), 1.0e6),
        t,
        tol(),
    )
    .expect("a box a million metres away does not reach the vessel")
    .body;
    let wall = core::f64::consts::PI * (r * r * h - (r - t) * (r - t) * (h - 2.0 * t));
    let boxwall = v(2.0, 3.0, 4.0) - v(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t);
    println!(
        "[r1] vessel alone {:.12}, in a far pair {:.12}, closed form {:.12}",
        volume(&alone),
        volume(&far),
        wall
    );
    assert!(
        (volume(&alone) - wall).abs() < 1e-12,
        "the vessel's own wall"
    );
    assert!(
        (volume(&far) - (wall + boxwall)).abs() < 1e-9,
        "the pair is the sum of the two walls"
    );
}

/// **A scope naming TWO of THREE solids.** Three boxes; move every
/// chart of the first two in one call; the third is deep-identical and
/// the two that moved both moved.
#[test]
fn r1_a_scope_of_two_of_three_solids() {
    let three = beside(
        &beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), 10.0),
        &boxy(2.0, 3.0, 4.0),
        20.0,
    );
    let solids: Vec<SolidKey> = three.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 3);
    let mut moves: Vec<topo::ChartMove<f64>> = Vec::new();
    for &s in &solids[..2] {
        for faces in charts_of(&three, s) {
            moves.push(topo::ChartMove {
                faces,
                distance: -0.1,
            });
        }
    }
    let before: Vec<Vec<String>> = solids.iter().map(|&s| deep_dump(&three, s)).collect();
    let mut work = three.clone();
    topo::offset_planes_together(&mut work, &moves, band(), tol())
        .expect("two of three solids move together");
    let after: Vec<Vec<String>> = solids.iter().map(|&s| deep_dump(&work, s)).collect();
    assert_ne!(before[0], after[0], "the first named solid moved");
    assert_ne!(before[1], after[1], "the second named solid moved");
    assert_eq!(before[2], after[2], "the unnamed solid is deep-identical");
    println!("[r1] two of three: the third solid is deep-identical");
}

// ---------------------------------------------------------------------
// Claim 3 — the roles read
// ---------------------------------------------------------------------

/// **The roles read is PER HOLLOW SOLID, counted at the funnel.** A
/// shell's role is the decided sign of its own signed volume, so a
/// solid's roles involve no other solid's shells — and a plain
/// neighbour, whose single shell is its boundary by arity, is never
/// classified at all. Counted as `chk_shell_volume_sign` verdicts: a
/// body of only plain solids reads nothing, a hollow solid reads its
/// own two, and a plain solid standing beside it adds none.
///
/// What this rules out is a classification escalating on a solid the
/// caller did not ask about and refusing one it did.
#[test]
fn r1_the_roles_read_is_per_hollow_solid() {
    use geom_core::k_stats::Bracket;
    let count = |body: &Body<f64>| -> usize {
        let bracket = Bracket::open();
        let _ = topo::shell(body, 0.02, tol());
        bracket
            .finish()
            .verdicts
            .iter()
            .filter(|v| v.predicate == "chk_shell_volume_sign")
            .count()
    };
    // Only plain solids: nothing is classified at all.
    let plain = beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let n_plain = count(&plain);
    // A hollow solid (2 shells) beside a plain one (1 shell).
    let mixed = beside(&hollow_box(), &boxy(2.0, 3.0, 4.0), 10.0);
    let n_mixed = count(&mixed);
    // The hollow solid alone.
    let lone = count(&hollow_box());
    println!(
        "[r1] chk_shell_volume_sign verdicts: plain-pair={n_plain}, hollow-alone={lone}, hollow+plain={n_mixed}"
    );
    assert_eq!(n_plain, 0, "a body of only plain solids reads nothing");
    assert_eq!(lone, 2, "the hollow solid's own two shells");
    // **The read is PER HOLLOW SOLID.** A plain neighbour adds nothing:
    // its shell is its boundary by arity, so it is never classified,
    // and a classification that escalated on it could not refuse the
    // hollow solid's shelling. This row asserted the whole-body read
    // (`n_mixed > lone`, measured 3 > 2) and is re-pinned to the
    // per-solid one.
    assert_eq!(
        n_mixed, lone,
        "the plain neighbour is not classified: {n_mixed} should equal {lone}"
    );
}

/// **A hollow solid beside a plain one still classifies correctly.**
/// The count that matters is the per-solid outer count; the plain
/// solid's own shell must be reachable in the roles list or the count
/// would be wrong.
#[test]
fn r1_a_hollow_and_a_plain_solid_shell_together() {
    let t = 0.02;
    let body = beside(&hollow_box(), &boxy(2.0, 3.0, 4.0), 10.0);
    let out = topo::shell(&body, t, tol())
        .expect("the mixed body shells")
        .body;
    println!(
        "[r1] hollow+plain: solids={} shells={}",
        out.solids().count(),
        out.shells().count()
    );
    assert_eq!(out.solids().count(), 3, "three thin solids");
    let roles = topo::classify_shells(&body, tol()).expect("the operand classifies");
    let outer = roles
        .iter()
        .filter(|c| c.role == topo::ShellRole::Outer)
        .count();
    assert_eq!(outer, 2, "one outer shell per solid, both read");
}

// ---------------------------------------------------------------------
// Claim 2 — cross-solid pairs, and the case where it might be wrong
// ---------------------------------------------------------------------

/// **A solid sitting INSIDE another solid's void.** The clearance gate
/// skips every cross-solid pair, so this builds however close the inner
/// part stands to the cavity wall. But the outer solid's VOID wall
/// dilates OUTWARD by `t` while the inner part's outer wall erodes
/// inward — so at a clearance below `t` the two thin solids' walls pass
/// through each other, and the row measures whether the verb notices.
#[test]
fn r1_a_part_inside_another_solids_void() {
    // hollow_box() is boxy(2,3,4) shelled at 0.25 — its void is the box
    // [0.25, 1.75] x [0.25, 2.75] x [0.25, 3.75].
    let t = 0.05;
    let inner = crate::verbs_shell::brick(0.27, 1.73, 0.27, 2.73, 0.27, 3.73);
    let mut body = hollow_box();
    let placed = topo::transform_rigid(
        &inner,
        &Affine3::translation(Vec3::new(0.0, 0.0, 0.0)),
        tol(),
    )
    .expect("identity");
    topo::graft_disjoint(&mut body, &placed, tol()).expect("the nested part grafts");
    println!(
        "[r1] nested operand: solids={} shells={} clearance=0.02 < t={t}",
        body.solids().count(),
        body.shells().count()
    );
    match topo::shell(&body, t, tol()) {
        Ok(s) => {
            let vol = volume(&s.body);
            let cav = v(1.5, 2.5, 3.5) - v(1.46, 2.46, 3.46);
            let outer = v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9);
            let part = v(1.46, 2.46, 3.46) - v(1.36, 2.36, 3.36);
            println!(
                "[r1] nested built: solids={} shells={} volume={vol:.9} (sum of walls {:.9})",
                s.body.solids().count(),
                s.body.shells().count(),
                outer + cav + part
            );
            println!(
                "[r1] nested tier-3: {:?}",
                topo::validate_geometric(&s.body, tol())
            );
        }
        Err(e) => println!("[r1] nested refused: {e}"),
    }
}

// ---------------------------------------------------------------------
// Claim 6 — the lift's door
// ---------------------------------------------------------------------

/// **A box beside a vessel, opened on the VESSEL's void ceiling.** The
/// lift's door must be the vessel's minted thin solid's — axial — not
/// the body's, which is neither planar nor axial. If the lift read the
/// whole result the vessel's cylindrical counterpart would take the
/// per-chart door and the rim would not land.
#[test]
fn r1_the_lift_door_is_the_designated_faces_solids() {
    let (r, h, t1, t2) = (1.0, 2.0, 0.2, 0.05);
    let hollow_vessel = topo::shell(&vessel(r, h), t1, tol())
        .expect("the vessel hollows")
        .body;
    let pair = beside(&hollow_vessel, &boxy(2.0, 3.0, 4.0), 10.0);
    // The void's ceiling: the z = h - t1 plane on the vessel's solid.
    let ves = pair.solids().map(|(k, _)| k).next().unwrap();
    // `vessel` revolves about +y, so the void's CEILING is the plane at
    // `y = h - t1` — a chart of two faces, split by the revolve's seam.
    let ceiling: Vec<FaceKey> = faces_of(&pair, ves)
        .into_iter()
        .filter(|&f| {
            let d = pair.get_face(f).unwrap();
            matches!(
                pair.get_surface(d.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.x.abs() < 1e-9 && normal.z.abs() < 1e-9
                        && (origin.y - (h - t1)).abs() < 1e-9
            )
        })
        .collect();
    assert_eq!(
        ceiling.len(),
        2,
        "the void ceiling chart, two faces at the seam"
    );
    let opened = topo::shell_open(&pair, t2, &ceiling, tol())
        .expect("the vessel's void ceiling opens beside a box")
        .body;
    println!(
        "[r1] lift beside a box: solids={} shells={} volume={:.12}",
        opened.solids().count(),
        opened.shells().count(),
        volume(&opened)
    );
    assert_eq!(
        topo::validate_geometric(&opened, tol()),
        Ok(()),
        "the opened pair is tier-3 valid"
    );
}

// ---------------------------------------------------------------------
// Claim 4 — the order assertion
// ---------------------------------------------------------------------

/// **Can the clone's solid order differ from the operand's?** The verb
/// asserts `cavity_solids == solids == out_solids` and refuses
/// `Corrupt`. Both are `body.clone()`, and a `SlotMap` clone preserves
/// slots and versions — so the assertion is a tautology on every body
/// the verb can reach. Measured over a body whose solid arena has been
/// churned: solids grafted in, one killed by the partition, more
/// grafted after.
#[test]
fn r1_the_solid_order_assertion_is_a_tautology_on_a_clone() {
    let churned = beside(
        &beside(&hollow_box(), &boxy(1.0, 1.0, 1.0), 10.0),
        &boxy(1.0, 1.0, 1.0),
        20.0,
    );
    let a: Vec<SolidKey> = churned.solids().map(|(k, _)| k).collect();
    let b: Vec<SolidKey> = churned.clone().solids().map(|(k, _)| k).collect();
    assert_eq!(a, b, "a clone's solid arena is the operand's, key for key");
    // And after the verb's own partition, which MINTS solids:
    let out = topo::shell(&churned, 0.02, tol()).expect("it shells").body;
    let c: Vec<SolidKey> = out.solids().map(|(k, _)| k).collect();
    let d: Vec<SolidKey> = out.clone().solids().map(|(k, _)| k).collect();
    assert_eq!(c, d, "so is a minted-solid body's");
    println!(
        "[r1] solid order: operand {} solids, result {} solids, clone equal in both",
        a.len(),
        c.len()
    );
}

// ---------------------------------------------------------------------
// The end-to-end exercise, from a consumer's seat
// ---------------------------------------------------------------------

/// **Two parts in one body, hollowed in one call, then opened.** The
/// consumer's route: build two parts, place them with `graft_disjoint`,
/// hollow, then name a wall to open. The friction this row records is
/// how a user NAMES "the inner wall of the second part" after two
/// hollowings — there is no name, only a geometric search of the
/// result's faces.
#[test]
fn r1_e2e_two_parts_one_body() {
    let t = 0.05;
    let assembly = beside(&boxy(2.0, 3.0, 4.0), &vessel(1.0, 2.0), 10.0);
    assert_eq!(
        topo::validate_geometric(&assembly, tol()),
        Ok(()),
        "the assembly is valid"
    );
    let hollow = topo::shell(&assembly, t, tol())
        .expect("both parts hollow in one call")
        .body;
    let wall_box = v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9);
    let wall_ves =
        core::f64::consts::PI * (1.0 * 1.0 * 2.0 - (1.0 - t) * (1.0 - t) * (2.0 - 2.0 * t));
    println!(
        "[r1] e2e hollow: solids={} shells={} volume={:.12} closed form {:.12}",
        hollow.solids().count(),
        hollow.shells().count(),
        volume(&hollow),
        wall_box + wall_ves
    );
    assert_eq!(hollow.solids().count(), 2);
    assert_eq!(hollow.shells().count(), 4);
    assert!((volume(&hollow) - (wall_box + wall_ves)).abs() < 1e-12);

    // Open the box part's ceiling: the z = 4 plane on solid 0.
    let bx = hollow.solids().map(|(k, _)| k).next().unwrap();
    let lid: Vec<FaceKey> = faces_of(&hollow, bx)
        .into_iter()
        .filter(|&f| {
            let d = hollow.get_face(f).unwrap();
            matches!(
                hollow.get_surface(d.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
                        && (origin.z - 4.0).abs() < 1e-9
            )
        })
        .collect();
    assert_eq!(lid.len(), 1, "one outer lid on the box");
    // **Friction, measured.** A second `shell` at the SAME `t` refuses
    // `WallClearance { gap: 0.05, needed: 0.1 }`: the first hollowing
    // left a 0.05 wall and the second needs `2t` of it. The consumer
    // has to know the wall it just built to pick the next thickness;
    // nothing in the verb's vocabulary tells them.
    let e = topo::shell_open(&hollow, t, &lid, tol())
        .expect_err("the same thickness twice does not fit the wall it just built");
    println!("[r1] e2e friction: a second hollow at the same t refuses — {e}");
    let t2 = 0.02;
    let opened = topo::shell_open(&hollow, t2, &lid, tol())
        .expect("a thinner second wall opens the box and leaves the vessel sealed")
        .body;
    let pi = core::f64::consts::PI;
    let closed = (v(2.0, 3.0, 4.0) - v(1.96, 2.96, 3.96))
        + (v(1.94, 2.94, 3.94) - v(1.9, 2.9, 3.9))
        + pi * (1.0 * 1.0 * 2.0 - 0.98 * 0.98 * 1.96)
        + pi * (0.97 * 0.97 * 1.94 - 0.95 * 0.95 * 1.9)
        - 1.96 * 2.96 * t2;
    println!(
        "[r1] e2e opened: solids={} shells={} volume={:.12} closed form {:.12}",
        opened.solids().count(),
        opened.shells().count(),
        volume(&opened),
        closed
    );
    assert_eq!(opened.solids().count(), 4, "four thin solids");
    assert_eq!(
        opened.shells().count(),
        7,
        "eight shells less the fused one"
    );
    assert!(
        (volume(&opened) - closed).abs() < 1e-12,
        "the four-solid opened assembly is the closed form: {} vs {closed}",
        volume(&opened)
    );
    assert_eq!(
        topo::validate_geometric(&opened, tol()),
        Ok(()),
        "the opened assembly is tier-3 valid"
    );
    let mesh = mesh::tessellate(&opened, 5e-3, tol()).expect("the opened assembly tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    println!(
        "[r1] e2e tessellation: {} patches, {} positions, watertight",
        mesh.patches.len(),
        mesh.positions.len()
    );
}

/// **`shell_open`'s designation vocabulary, from a user's seat.** After
/// two hollowings there are four thin solids and eight shells, and the
/// only handle a user has on "the inner wall of the second part" is a
/// `FaceKey` found by walking the result's geometry. This row records
/// what that walk costs.
#[test]
fn r1_naming_the_inner_wall_after_two_hollowings() {
    let once = topo::shell(&boxy(2.0, 3.0, 4.0), 0.25, tol())
        .expect("first hollow")
        .body;
    let twice = topo::shell(&once, 0.05, tol()).expect("second hollow").body;
    println!(
        "[r1] after two hollowings: solids={} shells={} faces={}",
        twice.solids().count(),
        twice.shells().count(),
        twice.faces().count()
    );
    // Every z-normal plane in the result, with its solid and shell —
    // the whole search a user must do to pick "the inner wall".
    let mut rows: Vec<String> = Vec::new();
    for (f, d) in twice.faces() {
        if let Some(geom::Surface::Plane { origin, normal, .. }) = twice.get_surface(d.surface)
            && normal.x.abs() < 1e-9
            && normal.y.abs() < 1e-9
        {
            let shell: ShellKey = d.shell;
            rows.push(format!(
                "z={:.2} n_z={:+.0} solid={:?} shell={shell:?} face={f:?}",
                origin.z,
                normal.z,
                solid_of(&twice, f)
            ));
        }
    }
    rows.sort();
    for r in &rows {
        println!("[r1] candidate {r}");
    }
    println!(
        "[r1] {} z-normal candidate faces to choose from",
        rows.len()
    );
}

/// **The closed form of the hollow-hollow-open row, re-derived.**
/// Independent arithmetic against the PR's `1.706688`.
#[test]
fn r1_rederive_the_hollow_hollow_open_closed_form() {
    let terms = [
        v(2.0, 3.0, 4.0) - v(1.98, 2.98, 3.98),
        v(1.92, 2.92, 3.92) - v(1.9, 2.9, 3.9),
        v(1.6, 2.6, 3.6) - v(1.58, 2.58, 3.58),
        v(1.52, 2.52, 3.52) - v(1.5, 2.5, 3.5),
    ];
    let lid = 1.52 * 2.52 * 0.01;
    let total: f64 = terms.iter().sum::<f64>() - lid;
    println!("[r1] terms {terms:?} lid {lid} total {total:.12}");
    assert!(
        (total - 1.706688).abs() < 1e-12,
        "the PR's figure re-derives: {total}"
    );
}

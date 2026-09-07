//! DOCM-8 review lane R2 probes (not a suite of record).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{
    block, body_of, declared_union, failure, flush_pairs, member_face, run, table,
};
use crate::fixture::{Recorder, ang, fname, len, scl, wall};

use editor_core::{
    BooleanOp, CapEnd, EntityKind, Node, ProfileDoc, RecipeNodeId, Resolution, RoleSeg, RunCtx,
    StableName, resolve,
};
use geom_core::Tol;

fn permutations(items: &[RecipeNodeId]) -> Vec<Vec<RecipeNodeId>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (i, &head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, head);
            out.push(tail);
        }
    }
    out
}

fn chain_pairs(union: RecipeNodeId, chain: &[RecipeNodeId]) -> Vec<(StableName, StableName)> {
    chain
        .windows(2)
        .flat_map(|w| flush_pairs(union, (w[0], w[0]), (w[1], w[1])))
        .collect()
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass")
        .volume
}

/// Every merged row of a table, as a sorted list of names.
fn merged_rows(t: &editor_core::NameTable) -> Vec<StableName> {
    let mut v: Vec<StableName> = t
        .iter()
        .filter(|(n, _)| n.path.iter().any(|s| matches!(s, RoleSeg::Merged(_))))
        .map(|(n, _)| n.clone())
        .collect();
    v.sort();
    v
}

/// P1 (C1): a FIVE-member chain, all 120 orders — one body, and the
/// merged-row SET (not just the volume) identical across every order.
#[test]
fn r2_p1_five_member_chain_fuses_in_all_120_orders_with_one_row_set() {
    let doc = ProfileDoc::empty_derived("r2_chain5", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, e) = block(doc, (1.9, 2.9), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (2.6, 3.6), (0.0, 1.0), 0.0, 1.0);
    let chain = [a, c, d, e, g];
    let mut reference: Option<Vec<StableName>> = None;
    let mut refusals = Vec::new();
    let orders = permutations(&chain);
    assert_eq!(orders.len(), 120);
    for order in orders {
        let (docx, union, _) = declared_union(doc.clone(), &order, |u| chain_pairs(u, &chain));
        let ev = run(&docx);
        if let Some(f) = failure(&ev, union) {
            refusals.push(format!("{order:?}: {f:?}"));
            continue;
        }
        let v = volume(&body_of(&ev, union));
        assert!((v - 3.6).abs() < 1e-9, "{order:?}: volume {v}");
        let rows = merged_rows(table(&ev, union));
        assert_eq!(rows.len(), 4, "{order:?}: {rows:?}");
        for r in &rows {
            let RoleSeg::Merged(set) = &r.path[0] else {
                panic!()
            };
            assert_eq!(set.len(), 5, "{order:?}: flat set of five, got {r}");
            assert!(
                set.iter()
                    .all(|c| matches!(c.path.as_slice(), [RoleSeg::FromMember { .. }])),
                "{order:?}: a constituent is not a bare member face: {r}"
            );
        }
        match &reference {
            None => reference = Some(rows),
            Some(want) => assert_eq!(&rows, want, "{order:?}: row set differs"),
        }
    }
    assert!(refusals.is_empty(), "refused orders:\n{}", refusals.join("\n"));
}

/// P2 (C1): a chain whose middle member touches THREE neighbours —
/// `a` and `d` along x (four flush families each) and `f` along y,
/// declared on the caps only. Every order of the four.
#[test]
fn r2_p2_middle_member_with_three_neighbours_every_order() {
    let doc = ProfileDoc::empty_derived("r2_star", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, f) = block(doc, (0.5, 1.5), (0.5, 1.5), 0.0, 1.0);
    let pairs = |u: RecipeNodeId| {
        let mut v = chain_pairs(u, &[a, c, d]);
        for seg in [RoleSeg::Cap(CapEnd::Start), RoleSeg::Cap(CapEnd::End)] {
            v.push((
                member_face(u, c, fname(c, seg.clone())),
                member_face(u, f, fname(f, seg)),
            ));
        }
        v
    };
    let mut reference: Option<Vec<StableName>> = None;
    let mut outcomes = Vec::new();
    for order in permutations(&[a, c, d, f]) {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        match failure(&ev, union) {
            Some(fl) => outcomes.push(format!("{order:?}: REFUSED {fl:?}")),
            None => {
                let v = volume(&body_of(&ev, union));
                let rows = merged_rows(table(&ev, union));
                outcomes.push(format!("{order:?}: fused v={v} rows={}", rows.len()));
                match &reference {
                    None => reference = Some(rows),
                    Some(want) => {
                        if &rows != want {
                            outcomes.push(format!("   ROW SET DIFFERS: {rows:?}"));
                        }
                    }
                }
            }
        }
    }
    eprintln!("P2 outcomes:\n{}", outcomes.join("\n"));
    assert!(
        outcomes.iter().all(|o| o.contains("fused") && !o.contains("DIFFERS")),
        "P2:\n{}",
        outcomes.join("\n")
    );
}

/// P2b (C1): the same star, but the y-neighbour also declares its
/// x-walls flush with the middle member's — walls that `a` and `d`
/// swallow in some orders. Records what each order does.
#[test]
fn r2_p2b_star_with_wall_declarations_every_order() {
    let doc = ProfileDoc::empty_derived("r2_star_walls", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, f) = block(doc, (0.5, 1.5), (0.5, 1.5), 0.0, 1.0);
    let pairs = |u: RecipeNodeId| {
        let mut v = chain_pairs(u, &[a, c, d]);
        for seg in [
            wall(1),
            wall(3),
            RoleSeg::Cap(CapEnd::Start),
            RoleSeg::Cap(CapEnd::End),
        ] {
            v.push((
                member_face(u, c, fname(c, seg.clone())),
                member_face(u, f, fname(f, seg)),
            ));
        }
        v
    };
    let mut outcomes = Vec::new();
    for order in permutations(&[a, c, d, f]) {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        match failure(&ev, union) {
            Some(fl) => outcomes.push(format!("{order:?}: REFUSED {fl:?}")),
            None => {
                let v = volume(&body_of(&ev, union));
                outcomes.push(format!("{order:?}: fused v={v}"));
            }
        }
    }
    eprintln!("P2b outcomes:\n{}", outcomes.join("\n"));
    assert!(
        outcomes.iter().all(|o| o.contains("fused")),
        "P2b:\n{}",
        outcomes.join("\n")
    );
}

/// P2c (C1): the star with EVERY coplanar contact declared — the
/// x-chain's four families, `c`–`f`'s x-walls and caps, and the
/// `a`–`f` / `d`–`f` cap overlaps. Records what each order does.
#[test]
fn r2_p2c_star_fully_declared_every_order() {
    let doc = ProfileDoc::empty_derived("r2_star_full", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    let (doc, f) = block(doc, (0.5, 1.5), (0.5, 1.5), 0.0, 1.0);
    let pairs = |u: RecipeNodeId| {
        let mut v = chain_pairs(u, &[a, c, d]);
        for seg in [wall(1), wall(3)] {
            v.push((
                member_face(u, c, fname(c, seg.clone())),
                member_face(u, f, fname(f, seg)),
            ));
        }
        for seg in [RoleSeg::Cap(CapEnd::Start), RoleSeg::Cap(CapEnd::End)] {
            for m in [a, c, d] {
                v.push((
                    member_face(u, m, fname(m, seg.clone())),
                    member_face(u, f, fname(f, seg.clone())),
                ));
            }
        }
        v
    };
    let mut outcomes = Vec::new();
    let mut fused = 0;
    let mut vanished = 0;
    for order in permutations(&[a, c, d, f]) {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        match failure(&ev, union) {
            Some(fl) => {
                let s = format!("{fl:?}");
                if s.contains("Vanished") {
                    vanished += 1;
                }
                outcomes.push(format!("{order:?}: REFUSED {s}"));
            }
            None => {
                fused += 1;
                let v = volume(&body_of(&ev, union));
                let rows = merged_rows(table(&ev, union));
                outcomes.push(format!("{order:?}: fused v={v} rows={}", rows.len()));
            }
        }
    }
    eprintln!("P2c outcomes (fused {fused}, vanished {vanished}):\n{}", outcomes.join("\n"));
    assert!(fused == 24, "P2c: {fused} of 24 orders fused, {vanished} refused Vanished");
}

/// P3 (C1): a chain built from PLACEMENTS of one prototype — three
/// transforms of one block, declared member-to-member; every order.
#[test]
fn r2_p3_chain_of_placements_of_one_prototype_every_order() {
    let doc = ProfileDoc::empty_derived("r2_placements", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let mut doc = doc;
    let mut placed = Vec::new();
    for dx in [0.0, 0.5, 1.2] {
        let (d2, t) = crate::fixture::insert(
            doc,
            Node::Transform {
                input: proto,
                translation: [len(dx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        );
        doc = d2;
        placed.push(t);
    }
    let chain = placed.clone();
    let pairs = |u: RecipeNodeId| {
        chain
            .windows(2)
            .flat_map(|w| flush_pairs(u, (w[0], proto), (w[1], proto)))
            .collect::<Vec<_>>()
    };
    let mut reference: Option<Vec<StableName>> = None;
    let mut outcomes = Vec::new();
    for order in permutations(&chain) {
        let (docx, union, _) = declared_union(doc.clone(), &order, pairs);
        let ev = run(&docx);
        match failure(&ev, union) {
            Some(fl) => outcomes.push(format!("{order:?}: REFUSED {fl:?}")),
            None => {
                let v = volume(&body_of(&ev, union));
                let rows = merged_rows(table(&ev, union));
                outcomes.push(format!("{order:?}: fused v={v} rows={}", rows.len()));
                match &reference {
                    None => reference = Some(rows),
                    Some(want) => {
                        if &rows != want {
                            outcomes.push(format!("   ROW SET DIFFERS: {rows:?}"));
                        }
                    }
                }
            }
        }
    }
    eprintln!("P3 outcomes:\n{}", outcomes.join("\n"));
    if let Some(r) = &reference {
        eprintln!("P3 rows: {r:#?}");
    }
    assert!(
        outcomes.iter().all(|o| o.contains("fused v=2.2") && !o.contains("DIFFERS")),
        "P3:\n{}",
        outcomes.join("\n")
    );
}

/// P4 (emphasis i): a merged cap SPLIT by a later member (a slab
/// rising through it), then merged again with a fourth member.
/// Measures: (a) the member-space spelling of a face inside the
/// fragmented merged row; (b) the accumulation spelling of each
/// fragment; and whether mint, collapse and the walker agree.
#[test]
fn r2_p4_fragment_of_a_merged_face_as_a_constituent() {
    let doc = ProfileDoc::empty_derived("r2_fragment", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    // A thin slab through the merged top cap, splitting it in x.
    let (doc, g) = block(doc, (0.7, 0.8), (-1.0, 2.0), 0.5, 3.0);
    let (doc, c) = block(doc, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    // Step 0: [a, b, g] alone — what rows does the fragmented cap get?
    let (doc0, u0, _) = declared_union(doc.clone(), &[a, b, g], |u| flush_pairs(u, (a, a), (b, b)));
    let ev0 = run(&doc0);
    if let Some(f) = failure(&ev0, u0) {
        panic!("P4 step 0 [a,b,g] refused: {f:?}");
    }
    let t0 = table(&ev0, u0);
    let frag_rows: Vec<StableName> = t0
        .iter()
        .filter(|(n, _)| {
            matches!(n.path.first(), Some(RoleSeg::Merged(_))) && n.path.len() > 1
        })
        .map(|(n, _)| n.clone())
        .collect();
    eprintln!("P4 fragment rows of [a,b,g]: {frag_rows:#?}");
    // (a) member spelling: b's END cap (z = 1, the split one) with c's.
    let (doc_a, ua, _) = declared_union(doc.clone(), &[a, b, g, c], |u| {
        let mut v = flush_pairs(u, (a, a), (b, b));
        v.push((
            member_face(u, b, fname(b, RoleSeg::Cap(CapEnd::End))),
            member_face(u, c, fname(c, RoleSeg::Cap(CapEnd::End))),
        ));
        v
    });
    let eva = run(&doc_a);
    eprintln!("P4(a) member spelling into a fragmented row: {:?}", failure(&eva, ua));
    // (b) accumulation spelling: each fragment row in turn.
    let mut fused_with = Vec::new();
    for fr in &frag_rows {
        let (doc_b, ub, _) = declared_union(doc.clone(), &[a, b, g, c], |u| {
            let mut v = flush_pairs(u, (a, a), (b, b));
            let mut row = fr.clone();
            row.node = u;
            v.push((row, member_face(u, c, fname(c, RoleSeg::Cap(CapEnd::End)))));
            v
        });
        let evb = run(&doc_b);
        match failure(&evb, ub) {
            Some(fl) => eprintln!("P4(b) fragment {fr}: REFUSED {fl:?}"),
            None => {
                let tb = table(&evb, ub);
                let rows = merged_rows(tb);
                eprintln!(
                    "P4(b) fragment {fr}: fused v={} merged rows:\n{:#?}",
                    volume(&body_of(&evb, ub)),
                    rows
                );
                fused_with.push(fr.clone());
            }
        }
    }
    eprintln!("P4 fragments that fused with c: {}", fused_with.len());
}

/// P6 (regression probe): the retired name of an INNER merged face
/// carried into an outer boolean. Before the flat mint it was a
/// constituent of the outer row (N3: "referencing one fails with the
/// merged name offered"); now it is a constituent of nothing.
#[test]
fn r2_p6_retired_inner_merged_face_offers_at_the_outer_boolean() {
    let mut rec = Recorder::new();
    let blk = |rec: &mut Recorder, (x0, x1): (f64, f64)| {
        let p = rec.profile(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, 0.0), (x1, 0.0), (x1, 1.0), (x0, 1.0)]],
        );
        rec.insert(Node::Extrude {
            profile: p,
            distance: len(1.0),
        })
    };
    let a = blk(&mut rec, (0.0, 1.0));
    let b = blk(&mut rec, (0.5, 1.5));
    let segs = [
        wall(0),
        wall(2),
        RoleSeg::Cap(CapEnd::Start),
        RoleSeg::Cap(CapEnd::End),
    ];
    let decl_ab = rec.insert(Node::declare_rest(
        segs.iter()
            .map(|seg| (fname(a, seg.clone()), fname(b, seg.clone())))
            .collect(),
    ));
    let inner = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a,
        b,
        declare: Some(decl_ab),
    });
    let c = blk(&mut rec, (1.2, 2.2));
    let wrap = |node, seg: fn(Box<StableName>) -> RoleSeg, inner: StableName| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![seg(Box::new(inner))],
    };
    let inner_row = |seg: RoleSeg| {
        let mut set = vec![
            wrap(inner, RoleSeg::FromA, fname(a, seg.clone())),
            wrap(inner, RoleSeg::FromB, fname(b, seg)),
        ];
        set.sort();
        StableName {
            kind: EntityKind::Face,
            node: inner,
            path: vec![RoleSeg::Merged(set)],
        }
    };
    let decl_ic = rec.insert(Node::declare_rest(
        segs.iter()
            .map(|seg| (inner_row(seg.clone()), fname(c, seg.clone())))
            .collect(),
    ));
    let outer = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: inner,
        b: c,
        declare: Some(decl_ic),
    });
    let ev = run(&rec.doc);
    assert!(failure(&ev, outer).is_none());
    let ctx = RunCtx {
        doc: &rec.doc,
        eval: &ev,
    };
    let seg = RoleSeg::Cap(CapEnd::End);
    let outer_row = table(&ev, outer)
        .iter()
        .find(|(n, _)| match n.path.first() {
            Some(RoleSeg::Merged(set)) => set
                .iter()
                .any(|x| matches!(&x.path[0], RoleSeg::FromB(i) if i.path[0] == seg)),
            _ => false,
        })
        .map(|(n, _)| n.clone())
        .unwrap();
    eprintln!("P6 outer row: {outer_row}");
    // The consumed operand face of the outer boolean: the inner merge, wrapped.
    let consumed = wrap(outer, RoleSeg::FromA, inner_row(seg.clone()));
    let r1 = resolve(ctx, &consumed);
    eprintln!("P6 resolve(FromA(inner Merged)) at outer = {r1:#?}");
    // One flat constituent: never a row anywhere.
    let flat = wrap(outer, RoleSeg::FromA, wrap(inner, RoleSeg::FromA, fname(a, seg.clone())));
    let r2 = resolve(ctx, &flat);
    eprintln!("P6 resolve(FromA(FromA(a.cap))) at outer = {r2:#?}");
    match r1 {
        Resolution::Failed(f) => assert!(
            f.offers.contains(&outer_row),
            "the consumed inner merged face no longer offers the outer row: offers={:?}",
            f.offers
        ),
        other => panic!("{other:?}"),
    }
}

/// P2d: isolates the star's area-overlap cap contact — two blocks
/// overlapping in a corner square, caps declared, nothing else.
#[test]
fn r2_p2d_area_overlap_caps_declared_alone() {
    let doc = ProfileDoc::empty_derived("r2_overlap", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, f) = block(doc, (0.5, 1.5), (0.5, 1.5), 0.0, 1.0);
    for order in [[a, f], [f, a]] {
        let (docx, union, _) = declared_union(doc.clone(), &order, |u| {
            [RoleSeg::Cap(CapEnd::Start), RoleSeg::Cap(CapEnd::End)]
                .into_iter()
                .map(|seg| {
                    (
                        member_face(u, a, fname(a, seg.clone())),
                        member_face(u, f, fname(f, seg)),
                    )
                })
                .collect()
        });
        let ev = run(&docx);
        eprintln!("P2d {order:?}: {:?}", failure(&ev, union));
    }
}

/// P4b (emphasis i, pair-boolean route): a declared merge, then a
/// SUBTRACT of a slab through the merged cap, then a union with a
/// fourth block declared against one fragment of the merged cap.
#[test]
fn r2_p4b_fragment_of_a_merged_face_via_pair_booleans() {
    let mut rec = Recorder::new();
    let blk = |rec: &mut Recorder, (x0, x1): (f64, f64), (y0, y1): (f64, f64), z0: f64, dz: f64| {
        let p = rec.profile(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
        );
        rec.insert(Node::Extrude {
            profile: p,
            distance: len(dz),
        })
    };
    let a = blk(&mut rec, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let b = blk(&mut rec, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let segs = [
        wall(0),
        wall(2),
        RoleSeg::Cap(CapEnd::Start),
        RoleSeg::Cap(CapEnd::End),
    ];
    let decl_ab = rec.insert(Node::declare_rest(
        segs.iter()
            .map(|seg| (fname(a, seg.clone()), fname(b, seg.clone())))
            .collect(),
    ));
    let inner = rec.insert(Node::Boolean {
        op: BooleanOp::Union,
        a,
        b,
        declare: Some(decl_ab),
    });
    let g = blk(&mut rec, (0.7, 0.8), (-1.0, 2.0), 0.5, 3.0);
    let cut = rec.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: inner,
        b: g,
        declare: None,
    });
    let ev = run(&rec.doc);
    eprintln!("P4b cut: {:?}", failure(&ev, cut));
    if failure(&ev, cut).is_some() {
        return;
    }
    let frags: Vec<StableName> = table(&ev, cut)
        .iter()
        .filter(|(n, _)| n.path.iter().any(|s| matches!(s, RoleSeg::Merged(_))))
        .map(|(n, _)| n.clone())
        .collect();
    eprintln!(
        "P4b rows of the cut carrying a Merged:\n{}",
        frags.iter().map(|n| format!("  {n:?}")).collect::<Vec<_>>().join("\n")
    );
    let c = blk(&mut rec, (1.2, 2.2), (0.0, 1.0), 0.0, 1.0);
    for fr in frags.iter().filter(|n| n.path.len() > 1) {
        let mut r2 = Recorder {
            doc: rec.doc.clone(),
            edits: Vec::new(),
        };
        let decl = r2.insert(Node::declare_rest(vec![(
            fr.clone(),
            fname(c, RoleSeg::Cap(CapEnd::End)),
        )]));
        let outer = r2.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: cut,
            b: c,
            declare: Some(decl),
        });
        let ev2 = run(&r2.doc);
        match failure(&ev2, outer) {
            Some(f) => eprintln!("P4b outer with {fr:?}: REFUSED {f:?}"),
            None => {
                let rows = merged_rows(table(&ev2, outer));
                eprintln!(
                    "P4b outer with {fr:?}: fused; merged rows:\n{}",
                    rows.iter().map(|n| format!("  {n:?}")).collect::<Vec<_>>().join("\n")
                );
            }
        }
    }
}

/// P7 (prose claim): the PR's own vanished fixture — `a`'s x = 1 wall
/// inside `big`, declared against `far`'s wall — in every member
/// order. Does "reordering the members changes nothing about whether
/// it resolves" hold for a face consumed by containment?
#[test]
fn r2_p7_containment_consumed_face_by_order() {
    let doc = ProfileDoc::empty_derived("r2_vanished_orders", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, big) = block(doc, (0.5, 3.0), (-1.0, 2.0), -1.0, 3.0);
    let (doc, far) = block(doc, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let mut outcomes = Vec::new();
    for order in permutations(&[a, big, far]) {
        let (docx, union, _) = declared_union(doc.clone(), &order, |u| {
            vec![(
                member_face(u, a, fname(a, wall(1))),
                member_face(u, far, fname(far, wall(3))),
            )]
        });
        let ev = run(&docx);
        let s = format!("{:?}", failure(&ev, union));
        outcomes.push(format!("{order:?}: {}", &s[..s.len().min(140)]));
    }
    eprintln!("P7 outcomes:\n{}", outcomes.join("\n"));
}

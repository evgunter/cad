//! M4 PR 3 naming tests, part 1 (spec D6): per-op role coverage for
//! the sweep ops, split, transform pass-through, pattern wrapping,
//! and Declare pairs resolving against the real tables.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, CapEnd, Datum, EntityKind, Entry, EvalOptions, Evaluation, MeridianEnd, NameTable,
    Node, ProfileDoc, ProfileEdgeRef, ProfileVertexRef, RecipeNodeId, RoleSeg, SplitHalf,
    StableName, evaluate,
};
use fixture::{ang, desc, insert, len};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default())
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

fn pe(l: u32, s: u32) -> ProfileEdgeRef {
    ProfileEdgeRef {
        loop_index: l,
        segment: s,
    }
}

fn pv(l: u32, v: u32) -> ProfileVertexRef {
    ProfileVertexRef {
        loop_index: l,
        vertex: v,
    }
}

/// A unit-square profile + extrude prelude (xy plane).
fn cube(doc: ProfileDoc, x0: f64, side: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (x0, 0.0),
                (x0 + side, 0.0),
                (x0 + side, side),
                (x0, side),
            ]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(side),
        },
    )
}

// ---- Extrude: the full role inventory of a cube, exact. ----

#[test]
fn extrude_names_every_boundary_entity_with_the_d2_roles() {
    let (doc, ext) = cube(ProfileDoc::empty_derived("m4_pr3_names"), 0.0, 1.0);
    let ev = run(&doc);
    let t = table(&ev, ext);
    // 1 body + 6 faces + 12 edges + 8 vertices.
    assert_eq!(t.len(), 27);
    // Body + caps.
    assert!(
        t.lookup(&name1(EntityKind::Body, ext, RoleSeg::OutputBody))
            .is_some()
    );
    for end in [CapEnd::Top, CapEnd::Bottom] {
        assert!(
            t.lookup(&name1(EntityKind::Face, ext, RoleSeg::Cap(end)))
                .is_some()
        );
    }
    // Per canonical segment: lateral, both rims; per vertex: strut,
    // both cap vertices.
    for s in 0..4 {
        assert!(
            t.lookup(&name1(EntityKind::Face, ext, RoleSeg::Lateral(pe(0, s))))
                .is_some()
        );
        for end in [CapEnd::Top, CapEnd::Bottom] {
            assert!(
                t.lookup(&name1(
                    EntityKind::Edge,
                    ext,
                    RoleSeg::RimEdge(end, pe(0, s))
                ))
                .is_some()
            );
            assert!(
                t.lookup(&name1(
                    EntityKind::Vertex,
                    ext,
                    RoleSeg::CapVertex(end, pv(0, s))
                ))
                .is_some()
            );
        }
        assert!(
            t.lookup(&name1(
                EntityKind::Edge,
                ext,
                RoleSeg::LateralEdge(pv(0, s))
            ))
            .is_some()
        );
    }
    // Every entry unique (no ties in a plain extrude), kinds agree.
    for (n, e) in t.iter() {
        match e {
            Entry::Unique(r) => assert_eq!(n.kind, r.key.kind()),
            Entry::Tied(_) => panic!("unexpected tie in an extrude table: {n:?}"),
        }
    }
}

// ---- Revolve: the M2 band/pole/seam taxonomy, all four shapes. ----

/// Profile on the xy plane (the y datum axis lies in it).
fn revolve_doc(pts: Vec<(f64, f64)>, angle: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m4_pr3_names");
    let (doc, p) = insert(
        doc,
        Node::Profile(desc([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts])),
    );
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [fixture::scl(0.0), fixture::scl(1.0), fixture::scl(0.0)],
        }),
    );
    insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(angle),
        },
    )
}

#[test]
fn partial_revolve_offset_names_bands_rims_caps_meridians() {
    let (doc, rev) = revolve_doc(
        vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
        std::f64::consts::FRAC_PI_2,
    );
    let ev = run(&doc);
    let t = table(&ev, rev);
    // Wedge annulus-quarter: 1 body + (4 bands + 2 caps) + (4 rims +
    // 8 meridians) + 8 meridian vertices.
    assert_eq!(t.len(), 27);
    for m in [MeridianEnd::Start, MeridianEnd::End] {
        assert!(
            t.lookup(&name1(EntityKind::Face, rev, RoleSeg::RevolveCap(m)))
                .is_some()
        );
    }
    for s in 0..4 {
        assert!(
            t.lookup(&name1(EntityKind::Face, rev, RoleSeg::Band(pe(0, s))))
                .is_some()
        );
        assert!(
            t.lookup(&name1(EntityKind::Edge, rev, RoleSeg::BandRim(pv(0, s))))
                .is_some()
        );
        for m in [MeridianEnd::Start, MeridianEnd::End] {
            assert!(
                t.lookup(&name1(
                    EntityKind::Edge,
                    rev,
                    RoleSeg::Meridian(m, pe(0, s))
                ))
                .is_some()
            );
            assert!(
                t.lookup(&name1(
                    EntityKind::Vertex,
                    rev,
                    RoleSeg::MeridianVertex(m, pv(0, s))
                ))
                .is_some()
            );
        }
    }
}

#[test]
fn partial_revolve_on_axis_names_axis_edge_and_poles() {
    // Segment 3 runs (0,1) → (0,0): ON the y axis.
    let (doc, rev) = revolve_doc(
        vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        std::f64::consts::FRAC_PI_2,
    );
    let ev = run(&doc);
    let t = table(&ev, rev);
    assert!(
        t.lookup(&name1(EntityKind::Edge, rev, RoleSeg::AxisEdge(pe(0, 3))))
            .is_some()
    );
    // The two on-axis profile vertices are poles.
    for v in [0, 3] {
        assert!(
            t.lookup(&name1(EntityKind::Vertex, rev, RoleSeg::Pole(pv(0, v))))
                .is_some()
        );
    }
    // Off-axis vertices carry start+end copies.
    for v in [1, 2] {
        for m in [MeridianEnd::Start, MeridianEnd::End] {
            assert!(
                t.lookup(&name1(
                    EntityKind::Vertex,
                    rev,
                    RoleSeg::MeridianVertex(m, pv(0, v))
                ))
                .is_some()
            );
        }
    }
}

#[test]
fn full_lamina_revolve_names_seam_chain_and_full_rims() {
    let (doc, rev) = revolve_doc(
        vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
        std::f64::consts::TAU,
    );
    let ev = run(&doc);
    let t = table(&ev, rev);
    // Square torus: 1 body + 4 bands + (4 rims + 4 seam meridians) +
    // 4 meridian vertices.
    assert_eq!(t.len(), 17);
    for s in 0..4 {
        assert!(
            t.lookup(&name1(EntityKind::Face, rev, RoleSeg::Band(pe(0, s))))
                .is_some()
        );
        assert!(
            t.lookup(&name1(EntityKind::Edge, rev, RoleSeg::BandRim(pv(0, s))))
                .is_some()
        );
        assert!(
            t.lookup(&name1(
                EntityKind::Edge,
                rev,
                RoleSeg::Meridian(MeridianEnd::Seam, pe(0, s))
            ))
            .is_some()
        );
        assert!(
            t.lookup(&name1(
                EntityKind::Vertex,
                rev,
                RoleSeg::MeridianVertex(MeridianEnd::Seam, pv(0, s))
            ))
            .is_some()
        );
    }
}

#[test]
fn full_wire_revolve_names_pi_band_and_poles() {
    // Cylinder: side 3 on the axis, full revolve → the wire case.
    let (doc, rev) = revolve_doc(
        vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        std::f64::consts::TAU,
    );
    let ev = run(&doc);
    let t = table(&ev, rev);
    // π-band roles exist (the wire case's second half).
    assert!(
        t.iter()
            .any(|(n, _)| matches!(n.path.first(), Some(RoleSeg::BandPi(_)))),
        "no BandPi role minted"
    );
    assert!(
        t.iter()
            .any(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Meridian(MeridianEnd::Pi, _)))),
        "no Meridian(Pi) role minted"
    );
    assert!(
        t.iter()
            .any(|(n, _)| matches!(n.path.first(), Some(RoleSeg::BandRimPi(_)))),
        "no BandRimPi role minted"
    );
    // Two poles: the on-axis profile vertices.
    let poles = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Pole(_))))
        .count();
    assert_eq!(poles, 2);
}

// ---- Split: sections, fragments, crossings, pass-through. ----

#[test]
fn split_names_sections_fragments_and_crossings() {
    let (doc, ext) = cube(ProfileDoc::empty_derived("m4_pr3_names"), 0.0, 2.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(1.0)],
            normal: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: ext,
            tool: plane,
        },
    );
    let ev = run(&doc);
    let t = table(&ev, split);
    // Both halves total: 2 bodies + 12 faces + 24 edges + 16 vertices.
    assert_eq!(t.len(), 54);
    for side in [SplitHalf::Above, SplitHalf::Below] {
        assert!(
            t.lookup(&name1(EntityKind::Body, split, RoleSeg::SplitBody(side)))
                .is_some()
        );
        assert!(
            t.lookup(&name1(
                EntityKind::Face,
                split,
                RoleSeg::SectionFace { side, section: 0 }
            ))
            .is_some()
        );
    }
    // Caps pass through UNCHANGED (names still minted at the extrude;
    // the split contributes no segment): Top lives in the above body
    // (index 0), Bottom below (index 1).
    let top = t
        .lookup(&name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top)))
        .expect("top cap passes through");
    let bottom = t
        .lookup(&name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Bottom)))
        .expect("bottom cap passes through");
    match (top, bottom) {
        (Entry::Unique(a), Entry::Unique(b)) => {
            assert_eq!(a.body, 0);
            assert_eq!(b.body, 1);
        }
        other => panic!("caps not unique: {other:?}"),
    }
    // Each side wall is a side-tagged fragment of its lateral; each
    // strut edge likewise; each strut crossing is a CrossingVertex;
    // each cut wall contributes a SectionEdge.
    for side in [SplitHalf::Above, SplitHalf::Below] {
        for s in 0..4 {
            let lateral = name1(EntityKind::Face, ext, RoleSeg::Lateral(pe(0, s)));
            assert!(
                t.lookup(&name1(
                    EntityKind::Face,
                    split,
                    RoleSeg::SplitFragment {
                        side,
                        parent: Box::new(lateral.clone())
                    }
                ))
                .is_some(),
                "missing wall fragment side={side:?} seg={s}"
            );
            assert!(
                t.lookup(&name1(
                    EntityKind::Edge,
                    split,
                    RoleSeg::SectionEdge {
                        side,
                        face: Box::new(lateral)
                    }
                ))
                .is_some(),
                "missing section edge side={side:?} seg={s}"
            );
            let strut = name1(EntityKind::Edge, ext, RoleSeg::LateralEdge(pv(0, s)));
            assert!(
                t.lookup(&name1(
                    EntityKind::Edge,
                    split,
                    RoleSeg::SplitFragment {
                        side,
                        parent: Box::new(strut.clone())
                    }
                ))
                .is_some(),
                "missing strut fragment side={side:?} v={s}"
            );
            assert!(
                t.lookup(&name1(
                    EntityKind::Vertex,
                    split,
                    RoleSeg::CrossingVertex {
                        side,
                        edge: Box::new(strut)
                    }
                ))
                .is_some(),
                "missing crossing vertex side={side:?} v={s}"
            );
        }
    }
}

// ---- Transform pass-through + Pattern wrapping. ----

#[test]
fn transform_passes_names_through_and_pattern_wraps_instances() {
    let (doc, ext) = cube(ProfileDoc::empty_derived("m4_pr3_names"), 0.0, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: ext,
            translation: [len(3.0), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: tr,
            count: editor_core::Expr::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let ev = run(&doc);
    // Transform: the SAME table value (names and keys verbatim).
    assert_eq!(table(&ev, tr), table(&ev, ext));
    // Pattern: every master entry wrapped per instance, body = i.
    let tp = table(&ev, pat);
    assert_eq!(tp.len(), 27 * 3);
    let master_top = name1(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::Top));
    for i in 0..3u32 {
        let wrapped = name1(
            EntityKind::Face,
            pat,
            RoleSeg::Instance {
                i,
                of: Box::new(master_top.clone()),
            },
        );
        match tp.lookup(&wrapped) {
            Some(Entry::Unique(e)) => assert_eq!(e.body, i),
            other => panic!("instance {i} top cap: {other:?}"),
        }
    }
}

// ---- Declare pairs resolve against the real tables (D6). ----

#[test]
fn declare_pairs_resolve_in_the_named_nodes_tables() {
    let (doc, a) = cube(ProfileDoc::empty_derived("m4_pr3_names"), 0.0, 1.0);
    let (doc, b) = cube(doc, 0.5, 1.0);
    let pair = (
        name1(EntityKind::Face, a, RoleSeg::Cap(CapEnd::Top)),
        name1(EntityKind::Face, b, RoleSeg::Cap(CapEnd::Bottom)),
    );
    let (doc, _decl) = insert(
        doc,
        Node::Declare {
            pairs: vec![pair.clone()],
        },
    );
    let ev = run(&doc);
    for name in [&pair.0, &pair.1] {
        let t = table(&ev, name.node);
        assert!(
            matches!(t.lookup(name), Some(Entry::Unique(_))),
            "declared name does not resolve: {name:?}"
        );
    }
}

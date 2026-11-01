//! M4 PR 3 naming tests, part 1 (spec D6): per-op role coverage for
//! the sweep ops, split, transform pass-through, family wrapping,
//! and authored face names resolving against the real tables.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, EntityKind, Entry, EvalOptions, Evaluation, LoopProgram, MeridianEnd,
    Node, ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef, ProgramArcData,
    ProgramStep, ProgramTarget, Qualifier, RecipeNodeId, RoleSeg, Sense, SitedRef, SplitHalf, band,
    band_rim, band_rim_pi, evaluate, meridian_vertex,
};
use fixture::{ang, insert, len, len2, minted, on_frame, revolve_about, table};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn pe(doc: &editor_core::ProfileDoc, node: RecipeNodeId, l: u32, s: u32) -> ProfileEdgeRef {
    crate::fixture::piece(doc, node, l as usize, s as usize)
}

fn pv(doc: &editor_core::ProfileDoc, node: RecipeNodeId, l: u32, v: u32) -> ProfileVertexRef {
    crate::fixture::vpiece(doc, node, l as usize, v as usize)
}

/// A unit-square profile + extrude prelude (xy plane).
fn cube(doc: ProfileDoc, x0: f64, side: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (x0, 0.0),
            (x0 + side, 0.0),
            (x0 + side, side),
            (x0, side),
        ]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(side),
        },
    )
}

// ---- Extrude: the full role inventory of a cube, exact. ----

#[test]
fn extrude_names_every_boundary_entity_with_the_d2_roles() {
    let (doc, ext) = cube(
        ProfileDoc::empty_derived("m4_pr3_names", Tol::witness()),
        0.0,
        1.0,
    );
    let ev = run(&doc);
    let t = table(&ev, ext);
    // 1 body + 6 faces + 12 edges + 8 vertices.
    assert_eq!(t.len(), 27);
    // Body + caps.
    assert!(
        t.lookup(&minted(EntityKind::Body, ext, RoleSeg::OutputBody))
            .is_some()
    );
    for end in [CapEnd::End, CapEnd::Start] {
        assert!(
            t.lookup(&minted(EntityKind::Face, ext, RoleSeg::Cap(end)))
                .is_some()
        );
    }
    // Per canonical segment: lateral, both rims; per vertex: strut,
    // both cap vertices.
    for s in 0..4 {
        assert!(
            t.lookup(&minted(
                EntityKind::Face,
                ext,
                RoleSeg::Lateral(pe(&doc, ext, 0, s).into())
            ))
            .is_some()
        );
        for end in [CapEnd::End, CapEnd::Start] {
            assert!(
                t.lookup(&minted(
                    EntityKind::Edge,
                    ext,
                    RoleSeg::RimEdge(end, pe(&doc, ext, 0, s).into())
                ))
                .is_some()
            );
            assert!(
                t.lookup(&minted(
                    EntityKind::Vertex,
                    ext,
                    RoleSeg::CapVertex(end, pv(&doc, ext, 0, s))
                ))
                .is_some()
            );
        }
        assert!(
            t.lookup(&minted(
                EntityKind::Edge,
                ext,
                RoleSeg::LateralEdge(pv(&doc, ext, 0, s))
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

/// Profile on the xy plane (the y axis lies in it).
fn revolve_doc(pts: Vec<(f64, f64)>, angle: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m4_pr3_names", Tol::witness());
    let (doc, p) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts]);
    insert(
        doc,
        revolve_about(p, (0.0, 0.0), crate::fixture::along_y(), ang(angle)),
    )
}

/// The natural meridian, on the same plane and axis as [`revolve_doc`]:
/// a bulge-1 semicircle from (0, −1) to (0, 1) closed by its on-axis
/// diameter. EVERY vertex of this loop is on the axis — the shape the
/// pole export exists for.
fn ball_doc(angle: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m4_pr3_names", Tol::witness());
    let meridian = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, -1.0])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(len2([0.0, 1.0])),
            b: fixture::scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![meridian],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        revolve_about(p, (0.0, 0.0), crate::fixture::along_y(), ang(angle)),
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
            t.lookup(&minted(EntityKind::Face, rev, RoleSeg::RevolveCap(m)))
                .is_some()
        );
    }
    for s in 0..4 {
        assert!(t.lookup(&band(rev, pe(&doc, rev, 0, s))).is_some());
        assert!(t.lookup(&band_rim(rev, pv(&doc, rev, 0, s))).is_some());
        for m in [MeridianEnd::Start, MeridianEnd::End] {
            assert!(
                t.lookup(&minted(
                    EntityKind::Edge,
                    rev,
                    RoleSeg::Meridian(m, pe(&doc, rev, 0, s).into())
                ))
                .is_some()
            );
            assert!(
                t.lookup(&meridian_vertex(m, rev, pv(&doc, rev, 0, s)))
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
        t.lookup(&minted(
            EntityKind::Edge,
            rev,
            RoleSeg::AxisEdge(pe(&doc, rev, 0, 3).into())
        ))
        .is_some()
    );
    // The two on-axis profile vertices are poles.
    for v in [0, 3] {
        assert!(
            t.lookup(&minted(
                EntityKind::Vertex,
                rev,
                RoleSeg::Pole(pv(&doc, rev, 0, v))
            ))
            .is_some()
        );
    }
    // Off-axis vertices carry start+end copies.
    for v in [1, 2] {
        for m in [MeridianEnd::Start, MeridianEnd::End] {
            assert!(
                t.lookup(&meridian_vertex(m, rev, pv(&doc, rev, 0, v)))
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
    // Square torus: 1 body + 4 bands + (4 rims + 2 seam meridians, the
    // cylinders'; a plane annulus has none) + 4 meridian vertices.
    assert_eq!(t.len(), 15);
    for s in 0..4 {
        assert!(t.lookup(&band(rev, pe(&doc, rev, 0, s))).is_some());
        assert!(t.lookup(&band_rim(rev, pv(&doc, rev, 0, s))).is_some());
        // Sides 0 and 2 sweep the plane annuli, 1 and 3 the cylinders.
        assert_eq!(
            t.lookup(&minted(
                EntityKind::Edge,
                rev,
                RoleSeg::Meridian(MeridianEnd::Seam, pe(&doc, rev, 0, s).into())
            ))
            .is_some(),
            s % 2 == 1,
            "side {s}: a seam meridian on a cylinder only"
        );
        assert!(
            t.lookup(&meridian_vertex(
                MeridianEnd::Seam,
                rev,
                pv(&doc, rev, 0, s)
            ))
            .is_some()
        );
    }
}

/// The flange's three plane annuli — the bottom, the step and the top —
/// carry no `Meridian(Seam, ·)`; its three cylinders do, the step
/// annulus's ring (the hub rim) included. 1 body + 6 bands + 6 rims +
/// 3 seam meridians + 6 meridian vertices = 22.
#[test]
fn full_flange_revolve_names_a_seam_meridian_on_its_cylinders_only() {
    let (doc, rev) = revolve_doc(
        vec![
            (1.0, 0.0),
            (3.0, 0.0),
            (3.0, 0.5),
            (2.0, 0.5),
            (2.0, 2.0),
            (1.0, 2.0),
        ],
        std::f64::consts::TAU,
    );
    let ev = run(&doc);
    let t = table(&ev, rev);
    assert_eq!(t.len(), 22);
    for s in 0..6 {
        // Even sides sweep the plane annuli, odd ones the cylinders.
        assert_eq!(
            t.lookup(&minted(
                EntityKind::Edge,
                rev,
                RoleSeg::Meridian(MeridianEnd::Seam, pe(&doc, rev, 0, s).into())
            ))
            .is_some(),
            s % 2 == 1,
            "side {s}: a seam meridian on a cylinder only"
        );
    }
}

#[test]
fn full_holed_revolve_names_the_cavity_loop() {
    // VERBS-RING: a holed profile fully revolved — the hollow ring.
    // The hole's cavity shell names exactly like a second lamina
    // loop: bands, full rims, its cylinders' seam meridians, meridian
    // vertices, all under loop index 1.
    let doc = ProfileDoc::empty_derived("m4_pr3_names", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![
            vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
            vec![(1.25, 0.25), (1.75, 0.25), (1.75, 0.75), (1.25, 0.75)],
        ],
    );
    let (doc, rev) = insert(
        doc,
        revolve_about(
            p,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    );
    let ev = run(&doc);
    let t = table(&ev, rev);
    // Two square-torus shells: 1 body + 2·(4 bands + 4 rims + 2 seam
    // meridians + 4 meridian vertices).
    assert_eq!(t.len(), 29);
    for l in 0..2 {
        for s in 0..4 {
            assert!(t.lookup(&band(rev, pe(&doc, rev, l, s))).is_some());
            assert!(t.lookup(&band_rim(rev, pv(&doc, rev, l, s))).is_some());
            // A seam meridian on a cylinder only; the loop's sides
            // alternate between cylinders and plane annuli.
            let has = |s: u32| {
                t.lookup(&minted(
                    EntityKind::Edge,
                    rev,
                    RoleSeg::Meridian(MeridianEnd::Seam, pe(&doc, rev, l, s).into()),
                ))
                .is_some()
            };
            assert_ne!(has(s), has((s + 1) % 4), "loop {l} side {s}");
            assert!(
                t.lookup(&meridian_vertex(
                    MeridianEnd::Seam,
                    rev,
                    pv(&doc, rev, l, s)
                ))
                .is_some()
            );
        }
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
    // Each off-axis rim is two half-arcs between the seam vertices, and
    // the builder spells the second exactly as emission minted it.
    let mut rims_pi: Vec<_> = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::BandRimPi(_))))
        .map(|(n, _)| n.clone())
        .collect();
    rims_pi.sort();
    let mut built = vec![
        band_rim_pi(rev, pv(&doc, rev, 0, 1)),
        band_rim_pi(rev, pv(&doc, rev, 0, 2)),
    ];
    built.sort();
    assert_eq!(
        rims_pi, built,
        "the BandRimPi arcs are the two rims' builders"
    );
    for v in [1, 2] {
        let whole = [
            band_rim(rev, pv(&doc, rev, 0, v)),
            band_rim_pi(rev, pv(&doc, rev, 0, v)),
        ];
        assert!(
            whole.iter().all(|n| t.lookup(n).is_some()),
            "rim {v} resolves at both halves"
        );
        assert_ne!(
            t.lookup(&whole[0]),
            t.lookup(&whole[1]),
            "rim {v}'s halves are two edges"
        );
    }
    // No poles: both on-axis profile vertices are disc centres, and a
    // plane wall is built whole, so neither is a vertex.
    let poles = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Pole(_))))
        .count();
    assert_eq!(poles, 0);
    // The discs are plain `Band` faces: one π twin, the cylinder's.
    let pi_bands = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::BandPi(_))))
        .count();
    assert_eq!(pi_bands, 1, "only the curved wall has a π twin");
}

/// M9-D1: the natural ball. Every vertex of the meridian is on-axis,
/// so the table stands entirely on the sweep's pole export — this row
/// was RED (the "all-on-axis loop" refusal) before it.
#[test]
fn full_revolve_of_an_all_on_axis_loop_names_both_poles() {
    let (doc, rev) = ball_doc(std::f64::consts::TAU);
    let ev = run(&doc);
    let t = table(&ev, rev);
    // The two-band sphere patch (V2 E2 F2): 1 body + 2 bands +
    // 2 meridians + 2 poles.
    assert_eq!(t.len(), 7);
    // Canonical vertex 0 is the authored start — (0, −1), the south
    // pole; 1 is (0, 1). Canonical segment 0 is the arc.
    for v in 0..2 {
        assert!(
            t.lookup(&minted(
                EntityKind::Vertex,
                rev,
                RoleSeg::Pole(pv(&doc, rev, 0, v))
            ))
            .is_some(),
            "pole {v} unnamed"
        );
    }
    for seg in [
        RoleSeg::Band(pe(&doc, rev, 0, 0).into()),
        RoleSeg::BandPi(pe(&doc, rev, 0, 0).into()),
        RoleSeg::Meridian(MeridianEnd::Seam, pe(&doc, rev, 0, 0).into()),
        RoleSeg::Meridian(MeridianEnd::Pi, pe(&doc, rev, 0, 0).into()),
    ] {
        let kind = match seg {
            RoleSeg::Band(_) | RoleSeg::BandPi(_) => EntityKind::Face,
            _ => EntityKind::Edge,
        };
        assert!(
            t.lookup(&minted(kind, rev, seg.clone())).is_some(),
            "{seg:?} unnamed — canonical segment 0 is not the arc"
        );
    }
    // The on-axis diameter sweeps to nothing: no segment-1 roles.
    assert!(t.lookup(&band(rev, pe(&doc, rev, 0, 1))).is_none());
}

/// M9-D1: the same all-on-axis meridian, partially revolved — the
/// wedge. Also RED before the fix (the partial arm refused
/// identically).
#[test]
fn partial_revolve_of_an_all_on_axis_loop_names_both_poles() {
    let (doc, rev) = ball_doc(std::f64::consts::FRAC_PI_2);
    let ev = run(&doc);
    let t = table(&ev, rev);
    // Ball wedge: 1 body + (1 band + 2 caps) + (2 meridians + 1 axis
    // edge) + 2 poles.
    assert_eq!(t.len(), 9);
    for v in 0..2 {
        assert!(
            t.lookup(&minted(
                EntityKind::Vertex,
                rev,
                RoleSeg::Pole(pv(&doc, rev, 0, v))
            ))
            .is_some(),
            "pole {v} unnamed"
        );
    }
    assert!(
        t.lookup(&minted(
            EntityKind::Edge,
            rev,
            RoleSeg::AxisEdge(pe(&doc, rev, 0, 1).into())
        ))
        .is_some(),
        "the on-axis diameter is the caps' shared axis edge"
    );
    assert!(t.lookup(&band(rev, pe(&doc, rev, 0, 0))).is_some());
}

// ---- Split: sections, fragments, crossings, pass-through. ----

#[test]
fn split_names_sections_fragments_and_crossings() {
    let (doc, ext) = cube(
        ProfileDoc::empty_derived("m4_pr3_names", Tol::witness()),
        0.0,
        2.0,
    );
    let tool = fixture::plane_on(&doc, ext, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: ext.into(),
            tool,
        },
    );
    let ev = run(&doc);
    // Each half its own table (NAMES N1), each total: 1 body + 6 faces
    // + 12 edges + 8 vertices, its body that table's body 0.
    let half = |side| fixture::half(&ev, split, side);
    for side in [SplitHalf::Above, SplitHalf::Below] {
        let t = half(side);
        assert_eq!(t.len(), 27, "{side:?}");
        assert!(
            t.lookup(&minted(EntityKind::Body, split, RoleSeg::SplitBody))
                .is_some()
        );
        assert!(
            t.lookup(&minted(EntityKind::Face, split, RoleSeg::SectionFace))
                .is_some()
        );
    }
    // Caps pass through UNCHANGED (names still minted at the extrude;
    // the split contributes no segment): the end cap lives in the
    // above half, the start cap below, each absent from the other.
    let cap = |side, end| half(side).lookup(&minted(EntityKind::Face, ext, RoleSeg::Cap(end)));
    match (
        cap(SplitHalf::Above, CapEnd::End),
        cap(SplitHalf::Below, CapEnd::Start),
    ) {
        (Some(Entry::Unique(a)), Some(Entry::Unique(b))) => {
            assert_eq!(a.body, 0);
            assert_eq!(b.body, 0);
        }
        other => panic!("caps not unique: {other:?}"),
    }
    assert!(cap(SplitHalf::Below, CapEnd::End).is_none());
    assert!(cap(SplitHalf::Above, CapEnd::Start).is_none());
    // Each side wall is a side-tagged fragment of its lateral; each
    // strut edge likewise; each strut crossing is a CrossingVertex;
    // each cut wall contributes a SectionEdge.
    for side in [SplitHalf::Above, SplitHalf::Below] {
        let t = half(side);
        for s in 0..4 {
            let lateral = minted(
                EntityKind::Face,
                ext,
                RoleSeg::Lateral(pe(&doc, ext, 0, s).into()),
            );
            assert!(
                t.lookup(&minted(
                    EntityKind::Face,
                    split,
                    RoleSeg::SplitFragment {
                        parent: lateral.clone().into()
                    }
                ))
                .is_some(),
                "missing wall fragment side={side:?} seg={s}"
            );
            assert!(
                t.lookup(&minted(
                    EntityKind::Edge,
                    split,
                    RoleSeg::SectionEdge {
                        face: lateral.into()
                    }
                ))
                .is_some(),
                "missing section edge side={side:?} seg={s}"
            );
            let strut = minted(
                EntityKind::Edge,
                ext,
                RoleSeg::LateralEdge(pv(&doc, ext, 0, s)),
            );
            // The strut's one piece on each side, named by its ends.
            let fragment = RoleSeg::SplitFragment {
                parent: strut.clone().into(),
            };
            assert_eq!(
                t.iter()
                    .filter(|(n, _)| {
                        n.node == split
                            && n.path.first() == Some(&fragment)
                            && matches!(
                                n.path.as_slice(),
                                [_, RoleSeg::Fragment(Qualifier::Ends(_))]
                            )
                    })
                    .count(),
                1,
                "missing strut fragment side={side:?} v={s}"
            );
            // The strut runs up from the start cap, so it enters the
            // half above the plane and leaves the half below it.
            let sense = match side {
                SplitHalf::Above => Sense::Enters,
                SplitHalf::Below => Sense::Leaves,
            };
            assert!(
                t.lookup(&minted(
                    EntityKind::Vertex,
                    split,
                    RoleSeg::CrossingVertex {
                        edge: strut.into(),
                        sense,
                    }
                ))
                .is_some(),
                "missing crossing vertex side={side:?} v={s}"
            );
        }
    }
}

// ---- Placement wrapping + family wrapping. ----

#[test]
fn a_placement_wraps_names_once_and_a_family_wraps_members() {
    let (doc, ext) = cube(
        ProfileDoc::empty_derived("m4_pr3_names", Tol::witness()),
        0.0,
        1.0,
    );
    let (doc, tr) = fixture::slid(doc, ext, ext, [3.0, 0.0, 0.0]);
    let (doc, k, pat) = crate::fixture::linear(doc, tr, [1.0, 0.0, 0.0], 2.0, 3);
    let ev = run(&doc);
    // Placement: every name of the body carried as it is, at the same
    // entity; the copy's body row is its own.
    let (moved, master) = (table(&ev, tr), table(&ev, ext));
    assert_eq!(moved.len(), master.len());
    for (name, entry) in master.iter() {
        let worn = if name.kind == EntityKind::Body {
            minted(EntityKind::Body, tr, RoleSeg::OutputBody)
        } else {
            name.clone()
        };
        assert_eq!(
            moved.lookup(&worn),
            Some(entry),
            "{name:?} worn by the copy"
        );
    }
    // Family: every member's entry wrapped by its index value, body = i.
    let tp = table(&ev, pat);
    assert_eq!(tp.len(), 27 * 3);
    let master_end = minted(EntityKind::Face, ext, RoleSeg::Cap(CapEnd::End));
    for i in 0..3u32 {
        let wrapped = minted(
            EntityKind::Face,
            pat,
            RoleSeg::Member {
                at: vec![(k, i)],
                of: master_end.clone().into(),
            },
        );
        match tp.lookup(&wrapped) {
            Some(Entry::Unique(e)) => assert_eq!(e.body, i),
            other => panic!("member {i} end cap: {other:?}"),
        }
    }
}

// ---- Authored names resolve against the real tables (D6). ----

#[test]
fn authored_face_names_resolve_in_the_named_nodes_tables() {
    let (doc, a) = cube(
        ProfileDoc::empty_derived("m4_pr3_names", Tol::witness()),
        0.0,
        1.0,
    );
    let (doc, b) = cube(doc, 0.5, 1.0);
    let pair = (
        SitedRef::new(a, minted(EntityKind::Face, a, RoleSeg::Cap(CapEnd::End))),
        SitedRef::new(b, minted(EntityKind::Face, b, RoleSeg::Cap(CapEnd::Start))),
    );
    let ev = run(&doc);
    for r in [&pair.0, &pair.1] {
        let name = &r.name;
        let t = table(&ev, r.at);
        assert!(
            matches!(t.lookup(name), Some(Entry::Unique(_))),
            "authored name does not resolve: {name:?}"
        );
    }
}

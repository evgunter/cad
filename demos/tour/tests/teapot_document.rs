//! **The teapot's document, tabulated** — the one-request roll the
//! scene ships, held to the class and to the kernel door.
//!
//! 1. **Every pair of the lid's rims composes in ONE request**, and so
//!    do the scene's three. A band slits — and its trimline crosses —
//!    ONE support's seam meridian, so two rims at the two ends of one
//!    meridian segment put two slits and two crossings on one source
//!    meridian. On this lid those pairs are `{1, 2}` (the flange cone's
//!    seam) and `{5, 0}` (the vent's). Their names tell them apart by
//!    the BAND that made each: those pairs build, and the two slits
//!    and two crossings on the flange seam carry that seam and one
//!    band each.
//! 2. **The one request builds the kernel's one-request body**: same
//!    census, the three bands' stored `(station, major, minor)` bit for
//!    bit, the mass to a relative 1e-14 — and the same face ORDER,
//!    which is what the tess-budget rows and the uv sheet's cells key
//!    on.
//! 3. **The names are a function of the recipe's names, not of its
//!    numbers**: the rolled lid's name set at two radii is one set.
//!
//! The meridian and the constants are re-spelled here because a demo
//! binary's module cannot be imported by an integration test — the
//! same reason `verbs_teapot.rs` carries its own `genus`. They are
//! copied from `src/teapot.rs` and nothing here may be edited without
//! editing it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use pncad::document::{
    CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Evaluation, Expr, LoopProgram, Node,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, ValuePayload, apply,
    evaluate,
};
use pncad::geom::Surface;
use pncad::geom_core::Tol;
use pncad::prelude::{
    EntityKind, MeridianEnd, ProfileEdgeRef, RoleSeg, StableName, fillet_edges, query,
};
use pncad::profile::ArcSweep;
use pncad::select::{band_rim, edge_name};
use pncad::topo::{Body, EdgeKey};

// ---- the lid's stations, from `src/teapot.rs` ----
const R_NECK: f64 = 3.0 / 64.0;
const Y_MOUTH: f64 = 8.0 / 64.0;
const LIFT: f64 = 1.0 / 32.0;
const LID_BASE: f64 = Y_MOUTH + LIFT;
const R_FLANGE: f64 = 14.0 / 256.0;
const Y_FLANGE: f64 = LID_BASE + 6.0 / 256.0;
const DOME_C: f64 = LID_BASE + 1.0 / 256.0;
const R_KNOB: f64 = 5.0 / 256.0;
const Y_KNOB: f64 = LID_BASE + 13.0 / 256.0;
const Y_TOP: f64 = LID_BASE + 18.0 / 256.0;
const R_VENT: f64 = 1.0 / 256.0;
const ROLL: f64 = 2.0 / 256.0;

/// The lid's three rolled rims, as the meridian vertex each stands at.
const ROLLED: [u32; 3] = [1, 2, 4];

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a finite length")
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a finite scalar")
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("a finite angle")
}
fn lpt(x: f64, y: f64) -> [Expr; 2] {
    [len(x), len(y)]
}
fn line_to(x: f64, y: f64) -> ProgramStep {
    ProgramStep::LineTo(ProgramTarget::Point(lpt(x, y)))
}

fn lid_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(R_VENT, LID_BASE)),
        line_to(R_FLANGE, LID_BASE),
        line_to(R_NECK, Y_FLANGE),
        ProgramStep::ArcTo(ProgramArcData::Center {
            c: lpt(0.0, DOME_C),
            winding: ArcSweep::Ccw,
            target: ProgramTarget::Point(lpt(R_KNOB, Y_KNOB)),
        }),
        line_to(R_KNOB, Y_TOP),
        line_to(R_VENT, Y_TOP),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

fn insert(doc: &mut Doc<ProfileProgram>, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode { node },
        tol,
        &pncad::document::RefusingReach,
    )
    .expect("the edit applies");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}

/// A fresh document carrying the sharp lid, and that node's id.
fn sharp_lid(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("teapot-lid", tol);
    let plane = insert(
        &mut doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    let axis = insert(
        &mut doc,
        Node::Datum(Datum::AxisInPlane {
            plane,
            origin: [len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0)],
        }),
        tol,
    );
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![lid_meridian()],
        }),
        tol,
    );
    let lid = insert(
        &mut doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(TAU),
        },
        tol,
    );
    (doc, lid)
}

fn eval(doc: &Doc<ProfileProgram>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
}

fn body_at(ev: &Evaluation<f64>, id: RecipeNodeId) -> Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}

/// Every torus band's stored `(station, major, minor)`, as BITS —
/// which is what "the same body" has to mean for a stored radius.
fn bands(b: &Body<f64>) -> Vec<(u64, u64, u64)> {
    let mut out: Vec<(u64, u64, u64)> = b
        .faces()
        .filter_map(|(_, f)| match b.get_surface(f.surface) {
            Some(Surface::Torus {
                major_radius,
                minor_radius,
                center,
                ..
            }) => Some((
                center.y.to_bits(),
                major_radius.to_bits(),
                minor_radius.to_bits(),
            )),
            _ => None,
        })
        .collect();
    out.sort_unstable();
    out
}

/// The lid rolled at `roll` over the rims at `vs` in ONE `Node::Fillet`
/// request: the document, the sharp lid's id and the rolled node's.
fn rolled_lid(
    vs: &[u32],
    roll: f64,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let (mut doc, lid) = sharp_lid(tol);
    let sel: Vec<StableName> = vs.iter().map(|&v| band_rim(lid, 0, v)).collect();
    let rolled = insert(&mut doc, Node::fillet(lid, len(roll), sel), tol);
    (doc, lid, rolled)
}

/// One `Node::Fillet` request over the rims at `vs`: the census it
/// built, or the refusal it answered.
fn roll_once(vs: &[u32], roll: f64, tol: Tol) -> Result<(usize, usize, usize), String> {
    let (doc, _, rolled) = rolled_lid(vs, roll, tol);
    let ev = eval(&doc, tol);
    match ev.node_error(rolled) {
        Some(e) => Err(format!("{:?}", e.kind)),
        None => Ok(census(&body_at(&ev, rolled))),
    }
}

/// **Every rim pair one request can roll, it can name.**
///
/// `{1, 2}` is the pair whose bands both slit and both cross the
/// flange cone's seam; `{2, 3}` and `{3, 4}` are adjacent and slit two
/// different seams. All of them build, and so does the scene's triple.
/// `{5, 0}` shares the vent's seam the same way; it is rolled at a
/// quarter of the scene's radius, where the concave vent rim has the
/// headroom the scene's radius does not give it.
#[test]
fn every_rim_pair_composes_in_one_request() {
    let tol = Tol::witness();
    assert_eq!(
        roll_once(&ROLLED, ROLL, tol),
        Ok((9, 18, 9)),
        "the scene's three rims in ONE request: three annulus bands over the sharp \
         lid's 6/12/6"
    );
    for pair in [[1u32, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]] {
        assert_eq!(
            roll_once(&pair, ROLL, tol),
            Ok((8, 16, 8)),
            "rims {pair:?} in ONE request: two annulus bands over the sharp lid's 6/12/6"
        );
    }
    assert_eq!(
        roll_once(&[5, 0], ROLL / 4.0, tol),
        Ok((8, 16, 8)),
        "the vent's two rims share its seam meridian as the flange's rim and the \
         dome's foot share the flange cone's"
    );
}

/// **Two slits on one source meridian carry two bands.**
///
/// The flange's rim (vertex 1) and the dome's foot (vertex 2) are the
/// two ends of meridian segment 1, and both bands slit its seam. So the
/// rolled table holds exactly two `BandSlit`s whose source edge is that
/// seam, and what tells them apart is the band: each carries its own
/// rim, and no other. The same holds of the `BandCross` each band's
/// trimline leaves on that seam.
#[test]
fn two_slits_on_one_meridian_carry_the_band_that_made_each() {
    let tol = Tol::witness();
    let (doc, lid, rolled) = rolled_lid(&ROLLED, ROLL, tol);
    let ev = eval(&doc, tol);
    assert!(
        ev.node_error(rolled).is_none(),
        "{:?}",
        ev.node_error(rolled)
    );
    let table = &ev.value(rolled).expect("the roll evaluated").name_table;
    // The flange cone's seam meridian: the sharp lid's meridian
    // segment 1, on the revolve's seam.
    let seam = StableName {
        kind: EntityKind::Edge,
        node: lid,
        path: vec![RoleSeg::Meridian(
            MeridianEnd::Seam,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 1,
            },
        )],
    };
    let want = |v: u32| vec![band_rim(lid, 0, v)];
    for role in ["slit", "cross"] {
        let mut bands: Vec<Vec<StableName>> = table
            .iter()
            .filter_map(|(n, _)| match (role, &n.path[..]) {
                ("slit", [RoleSeg::BandSlit { edge, band }])
                | ("cross", [RoleSeg::BandCross { edge, band }])
                    if **edge == seam =>
                {
                    Some(band.clone())
                }
                _ => None,
            })
            .collect();
        bands.sort();
        assert_eq!(
            bands,
            vec![want(1), want(2)],
            "the flange seam's {role}s: one per band that ends on it, each carrying its \
             own rim"
        );
    }
}

/// **The one request builds the kernel's one-request body.**
///
/// Same census, the same three bands bit for bit, the same mass, the
/// same face order: the document door adds names and nothing else.
#[test]
fn one_request_builds_the_kernels_body() {
    let tol = Tol::witness();
    let (doc, lid, rolled) = rolled_lid(&ROLLED, ROLL, tol);
    let ev = eval(&doc, tol);
    assert!(
        ev.node_error(rolled).is_none(),
        "the one-request roll builds: {:?}",
        ev.node_error(rolled)
    );
    let sharp = body_at(&ev, lid);
    let doc_body = body_at(&ev, rolled);

    // The kernel's request over the same three rims — their keys found
    // through the NAMES, so the two doors are asked for the same edges
    // and not merely for three edges each.
    let keys: Vec<EdgeKey> = ROLLED
        .iter()
        .map(|&v| {
            query::all_edges(&sharp)
                .into_iter()
                .find(|&k| edge_name(&ev, lid, 0, k).ok() == Some(&band_rim(lid, 0, v)))
                .expect("each rolled rim's key, by its name")
        })
        .collect();
    let kernel = fillet_edges(&sharp, &keys, ROLL, tol)
        .expect("the kernel door rolls all three in one request")
        .body;

    assert_eq!(census(&kernel), (9, 18, 9));
    assert_eq!(census(&doc_body), (9, 18, 9));
    assert_eq!(
        bands(&kernel),
        bands(&doc_body),
        "the three bands' STORED (station, major, minor) must be identical bits"
    );
    let p1 = pncad::topo::mass_properties(&kernel, tol).expect("kernel props");
    let p2 = pncad::topo::mass_properties(&doc_body, tol).expect("document props");
    assert!(
        ((p1.volume - p2.volume) / p1.volume).abs() < 1e-14
            && ((p1.surface_area - p2.surface_area) / p1.surface_area).abs() < 1e-14,
        "kernel V={} A={}; document V={} A={}",
        p1.volume,
        p1.surface_area,
        p2.volume,
        p2.surface_area
    );
    let order = |b: &Body<f64>| -> Vec<String> {
        b.faces()
            .map(|(_, f)| match b.get_surface(f.surface) {
                Some(Surface::Torus { center, .. }) => format!("torus@{}", center.y),
                Some(Surface::Plane { .. }) => "plane".to_owned(),
                Some(Surface::Cylinder { .. }) => "cylinder".to_owned(),
                Some(Surface::Cone { .. }) => "cone".to_owned(),
                Some(Surface::Sphere { .. }) => "sphere".to_owned(),
                _ => "?".to_owned(),
            })
            .collect()
    };
    assert_eq!(
        order(&kernel),
        order(&doc_body),
        "one request through either door lays the faces out in one order"
    );
}

/// **The rolled lid's names do not depend on the radius.**
///
/// Every role argument is a source NAME, so rolling the same rims at a
/// different radius re-mints the same name set: a name stays put when
/// a number moves.
#[test]
fn the_rolled_names_are_one_set_at_two_radii() {
    let tol = Tol::witness();
    let names = |roll: f64| -> Vec<StableName> {
        let (doc, _, rolled) = rolled_lid(&ROLLED, roll, tol);
        let ev = eval(&doc, tol);
        assert!(
            ev.node_error(rolled).is_none(),
            "{:?}",
            ev.node_error(rolled)
        );
        let mut out: Vec<StableName> = ev
            .value(rolled)
            .expect("the roll evaluated")
            .name_table
            .iter()
            .map(|(n, _)| n.clone())
            .collect();
        out.sort();
        out
    };
    let (a, b) = (names(ROLL), names(ROLL * 0.75));
    assert_eq!(
        a.len(),
        9 + 18 + 9 + 1,
        "one name per entity and the body's"
    );
    assert_eq!(a, b);
}

/// Every entity of the rolled body, keyed by its NAME, with a
/// geometric witness: a vertex's point, an edge's two end points
/// (sorted), a face's vertex count. Bits, so "same" means same.
fn named_geometry(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
) -> std::collections::BTreeMap<StableName, Vec<u64>> {
    let b = body_at(ev, node);
    let pt = |v: pncad::topo::VertexKey| -> [u64; 3] {
        let p = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
        [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]
    };
    let mut out = std::collections::BTreeMap::new();
    for (k, e) in b.edges() {
        let n = edge_name(ev, node, 0, k).expect("every edge named").clone();
        let a = pt(b.get_half_edge(e.he_plus).unwrap().start);
        let z = pt(b.get_half_edge(e.he_minus).unwrap().start);
        let mut ends = [a, z];
        ends.sort_unstable();
        out.insert(n, ends.concat());
    }
    for (n, _) in ev.value(node).unwrap().name_table.iter() {
        if n.kind == EntityKind::Vertex {
            let p = pncad::select::vertex_position(ev, node, n).expect("vertex reads");
            out.insert(n.clone(), vec![p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]);
        }
    }
    out
}

fn roll_named(
    vs: &[u32],
    roll: f64,
    tol: Tol,
) -> Result<std::collections::BTreeMap<StableName, Vec<u64>>, String> {
    let (doc, _, rolled) = rolled_lid(vs, roll, tol);
    let ev = eval(&doc, tol);
    match ev.node_error(rolled) {
        Some(e) => Err(format!("{:?}", e.kind)),
        None => Ok(named_geometry(&ev, rolled)),
    }
}

/// **No set of the lid's rims refuses at a NAME**: every pair of the
/// six, all six, and each half, at a quarter of the roll. A request
/// may still refuse on geometry; it may not refuse because two of its
/// outputs share a name.
#[test]
fn every_rim_set_at_a_quarter_roll_is_nameable() {
    let tol = Tol::witness();
    let mut sets: Vec<Vec<u32>> = Vec::new();
    for a in 0..6u32 {
        for b in (a + 1)..6 {
            sets.push(vec![a, b]);
        }
    }
    sets.push((0..6).collect());
    sets.push(vec![0, 1, 2]);
    sets.push(vec![3, 4, 5]);
    let mut bad = Vec::new();
    for s in &sets {
        if let Err(e) = &roll_once(s, ROLL / 4.0, tol)
            && (e.contains("Naming") || e.contains("Duplicate"))
        {
            bad.push((s.clone(), e.clone()));
        }
    }
    assert!(bad.is_empty(), "naming refusals: {bad:#?}");
}

/// **Two annulus bands on one PLANE cap compose and name their
/// output**: the underside (segment 0) carries rims 0 and 1, the top
/// (segment 4) rims 4 and 5, so each pair's bands both carve the
/// cap's radial seam. At a quarter of the roll, where the vent's
/// concave rims have headroom.
#[test]
fn two_bands_on_one_plane_cap_compose() {
    let tol = Tol::witness();
    for pair in [[0u32, 1], [4, 5]] {
        assert_eq!(
            roll_once(&pair, ROLL / 4.0, tol),
            Ok((8, 16, 8)),
            "rims {pair:?} share a plane cap"
        );
    }
}

/// **The selection ORDER moves no name** (N4): the kernel carves the
/// bands one after another, and which one goes first decides which
/// band records the remnant between two of them — so the name →
/// geometry map is compared across orders, `BandCut` included.
#[test]
fn the_selection_order_moves_no_name() {
    let tol = Tol::witness();
    for (a, b) in [
        (vec![1u32, 2, 4], vec![4u32, 2, 1]),
        (vec![1, 2], vec![2, 1]),
        (vec![5, 0], vec![0, 5]),
    ] {
        let roll = if a.contains(&5) { ROLL / 4.0 } else { ROLL };
        let ga = roll_named(&a, roll, tol).expect("a builds");
        let gb = roll_named(&b, roll, tol).expect("b builds");
        assert_eq!(
            ga.keys().collect::<Vec<_>>(),
            gb.keys().collect::<Vec<_>>(),
            "{a:?} vs {b:?}: name sets"
        );
        for (n, g) in &ga {
            assert_eq!(Some(g), gb.get(n), "{a:?} vs {b:?}: {n:?} moved");
        }
    }
}

/// **One `BandCut` survives between two bands on one seam**: on the
/// flange seam exactly one remnant is named, and it runs from one
/// band's crossing to the other's. Two would share the source
/// meridian's name — the uniqueness rests on each band retiring the
/// remnant row an earlier band recorded before recording its own.
#[test]
fn one_band_cut_survives_between_two_bands() {
    let tol = Tol::witness();
    let (doc, lid, rolled) = rolled_lid(&[1, 2], ROLL, tol);
    let ev = eval(&doc, tol);
    assert!(ev.node_error(rolled).is_none());
    let seam = StableName {
        kind: EntityKind::Edge,
        node: lid,
        path: vec![RoleSeg::Meridian(
            MeridianEnd::Seam,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 1,
            },
        )],
    };
    let g = named_geometry(&ev, rolled);
    let cuts: Vec<_> = g
        .iter()
        .filter(|(n, _)| matches!(&n.path[..], [RoleSeg::BandCut(e)] if **e == seam))
        .collect();
    let crosses: Vec<_> = g
        .iter()
        .filter(|(n, _)| matches!(&n.path[..], [RoleSeg::BandCross { edge, .. }] if **edge == seam))
        .map(|(_, p)| p.clone())
        .collect();
    assert_eq!(cuts.len(), 1, "{cuts:#?}");
    assert_eq!(crosses.len(), 2);
    let mut want = [crosses[0].clone(), crosses[1].clone()];
    want.sort();
    assert_eq!(
        cuts[0].1,
        &want.concat(),
        "the cut runs crossing to crossing"
    );
}

/// **The interval lane mints the f64 lane's names** (N4: a name is
/// float-free, so the scalar type cannot move it).
#[test]
fn the_interval_lane_mints_the_f64_names() {
    use pncad::geom_core::Interval;
    let tol = Tol::witness();
    let (doc, _, rolled) = rolled_lid(&ROLLED, ROLL, tol);
    let ef = eval(&doc, tol);
    let ei: Evaluation<Interval> = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let nf: Vec<StableName> = ef
        .value(rolled)
        .unwrap()
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .collect();
    let ni: Vec<StableName> = ei
        .value(rolled)
        .unwrap_or_else(|| panic!("the interval lane rolls: {:?}", ei.node_error(rolled)))
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .collect();
    assert_eq!(nf, ni);
}

/// **An upstream RENAME carries into a held slit's band.** The
/// program is ROTATED (same solid, starting at the flange), so every
/// vertex and segment index moves; a downstream node holds the flange
/// band's slit by name. After the edit the held name must still denote
/// the SAME slit. Rewriting the slit's `edge` but not its `band` would
/// leave it naming the OTHER band's slit on the same seam — a silent
/// retarget that no refusal catches, which is what this row is for.
#[test]
fn a_program_rename_carries_a_held_slits_band() {
    use pncad::document::LoopProvenance;
    let tol = Tol::witness();
    let (mut doc, lid, rolled) = rolled_lid(&[1, 2], ROLL, tol);
    let profile = match doc.node(lid) {
        Some(Node::Revolve { profile, .. }) => *profile,
        other => panic!("{other:?}"),
    };
    let ev = eval(&doc, tol);
    assert!(ev.node_error(rolled).is_none());
    let before = named_geometry(&ev, rolled);
    let slit = before
        .keys()
        .find(|n| {
            matches!(&n.path[..], [RoleSeg::BandSlit { band, .. }] if *band == vec![band_rim(lid, 0, 1)])
        })
        .expect("the flange band's slit")
        .clone();
    let holder = insert(
        &mut doc,
        Node::fillet(rolled, len(ROLL / 8.0), vec![slit.clone()]),
        tol,
    );

    let rotated = LoopProgram::Chain(vec![
        ProgramStep::At(lpt(R_FLANGE, LID_BASE)),
        line_to(R_NECK, Y_FLANGE),
        ProgramStep::ArcTo(ProgramArcData::Center {
            c: lpt(0.0, DOME_C),
            winding: ArcSweep::Ccw,
            target: ProgramTarget::Point(lpt(R_KNOB, Y_KNOB)),
        }),
        line_to(R_KNOB, Y_TOP),
        line_to(R_VENT, Y_TOP),
        line_to(R_VENT, LID_BASE),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let applied = apply(
        &doc,
        &DocEdit::SetProgram {
            node: profile,
            loops: vec![rotated],
            provenance: vec![LoopProvenance {
                from: Some(0),
                steps: vec![
                    Some(0),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(1),
                ],
            }],
        },
        tol,
        &pncad::document::RefusingReach,
    )
    .expect("the rotation applies");
    let doc = applied.doc;
    let held = match doc.node(holder) {
        Some(Node::Fillet { selection, .. }) => selection[0].clone(),
        other => panic!("{other:?}"),
    };
    assert_ne!(
        held, slit,
        "the rotation moved the indices (else the probe is vacuous)"
    );
    let ev = eval(&doc, tol);
    assert!(
        ev.node_error(rolled).is_none(),
        "{:?}",
        ev.node_error(rolled)
    );
    let after = named_geometry(&ev, rolled);
    let g = after.get(&held).expect("the held slit still resolves");
    let close = |a: &[u64], b: &[u64]| {
        a.iter()
            .zip(b)
            .all(|(x, y)| (f64::from_bits(*x) - f64::from_bits(*y)).abs() < 1e-9)
    };
    // The endpoint order is by bits; compare as sets of two points.
    let (b0, b1) = before[&slit].split_at(3);
    let (a0, a1) = g.split_at(3);
    assert!(
        (close(a0, b0) && close(a1, b1)) || (close(a0, b1) && close(a1, b0)),
        "the held name retargeted: before {:?}, after {:?}",
        before[&slit]
            .iter()
            .map(|x| f64::from_bits(*x))
            .collect::<Vec<_>>(),
        g.iter().map(|x| f64::from_bits(*x)).collect::<Vec<_>>()
    );
}

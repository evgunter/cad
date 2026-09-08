//! LIB-TEAPOT reviewer lane R2 probes (PR 2206, head 21e6d1e28).
//!
//! Executed falsifications of the PR's claims about the teapot's
//! document: the `BandSlit` collision and its adjacency shape, the
//! two-request roll against the kernel's one-request roll, the spout's
//! placement measured OFF THE PLACED BODY, the mouth's two names, and
//! the canonical order `select` answers in.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use crate::corpus::{body_of, eval};
use crate::fixture::{Recorder, ang, axis_in_plane, frame, len, scl};
use editor_core::{
    EntityKind, Expr, LoopProgram, MeridianEnd, NamePat, Node, NodeErrorKind, ProfileEdgeRef,
    ProfileProgram, ProfileVertexRef, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId,
    RoleSeg, SegPat, SegTag, Selector, StableName, edge_frame, edge_name, face_carrier_kind,
    face_frame, face_name, select, vertex_position,
};
use geom_core::{Mat3, Point3, Tol, Vec3};
use topo::ShellError;

// ---- the tour's constants, verbatim ----
const R_FOOT: f64 = 4.0 / 64.0;
const R_NECK: f64 = 3.0 / 64.0;
const Y_FOOT: f64 = 1.0 / 64.0;
const Y_BELLY_C: f64 = 4.0 / 64.0;
const Y_MOUTH: f64 = 8.0 / 64.0;
const WALL: f64 = 1.0 / 128.0;
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
const SPOUT_LEN: f64 = 8.0 / 64.0;
const SPOUT_R0: f64 = 6.0 / 256.0;
const SPOUT_R1: f64 = 3.0 / 256.0;
const SPOUT_WALL: f64 = 1.0 / 256.0;
const SPOUT_ROOT: [f64; 3] = [-1.0 / 32.0, 3.0 / 64.0, 0.0];
const SPOUT_DIR: [f64; 3] = [-0.8, 0.6, 0.0];

fn lpt(x: f64, y: f64) -> [Expr; 2] {
    [len(x), len(y)]
}
fn line_to(x: f64, y: f64) -> ProgramStep {
    ProgramStep::LineTo(ProgramTarget::Point(lpt(x, y)))
}
fn arc_to(cx: f64, cy: f64, winding: profile::ArcSweep, x: f64, y: f64) -> ProgramStep {
    ProgramStep::ArcTo(ProgramArcData::Center {
        c: lpt(cx, cy),
        winding,
        target: ProgramTarget::Point(lpt(x, y)),
    })
}
fn band(node: RecipeNodeId, seg: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Band(ProfileEdgeRef {
            loop_index: 0,
            segment: seg,
        })],
    }
}
fn band_pi(node: RecipeNodeId, seg: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::BandPi(ProfileEdgeRef {
            loop_index: 0,
            segment: seg,
        })],
    }
}
fn band_rim(node: RecipeNodeId, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![RoleSeg::BandRim(ProfileVertexRef {
            loop_index: 0,
            vertex,
        })],
    }
}
fn meridian_vertex(node: RecipeNodeId, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![RoleSeg::MeridianVertex(
            MeridianEnd::Seam,
            ProfileVertexRef {
                loop_index: 0,
                vertex,
            },
        )],
    }
}
fn carried(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: inner.kind,
        node,
        path: vec![RoleSeg::FromTarget(Box::new(inner))],
    }
}

fn vessel_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        line_to(R_FOOT, 0.0),
        line_to(R_FOOT, Y_FOOT),
        arc_to(0.0, Y_BELLY_C, profile::ArcSweep::Ccw, R_NECK, Y_MOUTH),
        line_to(0.0, Y_MOUTH),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}
fn lid_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(R_VENT, LID_BASE)),
        line_to(R_FLANGE, LID_BASE),
        line_to(R_NECK, Y_FLANGE),
        arc_to(0.0, DOME_C, profile::ArcSweep::Ccw, R_KNOB, Y_KNOB),
        line_to(R_KNOB, Y_TOP),
        line_to(R_VENT, Y_TOP),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}
fn spout_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(SPOUT_R0 - SPOUT_WALL, 0.0)),
        line_to(SPOUT_R0, 0.0),
        line_to(SPOUT_R1, SPOUT_LEN),
        line_to(SPOUT_R1 - SPOUT_WALL, SPOUT_LEN),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The tour's frame (u = +X, v = +Y) and its in-plane axis.
fn frame_axis(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let plane = r.insert(frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let axis = r.insert(axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    (plane, axis)
}
fn revolved(
    r: &mut Recorder,
    plane: RecipeNodeId,
    axis: RecipeNodeId,
    lp: LoopProgram,
) -> RecipeNodeId {
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![lp],
    }));
    r.insert(Node::Revolve {
        profile,
        axis,
        angle: ang(TAU),
    })
}
fn census(b: &topo::Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}
fn tori(b: &topo::Body<f64>) -> Vec<(u64, u64, u64)> {
    let mut out: Vec<(u64, u64, u64)> = b
        .faces()
        .filter_map(|(_, f)| match b.get_surface(f.surface) {
            Some(geom::Surface::Torus {
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

/// One-request roll of the rims at `vs` on the lid; the refusal or the body.
fn roll_once(vs: &[u32]) -> Result<(usize, usize, usize), String> {
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let lid = revolved(&mut r, plane, axis, lid_meridian());
    let sel: Vec<StableName> = vs.iter().map(|&v| band_rim(lid, v)).collect();
    let rolled = r.insert(Node::fillet(lid, len(ROLL), sel));
    let ev = eval::<f64>(&r.doc);
    match ev.node_error(rolled) {
        Some(e) => Err(format!("{:?}", e.kind)),
        None => Ok(census(body_of(&ev, rolled))),
    }
}

fn slit_names(v: u32) -> Vec<String> {
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let lid = revolved(&mut r, plane, axis, lid_meridian());
    let rolled = r.insert(Node::fillet(lid, len(ROLL), vec![band_rim(lid, v)]));
    let ev = eval::<f64>(&r.doc);
    select(
        &ev,
        rolled,
        &Selector::of(NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::BandSlit))),
    )
    .iter()
    .map(|n| format!("{:?}", n.path))
    .collect()
}

#[test]
fn r2_bandslit_collision_is_adjacency_shaped() {
    // What each single-rim band names as its slit.
    for v in [1u32, 2, 4] {
        println!("rim v={v}: slits = {:?}", slit_names(v));
    }
    // The scene's three rims in ONE request: the filed refusal.
    let all = roll_once(&[1, 2, 4]);
    println!("roll {{1,2,4}} -> {all:?}");
    let e = all.clone().unwrap_err();
    assert!(
        e.starts_with("Naming(Duplicate")
            && e.contains("BandSlit")
            && e.contains("Meridian(Seam, ProfileEdgeRef { loop_index: 0, segment: 1 })"),
        "{e}"
    );
    // Adjacent pair alone collides; the non-adjacent pairs do not.
    let adj = roll_once(&[1, 2]);
    println!("roll {{1,2}} -> {adj:?}");
    assert!(adj.clone().unwrap_err().starts_with("Naming(Duplicate"));
    let far = roll_once(&[1, 4]);
    println!("roll {{1,4}} -> {far:?}");
    assert_eq!(far.unwrap(), (8, 16, 8));
    let far2 = roll_once(&[2, 4]);
    println!("roll {{2,4}} -> {far2:?}");
    assert_eq!(far2.unwrap(), (8, 16, 8));
}

#[test]
fn r2_two_requests_equal_the_kernels_one_request() {
    let tol = Tol::witness();
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let lid = revolved(&mut r, plane, axis, lid_meridian());
    let first = r.insert(Node::fillet(lid, len(ROLL), vec![band_rim(lid, 1)]));
    let second = r.insert(Node::fillet(
        first,
        len(ROLL),
        vec![
            carried(first, band_rim(lid, 2)),
            carried(first, band_rim(lid, 4)),
        ],
    ));
    let ev = eval::<f64>(&r.doc);
    assert!(
        ev.node_error(second).is_none(),
        "{:?}",
        ev.node_error(second)
    );
    let sharp = body_of(&ev, lid).clone();
    let two = body_of(&ev, second).clone();

    // The kernel's ONE request on the same sharp body, keys found by NAME.
    let keys: Vec<topo::EdgeKey> = [1u32, 2, 4]
        .iter()
        .map(|&v| {
            topo::query::all_edges(&sharp)
                .into_iter()
                .find(|&k| edge_name(&ev, lid, 0, k).ok() == Some(&band_rim(lid, v)))
                .expect("the rim's key")
        })
        .collect();
    let one = sweep::blend::build::fillet_edges(&sharp, &keys, ROLL, tol)
        .expect("the kernel rolls all three")
        .body;
    assert_eq!(census(&one), (9, 18, 9));
    assert_eq!(census(&two), (9, 18, 9));
    let p1 = topo::mass_properties(&one, tol).unwrap();
    let p2 = topo::mass_properties(&two, tol).unwrap();
    println!(
        "one-request V={} A={}; two-request V={} A={}; dV/V={:e} dA/A={:e}",
        p1.volume,
        p1.surface_area,
        p2.volume,
        p2.surface_area,
        (p1.volume - p2.volume) / p1.volume,
        (p1.surface_area - p2.surface_area) / p1.surface_area
    );
    let t1 = tori(&one);
    let t2 = tori(&two);
    println!("tori one={t1:?}\ntori two={t2:?}");
    assert_eq!(
        t1, t2,
        "the three bands' stored (station, R, r) differ between the two spellings"
    );
    assert!(((p1.volume - p2.volume) / p1.volume).abs() < 1e-14);
    // The face ORDER: the torus bands' positions in face iteration.
    let order = |b: &topo::Body<f64>| -> Vec<String> {
        b.faces()
            .map(|(_, f)| match b.get_surface(f.surface) {
                Some(geom::Surface::Torus { center, .. }) => format!("torus@{}", center.y),
                Some(geom::Surface::Plane { .. }) => "plane".into(),
                Some(geom::Surface::Cylinder { .. }) => "cyl".into(),
                Some(geom::Surface::Cone { .. }) => "cone".into(),
                Some(geom::Surface::Sphere { .. }) => "sphere".into(),
                _ => "?".into(),
            })
            .collect()
    };
    println!(
        "face order one={:?}\nface order two={:?}",
        order(&one),
        order(&two)
    );
}

#[test]
fn r2_spout_placement_measured_off_the_placed_body() {
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let body = revolved(&mut r, plane, axis, spout_meridian());
    let turn = (-SPOUT_DIR[0]).atan2(SPOUT_DIR[1]);
    let spout = r.insert(Node::Transform {
        input: body,
        translation: [len(SPOUT_ROOT[0]), len(SPOUT_ROOT[1]), len(SPOUT_ROOT[2])],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(turn),
    });
    let ev = eval::<f64>(&r.doc);
    assert!(ev.node_error(spout).is_none());

    // The matrix, entry by entry, against the hand-written exact one.
    let m = Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), turn);
    let hand = Mat3::from_cols(
        Vec3::new(SPOUT_DIR[1], -SPOUT_DIR[0], 0.0),
        Vec3::new(SPOUT_DIR[0], SPOUT_DIR[1], 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let (a, b) = (format!("{m:?}"), format!("{hand:?}"));
    println!("rotation_about = {a}\nhand           = {b}");
    for (x, y) in [
        (
            m * Vec3::new(1.0, 0.0, 0.0),
            hand * Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            m * Vec3::new(0.0, 1.0, 0.0),
            hand * Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            m * Vec3::new(0.0, 0.0, 1.0),
            hand * Vec3::new(0.0, 0.0, 1.0),
        ),
    ] {
        assert_eq!(
            (x.x.to_bits(), x.y.to_bits(), x.z.to_bits()),
            (y.x.to_bits(), y.y.to_bits(), y.z.to_bits()),
            "column differs: {x:?} vs {y:?}"
        );
    }

    // The placed body's root rim (vertex 1: between the root annulus and
    // the outer cone) and tip rim (vertex 2), read THROUGH THE TRANSFORM
    // NODE by the revolve-minted names.
    let root = edge_frame(&ev, spout, &band_rim(body, 1)).expect("root rim through the transform");
    let tip = edge_frame(&ev, spout, &band_rim(body, 2)).expect("tip rim through the transform");
    let root_res = (root.origin - Point3::new(SPOUT_ROOT[0], SPOUT_ROOT[1], SPOUT_ROOT[2])).norm();
    let want_tip = Point3::new(
        SPOUT_ROOT[0] + SPOUT_LEN * SPOUT_DIR[0],
        SPOUT_ROOT[1] + SPOUT_LEN * SPOUT_DIR[1],
        0.0,
    );
    let tip_res = (tip.origin - want_tip).norm();
    let dir = Vec3::new(SPOUT_DIR[0], SPOUT_DIR[1], SPOUT_DIR[2]);
    let axis_res = (root.axis - dir).norm().min((root.axis + dir).norm());
    let annulus = face_frame(&ev, spout, &band(body, 0)).expect("root annulus");
    println!(
        "root rim centre {:?} residual {root_res:e}; tip rim centre {:?} residual {tip_res:e}; \
         rim axis {:?} vs dir residual {axis_res:e}; root annulus frame origin {:?} axis {:?} \
         (kind {:?})",
        root.origin,
        tip.origin,
        root.axis,
        annulus.origin,
        annulus.axis,
        face_carrier_kind(&ev, spout, &band(body, 0)).unwrap()
    );
    assert_eq!(root_res, 0.0);
    assert_eq!(axis_res, 0.0);
    assert!(tip_res < 1e-16, "{tip_res}");
}

#[test]
fn r2_mouth_by_name_and_its_mutants() {
    let cup_with = |open: fn(RecipeNodeId) -> Vec<StableName>| {
        let mut r = Recorder::new();
        let (plane, axis) = frame_axis(&mut r);
        let pot = revolved(&mut r, plane, axis, vessel_meridian());
        let cup = r.insert(Node::shell(pot, len(WALL), open(pot)));
        let ev = eval::<f64>(&r.doc);
        (r, pot, cup, ev)
    };
    // Only BandPi designated: the kernel's partial-chart gate.
    let (_, _, cup, ev) = cup_with(|pot| vec![band_pi(pot, 3)]);
    match ev.node_error(cup).map(|e| &e.kind) {
        Some(NodeErrorKind::Shell(inner)) => {
            assert!(
                matches!(**inner, ShellError::OpenFaceChartPartial { .. }),
                "{inner:?}"
            );
            println!("BandPi only -> {inner}");
        }
        other => panic!("expected the shell gate, got {other:?}"),
    }
    // Band first (the scene) and BandPi first (swapped).
    let (_, _, cup_a, ev_a) = cup_with(|pot| vec![band(pot, 3), band_pi(pot, 3)]);
    let (_, _, cup_b, ev_b) = cup_with(|pot| vec![band_pi(pot, 3), band(pot, 3)]);
    for (what, cup, ev) in [("Band first", cup_a, &ev_a), ("BandPi first", cup_b, &ev_b)] {
        assert!(ev.node_error(cup).is_none());
        let rims = select(
            ev,
            cup,
            &Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Rim))),
        );
        let body = body_of(ev, cup);
        let p = topo::mass_properties(body, Tol::witness()).unwrap();
        println!(
            "{what}: rim names {:?}; census {:?}; V bits {:#x}; A bits {:#x}",
            rims.iter()
                .map(|n| format!("{:?}", n.path))
                .collect::<Vec<_>>(),
            census(body),
            p.volume.to_bits(),
            p.surface_area.to_bits()
        );
        assert_eq!(rims.len(), 1);
    }
    let va = topo::mass_properties(body_of(&ev_a, cup_a), Tol::witness())
        .unwrap()
        .volume;
    let vb = topo::mass_properties(body_of(&ev_b, cup_b), Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(
        va.to_bits(),
        vb.to_bits(),
        "the two orders differ in volume"
    );
}

#[test]
fn r2_select_canonical_order_by_geometry() {
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let pot = revolved(&mut r, plane, axis, vessel_meridian());
    let lid = revolved(&mut r, plane, axis, lid_meridian());
    let first = r.insert(Node::fillet(lid, len(ROLL), vec![band_rim(lid, 1)]));
    let ev = eval::<f64>(&r.doc);
    let bands = select(
        &ev,
        pot,
        &Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Band))),
    );
    for (i, n) in bands.iter().enumerate() {
        let f = face_frame(&ev, pot, n).unwrap();
        println!(
            "bands[{i}] = {:?} kind {:?} origin.y {}",
            n.path,
            face_carrier_kind(&ev, pot, n).unwrap(),
            f.origin.y
        );
    }
    assert_eq!(bands.len(), 4);
    assert_eq!(face_frame(&ev, pot, &bands[3]).unwrap().origin.y, Y_MOUTH);
    let rims = select(
        &ev,
        lid,
        &Selector::of(NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::BandRim))),
    );
    let stations = [LID_BASE, LID_BASE, Y_FLANGE, Y_KNOB, Y_TOP, Y_TOP];
    for (i, n) in rims.iter().enumerate() {
        let f = edge_frame(&ev, lid, n).unwrap();
        let p = vertex_position(&ev, lid, &meridian_vertex(lid, i as u32)).unwrap();
        println!(
            "rims[{i}] = {:?} centre.y {} radius {}",
            n.path,
            f.origin.y,
            p.x.hypot(p.z)
        );
        assert_eq!(f.origin.y, stations[i]);
    }
    let carried_rims = select(
        &ev,
        first,
        &Selector::of(NamePat::of_kind(EntityKind::Edge).seg(
            SegPat::tag(SegTag::FromTarget).of([NamePat::any().seg(SegPat::tag(SegTag::BandRim))]),
        )),
    );
    for (i, n) in carried_rims.iter().enumerate() {
        println!(
            "carried[{i}] = {:?} centre.y {}",
            n.path,
            edge_frame(&ev, first, n).unwrap().origin.y
        );
    }
    assert_eq!(carried_rims.len(), 5);
    assert_eq!(
        edge_frame(&ev, first, &carried_rims[1]).unwrap().origin.y,
        Y_FLANGE
    );
    assert_eq!(
        edge_frame(&ev, first, &carried_rims[3]).unwrap().origin.y,
        Y_TOP
    );
    // The mutant the brief asks for: vertex 3 (the dome/knob junction) in
    // place of vertex 4. Does it roll, and what does it look like?
    let second = r.insert(Node::fillet(
        first,
        len(ROLL),
        vec![
            carried(first, band_rim(lid, 2)),
            carried(first, band_rim(lid, 3)),
        ],
    ));
    let ev = eval::<f64>(&r.doc);
    match ev.node_error(second) {
        Some(e) => println!("mutant v=3: REFUSES {:?}", e.kind),
        None => {
            let b = body_of(&ev, second);
            let sharp = body_of(&ev, lid);
            let dv = topo::mass_properties(sharp, Tol::witness()).unwrap().volume
                - topo::mass_properties(b, Tol::witness()).unwrap().volume;
            println!(
                "mutant v=3: builds, census {:?}, dV = {dv:e}, tori {:?}",
                census(b),
                tori(b)
            );
        }
    }
}

/// Wall 3's payload: the DOCUMENT's cup and the kernel-direct `shell_open`
/// cup, each unioned with the same placed spout, side by side — and the
/// planar faces' keys and stations on both cups, so the `other_face` key
/// the refusal names can be read.
#[test]
fn r2_wall3_payload_document_vs_kernel_direct() {
    let tol = Tol::witness();
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let pot = revolved(&mut r, plane, axis, vessel_meridian());
    let cup = r.insert(Node::shell(
        pot,
        len(WALL),
        vec![band(pot, 3), band_pi(pot, 3)],
    ));
    let sbody = revolved(&mut r, plane, axis, spout_meridian());
    let turn = (-SPOUT_DIR[0]).atan2(SPOUT_DIR[1]);
    let spout = r.insert(Node::Transform {
        input: sbody,
        translation: [len(SPOUT_ROOT[0]), len(SPOUT_ROOT[1]), len(SPOUT_ROOT[2])],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(turn),
    });
    let ev = eval::<f64>(&r.doc);
    let bellied = body_of(&ev, pot);
    let doc_cup = body_of(&ev, cup);
    let spout_b = body_of(&ev, spout);
    let mouth: Vec<topo::FaceKey> = [band(pot, 3), band_pi(pot, 3)]
        .iter()
        .map(|n| {
            bellied
                .faces()
                .map(|(k, _)| k)
                .find(|&k| face_name(&ev, pot, 0, k).ok() == Some(n))
                .expect("the mouth half's key")
        })
        .collect();
    let kernel_cup = topo::shell_open(bellied, WALL, &mouth, tol)
        .expect("kernel cup")
        .body;
    let planes = |b: &topo::Body<f64>| -> Vec<String> {
        b.faces()
            .filter_map(|(k, f)| match b.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, .. }) => Some(format!("{k:?}@y={}", origin.y)),
                _ => None,
            })
            .collect()
    };
    println!("document cup planar faces: {:?}", planes(doc_cup));
    println!("kernel   cup planar faces: {:?}", planes(&kernel_cup));
    let doc_face_order: Vec<String> = doc_cup.faces().map(|(k, _)| format!("{k:?}")).collect();
    let ker_face_order: Vec<String> = kernel_cup.faces().map(|(k, _)| format!("{k:?}")).collect();
    println!(
        "document cup face keys: {doc_face_order:?}\nkernel   cup face keys: {ker_face_order:?}"
    );
    let d = topo::union(doc_cup, spout_b, tol)
        .err()
        .map(|e| format!("{e:?}"));
    let k = topo::union(&kernel_cup, spout_b, tol)
        .err()
        .map(|e| format!("{e:?}"));
    println!("document cup ∪ spout -> {d:?}\nkernel   cup ∪ spout -> {k:?}");
    assert_eq!(d, k, "the two cups' refusals differ");
}

/// What the shipped `rim_circle` read-back (centre from `edge_frame` of
/// the rim NAME, radius from the meridian VERTEX name) cannot tell
/// apart: two rims on ONE station. If `BandRim(1)` resolved to the
/// vent's rim at vertex 0 (same station, LID_BASE), the read-back would
/// still answer (LID_BASE, R_FLANGE), because the radius never comes
/// from the edge. Executed as the mutant: the EDGE of vertex 0 read
/// beside the VERTEX of vertex 1.
#[test]
fn r2_rim_circle_readback_cannot_see_a_wrong_rim_on_the_same_station() {
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let lid = revolved(&mut r, plane, axis, lid_meridian());
    let ev = eval::<f64>(&r.doc);
    let readback = |edge_v: u32, vertex_v: u32| -> (f64, f64) {
        let centre = edge_frame(&ev, lid, &band_rim(lid, edge_v)).unwrap().origin;
        let p = vertex_position(&ev, lid, &meridian_vertex(lid, vertex_v)).unwrap();
        assert!((p.y - centre.y).abs() < 1e-12);
        (centre.y, p.x.hypot(p.z))
    };
    let honest = readback(1, 1);
    let mutant = readback(0, 1);
    println!("honest (edge 1, vertex 1) = {honest:?}; mutant (edge 0, vertex 1) = {mutant:?}");
    assert_eq!(
        honest, mutant,
        "the read-back distinguishes the two rims after all"
    );
    let mutant_top = readback(5, 4);
    println!(
        "mutant (edge 5, vertex 4) = {mutant_top:?} vs honest {:?}",
        readback(4, 4)
    );
    assert_eq!(mutant_top, readback(4, 4));
}

/// One-request rolls at a smaller radius (so the concave vent/knob
/// rims have headroom): which ADJACENT pairs collide? The filed issue
/// says any adjacent pair does; the slit is named by ONE support's
/// meridian, so the answer is per pair.
fn roll_once_at(vs: &[u32], roll: f64) -> Result<(usize, usize, usize), String> {
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let lid = revolved(&mut r, plane, axis, lid_meridian());
    let sel: Vec<StableName> = vs.iter().map(|&v| band_rim(lid, v)).collect();
    let rolled = r.insert(Node::fillet(lid, len(roll), sel));
    let ev = eval::<f64>(&r.doc);
    match ev.node_error(rolled) {
        Some(e) => Err(format!("{:?}", e.kind)),
        None => Ok(census(body_of(&ev, rolled))),
    }
}

#[test]
fn r2_which_adjacent_pairs_collide() {
    for v in 0..6u32 {
        let mut r = Recorder::new();
        let (plane, axis) = frame_axis(&mut r);
        let lid = revolved(&mut r, plane, axis, lid_meridian());
        let rolled = r.insert(Node::fillet(lid, len(ROLL / 4.0), vec![band_rim(lid, v)]));
        let ev = eval::<f64>(&r.doc);
        let slits: Vec<String> = select(
            &ev,
            rolled,
            &Selector::of(NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::BandSlit))),
        )
        .iter()
        .map(|n| format!("{:?}", n.path))
        .collect();
        println!(
            "rim v={v} alone at ROLL/4: {} ; slits {slits:?}",
            ev.node_error(rolled)
                .map_or("builds".to_string(), |e| format!("{:?}", e.kind))
        );
    }
    for pair in [[0u32, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 0]] {
        let out = roll_once_at(&pair, ROLL / 4.0);
        let verdict = match &out {
            Ok(c) => format!("builds {c:?}"),
            Err(e) if e.starts_with("Naming(Duplicate") => {
                "COLLIDES (Naming(Duplicate))".to_string()
            }
            Err(e) => format!("refuses otherwise: {}", &e[..e.len().min(120)]),
        };
        println!("adjacent pair {pair:?}: {verdict}");
    }
}

/// The merge base's kernel-direct cup — `validated(SketchPlane::xy())`
/// + `revolve` + the plane-chart scan + `shell_open` — unioned with the
/// document's spout, so wall 3's `other_face: FaceKey(1v1)` on the base
/// can be read beside the document's `FaceKey(2v1)`.
#[test]
fn r2_wall3_other_face_on_the_merge_base_spelling() {
    use profile::{ArcSweep, Center, Open, ProfileLoop, SketchPlane, Start};
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(geom_core::Point2::new(0.0, 0.0))
        .line_to(geom_core::Point2::new(R_FOOT, 0.0), tol)
        .expect("base disc")
        .line_to(geom_core::Point2::new(R_FOOT, Y_FOOT), tol)
        .expect("foot")
        .arc_to(
            Center {
                c: geom_core::Point2::new(0.0, Y_BELLY_C),
                winding: ArcSweep::Ccw,
                p: geom_core::Point2::new(R_NECK, Y_MOUTH),
            },
            tol,
        )
        .expect("belly")
        .line_to(geom_core::Point2::new(0.0, Y_MOUTH), tol)
        .expect("mouth disc")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let prof = profile::Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("validates");
    let bellied = sweep::revolve(
        &prof,
        sweep::RevolveAxis {
            origin: geom_core::Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        tol,
    )
    .expect("revolves")
    .body;
    let mouth: Vec<topo::FaceKey> = bellied
        .faces()
        .filter(|(_, f)| {
            matches!(bellied.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - Y_MOUTH).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(mouth.len(), 2);
    let cup = topo::shell_open(&bellied, WALL, &mouth, tol)
        .expect("base cup")
        .body;
    let planes: Vec<String> = cup
        .faces()
        .filter_map(|(k, f)| match cup.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, .. }) => Some(format!("{k:?}@y={}", origin.y)),
            _ => None,
        })
        .collect();
    println!("merge-base-style cup planar faces: {planes:?}");
    println!(
        "merge-base-style cup face keys: {:?}",
        cup.faces()
            .map(|(k, _)| format!("{k:?}"))
            .collect::<Vec<_>>()
    );
    // The document's spout, as before.
    let mut r = Recorder::new();
    let (plane, axis) = frame_axis(&mut r);
    let sbody = revolved(&mut r, plane, axis, spout_meridian());
    let turn = (-SPOUT_DIR[0]).atan2(SPOUT_DIR[1]);
    let spout = r.insert(Node::Transform {
        input: sbody,
        translation: [len(SPOUT_ROOT[0]), len(SPOUT_ROOT[1]), len(SPOUT_ROOT[2])],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(turn),
    });
    let ev = eval::<f64>(&r.doc);
    let e = topo::union(&cup, body_of(&ev, spout), tol)
        .err()
        .map(|e| format!("{e:?}"));
    println!("merge-base-style cup ∪ spout -> {e:?}");
}

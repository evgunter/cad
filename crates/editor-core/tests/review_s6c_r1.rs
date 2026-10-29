//! R1's probes on INTENT s6 C (review only).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use geom_core::{Bounds, Interval, Real, Sym};

fn upright() -> editor_core::Frame {
    editor_core::test_support::axes_at([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0])
}

fn closing(mut steps: Vec<ProgramStep<Formula>>, home: [f64; 2]) -> Vec<ProgramStep<Formula>> {
    steps.extend([
        ProgramStep::LineTo(ProgramTarget::Point(len2(home))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    steps
}

fn run_to(at: [f64; 2], d: [f64; 2], passage: TracePassage, nth: u32) -> Vec<ProgramStep<Formula>> {
    vec![
        ProgramStep::At(len2(at)),
        ProgramStep::Toward { dx: scl(d[0]), dy: scl(d[1]) },
        ProgramStep::LineTo(ProgramTarget::OnTrace { face: 0, crossing: TraceCrossing { passage, nth } }),
    ]
}

/// P1: the Sym lane keeps the radius symbol in the traced wall.
#[test]
fn r1_sym_lane_keeps_the_radius() {
    let width = Tol::witness().eps() / 4096.0;
    let scene = traced_boss(RADIUS, Some(width), boss_steps());
    let r = scene.doc.var_named("r").expect("r");
    let opts = EvalOptions { symbols: true, ..EvalOptions::default() };
    let opts = &opts;
    let ((), counts) = geom_core::sym::with_session_rules(geom_core::SymBudget { max_terms: 4096, max_degree: 128 }, geom_core::SymRules::shipped(), || {
        let ev: Evaluation<Sym<f64>> =
            evaluate(&scene.doc, None, &CancelToken::new(), opts, Tol::witness());
        assert!(ev.node_error(scene.boss).is_none(), "{:?}", ev.node_error(scene.boss).map(|e| e.kind.to_string()));
        let wall = vertices(&ev, scene.boss)
            .into_iter()
            .map(|p| p[1])
            .max_by(|a, b| a.value.total_cmp(&b.value))
            .unwrap();
        let leaf = editor_core::ParamBox::from_axes(std::collections::BTreeMap::new());
        let rsym = match editor_core::var_env_over::<Sym<f64>, _>(&scene.doc, &leaf).unwrap().bindings[&r] {
            editor_core::ParamValue::Continuous { value, .. } => value,
            ref o => panic!("{o:?}"),
        };
        let band = geom_core::predicate::Band::new(1e-12, 1e-11).unwrap();
        let before = geom_core::sym::session_counts();
        let s1 = geom_core::k_stats::decide("r1", geom_core::predicate::Margin::of(wall - rsym), band);
        let mid = geom_core::sym::session_counts();
        let s2 = geom_core::k_stats::decide("r1", geom_core::predicate::Margin::of(wall - Sym::from_f64(RADIUS)), band);
        let after = geom_core::sym::session_counts();
        let rim = vertices(&ev, scene.cylinder).into_iter().map(|p| p[0]).max_by(|a, b| a.value.total_cmp(&b.value)).unwrap();
        let s3 = geom_core::k_stats::decide("r1", geom_core::predicate::Margin::of(rim - rsym), band);
        let ctl = geom_core::sym::session_counts();
        eprintln!("R1-SYM control rim-r: {s3:?} symbolic {:?} -> {:?}", after.map(|c| c.symbolic), ctl.map(|c| c.symbolic));
        let bx = vertices(&ev, scene.boss).into_iter().map(|p| p[0]).max_by(|a, b| a.value.total_cmp(&b.value)).unwrap();
        let s4 = geom_core::k_stats::decide("r1", geom_core::predicate::Margin::of(bx - Sym::from_f64(REACH)), band);
        eprintln!("R1-SYM control boss x - REACH: {s4:?} symbolic -> {:?}", geom_core::sym::session_counts().map(|c| c.symbolic));
        eprintln!("R1-SYM wall={:?} rsym={:?} R1-SYM wall-r: {s1:?} counts {before:?} -> {mid:?} R1-SYM wall-0.5: {s2:?} -> {after:?}", wall.value, rsym.value);
    });
    eprintln!("R1-SYM session {counts:?}");
}

/// P2: an Interval box wider than ε/4096.
#[test]
fn r1_interval_wide_box() {
    for width in [1e-6, 1e-4, 1e-2] {
        let scene = traced_boss(RADIUS, Some(width), boss_steps());
        let boxed = EvalOptions {
            param_box: Some(std::sync::Arc::new(editor_core::analysis::ParamBox::of(
                &editor_core::analysis::analyzed_box(&scene.doc, &editor_core::analysis::AnalysisPolicy::default()),
            ))),
            ..EvalOptions::default()
        };
        let ev = evaluate::<Interval>(&scene.doc, None, &CancelToken::new(), &boxed, Tol::witness());
        match ev.node_error(scene.boss).or_else(|| ev.node_error(scene.footprint)) {
            Some(e) => eprintln!("R1-IV width {width}: refused {}", e.kind),
            None => {
                let ys: Vec<Interval> = vertices(&ev, scene.boss).into_iter().map(|p| p[1]).collect();
                let wall = ys.iter().copied().max_by(|a, b| a.hi().total_cmp(&b.hi())).unwrap();
                let xs: Vec<Interval> = vertices(&ev, scene.boss).into_iter().map(|p| p[0]).collect();
                eprintln!("R1-IV width {width}: wall [{}, {}] contains box: {}; xs {:?}", wall.lo(), wall.hi(),
                    wall.lo() <= RADIUS - width && wall.hi() >= RADIUS + width, xs);
            }
        }
    }
}

/// P3: a frame of one space, a face of another.
#[test]
fn r1_cross_space_frame_and_face() {
    let (doc, a) = cylinder(RADIUS, None);
    let (doc, disc2) = fixture::insert(doc, Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Circle { centre: [len(0.0), len(0.0)], radius: len(0.75) }],
        ids: Vec::new(), on: None,
    }));
    let (doc, b) = fixture::insert(doc, Node::Extrude { profile: disc2.into(), distance: len(1.0) });
    assert_ne!(doc.node_space(a), doc.node_space(b), "two spaces");
    let ev = eval(&doc);
    let wall_b = face(&ev, b, SurfaceKind::Cylinder);
    let frame = frame_at_origin(&doc, a);
    let (doc, p) = fixture::insert(doc, Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Chain(boss_steps())],
        ids: Vec::new(),
        on: Some(DrawnOn { frame, faces: vec![Operand::select(Operand::output(b, 0), vec![wall_b])] }),
    }));
    let ev = eval(&doc);
    eprintln!("R1-SPACE space_fault={:?} err={:?}", doc.node_space(p), ev.node_error(p).map(|e| e.kind.to_string()));
    assert!(ev.node_error(p).is_some(), "a frame of A with a face of B must refuse");
}

/// P4: a cylinder crossing far along a nearly axial ray.
#[test]
fn r1_cylinder_far_crossing() {
    let (doc, cyl) = cylinder(RADIUS, None);
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    for dx in [0.5, 0.1, 0.01] {
        let steps = closing(run_to([0.0, 0.5], [dx, 1.0], TracePassage::Leave, 1), [-1.0, 0.5]);
        let (doc, p) = drawn_on(doc.clone(), cyl, &upright(), wall.clone(), steps);
        let ev = eval(&doc);
        match ev.node_error(p) {
            Some(e) => eprintln!("R1-FAR dx {dx}: refused {}", e.kind),
            None => eprintln!("R1-FAR dx {dx}: max x {}", swept_max(doc, p, 0)),
        }
    }
}

/// P5: the torus crossings, with a closing point that cannot mask the crossing.
#[test]
fn r1_torus_nth() {
    let doc = ProfileDoc::empty_derived("r1_torus", Tol::witness());
    let (doc, ring) = fixture::insert(doc, Node::Tube {
        major_radius: len(2.0), window: editor_core::TubeWindow::Full, minor_radius: len(0.5),
    });
    let ev = eval(&doc);
    let tube = face(&ev, ring, SurfaceKind::Torus);
    for (frame, label) in [(editor_core::Frame::IDENTITY, "mid"), (upright(), "meridian")] {
        for (passage, nth) in [(TracePassage::Enter, 1), (TracePassage::Leave, 1), (TracePassage::Enter, 2), (TracePassage::Leave, 2)] {
            let steps = closing(run_to([-3.0, 0.0], [1.0, 0.0], passage, nth), [-3.0, 1.0]);
            let (d, p) = drawn_on(doc.clone(), ring, &frame, tube.clone(), steps);
            let ev = eval(&d);
            match ev.node_error(p) {
                Some(e) => eprintln!("R1-TORUS {label} {passage:?} {nth}: refused {}", e.kind),
                None => eprintln!("R1-TORUS {label} {passage:?} {nth}: max x {}", swept_max(d, p, 0)),
            }
        }
    }
}

/// P6: a concave face (a ring's inner wall): leaving the hole enters the material.
#[test]
fn r1_concave_ring() {
    let doc = ProfileDoc::empty_derived("r1_ring", Tol::witness());
    let (doc, prof) = fixture::insert(doc, Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::Circle { centre: [len(0.0), len(0.0)], radius: len(1.0) },
            LoopProgram::Circle { centre: [len(0.0), len(0.0)], radius: len(0.5) },
        ],
        ids: Vec::new(), on: None,
    }));
    let (doc, ring) = fixture::insert(doc, Node::Extrude { profile: prof.into(), distance: len(1.0) });
    let ev = eval(&doc);
    assert!(ev.node_error(ring).is_none(), "{:?}", ev.node_error(ring));
    let walls: Vec<StableName> = all_faces(&ev, ring).into_iter()
        .filter(|n| face_carrier_kind(&ev, ring, n) == Ok(SurfaceKind::Cylinder)).collect();
    for w in walls {
        for passage in [TracePassage::Enter, TracePassage::Leave] {
            let steps = closing(run_to([0.0, 0.0], [0.0, 1.0], passage, 1), [-2.0, 0.0]);
            let (d, p) = drawn_on(doc.clone(), ring, &editor_core::Frame::IDENTITY, w.clone(), steps);
            let ev = eval(&d);
            match ev.node_error(p) {
                Some(e) => eprintln!("R1-RING {w:?} {passage:?}: refused {}", e.kind),
                None => eprintln!("R1-RING {w:?} {passage:?}: max y {}", swept_max(d, p, 1)),
            }
        }
    }
}

/// P7: moving the traced body (its centre, a variable) moves the trace.
#[test]
fn r1_moved_cylinder() {
    let doc = ProfileDoc::empty_derived("r1_move", Tol::witness());
    let mut doc = doc;
    for (n, v) in [("r", RADIUS), ("cy", 0.0)] {
        doc = fixture::step(doc, DocEdit::DeclareVar {
            name: VarName::new(n).unwrap(),
            def: VarDecl::Free(FreeVar::Continuous { dim: Dimension::Length, value: v,
                display_unit: UnitSym::canonical_for(Dimension::Length), distribution: None }),
        }).0;
    }
    let cy = Formula::named(VarName::new("cy").unwrap(), Dimension::Length);
    let (doc, disc) = fixture::insert(doc, Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Circle { centre: [len(0.0), cy], radius: r() }],
        ids: Vec::new(), on: None,
    }));
    let (doc, cyl) = fixture::insert(doc, Node::Extrude { profile: disc.into(), distance: len(HEIGHT) });
    let ev = eval(&doc);
    let wall = face(&ev, cyl, SurfaceKind::Cylinder);
    let (doc, p) = drawn_on(doc, cyl, &editor_core::Frame::IDENTITY, wall, boss_steps());
    let (doc, boss) = fixture::insert(doc, Node::Extrude { profile: p.into(), distance: len(BOSS_H) });
    for v in [0.0, 0.25, -0.125] {
        let (d, _) = fixture::step(doc.clone(), DocEdit::SetVarValue {
            var: VarName::new("cy").unwrap().into(), value: editor_core::FreeValue::Continuous(v) });
        let ev = eval(&d);
        match ev.node_error(boss) {
            Some(e) => eprintln!("R1-MOVE cy {v}: {}", e.kind),
            None => eprintln!("R1-MOVE cy {v}: boss max y {} (expect {})", max_at(&ev, boss, 1), v + RADIUS),
        }
    }
}

/// P8: the cone revolved on a rotated, offset-free frame of the sphere: the apex lands on world z.
#[test]
fn r1_revolve_on_a_rotated_frame() {
    let (doc, sph) = sphere();
    let ev = eval(&doc);
    let wall = face(&ev, sph, SurfaceKind::Sphere);
    let (doc, section) = drawn_on(doc, sph, &upright(), wall, traced_cone());
    let (doc, cone) = fixture::insert(doc, fixture::revolve_about(section, (0.0, 0.0), fixture::along_y(), fixture::ang(std::f64::consts::TAU)));
    let ev = eval(&doc);
    assert!(ev.node_error(cone).is_none(), "{:?}", ev.node_error(cone).map(|e| e.kind.to_string()));
    let pts = vertices(&ev, cone);
    let apex = pts.iter().min_by(|a, b| a[2].total_cmp(&b[2])).unwrap();
    eprintln!("R1-REV apex {apex:?} (expect [0,0,{APEX}]); space same: {}", doc.node_space(cone) == doc.node_space(sph));
    for p in &pts { eprintln!("R1-REV v {p:?}"); }
}

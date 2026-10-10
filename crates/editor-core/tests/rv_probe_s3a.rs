//! Reviewer probes for PR #4556 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len, scl, ang};
use editor_core::eval::bound_pose;
use editor_core::pose::{PoseCoords, PoseDef};
use editor_core::{
    CancelToken, DocEdit, EvalOptions, Evaluation, ExtrudeSide, Formula, Node, NodeResult,
    Operand, OperandSlot, PoseValue, ProfileDoc, RecipeNodeId, SlotId, SlotValue, VarKind,
    all_faces, evaluate, face_carrier_kind,
};
use geom::SurfaceKind;
use geom_core::{Tol, UnitVec3};

fn eval(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(doc, prior, &CancelToken::new(), &EvalOptions::default(), Tol::witness())
}

fn pose(def: PoseDef<Operand, Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}

fn in_frame_plane(f: RecipeNodeId, origin: [f64; 3], normal: [f64; 3]) -> Operand {
    pose(PoseDef::InFrame {
        frame: Operand::output(f, 0),
        coords: PoseCoords::Plane { origin: origin.map(len), normal: normal.map(scl) },
    })
}

fn in_frame_point(f: RecipeNodeId, p: [f64; 3]) -> Operand {
    pose(PoseDef::InFrame {
        frame: Operand::output(f, 0),
        coords: PoseCoords::Point { position: p.map(len) },
    })
}

/// The tool: the plane normal to the line (F1's x=0) ∩ (F2's z=0) through
/// the foot of `point` on it.
fn tool(f1: RecipeNodeId, f2: RecipeNodeId, point_frame: RecipeNodeId) -> Operand {
    pose(PoseDef::Project {
        of: pose(PoseDef::Through {
            axis: pose(PoseDef::Meet {
                a: in_frame_plane(f1, [0.0; 3], [1.0, 0.0, 0.0]),
                b: in_frame_plane(f2, [0.0; 3], [0.0, 0.0, 1.0]),
            }),
            point: in_frame_point(point_frame, [0.5, 0.5, 0.5]),
        }),
        to: VarKind::Plane,
    })
}

fn split_doc() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("rv_collide", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude { profile: p.into(), distance: len(1.0), side: ExtrudeSide::Along },
    );
    let (doc, f1) = fixture::insert(doc, fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, f2) =
        fixture::insert(doc, fixture::frame([0.3, 0.2, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, split) = fixture::insert(
        doc,
        Node::Split { target: Operand::output(cube, 0), tool: tool(f1, f2, f1) },
    );
    (doc, cube, f1, f2, split)
}

fn split_tool(doc: &ProfileDoc, ev: &Evaluation<f64>, split: RecipeNodeId) -> String {
    let Some(Node::Split { tool, .. }) = doc.node(split) else { panic!() };
    format!("{:?}", bound_pose(doc, ev, OperandSlot::Tool, *tool, Tol::witness()))
}

/// Claim 8: two different pose trees whose leaves dedupe to the same
/// port list and whose words agree share a content key.
#[test]
fn rv_content_key_collision_through_deduped_read_ports() {
    let (doc1, _cube, f1, f2, split) = split_doc();
    let ev1 = eval(&doc1, None);
    let doc2 = editor_core::apply(
        &doc1,
        &DocEdit::SetParam {
            node: split,
            slot: SlotId::Operand(OperandSlot::Tool),
            value: SlotValue::Read(tool(f1, f2, f2)),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("re-tools")
    .doc;
    let fresh2 = eval(&doc2, None);
    let memo2 = eval(&doc2, Some(&ev1));
    let key = |ev: &Evaluation<f64>| match ev.nodes.get(&split) {
        Some(NodeResult::Ok(v)) => v.content_key.0,
        other => panic!("{other:?}"),
    };
    let body = |ev: &Evaluation<f64>| match ev.nodes.get(&split) {
        Some(NodeResult::Ok(v)) => format!("{:?}", v.payload),
        other => panic!("{other:?}"),
    };
    println!("RV tool1 {}", split_tool(&doc1, &ev1, split));
    println!("RV tool2 {}", split_tool(&doc2, &fresh2, split));
    println!("RV key1 {:032x} key2 {:032x}", key(&ev1), key(&fresh2));
    println!("RV memo reused={} recomputed={}", memo2.reused, memo2.recomputed);
    let differs_geom = body(&ev1) != body(&fresh2);
    println!("RV geometry differs between the two tools: {differs_geom}");
    println!("RV memo body == fresh body: {}", body(&memo2) == body(&fresh2));
    assert!(differs_geom, "premise: the two tools split differently");
    assert_ne!(key(&ev1), key(&fresh2), "two tools, one content key");
    assert_eq!(body(&memo2), body(&fresh2), "the memo served the other tool's halves");
}

fn ring(a_origin: [f64; 3], na: [f64; 3], nb: [f64; 3], z0: f64, h: f64) -> String {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("rv_meet", Tol::witness()),
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(2.0, 0.5), (3.0, 0.5), (3.0, 1.5), (2.0, 1.5)]],
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude { profile: p.into(), distance: len(h), side: ExtrudeSide::Along },
    );
    let (doc, world) =
        fixture::insert(doc, fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, ring) = fixture::insert(
        doc,
        Node::Pattern {
            input: block.into(),
            count: Formula::count(3),
            kind: editor_core::PatternKind::Circular {
                axis: pose(PoseDef::Meet {
                    a: in_frame_plane(world, a_origin, na),
                    b: in_frame_plane(world, [0.0; 3], nb),
                }),
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        },
    );
    let ev = eval(&doc, None);
    format!("{:?}", ev.value(ring).expect("the ring builds").payload)
}

/// Claim 5: a Meet's representative point is invisible to the ring.
#[test]
fn rv_meet_representative_general_coordinates() {
    // Same lines as the PR's row, but the block at a non-dyadic height.
    let base = ring([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.1, 0.37);
    let mut moved_same = 0;
    let mut total = 0;
    for moved in [[0.0, 4.0, 8.0], [0.0, -0.75, 3.0], [0.0, 0.1, 0.3], [0.0, 0.0, 0.7], [0.0, 0.0, 1.3]] {
        total += 1;
        let r = ring(moved, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.1, 0.37);
        if r == base {
            moved_same += 1;
        } else {
            println!("RV axis-aligned differs at {moved:?}");
        }
    }
    // A tilted line: a = plane through origin with normal (0.6,0.8,0), b with (0,0.6,0.8).
    let na = [0.6, 0.8, 0.0];
    let nb = [0.0, 0.6, 0.8];
    let tbase = ring([0.0; 3], na, nb, 0.1, 0.37);
    for moved in [[-0.8, 0.6, 2.0], [4.0, -3.0, -1.5], [0.08, -0.06, 0.7], [-0.4, 0.3, 0.0]] {
        total += 1;
        let r = ring(moved, na, nb, 0.1, 0.37);
        if r == tbase {
            moved_same += 1;
        } else {
            println!("RV tilted differs at {moved:?}");
        }
    }
    println!("RV representative: {moved_same}/{total} bit-identical");
    assert_eq!(moved_same, total);
}

/// Claim 6: port 1's axis vs the sweep's own axis (a cylinder wall's).
#[test]
fn rv_revolve_axis_output_matches_the_sweep_axis() {
    let s = 1.0 / 3.0_f64.sqrt();
    let (doc, _plane, profile) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("rv_rev", Tol::witness()),
        [0.1, 0.2, 0.3],
        [s, s, s],
        [1.0 / 2.0_f64.sqrt(), -1.0 / 2.0_f64.sqrt(), 0.0],
        vec![fixture::square(2.0, 0.3, 0.5)],
    );
    let (doc, revolve) = fixture::insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_origin: [len(0.0), len(0.0)],
            axis_direction: [scl(0.0), scl(0.7)],
            angle: ang(std::f64::consts::TAU),
        },
    );
    let ev = eval(&doc, None);
    assert!(matches!(ev.nodes.get(&revolve), Some(NodeResult::Ok(_))), "{:?}", ev.nodes.get(&revolve));
    let wall = all_faces(&ev, revolve)
        .into_iter()
        .find(|n| face_carrier_kind(&ev, revolve, n) == Ok(SurfaceKind::Cylinder))
        .expect("a cylindrical wall");
    let axis = doc.output(revolve, 1).unwrap();
    let (doc, ring) = fixture::insert(
        doc,
        Node::Pattern {
            input: Operand::output(revolve, 0),
            count: Formula::count(2),
            kind: editor_core::PatternKind::Circular {
                axis: pose(PoseDef::Axis {
                    of: Operand::select(Operand::output(revolve, 0), vec![wall]),
                }),
                step: ang(0.5),
            },
        },
    );
    let ev = eval(&doc, None);
    let Some(Node::Pattern { kind: editor_core::PatternKind::Circular { axis: wall_axis, .. }, .. }) =
        doc.node(ring)
    else {
        panic!()
    };
    let out = bound_pose(&doc, &ev, OperandSlot::Axis, axis, Tol::witness());
    let sweep = bound_pose(&doc, &ev, OperandSlot::Axis, *wall_axis, Tol::witness());
    let (Ok(PoseValue::Axis { origin: o1, dir: d1 }), Ok(PoseValue::Axis { origin: o2, dir: d2 })) =
        (out, sweep)
    else {
        panic!()
    };
    let (d1, d2) = (UnitVec3::get(d1), UnitVec3::get(d2));
    println!("RV port1 dir {:?} origin {:?}", (d1.x, d1.y, d1.z), (o1.x, o1.y, o1.z));
    println!("RV wall  dir {:?} origin {:?}", (d2.x, d2.y, d2.z), (o2.x, o2.y, o2.z));
    let off = (o2 - o1) - d1 * (o2 - o1).dot(d1);
    println!(
        "RV dir bit-equal {} ; |d1-d2| {:e} ; wall origin's distance from port-1 line {:e}",
        (d1.x, d1.y, d1.z) == (d2.x, d2.y, d2.z),
        (d1 - d2).norm(),
        off.norm()
    );
}

pub fn flip_doc() -> ProfileDoc {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("rv_load", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude { profile: p.into(), distance: len(1.0), side: ExtrudeSide::Along },
    );
    let cap = fixture::fname(cube, editor_core::RoleSeg::Cap(editor_core::CapEnd::End));
    let (doc, _split) = fixture::insert(
        doc,
        Node::Split {
            target: Operand::output(cube, 0),
            tool: pose(PoseDef::Standoff {
                plane: pose(PoseDef::Flip {
                    pose: pose(PoseDef::Plane {
                        face: Operand::select(Operand::output(cube, 0), vec![cap]),
                    }),
                }),
                by: len(0.5),
            }),
        },
    );
    doc
}

#[test]
fn rv_dump_pose_json() {
    let doc = flip_doc();
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("saves");
    std::fs::write("/tmp/claude-0/rv/flip_doc.pncad", &text).unwrap();
    let back = editor_core::persist::load(&text, Tol::witness()).expect("loads");
    let again = editor_core::persist::save(&back.snapshot, &back.edits, Tol::witness()).unwrap();
    println!("RV round trip bit-identical: {}", again == text);
}

#[test]
fn rv_load_mutants() {
    let dir = "/tmp/claude-0/rv/mutants";
    let mut names: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    names.sort();
    for path in names {
        let text = std::fs::read_to_string(&path).unwrap();
        let got = std::panic::catch_unwind(|| editor_core::persist::load(&text, Tol::witness()).map(|_| ()));
        match got {
            Ok(Ok(())) => println!("RV MUTANT {} LOADED", path.display()),
            Ok(Err(e)) => println!("RV MUTANT {} refused: {e}", path.display()),
            Err(_) => println!("RV MUTANT {} PANICKED", path.display()),
        }
    }
}

fn boxed(doc: ProfileDoc, lo: [f64; 3], hi: [f64; 3]) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        doc,
        [0.0, 0.0, lo[2]],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(lo[0], lo[1]), (hi[0], lo[1]), (hi[0], hi[1]), (lo[0], hi[1])]],
    );
    fixture::insert(
        doc,
        Node::Extrude { profile: p.into(), distance: len(hi[2] - lo[2]), side: ExtrudeSide::Along },
    )
}

/// Claim 2: a planar face whose carrier sense is reversed (a pocket's
/// floor, minted by the tool) reads with its OUTWARD normal (+z).
#[test]
fn rv_pocket_floor_reads_outward() {
    let doc = ProfileDoc::empty_derived("rv_pocket", Tol::witness());
    let (doc, blank) = boxed(doc, [0.0, 0.0, 0.0], [2.0, 2.0, 1.0]);
    let (doc, tool) = boxed(doc, [0.5, 0.5, 0.5], [1.5, 1.5, 2.0]);
    let (doc, cut) = fixture::insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Subtract,
            a: blank.into(),
            b: tool.into(),
            declare: Vec::new(),
        },
    );
    let ev = eval(&doc, None);
    let planar: Vec<_> = all_faces(&ev, cut)
        .into_iter()
        .filter(|n| face_carrier_kind(&ev, cut, n) == Ok(SurfaceKind::Plane))
        .collect();
    let mut doc = doc;
    let mut splits = Vec::new();
    for face in &planar {
        let (d, s) = fixture::insert(
            doc,
            Node::Split {
                target: Operand::output(cut, 0),
                tool: pose(PoseDef::Plane { face: Operand::select(Operand::output(cut, 0), vec![face.clone()]) }),
            },
        );
        doc = d;
        splits.push(s);
    }
    let ev = eval(&doc, None);
    let mut wrong = 0;
    for s in splits {
        let Some(Node::Split { tool, .. }) = doc.node(s) else { panic!() };
        let Ok(PoseValue::Plane { origin, normal }) = bound_pose(&doc, &ev, OperandSlot::Tool, *tool, Tol::witness()) else { panic!() };
        let n = UnitVec3::get(normal);
        // outward test: a point a little along n from the face's plane, at the
        // face's own location, must be outside the solid [0,2]^2x[0,1] minus pocket.
        println!("RV face plane origin {:?} normal {:?}", (origin.x, origin.y, origin.z), (n.x, n.y, n.z));
        if (origin.z - 0.5).abs() < 1e-12 && n.z.abs() > 0.5 && n.z < 0.0 {
            wrong += 1;
        }
    }
    println!("RV pocket floor read inward: {wrong}");
    assert_eq!(wrong, 0);
}

/// Claim 2, on a revolve's annuli: one is minted `sense: false`.
#[test]
fn rv_revolved_annuli_read_outward() {
    let (doc, _plane, profile) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("rv_annuli", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![fixture::square(2.0, 0.0, 0.5)],
    );
    let (doc, revolve) = fixture::insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_origin: [len(0.0), len(0.0)],
            axis_direction: [scl(0.0), scl(1.0)],
            angle: ang(std::f64::consts::TAU),
        },
    );
    let ev = eval(&doc, None);
    let planar: Vec<_> = all_faces(&ev, revolve)
        .into_iter()
        .filter(|n| face_carrier_kind(&ev, revolve, n) == Ok(SurfaceKind::Plane))
        .collect();
    assert_eq!(planar.len(), 2);
    let mut doc = doc;
    let mut splits = Vec::new();
    for face in &planar {
        let (d, s) = fixture::insert(
            doc,
            Node::Split {
                target: Operand::output(revolve, 0),
                tool: pose(PoseDef::Plane { face: Operand::select(Operand::output(revolve, 0), vec![face.clone()]) }),
            },
        );
        doc = d;
        splits.push(s);
    }
    let ev = eval(&doc, None);
    let mut wrong = 0;
    for s in splits {
        let Some(Node::Split { tool, .. }) = doc.node(s) else { panic!() };
        let Ok(PoseValue::Plane { origin, normal }) = bound_pose(&doc, &ev, OperandSlot::Tool, *tool, Tol::witness()) else { panic!() };
        let n = UnitVec3::get(normal);
        println!("RV annulus origin {:?} normal {:?}", (origin.x, origin.y, origin.z), (n.x, n.y, n.z));
        if n.z * origin.z < 0.0 { wrong += 1; }
    }
    println!("RV annuli read inward: {wrong}");
    assert_eq!(wrong, 0);
}

fn floats(s: &str) -> Vec<f64> {
    s.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == 'e'))
        .filter_map(|t| t.parse::<f64>().ok())
        .collect()
}

#[test]
fn rv_meet_representative_magnitude() {
    let na = [0.6, 0.8, 0.0];
    let nb = [0.0, 0.6, 0.8];
    let a = floats(&ring([0.0; 3], na, nb, 0.1, 0.37));
    let b = floats(&ring([4.0, -3.0, -1.5], na, nb, 0.1, 0.37));
    assert_eq!(a.len(), b.len());
    let worst = a.iter().zip(&b).map(|(x, y)| (x - y).abs()).fold(0.0_f64, f64::max);
    let n = a.iter().zip(&b).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
    println!("RV meet repr: {n} of {} numbers differ, worst {worst:e}", a.len());
}

/// Claim 3 via SetParam and via nesting.
#[test]
fn rv_observed_through_set_param_and_nesting() {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("rv_obs", Tol::witness()),
        [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, cube) = fixture::insert(doc, Node::Extrude { profile: p.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc, measure) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![fixture::cap_ref(cube, editor_core::CapEnd::Start), fixture::cap_ref(cube, editor_core::CapEnd::End)],
    );
    let out = fixture::output(&doc, measure);
    let cap = fixture::fname(cube, editor_core::RoleSeg::Cap(editor_core::CapEnd::End));
    let observed_tool = || pose(PoseDef::Flip {
        pose: pose(PoseDef::Standoff {
            plane: pose(PoseDef::Plane { face: Operand::select(Operand::output(cube, 0), vec![cap.clone()]) }),
            by: Formula::mul(Formula::var(out, editor_core::Dimension::Length), scl(-0.5)).unwrap(),
        }),
    });
    let nested = fixture::insert_refused(&doc, Node::Split { target: Operand::output(cube, 0), tool: observed_tool() });
    println!("RV nested insert: {nested}");
    let (doc, split) = fixture::insert(doc, Node::Split {
        target: Operand::output(cube, 0),
        tool: pose(PoseDef::Plane { face: Operand::select(Operand::output(cube, 0), vec![cap.clone()]) }),
    });
    let set = editor_core::apply(
        &doc,
        &DocEdit::SetParam { node: split, slot: SlotId::Operand(OperandSlot::Tool), value: SlotValue::Read(observed_tool()), fresh: Vec::new() },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    match set {
        Ok(applied) => {
            println!("RV set_param ACCEPTED");
            let saved = editor_core::persist::save(&applied.doc, &[], Tol::witness());
            println!("RV save of it: {:?}", saved.map(|_| "saved"));
        }
        Err(e) => println!("RV set_param refused: {e}"),
    }
}

#[test]
fn rv_dump_observed_doc() {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("rv_obs2", Tol::witness()),
        [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, cube) = fixture::insert(doc, Node::Extrude { profile: p.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc, measure) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![fixture::cap_ref(cube, editor_core::CapEnd::Start), fixture::cap_ref(cube, editor_core::CapEnd::End)],
    );
    let out = fixture::output(&doc, measure);
    println!("RV measure out {out:?}");
    let cap = fixture::fname(cube, editor_core::RoleSeg::Cap(editor_core::CapEnd::End));
    let (doc, _split) = fixture::insert(doc, Node::Split {
        target: Operand::output(cube, 0),
        tool: pose(PoseDef::Standoff {
            plane: pose(PoseDef::Plane { face: Operand::select(Operand::output(cube, 0), vec![cap]) }),
            by: len(-0.25),
        }),
    });
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("saves");
    std::fs::write("/tmp/claude-0/rv/obs_doc.pncad", &text).unwrap();
}

#[test]
fn rv_meet_nearly_parallel() {
    for tilt in [1e-6, 1e-8, 1e-10, 1e-12] {
        let (doc, p) = fixture::on_frame(
            ProfileDoc::empty_derived("rv_np", Tol::witness()),
            [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
            vec![vec![(2.0, 0.5), (3.0, 0.5), (3.0, 1.5), (2.0, 1.5)]],
        );
        let (doc, block) = fixture::insert(doc, Node::Extrude { profile: p.into(), distance: len(1.0), side: ExtrudeSide::Along });
        let (doc, world) = fixture::insert(doc, fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        let (doc, ring) = fixture::insert(doc, Node::Pattern {
            input: block.into(),
            count: Formula::count(3),
            kind: editor_core::PatternKind::Circular {
                axis: pose(PoseDef::Meet {
                    a: in_frame_plane(world, [0.0; 3], [1.0, 0.0, 0.0]),
                    b: in_frame_plane(world, [0.001, 0.0, 0.0], [1.0, tilt, 0.0]),
                }),
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        });
        let ev = eval(&doc, None);
        let Some(Node::Pattern { kind: editor_core::PatternKind::Circular { axis, .. }, .. }) = doc.node(ring) else { panic!() };
        let v = bound_pose(&doc, &ev, OperandSlot::Axis, *axis, Tol::witness());
        println!("RV tilt {tilt:e}: eps {:e} -> {v:?}", Tol::witness().eps());
    }
}

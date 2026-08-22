//! M4 PR 2 wiring tests (spec D3): datum evaluation, the revolve
//! axis's decided in-plane projection, rotational transforms,
//! patterns as data, Declare pass-through, and the typed operand
//! refusal doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use editor_core::{
    BooleanOp, CancelToken, Datum, EvalOptions, Evaluation, Node, NodeErrorKind, NodeResult,
    PatternKind, ProfileDoc, ValuePayload, evaluate,
};
use fixture::{ang, desc, insert, len, scl};
use geom_core::Tol;
use topo::{mass_properties, validate, validate_closed};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit-square profile `[x0,x0+1]×[y0,y0+1]` on the xy plane plus
/// its extrude, as a two-node prelude.
fn unit_cube(doc: ProfileDoc, x0: f64, y0: f64) -> (ProfileDoc, editor_core::RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (x0, y0),
                (x0 + 1.0, y0),
                (x0 + 1.0, y0 + 1.0),
                (x0, y0 + 1.0),
            ]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    )
}

fn y_axis(doc: ProfileDoc, origin_z: f64) -> (ProfileDoc, editor_core::RecipeNodeId) {
    insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(origin_z)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    )
}

#[test]
fn revolve_wires_the_datum_axis_through_the_sketch_plane() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    // Unit square touching the axis: full revolve → cylinder r=1 h=1.
    let (doc, prof) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, axis) = y_axis(doc, 0.0);
    let (doc, rev) = insert(
        doc,
        Node::Revolve {
            profile: prof,
            axis,
            angle: ang(TAU), // decided coincident with τ ⇒ Full
        },
    );
    let ev = run(&doc);
    let ValuePayload::Body(body) = &ev.value(rev).expect("revolve evaluated").payload else {
        panic!("expected body");
    };
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    let vol = mass_properties(body, Tol::witness()).unwrap().volume;
    assert!((vol - PI).abs() < 1e-9, "cylinder volume π, got {vol}");
}

#[test]
fn revolve_axis_out_of_plane_is_a_typed_refusal() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, prof) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)]],
        )),
    );
    let (doc, axis) = y_axis(doc, 1.0); // origin OFF the sketch plane
    let (doc, rev) = insert(
        doc,
        Node::Revolve {
            profile: prof,
            axis,
            angle: ang(PI),
        },
    );
    let ev = run(&doc);
    match ev.nodes.get(&rev) {
        Some(NodeResult::Failed(e)) => {
            assert!(matches!(
                e.kind,
                NodeErrorKind::AxisNotInSketchPlane { axis: a } if a == axis
            ));
        }
        other => panic!("expected Failed, got {other:?}"),
    }
}

#[test]
fn rotational_transform_wires_and_preserves_volume() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 2.0, 0.0);
    let (doc, moved) = insert(
        doc,
        Node::Transform {
            input: cube,
            translation: [len(0.25), len(-1.5), len(3.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(FRAC_PI_2),
        },
    );
    let ev = run(&doc);
    let ValuePayload::Body(body) = &ev.value(moved).unwrap().payload else {
        panic!("expected body");
    };
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    let m = mass_properties(body, Tol::witness()).unwrap();
    assert!((m.volume - 1.0).abs() < 1e-12);
    assert!((m.surface_area - 6.0).abs() < 1e-12);
}

#[test]
fn linear_pattern_evaluates_instances_as_data() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 0.0, 0.0);
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: cube,
            count: editor_core::Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(2.0), scl(0.0), scl(0.0)], // normalized by eval
                spacing: len(2.0),
            },
        },
    );
    let ev = run(&doc);
    let ValuePayload::Instances(instances) = &ev.value(pat).unwrap().payload else {
        panic!("expected instances");
    };
    assert_eq!(instances.len(), 3);
    for (i, inst) in instances.iter().enumerate() {
        assert_eq!(validate(inst), Ok(()));
        assert_eq!(mass_properties(inst, Tol::witness()).unwrap().volume, 1.0); // dyadic exact
        // Instance i sits at x offset 2i exactly (translation-only).
        let min_x = inst
            .points()
            .map(|(_, p)| p.x)
            .fold(f64::INFINITY, f64::min);
        assert_eq!(min_x, 2.0 * i as f64);
    }
    // Patterns do NOT implicitly union: consuming one as a boolean
    // operand is a typed refusal (part selection is PR 3 naming).
    let (doc2, other) = unit_cube(doc.clone(), 10.0, 10.0);
    let (doc2, boolean) = insert(
        doc2,
        Node::Boolean {
            op: BooleanOp::Union,
            a: pat,
            b: other,
            declare: None,
        },
    );
    let ev2 = run(&doc2);
    match ev2.nodes.get(&boolean) {
        Some(NodeResult::Failed(e)) => assert!(matches!(
            e.kind,
            NodeErrorKind::WrongOperand {
                expected: "body",
                found: "instances",
                ..
            }
        )),
        other => panic!("expected Failed, got {other:?}"),
    }
}

#[test]
fn circular_pattern_rotates_about_the_datum_axis() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 2.0, 0.0);
    // The z axis through the origin, as a datum.
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: cube,
            count: editor_core::Expr::count(4),
            kind: PatternKind::Circular {
                axis,
                step: ang(FRAC_PI_2),
            },
        },
    );
    let ev = run(&doc);
    let ValuePayload::Instances(instances) = &ev.value(pat).unwrap().payload else {
        panic!("expected instances");
    };
    assert_eq!(instances.len(), 4);
    for inst in instances {
        assert_eq!(validate(inst), Ok(()));
        assert!((mass_properties(inst, Tol::witness()).unwrap().volume - 1.0).abs() < 1e-12);
    }
}

#[test]
fn typed_refusal_doors() {
    // Degenerate datum normal.
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.0)],
            normal: [scl(0.0), scl(0.0), scl(0.0)],
        }),
    );
    let ev = run(&doc);
    match ev.nodes.get(&plane) {
        Some(NodeResult::Failed(e)) => assert!(matches!(
            e.kind,
            NodeErrorKind::DegenerateDirection {
                role: "datum plane normal"
            }
        )),
        other => panic!("expected Failed, got {other:?}"),
    }

    // Non-positive pattern count.
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 0.0, 0.0);
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: cube,
            count: editor_core::Expr::count(0),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(1.0),
            },
        },
    );
    let ev = run(&doc);
    match ev.nodes.get(&pat) {
        Some(NodeResult::Failed(e)) => assert!(matches!(
            e.kind,
            NodeErrorKind::NonPositiveCount { count: 0 }
        )),
        other => panic!("expected Failed, got {other:?}"),
    }

    // A Split value as a boolean operand (needs PR 3 naming).
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split_node) = insert(doc, Node::Split { target: cube, tool });
    let (doc, second) = unit_cube(doc, 5.0, 5.0);
    let (doc, boolean) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Intersect,
            a: split_node,
            b: second,
            declare: None,
        },
    );
    let ev = run(&doc);
    match ev.nodes.get(&boolean) {
        Some(NodeResult::Failed(e)) => assert!(matches!(
            e.kind,
            NodeErrorKind::WrongOperand {
                expected: "body",
                found: "split",
                ..
            }
        )),
        other => panic!("expected Failed, got {other:?}"),
    }
}

#[test]
fn declare_passes_through_and_boolean_accepts_it() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, a) = unit_cube(doc, 0.0, 0.0);
    let (doc, b) = unit_cube(doc, 0.5, 0.0); // overlapping, flush y/z planes
    // M4 PR 5: the flush contacts are DECLARED by name — this test's
    // Declare is now the LIVE lane, not pass-through data.
    let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b);
    let (doc, boolean) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    match &ev.value(decl).expect("declare is a value").payload {
        ValuePayload::Declarations(pairs) => assert_eq!(pairs.len(), 4),
        other => panic!("expected declarations, got {}", other.kind_name()),
    }
    let ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) =
        &ev.value(boolean).expect("boolean evaluated").payload
    else {
        panic!("expected boolean body");
    };
    assert_eq!(mass_properties(body, Tol::witness()).unwrap().volume, 1.5); // dyadic union

    // A non-Declare node on the declare edge: typed refusal.
    let (doc2, bad) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(a),
        },
    );
    let ev2 = run(&doc2);
    match ev2.nodes.get(&bad) {
        Some(NodeResult::Failed(e)) => assert!(matches!(
            e.kind,
            NodeErrorKind::WrongOperand {
                expected: "declarations",
                ..
            }
        )),
        other => panic!("expected Failed, got {other:?}"),
    }
}

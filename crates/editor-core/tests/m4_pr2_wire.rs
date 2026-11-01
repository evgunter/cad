//! M4 PR 2 wiring tests (spec D3): the revolve axis's decided in-plane
//! projection, rotational placements, families as data (D10
//! `[repetition]`), the placement axis's direction door, and the typed
//! operand refusal doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use editor_core::{
    CancelToken, EvalOptions, Evaluation, Node, NodeErrorKind, ProfileDoc, TargetAxis,
    ValuePayload, evaluate,
};
use fixture::{ang, insert, len, on_frame};
use geom_core::Tol;
use topo::{validate, validate_closed};

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
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (x0, y0),
            (x0 + 1.0, y0),
            (x0 + 1.0, y0 + 1.0),
            (x0, y0 + 1.0),
        ]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    )
}

#[test]
fn revolve_wires_its_axis_through_the_sketch_plane() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    // Unit square touching the axis: full revolve → cylinder r=1 h=1.
    let (doc, prof) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    // The sketch's own +y through its origin — the same line the
    // world-space `y_axis` named, in the coordinates the revolve reads.
    // Decided coincident with τ ⇒ Full.
    let (doc, rev) = insert(
        doc,
        fixture::revolve_about(prof, (0.0, 0.0), crate::fixture::along_y(), ang(TAU)),
    );
    let ev = run(&doc);
    let ValuePayload::Body(body) = &ev.value(rev).expect("revolve evaluated").payload else {
        panic!("expected body");
    };
    assert_eq!(validate(body.body()), Ok(()));
    assert_eq!(validate_closed(body.body()), Ok(()));
    let vol = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(body.body()),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    assert!((vol - PI).abs() < 1e-9, "cylinder volume π, got {vol}");
}

#[test]
fn rotational_placement_wires_and_preserves_volume() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 2.0, 0.0);
    let (doc, moved) = fixture::xform(doc, cube, [0.25, -1.5, 3.0], [0.0, 0.0, 1.0], FRAC_PI_2);
    let ev = run(&doc);
    let ValuePayload::Body(body) = &ev.value(moved).unwrap().payload else {
        panic!("expected body");
    };
    assert_eq!(validate(body.body()), Ok(()));
    assert_eq!(validate_closed(body.body()), Ok(()));
    let m = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(body.body()),
            Tol::witness(),
        ),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    assert!((m.enclosure.volume - 1.0).abs() < 1e-12);
    assert!((m.enclosure.surface_area - 6.0).abs() < 1e-12);
}

/// The measured volume of `body`.
fn volume(body: &topo::Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), Tol::witness()),
        Tol::witness(),
    )
    .volume
}

/// **A row is its members as data**: one placement over `k =
/// index(3)` slid `scalar(k)·2` along x is three bodies, member `k` at
/// `x = 2k` exactly, each the cube; and a union reads the family
/// spelled at one member seat as its every member (D10 `[repetition]`).
#[test]
fn a_row_evaluates_its_members_as_data() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 0.0, 0.0);
    let (doc, _, row) = fixture::linear(doc, cube, [1.0, 0.0, 0.0], 2.0, 3);
    let ev = run(&doc);
    let ValuePayload::Instances(members) = &ev.value(row).unwrap().payload else {
        panic!("expected a family of bodies");
    };
    assert_eq!(members.len(), 3);
    for (i, member) in members.iter().enumerate() {
        assert_eq!(validate(member.body()), Ok(()));
        assert_eq!(volume(member.body()), 1.0); // dyadic exact
        // Member i sits at x offset 2i exactly (translation-only).
        let min_x = crate::fixture::in_space(member)
            .points()
            .map(|(_, p)| p.x)
            .fold(f64::INFINITY, f64::min);
        assert_eq!(min_x, 2.0 * i as f64);
    }
    // A family spelled at one member seat is its every member.
    let (doc, far) = unit_cube(doc, 10.0, 10.0);
    let (doc, other) = fixture::stood_in(doc, far, cube);
    let (doc, joined) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![row.into(), other.into()]),
        },
    );
    let ev = run(&doc);
    let ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) = &ev
        .value(joined)
        .unwrap_or_else(|| panic!("{:?}", ev.node_error(joined)))
        .payload
    else {
        panic!("the union is a body");
    };
    // The three members and the far cube, apart: four solids.
    assert_eq!(body.body().solids().count(), 4);
}

/// **A ring turns its members about an axis read off the body**: an
/// `Axis` mate onto the z axis through the origin, spun
/// `scalar(k)·quarter`, is four bodies, each the cube.
#[test]
fn a_ring_rotates_about_an_axis_read_off_the_body() {
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 2.0, 0.0);
    // The z axis through the origin, read off the cube.
    let axis = fixture::axis_on(&doc, cube, [0.0; 3], [0.0, 0.0, 1.0]);
    let (doc, _, ring) = fixture::circular(doc, cube, (axis, TargetAxis::Z), FRAC_PI_2, 4);
    let ev = run(&doc);
    let ValuePayload::Instances(members) = &ev
        .value(ring)
        .unwrap_or_else(|| panic!("{:?}", ev.node_error(ring)))
        .payload
    else {
        panic!("expected a family of bodies");
    };
    assert_eq!(members.len(), 4);
    for member in members {
        assert_eq!(validate(member.body()), Ok(()));
        assert!((volume(member.body()) - 1.0).abs() < 1e-12);
    }
}

#[test]
fn typed_refusal_doors() {
    // A split named alone as a boolean operand is either of its two
    // halves: the door refuses it, and the read names a port.
    let doc = ProfileDoc::empty_derived("m4_pr2_wire", Tol::witness());
    let (doc, cube) = unit_cube(doc, 0.0, 0.0);
    let tool = fixture::plane_on(&doc, cube, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split_node) = insert(
        doc,
        Node::Split {
            target: cube.into(),
            tool,
        },
    );
    let (doc, second) = unit_cube(doc, 5.0, 5.0);
    let refusal = crate::fixture::insert_refused(
        &doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![split_node.into(), second.into()]),
        },
    );
    assert!(
        matches!(
            &refusal,
            editor_core::EditError::AmbiguousOutput { input, .. } if input.id() == split_node
        ),
        "{refusal:?}"
    );
}

/// **A placement frame's rotation axis, same fact, its own door** —
/// and its own test, because two asserts in one function only ever
/// surface the first failure.
///
/// The two directions this layer owns are wired separately and one
/// reaching the door proves nothing about the other. The refusal a
/// user reads must be about the axis they wrote, not about a matrix
/// property derived from it.
#[test]
fn non_finite_placement_axis_refuses_at_the_direction_door() {
    let refusal = editor_core::Frame::rotate_then_translate(
        [0.0, 1e200, 0.0],
        FRAC_PI_2,
        [0.0; 3],
        fixture::band(),
    )
    .expect_err("an axis of no finite length refuses");
    assert!(
        matches!(
            refusal.kind(),
            NodeErrorKind::NonFiniteDirection {
                role: "placement rotation axis"
            }
        ),
        "expected the direction door's refusal, got {:?}",
        refusal.kind()
    );
    let said = refusal.kind().to_string();
    assert!(
        said.starts_with("the placement rotation axis has no finite length"),
        "the refusal names the axis once: {said}"
    );
}

/// **The mapping itself: every arm of the kernel's typed refusal, and
/// the role word it carries**, through a placement frame's rotation
/// axis.
///
/// The length decision is one body in `topo::query` and this layer's
/// door is a call to it under its own funnel name, so what this layer
/// still owns is exactly two things — WHICH `NodeErrorKind` each
/// kernel refusal becomes, and WHICH vector the sentence names. A
/// mapping that sent two kernel arms to one `NodeErrorKind`, or that
/// dropped the role, would leave a one-arm row green.
///
/// So all four arms are walked here in one place, and the role word
/// travels with each: a length that is not a finite number, a length
/// that underflowed out of the format, a length decided to zero, and a
/// length that lands in the ambiguity band. The last needs a length
/// strictly inside (ε, K·ε), built from the run's OWN ε and K (their
/// geometric mean) so that the row means the same thing at every ε the
/// matrix runs.
#[test]
fn the_kernel_refusal_maps_onto_every_arm_of_this_layers_door() {
    let eps = Tol::witness().eps();
    let in_band = eps * Tol::witness().k().sqrt();
    for (component, expected) in [
        (1e200, "non-finite"),
        (1e-180, "underflowed"),
        (0.0, "degenerate"),
        (in_band, "escalated"),
    ] {
        let refusal = editor_core::Frame::rotate_then_translate(
            [component, 0.0, 0.0],
            FRAC_PI_2,
            [0.0; 3],
            fixture::band(),
        )
        .expect_err("a refused axis");
        match (refusal.kind(), expected) {
            (NodeErrorKind::NonFiniteDirection { role }, "non-finite")
            | (NodeErrorKind::UnderflowedDirection { role }, "underflowed")
            | (NodeErrorKind::DegenerateDirection { role }, "degenerate") => {
                assert_eq!(*role, "placement rotation axis", "the role word travels");
            }
            (NodeErrorKind::PlacementAxisEscalated { predicate, source }, "escalated") => {
                assert_eq!(
                    *predicate, "eval_direction_norm",
                    "an in-band length escalates naming THIS layer's funnel site"
                );
                // The name the FUNNEL recorded, which is a different
                // fact: the escalation carries what `decide` was
                // actually called with.
                assert_eq!(
                    source.predicate,
                    Some("eval_direction_norm"),
                    "the kernel decided this length under the site it was PASSED"
                );
            }
            (other, _) => panic!("the {expected} length mapped to {other:?}"),
        }
    }
}

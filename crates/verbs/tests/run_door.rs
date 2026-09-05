//! **The dispatch adds nothing**: the run doors and the op doors they
//! call produce the same body, the same birth record and the same
//! refusal.
//!
//! The point of the crate is that a caller can address an operation by
//! its NAME and get exactly what the door gives. These rows are what
//! makes that checkable: they run both paths on the same fixture and
//! compare bit-for-bit, so a dispatch that reordered arguments, dropped
//! a parameter or re-derived a band would red here rather than showing
//! up as drifted geometry three layers up. The arity rows at the end
//! are the run doors' one own decision — refusing the operand count a
//! verb does not declare — exercised in both directions so neither
//! mismatch arm is untested code.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use geom_core::{Affine3, Vec3};
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::{Extrusion, Revolution};
use topo::{Body, BooleanDeclarations, BooleanOp, BooleanResult, SweepStrategy, boolean_op_with};
use verbs::{Arity, PairOut, Verb, VerbError, VerbKind, VerbRecord};

use crate::fixture::{disc, offset_disc, tol, x_axis};

/// Every vertex point's BITS, plus the entity census — enough that a
/// carve differing anywhere in position or structure differs here.
fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
        let _ = writeln!(
            s,
            "{k:?} {:016x} {:016x} {:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        );
    }
    s
}

/// The blend channel of a run's record, or a loud failure.
fn blend_record(record: VerbRecord<f64>) -> Option<sweep::blend::naming::BlendNaming> {
    let VerbRecord::Blend(naming) = record else {
        panic!("a blend run produced another family's record: {record:?}");
    };
    naming
}

#[test]
fn the_fillet_dispatch_is_the_fillet_door() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = fillet_edges(&cube, &edges, 0.1, tol()).unwrap();
    let via = Verb::Fillet {
        edges: edges.clone(),
        radius: 0.1,
    }
    .run(&cube, tol())
    .unwrap();

    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{:?}", blend_record(via.record)),
        "the birth record is carried across, not rebuilt"
    );
}

#[test]
fn the_chamfer_dispatch_is_the_chamfer_door() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = chamfer_edges(&cube, &edges, 0.1, tol()).unwrap();
    let via = Verb::Chamfer {
        edges: edges.clone(),
        distance: 0.1,
    }
    .run(&cube, tol())
    .unwrap();

    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{:?}", blend_record(via.record))
    );
}

/// **A refusal crosses unaltered**, verb label included — the whole
/// reason `VerbError` wraps rather than re-classifies. The fixture is
/// the chamfer door's nonpositive-setback gate, which is the one
/// refusal reachable without building a degenerate body.
#[test]
fn a_refusal_crosses_the_dispatch_unaltered() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = chamfer_edges(&cube, &edges, 0.0, tol()).unwrap_err();
    let via = Verb::Chamfer {
        edges,
        distance: 0.0,
    }
    .run(&cube, tol())
    .unwrap_err();

    let VerbError::Blend(carried) = via else {
        panic!("a blend refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

/// A unit cube translated by `d` — the boolean rows' second operand.
fn shifted_cube(d: Vec3<f64>) -> Body<f64> {
    let cube = sweep::test_support::cube(1.0, tol());
    let map = Affine3::translation(d);
    topo::transform_rigid(&cube, &map, tol()).expect("a translation is rigid")
}

#[test]
fn the_boolean_dispatch_is_the_boolean_door() {
    let a = sweep::test_support::cube(1.0, tol());
    // A proper crossing: overlap in every axis, no face-on-face rest.
    let b = shifted_cube(Vec3::new(0.5, 0.5, 0.5));

    let decls = BooleanDeclarations::none();
    let door = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &decls,
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap();
    let via = Verb::Boolean {
        op: BooleanOp::Union,
        declare: BooleanDeclarations::none(),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap();

    let BooleanResult::Body(door) = door else {
        panic!("the fixture's union is a body");
    };
    let PairOut::Out(via) = via else {
        panic!("the dispatch's union is a body: {via:?}");
    };
    assert_eq!(dump(&door.body), dump(&via.body));
    let VerbRecord::Boolean {
        kind,
        contacts,
        naming,
    } = via.record
    else {
        panic!("a boolean run produced another family's record");
    };
    assert_eq!(door.kind, kind, "the result classification is carried");
    assert_eq!(
        door.contacts, contacts,
        "the surviving contacts are carried"
    );
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{naming:?}"),
        "the birth record is carried across, not rebuilt"
    );
}

/// **The typed empty success crosses as itself** (F8: ∅ is a value,
/// not an error) — disjoint operands intersected.
#[test]
fn an_empty_boolean_result_crosses_as_the_typed_empty() {
    let a = sweep::test_support::cube(1.0, tol());
    let b = shifted_cube(Vec3::new(3.0, 0.0, 0.0));

    let door = boolean_op_with(
        BooleanOp::Intersect,
        &a,
        &b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap();
    assert!(matches!(door, BooleanResult::Empty));

    let via = Verb::Boolean {
        op: BooleanOp::Intersect,
        declare: BooleanDeclarations::none(),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap();
    assert!(
        matches!(via, PairOut::Empty),
        "the dispatch turned the typed empty into {via:?}"
    );
}

/// **A boolean refusal crosses unaltered.** The fixture is two cubes
/// resting face on face with nothing declared — the undeclared-
/// coincidence refusal, reached identically both ways.
#[test]
fn a_boolean_refusal_crosses_the_dispatch_unaltered() {
    let a = sweep::test_support::cube(1.0, tol());
    let b = shifted_cube(Vec3::new(1.0, 0.0, 0.0));

    let door = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_err();
    let via = Verb::Boolean {
        op: BooleanOp::Union,
        declare: BooleanDeclarations::none(),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap_err();

    let VerbError::Boolean(carried) = via else {
        panic!("a boolean refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

/// One representative verb per vocabulary row, payload minimal.
fn sample(kind: VerbKind) -> Verb<f64> {
    match kind {
        VerbKind::Fillet => Verb::Fillet {
            edges: Vec::new(),
            radius: 0.1,
        },
        VerbKind::Chamfer => Verb::Chamfer {
            edges: Vec::new(),
            distance: 0.1,
        },
        VerbKind::Extrude => Verb::Extrude { distance: 1.0 },
        VerbKind::Revolve => Verb::Revolve {
            axis: x_axis(),
            revolution: Revolution::Full,
        },
        VerbKind::Boolean(op) => Verb::Boolean {
            op,
            declare: BooleanDeclarations::none(),
        },
    }
}

/// **Each door refuses exactly the verbs whose declared arity is not
/// its own**, over the whole vocabulary — the typed mismatch refusal
/// exercised in both directions, so neither door's cross-arity arms
/// are untested. The declared-arity door's behavior is the dispatch
/// rows above; what is pinned here is that the OTHER door answers
/// `Arity` with the right verb and the right count, and never runs the
/// op — proven by the returned variant itself: `Arity` is minted only
/// at the doors' own mismatch arms, before any op door is reached, so
/// its arrival IS the non-execution.
#[test]
fn each_door_refuses_the_undeclared_arity() {
    let a = sweep::test_support::cube(1.0, tol());
    let b = shifted_cube(Vec3::new(0.5, 0.5, 0.5));
    let disc = disc(0.5);

    for kind in VerbKind::ALL {
        let verb = sample(*kind);
        // Every door but this verb's own, so no shape is left untried:
        // the refusal must come back from each of them, naming the verb
        // and the door that refused.
        let refusals: Vec<(Arity, VerbError)> = [
            (Arity::One, verb.run(&a, tol()).err()),
            (
                Arity::Two,
                verb.run_pair(&a, &b, SweepStrategy::Realized, tol()).err(),
            ),
            (Arity::Profile, verb.run_profile(&disc, tol()).err()),
        ]
        .into_iter()
        .filter(|(door, _)| *door != kind.arity())
        .map(|(door, err)| {
            (
                door,
                err.unwrap_or_else(|| panic!("{kind:?} ran through the {door:?} door")),
            )
        })
        .collect();
        assert_eq!(refusals.len(), 2, "there are three doors, not three-plus");
        for (door, err) in refusals {
            let VerbError::Arity { verb: who, given } = err else {
                panic!("{kind:?} at the {door:?} door refused with {err:?}, not Arity");
            };
            assert_eq!(who, *kind);
            assert_eq!(given, door);
        }
    }
}

/// **The arity refusal's sentence, pinned.**
///
/// It is the one refusal string this door owns rather than forwards,
/// and it reads the verb's DECLARED operand out of `VerbKind::arity`
/// while naming the door it was handed to — so a shape added to
/// `Arity` changes what it says without any match to visit. That is
/// worth having and worth pinning: the sentence is what a direct
/// caller who picked the wrong door gets, and nothing else asserts a
/// word of it.
#[test]
fn the_arity_refusal_names_the_declared_operand_and_the_door() {
    let err = Verb::Fillet {
        edges: Vec::new(),
        radius: 0.1_f64,
    }
    .run_profile(&disc(0.5), tol())
    .expect_err("a fillet is not a profile verb");
    assert_eq!(
        err.to_string(),
        "the Fillet verb declares a One operand and was run through the Profile door"
    );

    let err = Verb::Extrude { distance: 1.0_f64 }
        .run(&sweep::test_support::cube(1.0, tol()), tol())
        .expect_err("an extrude is not a one-body verb");
    assert_eq!(
        err.to_string(),
        "the Extrude verb declares a Profile operand and was run through the One door"
    );
}

#[test]
fn the_extrude_dispatch_is_the_extrude_door() {
    let profile = disc(0.5);
    let door = sweep::extrude(&profile, Extrusion::Distance(1.0), tol()).unwrap();
    let via = Verb::Extrude { distance: 1.0 }
        .run_profile(&profile, tol())
        .unwrap();

    let VerbRecord::Extrude(via) = via else {
        panic!("an extrude run produced another family's record");
    };
    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.side_faces),
        format!("{:?}", via.side_faces),
        "the birth record is carried across, not rebuilt"
    );
    assert_eq!(format!("{:?}", door.top), format!("{:?}", via.top));
    assert_eq!(format!("{:?}", door.bottom), format!("{:?}", via.bottom));
    assert_eq!(
        format!("{:?}", door.strut_edges),
        format!("{:?}", via.strut_edges)
    );
    assert_eq!(format!("{:?}", door.solid), format!("{:?}", via.solid));
    assert_eq!(format!("{:?}", door.shell), format!("{:?}", via.shell));
}

#[test]
fn the_revolve_dispatch_is_the_revolve_door() {
    // Negative y: the door's half-plane about the +x axis is
    // `(p − origin).perp_dot(dir) ≥ 0`, which is `−y`.
    let profile = offset_disc(0.25, 1.0);
    let door = sweep::revolve(&profile, x_axis(), Revolution::Full, tol()).unwrap();
    let via = Verb::Revolve {
        axis: x_axis(),
        revolution: Revolution::Full,
    }
    .run_profile(&profile, tol())
    .unwrap();

    let VerbRecord::Revolve(via) = via else {
        panic!("a revolve run produced another family's record");
    };
    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.walls),
        format!("{:?}", via.walls),
        "the birth record is carried across, not rebuilt"
    );
    assert_eq!(format!("{:?}", door.rims), format!("{:?}", via.rims));
    assert_eq!(format!("{:?}", door.poles), format!("{:?}", via.poles));
    assert_eq!(format!("{:?}", door.kind), format!("{:?}", via.kind));
    assert_eq!(format!("{:?}", door.solid), format!("{:?}", via.solid));
    assert_eq!(format!("{:?}", door.shell), format!("{:?}", via.shell));
    assert_eq!(
        format!("{:?}", door.cavities),
        format!("{:?}", via.cavities)
    );
}

/// **A sweep refusal crosses unaltered**, both doors. The fixtures are
/// each door's own degenerate-extent gate — a zero distance and a zero
/// angle — which is the refusal reachable without building a
/// degenerate profile.
#[test]
fn a_sweep_refusal_crosses_the_dispatch_unaltered() {
    let profile = disc(0.5);
    let door = sweep::extrude(&profile, Extrusion::Distance(0.0), tol()).unwrap_err();
    let via = Verb::Extrude { distance: 0.0 }
        .run_profile(&profile, tol())
        .unwrap_err();
    let VerbError::Extrude(carried) = via else {
        panic!("an extrude refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());

    let off = offset_disc(0.25, 1.0);
    let door = sweep::revolve(&off, x_axis(), Revolution::Partial(0.0), tol()).unwrap_err();
    let via = Verb::Revolve {
        axis: x_axis(),
        revolution: Revolution::Partial(0.0),
    }
    .run_profile(&off, tol())
    .unwrap_err();
    let VerbError::Revolve(carried) = via else {
        panic!("a revolve refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

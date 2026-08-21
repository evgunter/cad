//! **The contact vocabulary at the boolean door** (CONTACT-DESIGN
//! C3/C4): the class is data the op ACTS on, the records replay
//! bit-identically, and a declaration is trusted exactly on its
//! bridged residue.
//!
//! The per-class verification tables and their ε rows live beside the
//! ladders they exercise (`boolean::carrier_eq`,
//! `boolean::contact_verify`); this file pins the behaviour a CALLER
//! can see.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use geom_core::Tol;
use topo::{
    Body, BooleanError, BooleanResult, ContactClass, FacePairDeclaration, mass_properties,
    union_with,
};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// A flush stack: two bricks meeting on z = 1, independently authored.
fn stacked() -> (Body<f64>, Body<f64>) {
    (
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        brick((0.5, 1.5), (0.25, 1.25), (1.0, 2.0)),
    )
}

/// A declared planar `Rest` still unions, and its RECORDS replay
/// bit-identically — the D9/C4 replay clause checked on the records
/// themselves rather than on naming standing in for them (the new
/// `PartialEq`). Records carry arena keys, and key identity is part of
/// what determinism means for a rerun of the same pipeline.
#[test]
fn declared_rest_unions_and_replays_records_bit_identically() {
    let (a, b) = stacked();
    let decls = flush_declarations(&a, &b);
    assert!(
        !decls.coincident_faces.is_empty(),
        "the stack's shared cap must be found"
    );
    assert!(
        decls
            .coincident_faces
            .iter()
            .all(|d| d.class == ContactClass::Rest),
        "a flush cap is the conformal class, spelled out"
    );

    let first = union_with(&a, &b, &decls, Tol::witness()).expect("declared flush union runs");
    let second = union_with(&a, &b, &decls, Tol::witness()).expect("the rerun runs");
    let (BooleanResult::Body(x), BooleanResult::Body(y)) = (first, second) else {
        panic!("an overlapping union cannot be Empty");
    };
    assert_eq!(
        x.contacts, y.contacts,
        "bit-identical replay must reproduce the RECORDS bit-identically (D9/C4)"
    );
    assert_eq!(
        mass_properties(&x.body, Tol::witness()).unwrap().volume,
        mass_properties(&y.body, Tol::witness()).unwrap().volume
    );
}

/// The undeclared kiss still refuses: value equality never glues, at
/// any ε, at any backend (C4's invariant).
#[test]
fn undeclared_kiss_still_refuses() {
    let (a, b) = stacked();
    let err = topo::union(&a, &b, Tol::witness()).expect_err("an undeclared kiss must refuse");
    assert!(
        matches!(
            err,
            BooleanError::UndeclaredCoincidence { .. } | BooleanError::Escalated { .. }
        ),
        "{err:?}"
    );
}

/// **The class is acted on, not carried and ignored** — the row a
/// class-ignoring implementation fails.
///
/// The pair below is a genuine flush cap, so a mutant that reads the
/// declaration and drops its class unions happily. The op instead
/// refuses typed: its classification stages act on `Rest`
/// declarations, and a class they cannot act on is refused at the
/// door rather than silently treated as the one they can.
#[test]
fn a_wrong_class_declaration_refuses_instead_of_being_ignored() {
    let (a, b) = stacked();
    let rest = flush_declarations(&a, &b);
    let mut tangent = rest.clone();
    for d in &mut tangent.coincident_faces {
        *d = FacePairDeclaration::new(d.a, d.b, ContactClass::Tangent);
    }
    assert!(
        union_with(&a, &b, &rest, Tol::witness()).is_ok(),
        "the same pair under the right class unions (the green half)"
    );
    let err = union_with(&a, &b, &tangent, Tol::witness())
        .expect_err("a class the stages cannot act on must refuse at the door");
    let msg = err.to_string();
    assert!(
        matches!(
            err,
            BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent
            }
        ),
        "{err:?}"
    );
    assert!(
        msg.contains("Tangent") && msg.contains("Rest"),
        "the refusal names both the class asked for and the class it acts on: {msg}"
    );
}

/// A declared `Rest` whose carriers are DEFINITELY distinct is
/// contradicted at use, and the message names the margin that decided
/// plus the two-arm recourse — not a bare "contradicted".
///
/// AQ6's steer rides here too: the counter-evidence is a plane OFFSET
/// (a separation), which is exactly the shape a designed clearance
/// has, so the recourse names `Fit { gap }` AND names its deferral
/// rather than pointing at a variant that does not exist.
#[test]
fn a_false_rest_is_contradicted_naming_the_margin_and_steering_to_fit() {
    // Full-face stacked plates: the mate is a pure REST contact, so
    // the declared-REST lane runs and verifies every declared pair.
    let a = brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
    let cap_of = |body: &Body<f64>, z: f64| -> topo::FaceKey {
        body.faces()
            .find(|(_, f)| match body.get_surface(f.surface) {
                Some(&topo::Surface::Plane { origin, normal, .. }) => {
                    normal.z.abs() > 0.5 && (origin.z - z).abs() < 1e-12
                }
                _ => false,
            })
            .map(|(k, _)| k)
            .unwrap()
    };
    // The genuine contact stays declared (otherwise the op refuses at
    // the undeclared-coincidence door and never reaches the lie), and
    // A's z = 0 floor is ALSO declared flush with B's z = 2 ceiling:
    // parallel, definitely offset by 2 m. The lane verifies every
    // declared pair, so the lie meets geometry and refuses.
    let mut decls = flush_declarations(&a, &b);
    decls
        .coincident_faces
        .push(FacePairDeclaration::rest(cap_of(&a, 0.0), cap_of(&b, 2.0)));
    let err = union_with(&a, &b, &decls, Tol::witness()).expect_err("a false Rest must refuse");
    let msg = err.to_string();
    match &err {
        BooleanError::ContactContradicted {
            declaration,
            margin,
            steer,
        } => {
            assert_eq!(declaration.class, ContactClass::Rest);
            assert_eq!(margin.predicate, Some("bool_plane_offset"));
            assert_eq!(*steer, Some(topo::FIT_DEFERRAL));
        }
        other => panic!("expected ContactContradicted, got {other:?}"),
    }
    assert!(msg.contains(topo::CONTACT_RECOURSE), "{msg}");
    assert!(
        !msg.contains("lower the tolerance"),
        "a contact refusal has no tolerance arm (SELECT §3d): {msg}"
    );
    assert!(msg.contains("Fit { gap }"), "{msg}");
    assert!(
        msg.contains("not yet built"),
        "the deferral is NAMED, never a dead pointer: {msg}"
    );
}

/// The declared-pair index answers "declared as WHAT", not merely
/// "declared" — the erasure the map replaced the set to remove.
#[test]
fn a_declaration_without_a_class_is_unrepresentable() {
    let (a, b) = stacked();
    let decls = flush_declarations(&a, &b);
    // Every constructor takes the class as an argument; there is no
    // class-less mint, so "I forgot the class" is a compile error and
    // never a silent `Rest`.
    let d = decls.coincident_faces[0];
    assert_eq!(FacePairDeclaration::rest(d.a, d.b), d);
    assert_ne!(
        FacePairDeclaration::new(d.a, d.b, ContactClass::Tangent),
        d,
        "two classes on one pair are two different declarations"
    );
}

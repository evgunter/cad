//! **The whole-shell `On` verdict** (`boolean::shell_witness`): an
//! uncut shell every witness of which lies on the other operand's
//! boundary is kept or dropped by the settled coincidence pairs between
//! its faces and one shell of the other operand, and refuses
//! `CoincidentShell` where those pairs do not certify it.
//!
//! The settled pairs here are rung 1's: every description is stamped
//! with a recipe source, as the recipe layer stamps a minted body, so
//! one stamped body twice is one carrier face for face. The declared
//! rung is `contained_flush_witness`'s.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::brick;
use geom_core::Tol;
use topo::{
    Body, BooleanError, BooleanResult, BooleanResultKind, GeomSource, Operand, ShellOrientation,
    intersect, mass_properties, subtract, union,
};

/// Stamps every surface of `b` with a distinct `minted(node, i)`, as
/// the recipe layer's `stamp_minted` does.
fn stamped(mut b: Body<f64>, node: u64) -> Body<f64> {
    let keys: Vec<_> = b.surfaces().map(|(k, _)| k).collect();
    for (i, k) in keys.into_iter().enumerate() {
        b.set_surface_source(k, GeomSource::minted(node, u32::try_from(i).unwrap()))
            .unwrap();
    }
    b
}

fn unit_block(node: u64) -> Body<f64> {
    let unit = (0.0, 1.0);
    stamped(brick(unit, unit, unit, Tol::witness()), node)
}

/// The body's volume, shell count and result kind, the body valid at
/// tier 3.
fn read(
    label: &str,
    result: Result<BooleanResult<f64>, BooleanError>,
) -> (f64, usize, BooleanResultKind) {
    let tol = Tol::witness();
    let BooleanResult::Body(out) = result.unwrap_or_else(|e| panic!("{label}: refused {e:?}"))
    else {
        panic!("{label}: empty, expected a body");
    };
    assert_eq!(
        topo::validate_geometric(&out.body, tol),
        Ok(()),
        "{label}: the result is tier-3 valid"
    );
    let volume = mass_properties(&out.body, tol).unwrap().volume;
    (volume, out.body.shells().count(), out.kind)
}

fn assert_empty(label: &str, result: Result<BooleanResult<f64>, BooleanError>) {
    match result {
        Ok(BooleanResult::Empty) => {}
        Ok(BooleanResult::Body(b)) => {
            panic!("{label}: expected the typed empty result, got {:?}", b.kind)
        }
        Err(e) => panic!("{label}: refused {e:?}"),
    }
}

/// One stamped body at both seats: `A ∪ A = A ∩ A = A` (A's copy, one
/// shell), `A − A` the typed empty result.
#[test]
fn one_body_twice_is_answered_under_every_op() {
    let tol = Tol::witness();
    let a = unit_block(7);
    for (label, result) in [
        ("A ∪ A", union(&a, &a, tol)),
        ("A ∩ A", intersect(&a, &a, tol)),
    ] {
        let (volume, shells, kind) = read(label, result);
        assert!((volume - 1.0).abs() < 1e-12, "{label}: volume {volume}");
        assert_eq!(shells, 1, "{label}: one copy of the shell is kept");
        assert_eq!(
            kind,
            BooleanResultKind::OperandA,
            "{label}: the copy kept is A's"
        );
    }
    assert_empty("A − A", subtract(&a, &a, tol));
}

/// `(X ∪ Z) op X` and `X op (X ∪ Z)`, Z disjoint from X: X's shell in
/// the union is X's own, so it is `On` X, while Z's shell reads Out by
/// its witnesses.
#[test]
fn a_union_member_carried_unchanged_lies_on_itself() {
    let tol = Tol::witness();
    let x = unit_block(7);
    let z = stamped(brick((3.0, 4.0), (0.0, 1.0), (0.0, 2.0), tol), 8);
    let BooleanResult::Body(xz) = union(&x, &z, tol).unwrap() else {
        panic!("X ∪ Z is not empty");
    };
    let xz = xz.body;
    // The volume and shell count each answer has; `None` is empty.
    let cases = [
        ("(X ∪ Z) ∪ X", union(&xz, &x, tol), Some((3.0, 2))),
        ("(X ∪ Z) ∩ X", intersect(&xz, &x, tol), Some((1.0, 1))),
        ("(X ∪ Z) − X", subtract(&xz, &x, tol), Some((2.0, 1))),
        ("X ∪ (X ∪ Z)", union(&x, &xz, tol), Some((3.0, 2))),
        ("X ∩ (X ∪ Z)", intersect(&x, &xz, tol), Some((1.0, 1))),
        ("X − (X ∪ Z)", subtract(&x, &xz, tol), None),
    ];
    for (label, result, want) in cases {
        match want {
            Some((v, n)) => {
                let (volume, shells, _) = read(label, result);
                assert!(
                    (volume - v).abs() < 1e-12,
                    "{label}: volume {volume}, want {v}"
                );
                assert_eq!(shells, n, "{label}: shell count");
            }
            None => assert_empty(label, result),
        }
    }
}

/// A lump filling a void: Y, and X with Y's void. Y's shell and the
/// void's are one surface with opposite orientations, so ∪ and ∩ drop
/// both and A − B keeps A's.
#[test]
fn a_lump_filling_a_void_lies_on_it_opposed() {
    let tol = Tol::witness();
    let x = stamped(brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), tol), 7);
    let y = stamped(brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol), 8);
    let BooleanResult::Body(hollow) = subtract(&x, &y, tol).unwrap() else {
        panic!("X − Y is not empty");
    };
    assert_eq!(
        hollow.kind,
        BooleanResultKind::Voided,
        "the fixture is X with Y's void"
    );
    let hollow = hollow.body;
    let (volume, shells, _) = read("Y ∪ (X − Y)", union(&y, &hollow, tol));
    assert!(
        (volume - 27.0).abs() < 1e-12,
        "Y ∪ (X − Y): volume {volume}"
    );
    assert_eq!(
        shells, 1,
        "Y ∪ (X − Y): the void is filled, so its shell and Y's are gone"
    );
    assert_empty("Y ∩ (X − Y)", intersect(&y, &hollow, tol));
    let (volume, shells, kind) = read("Y − (X − Y)", subtract(&y, &hollow, tol));
    assert!((volume - 1.0).abs() < 1e-12, "Y − (X − Y): volume {volume}");
    assert_eq!(
        (shells, kind),
        (1, BooleanResultKind::OperandA),
        "Y − (X − Y) keeps Y's shell"
    );
    let (volume, shells, _) = read("(X − Y) − Y", subtract(&hollow, &y, tol));
    assert!(
        (volume - 26.0).abs() < 1e-12,
        "(X − Y) − Y: volume {volume}"
    );
    assert_eq!(shells, 2, "(X − Y) − Y keeps the void's shell");
}

/// X, against X with a pocket sunk from its top face away from the
/// top face's witness points. Under ∪ no crossing is found, every
/// witness of X's shell lies on the pocketed body's boundary, and each
/// of X's faces is settled with that body's matching face, but the
/// pocket's faces pair with none of X's, so no shell covers X's back:
/// a per-face check would call X `On` and keep one copy, where this
/// refuses. Under ∩ and − the pocket's rim is a seam, and the answer
/// is the pocketed body and the pocket.
#[test]
fn a_shell_its_partner_does_not_cover_back_refuses() {
    let tol = Tol::witness();
    let x = unit_block(7);
    let pocket: Body<f64> = brick((0.05, 0.25), (0.75, 0.95), (0.5, 2.0), tol);
    let BooleanResult::Body(pocketed) = subtract(&x, &pocket, tol).unwrap() else {
        panic!("X − pocket is not empty");
    };
    let pocketed = pocketed.body;
    match union(&x, &pocketed, tol) {
        Err(BooleanError::CoincidentShell {
            operand: Operand::A,
            orientation: ShellOrientation::Same,
            ..
        }) => {}
        other => panic!("X ∪ pocketed: expected CoincidentShell on A's shell, got {other:?}"),
    }
    let rim = 0.2 * 0.2 * 0.5;
    for (label, result, want) in [
        ("X ∩ pocketed", intersect(&x, &pocketed, tol), 1.0 - rim),
        ("X − pocketed", subtract(&x, &pocketed, tol), rim),
    ] {
        let (volume, shells, kind) = read(label, result);
        assert!(
            (volume - want).abs() < 1e-12,
            "{label}: volume {volume}, want {want}"
        );
        assert_eq!((shells, kind), (1, BooleanResultKind::Seamed), "{label}");
    }
}

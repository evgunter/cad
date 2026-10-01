//! CERT-M2 R1 probe. Dumps the verdicts of `validate_pseudomanifold`,
//! `contact_marks` and `validate_geometric` at the certifying scalars,
//! and `mass_properties`, over [`crate::common::cert_corpus`]'s valid
//! and corrupt bodies at f64 / Dual64 / Interval, through both door
//! families. Run with `--nocapture`, grep `M2R1|`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::Tol;
use topo::{Body, ContactRecords};

use crate::common::cert_corpus::{corpus, f64_only_corpus};

/// The three passes at one scalar, as the door table [`dump`] walks:
/// the certified names ([`certified`]) or their `_structural` twins
/// ([`structural`]), whichever the scalar's bound can form. One dump
/// over a table rather than a dump per family, so the two families
/// print under one set of labels and the rows compare across scalars.
struct Doors<T: geom_core::Decide> {
    pseudomanifold: PseudomanifoldDoor<T>,
    marks: MarksDoor<T>,
    mass: MassDoor<T>,
}

type PseudomanifoldDoor<T> =
    fn(&Body<T>, &ContactRecords, Tol) -> Result<(), Vec<topo::ValidationError>>;
type MarksDoor<T> = fn(&Body<T>, Tol) -> Result<Vec<String>, Vec<topo::ValidationError>>;
type MassDoor<T> = fn(&Body<T>, Tol) -> Result<topo::MassProperties<T>, topo::MassPropsError>;

/// The marks pass's map, rendered in one stable order.
fn sorted_marks<K: core::fmt::Debug, V: core::fmt::Debug>(
    marks: impl IntoIterator<Item = (K, V)>,
) -> Vec<String> {
    let mut v: Vec<String> = marks
        .into_iter()
        .map(|(k, m)| format!("{k:?}={m:?}"))
        .collect();
    v.sort();
    v
}

/// The certified doors — every scalar whose bound names the right.
fn certified<T: geom_core::CertifiedBounds + topo::AtRestPolicy>() -> Doors<T> {
    Doors {
        pseudomanifold: topo::validate_pseudomanifold::<T>,
        marks: |body, tol| topo::contact_marks(body, tol).map(|m| sorted_marks(m.iter())),
        mass: topo::mass_properties::<T>,
    }
}

/// The `_structural` twins — the doors a scalar without certification
/// rights measures through.
fn structural<T: geom_core::Bounds + topo::AtRestPolicy>() -> Doors<T> {
    Doors {
        pseudomanifold: topo::validate_pseudomanifold_structural::<T>,
        marks: |body, tol| {
            topo::contact_marks_structural(body, tol).map(|m| sorted_marks(m.iter()))
        },
        mass: topo::mass_properties_structural::<T>,
    }
}

/// The three passes at one scalar through `doors`, under one set of
/// labels, so the rows compare across scalars and across the two door
/// families.
fn dump<T: geom_core::Decide + core::fmt::Debug>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
    doors: &Doors<T>,
) {
    let tol = Tol::witness();
    println!(
        "M2R1|{scalar}|{name}|pseudomanifold|{:?}",
        (doors.pseudomanifold)(body, &ContactRecords::default(), tol)
    );
    println!(
        "M2R1|{scalar}|{name}|contact_marks|{:?}",
        (doors.marks)(body, tol)
    );
    println!(
        "M2R1|{scalar}|{name}|mass_properties|{:?}",
        (doors.mass)(body, tol)
    );
}

fn dump_composed<T: geom_core::CertifiedBounds + core::fmt::Debug + topo::AtRestPolicy>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
) {
    println!(
        "M2R1|{scalar}|{name}|validate_geometric|{:?}",
        topo::validate_geometric(body, Tol::witness())
    );
}

#[test]
fn m2r1_passes_f64() {
    for (n, b) in corpus::<f64>() {
        dump("f64", &n, &b, &certified());
        dump_composed("f64", &n, &b);
    }
    for (n, b) in f64_only_corpus() {
        dump("f64", &n, &b, &certified());
        dump_composed("f64", &n, &b);
    }
}

#[test]
fn m2r1_passes_dual64() {
    for (n, b) in corpus::<geom_core::Dual64>() {
        dump("dual64", &n, &b, &structural());
    }
}

/// The declared straddle seat at `f64` through BOTH door families —
/// the one dump row whose `_structural` line carries a
/// `CensusLaneUnsupported`. The certified door examines the declared
/// pair and certifies the seat (`Ok`); the `_structural` door holds no
/// region door at any scalar, refuses the pair typed and leaves the two
/// crossings the declaration backs as `UndeclaredContact`.
#[test]
fn m2r1_declared_seat_f64() {
    let tol = Tol::witness();
    let seat = topo::test_support::straddle_seat(tol);
    let records = ContactRecords {
        patches: vec![topo::PatchContact {
            face_a: seat.post_top,
            face_b: seat.shelf_bottom,
        }],
        ..ContactRecords::default()
    };
    for (family, doors) in [
        ("certified", certified::<f64>()),
        ("structural", structural::<f64>()),
    ] {
        println!(
            "M2R1|f64|straddle_seat~declared|pseudomanifold~{family}|{:?}",
            (doors.pseudomanifold)(&seat.body, &records, tol)
        );
    }
}

/// **The `_structural` door at `Dual64` answers the certified door's
/// `f64` measurement on every closed-form body, and refuses exactly
/// where the lane would have enclosed** — the content of the `None`
/// path, body by body over the corpus, against a door that reaches it
/// by another route. A dual's value channel is bit-identical to the
/// `f64` build's (the dual contract), and a closed-form face computes
/// the same through either door, so on every body the closed form
/// covers the two agree to the bit with pads of `0` on both sides; on
/// the oblique-cut cylinders, whose ellipse-trimmed face needs the
/// quadrature, the certified door encloses (pads above `0`) and the
/// structural one refuses typed at the props layer. A `None` path that
/// decided anything the closed form does not, or a closed form that
/// drifted from the certified walk on a closed-form face, reds here.
#[test]
fn m2r1_structural_at_dual64_is_the_f64_closed_form_and_refuses_where_the_lane_would_enclose() {
    let tol = Tol::witness();
    let base = corpus::<f64>();
    let dual = corpus::<geom_core::Dual64>();
    assert_eq!(base.len(), dual.len(), "the corpus builds at both scalars");
    let mut refused = Vec::new();
    for ((name, b), (dname, d)) in base.iter().zip(&dual) {
        assert_eq!(name, dname);
        let certified = topo::mass_properties(b, tol)
            .unwrap_or_else(|e| panic!("{name}: the certified door refused a corpus body: {e:?}"));
        match topo::mass_properties_structural(d, tol) {
            Ok(s) => {
                assert_eq!(
                    (s.volume.value.to_bits(), s.surface_area.value.to_bits()),
                    (certified.volume.to_bits(), certified.surface_area.to_bits()),
                    "{name}: the dual's value channel is the f64 certified measurement"
                );
                assert_eq!(
                    (
                        s.volume_pad.to_bits(),
                        s.area_pad.to_bits(),
                        certified.volume_pad.to_bits(),
                        certified.area_pad.to_bits()
                    ),
                    (0, 0, 0, 0),
                    "{name}: a closed-form body carries pads of 0 through both doors"
                );
            }
            Err(topo::MassPropsError::Face { .. }) => {
                assert!(
                    certified.volume_pad > 0.0,
                    "{name}: the structural door refused a face the certified door did not                      need the quadrature for"
                );
                refused.push(name.clone());
            }
            Err(other) => panic!("{name}: the structural door refused with {other:?}"),
        }
    }
    let mut expect = vec![
        "cut_cylinder_above".to_string(),
        "cut_cylinder_below".to_string(),
        "cut_cylinder_above~reverted".to_string(),
        "cut_cylinder_below~reverted".to_string(),
    ];
    refused.sort();
    expect.sort();
    assert_eq!(
        refused, expect,
        "exactly the ellipse-trimmed bodies separate the two measurement doors"
    );
}

#[test]
fn m2r1_passes_interval() {
    for (n, b) in corpus::<geom_core::Interval>() {
        dump("interval", &n, &b, &certified());
        dump_composed("interval", &n, &b);
    }
}

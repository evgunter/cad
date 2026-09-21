//! CERT-M2 R1 probe (both-tree compatible): the DL3 corpus's product
//! bodies through the two passes the PR calls byte-identical, at f64 and
//! Dual64 (and Interval), plus the composed door at f64.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::corpus;

use corpus::{documents, eval};
use editor_core::product_recorded;
use geom_core::{Decide, Tol};
use topo::{AtRestPolicy, Body};

/// The three passes at one scalar, as the door table [`dump`] walks:
/// the certified names ([`certified`]) or their `_structural` twins
/// ([`structural`]), whichever the scalar's bound can form. One dump
/// over a table rather than a dump per family, so the two families
/// print under one set of labels and the rows compare across scalars.
struct Doors<T: Decide> {
    pseudomanifold:
        fn(&Body<T>, &topo::ContactRecords, Tol) -> Result<(), Vec<topo::ValidationError>>,
    marks: fn(&Body<T>, Tol) -> Result<Vec<String>, Vec<topo::ValidationError>>,
    mass: fn(&Body<T>, Tol) -> Result<topo::MassProperties<T>, topo::MassPropsError>,
}

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
fn certified<T: Decide + geom_core::CertifiedBounds + AtRestPolicy>() -> Doors<T> {
    Doors {
        pseudomanifold: topo::validate_pseudomanifold::<T>,
        marks: |body, tol| topo::contact_marks(body, tol).map(|m| sorted_marks(m.iter())),
        mass: topo::mass_properties::<T>,
    }
}

/// The `_structural` twins — the doors a scalar without certification
/// rights measures through.
fn structural<T: Decide + AtRestPolicy + geom_core::Bounds>() -> Doors<T> {
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
fn dump<T: Decide + core::fmt::Debug>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
    contacts: &topo::ContactRecords,
    doors: &Doors<T>,
) {
    let tol = Tol::witness();
    println!(
        "M2R1C|{scalar}|{name}|pseudomanifold|{:?}",
        (doors.pseudomanifold)(body, contacts, tol)
    );
    println!(
        "M2R1C|{scalar}|{name}|contact_marks|{:?}",
        (doors.marks)(body, tol)
    );
    println!(
        "M2R1C|{scalar}|{name}|mass_properties|{:?}",
        (doors.mass)(body, tol)
    );
}

fn run<T: editor_core::EvalScalar + core::fmt::Debug>(
    scalar: &str,
    doors: &Doors<T>,
) -> Vec<(String, editor_core::Product<T>)> {
    let tol = Tol::witness();
    let mut out = Vec::new();
    for doc in documents() {
        let ev = eval::<T>(&doc.doc);
        match product_recorded(&doc.doc, &ev, tol) {
            Ok(p) => {
                dump(scalar, doc.name, &p.body, &p.contacts, doors);
                out.push((doc.name.to_string(), p));
            }
            Err(e) => println!("M2R1C|{scalar}|{}|gather|Err({e:?})", doc.name),
        }
    }
    out
}

#[test]
fn m2r1_corpus_f64() {
    for (n, p) in run::<f64>("f64", &certified()) {
        println!(
            "M2R1C|f64|{n}|validate_geometric|{:?}",
            topo::validate_geometric(&p.body, Tol::witness())
        );
    }
}

#[test]
fn m2r1_corpus_dual64() {
    let _ = run::<geom_core::Dual64>("dual64", &structural());
}

#[cfg(feature = "interval")]
#[test]
fn m2r1_corpus_interval() {
    for (n, p) in run::<geom_core::Interval>("interval", &certified()) {
        println!(
            "M2R1C|interval|{n}|validate_geometric|{:?}",
            topo::validate_geometric(&p.body, Tol::witness())
        );
    }
}

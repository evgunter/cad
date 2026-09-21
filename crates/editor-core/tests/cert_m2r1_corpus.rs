//! CERT-M2 R1 probe (both-tree compatible): the DL3 corpus's product
//! bodies through the two passes the PR calls byte-identical, at f64 and
//! Dual64 (and Interval), plus the composed door at f64.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::corpus;

use corpus::{documents, eval};
use editor_core::product_recorded;
use geom_core::{Decide, Tol};
use topo::{AtRestPolicy, Body};

/// The three passes, each scalar through the door its bound can name —
/// the certified doors here, the `_structural` twins in
/// [`dump_structural`] — under one set of labels, so the rows compare
/// across scalars.
fn dump<T: Decide + geom_core::CertifiedBounds + AtRestPolicy + core::fmt::Debug>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
    contacts: &topo::ContactRecords,
) {
    let tol = Tol::witness();
    println!(
        "M2R1C|{scalar}|{name}|pseudomanifold|{:?}",
        topo::validate_pseudomanifold(body, contacts, tol)
    );
    let marks = topo::contact_marks(body, tol).map(|m| {
        let mut v: Vec<String> = m.iter().map(|(k, m)| format!("{k:?}={m:?}")).collect();
        v.sort();
        v
    });
    println!("M2R1C|{scalar}|{name}|contact_marks|{marks:?}");
    println!(
        "M2R1C|{scalar}|{name}|mass_properties|{:?}",
        topo::mass_properties(body, tol)
    );
}

/// [`dump`] through the `_structural` twins — the doors a scalar
/// without certification rights measures through.
fn dump_structural<T: Decide + AtRestPolicy + geom_core::Bounds + core::fmt::Debug>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
    contacts: &topo::ContactRecords,
) {
    let tol = Tol::witness();
    println!(
        "M2R1C|{scalar}|{name}|pseudomanifold|{:?}",
        topo::validate_pseudomanifold_structural(body, contacts, tol)
    );
    let marks = topo::contact_marks_structural(body, tol).map(|m| {
        let mut v: Vec<String> = m.iter().map(|(k, m)| format!("{k:?}={m:?}")).collect();
        v.sort();
        v
    });
    println!("M2R1C|{scalar}|{name}|contact_marks|{marks:?}");
    println!(
        "M2R1C|{scalar}|{name}|mass_properties|{:?}",
        topo::mass_properties_structural(body, tol)
    );
}

fn run<T: editor_core::EvalScalar + core::fmt::Debug>(
    scalar: &str,
    dump: fn(&str, &str, &Body<T>, &topo::ContactRecords),
) -> Vec<(String, editor_core::Product<T>)> {
    let tol = Tol::witness();
    let mut out = Vec::new();
    for doc in documents() {
        let ev = eval::<T>(&doc.doc);
        match product_recorded(&doc.doc, &ev, tol) {
            Ok(p) => {
                dump(scalar, doc.name, &p.body, &p.contacts);
                out.push((doc.name.to_string(), p));
            }
            Err(e) => println!("M2R1C|{scalar}|{}|gather|Err({e:?})", doc.name),
        }
    }
    out
}

#[test]
fn m2r1_corpus_f64() {
    for (n, p) in run::<f64>("f64", dump) {
        println!(
            "M2R1C|f64|{n}|validate_geometric|{:?}",
            topo::validate_geometric(&p.body, Tol::witness())
        );
    }
}

#[test]
fn m2r1_corpus_dual64() {
    let _ = run::<geom_core::Dual64>("dual64", dump_structural);
}

#[cfg(feature = "interval")]
#[test]
fn m2r1_corpus_interval() {
    for (n, p) in run::<geom_core::Interval>("interval", dump) {
        println!(
            "M2R1C|interval|{n}|validate_geometric|{:?}",
            topo::validate_geometric(&p.body, Tol::witness())
        );
    }
}

//! CERT-M2 R1 probe (both-tree compatible): the DL3 corpus's product
//! bodies through the two passes the PR calls byte-identical, at f64 and
//! Dual64 (and Interval), plus the composed door at f64.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::corpus;

use corpus::{documents, eval};
use editor_core::product_recorded;
use geom_core::{Decide, Tol};
use topo::{AtRestPolicy, Body, PropsQuadLane};

fn dump<T: Decide + PropsQuadLane + AtRestPolicy + core::fmt::Debug>(
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

fn run<T: editor_core::EvalScalar + core::fmt::Debug>(
    scalar: &str,
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
    for (n, p) in run::<f64>("f64") {
        println!(
            "M2R1C|f64|{n}|validate_geometric|{:?}",
            topo::validate_geometric(&p.body, Tol::witness())
        );
    }
}

#[test]
fn m2r1_corpus_dual64() {
    let _ = run::<geom_core::Dual64>("dual64");
}

#[cfg(feature = "interval")]
#[test]
fn m2r1_corpus_interval() {
    for (n, p) in run::<geom_core::Interval>("interval") {
        println!(
            "M2R1C|interval|{n}|validate_geometric|{:?}",
            topo::validate_geometric(&p.body, Tol::witness())
        );
    }
}

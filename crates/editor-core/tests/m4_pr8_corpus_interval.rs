//! M4 PR 8a spec D1, Interval lane: every Band 4 corpus document
//! evaluates GREEN at the certified interval scalar too.
//!
//! The document layer is scalar-independent (stored exact `f64`s embed
//! through `Real::from_f64`), so a divergence here means a PREDICATE
//! decided differently under enclosure arithmetic — exactly the class
//! of latent thin-margin bug the corpus exists to catch. The mass
//! pins stay in the `f64` lane: an interval mass property is an
//! enclosure, and `==` against a dyadic oracle is the wrong question
//! to ask of it (the enclosure CONTAINING the oracle is checked by
//! `topo`'s own interval rows).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use editor_core::EvalOutcome;
use geom_core::Interval;
use topo::{validate, validate_closed};

use corpus::{body_of, documents, eval, failures};

#[test]
fn every_document_evaluates_green_at_interval() {
    for d in documents() {
        let ev = eval::<Interval>(&d.doc);
        let bad = failures(&ev);
        assert!(
            bad.is_empty(),
            "{}: {} node(s) did not evaluate green under Interval:\n{}",
            d.name,
            bad.len(),
            bad.join("\n")
        );
        assert_eq!(ev.outcome, EvalOutcome::Completed, "{}: outcome", d.name);
        assert_eq!(ev.order.len(), d.len(), "{}: evaluation order", d.name);
        if let Some(result) = d.result {
            let body = body_of(&ev, result);
            assert_eq!(validate(body), Ok(()), "{}: tier 1 (Interval)", d.name);
            assert_eq!(
                validate_closed(body),
                Ok(()),
                "{}: closed (Interval)",
                d.name
            );
        }
    }
}

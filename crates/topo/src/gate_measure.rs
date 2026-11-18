//! MEASUREMENT ONLY (embed/gate-measure, not for merge): one row per
//! at-rest gate ask to `$CAD_GATE_LOG/<pid>.log`.
#![allow(missing_docs, clippy::all, clippy::pedantic)]

use crate::clearance::{Embedding, EmbeddingReport, EngineRefusal as R};
use crate::{Body, FaceKey};
use std::cell::RefCell;
use std::io::Write as _;

thread_local! {
    /// The records the door that is finishing holds, and its name.
    pub static HELD: RefCell<Option<(&'static str, crate::ContactRecords)>> = const { RefCell::new(None) };
}

pub fn holding(site: &'static str, records: &crate::ContactRecords) {
    HELD.with(|h| *h.borrow_mut() = Some((site, records.clone())));
}

pub fn clear() {
    HELD.with(|h| *h.borrow_mut() = None);
}

pub fn log<T: geom_core::Real>(
    body: &Body<T>,
    faces: &[FaceKey],
    r: &EmbeddingReport,
    secs: std::time::Duration,
) {
    let Ok(dir) = std::env::var("CAD_GATE_LOG") else {
        return;
    };
    let args: Vec<String> = std::env::args().collect();
    let test = args
        .iter()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_default();
    let bin = args.first().cloned().unwrap_or_default();
    let named: Vec<FaceKey> = match &r.verdict {
        Embedding::Crosses(c) => vec![c.faces.0, c.faces.1],
        Embedding::Refused(
            R::Unsupported { face, .. }
            | R::CollapsedChart { face, .. }
            | R::TorusNotRing { face, .. },
        ) => vec![*face],
        Embedding::Refused(
            R::Uncertified { a, b, .. }
            | R::PoisonEnclosure { a, b }
            | R::BoxDependent { a, b, .. },
        ) => vec![*a, *b],
        _ => vec![],
    };
    let kinds: Vec<String> = named
        .iter()
        .map(|&f| {
            body.face_surface(f).map_or("?".to_owned(), |s| {
                format!("{:?}@{:?}", s.kind(), body.solid_of_face(f))
            })
        })
        .collect();
    let held = HELD.with(|h| {
        h.borrow().as_ref().map(|(site, recs)| {
            let pairs = recs.face_pairs(body);
            let in_solid: Vec<String> = recs
                .rows()
                .iter()
                .filter(|c| {
                    let [x, y] = c.cells();
                    let s = |cell| crate::census_cell_solid(body, cell);
                    s(x).is_some() && s(x) == s(y)
                })
                .map(|c| {
                    format!("{:?}", c.kind())
                        .split('(')
                        .next()
                        .unwrap_or("")
                        .to_owned()
                })
                .collect();
            let covers = matches!(named.as_slice(), [a, b] if pairs.holds(*a, *b));
            format!(
                "{site} records={} in_solid={:?} covers_crossing={covers}",
                recs.len(),
                in_solid
            )
        })
    });
    let rc = &r.receipt;
    let line = format!(
        "GATE\t{bin}\t{test}\t{}\t{}\t{}\t{}\t{}\t{:.4}\t{:?}\tKINDS {kinds:?} SOLIDS {} HELD {}\n",
        std::any::type_name::<T>(),
        faces.len(),
        rc.candidates,
        rc.splits,
        rc.discharged,
        secs.as_secs_f64(),
        r.verdict,
        body.solids().count(),
        held.unwrap_or_else(|| "none".to_owned()),
    );
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{dir}/{}.log", std::process::id()))
    {
        let _ = f.write_all(line.as_bytes());
    }
}

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
        "GATE\t{bin}\t{test}\t{}\t{}\t{}\t{}\t{}\t{:.4}\t{:?}\tKINDS {kinds:?} SOLIDS {} HELD {} SITE {}\n",
        std::any::type_name::<T>(),
        faces.len(),
        rc.candidates,
        rc.splits,
        rc.discharged,
        secs.as_secs_f64(),
        r.verdict,
        body.solids().count(),
        held.unwrap_or_else(|| "none".to_owned()),
        current_site(),
    );
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{dir}/{}.log", std::process::id()))
    {
        let _ = f.write_all(line.as_bytes());
    }
}

/// Item 1: `CAD_GATE_OFF` skips the ask (the suite without the gate).
pub fn off() -> bool {
    std::env::var_os("CAD_GATE_OFF").is_some()
}

fn write(line: &str) {
    let Ok(dir) = std::env::var("CAD_GATE_LOG") else {
        return;
    };
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{dir}/{}.log", std::process::id()))
    {
        let _ = f.write_all(line.as_bytes());
    }
}

fn test_name() -> String {
    std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .unwrap_or_default()
}

thread_local! {
    static SEEN: RefCell<std::collections::BTreeSet<(usize, usize, usize)>> = RefCell::new(Default::default());
}

/// Item 4: every face pair of a holding ask asked alone (`Side`
/// restricted to the two faces, their self-pairs excluded), against the
/// whole ask. `CAD_GATE_DECOMPOSE=<min leaves>`; each body signature once.
pub fn decompose<T: geom_core::Decide + geom_core::CertifiedBounds>(
    body: &Body<T>,
    faces: &[FaceKey],
    whole: &EmbeddingReport,
    whole_secs: f64,
    band: geom_core::Band,
) {
    use crate::clearance::{FacePairs, GATE_BUDGETS, Side, embedded};
    let Some(min) = std::env::var("CAD_GATE_DECOMPOSE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
    else {
        return;
    };
    let w = &whole.receipt;
    let leaves =
        |r: &crate::clearance::CellReceipt| r.discharged + r.violated + r.refused + r.within;
    if !matches!(whole.verdict, Embedding::Embedded) || leaves(w) < min {
        return;
    }
    let sig = (faces.len(), w.candidates, leaves(w));
    if !SEEN.with(|s| s.borrow_mut().insert(sig)) {
        return;
    }
    let (mut n, mut sum_leaves, mut sum_splits, mut sum_disc, mut secs) =
        (0usize, 0usize, 0usize, 0usize, 0f64);
    let mut differ: Vec<String> = Vec::new();
    let mut top: Vec<(usize, FaceKey, FaceKey)> = Vec::new();
    for (i, &a) in faces.iter().enumerate() {
        for &b in &faces[i..] {
            let mut ex: Vec<(FaceKey, FaceKey)> = Vec::new();
            let fs = if a == b {
                vec![a]
            } else {
                ex.push((a, a));
                ex.push((b, b));
                vec![a, b]
            };
            let side = Side {
                body,
                faces: fs,
                excluded: ex.into_iter().collect::<FacePairs>(),
            };
            let t = std::time::Instant::now();
            let r = embedded(&side, band, GATE_BUDGETS);
            let dt = t.elapsed().as_secs_f64();
            if r.receipt.candidates == 0 {
                continue;
            }
            n += 1;
            secs += dt;
            let l = leaves(&r.receipt);
            sum_leaves += l;
            sum_splits += r.receipt.splits;
            sum_disc += r.receipt.discharged;
            top.push((l, a, b));
            if !matches!(r.verdict, Embedding::Embedded) {
                differ.push(format!("{a:?}/{b:?}:{:?}", r.verdict));
            }
        }
    }
    top.sort_by(|x, y| y.0.cmp(&x.0));
    let kinds = |f: FaceKey| {
        body.face_surface(f)
            .map_or("?".to_owned(), |s| format!("{:?}", s.kind()))
    };
    let tops: Vec<String> = top
        .iter()
        .take(3)
        .map(|(l, a, b)| format!("{l}:{}/{}", kinds(*a), kinds(*b)))
        .collect();
    write(&format!(
        "DECOMP\t{}\tfaces={}\twhole_cand={}\tpairs_asked={n}\twhole_leaves={}\tsum_leaves={sum_leaves}\twhole_splits={}\tsum_splits={sum_splits}\twhole_disc={}\tsum_disc={sum_disc}\twhole_secs={whole_secs:.4}\tsum_secs={secs:.4}\tmax_pair={}\ttop={tops:?}\tdiffer={differ:?}\n",
        test_name(),
        faces.len(),
        w.candidates,
        leaves(w),
        w.splits,
        w.discharged,
        top.first().map_or(0, |t| t.0),
    ));
}

/// Item 6: the census over a Boolean result with the records its door holds.
pub fn census_line(records: usize, findings: usize, secs: f64) {
    write(&format!(
        "CENSUS\t{}\trecords={records}\tfindings={findings}\tsecs={secs:.4}\n",
        test_name()
    ));
}

thread_local! {
    static SITE: std::cell::Cell<&'static str> = const { std::cell::Cell::new("direct") };
    static OPERANDS: RefCell<Option<(std::collections::BTreeMap<u64, usize>, std::collections::BTreeMap<u64, usize>)>> = const { RefCell::new(None) };
}

/// The finish site an ask belongs to, until the guard drops.
pub struct SiteGuard(&'static str);
impl Drop for SiteGuard {
    fn drop(&mut self) {
        SITE.with(|s| s.set(self.0));
    }
}
#[must_use]
pub fn site(name: &'static str) -> SiteGuard {
    SiteGuard(SITE.with(|s| s.replace(name)))
}
pub fn current_site() -> &'static str {
    SITE.with(|s| s.get())
}

fn hash(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

/// (whole face, carrier and sense) fingerprints of `f`.
fn prints<T: geom_core::Real>(body: &Body<T>, f: FaceKey) -> Option<(u64, u64)> {
    let face = body.get_face(f)?;
    let carrier = format!("{:?}|{}", body.get_surface(face.surface)?, face.sense);
    let mut rims: Vec<String> = body
        .half_edges()
        .filter(|(_, he)| body.get_loop(he.parent_loop).map(|l| l.face) == Some(f))
        .map(|(_, he)| {
            let e = body.get_edge(he.edge);
            let curve = e
                .and_then(|e| body.get_curve_geom(e.curve))
                .map(|c| format!("{c:?}"))
                .unwrap_or_default();
            let at = body
                .get_vertex(he.start)
                .and_then(|v| body.get_point(v.point))
                .map(|p| format!("{p:?}"))
                .unwrap_or_default();
            format!("{curve}@{at}")
        })
        .collect();
    rims.sort();
    Some((
        hash(&format!("{carrier}#{}", rims.join(";"))),
        hash(&carrier),
    ))
}

/// The operands a deciding door was handed, fingerprinted by face.
pub fn operands<T: geom_core::Real>(bodies: &[&Body<T>]) {
    if std::env::var_os("CAD_GATE_CARRY").is_none() {
        return;
    }
    let (mut whole, mut carrier) = (
        std::collections::BTreeMap::new(),
        std::collections::BTreeMap::new(),
    );
    for (k, b) in bodies.iter().enumerate() {
        for (f, _) in b.faces() {
            if let Some((w, c)) = prints(b, f) {
                whole.insert(w, k);
                carrier.insert(c, k);
            }
        }
    }
    OPERANDS.with(|o| *o.borrow_mut() = Some((whole, carrier)));
}

/// Item 3: each candidate face pair of a holding ask at a deciding door,
/// asked alone (exact on holding asks, item 4), classed by where its two
/// faces come from: carried (both bit-identical to faces of one operand),
/// two operands, trimmed (both on an operand's carrier, one or both with
/// new loops), new (a face on no operand's carrier).
pub fn carry<T: geom_core::Decide + geom_core::CertifiedBounds>(
    body: &Body<T>,
    faces: &[FaceKey],
    whole: &EmbeddingReport,
    whole_secs: f64,
    band: geom_core::Band,
) {
    use crate::clearance::{FacePairs, GATE_BUDGETS, Side, embedded};
    if std::env::var_os("CAD_GATE_CARRY").is_none()
        || !matches!(whole.verdict, Embedding::Embedded)
        || !matches!(current_site(), "boolean" | "split" | "shell" | "blend")
    {
        return;
    }
    let Some((whole_fp, carrier_fp)) = OPERANDS.with(|o| o.borrow().clone()) else {
        return;
    };
    #[derive(Clone, Copy)]
    enum Of {
        Same(usize),
        Trim(usize),
        New,
    }
    let of = |f: FaceKey| match prints(body, f) {
        Some((w, _)) if whole_fp.contains_key(&w) => Of::Same(whole_fp[&w]),
        Some((_, c)) if carrier_fp.contains_key(&c) => Of::Trim(carrier_fp[&c]),
        _ => Of::New,
    };
    let leaves =
        |r: &crate::clearance::CellReceipt| r.discharged + r.violated + r.refused + r.within;
    let mut tally: std::collections::BTreeMap<&'static str, (usize, usize, f64)> =
        Default::default();
    for (i, &a) in faces.iter().enumerate() {
        for &b in &faces[i..] {
            let mut ex: Vec<(FaceKey, FaceKey)> = Vec::new();
            let fs = if a == b {
                vec![a]
            } else {
                ex.push((a, a));
                ex.push((b, b));
                vec![a, b]
            };
            let side = Side {
                body,
                faces: fs,
                excluded: ex.into_iter().collect::<FacePairs>(),
            };
            let t = std::time::Instant::now();
            let r = embedded(&side, band, GATE_BUDGETS);
            let dt = t.elapsed().as_secs_f64();
            if r.receipt.candidates == 0 {
                continue;
            }
            let class = match (of(a), of(b)) {
                (Of::New, _) | (_, Of::New) => "new",
                (Of::Same(x), Of::Same(y)) if x == y => "carried",
                (Of::Same(x) | Of::Trim(x), Of::Same(y) | Of::Trim(y)) if x != y => "two-operands",
                _ => "trimmed",
            };
            let e = tally.entry(class).or_default();
            e.0 += 1;
            e.1 += leaves(&r.receipt);
            e.2 += dt;
        }
    }
    let cells: Vec<String> = tally
        .iter()
        .map(|(k, (n, l, s))| format!("{k}:{n}:{l}:{s:.4}"))
        .collect();
    write(&format!(
        "CARRY\t{}\t{}\tfaces={}\twhole_secs={whole_secs:.4}\t{}\n",
        test_name(),
        current_site(),
        faces.len(),
        cells.join(" ")
    ));
}

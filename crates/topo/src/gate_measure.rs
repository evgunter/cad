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
    static DECLARED: std::cell::Cell<[usize; 6]> = const { std::cell::Cell::new([0; 6]) };
    static REFUSED_FIRST: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn holding(site: &'static str, records: &crate::ContactRecords) {
    HELD.with(|h| *h.borrow_mut() = Some((site, records.clone())));
}

pub fn clear() {
    HELD.with(|h| *h.borrow_mut() = None);
    DECLARED.with(|d| d.set([0; 6]));
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
    let line = line.replace(
        " SITE ",
        &format!(
            " REFUSED_FIRST {} SITE ",
            REFUSED_FIRST.with(|c| c.replace(false))
        ),
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

pub fn emit(line: &str) {
    write(&format!("{}\t{line}", test_name()));
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

/// A refusal was kept before the walk found its verified crossing.
pub fn refused_before_crossing() {
    REFUSED_FIRST.with(|c| c.set(true));
}

/// The declared operand records a Boolean door holds, by kind
/// (vv, vf, ve, ee, ef, ff), summed over both operands.
pub fn declared(records: [&crate::boolean::OperandRecords; 2]) {
    let mut k = [0usize; 6];
    for r in records {
        k[0] += r.vv.len();
        k[1] += r.vf.len();
        k[2] += r.ve.len();
        k[3] += r.ee.len();
        k[4] += r.ef.len();
        k[5] += r.ff.len();
    }
    DECLARED.with(|d| d.set(k));
}

// ---- The walk measurement (M1–M4): per cell-pair events of one ask. ----

#[derive(Default)]
struct WalkRec {
    events: Vec<(FaceKey, FaceKey, usize, f64, bool, u32, Option<String>)>,
    pending: Option<String>,
    open: Vec<(FaceKey, FaceKey)>,
    crossed: Option<(FaceKey, FaceKey)>,
    /// Per walk event (same index), the touching split it made: (reason
    /// code, same as its parent's).
    splits: std::collections::BTreeMap<usize, (u32, bool)>,
    pending_split: Option<(u32, bool)>,
    /// The crossing's (depth, fresh subtree root, touching split after
    /// the first refusal in its lineage).
    crossed_at: Option<(u32, bool, bool)>,
    /// Straddling distance splits by (faces share, nearest touching
    /// ancestor's reason, first touching ancestor's reason).
    straddle: std::collections::BTreeMap<(&'static str, String, String), (usize,)>,
    /// Per face pair, the least upper end of a straddling separation.
    min_hi: std::collections::BTreeMap<(FaceKey, FaceKey), f64>,
    pair_share: std::collections::BTreeMap<(FaceKey, FaceKey), &'static str>,
    /// Per face pair, the deepest straddling split: (depth, lo, hi).
    deepest: std::collections::BTreeMap<(FaceKey, FaceKey), (u32, f64, f64)>,
    /// Per walk event, its (first, nearest) touching ancestors.
    lineage: Vec<(u32, u32)>,
    /// Box traces: the last straddling split per (face pair, depth).
    boxes: std::collections::BTreeMap<((FaceKey, FaceKey), u32), String>,
}

thread_local! {
    static REASON: RefCell<Option<String>> = const { RefCell::new(None) };
    static LOCAL_SITE: RefCell<Option<String>> = const { RefCell::new(None) };
    static MARGIN: RefCell<Option<&'static str>> = const { RefCell::new(None) };
    static CODES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static CUR: std::cell::Cell<(u32, bool, bool)> = const { std::cell::Cell::new((0, true, false)) };
    static LINEAGE: std::cell::Cell<(u32, u32)> = const { std::cell::Cell::new((0, 0)) };
}

pub fn local_site(tag: &'static str, line: u32) {
    LOCAL_SITE.with(|s| *s.borrow_mut() = Some(format!("{tag}:{line}")));
}
pub fn margin_undecided(lo: f64, hi: f64, esc: f64) {
    let class = if lo > -esc && hi < esc {
        "margin-wholly-in-band"
    } else if lo < 0.0 && hi > 0.0 {
        "margin-straddles-zero"
    } else {
        "margin-straddles-band-edge"
    };
    MARGIN.with(|m| *m.borrow_mut() = Some(class));
}
pub fn clear_margin() {
    MARGIN.with(|m| *m.borrow_mut() = None);
}
pub fn clear_reason() {
    REASON.with(|r| *r.borrow_mut() = None);
    LOCAL_SITE.with(|s| *s.borrow_mut() = None);
    MARGIN.with(|m| *m.borrow_mut() = None);
}
pub fn set_reason(r: String) {
    let site = LOCAL_SITE.with(|s| s.borrow().clone());
    let r = if r.contains("Local") {
        format!("{r}@{}", site.unwrap_or_default())
    } else {
        r
    };
    REASON.with(|x| *x.borrow_mut() = Some(r));
}
pub fn set_reason_undecided() {
    let m = MARGIN
        .with(|m| *m.borrow())
        .unwrap_or("no-undecided-margin-read");
    REASON.with(|x| *x.borrow_mut() = Some(format!("both-presented:{m}")));
}
pub fn reason<O>(tag: &str, o: O) -> O {
    REASON.with(|x| *x.borrow_mut() = Some(tag.to_owned()));
    o
}
pub fn current_lineage(first: u32, near: u32) {
    LINEAGE.with(|l| l.set((first, near)));
}
pub fn current_task(depth: u32, fresh: bool, after: bool) {
    CUR.with(|c| c.set((depth, fresh, after)));
}
fn code_of(key: String) -> u32 {
    CODES.with(|c| {
        let mut c = c.borrow_mut();
        match c.iter().position(|k| *k == key) {
            Some(i) => i as u32 + 1,
            None => {
                c.push(key);
                c.len() as u32
            }
        }
    })
}
fn key_of(code: u32) -> String {
    CODES.with(|c| {
        c.borrow()
            .get(code as usize - 1)
            .cloned()
            .unwrap_or_default()
    })
}

/// A touching cell pair the local certificate answered `Split` for: its
/// reason interned (class, site, shared cell), returned for its children.
pub fn split_event(
    _pair: (FaceKey, FaceKey),
    _depth: u32,
    arm: crate::clearance::CertificateArm,
    shared: &crate::clearance::SharedCell,
    parent: u32,
    _sep: geom_core::interval::Interval,
) -> u32 {
    let why = match format!("{arm:?}").as_str() {
        "Monotone" => "monotone".to_owned(),
        "SecondOrder" => "second-order".to_owned(),
        _ => REASON
            .with(|r| r.borrow_mut().take())
            .unwrap_or_else(|| "departure-other".to_owned()),
    };
    let code = code_of(format!("{why}#{shared:?}"));
    WALK.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            w.pending_split = Some((code, parent == code));
        }
    });
    code
}

thread_local! {
    static WALK: RefCell<Option<WalkRec>> = const { RefCell::new(None) };
    static CONTINUING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn short(r: &R) -> String {
    match r {
        R::Uncertified { arm, shared, .. } => {
            let sh = format!("{shared:?}");
            format!("Uncertified/{arm:?}/{}", sh.split('(').next().unwrap_or(""))
        }
        R::Budget { stop: b, .. } => format!("{b:?}").split(' ').next().unwrap_or("").to_owned(),
        other => format!("{other:?}")
            .split([' ', '(', '{'])
            .next()
            .unwrap_or("")
            .to_owned(),
    }
}

/// `slot.get_or_insert(r)`, noting `r`'s kind for the walk record.
pub fn noted(slot: &mut Option<R>, r: R) -> &mut R {
    WALK.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            w.pending = Some(short(&r));
        }
    });
    slot.get_or_insert(r)
}

pub fn walk_event(pair: (FaceKey, FaceKey), splits: usize, secs: f64, refused: bool, depth: u32) {
    WALK.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            let kind = w.pending.take();
            w.lineage.push(LINEAGE.with(|l| l.get()));
            if let Some(sp) = w.pending_split.take() {
                let n = w.events.len();
                w.splits.insert(n, sp);
            }
            w.events.push((
                pair.0,
                pair.1,
                splits,
                secs,
                refused,
                depth,
                if refused { kind } else { None },
            ));
        }
    });
}

pub fn walk_overflow(open: Vec<(FaceKey, FaceKey)>) {
    WALK.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            w.open = open;
        }
    });
}

pub fn crossed_in(a: FaceKey, b: FaceKey) {
    WALK.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            w.crossed.get_or_insert((a, b));
            if w.crossed_at.is_none() {
                w.crossed_at = Some(CUR.with(|c| c.get()));
            }
        }
    });
}

pub fn continuing() -> bool {
    CONTINUING.with(|c| c.get())
}

/// Starts recording the ask about to run, where `CAD_GATE_WALK` is set.
pub fn walk_start() {
    CUR.with(|c| c.set((0, true, false)));
    if std::env::var_os("CAD_GATE_WALK").is_some() {
        WALK.with(|w| *w.borrow_mut() = Some(WalkRec::default()));
    }
}

/// Ends the record and writes its `WALK` row (refusing or crossing asks
/// only).
pub fn walk_finish(r: &EmbeddingReport, secs: f64) {
    let Some(rec) = WALK.with(|w| w.borrow_mut().take()) else {
        return;
    };
    if matches!(r.verdict, Embedding::Embedded) {
        return;
    }
    let key = |a: FaceKey, b: FaceKey| (a.min(b), a.max(b));
    let first = rec.events.iter().position(|e| e.4);
    let mut refusing: std::collections::BTreeMap<(FaceKey, FaceKey), (usize, Vec<String>)> =
        Default::default();
    for (n, e) in rec.events.iter().enumerate() {
        if e.4 {
            let ent = refusing.entry(key(e.0, e.1)).or_insert((n, Vec::new()));
            if let Some(k) = &e.6 {
                if !ent.1.contains(k) {
                    ent.1.push(k.clone());
                }
            }
        }
    }
    let (mut before_s, mut before_t) = (0usize, 0f64);
    let mut part = [(0usize, 0f64); 4]; // a, b, c-before-own, c-after-own
    let (mut depth0, mut splits_at0, mut kind0, mut pair0) = (0u32, 0usize, String::new(), None);
    if let Some(f) = first {
        let e0 = &rec.events[f];
        depth0 = e0.5;
        splits_at0 = rec.events[..f].iter().map(|e| e.2).sum();
        kind0 = e0.6.clone().unwrap_or_default();
        pair0 = Some(key(e0.0, e0.1));
    }
    for (n, e) in rec.events.iter().enumerate() {
        let p = key(e.0, e.1);
        match first {
            Some(f) if n > f => {
                let slot = if Some(p) == pair0 {
                    0
                } else {
                    match refusing.get(&p) {
                        None => 1,
                        Some((own, _)) if n < *own => 2,
                        Some(_) => 3,
                    }
                };
                part[slot].0 += e.2;
                part[slot].1 += e.3;
            }
            _ => {
                before_s += e.2;
                before_t += e.3;
            }
        }
    }
    let crossed = rec.crossed.map(|(a, b)| key(a, b));
    let cross_where = match (crossed, pair0) {
        (None, _) => "none",
        (Some(_), None) => "no-refusal",
        (Some(c), Some(p)) if c == p => "same-pair-as-first-refusal",
        (Some(c), _) if refusing.contains_key(&c) => "another-refusing-pair",
        _ => "a-pair-that-did-not-refuse",
    };
    // M5a: touching splits by reason class.
    let mut reasons: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
    for &(code, same) in rec.splits.values() {
        let k = key_of(code);
        let class = k.split('#').next().unwrap_or("").to_owned();
        let e = reasons.entry(class).or_default();
        e.0 += 1;
        e.1 += usize::from(same);
    }
    // M5b: per refusing face pair, the first depth where a split repeats
    // its parent's reason, reads a margin wholly in band, or holds a
    // Structural part; and what the pair spent below that depth.
    let strict = |code: u32| {
        let k = key_of(code);
        k.contains("wholly-in-band") || k.contains("Structural")
    };
    // Per refusing pair: (strict) the first depth with a split whose reason
    // no smaller cell changes; (repeat) the depth below which every split
    // repeats its parent's reason exactly (class, site, shared cell). And
    // what the pair spent below each.
    let (mut st_splits, mut st_secs, mut st_pairs) = (0usize, 0f64, 0usize);
    let (mut m5b_splits, mut m5b_secs, mut m5b_pairs) = (0usize, 0f64, 0usize);
    let mut m5b_depths: Vec<u32> = Vec::new();
    let mut st_depths: Vec<u32> = Vec::new();
    for p in refusing.keys() {
        let mine: Vec<(
            usize,
            &(FaceKey, FaceKey, usize, f64, bool, u32, Option<String>),
        )> = rec
            .events
            .iter()
            .enumerate()
            .filter(|(_, e)| key(e.0, e.1) == *p)
            .collect();
        let d_strict = mine
            .iter()
            .filter(|(n, _)| rec.splits.get(n).is_some_and(|&(c, _)| strict(c)))
            .map(|(_, e)| e.5)
            .min();
        if let Some(d) = d_strict {
            st_pairs += 1;
            st_depths.push(d);
            for (_, e) in mine.iter().filter(|(_, e)| e.5 > d) {
                st_splits += e.2;
                st_secs += e.3;
            }
        }
        let changed = mine
            .iter()
            .filter(|(n, _)| rec.splits.get(n).is_some_and(|&(_, same)| !same))
            .map(|(_, e)| e.5)
            .max();
        let d_rep = changed.map_or(0, |d| d + 1);
        let below: Vec<_> = mine.iter().filter(|(_, e)| e.5 > d_rep).collect();
        if !below.is_empty() {
            m5b_pairs += 1;
            m5b_depths.push(d_rep);
            for (_, e) in below {
                m5b_splits += e.2;
                m5b_secs += e.3;
            }
        }
    }
    st_depths.sort_unstable();
    m5b_depths.sort_unstable();
    let m5c = rec
        .crossed_at
        .map_or("none".to_owned(), |(d, fresh, after)| {
            format!("depth={d},fresh={fresh},touching_split_after_first_refusal_in_lineage={after}")
        });
    write(&format!(
        "M5\t{}\tverdict_crosses={}\tsplit_reasons={reasons:?}\tm5b_pairs={m5b_pairs}/{}\tm5b_depths={m5b_depths:?}\tm5b_after={m5b_splits}:{m5b_secs:.4}\tstrict_pairs={st_pairs}\tstrict_depths={st_depths:?}\tstrict_after={st_splits}:{st_secs:.4}\tsecs={secs:.4}\tm5c={m5c}\n",
        test_name(),
        matches!(r.verdict, Embedding::Crosses(_)),
        refusing.len(),
    ));
    {
        let mut sub: std::collections::BTreeMap<String, (usize, f64)> = Default::default();
        for (n, e) in rec.events.iter().enumerate() {
            if !refusing.contains_key(&key(e.0, e.1)) {
                continue;
            }
            let first = rec.lineage.get(n).map_or(0, |l| l.0);
            let class = if first == 0 {
                "no-touching-ancestor".to_owned()
            } else {
                key_of(first).split('#').next().unwrap_or("").to_owned()
            };
            let x = sub.entry(class).or_default();
            x.0 += e.2;
            x.1 += e.3;
        }
        let near: Vec<String> = rec
            .min_hi
            .iter()
            .filter(|(k, _)| rec.pair_share.get(*k) == Some(&"shares-nothing"))
            .map(|(_, h)| format!("{h:.3e}"))
            .collect();
        let deep: Vec<String> = rec
            .deepest
            .iter()
            .map(|(k, (d, lo, hi))| {
                format!(
                    "{}:{d}:{lo:.3e}:{hi:.3e}",
                    rec.pair_share.get(k).copied().unwrap_or("?")
                )
            })
            .collect();
        let st: Vec<String> = rec
            .straddle
            .iter()
            .map(|((a, b, c), (n,))| format!("{a}|{b}|{c}={n}"))
            .collect();
        write(&format!(
            "STRADDLE\t{}\tsecs={secs:.4}\tsplits={st:?}\tshare_nothing_min_hi={near:?}\trefusing_spend_by_first_touching_ancestor={sub:?}\tdeepest={deep:?}\n",
            test_name()
        ));
    }
    for ((p, d), line) in &rec.boxes {
        write(&format!("BOX\t{}\t{p:?}\tdepth={d}\t{line}\n", test_name()));
    }
    let kinds: Vec<String> = refusing.values().map(|(_, k)| k.join("+")).collect();
    let verdict = match &r.verdict {
        Embedding::Crosses(_) => "Crosses".to_owned(),
        Embedding::Refused(x) => short(x),
        Embedding::Embedded => unreachable!(),
    };
    write(&format!(
        "WALK\t{}\tverdict={verdict}\tsecs={secs:.4}\tsplits={}\tfirst_depth={depth0}\tsplits_at_first={splits_at0}\tfirst_kind={kind0}\tbefore={before_s}:{before_t:.4}\ta={}:{:.4}\tb={}:{:.4}\tc_pre={}:{:.4}\tc_post={}:{:.4}\td_open={}\trefusing_pairs={}\tkinds={kinds:?}\tcrossing={cross_where}\n",
        test_name(),
        r.receipt.splits,
        part[0].0,
        part[0].1,
        part[1].0,
        part[1].1,
        part[2].0,
        part[2].1,
        part[3].0,
        part[3].1,
        rec.open.len(),
        refusing.len(),
    ));
}

/// M4: the ask run again past its first crossing (verdict kept), its cost.
pub fn continue_cost<T: geom_core::Decide + geom_core::CertifiedBounds>(
    side: &crate::clearance::Side<'_, T>,
    band: geom_core::Band,
    stopped: &EmbeddingReport,
    stopped_secs: f64,
) {
    if std::env::var_os("CAD_GATE_M4").is_none()
        || !matches!(stopped.verdict, Embedding::Crosses(_))
    {
        return;
    }
    CONTINUING.with(|c| c.set(true));
    let t = std::time::Instant::now();
    let r = crate::clearance::embedded(side, band, crate::clearance::GATE_BUDGETS);
    let secs = t.elapsed().as_secs_f64();
    CONTINUING.with(|c| c.set(false));
    let v = match &r.verdict {
        Embedding::Crosses(_) => "Crosses".to_owned(),
        Embedding::Refused(x) => short(x),
        Embedding::Embedded => "Embedded".to_owned(),
    };
    write(&format!(
        "M4\t{}\tstopped_splits={}\tstopped_secs={stopped_secs:.4}\tcontinued_splits={}\tcontinued_secs={secs:.4}\tcontinued_verdict={v}\tviolated_cells={}\n",
        test_name(),
        stopped.receipt.splits,
        r.receipt.splits,
        r.receipt.violated
    ));
}

/// M5: a non-touching cell pair split because its distance is undecided:
/// classed by its separation's enclosure against the band.
pub fn distance_split(
    lo: f64,
    hi: f64,
    esc: f64,
    parent: u32,
    shares: u8,
    pair: (FaceKey, FaceKey),
) -> u32 {
    let class = if hi < esc {
        "distance:separation-wholly-in-band"
    } else if lo < esc {
        "distance:separation-straddles-band-edge"
    } else {
        "distance:separation-past-band-undecided"
    };
    let code = code_of(format!("{class}#distance"));
    let (tfirst, tnear) = LINEAGE.with(|l| l.get());
    WALK.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            w.pending_split = Some((code, parent == code));
            if lo < esc {
                let share =
                    ["shares-nothing", "shares-a-cell", "same-face"][usize::from(shares.min(2))];
                let anc = if tnear == 0 {
                    "no-touching-ancestor".to_owned()
                } else {
                    key_of(tnear).split('#').next().unwrap_or("").to_owned()
                };
                let first = if tfirst == 0 {
                    String::new()
                } else {
                    key_of(tfirst).split('#').next().unwrap_or("").to_owned()
                };
                let e = w.straddle.entry((share, anc, first)).or_default();
                e.0 += 1;
                let k = (pair.0.min(pair.1), pair.0.max(pair.1));
                let m = w.min_hi.entry(k).or_insert(f64::INFINITY);
                if hi < *m {
                    *m = hi;
                }
                let depth = CUR.with(|c| c.get().0);
                let dd = w.deepest.entry(k).or_insert((0, 0.0, 0.0));
                if depth >= dd.0 {
                    *dd = (depth, lo, hi);
                }
                w.pair_share.insert(k, share);
            }
        }
    });
    code
}

/// The box trace of a straddling split between faces sharing nothing:
/// the last per face pair and depth, written when the ask ends.
pub fn box_trace(pair: (FaceKey, FaceKey), depth: u32, line: String) {
    WALK.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            w.boxes.insert((pair, depth), line);
        }
    });
}

//! Probe (M16). `BOXFINISH_M16=<file>` appends one line per decision the
//! value channel did not sign, with what enclosing the margin's form
//! over the parameters' brackets would read. `BOXFINISH_M16_ON=1` makes
//! that reading the answer (probe tree only).
#![allow(clippy::unwrap_used, clippy::expect_used, missing_docs)]

use super::{SESSION, Session, SymId, SymOp, early_form, plain_form, signed};
use crate::interval::Interval;
use crate::predicate::{Band, Decide, Sign};
use crate::real::Real;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Mutex, OnceLock};

fn sink() -> Option<&'static Mutex<std::fs::File>> {
    static SINK: OnceLock<Option<Mutex<std::fs::File>>> = OnceLock::new();
    SINK.get_or_init(|| {
        std::env::var("BOXFINISH_M16").ok().map(|p| {
            Mutex::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(p)
                    .expect("the M16 sink opens"),
            )
        })
    })
    .as_ref()
}

/// The step set whose reading answers, where the ON switch is set
/// (`BOXFINISH_M16_ON=<bits>`; `1` alone is the shipped walk's reach).
fn answer_steps() -> Option<u8> {
    static ON: OnceLock<Option<u8>> = OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("BOXFINISH_M16_ON")
            .ok()
            .map(|v| v.parse::<u8>().unwrap_or(0) & !64)
    })
}

fn answers() -> bool {
    answer_steps().is_some()
}

fn values_on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("BOXFINISH_M16_VALUES").is_some())
}

thread_local! {
    static VALUES: RefCell<HashMap<u128, (f64, f64)>> = RefCell::new(HashMap::new());
}

/// Probe side table: the lane value each node was minted with, on this
/// thread since the last [`context`].
pub(super) fn value_note<T: 'static>(id: SymId, value: &T) {
    if !values_on() {
        return;
    }
    use crate::real::Bounds;
    let any: &dyn std::any::Any = value;
    if let Some(v) = any.downcast_ref::<Interval>() {
        let (lo, hi) = (v.lo(), v.hi());
        VALUES.with(|t| t.borrow_mut().insert(id.bits(), (lo, hi)));
    }
}

#[must_use]
pub fn on() -> bool {
    sink().is_some() || answers()
}

thread_local! {
    static CONTEXT: RefCell<String> = const { RefCell::new(String::new()) };
    static SAMPLED: RefCell<HashMap<&'static str, u32>> = RefCell::new(HashMap::new());
    static VALUED: RefCell<HashMap<&'static str, u32>> = RefCell::new(HashMap::new());
    static STRADDLED: RefCell<HashMap<&'static str, u32>> = RefCell::new(HashMap::new());
    static RENDERED: RefCell<HashMap<&'static str, u32>> = RefCell::new(HashMap::new());
    static DEFINITE: RefCell<HashMap<&'static str, u32>> = RefCell::new(HashMap::new());
    static POINTED: RefCell<HashMap<&'static str, u32>> = RefCell::new(HashMap::new());
}

/// Names what the following decisions belong to (a body, a leaf).
pub fn context(what: &str) {
    CONTEXT.with(|c| *c.borrow_mut() = what.to_owned());
    VALUES.with(|t| t.borrow_mut().clear());
}

/// A free line in the sink.
pub fn note(line: &str) {
    if let Some(f) = sink() {
        let _ = writeln!(f.lock().unwrap(), "NOTE\t{line}");
    }
}

#[derive(Clone, Copy)]
pub(super) struct Pre {
    plain_ns: u128,
    zero: bool,
    boxed: bool,
}

pub(super) fn prebuild(node: SymId) -> Pre {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let Some(sess) = slot.as_mut() else {
            return Pre {
                plain_ns: 0,
                zero: false,
                boxed: false,
            };
        };
        if sess.budget.max_terms == 0 {
            return Pre {
                plain_ns: 0,
                zero: false,
                boxed: false,
            };
        }
        let boxed = sess.params.values().any(|&(lo, hi)| lo < hi);
        let t0 = std::time::Instant::now();
        let plain = plain_form(sess, node);
        Pre {
            plain_ns: t0.elapsed().as_nanos(),
            zero: plain.is_zero(),
            boxed,
        }
    })
}

struct Read {
    class: String,
    lo: f64,
    hi: f64,
    sign: Option<Sign>,
}

fn classify(read: Result<Interval, String>, (vlo, vhi): (f64, f64), band: Band) -> Read {
    use crate::real::Bounds;
    match read {
        Err(why) => Read {
            class: format!("declines:{why}"),
            lo: f64::NAN,
            hi: f64::NAN,
            sign: None,
        },
        Ok(form) => {
            let (flo, fhi) = (form.lo(), form.hi());
            let (lo, hi) = (flo.max(vlo), fhi.min(vhi));
            if lo > hi {
                return Read {
                    class: "EMPTY".into(),
                    lo: flo,
                    hi: fhi,
                    sign: None,
                };
            }
            match Interval::from_bounds(lo, hi).sign_within(band) {
                Ok(d) => Read {
                    class: format!("{:?}", d.sign),
                    lo,
                    hi,
                    sign: Some(d.sign),
                },
                Err(_) if lo.abs().max(hi.abs()) < band.escalate() => Read {
                    class: "sliver".into(),
                    lo,
                    hi,
                    sign: None,
                },
                Err(_) => Read {
                    class: "straddles".into(),
                    lo,
                    hi,
                    sign: None,
                },
            }
        }
    }
}

/// The margin's graph at the member `f` of the way across every bracket.
fn member(sess: &Session, node: SymId, f: f64) -> Result<f64, String> {
    let mut memo: HashMap<SymId, Option<f64>> = HashMap::new();
    let mut blocked = None;
    let mut stack = vec![(node, false)];
    while let Some((id, expanded)) = stack.pop() {
        if memo.contains_key(&id) {
            continue;
        }
        let Some(n) = sess.node_of(id) else {
            memo.insert(id, None);
            blocked.get_or_insert("unknown-node".to_owned());
            continue;
        };
        if !expanded {
            stack.push((id, true));
            stack.extend(n.kids().iter().map(|k| (*k, false)));
            continue;
        }
        let kid = |i: usize| memo.get(&n.kids[i]).copied().flatten();
        let v = match n.op {
            SymOp::Param => match sess.params.get(&super::indet_param(n.payload)) {
                Some(&(lo, hi)) => Some(lo + f * (hi - lo)),
                None => {
                    blocked.get_or_insert("param-without-bracket".to_owned());
                    None
                }
            },
            SymOp::Opaque | SymOp::Hull | SymOp::Free => {
                blocked.get_or_insert(format!("{:?}", n.op));
                None
            }
            SymOp::Lit => Some(f64::from_bits(n.payload)),
            SymOp::Pi => Some(f64::pi()),
            SymOp::Add => (|| Some(kid(0)? + kid(1)?))(),
            SymOp::Sub => (|| Some(kid(0)? - kid(1)?))(),
            SymOp::Mul => match sess.node_of(n.kids[1]) {
                Some(inv) if inv.op == SymOp::Inv => {
                    (|| Some(kid(0)? / memo.get(&inv.kids[0]).copied().flatten()?))()
                }
                _ => (|| Some(kid(0)? * kid(1)?))(),
            },
            SymOp::Neg => kid(0).map(|x| -x),
            SymOp::Powi => kid(0).map(|x| Real::powi(x, n.payload as u32 as i32)),
            SymOp::Inv => kid(0).map(|x| 1.0 / x),
            SymOp::Sqrt => kid(0).map(Real::sqrt),
            SymOp::Abs => kid(0).map(Real::abs),
            SymOp::Sin => kid(0).map(|x| Real::sin_cos(x).0),
            SymOp::Cos => kid(0).map(|x| Real::sin_cos(x).1),
            SymOp::Tan => kid(0).map(Real::tan),
            SymOp::Asin => kid(0).map(Real::asin),
            SymOp::Acos => kid(0).map(Real::acos),
            SymOp::Atan => kid(0).map(Real::atan),
            SymOp::Floor => kid(0).map(Real::floor),
            SymOp::LessSin => kid(0).map(Real::less_sin),
            SymOp::Atan2 => (|| Some(Real::atan2(kid(0)?, kid(1)?)))(),
            SymOp::Min => (|| Some(Real::min(kid(0)?, kid(1)?)))(),
            SymOp::Max => (|| Some(Real::max(kid(0)?, kid(1)?)))(),
            SymOp::Copysign => (|| Some(Real::copysign(kid(0)?, kid(1)?)))(),
            SymOp::Select => (|| Some(Real::select_le_zero(kid(0)?, kid(1)?, kid(2)?)))(),
        };
        memo.insert(id, v);
    }
    memo.get(&node)
        .copied()
        .flatten()
        .ok_or_else(|| blocked.unwrap_or_else(|| "no-value".to_owned()))
}

/// The margin's graph enclosed op by op at ONE member of the box (`f`
/// of the way across every bracket), outward-rounded: the point reading.
fn point(sess: &Session, node: SymId, f: f64) -> Result<Interval, String> {
    use crate::real::Real as R;
    let mut memo: HashMap<SymId, Option<Interval>> = HashMap::new();
    let mut blocked = None;
    let mut stack = vec![(node, false)];
    while let Some((id, expanded)) = stack.pop() {
        if memo.contains_key(&id) {
            continue;
        }
        let Some(n) = sess.node_of(id) else {
            memo.insert(id, None);
            blocked.get_or_insert("unknown-node".to_owned());
            continue;
        };
        if !expanded {
            stack.push((id, true));
            stack.extend(n.kids().iter().map(|k| (*k, false)));
            continue;
        }
        let kid = |i: usize| memo.get(&n.kids[i]).copied().flatten();
        let v: Option<Interval> = match n.op {
            SymOp::Param => match sess.params.get(&super::indet_param(n.payload)) {
                Some(&(lo, hi)) => Some(<Interval as R>::from_f64(lo + f * (hi - lo))),
                None => {
                    blocked.get_or_insert("param-without-bracket".to_owned());
                    None
                }
            },
            SymOp::Opaque | SymOp::Hull | SymOp::Free => {
                blocked.get_or_insert(format!("{:?}", n.op));
                None
            }
            SymOp::Lit => Some(<Interval as R>::from_f64(f64::from_bits(n.payload))),
            SymOp::Pi => Some(<Interval as R>::pi()),
            SymOp::Add => (|| Some(kid(0)? + kid(1)?))(),
            SymOp::Sub => (|| Some(kid(0)? - kid(1)?))(),
            SymOp::Mul => match sess.node_of(n.kids[1]) {
                Some(inv) if inv.op == SymOp::Inv => {
                    (|| Some(kid(0)? / memo.get(&inv.kids[0]).copied().flatten()?))()
                }
                _ => (|| Some(kid(0)? * kid(1)?))(),
            },
            SymOp::Neg => kid(0).map(|x| -x),
            SymOp::Powi => kid(0).map(|x| R::powi(x, n.payload as u32 as i32)),
            SymOp::Inv => kid(0).map(|x| <Interval as R>::from_f64(1.0) / x),
            SymOp::Sqrt => kid(0).map(R::sqrt),
            SymOp::Abs => kid(0).map(R::abs),
            SymOp::Sin => kid(0).map(|x| R::sin_cos(x).0),
            SymOp::Cos => kid(0).map(|x| R::sin_cos(x).1),
            SymOp::Tan => kid(0).map(R::tan),
            SymOp::Asin => kid(0).map(R::asin),
            SymOp::Acos => kid(0).map(R::acos),
            SymOp::Atan => kid(0).map(R::atan),
            SymOp::Floor => kid(0).map(R::floor),
            SymOp::LessSin => kid(0).map(R::less_sin),
            SymOp::Atan2 => (|| Some(R::atan2(kid(0)?, kid(1)?)))(),
            SymOp::Min => (|| Some(R::min(kid(0)?, kid(1)?)))(),
            SymOp::Max => (|| Some(R::max(kid(0)?, kid(1)?)))(),
            SymOp::Copysign => (|| Some(R::copysign(kid(0)?, kid(1)?)))(),
            SymOp::Select => (|| Some(R::select_le_zero(kid(0)?, kid(1)?, kid(2)?)))(),
        };
        memo.insert(id, v);
    }
    match memo.get(&node).copied().flatten() {
        Some(v) if v.is_certified() => Ok(v),
        Some(_) => Err("poison".to_owned()),
        None => Err(blocked.unwrap_or_else(|| "no-value".to_owned())),
    }
}

/// An ask whose margin is a domain violation: it never reaches the tier.
pub(super) fn invalid(today_err: bool) {
    if let Some(f) = sink() {
        let _ = writeln!(
            f.lock().unwrap(),
            "INVALID\t{}\t{}\t{}",
            CONTEXT.with(|c| c.borrow().clone()),
            crate::k_stats::current_predicate(),
            if today_err { "undecided" } else { "decided" }
        );
    }
}

/// A sample of the asks the VALUE signs: is each member's `f64` margin
/// inside the leaf's value enclosure?
pub(super) fn definite(node: SymId, enc: (f64, f64)) {
    let Some(sink) = sink() else { return };
    let name = crate::k_stats::current_predicate();
    let take = DEFINITE.with(|m| {
        let mut m = m.borrow_mut();
        let n = m.entry(name).or_insert(0);
        *n += 1;
        *n <= 20
    });
    if !take {
        return;
    }
    SESSION.with(|s| {
        let slot = s.borrow();
        let Some(sess) = slot.as_ref() else { return };
        if !sess.params.values().any(|&(lo, hi)| lo < hi) {
            return;
        }
        let ms: Vec<String> = [0.0, 0.25, 0.5, 0.75, 1.0]
            .iter()
            .map(|&f| match member(sess, node, f) {
                Ok(v) => format!("{}:{v:e}", if enc.0 <= v && v <= enc.1 { "in" } else { "OUT" }),
                Err(why) => format!("NA:{why}"),
            })
            .collect();
        let _ = writeln!(
            sink.lock().unwrap(),
            "DEF\t{}\t{name}\t{:e}\t{:e}\t{}",
            CONTEXT.with(|c| c.borrow().clone()),
            enc.0,
            enc.1,
            ms.join(",")
        );
    });
}

/// The margin's graph: every node that can stand as an indeterminate,
/// and the forms of each hull's two children.
struct Graph {
    kinds: HashMap<u128, (signed::M16Kind, u128)>,
    hulls: HashMap<u128, [std::sync::Arc<super::form::Form>; 2]>,
}

fn graph(sess: &mut Session, node: SymId) -> Graph {
    use signed::M16Kind as K;
    let mut kinds = HashMap::new();
    let mut hull_kids = Vec::new();
    let mut seen: HashMap<SymId, ()> = HashMap::new();
    let mut stack = vec![node];
    while let Some(id) = stack.pop() {
        if seen.insert(id, ()).is_some() {
            continue;
        }
        let Some(n) = sess.node_of(id) else {
            kinds.insert(id.bits(), (K::Node(SymOp::Opaque), id.bits()));
            continue;
        };
        kinds.entry(id.bits()).or_insert((K::Node(n.op), id.bits()));
        match n.op {
            SymOp::Param => {
                let indet = super::indet_param(n.payload);
                if !sess.params.contains_key(&indet) {
                    kinds.insert(indet, (K::BareParam, id.bits()));
                }
            }
            SymOp::Opaque => {
                kinds.insert(n.opaque_indet(), (K::Opaque, id.bits()));
            }
            SymOp::Free => {
                kinds.insert(super::INDET_FREE, (K::Free, id.bits()));
            }
            SymOp::Hull => {
                let indet = super::indet_hull(n.kids[0], n.kids[1]);
                kinds.insert(indet, (K::Hull, id.bits()));
                hull_kids.push((indet, n.kids[0], n.kids[1]));
            }
            _ => {}
        }
        stack.extend(n.kids().iter().copied());
    }
    let hulls = hull_kids
        .into_iter()
        .map(|(indet, a, b)| (indet, [plain_form(sess, a), plain_form(sess, b)]))
        .collect();
    Graph { kinds, hulls }
}

const STEPS: [u8; 7] = [0, 1, 3, 7, 15, 47, 111];

type Frozen = HashMap<u128, (SymOp, u64, Vec<std::sync::Arc<super::form::Form>>)>;

/// One reading at `steps`: the plain form, else the early one. Frozen
/// nodes the walk meets are opened (their children's plain forms built)
/// and the walk repeated, a bounded number of times.
#[allow(clippy::too_many_arguments)]
fn read_at(
    sess: &mut Session,
    forms: (&super::form::Form, Option<&super::form::Form>),
    g: &Graph,
    frozen: &mut Frozen,
    values: &HashMap<u128, (f64, f64)>,
    steps: u8,
    enc: (f64, f64),
    band: Band,
) -> (Read, &'static str, u128) {
    let t0 = std::time::Instant::now();
    let mut rounds = 0;
    loop {
        let (out, want) = {
            let cx = signed::M16Ctx {
                params: &sess.params,
                atoms: &sess.atoms,
                steps,
                graph: &g.kinds,
                hulls: &g.hulls,
                values,
                frozen,
                memo: RefCell::new(HashMap::new()),
                want: std::cell::Cell::new(None),
            };
            let p = classify(signed::m16_form(forms.0, &cx, 0), enc, band);
            let out = if p.sign.is_some() || p.class == "EMPTY" {
                (p, "plain")
            } else {
                match forms.1 {
                    Some(early) => {
                        cx.memo.borrow_mut().clear();
                        let e = classify(signed::m16_form(early, &cx, 0), enc, band);
                        if e.sign.is_some() || e.class == "EMPTY" || p.class.starts_with("declines")
                        {
                            (e, "early")
                        } else {
                            (p, "plain")
                        }
                    }
                    None => (p, "plain"),
                }
            };
            (out, cx.want.get())
        };
        let decided = out.0.sign.is_some() || out.0.class == "EMPTY";
        match want {
            Some(id) if !decided && rounds < 400 && !frozen.contains_key(&id) => {
                rounds += 1;
                let entry = match sess.node_of(SymId(id)) {
                    Some(n) => {
                        let kids = n.kids().to_vec();
                        (
                            n.op,
                            n.payload,
                            kids.into_iter().map(|k| plain_form(sess, k)).collect(),
                        )
                    }
                    None => (SymOp::Opaque, 0, Vec::new()),
                };
                frozen.insert(id, entry);
            }
            _ => return (out.0, out.1, t0.elapsed().as_nanos()),
        }
    }
}

/// Records the reading at each cumulative step; `Some(sign)` only when
/// the reading is switched on as the answer and decides where today's
/// path is undecided.
pub(super) fn probe(
    node: SymId,
    enc: (f64, f64),
    band: Band,
    pre: Pre,
    today: &'static str,
    ladder_ns: u128,
) -> Option<Sign> {
    if !pre.boxed {
        return None;
    }
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        let name = crate::k_stats::current_predicate();
        let undecided = today == "undecided";
        let plain = plain_form(sess, node);
        if !undecided && plain.is_zero() {
            if let Some(f) = sink() {
                let _ = writeln!(
                    f.lock().unwrap(),
                    "ZERO\t{}\t{name}\t{today}\t{}",
                    CONTEXT.with(|c| c.borrow().clone()),
                    enc.0 <= 0.0 && 0.0 <= enc.1
                );
            }
            return None;
        }
        let early = sess.rules.early.then(|| early_form(sess, node));
        let g = graph(sess, node);
        let values = VALUES.with(|t| std::mem::take(&mut *t.borrow_mut()));
        let mut frozen = Frozen::new();
        let sampled = |cap: u32,
                       table: &'static std::thread::LocalKey<
            RefCell<HashMap<&'static str, u32>>,
        >| {
            table.with(|m| {
                let mut m = m.borrow_mut();
                let n = m.entry(name).or_insert(0);
                *n += 1;
                *n <= cap
            })
        };
        let take_values = values_on() && undecided && sampled(40, &VALUED);
        let on = answer_steps();
        let steps: Vec<u8> = if !undecided {
            vec![15]
        } else if let (Some(on), None) = (on, sink()) {
            vec![on]
        } else {
            STEPS
                .iter()
                .copied()
                .filter(|st| st & 64 == 0 || take_values)
                .collect()
        };
        let reads: Vec<(u8, Read, &'static str, u128)> = steps
            .iter()
            .map(|&st| {
                let (r, form, ns) = read_at(
                    sess,
                    (&plain, early.as_deref()),
                    &g,
                    &mut frozen,
                    &values,
                    st,
                    enc,
                    band,
                );
                (st, r, form, ns)
            })
            .collect();
        VALUES.with(|t| *t.borrow_mut() = values);
        let answer = on.and_then(|on| {
            reads
                .iter()
                .find(|(st, ..)| *st == on)
                .and_then(|(_, r, ..)| r.sign)
        });
        let best = reads.iter().rev().find(|(st, ..)| st & 64 == 0);
        let straddles = best.is_some_and(|(_, r, ..)| r.class == "straddles");
        let want_members = match best.and_then(|(_, r, ..)| r.sign) {
            Some(_) => sampled(if undecided { 400 } else { 20 }, &SAMPLED),
            None => undecided && straddles && sampled(60, &STRADDLED),
        };
        let members = if want_members {
            [0.0, 0.25, 0.5, 0.75, 1.0]
                .map(|f| match member(sess, node, f) {
                    Ok(v) => match v.sign_within(band) {
                        Ok(d) => format!("{:?}:{v:e}", d.sign),
                        Err(_) => format!("Undecided:{v:e}"),
                    },
                    Err(why) => format!("NA:{why}"),
                })
                .join(",")
        } else {
            String::new()
        };
        let decided = best.and_then(|(_, r, ..)| r.sign).is_some();
        let p_cell = if decided && (undecided || sampled(20, &POINTED)) {
            match point(sess, node, 0.5) {
                Ok(p) => {
                    use crate::real::Bounds;
                    let class = match p.sign_within(band) {
                        Ok(d) => format!("{:?}", d.sign),
                        Err(_) => "straddles".to_owned(),
                    };
                    format!("{class}|{:e}|{:e}", p.lo(), p.hi())
                }
                Err(why) => format!("declines:{why}||"),
            }
        } else {
            String::new()
        };
        if let Some(f) = sink() {
            let cells: Vec<String> = reads
                .iter()
                .map(|(st, r, form, ns)| {
                    format!("{st}|{}|{form}|{:e}|{:e}|{ns}", r.class, r.lo, r.hi)
                })
                .collect();
            let _ = writeln!(
                f.lock().unwrap(),
                "ASK2\t{}\t{name}\t{today}\t{:e}\t{:e}\t{:e}\t{:e}\t{}\t{}\t{}\t{}\t{p_cell}",
                CONTEXT.with(|c| c.borrow().clone()),
                enc.0,
                enc.1,
                band.zero(),
                band.escalate(),
                pre.plain_ns,
                ladder_ns,
                cells.join(";"),
                members,
            );
            if undecided && straddles && sampled(2, &RENDERED) {
                let _ = writeln!(
                    f.lock().unwrap(),
                    "REP\t{}\t{name}\tvalue=[{:e},{:e}]\tplain=<{}>\tearly=<{}>",
                    CONTEXT.with(|c| c.borrow().clone()),
                    enc.0,
                    enc.1,
                    super::report::m16_render(sess, &plain),
                    early
                        .as_deref()
                        .map_or(String::new(), |e| super::report::m16_render(sess, e)),
                );
            }
        }
        (answers() && undecided).then_some(answer).flatten()
    })
}

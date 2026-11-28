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

fn answers() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("BOXFINISH_M16_ON").is_some())
}

#[must_use]
pub fn on() -> bool {
    sink().is_some() || answers()
}

thread_local! {
    static CONTEXT: RefCell<String> = const { RefCell::new(String::new()) };
    static SAMPLED: RefCell<HashMap<&'static str, u32>> = RefCell::new(HashMap::new());
}

/// Names what the following decisions belong to (a body, a leaf).
pub fn context(what: &str) {
    CONTEXT.with(|c| *c.borrow_mut() = what.to_owned());
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

/// What in the margin's graph an enclosure walk cannot bracket.
fn dag_flags(sess: &Session, node: SymId) -> String {
    let (mut frozen, mut bare, mut opaque, mut hull, mut free, mut unknown) =
        (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let mut seen: HashMap<SymId, ()> = HashMap::new();
    let mut stack = vec![node];
    while let Some(id) = stack.pop() {
        if seen.insert(id, ()).is_some() {
            continue;
        }
        if sess.budget_frozen.contains_key(&id) {
            frozen += 1;
        }
        let Some(n) = sess.node_of(id) else {
            unknown += 1;
            continue;
        };
        match n.op {
            SymOp::Param if !sess.params.contains_key(&super::indet_param(n.payload)) => bare += 1,
            SymOp::Opaque => opaque += 1,
            SymOp::Hull => hull += 1,
            SymOp::Free => free += 1,
            _ => {}
        }
        stack.extend(n.kids().iter().copied());
    }
    format!(
        "frozen={frozen},bare-param={bare},opaque={opaque},hull={hull},free={free},unknown={unknown}"
    )
}

/// Records the reading; `Some(sign)` only when the reading is switched on
/// as the answer and decides where today's path did not prove zero.
pub(super) fn probe(
    node: SymId,
    enc: (f64, f64),
    band: Band,
    pre: Pre,
    today: &'static str,
    ladder_ns: u128,
) -> Option<Sign> {
    if !pre.boxed {
        if let Some(f) = sink() {
            let _ = writeln!(
                f.lock().unwrap(),
                "POINT\t{}\t{}\t{today}",
                CONTEXT.with(|c| c.borrow().clone()),
                crate::k_stats::current_predicate()
            );
        }
        return None;
    }
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        let name = crate::k_stats::current_predicate();
        let plain = plain_form(sess, node);
        let size = (plain.num.terms().len(), plain.den.terms().len());
        let p = classify(signed::m16_form(&plain, &sess.params, &sess.atoms, 0), enc, band);
        let needs_early = p.sign.is_none() && p.class != "EMPTY";
        let (e, early_ns) = if needs_early && sess.rules.early {
            let t0 = std::time::Instant::now();
            let early = early_form(sess, node);
            let ns = t0.elapsed().as_nanos();
            (
                Some(classify(signed::m16_form(&early, &sess.params, &sess.atoms, 0), enc, band)),
                ns,
            )
        } else {
            (None, 0)
        };
        let sign = p.sign.or(e.as_ref().and_then(|e| e.sign));
        let flags = if sign.is_none() && today == "undecided" {
            dag_flags(sess, node)
        } else {
            "-".to_owned()
        };
        let members = match sign {
            Some(_) if SAMPLED.with(|m| {
                let mut m = m.borrow_mut();
                let n = m.entry(name).or_insert(0);
                *n += 1;
                *n <= 40
            }) =>
            {
                [0.0, 0.25, 0.5, 0.75, 1.0]
                    .map(|f| match member(sess, node, f) {
                        Ok(v) => match v.sign_within(band) {
                            Ok(d) => format!("{:?}:{v:e}", d.sign),
                            Err(_) => format!("Undecided:{v:e}"),
                        },
                        Err(why) => format!("NA:{why}"),
                    })
                    .join(",")
            }
            _ => String::new(),
        };
        if let Some(f) = sink() {
            let _ = writeln!(
                f.lock().unwrap(),
                "ASK\t{}\t{name}\t{today}\t{:e}\t{:e}\t{}\t{}\t{}\t{}\t{}:{}\t{}\t{:e}\t{:e}\t{}\t{}\t{}\t{:?}",
                CONTEXT.with(|c| c.borrow().clone()),
                enc.0,
                enc.1,
                pre.zero,
                pre.plain_ns,
                ladder_ns,
                p.class,
                size.0,
                size.1,
                flags,
                p.lo,
                p.hi,
                e.as_ref().map_or("-", |e| e.class.as_str()),
                early_ns,
                members,
                sess.witness,
            );
        }
        (answers() && today == "undecided").then_some(sign).flatten()
    })
}

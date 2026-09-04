//! **The shape report**: an instrument that records, per predicate
//! decision made at a [`Sym`](super::Sym) scalar, how it was answered
//! — and for a decision the numeric channel had to answer, the NORMAL
//! FORM of the residual that blocked the symbolic one, rendered with
//! its atoms spelled out (`sqrt` of what argument, `cos` of what).
//!
//! It exists so that the question "which rule would discharge this
//! site" is answered by reading the residual rather than by guessing
//! at it, which is the order ERROR-DESIGN E12's reserve clause asks
//! for: measure the miss, then build the mechanism the measurement
//! justifies. Evidence-only, thread-local, off unless installed; an
//! ordinary replay pays one flag read per decision for it.
//!
//! Parameter symbols are hashes, so a harness that wants names in the
//! rendering registers them (`name_param`) before evaluating.

use core::cell::{Cell, RefCell};
use std::collections::BTreeMap;

use super::{
    Discharge, Form, INDET_PI, ParamSymbol, Poly, Rat, SESSION, Session, SymId, SymOp, indet_param,
    plain_form,
};
use crate::predicate::{Indeterminate, MarginDiag, Sign};

/// How one decision at the symbolic scalar came out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeOutcome {
    /// A symbolic `Zero` with no value read.
    Theorem,
    /// A symbolic `Zero` through a clause-3 fold.
    SignGated,
    /// The numeric channel certified a definite non-zero sign; the
    /// form was never built.
    Definite(Sign),
    /// The numeric channel answered `Zero` inside the band; the form
    /// was NOT the zero form.
    NumericZero,
    /// The numeric channel could not decide; the form was not zero.
    Indeterminate,
    /// A domain violation; the identity test was never asked.
    Invalid,
}

/// One recorded decision.
#[derive(Clone, Debug)]
pub struct DecisionShape {
    /// The funnel predicate that asked.
    pub predicate: &'static str,
    /// How it was answered.
    pub outcome: ShapeOutcome,
    /// The residual's normal form, for a decision that stayed
    /// numeric without a definite sign — `None` otherwise, and `None`
    /// outside a session.
    pub form: Option<String>,
}

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static SHAPES: RefCell<Vec<DecisionShape>> = const { RefCell::new(Vec::new()) };
    static NAMES: RefCell<BTreeMap<u128, String>> = const { RefCell::new(BTreeMap::new()) };
}

/// Installs the report on this thread, dropping anything recorded.
pub fn start_shape_report() {
    SHAPES.with(|s| s.borrow_mut().clear());
    ACTIVE.set(true);
}

/// Removes the report and answers everything recorded since
/// [`start_shape_report`].
pub fn take_shape_report() -> Vec<DecisionShape> {
    ACTIVE.set(false);
    SHAPES.with(|s| core::mem::take(&mut *s.borrow_mut()))
}

/// Registers a parameter's NAME for rendering, on this thread.
pub fn name_param(name: &str) {
    NAMES.with(|n| {
        n.borrow_mut()
            .insert(indet_param(ParamSymbol::of(name).0), name.to_owned());
    });
}

pub(super) fn active() -> bool {
    ACTIVE.get()
}

/// Records one decision, if the report is installed.
pub(super) fn record(
    numeric: &Result<Sign, Indeterminate>,
    discharge: Option<Discharge>,
    form: Option<String>,
) {
    if !active() {
        return;
    }
    let outcome = match (discharge, numeric) {
        (Some(Discharge::Theorem), _) => ShapeOutcome::Theorem,
        (Some(Discharge::SignGated), _) => ShapeOutcome::SignGated,
        (None, Ok(Sign::Zero)) => ShapeOutcome::NumericZero,
        (None, Ok(s)) => ShapeOutcome::Definite(*s),
        (None, Err(e)) if matches!(e.margin, MarginDiag::Invalid) => ShapeOutcome::Invalid,
        (None, Err(_)) => ShapeOutcome::Indeterminate,
    };
    SHAPES.with(|s| {
        s.borrow_mut().push(DecisionShape {
            predicate: crate::k_stats::current_predicate(),
            outcome,
            form,
        });
    });
}

/// The rendered PLAIN normal form of `id` in the installed session —
/// the residual the numeric channel had to answer, with its atoms
/// spelled out — or `None` outside a session (or the tier off).
pub(super) fn render_node(id: SymId) -> Option<String> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        if sess.budget.max_terms == 0 {
            return None;
        }
        let f = plain_form(sess, id);
        Some(render_form(sess, &f, 0))
    })
}

/// Nested atoms render to this depth, then `…`.
const DEPTH: usize = 4;

fn render_form(sess: &Session, f: &Form, depth: usize) -> String {
    if f.poisoned {
        return "⊥".to_owned();
    }
    let num = render_poly(sess, &f.num, depth);
    if f.den == Poly::one() {
        num
    } else {
        format!("({num}) / ({})", render_poly(sess, &f.den, depth))
    }
}

fn render_poly(sess: &Session, p: &Poly, depth: usize) -> String {
    if p.is_zero() {
        return "0".to_owned();
    }
    p.terms
        .iter()
        .map(|(m, c)| {
            let mut parts = vec![render_rat(*c)];
            for &(id, e) in m {
                let v = render_indet(sess, id, depth);
                parts.push(if e == 1 { v } else { format!("{v}^{e}") });
            }
            parts.join("·")
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn render_rat(c: Rat) -> String {
    if c.den == 1 && (0..=40).contains(&c.exp2) {
        return format!("{}", c.num << c.exp2);
    }
    if c.den == 1 {
        return format!("{}·2^{}", c.num, c.exp2);
    }
    format!("{}/{}·2^{}", c.num, c.den, c.exp2)
}

fn render_indet(sess: &Session, id: u128, depth: usize) -> String {
    if id == INDET_PI {
        return "π".to_owned();
    }
    if let Some(name) = NAMES.with(|n| n.borrow().get(&id).cloned()) {
        return name;
    }
    if sess.params.contains_key(&id) {
        return format!("param#{:08x}", id as u32);
    }
    if let Some(atom) = sess.atoms.get(&id) {
        let name = match atom.op {
            SymOp::Sqrt => "sqrt",
            SymOp::Abs => "abs",
            SymOp::Sin => "sin",
            SymOp::Cos => "cos",
            SymOp::Tan => "tan",
            SymOp::Asin => "asin",
            SymOp::Acos => "acos",
            SymOp::Atan => "atan",
            SymOp::Floor => "floor",
            SymOp::Atan2 => "atan2",
            SymOp::Min => "min",
            SymOp::Max => "max",
            SymOp::Copysign => "copysign",
            _ => "?",
        };
        if depth >= DEPTH {
            return format!("{name}(…)");
        }
        let args = atom
            .args
            .iter()
            .flatten()
            .map(|a| render_form(sess, a, depth + 1))
            .collect::<Vec<_>>()
            .join(", ");
        return format!("{name}({args})");
    }
    format!("?#{:08x}", id as u32)
}

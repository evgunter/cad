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
    Discharge, Form, INDET_PI, ParamSymbol, Poly, Rat, SESSION, Session, SymId, SymOp, early_form,
    indet_param, plain_form,
};
use crate::predicate::{Indeterminate, MarginDiag, Sign};

/// How one decision at the symbolic scalar came out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeOutcome {
    /// A symbolic `Zero` with no value read.
    Theorem,
    /// A symbolic `Zero` through a clause-3 fold.
    SignGated,
    /// A `Zero` through a REGISTERED IDENTITY — an axiom a constructor
    /// stated about what it built (`Sym::register_equal`), not a
    /// theorem the tier proved.
    Registered,
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

/// The size of one quotient form: numerator and denominator, each as
/// `(terms, total degree)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormSize {
    /// The numerator's term count and total degree.
    pub num: (usize, u32),
    /// The denominator's term count and total degree.
    pub den: (usize, u32),
}

/// One recorded decision.
#[derive(Clone, Debug)]
pub struct DecisionShape {
    /// The funnel predicate that asked.
    pub predicate: &'static str,
    /// How it was answered.
    pub outcome: ShapeOutcome,
    /// The residual's PLAIN normal form, for a decision that stayed
    /// numeric without a definite sign — `None` otherwise, and `None`
    /// outside a session.
    pub form: Option<String>,
    /// The same residual's EARLY form (the session's rules applied per
    /// node), rendered beside the plain one, so a reader can see what
    /// the rules reached and what stood after them. `None` where
    /// `form` is, and `None` when the session runs no early walk.
    pub early_form: Option<String>,
    /// The two forms' SIZES — `(terms, degree)` of the numerator and
    /// of the denominator, plain then early — so a residual's shape is
    /// a number and not an eyeballed rendering.
    pub sizes: Option<[FormSize; 2]>,
    /// The DAG below the residual, one line per node to the depth
    /// [`explain_depth`] set: op, early-form size, and `FROZEN` where
    /// the early walk could not build the node's form — the line that
    /// says WHICH product the ring or the budget refused.
    pub explain: Option<String>,
    /// The certified enclosure the numeric channel classified
    /// (`Interval` lane only; `None` at every other scalar). This is
    /// what makes a blocked decision READABLE as a distance from the
    /// band rather than as a name — see [`crate::Decide::enclosure_probe`].
    pub enclosure: Option<(f64, f64)>,
}

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    /// How many levels below a blocked residual [`explain`] walks
    /// (zero: no explanation is rendered).
    static EXPLAIN: Cell<usize> = const { Cell::new(0) };
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

/// Sets how many DAG levels below a blocked residual the report
/// EXPLAINS ([`DecisionShape::explain`]): per node its op, its early
/// form's size and whether the early walk FROZE it. Zero (the default)
/// renders no explanation.
pub fn explain_depth(levels: usize) {
    EXPLAIN.set(levels);
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

/// Records one decision, if the report is installed. `symbolic` is how
/// the tier answered it, if it did: an unconditional theorem, or one
/// gated on a clause-3 sign read (rule C).
pub(super) fn record(
    numeric: &Result<Sign, Indeterminate>,
    symbolic: Option<Discharge>,
    rendered: Option<Rendered>,
    enclosure: Option<(f64, f64)>,
) {
    if !active() {
        return;
    }
    let outcome = match (symbolic, numeric) {
        (Some(Discharge::Theorem), _) => ShapeOutcome::Theorem,
        (Some(Discharge::SignGated), _) => ShapeOutcome::SignGated,
        (Some(Discharge::Registered), _) => ShapeOutcome::Registered,
        (None, Ok(Sign::Zero)) => ShapeOutcome::NumericZero,
        (None, Ok(s)) => ShapeOutcome::Definite(*s),
        (None, Err(e)) if matches!(e.margin, MarginDiag::Invalid) => ShapeOutcome::Invalid,
        (None, Err(_)) => ShapeOutcome::Indeterminate,
    };
    let (form, early_form, sizes, explain) = match rendered {
        Some(r) => (Some(r.plain), r.early, Some(r.sizes), r.explain),
        None => (None, None, None, None),
    };
    SHAPES.with(|s| {
        s.borrow_mut().push(DecisionShape {
            predicate: crate::k_stats::current_predicate(),
            outcome,
            form,
            early_form,
            sizes,
            explain,
            enclosure,
        });
    });
}

/// What [`render_node`] hands the recorder: both renderings and both
/// sizes of one blocked residual.
pub(super) struct Rendered {
    pub(super) plain: String,
    pub(super) early: Option<String>,
    pub(super) sizes: [FormSize; 2],
    pub(super) explain: Option<String>,
}

/// The DAG below `root` to `levels` deep, one line per node: its op,
/// its early form's size, and `FROZEN` where the early memo holds the
/// node's own indeterminate — the shape of a refusal, read off the
/// tree rather than guessed from the residual. Nodes are visited in
/// child order and a node reached twice is rendered once.
/// The largest numerator (terms) [`explain`] renders in full.
const EXPLAIN_RENDER_TERMS: usize = 80;

/// The most characters of one rendered form [`explain`] prints.
const EXPLAIN_RENDER_CHARS: usize = 1500;

fn explain(sess: &mut Session, root: SymId, levels: usize) -> String {
    use core::fmt::Write as _;
    let mut out = String::new();
    let mut seen: super::IdMap<()> = super::IdMap::default();
    let mut stack = vec![(root, 0usize)];
    while let Some((id, depth)) = stack.pop() {
        if seen.insert(id, ()).is_some() {
            continue;
        }
        let pad = "  ".repeat(depth);
        let Some(node) = sess.nodes.get(&id).copied() else {
            let _ = writeln!(out, "{pad}[unrecorded #{:08x}]", id.bits() as u32);
            continue;
        };
        let e = early_form(sess, id);
        let frozen = e.den == Poly::one()
            && e.num.terms.len() == 1
            && e.num
                .terms
                .keys()
                .next()
                .is_some_and(|m| m.as_slice() == [(id.bits(), 1)]);
        let payload = match node.op {
            SymOp::Lit => format!(" {}", f64::from_bits(node.payload)),
            SymOp::Powi => format!(" ^{}", node.payload as u32 as i32),
            _ => String::new(),
        };
        let shape = if frozen {
            "FROZEN".to_owned()
        } else {
            let sz = size_of(&e);
            format!("num {:?} den {:?}", sz.num, sz.den)
        };
        let _ = writeln!(
            out,
            "{pad}{:?}{payload} #{:08x}: {shape}",
            node.op,
            id.bits() as u32
        );
        // A small form is worth reading in full: the two sides of a
        // residual that does not cancel are usually a few dozen terms.
        if !frozen && e.num.terms.len() <= EXPLAIN_RENDER_TERMS {
            let text = render_form(sess, &e, 0);
            let cut = text
                .char_indices()
                .nth(EXPLAIN_RENDER_CHARS)
                .map_or(text.len(), |(i, _)| i);
            let _ = writeln!(
                out,
                "{pad}  = {}{}",
                &text[..cut],
                if cut < text.len() { "…" } else { "" }
            );
        }
        if depth < levels {
            for k in node.kids[..node.op.arity()].iter().rev() {
                stack.push((*k, depth + 1));
            }
        }
    }
    out
}

fn size_of(f: &Form) -> FormSize {
    FormSize {
        num: (f.num.terms.len(), f.num.degree()),
        den: (f.den.terms.len(), f.den.degree()),
    }
}

/// The rendered PLAIN normal form of `id` in the installed session —
/// the residual the numeric channel had to answer, with its atoms
/// spelled out — beside its EARLY form and both sizes; `None` outside
/// a session (or the tier off).
pub(super) fn render_node(id: SymId) -> Option<Rendered> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        if sess.budget.max_terms == 0 {
            return None;
        }
        let f = plain_form(sess, id);
        let plain = render_form(sess, &f, 0);
        let (early, early_size) = if sess.rules.early {
            let e = early_form(sess, id);
            (Some(render_form(sess, &e, 0)), size_of(&e))
        } else {
            (None, size_of(&f))
        };
        let levels = EXPLAIN.get();
        let explain = (levels > 0 && sess.rules.early).then(|| explain(sess, id, levels));
        Some(Rendered {
            plain,
            early,
            sizes: [size_of(&f), early_size],
            explain,
        })
    })
}

/// **The rendered plain normal form of any node**, for evidence that
/// has to quote TWO forms side by side — a registrant's expression
/// against its intended consumer's, when a registered identity does not
/// reach the consumer because the two are not the same node
/// (`Sym::register_equal`'s same-object clause). `None` outside a
/// session, or with the tier off.
///
/// The public half of [`render_node`], which the `Decide` impl uses for
/// the residual that BLOCKED; this one names its node, because the
/// interesting pair is usually two nodes no decide site ever asked
/// about together.
#[must_use]
pub fn render_of(node: SymId) -> Option<String> {
    render_node(node).map(|r| r.plain)
}

/// **The rendered EARLY form of any node** — [`render_of`]'s twin for
/// the walk the atom algebra runs in, so a probe can quote what the
/// rules reached beside what the plain form holds. `None` outside a
/// session, with the tier off, or when the session runs no early walk.
#[must_use]
pub fn render_early_of(node: SymId) -> Option<String> {
    render_node(node).and_then(|r| r.early)
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
            let mut parts = vec![render_rat(c)];
            for &(id, e) in m {
                let v = render_indet(sess, id, depth);
                parts.push(if e == 1 { v } else { format!("{v}^{e}") });
            }
            parts.join("·")
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn render_rat(c: &Rat) -> String {
    if c.den.is_one() && (0..=40).contains(&c.exp2) {
        return format!("{}", c.num.shl(c.exp2 as usize));
    }
    if c.den.is_one() {
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

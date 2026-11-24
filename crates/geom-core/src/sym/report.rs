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

use super::form::{Form, Poly};
use super::rational::Rat;
use super::{
    INDET_PI, ParamSymbol, SESSION, Session, SymId, SymOp, early_form, indet_param, plain_form,
};
use crate::predicate::{Decided, Indeterminate, Sign};

/// How one decision at the symbolic scalar came out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeOutcome {
    /// A symbolic `Zero`: the tier proved the margin zero.
    Symbolic,
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

// The install / take scaffold below is spelled again in `profile`
// (two `Cell`s and a `RefCell` of a different payload): less than a
// generic would cost to name, and each copy says so.
thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    /// How many levels below a blocked residual [`explain`] walks
    /// (zero: no explanation is rendered).
    static EXPLAIN: Cell<usize> = const { Cell::new(0) };
    /// The most characters of one rendered form [`explain`] prints
    /// ([`explain_render_chars`]).
    static RENDER_CHARS: Cell<usize> = const { Cell::new(EXPLAIN_RENDER_CHARS) };
    static SHAPES: RefCell<Vec<DecisionShape>> = const { RefCell::new(Vec::new()) };
    static NAMES: RefCell<BTreeMap<u128, String>> = const { RefCell::new(BTreeMap::new()) };
}

/// Installs the report on this thread, dropping anything recorded.
pub fn start_shape_report() {
    SHAPES.with(|s| s.borrow_mut().clear());
    ACTIVE.set(true);
}

/// Removes the report and answers everything recorded since
/// [`start_shape_report`]. The explanation's render width goes back to
/// its default here too ([`explain_render_chars`] is per report, not
/// per thread: a row that raised it cannot leak the raised width into
/// a later decision rendered on the same thread of a shared test
/// binary).
pub fn take_shape_report() -> Vec<DecisionShape> {
    ACTIVE.set(false);
    RENDER_CHARS.set(EXPLAIN_RENDER_CHARS);
    SHAPES.with(|s| core::mem::take(&mut *s.borrow_mut()))
}

/// How many decisions the report holds ([`super::aside`]'s mark).
pub(super) fn mark() -> usize {
    SHAPES.with(|s| s.borrow().len())
}

/// Drops every decision recorded after `mark` ([`super::Aside::discard`]).
pub(super) fn truncate(mark: usize) {
    SHAPES.with(|s| s.borrow_mut().truncate(mark));
}

/// Sets how many DAG levels below a blocked residual the report
/// EXPLAINS ([`DecisionShape::explain`]): per node its op, its early
/// form's size and whether the early walk FROZE it. Zero (the default)
/// renders no explanation.
pub fn explain_depth(levels: usize) {
    EXPLAIN.set(levels);
}

/// Sets the most characters of one rendered form the explanation
/// prints (the default cuts at 1500, which is a page and is where an
/// atom's argument gets cut open when the form carrying it is wide —
/// a row that has to READ the argument raises it). Set it after
/// [`start_shape_report`]: [`take_shape_report`] restores the default.
pub fn explain_render_chars(chars: usize) {
    RENDER_CHARS.set(chars);
}

/// Registers the NAME `symbol` renders as, on this thread.
pub fn name_param(symbol: ParamSymbol, name: &str) {
    NAMES.with(|n| {
        n.borrow_mut()
            .insert(indet_param(symbol.0), name.to_owned());
    });
}

pub(super) fn active() -> bool {
    ACTIVE.get()
}

/// Records one decision, if the report is installed. `symbolic` is
/// whether the tier proved it zero.
pub(super) fn record(
    numeric: &Result<Decided, Indeterminate>,
    symbolic: bool,
    rendered: Option<Rendered>,
    enclosure: Option<(f64, f64)>,
) {
    if !active() {
        return;
    }
    let outcome = match (symbolic, numeric) {
        (true, _) => ShapeOutcome::Symbolic,
        (
            false,
            Ok(Decided {
                sign: Sign::Zero, ..
            }),
        ) => ShapeOutcome::NumericZero,
        (false, Ok(Decided { sign, .. })) => ShapeOutcome::Definite(*sign),
        (false, Err(e)) if e.margin.is_invalid() => ShapeOutcome::Invalid,
        (false, Err(_)) => ShapeOutcome::Indeterminate,
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

/// The largest numerator (terms) [`explain`] renders in full.
const EXPLAIN_RENDER_TERMS: usize = 80;

/// The most characters of one rendered form [`explain`] prints unless
/// [`explain_render_chars`] set another.
const EXPLAIN_RENDER_CHARS: usize = 1500;

/// The DAG below `root` to `levels` deep, one line per node: its op,
/// its early form's size, and `FROZEN` where the early memo holds the
/// node's own indeterminate — the shape of a refusal, read off the
/// tree rather than guessed from the residual. Nodes are visited in
/// child order and a node reached twice is rendered once; the walk
/// stops where the text reaches [`TEXT_BUDGET`], saying how many nodes
/// it left.
fn explain(sess: &mut Session, root: SymId, levels: usize) -> String {
    let mut out = Ink::new(TEXT_BUDGET);
    let mut seen: super::IdMap<()> = super::IdMap::default();
    let mut stack = vec![(root, 0usize)];
    while let Some((id, depth)) = stack.pop() {
        if seen.insert(id, ()).is_some() {
            continue;
        }
        if out.dry {
            // This node and what the stack still holds (a node the walk
            // would reach twice is counted at each).
            out.mark(&format!(
                "‹{} nodes on the walk are not explained›\n",
                stack.len() + 1
            ));
            break;
        }
        let pad = "  ".repeat(depth);
        let Some(node) = sess.nodes.get(&id).copied() else {
            out.put(&format!("{pad}[unrecorded #{:08x}]\n", id.bits() as u32));
            continue;
        };
        let e = early_form(sess, id);
        let frozen = e.den == Poly::one()
            && e.num.terms().len() == 1
            && e.num
                .terms()
                .first()
                .is_some_and(|(m, _)| m.as_slice() == [(id.bits(), 1)]);
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
        let mut lines = format!(
            "{pad}{:?}{payload} #{:08x}: {shape}\n",
            node.op,
            id.bits() as u32
        );
        // A small form is worth reading in full: the two sides of a
        // residual that does not cancel are usually a few dozen terms.
        if !frozen && e.num.terms().len() <= EXPLAIN_RENDER_TERMS {
            let text = render_form(sess, &e);
            let cut = text
                .char_indices()
                .nth(RENDER_CHARS.get())
                .map_or(text.len(), |(i, _)| i);
            lines.push_str(&format!(
                "{pad}  = {}{}\n",
                &text[..cut],
                if cut < text.len() { "…" } else { "" }
            ));
        }
        out.put(&lines);
        if depth < levels {
            for k in node.kids().iter().rev() {
                stack.push((*k, depth + 1));
            }
        }
    }
    out.out
}

/// The size of a form — this report's and the cost profile's one
/// spelling of it.
pub(super) fn size_of(f: &Form) -> FormSize {
    FormSize {
        num: (f.num.terms().len(), f.num.degree()),
        den: (f.den.terms().len(), f.den.degree()),
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
        let plain = render_form(sess, &f);
        let (early, early_size) = if sess.rules.early {
            let e = early_form(sess, id);
            (Some(render_form(sess, &e)), size_of(&e))
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

/// **The most bytes the report renders for ONE decision**, over
/// everything it writes for it: the plain form, the early form and the
/// explanation each take a third ([`TEXT_BUDGET`]). The depth limit
/// alone bounds nothing: a form's atoms each spell their own argument,
/// so a text's size is the PRODUCT of the fan-out over the levels.
pub const RENDER_BUDGET: usize = 3 * TEXT_BUDGET;

/// One text's share of [`RENDER_BUDGET`].
const TEXT_BUDGET: usize = 256 * 1024;

/// What a text keeps back from its content for the marks that say what
/// was left out: the two tails and the quotient's frame of a form cut
/// by terms, or the explanation's closing line.
const TAIL_RESERVE: usize = 160;

/// A text that takes no byte past its limit: content stops at
/// `limit - TAIL_RESERVE` and stays stopped (`dry`), and only the marks
/// naming what was left out go in the reserve.
struct Ink {
    out: String,
    limit: usize,
    dry: bool,
}

impl Ink {
    fn new(limit: usize) -> Self {
        Self {
            out: String::new(),
            limit,
            dry: false,
        }
    }

    /// Content: written whole or not at all, and never after a refusal.
    fn put(&mut self, s: &str) {
        if self.dry || self.out.len() + s.len() + TAIL_RESERVE > self.limit {
            self.dry = true;
        } else {
            self.out.push_str(s);
        }
    }

    /// A mark in the reserve.
    fn mark(&mut self, s: &str) {
        let fits = self.out.len() + s.len() <= self.limit;
        debug_assert!(fits, "the tail reserve holds every mark of one text");
        if fits {
            self.out.push_str(s);
        }
    }
}

/// Where a [`Pen`] writes: a text, or a count of the bytes the text
/// would take, with each atom's size at each depth counted once.
enum Sink {
    Write(Ink),
    Measure {
        bytes: usize,
        atoms: BTreeMap<(u128, usize), usize>,
    },
}

/// One rendering of a form: atoms spell their arguments for `spell`
/// levels, and below that an atom is `name(…)` when `spell` is the
/// full [`DEPTH`] and is NAMED (head, node, size) when the budget
/// lowered it.
struct Pen<'a> {
    sess: &'a Session,
    spell: usize,
    sink: Sink,
}

impl Pen<'_> {
    fn put(&mut self, s: &str) {
        match &mut self.sink {
            Sink::Write(ink) => ink.put(s),
            Sink::Measure { bytes, .. } => *bytes = bytes.saturating_add(s.len()),
        }
    }

    /// A quotient's own punctuation, which a text cut by terms still
    /// carries.
    fn frame(&mut self, s: &str) {
        self.put(s);
        if let Sink::Write(ink) = &mut self.sink
            && ink.dry
        {
            ink.mark(s);
        }
    }

    fn form(&mut self, f: &Form, depth: usize) {
        if f.poisoned {
            self.put("⊥");
        } else if f.den == Poly::one() {
            self.poly(&f.num, depth);
        } else {
            self.frame("(");
            self.poly(&f.num, depth);
            self.frame(") / (");
            self.poly(&f.den, depth);
            self.frame(")");
        }
    }

    fn poly(&mut self, p: &Poly, depth: usize) {
        if p.is_zero() {
            self.put("0");
            return;
        }
        let terms = p.terms();
        for (i, (m, c)) in terms.iter().enumerate() {
            let start = match &self.sink {
                Sink::Write(ink) => ink.out.len(),
                Sink::Measure { .. } => 0,
            };
            if i > 0 {
                self.put(" + ");
            }
            self.put(&render_rat(c));
            for &(id, e) in m {
                self.put("·");
                self.indet(id, depth);
                if e != 1 {
                    self.put(&format!("^{e}"));
                }
            }
            if let Sink::Write(ink) = &mut self.sink
                && ink.dry
            {
                // The term that did not fit goes whole, and the rest of
                // the polynomial is counted, not spelled.
                ink.out.truncate(start);
                let left = terms.len() - i;
                ink.mark(&match i {
                    0 => format!("‹{left} terms›"),
                    _ => format!(" + ‹{left} more terms›"),
                });
                return;
            }
        }
    }

    fn indet(&mut self, id: u128, depth: usize) {
        if id == INDET_PI {
            return self.put("π");
        }
        if let Some(name) = NAMES.with(|n| n.borrow().get(&id).cloned()) {
            return self.put(&name);
        }
        let Some(atom) = self.sess.atoms.get(&id) else {
            return self.put(&format!("?#{:08x}", id as u32));
        };
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
            SymOp::LessSin => "less_sin",
            SymOp::Atan2 => "atan2",
            SymOp::Min => "min",
            SymOp::Max => "max",
            SymOp::Copysign => "copysign",
            SymOp::Select => "select",
            _ => "?",
        };
        self.put(name);
        if depth >= self.spell {
            if self.spell == DEPTH {
                return self.put("(…)");
            }
            let terms: usize = atom
                .args
                .iter()
                .flatten()
                .map(|a| a.num.terms().len() + a.den.terms().len())
                .sum();
            return self.put(&format!("#{:08x}‹{terms} terms›", id as u32));
        }
        if let Sink::Measure { bytes, atoms } = &mut self.sink
            && let Some(&known) = atoms.get(&(id, depth))
        {
            *bytes = bytes.saturating_add(known);
            return;
        }
        let before = match &self.sink {
            Sink::Measure { bytes, .. } => *bytes,
            Sink::Write(_) => 0,
        };
        self.put("(");
        for (i, a) in atom.args.iter().flatten().enumerate() {
            if i > 0 {
                self.put(", ");
            }
            self.form(a, depth + 1);
        }
        self.put(")");
        if let Sink::Measure { bytes, atoms } = &mut self.sink {
            atoms.insert((id, depth), bytes.saturating_sub(before));
        }
    }
}

/// How many bytes `f` takes rendered with `spell` levels of atoms
/// spelled (saturating).
fn measure(sess: &Session, f: &Form, spell: usize) -> usize {
    let mut pen = Pen {
        sess,
        spell,
        sink: Sink::Measure {
            bytes: 0,
            atoms: BTreeMap::new(),
        },
    };
    pen.form(f, 0);
    match pen.sink {
        Sink::Measure { bytes, .. } => bytes,
        Sink::Write(_) => 0,
    }
}

/// `f` as text, within [`TEXT_BUDGET`] and a function of the form
/// alone: spelled to the deepest level (at most [`DEPTH`]) at which
/// the whole text fits, and where it fits at none, with every atom
/// named and the terms past the budget counted.
fn render_form(sess: &Session, f: &Form) -> String {
    let room = TEXT_BUDGET - TAIL_RESERVE;
    let spell = (0..=DEPTH)
        .rev()
        .find(|&spell| measure(sess, f, spell) <= room)
        .unwrap_or(0);
    let mut pen = Pen {
        sess,
        spell,
        sink: Sink::Write(Ink::new(TEXT_BUDGET)),
    };
    pen.form(f, 0);
    match pen.sink {
        Sink::Write(ink) => ink.out,
        Sink::Measure { .. } => String::new(),
    }
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

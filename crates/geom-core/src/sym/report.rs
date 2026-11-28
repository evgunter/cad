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
//!
//! What it writes for one decision is bounded (`RENDER_BUDGET`): a
//! form too large for its share is spelled less deep, its unspelled
//! atoms named and its unwritten terms counted.

use core::cell::{Cell, RefCell};
use std::collections::BTreeMap;

use super::form::{Form, Mono, Poly};
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
    /// outside a session. This text, [`Self::early_form`] and
    /// [`Self::explain`] together take at most [`RENDER_BUDGET`] bytes.
    /// A form too large to spell to the full depth NAMES the atoms it
    /// leaves unspelled, `sqrt@89abcdef‹12 terms›` (the head, the
    /// atom's id, its argument's terms: an atom id, under `@`, is not
    /// one of the node ids [`Self::explain`] writes under `#`), and
    /// one too wide even so counts the terms it leaves,
    /// `‹40 more terms›`. A count of terms is always one polynomial's.
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
    static SHAPES: RefCell<Vec<DecisionShape>> = const { RefCell::new(Vec::new()) };
    static NAMES: RefCell<BTreeMap<u128, String>> = const { RefCell::new(BTreeMap::new()) };
}

/// Installs the report on this thread, dropping anything recorded.
pub fn start_shape_report() {
    SHAPES.with(|s| s.borrow_mut().clear());
    ACTIVE.set(true);
}

/// Removes the report and answers everything recorded since
/// [`start_shape_report`]. [`explain_depth`] stays as set: it is a
/// dial a harness sets once, before the replays that each install a
/// report of their own.
pub fn take_shape_report() -> Vec<DecisionShape> {
    ACTIVE.set(false);
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

/// The largest numerator (terms) whose form [`explain`] writes beside
/// its node; a wider one is described by its size alone.
const EXPLAIN_FORM_TERMS: usize = 80;

/// The most bytes of one node's form [`explain`] writes: a page, and
/// room for every text of 1500 characters.
const EXPLAIN_FORM_BYTES: usize = 4500;

/// The DAG below `root` to `levels` deep, one line per node: its op,
/// its early form's size, and `FROZEN` where the early memo holds the
/// node's own indeterminate — the shape of a refusal, read off the
/// tree rather than guessed from the residual. Nodes are visited in
/// child order and a node reached twice is rendered once. The text
/// takes at most [`TEXT_BUDGET`]: where a node's lines do not fit it
/// ends, with a line naming that node and the length of the walk's
/// stack there.
fn explain(sess: &mut Session, root: SymId, levels: usize) -> String {
    // The walk's stack holds a node once per parent that pushed it, a
    // node already rendered among them, and nothing below a node not
    // yet reached: its length is not a count of the nodes left.
    let ends_before = |id: SymId, stack: usize| {
        format!(
            "‹the explanation ends before node #{:08x}; the walk's stack held {stack} more›\n",
            id.bits() as u32
        )
    };
    let mut out = Ink::new(TEXT_BUDGET);
    let mut seen: super::IdMap<()> = super::IdMap::default();
    let mut stack: Vec<(SymId, usize)> = Vec::new();
    let mut next_unseen = |stack: &mut Vec<(SymId, usize)>| {
        while let Some((id, depth)) = stack.pop() {
            if seen.insert(id, ()).is_none() {
                return Some((id, depth));
            }
        }
        None
    };
    stack.push((root, 0));
    let mut at = next_unseen(&mut stack);
    // The line that ends the text before the node at hand, set aside
    // before that node is written.
    let Some(mut end) = out.reserve(ends_before(root, 0)) else {
        return out.finish();
    };
    while let Some((id, depth)) = at {
        let lines = explain_node(sess, id, depth);
        if depth < levels
            && let Some(node) = sess.node_of(id)
        {
            stack.extend(node.kids().iter().rev().map(|k| (*k, depth + 1)));
        }
        at = next_unseen(&mut stack);
        let after = match at {
            None => None,
            Some((following, _)) => match out.reserve(ends_before(following, stack.len())) {
                Some(slot) => Some(slot),
                None => break,
            },
        };
        if !out.put(&lines) {
            if let Some(slot) = after {
                out.release(slot);
            }
            break;
        }
        out.release(end);
        match after {
            Some(slot) => end = slot,
            None => return out.finish(),
        }
    }
    out.write(end);
    out.finish()
}

/// One node's lines of the explanation.
fn explain_node(sess: &mut Session, id: SymId, depth: usize) -> String {
    let pad = "  ".repeat(depth);
    let Some(node) = sess.node_of(id) else {
        return format!("{pad}[unrecorded #{:08x}]\n", id.bits() as u32);
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
    if !frozen && e.num.terms().len() <= EXPLAIN_FORM_TERMS {
        let text = render_form(sess, &e, EXPLAIN_FORM_BYTES);
        lines.push_str(&format!("{pad}  = {text}\n"));
    }
    lines
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
        let plain = render_form(sess, &f, TEXT_BUDGET);
        let (early, early_size) = if sess.rules.early {
            let e = early_form(sess, id);
            (Some(render_form(sess, &e, TEXT_BUDGET)), size_of(&e))
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

/// How many atoms deep an argument is spelled; an atom below that is
/// `name(…)`. A limit on nesting alone: every atom spells its argument
/// at every site, so the text of a form this deep is the product of
/// the fan-out over the levels, and [`render_form`] is what bounds it.
const DEPTH: usize = 4;

/// **The most bytes the report writes for one decision**: its plain
/// form, its early form and its explanation each take at most a third
/// ([`TEXT_BUDGET`]).
///
/// Thirds, and not one pool the three draw from, so that each text is
/// a function of its own form: [`render_of`] of a node and the
/// [`DecisionShape::form`] of a decision on it are the same bytes,
/// whatever the other two texts took. The cost is a share one text
/// leaves unused and no other can borrow.
pub const RENDER_BUDGET: usize = 3 * TEXT_BUDGET;

/// One text's share of [`RENDER_BUDGET`]. A ceiling, not a fill line:
/// a form is spelled one whole level shallower at a time
/// ([`render_form`]), so a text just past the share falls well under
/// it.
///
/// Sized on the documents the report's readers replay (`editor-core`'s
/// `*_interval` suites, 1,692 rendered decisions): the median text is
/// 635 bytes, nineteen in twenty are under 18 kB, and the share leaves
/// all but 35 of them (one printing row's) spelled to the full depth.
/// Nothing re-takes those figures and nothing has to: a text past the
/// share is shortened and says so, and a row that reads one sees it.
const TEXT_BUDGET: usize = 256 * 1024;

use ink::Ink;

mod ink {
    /// **A text that takes no byte past its limit.** Content goes in
    /// whole or not at all ([`Ink::put`]). A MARK, the text that says
    /// what was left out, is set aside before the content it may have
    /// to stand for is tried ([`Ink::reserve`]): its bytes are counted
    /// from then on, so writing it later ([`Ink::write`]) has no way
    /// to fail.
    pub(super) struct Ink {
        out: String,
        limit: usize,
        /// The bytes of the marks set aside and neither written nor
        /// released: `out.len() + aside <= limit`.
        aside: usize,
    }

    /// A mark set aside in an [`Ink`].
    pub(super) struct Slot(String);

    impl Ink {
        pub(super) fn new(limit: usize) -> Self {
            Self {
                out: String::new(),
                limit,
                aside: 0,
            }
        }

        fn fits(&self, bytes: usize) -> bool {
            self.out.len() + self.aside + bytes <= self.limit
        }

        /// Writes `s` whole if it fits beside the marks set aside, and
        /// says whether it did.
        pub(super) fn put(&mut self, s: &str) -> bool {
            let fits = self.fits(s.len());
            if fits {
                self.out.push_str(s);
            }
            fits
        }

        /// Sets `mark` aside, if it fits beside the others.
        pub(super) fn reserve(&mut self, mark: String) -> Option<Slot> {
            self.fits(mark.len()).then(|| {
                self.aside += mark.len();
                Slot(mark)
            })
        }

        /// Writes a mark set aside.
        pub(super) fn write(&mut self, Slot(mark): Slot) {
            self.aside -= mark.len();
            self.out.push_str(&mark);
        }

        /// Gives back the room of a mark that was not needed.
        pub(super) fn release(&mut self, Slot(mark): Slot) {
            self.aside -= mark.len();
        }

        pub(super) fn len(&self) -> usize {
            self.out.len()
        }

        pub(super) fn finish(self) -> String {
            self.out
        }
    }
}

/// Where a [`Pen`] writes.
trait Sink {
    fn put(&mut self, s: &str);

    /// The bytes taken so far.
    fn len(&self) -> usize;

    /// Takes the atom at `key` (its id and the depth it stands at)
    /// from an earlier pass over it, where this sink keeps one.
    fn recall(&mut self, _key: (u128, usize)) -> bool {
        false
    }

    /// Keeps the atom at `key`, which began at `from`.
    fn remember(&mut self, _key: (u128, usize), _from: usize) {}
}

/// The bytes a text would take, each atom walked once per depth: an
/// atom's text at a depth is the same at every site.
#[derive(Default)]
struct Count {
    bytes: usize,
    atoms: BTreeMap<(u128, usize), usize>,
}

impl Sink for Count {
    fn put(&mut self, s: &str) {
        self.bytes = self.bytes.saturating_add(s.len());
    }

    fn len(&self) -> usize {
        self.bytes
    }

    fn recall(&mut self, key: (u128, usize)) -> bool {
        let known = self.atoms.get(&key).copied();
        if let Some(bytes) = known {
            self.bytes = self.bytes.saturating_add(bytes);
        }
        known.is_some()
    }

    fn remember(&mut self, key: (u128, usize), from: usize) {
        self.atoms.insert(key, self.bytes.saturating_sub(from));
    }
}

/// A text whose size was counted first: `whole` goes false if a piece
/// did not fit after all.
struct Counted {
    ink: Ink,
    whole: bool,
}

impl Sink for Counted {
    fn put(&mut self, s: &str) {
        self.whole &= self.ink.put(s);
    }

    fn len(&self) -> usize {
        self.ink.len()
    }
}

/// One term of a polynomial with no atom spelled, which is as large as
/// the term and no larger.
impl Sink for String {
    fn put(&mut self, s: &str) {
        self.push_str(s);
    }

    fn len(&self) -> usize {
        str::len(self)
    }
}

/// `n` terms of ONE polynomial, in words: the count every mark gives.
fn terms(n: usize) -> String {
    match n {
        1 => "1 term".to_owned(),
        _ => format!("{n} terms"),
    }
}

/// The mark that stands for the `left` terms after the last one written.
fn more(left: usize) -> String {
    match left {
        1 => " + ‹1 more term›".to_owned(),
        _ => format!(" + ‹{left} more terms›"),
    }
}

/// The mark a polynomial none of whose terms is written leaves.
fn unwritten(p: &Poly) -> String {
    if p.is_zero() {
        "0".to_owned()
    } else {
        format!("‹{}›", terms(p.terms().len()))
    }
}

/// One rendering of a form. Atoms spell their arguments for `spell`
/// levels. Below that an atom is `name(…)` when `spell` is the whole
/// [`DEPTH`], and where the budget lowered it the atom is NAMED:
/// `sqrt@89abcdef‹12 terms›` is its head, its ATOM id and its
/// argument's size.
///
/// An atom id (`@`) is the low word of the atom's indeterminate, a hash
/// of its head and its arguments' forms: two sites naming one id hold
/// one atom. It is not a DAG node's id, which the explanation writes
/// under `#`, and no table leads from one to the other. An argument's
/// size is its polynomial's terms, `n terms / d terms` for a quotient,
/// and one size per argument.
struct Pen<'a, S> {
    sess: &'a Session,
    spell: usize,
    sink: S,
}

impl<S: Sink> Pen<'_, S> {
    fn form(&mut self, f: &Form, depth: usize) {
        if f.poisoned {
            self.sink.put("⊥");
        } else if f.den == Poly::one() {
            self.poly(&f.num, depth);
        } else {
            self.sink.put("(");
            self.poly(&f.num, depth);
            self.sink.put(") / (");
            self.poly(&f.den, depth);
            self.sink.put(")");
        }
    }

    fn poly(&mut self, p: &Poly, depth: usize) {
        if p.is_zero() {
            self.sink.put("0");
        }
        for (i, (m, c)) in p.terms().iter().enumerate() {
            if i > 0 {
                self.sink.put(" + ");
            }
            self.term(m, c, depth);
        }
    }

    fn term(&mut self, m: &Mono, c: &Rat, depth: usize) {
        self.sink.put(&render_rat(c));
        for &(id, e) in m {
            self.sink.put("·");
            self.indet(id, depth);
            if e != 1 {
                self.sink.put(&format!("^{e}"));
            }
        }
    }

    fn indet(&mut self, id: u128, depth: usize) {
        if id == INDET_PI {
            return self.sink.put("π");
        }
        if let Some(name) = NAMES.with(|n| n.borrow().get(&id).cloned()) {
            return self.sink.put(&name);
        }
        let sess = self.sess;
        let Some(atom) = sess.atoms.get(&id) else {
            return self.sink.put(&format!("?#{:08x}", id as u32));
        };
        self.sink.put(match atom.op {
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
        });
        let args = atom.args.iter().flatten();
        if depth >= self.spell {
            if self.spell == DEPTH {
                return self.sink.put("(…)");
            }
            let sizes: Vec<String> = args
                .map(|a| match (a.poisoned, a.den == Poly::one()) {
                    (true, _) => "⊥".to_owned(),
                    (false, true) => terms(a.num.terms().len()),
                    (false, false) => format!(
                        "{} / {}",
                        terms(a.num.terms().len()),
                        terms(a.den.terms().len())
                    ),
                })
                .collect();
            return self
                .sink
                .put(&format!("@{:08x}‹{}›", id as u32, sizes.join(", ")));
        }
        if self.sink.recall((id, depth)) {
            return;
        }
        let from = self.sink.len();
        self.sink.put("(");
        for (i, a) in args.enumerate() {
            if i > 0 {
                self.sink.put(", ");
            }
            self.form(a, depth + 1);
        }
        self.sink.put(")");
        self.sink.remember((id, depth), from);
    }
}

/// **`f` as text of at most `limit` bytes**, a function of the form
/// and the registered names alone.
///
/// It is spelled to the deepest level, [`DEPTH`] at most, at which the
/// whole text fits, each candidate counted before it is written. Where
/// it fits at none, every atom is named and each polynomial's terms
/// are written while they fit, the rest counted ([`cut_form`]).
fn render_form(sess: &Session, f: &Form, limit: usize) -> String {
    for spell in (0..=DEPTH).rev() {
        let mut count = Pen {
            sess,
            spell,
            sink: Count::default(),
        };
        count.form(f, 0);
        if count.sink.bytes > limit {
            continue;
        }
        let mut pen = Pen {
            sess,
            spell,
            sink: Counted {
                ink: Ink::new(limit),
                whole: true,
            },
        };
        pen.form(f, 0);
        debug_assert!(pen.sink.whole, "a text takes the bytes it was counted at");
        if pen.sink.whole {
            return pen.sink.ink.finish();
        }
    }
    cut_form(sess, f, limit)
}

/// `f` with every atom named and its terms written while they fit:
/// `(… + ‹244 more terms›) / (‹436 terms›)`. Every mark the text can
/// come to owe is set aside before a term is tried. A limit under the
/// length of those marks, a hundred bytes or so and neither limit
/// here, keeps the ones that fit, in order.
fn cut_form(sess: &Session, f: &Form, limit: usize) -> String {
    let mut ink = Ink::new(limit);
    if f.poisoned {
        ink.put("⊥");
        return ink.finish();
    }
    let Some(num) = ink.reserve(unwritten(&f.num)) else {
        return ink.finish();
    };
    if f.den == Poly::one() {
        cut_poly(sess, &mut ink, &f.num, num);
        return ink.finish();
    }
    let rest = (|| {
        Some((
            ink.reserve(") / (".to_owned())?,
            ink.reserve(unwritten(&f.den))?,
            ink.reserve(")".to_owned())?,
        ))
    })();
    let Some((over, den, close)) = rest else {
        ink.write(num);
        return ink.finish();
    };
    if !ink.put("(") {
        ink.write(num);
        return ink.finish();
    }
    cut_poly(sess, &mut ink, &f.num, num);
    ink.write(over);
    cut_poly(sess, &mut ink, &f.den, den);
    ink.write(close);
    ink.finish()
}

/// `p`'s terms, atoms named, while each fits beside the mark that
/// counts the terms after it; `unwritten` is the mark for the whole of
/// `p`, set aside by the caller.
fn cut_poly(sess: &Session, ink: &mut Ink, p: &Poly, unwritten: ink::Slot) {
    let terms_of = p.terms();
    let mut owed = unwritten;
    for (i, (m, c)) in terms_of.iter().enumerate() {
        let mut term = Pen {
            sess,
            spell: 0,
            sink: if i > 0 {
                " + ".to_owned()
            } else {
                String::new()
            },
        };
        term.term(m, c, 0);
        let left = terms_of.len() - i - 1;
        let after = if left == 0 {
            None
        } else {
            match ink.reserve(more(left)) {
                Some(slot) => Some(slot),
                None => break,
            }
        };
        if !ink.put(&term.sink) {
            if let Some(slot) = after {
                ink.release(slot);
            }
            break;
        }
        ink.release(owed);
        match after {
            Some(slot) => owed = slot,
            None => return,
        }
    }
    ink.write(owed);
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

/// Probe (M16b): a form's text inside an already-borrowed session.
pub(super) fn m16_render(sess: &Session, f: &Form) -> String {
    render_form(sess, f, 1500)
}

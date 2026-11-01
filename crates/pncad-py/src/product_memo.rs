//! **The gathered product, memoized on an evaluation.**
//!
//! A Python `Evaluation` is the immutable (document, evaluation) pair
//! captured at `evaluate`, and a document's product is a pure function
//! of that pair and the run's tolerance. Three bound doors want one —
//! `run_checks`, `product` and `product_named` — and this module is
//! where that answer is computed once and kept, so a caller asking
//! several questions of one evaluation pays for one gather.
//!
//! **The memo is OF the evaluation, and gathers from the document the
//! evaluation captured.** A Python `Doc` is mutable and its identity
//! survives every edit, so a product gathered from the `doc` argument
//! would answer about whatever that handle held at the moment of the
//! first ask — which is a memo that can go stale. Gathering from the
//! captured half of the pair is what makes the memo's key the
//! evaluation itself; the `doc` argument is read for the DI3 pairing
//! and nothing else.
//!
//! **Python-independent on purpose.** Everything here is ordinary Rust
//! over the `pncad` façade, so the default (no-`python`) build compiles
//! and TESTS it — which is where the gather counts below are pinned,
//! against `pncad::document::gathers_on_this_thread`. The
//! `#[pyfunction]`s in [`crate::py`] are the argument crossing and this
//! module is the behaviour.

use std::sync::OnceLock;

use pncad::document as d;
use pncad::tolerance::{Tol, Tolerance};
use pncad::topo;

/// One evaluation's gathered product, materialized on first ask.
///
/// `OnceLock` is the interior mutability this crate already uses for a
/// materialize-once handle (`py::pick::NodePick`'s mesh), and one slot
/// is all the memo needs: the product it holds is the answer for the
/// tolerance recorded beside it, and a run commits exactly one
/// tolerance ([`Tol`] is the witness of that commitment).
#[derive(Debug, Default)]
pub struct ProductMemo {
    slot: OnceLock<Gathered>,
}

/// The held product and the tolerance it was gathered at.
///
/// The tolerance is carried because it is a PRECONDITION of reuse, not
/// because it varies today: a gather decides against ε and K, so a
/// product gathered at one tolerance is not the answer at another.
#[derive(Debug)]
struct Gathered {
    at: Tolerance,
    product: d::Product<f64>,
}

impl ProductMemo {
    /// **The DI3 pairing gate, asked before the memo is consulted**:
    /// `asked` must be the document `evaluation` is of.
    ///
    /// The gather checks this for itself, which is enough while every
    /// door gathers; a door that answers from a memo instead reaches
    /// no gather, so the check is stated here and each caller wraps
    /// [`d::Mispaired`] in its own typed refusal — the same one the
    /// gather would have raised, under the same tag.
    pub fn paired(
        evaluation: &d::Evaluation<f64>,
        asked: d::DocumentId,
    ) -> Result<(), d::Mispaired> {
        if asked == evaluation.document {
            return Ok(());
        }
        Err(d::Mispaired {
            expected: asked,
            found: evaluation.document,
        })
    }

    /// The product of `(doc, evaluation)` at `tol`, gathered on the
    /// first ask and kept for every later one, handed to `f` by
    /// reference so each door copies out only what it needs.
    ///
    /// `doc` is the evaluation's OWN document (module docs); the
    /// caller's document is what [`Self::paired`] reads.
    ///
    /// A gather that REFUSES is not memoized: the refusal carries no
    /// product to keep, so a door that refuses re-gathers next time —
    /// never worse than a façade that gathered every time.
    pub fn with<R>(
        &self,
        doc: &d::ProfileDoc,
        evaluation: &d::Evaluation<f64>,
        tol: Tol,
        f: impl FnOnce(&d::Product<f64>) -> R,
    ) -> Result<R, d::ProductError> {
        self.with_at(doc, evaluation, tol.get(), tol, f)
    }

    /// [`Self::with`] with the memo key supplied rather than read off
    /// the run — the seam the tolerance-keying test drives, because a
    /// process commits one tolerance and cannot offer a second.
    pub(crate) fn with_at<R>(
        &self,
        doc: &d::ProfileDoc,
        evaluation: &d::Evaluation<f64>,
        at: Tolerance,
        tol: Tol,
        f: impl FnOnce(&d::Product<f64>) -> R,
    ) -> Result<R, d::ProductError> {
        if let Some(held) = self.slot.get() {
            if held.at == at {
                return Ok(f(&held.product));
            }
            // A different tolerance is a different question. The slot
            // keeps the entry it has and this answer is not kept.
            return Ok(f(&d::product_recorded(doc, evaluation, tol)?));
        }
        let product = d::product_recorded(doc, evaluation, tol)?;
        let held = self.slot.get_or_init(|| Gathered { at, product });
        if held.at == at {
            return Ok(f(&held.product));
        }
        // Another thread filled the slot at a tolerance this one is not
        // asking at. Unreachable while a run commits one tolerance, and
        // answered rather than assumed away.
        Ok(f(&d::product_recorded(doc, evaluation, tol)?))
    }
}

/// **`run_checks`'s body**: the registry over the memoized product.
///
/// The wrapper's laziness is kept and it is the point — a
/// configuration whose enabled residents all read the evaluation has
/// nothing to gather FOR, so a subject-free run must not fill the
/// memo. So is its refusal posture: a gather that refuses is a
/// [`d::Subject`] the residents report over, never an error, because
/// only a resident that reads the subject is entitled to complain
/// about it.
pub fn checks_report(
    memo: &ProductMemo,
    doc: &d::ProfileDoc,
    evaluation: &d::Evaluation<f64>,
    cfg: &d::ChecksConfig,
    tol: Tol,
) -> Result<d::ChecksReport, d::ChecksError> {
    if !cfg.needs_a_subject() {
        // The kernel wrapper's own lazy path, which gathers nothing —
        // reached through the wrapper rather than re-spelled, because
        // the sentence a subject-free run reports under is the
        // registry's to word.
        return d::run_checks(doc, evaluation, cfg, tol);
    }
    // The gather's refusal is a SUBJECT the residents report over, not
    // an error, and WHICH subject it is belongs to `Subject::refused`:
    // the empty-document class is a subject that reports clean, every
    // other class one the subject-reading residents refuse over. This
    // caller hands the error over whole rather than drawing that line
    // a second time.
    match memo.with(doc, evaluation, tol, |product| {
        d::run_checks_on(doc, evaluation, d::Subject::Product(product), cfg, tol)
    }) {
        Ok(report) => report,
        Err(source) => d::run_checks_on(doc, evaluation, d::Subject::refused(source), cfg, tol),
    }
}

/// **`product`'s body**: the memoized product's aggregate body, as it
/// stands in the world ([`pncad::export::product_body_in_space`]).
pub fn body(
    memo: &ProductMemo,
    doc: &d::ProfileDoc,
    evaluation: &d::Evaluation<f64>,
    tol: Tol,
) -> Result<topo::Body<f64>, d::ProductError> {
    memo.with(doc, evaluation, tol, |product| standing(product, doc, tol))?
}

/// The product's body composed into the world through the façade's
/// read-out door, which builds the gather's `Frame` refusal itself.
fn standing(
    product: &d::Product<f64>,
    doc: &d::ProfileDoc,
    tol: Tol,
) -> Result<topo::Body<f64>, d::ProductError> {
    pncad::export::product_body_in_space(product, doc, tol).map(std::borrow::Cow::into_owned)
}

/// **`product_named`'s body**: the aggregate as it stands in the world
/// and the names its entities answer to, in the product's own table
/// order.
pub fn body_and_names(
    memo: &ProductMemo,
    doc: &d::ProfileDoc,
    evaluation: &d::Evaluation<f64>,
    tol: Tol,
) -> Result<(topo::Body<f64>, Vec<pncad::prelude::StableName>), d::ProductError> {
    memo.with(doc, evaluation, tol, |product| {
        Ok((
            standing(product, doc, tol)?,
            product.names.iter().map(|(name, _)| name.clone()).collect(),
        ))
    })?
}

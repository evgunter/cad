//! **How far a field can move before something breaks**, as the
//! session asks it: the field a probe is taken for, the reading it
//! answers with, and the search itself.
//!
//! A VOCABULARY over [`crate::bounds`]'s search. Every function here
//! takes the document it searches as an argument rather than reading a
//! session — the property [`probe_scale`]'s note argues for, extended
//! to the whole probe: one document answers where the search starts,
//! what it steps by, and what every candidate is judged against.

use std::sync::Arc;

use pncad::document::{
    CancelToken, Dimension, Doc, DocEdit, DocParam, EvalOptions, Evaluation, ParamName,
    PartResolver, ProfileProgram, RecipeNodeId, SlotId, apply, evaluate,
};
use pncad::geom_core::Tol;
use pncad::quantity::UnitDef;

use crate::bounds;
use crate::props::{self, SlotValue};
use crate::session::refuse::Refusal;

/// **One locally-valid-range probe's answer**, with everything a panel
/// needs to say it: the field it was taken for, the range found, and
/// the NOTATION the search ran in.
///
/// The unit is carried rather than looked up again at the reading, and
/// that is the whole reason this is a struct rather than a pair. The
/// probe seeds one step of the unit the field is WRITTEN in
/// ([`probe_seed`]), so a reading in any other unit
/// describes a search that did not happen — it would say metres about
/// a range found in millimetres. A panel that re-read the unit off the
/// row it is drawing would agree with the search only for as long as
/// nothing came between the two reads; carrying it makes the agreement
/// the same value read twice.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundsReading {
    /// The field the probe was taken for.
    pub target: BoundsTarget,
    /// The range the search established.
    pub bounds: bounds::Bounds,
    /// The unit the search ran in — one of it is the probe's step.
    /// `None` is the field that names no notation at all (a count, a
    /// bare scalar), whose step is 1.
    pub unit: Option<UnitDef>,
}

impl BoundsReading {
    /// The reading as one line, in the unit the search used — the one
    /// place a probe's result becomes a sentence, for a slot field and
    /// a document parameter's alike.
    pub fn wording(&self) -> String {
        self.bounds.wording(self.unit)
    }
}

/// The field a locally-valid-range probe was taken for.
///
/// Two arms rather than one with an `Option`, for the reason
/// `BeginGesture` and `BeginParamGesture` are two doors: a slot and a
/// document parameter are addressed differently, and collapsing them
/// puts an `Option` in every arm that reads one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundsTarget {
    /// A node's named slot.
    Slot {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// A document parameter.
    Param {
        /// The parameter.
        name: ParamName,
    },
}

/// Run one locally-valid-range probe, inline.
///
/// The oracle is the whole of what "valid" means here: apply the
/// candidate value to the shown document, evaluate, and ask whether
/// the failing set grew ([`bounds::Verdict::no_worse_than`]). An
/// edit the door itself REFUSES — a profile program that stops
/// being a legal walk, a non-finite literal — counts as invalid
/// without an evaluation, which is the same answer for the same
/// reason: at that value the document does not stand.
///
/// Every sample runs against the landed evaluation as its memo, so
/// it re-runs the edited node's downstream cone rather than the
/// whole recipe.
pub(super) fn probe_bounds(
    base: &Doc<ProfileProgram>,
    target: BoundsTarget,
    prior: Option<&Evaluation<f64>>,
    resolver: &Option<Arc<dyn PartResolver>>,
    tol: Tol,
) -> Result<BoundsReading, Refusal> {
    let (origin, unit, integral) = probe_scale(base, &target)?;
    let seed = probe_seed(unit);
    // The baseline is taken at the value the field HAS, from the
    // same oracle every sample goes through — so "no worse than the
    // baseline" compares two runs of one function rather than a run
    // against the landed evaluation, which may have been taken at a
    // different memo state.
    let baseline = bounds::Verdict::of(&evaluate_with(base, prior, resolver, tol));
    let result = bounds::probe(
        bounds::BoundsProbe::new(origin, seed, integral),
        |candidate| {
            let Some(edit) = probe_edit(base, &target, candidate) else {
                return false;
            };
            match apply(base, &edit, tol) {
                Ok(applied) => {
                    let eval = evaluate_with(&applied.doc, prior, resolver, tol);
                    bounds::Verdict::of(&eval).no_worse_than(&baseline)
                }
                // The edit door refused: at this value there is no
                // document to evaluate, which is as invalid as a
                // value gets.
                Err(_) => false,
            }
        },
    );
    // The unit stored here is the one `probe_seed` above stepped
    // by: the reading and the search are one value read twice.
    Ok(BoundsReading {
        target,
        bounds: result,
        unit,
    })
}

/// One written `unit`, in canonical terms — the probe's step, and
/// the one place that arithmetic is spelled, so a slot's seed and a
/// parameter's seed are the same answer to the same question.
/// `None` is the field that names no unit at all (a count, a bare
/// scalar), whose step is 1.
fn probe_seed(unit: Option<UnitDef>) -> f64 {
    unit.map_or(1.0, |unit| props::from_written(1.0, unit))
}

/// The probe's three inputs, read off the field **in `doc`**: where
/// it is now, the unit it is written in, and whether its answer is
/// an integer.
///
/// The unit rather than the step, so that the number the search
/// walks by ([`probe_seed`]) and the number the reading is
/// written in ([`BoundsReading::unit`]) come from one answer to one
/// question. One of that unit is the step — one millimetre for a
/// field written in millimetres, one radian for one written
/// canonically, 1 for a count or a bare scalar. That is the scale a
/// user thinks in, which is the scale a range should be searched
/// and reported at.
///
/// **A free function over the document the probe searches**,
/// not a method that reads the session again: the two arms below
/// once read two different documents (the shown one and the
/// committed one), which agreed only because a probe refuses while
/// a gesture is in flight
/// ([`super::SessionOp::permitted_during_value_gesture`]). Taking the
/// document as an argument makes that agreement structural instead
/// of circumstantial.
fn probe_scale(
    doc: &Doc<ProfileProgram>,
    target: &BoundsTarget,
) -> Result<(f64, Option<UnitDef>, bool), Refusal> {
    match target {
        BoundsTarget::Slot { node, slot } => {
            let rows = props::slot_rows(doc, *node);
            // One refusal for both misses — the node does not carry
            // the slot, and the slot carries no readable value —
            // because a probe needs a place to search FROM and
            // neither case gives it one.
            let found = rows
                .into_iter()
                .find(|row| row.slot == *slot)
                .and_then(|row| Some((row.value.ok()?, row.dimension, row.unit)));
            let Some((value, dimension, remembered)) = found else {
                return Err(Refusal::NoSuchSlot {
                    node: *node,
                    slot: *slot,
                });
            };
            let value = value.as_f64();
            // Whatever unit the field is written in — through
            // `rendering_unit`, so a computed slot's step is the
            // same unit the panel shows it in rather than a second
            // answer to the same question.
            let unit = props::rendering_unit(dimension, remembered);
            Ok((value, unit, dimension == Dimension::Count))
        }
        BoundsTarget::Param { name } => {
            let Some(param) = doc.params().get(name) else {
                return Err(Refusal::NoSuchParam(name.clone()));
            };
            // Same rule as a slot's: one of whatever unit the
            // field is WRITTEN in. A continuous parameter names the
            // notation it was authored in
            // (`DocParam::Continuous::display_unit`, which rides
            // with the declaration and no value edit disturbs), so
            // a millimetre parameter is searched in millimetres. A
            // `Count` is a number rather than a quantity, has no
            // unit to name, and steps by 1.
            let (value, remembered) = match param {
                DocParam::Continuous {
                    value,
                    display_unit,
                    ..
                } => (*value, Some(display_unit.def())),
                DocParam::Count { value } => (*value as f64, None),
            };
            // Through `rendering_unit` for the slot arm's reason:
            // one function answers "what unit is this field written
            // in" for both fields, so the panel row and the probe
            // cannot come to two answers.
            let unit = props::rendering_unit(param.dim(), remembered);
            Ok((value, unit, param.dim() == Dimension::Count))
        }
    }
}

/// The edit that puts `value` into the probed field, or `None` when
/// the value cannot be expressed there at all.
fn probe_edit(
    doc: &Doc<ProfileProgram>,
    target: &BoundsTarget,
    value: f64,
) -> Option<DocEdit<ProfileProgram>> {
    match target {
        BoundsTarget::Slot { node, slot } => props::slot_edit(
            *node,
            *slot,
            SlotValue::of(slot.dimension(), value),
            props::slot_unit(doc, *node, *slot),
        )
        .ok(),
        BoundsTarget::Param { name } => {
            // The dimension is read off the DECLARATION only to
            // decide which `SlotValue` arm the sample becomes; the
            // edit itself carries a value and nothing else, so a
            // probe cannot disturb the parameter's declaration
            // (`props::param_edit`'s door).
            let dimension = doc.params().get(name)?.dim();
            Some(props::param_edit(
                name.clone(),
                SlotValue::of(dimension, value),
            ))
        }
    }
}

/// One evaluation of one document, outside the seam.
///
/// **The seam is for the PICTURE**; this is for a question asked about
/// a document nobody is going to look at (a range probe's candidate).
/// Routing it through the seam would cancel the run the viewport is
/// waiting for — the seam's ruled cancel-and-restart policy — which is
/// exactly the wrong trade for a query the user asked for BESIDE the
/// picture rather than instead of it.
///
/// A fresh `CancelToken` per call, never set: these runs are bounded by
/// the probe's sample cap, and nothing exists to cancel them from.
fn evaluate_with(
    doc: &Doc<ProfileProgram>,
    prior: Option<&Evaluation<f64>>,
    resolver: &Option<Arc<dyn PartResolver>>,
    tol: Tol,
) -> Evaluation<f64> {
    evaluate(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions {
            resolver: resolver.clone(),
            ..EvalOptions::default()
        },
        tol,
    )
}

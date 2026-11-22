//! **The authoring vocabularies the panels offer**, and how a panel
//! field is written.
//!
//! A VOCABULARY module (`crates/viewer/README.md`, Module boundaries):
//! values, their wording, and pure functions over them. Each enum here
//! MIRRORS a kernel or sketch enum, kept separate from it because what
//! a form offers is a product decision and what the kernel accepts is
//! not. Nothing here names `DocSession`, `ViewerApp` or `egui`.
//!
//! **What is hand-maintained here is the mirror, not the membership.**
//! The enums declared here declare themselves and their `ALL` in one
//! declaration through the crate's `vocabulary!` macro
//! (`crates/viewer/src/vocab.rs`), so no list on this page can fall
//! behind the enum beside it. A vocabulary the KERNEL owns is not
//! mirrored at all: the boolean operations, the path verbs, the arc
//! modes and the target forms are drawn from the kernel's own `ALL`
//! and only their words are written here, at an exhaustive match (the
//! verbs not even that — `profile::Verb`'s `Display` is its word). What
//! no compiler holds is a DELIBERATELY PARTIAL list, which claims no
//! completeness and so cannot be held to it (`partial_mirror!`,
//! `crates/viewer/src/vocab.rs`, holds such a list to the weaker thing
//! that IS true of it; `crate::matetool::MATE_KINDS` is held that way,
//! beside the tool that reads it). (Code spans rather than links:
//! everything on this page is `pub(crate)`, so an intra-doc link from a
//! public module page does not resolve.)
//!
//! [`FieldWriting`] is the same kind of decision one level down: the
//! unit a field is written in and how many of it one pixel of drag is
//! worth.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::Dimension;
use pncad::profile::{ArcMode, TargetKind};
use pncad::quantity::UnitDef;
use pncad::topo::BooleanOp;

use crate::props;
use crate::vocab::vocabulary;

vocabulary! {
    /// The pattern form's rule choice — the two PARAMETRIC rules. An
    /// enum rather than an index into a label list, because a consumer
    /// can match an enum and name every rule, and cannot do that with an
    /// index. `Explicit` is absent by the plan's ruling: a list of
    /// absolute frames is not a form's job.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum PatternLayoutChoice {
        /// Stepped along a direction.
        Linear = "linear",
        /// Stepped around an axis given by a point and a direction.
        Circular = "circular",
    }

    /// Every rule with its radio label, in form order.
    pub(crate) const ALL;
}

/// The button a boolean operation is offered under — the KERNEL's enum
/// and its own words, so what a user reads and what the node carries
/// cannot drift into two vocabularies.
///
/// **A match, not a table**, and that is the whole of what holds this
/// form to the kernel: [`BooleanOp`] is declared in `topo`, so no list
/// written here can be projected from the declaration the way every
/// [`crate::vocab::vocabulary`] list on this page is — but the
/// declaration publishes `BooleanOp::ALL`, and the form draws one
/// button per entry of it. A fourth operation therefore arrives in
/// this form with no MEMBERSHIP edit here — it gets its button from
/// the kernel's list, and its word from this match.
///
/// **The order is `ALL`'s**, which is the kernel's declaration order,
/// and the type's own doc says that order carries no meaning. The form
/// claims none for it either: it is the one order that cannot fall out
/// of step with the vocabulary, which is worth more here than an
/// arrangement a reader would have to maintain by hand.
pub(crate) fn boolean_op_label(op: BooleanOp) -> &'static str {
    match op {
        BooleanOp::Union => "union",
        BooleanOp::Intersect => "intersect",
        BooleanOp::Subtract => "subtract",
    }
}

pub(crate) use crate::tree::split_half_label;

vocabulary! {
    /// The profile editor's add-loop choices: the templates a loop
    /// starts as (`crate::drafts::template_steps`) — one complete-loop
    /// step, a circle or a rectangle, or a chain of verbs to edit.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum ShapeKind {
        /// A circle.
        Circle = "circle",
        /// A centred rectangle.
        Rectangle = "rectangle",
        /// A chain of authoring verbs — the whole PATHS vocabulary.
        Path = "path",
    }

    /// Every template with its label, in the add-loop row's order.
    pub(crate) const ALL;
}

/// The word the arc-mode picker shows for a mode of the KERNEL's
/// [`ArcMode`], whose `ALL` the picker offers.
///
/// **A match, not a table**, for the reason [`boolean_op_label`] is
/// one: a mode the vocabulary gains reaches the picker from
/// `ArcMode::ALL` with no membership edit here, and its word from
/// this match.
/// The verbs need no such function: `profile::Verb`'s own `Display`
/// is the authoring spelling, declared on the transition table's row.
pub(crate) fn arc_mode_label(mode: ArcMode) -> &'static str {
    match mode {
        ArcMode::Radius => "radius",
        ArcMode::Bulge => "bulge",
        ArcMode::Via => "via",
        ArcMode::Center => "centre",
        ArcMode::Sweep => "sweep",
        ArcMode::ArcLen => "arc length",
    }
}

/// The word a target control shows for a form of the KERNEL's
/// [`TargetKind`] — [`arc_mode_label`]'s rule, one level down.
pub(crate) fn target_kind_label(kind: TargetKind) -> &'static str {
    match kind {
        TargetKind::Point => "point",
        TargetKind::Start => "Start (close)",
        TargetKind::StartArriving => "Start, arriving tangent",
    }
}

/// The fewest subdivisions the `circle_split` count field offers —
/// the kernel's own floor (`profile::Step::CircleSplit`'s `n`, which
/// refuses below two at replay).
pub(crate) const MIN_CIRCLE_SPLIT: usize = 2;

/// The most subdivisions the `circle_split` count field offers.
///
/// A product cap and not the kernel's: the kernel takes any count, but
/// the form's preview replays the loop every frame, and replaying it
/// builds one vertex per subdivision. A thousand vertices is far past
/// any subdivision a seam is aligned with and still cheap to redraw.
///
/// **It binds AUTHORING only.** A committed profile can hold a larger
/// count (the document admits any), and the same field shows it when
/// the editor is opened on that profile: the field is drawn with
/// `clamp_existing_to_range(false)`, so the cap limits what a drag or
/// a typed number can make and never rewrites a count it was handed.
pub(crate) const MAX_CIRCLE_SPLIT: usize = 1024;

/// **How ONE FIELD is written**: the unit it shows and authors in, and
/// the tick it is scrubbed at ([`props::drag_tick`]), in that unit.
///
/// The two are one value because they are one decision. A tick is a
/// number of whatever the field says, so a tick chosen without the unit
/// is half a millimetre applied to a field showing metres — the same
/// gesture made a thousand times coarser by a change of notation.
///
/// The panel fields build one off the row they are drawn for; a
/// creation form's draft has no row behind it, so the form builds one
/// off the dimension it already knows and the working notation
/// (`crate::widgets::named_field`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldWriting {
    /// The unit the field shows and authors in — [`props::rendering_unit`]'s
    /// answer, so a computed slot and a written literal agree. `None`
    /// is the field that names no notation at all (a count).
    pub unit: Option<UnitDef>,
    /// One drag tick, IN [`Self::unit`] — 0.5 for a millimetre field,
    /// 0.0005 for the same field written in metres.
    pub tick: f64,
}

impl FieldWriting {
    /// How a field of `dimension` whose value remembers `stored` is
    /// written. `stored` is the row's own `unit` — the fact the
    /// document carries, before [`props::rendering_unit`] reads a value
    /// that remembers nothing in the working `notation`.
    pub fn of(dimension: Dimension, stored: Option<UnitDef>, notation: props::Notation) -> Self {
        let unit = props::rendering_unit(dimension, stored, notation);
        Self {
            unit,
            tick: props::shown_in(unit, props::drag_tick(Some(dimension))),
        }
    }

    /// A field whose dimension is not chosen yet: the bare number, at
    /// [`props::drag_tick`]'s tick for no dimension.
    pub const fn unpicked() -> Self {
        Self {
            unit: None,
            tick: props::drag_tick(None),
        }
    }

    /// One canonical value as this field SHOWS it.
    pub fn shown(self, canonical: f64) -> f64 {
        props::shown_in(self.unit, canonical)
    }

    /// One number read out of this field — dragged or typed — back in
    /// canonical terms. [`Self::shown`]'s inverse, and the door every
    /// value crossing out of a panel field goes through, because what
    /// crosses `props` is canonical.
    pub fn authored(self, shown: f64) -> f64 {
        props::authored_in(self.unit, shown)
    }
}

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
//! [`FieldWriting`] and the drag speeds are the same kind of decision
//! one level down: how many of a unit one pixel of drag is worth.
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
    /// The add-profile form's loop choice: the two templates, or a PATH
    /// authored verb by verb.
    ///
    /// An enum for the reason [`PatternLayoutChoice`] is one — and the templates
    /// stay in it rather than being folded into the path arm because they
    /// are not chains: a circle is a seamless closed carrier no chain of
    /// legs can spell, and a rectangle is four `line_to`s nobody should
    /// have to type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum ShapeKind {
        /// A circle, optionally with a concentric bore.
        Circle = "circle",
        /// A centred rectangle.
        Rectangle = "rectangle",
        /// A chain of authoring verbs — the whole PATHS vocabulary.
        Path = "path",
    }

    /// Every shape with its radio label, in form order.
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

/// One drag tick of a LENGTH field, in metres — half a millimetre.
/// The creation forms' and the property panel's alike ([`drag_tick`]
/// is where the panel picks it), so one gesture over a length cannot
/// come to mean two different steps.
pub(crate) const FIELD_DRAG_SPEED: f64 = 0.0005;

/// One drag tick of an ANGLE field, in radians — a third of a degree,
/// so a full turn is a drag of a few hundred pixels rather than of
/// several screens.
///
/// A separate constant because the unit is: dragging a radian field at
/// the metre field's speed moves it by 0.0005 rad per pixel, which is
/// a quarter-turn per three thousand pixels.
pub(crate) const ANGLE_DRAG_SPEED: f64 = 0.005;

/// One drag tick of a DIMENSIONLESS field — a direction or a normal
/// component, whose useful range is roughly [-1, 1].
///
/// The length speed applied here made these fields effectively
/// undraggable: at 0.0005 per pixel, moving a component from 0 to 1
/// took two thousand pixels of drag. A hundredth per pixel spans the
/// whole range in one comfortable gesture, and the exact value stays a
/// keyboard edit either way.
pub(crate) const UNIT_DRAG_SPEED: f64 = 0.01;

/// One drag tick of a COUNT field — instances are whole, so the field
/// is dragged in tenths of one and lands on integers.
pub(crate) const COUNT_DRAG_SPEED: f64 = 0.1;

/// **The drag tick a slot of `dimension` is scrubbed at**, in
/// CANONICAL units — the property panel's pick from the same four
/// constants the creation forms choose between by hand.
///
/// A dimension branch and not one number, because the useful range of
/// a slot is its dimension's: half a millimetre per pixel is a good
/// length tick and a terrible angle one — at 0.0005 rad it takes
/// twelve thousand pixels to drag a full turn, which is the same
/// arithmetic [`ANGLE_DRAG_SPEED`] exists to answer for the forms.
/// A `Count` never reaches here (its slots are structural, and the
/// panel steps those in whole units), so it takes the count tick for
/// completeness rather than for use.
pub(crate) fn drag_tick(dimension: Dimension) -> f64 {
    match dimension {
        Dimension::Length => FIELD_DRAG_SPEED,
        Dimension::Angle => ANGLE_DRAG_SPEED,
        Dimension::Scalar => UNIT_DRAG_SPEED,
        Dimension::Count => COUNT_DRAG_SPEED,
    }
}

/// **How ONE PANEL FIELD is written**: the unit it shows and authors
/// in, and the tick it is scrubbed at, taken together off the row it is
/// drawn for.
///
/// The two are one value because they are one decision. A tick is a
/// number of whatever the field says, so a tick chosen without the unit
/// is half a millimetre applied to a field showing metres — the same
/// gesture made a thousand times coarser by a change of notation.
///
/// **The two panel fields this answers for are the SLOT field
/// (`ViewerBehavior::slot_value_ui`) and the DOCUMENT VARIABLE's
/// (`ViewerBehavior::properties_ui`'s `Selection::Variable` arm)** — the
/// two a user drags to move the same kind of number. It is not the
/// creation forms' answer: those hold canonical drafts and pick their
/// tick from the four constants by hand at each field
/// ([`crate::widgets::named_field`] and its callers). The RULE has one home,
/// this module, which holds the four constants and [`drag_tick`]
/// beside this type; what is still open is those hand-picked call
/// sites, which sit in `widgets`, [`crate::pane::create`] and
/// [`crate::pane::properties`] (`work/forms/drag-tick-has-three-homes.md`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldWriting {
    /// The unit the field shows and authors in — [`props::rendering_unit`]'s
    /// answer, so a computed slot and a written literal agree. `None`
    /// is the field that names no notation at all (a count, a bare
    /// scalar).
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
        // A COUNT field steps by one whatever it is written in: what it
        // holds is a count, and a tenth of an instance is not a value
        // it can take. Read off the dimension and not off a
        // structurality flag beside it — `SlotId::is_structural` is
        // itself defined as "the dimension is Count"
        // ([`props::SlotValue::of`] argues this at length), so a second
        // argument would only be a way for the two to disagree.
        let tick = if dimension == Dimension::Count {
            1.0
        } else {
            props::shown_in(unit, drag_tick(dimension))
        };
        Self { unit, tick }
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

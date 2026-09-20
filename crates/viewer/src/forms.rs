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
//! completeness and so cannot be held to it.
//! Both partial mirrors on this page are held to the weaker thing that
//! IS true of them: `partial_mirror!` (`crates/viewer/src/vocab.rs`)
//! classifies every variant of the mirrored enum as offered here or as
//! deliberately absent with its reason, so neither `MatePrimitive` nor
//! `DatumSpec` can grow past this form in silence. The two take
//! different shapes of that one macro, because what they offer differs:
//! `MATE_PRIMITIVES` is a hand-written list and its roster holds a seat
//! per offered entry, while `DatumKindChoice` is an enum whose `ALL` is
//! projected, so its roster names a counterpart and has no seat to
//! hold. Each says so at its own site. (Code spans rather
//! than links: everything on this page is `pub(crate)`, so an
//! intra-doc link from a public module page does not resolve.)
//!
//! [`FieldWriting`] and the drag speeds are the same kind of decision
//! one level down: how many of a unit one pixel of drag is worth.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{BooleanOp, Dimension, MatePrimitive};
use pncad::profile::{ArcMode, TargetKind};
use pncad::quantity::UnitDef;

use crate::props;
use crate::session::DatumSpec;
use crate::vocab::{partial_mirror, vocabulary};

vocabulary! {
    /// The pattern form's rule choice — the two PARAMETRIC rules, an enum
    /// for the reason [`DatumKindChoice`] is one. `Explicit` is absent by the
    /// plan's ruling: a list of absolute frames is not a form's job.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum PatternKindChoice {
        /// Stepped along a direction.
        Linear = "linear",
        /// Stepped around a picked datum axis.
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
/// the kernel's list — and it cannot arrive silently either, because
/// it has no word until this match is given one, which is a compile
/// error and not a missing button.
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

vocabulary! {
    /// The add-datum form's kind choice — one form, and **every arm of
    /// [`crate::session::DatumSpec`]**. An enum rather than an index
    /// into a label list, so every consumer matches exhaustively and a
    /// new kind cannot leave a silent wildcard arm behind.
    ///
    /// `AxisInPlane` is the one kind that needs a PICK as well as
    /// numbers: its frame is a document node, chosen from the frames
    /// the document holds, and its origin and direction are that
    /// frame's own 2-D coordinates. It is the only node the revolve
    /// tool's axis seat admits.
    ///
    /// **`Choice` because `viewer::DatumKind` is a different type** —
    /// the tag [`crate::datums::DatumDraw`] carries for how a datum is
    /// DRAWN, which partitions the datum VALUES rather than selecting
    /// among the specs. This crate already spells a form's choice
    /// apart from the thing chosen among that way
    /// ([`PatternKindChoice`], [`crate::blend::BlendKindChoice`]). The
    /// two do not have the same members: an axis in a sketch is its
    /// own choice here, because authoring one takes a frame pick, and
    /// is drawn as the axis it is, so the draw tag has no member for
    /// it. `ALL` is this side's alone in consequence: it is the radio
    /// row's offering, in form order, and a drawing's tag claims no
    /// such thing.
    ///
    /// **Held to `DatumSpec` by a roster.** The direction the compiler
    /// already held is kind-to-spec: `Drafts::datum_spec`'s lowering
    /// match is exhaustive over this enum, so a choice with no spec to lower
    /// to does not build. The `partial_mirror!` invocation below holds
    /// spec-to-kind: every `DatumSpec` arm is classified as offered
    /// here or as deliberately absent with its reason, so a new arm
    /// cannot arrive with no form edit and nothing saying so. An empty
    /// absent section means the form offers the whole spec. It takes
    /// the `onto` shape rather than a seat roster because the offering
    /// is an ENUM whose `ALL` is projected from its declaration, so
    /// naming a counterpart there already says the radio row draws it
    /// (`crates/viewer/src/vocab.rs`).
    ///
    /// **That roster holds nothing between this enum and
    /// [`crate::datums::DatumKind`]**: what it mirrors is `DatumSpec`,
    /// and the draw tag is not party to it.
    ///
    /// **Declared in FORM order**, which is the order [`DatumKindChoice::ALL`]
    /// is projected in and therefore the order the radio row is drawn
    /// in: the frame sits next to the plane because that is the choice
    /// a reader is actually making — the same surface, with or without
    /// a stated direction on it — and the axis in a sketch sits next to
    /// the axis for the same reason.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum DatumKindChoice {
        /// A plane datum.
        Plane = "plane",
        /// A sketch frame — an oriented plane.
        Frame = "frame",
        /// An axis datum, in world coordinates.
        Axis = "axis",
        /// An axis written in a picked sketch frame — a revolve's axis.
        AxisInPlane = "axis in sketch",
        /// A point datum.
        Point = "point",
    }

    /// Every kind with its radio label, in form order.
    pub(crate) const ALL;
}

partial_mirror! {
    DatumSpec, onto DatumKindChoice,
    offered [
        Plane { .. } => Plane,
        Axis { .. } => Axis,
        Point { .. } => Point,
        AxisInPlane { .. } => AxisInPlane,
        Frame { .. } => Frame,
    ],
    absent [],
}

vocabulary! {
    /// The add-profile form's loop choice: the two templates, or a PATH
    /// authored verb by verb.
    ///
    /// An enum for the reason [`DatumKindChoice`] is one — and the templates
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
/// `ArcMode::ALL` with no membership edit here, and has no word until
/// this match gives it one — a compile error, not a missing option.
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

/// **Whether a path editor may change its program's SHAPE** — the
/// verbs, their order and number, each arc's mode, side and winding,
/// each target's form, a split circle's count — or only its numbers.
///
/// The add-profile form's editor is one editor with two doors. Opened
/// on nothing it authors a new node and every control is live
/// ([`ShapeEdits::Free`]). Opened on a committed profile it commits as
/// slot writes, and the document's edit vocabulary writes a program's
/// ARGUMENTS and has no door that rewrites its shape, so the controls
/// that would are shown and not taken ([`ShapeEdits::Locked`], said
/// once over the list as [`SHAPE_LOCKED`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShapeEdits {
    /// Every control is live.
    Free,
    /// The shape controls are drawn disabled.
    Locked,
}

impl ShapeEdits {
    /// Whether the shape controls take input.
    pub(crate) fn free(self) -> bool {
        self == Self::Free
    }
}

/// What a locked editor says about its greyed controls, once, above
/// the list.
pub(crate) const SHAPE_LOCKED: &str = "the numbers are editable here; the shape (the steps, \
     their verbs and order, arc modes, sides and targets) is not — the document has no edit that \
     rewrites a committed profile's program";

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
/// (`ViewerBehavior::slot_value_ui`) and the DOCUMENT PARAMETER's
/// (`ViewerBehavior::properties_ui`'s `Selection::Param` arm)** — the
/// two a user drags to move the same kind of number. It is not the
/// creation forms' answer: those hold canonical drafts and pick their
/// tick from the four constants by hand at each field
/// ([`crate::widgets::named_field`] and its callers). The RULE has one home,
/// this module, which holds the four constants and [`drag_tick`]
/// beside this type; what is still open is those hand-picked call
/// sites, which sit in `widgets`, [`crate::pane::create`] and
/// [`crate::pane::properties`] (`work/chrome/drag-tick-has-three-homes.md`).
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
    /// document carries, before [`props::rendering_unit`] chooses what
    /// a value that remembers nothing reads as.
    pub fn of(dimension: Dimension, stored: Option<UnitDef>) -> Self {
        let unit = props::rendering_unit(dimension, stored);
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

/// The primitives the chrome offers, with their labels. The op
/// vocabulary accepts any [`MatePrimitive`]; these are the three the
/// panel can spell without a numeric field (`PlanarRest`'s offset is
/// authored 0 — a flush rest; a standoff is typed through the tree's
/// ordinary property doors once the node exists).
///
/// **Deliberately partial**, which is why it is hand-written and not
/// projected: [`MatePrimitive`] has a fourth variant (`Clocking`) that
/// the kernel represents so it can REFUSE it, and a form offering it
/// would be offering a refusal. Completeness is exactly what this list
/// does not claim, so a mechanism that forced it would be forcing the
/// wrong thing — mapping this form over a published `ALL` the way the
/// boolean buttons are mapped is precisely the wrong fix here.
///
/// **A decision per variant, and the compiler holds the decision.**
/// Nothing here forces the list to be COMPLETE — completeness is what
/// it does not claim. What the `partial_mirror!` invocation below
/// forces (`crates/viewer/src/vocab.rs` declares the macro) is that
/// every [`MatePrimitive`] variant is either offered at a seat of this
/// list or named below as deliberately absent, with the reason it is
/// absent. A primitive added to the kernel enum is neither until
/// someone writes one of the two, and the build says so.
pub(crate) const MATE_PRIMITIVES: [(MatePrimitive, &str); 3] = [
    (MatePrimitive::FrameCoincidence, "frame coincidence"),
    (MatePrimitive::Coaxial, "coaxial"),
    (MatePrimitive::PlanarRest { offset: 0.0 }, "planar rest"),
];

partial_mirror! {
    MatePrimitive, labelled MATE_PRIMITIVES,
    offered [FrameCoincidence, Coaxial, PlanarRest { .. }],
    absent [
        Clocking => "the kernel represents it so it can REFUSE it \
                     (`mate::solve` faults `TableLacks` on a standalone \
                     clocking), and a form offering it would be offering \
                     a refusal",
    ],
}

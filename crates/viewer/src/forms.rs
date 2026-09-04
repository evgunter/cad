//! **The authoring vocabularies the panels offer**, and how a panel
//! field is written.
//!
//! A VOCABULARY module (`crates/viewer/README.md`, Module boundaries):
//! values, their wording, and pure functions over them. Each enum here
//! is a hand-maintained mirror of a kernel or sketch enum, kept
//! separate from it because what a form offers is a product decision
//! and what the kernel accepts is not. Nothing here names `DocSession`,
//! `ViewerApp` or `egui`.
//!
//! [`FieldWriting`] and the drag speeds are the same kind of decision
//! one level down: how many of a unit one pixel of drag is worth.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{BooleanOp, Dimension, MatePrimitive};
use pncad::profile::{ArcSide, ArcSweep};
use pncad::quantity::UnitDef;

use crate::props;
use crate::sketch::{ArcSpec, PathStep, PathTarget};

/// The pattern form's rule choice — the two PARAMETRIC rules, an enum
/// for the reason [`DatumKind`] is one. `Explicit` is absent by the
/// plan's ruling: a list of absolute frames is not a form's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PatternKindChoice {
    /// Stepped along a direction.
    Linear,
    /// Stepped around a picked datum axis.
    Circular,
}

impl PatternKindChoice {
    /// Every rule with its radio label, in form order.
    pub(crate) const ALL: [(Self, &'static str); 2] =
        [(Self::Linear, "linear"), (Self::Circular, "circular")];
}

/// The boolean operations the form offers, with their labels — the
/// KERNEL's enum and its own words, so the button a user reads and the
/// operation the node carries cannot drift into two vocabularies.
pub(crate) const BOOLEAN_OPS: [(BooleanOp, &str); 3] = [
    (BooleanOp::Union, "union"),
    (BooleanOp::Subtract, "subtract"),
    (BooleanOp::Intersect, "intersect"),
];

/// The add-datum form's kind choice — one form, the three
/// [`crate::session::DatumSpec`] arms. An enum rather than an index into a label
/// list, so every consumer matches exhaustively and a fourth kind
/// cannot leave a silent wildcard arm behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DatumKind {
    /// A plane datum.
    Plane,
    /// An axis datum.
    Axis,
    /// A point datum.
    Point,
    /// A sketch frame — an oriented plane.
    Frame,
}

impl DatumKind {
    /// Every kind with its radio label, in form order.
    ///
    /// The frame sits next to the plane because that is the choice a
    /// reader is actually making: the same surface, with or without a
    /// stated direction on it.
    pub(crate) const ALL: [(Self, &'static str); 4] = [
        (Self::Plane, "plane"),
        (Self::Frame, "frame"),
        (Self::Axis, "axis"),
        (Self::Point, "point"),
    ];
}

/// The add-profile form's loop choice: the two templates, or a PATH
/// authored verb by verb.
///
/// An enum for the reason [`DatumKind`] is one — and the templates
/// stay in it rather than being folded into the path arm because they
/// are not chains: a circle is a seamless closed carrier no chain of
/// legs can spell, and a rectangle is four `line_to`s nobody should
/// have to type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShapeKind {
    /// A circle, optionally with a concentric bore.
    Circle,
    /// A centred rectangle.
    Rectangle,
    /// A chain of authoring verbs — the whole PATHS vocabulary.
    Path,
}

impl ShapeKind {
    /// Every shape with its radio label, in form order.
    pub(crate) const ALL: [(Self, &'static str); 3] = [
        (Self::Circle, "circle"),
        (Self::Rectangle, "rectangle"),
        (Self::Path, "path"),
    ];
}

/// **The authoring verbs the path form offers**, with the names the
/// algebra itself gives them.
///
/// A tag beside [`PathStep`] rather than a method on it: the form
/// needs to name a verb BEFORE it has a step (the "add" control's
/// choice), and a step needs to name its own verb (the row's combo),
/// so the tag is the thing both hold. [`PathVerb::fresh`] is the one
/// place a default step per verb is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PathVerb {
    /// Bind the tip's position.
    At,
    /// Bind the tip's outgoing direction, absolutely.
    Angle,
    /// Bind it by exact components.
    Toward,
    /// Leave along the incoming tangent.
    Tangent,
    /// Leave along its reverse.
    Cusp,
    /// Leave at an angle from it.
    Turn,
    /// A straight leg of a stated length.
    Line,
    /// A straight leg to a target.
    LineTo,
    /// A sharp arc leg.
    ArcTo,
    /// An arc leg leaving along the bound direction.
    TangentArcTo,
    /// A structural vertex on the incoming carrier.
    ArcContinue,
    /// Round the corner: line in, line out.
    Fillet,
    /// Round it with an arc on the arrival side.
    FilletArc,
    /// Round it with an arc on the incoming side.
    ArcFillet,
    /// Round it with an arc on both.
    ArcFilletArc,
    /// The anchor a fillet's arrival side is aimed at.
    FarEndTo,
    /// The seam fillet's close.
    CloseTo,
}

impl PathVerb {
    /// Every verb, in the algebra's own order — the "add step" menu
    /// and the row combo's options. Labels come from
    /// [`PathVerb::label`], so this list carries the ORDER and
    /// nothing a second copy of it could get wrong.
    pub(crate) const ALL: [Self; 17] = [
        Self::At,
        Self::Angle,
        Self::Toward,
        Self::Tangent,
        Self::Cusp,
        Self::Turn,
        Self::Line,
        Self::LineTo,
        Self::ArcTo,
        Self::TangentArcTo,
        Self::ArcContinue,
        Self::Fillet,
        Self::FilletArc,
        Self::ArcFillet,
        Self::ArcFilletArc,
        Self::FarEndTo,
        Self::CloseTo,
    ];

    /// This verb's label — a match rather than a search through
    /// [`PathVerb::ALL`], so a verb with no label is a compile error
    /// rather than a `?` on somebody's screen. (Whether a verb
    /// reaches the MENU is [`PathVerb::ALL`]'s to answer, and nothing
    /// checks that: the type is private behind the `app` feature, so
    /// no row can see it — issue #1385.)
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::At => "at",
            Self::Angle => "angle",
            Self::Toward => "toward",
            Self::Tangent => "tangent",
            Self::Cusp => "cusp",
            Self::Turn => "turn",
            Self::Line => "line",
            Self::LineTo => "line_to",
            Self::ArcTo => "arc_to",
            Self::TangentArcTo => "tangent_arc_to",
            Self::ArcContinue => "arc_continue",
            Self::Fillet => "fillet",
            Self::FilletArc => "fillet_arc",
            Self::ArcFillet => "arc_fillet",
            Self::ArcFilletArc => "arc_fillet_arc",
            Self::FarEndTo => "to (far end)",
            Self::CloseTo => "to Start (close)",
        }
    }

    /// Which verb a step names.
    pub(crate) fn of(step: &PathStep) -> Self {
        match step {
            PathStep::At(_) => Self::At,
            PathStep::Angle(_) => Self::Angle,
            PathStep::Toward { .. } => Self::Toward,
            PathStep::Tangent => Self::Tangent,
            PathStep::Cusp => Self::Cusp,
            PathStep::Turn(_) => Self::Turn,
            PathStep::Line(_) => Self::Line,
            PathStep::LineTo(_) => Self::LineTo,
            PathStep::ArcTo(_) => Self::ArcTo,
            PathStep::TangentArcTo(_) => Self::TangentArcTo,
            PathStep::ArcContinue(_) => Self::ArcContinue,
            PathStep::Fillet(_) => Self::Fillet,
            PathStep::FilletArc { .. } => Self::FilletArc,
            PathStep::ArcFillet { .. } => Self::ArcFillet,
            PathStep::ArcFilletArc { .. } => Self::ArcFilletArc,
            PathStep::FarEndTo(_) => Self::FarEndTo,
            PathStep::CloseTo => Self::CloseTo,
        }
    }

    /// A step of this verb with the form's starting numbers.
    ///
    /// **Millimetre-scale, never zero.** A leg of length zero and a
    /// fillet of radius zero are both geometry refusals, so a fresh
    /// step that carried them would put the form in a refusing state
    /// the moment a verb was added — which reads as the form
    /// rejecting the verb rather than waiting for its number.
    pub(crate) fn fresh(self) -> PathStep {
        let point = [0.01, 0.0];
        let arc = ArcSpec::Radius {
            r: 0.01,
            side: ArcSide::Left,
        };
        match self {
            Self::At => PathStep::At([0.0, 0.0]),
            Self::Angle => PathStep::Angle(0.0),
            Self::Toward => PathStep::Toward { dx: 1.0, dy: 0.0 },
            Self::Tangent => PathStep::Tangent,
            Self::Cusp => PathStep::Cusp,
            Self::Turn => PathStep::Turn(0.0),
            Self::Line => PathStep::Line(0.01),
            Self::LineTo => PathStep::LineTo(PathTarget::Point(point)),
            Self::ArcTo => PathStep::ArcTo(arc),
            Self::TangentArcTo => PathStep::TangentArcTo(PathTarget::Point(point)),
            Self::ArcContinue => PathStep::ArcContinue(point),
            Self::Fillet => PathStep::Fillet(0.001),
            Self::FilletArc => PathStep::FilletArc {
                radius: 0.001,
                spec: arc,
            },
            Self::ArcFillet => PathStep::ArcFillet {
                spec: arc,
                radius: 0.001,
            },
            Self::ArcFilletArc => PathStep::ArcFilletArc {
                spec: arc,
                radius: 0.001,
                spec2: arc,
            },
            Self::FarEndTo => PathStep::FarEndTo(point),
            Self::CloseTo => PathStep::CloseTo,
        }
    }
}

/// **Which of [`ArcSpec`]'s six modes the form is offering** — the
/// tag [`PathVerb`] is, for the reason it is one: the picker needs to
/// name a mode before there is a spec in it, and a spec needs to name
/// its own mode. An index into a label table would couple the two by
/// position, so a reordered table would silently relabel every mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArcMode {
    /// The carrier's radius and the side its centre is on.
    Radius,
    /// The endpoint and an authored bulge.
    Bulge,
    /// A point the arc passes through, and the endpoint.
    Via,
    /// The carrier centre, the travel sense, and the endpoint.
    Center,
    /// The carrier and how far round it to go.
    Sweep,
    /// The carrier and the distance travelled along it.
    ArcLen,
}

impl ArcMode {
    /// Every mode, in the vocabulary's own order — the picker's
    /// options.
    pub(crate) const ALL: [Self; 6] = [
        Self::Radius,
        Self::Bulge,
        Self::Via,
        Self::Center,
        Self::Sweep,
        Self::ArcLen,
    ];

    /// This mode's label.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Radius => "radius",
            Self::Bulge => "bulge",
            Self::Via => "via",
            Self::Center => "centre",
            Self::Sweep => "sweep",
            Self::ArcLen => "arc length",
        }
    }

    /// Which mode a spec is in.
    pub(crate) fn of(spec: &ArcSpec) -> Self {
        match spec {
            ArcSpec::Radius { .. } => Self::Radius,
            ArcSpec::Bulge { .. } => Self::Bulge,
            ArcSpec::Via { .. } => Self::Via,
            ArcSpec::Center { .. } => Self::Center,
            ArcSpec::Sweep { .. } => Self::Sweep,
            ArcSpec::ArcLen { .. } => Self::ArcLen,
        }
    }

    /// A spec of this mode with the form's starting numbers —
    /// millimetre-scale and never degenerate, for the reason
    /// [`PathVerb::fresh`]'s are.
    pub(crate) fn fresh(self) -> ArcSpec {
        let target = PathTarget::Point([0.01, 0.0]);
        match self {
            Self::Radius => ArcSpec::Radius {
                r: 0.01,
                side: ArcSide::Left,
            },
            Self::Bulge => ArcSpec::Bulge { target, b: 0.5 },
            Self::Via => ArcSpec::Via {
                q: [0.005, 0.005],
                target,
            },
            Self::Center => ArcSpec::Center {
                c: [0.0, 0.0],
                winding: ArcSweep::Ccw,
                target,
            },
            Self::Sweep => ArcSpec::Sweep {
                r: 0.01,
                side: ArcSide::Left,
                angle: core::f64::consts::FRAC_PI_2,
            },
            Self::ArcLen => ArcSpec::ArcLen {
                r: 0.01,
                side: ArcSide::Left,
                len: 0.01,
            },
        }
    }
}

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
/// (`widgets::named_field` and its callers). The RULE has one home,
/// this module, which holds the four constants and [`drag_tick`]
/// beside this type; what is still open is those hand-picked call
/// sites, which sit in `widgets`, `pane::create` and
/// `pane::properties` (`work/chrome/drag-tick-has-three-homes.md`).
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
pub(crate) const MATE_PRIMITIVES: [(MatePrimitive, &str); 3] = [
    (MatePrimitive::FrameCoincidence, "frame coincidence"),
    (MatePrimitive::Coaxial, "coaxial"),
    (MatePrimitive::PlanarRest { offset: 0.0 }, "planar rest"),
];

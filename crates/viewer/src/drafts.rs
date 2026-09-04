//! **The in-flight form state**: what a panel is mid-edit on, and its
//! lowering to the document's own types.
//!
//! A VOCABULARY module (`crates/viewer/README.md`, Module boundaries).
//! [`Drafts`] is layer-3 state that never enters the document — an
//! expression the user is typing is not an edit until they commit it —
//! and its methods turn typed field values into [`Expr`]s and
//! [`LoopProgram`]s. Nothing here names `DocSession`, `ViewerApp` or
//! `egui`; the panels write these fields and read nothing back.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! toolkit type (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    BooleanOp, Dimension, DimensionError, Expr, LoopProgram, ParamName, RecipeNodeId, SlotId,
};
use pncad::quantity::{self, AngleUnit, LengthUnit, WrittenAngle, WrittenLength};

use crate::blend::BlendKindChoice;
use crate::combine::PatternOutputChoice;
use crate::forms::{DatumKind, PatternKindChoice, ShapeKind};
use crate::seats::SeatError;
use crate::session::ProfileShape;
use crate::sketch::{self, PathStep, PathTarget};

/// Transient text a panel is mid-edit on.
///
/// Layer-3 state that never enters the document: an expression the
/// user is typing is not an edit until they commit it, and a draft
/// abandoned by selecting elsewhere leaves nothing behind.
#[derive(Debug)]
pub(crate) struct Drafts {
    /// The View pane's δ field while it has focus, in millimetres as
    /// typed. `None` whenever it does not, so an unfocused field shows
    /// the δ actually in force — including one the triangle budget
    /// chose after this field last committed.
    pub(crate) delta_mm: Option<String>,
    /// The slot whose value field is holding REFUSED text.
    ///
    /// The field's text is egui's while it has focus and the
    /// document's afterwards, so there is only one thing this layer
    /// has to remember: text a parse refusal sent back
    /// ([`crate::frame::retype_draft`]). Acting on the refusal — going off to
    /// declare the parameter it named — must not cost the text that
    /// raised it, so the field keeps showing it until an expression
    /// edit for that slot lands.
    pub(crate) expr_target: Option<(RecipeNodeId, SlotId)>,
    /// The refused text itself.
    pub(crate) expr_text: String,
    /// The add-parameter form's name field.
    pub(crate) new_param_name: String,
    /// Its chosen dimension — `None` until the user picks one, and
    /// the Create button waits for the pick. The offer path lands
    /// here from an expression whose context does not determine the
    /// new parameter's dimension, and a silently-defaulted one would
    /// be a guess none of this program's doors make.
    pub(crate) new_param_dimension: Option<Dimension>,
    /// Its value field.
    pub(crate) new_param_value: f64,
    /// The name an unknown-parameter refusal offered to create
    /// ([`crate::frame::creation_offer`]); shown over the form while the
    /// name field still says it.
    pub(crate) new_param_offer: Option<ParamName>,
    /// The mate tool's class/alignment choice, as widget state: an
    /// index into [`crate::matetool::admitted_classes`], an index into
    /// [`crate::forms::MATE_PRIMITIVES`], and the sense toggle. Draft chrome state
    /// only — the typed choice is minted at commit.
    pub(crate) mate_class: usize,
    pub(crate) mate_primitive: usize,
    pub(crate) mate_opposed: bool,
    /// The toolbar's New… form: `Some(text)` while the name field is
    /// open, `None` while it is not. The op is not emitted until
    /// Create — a name in flight is a draft, not a document.
    pub(crate) new_doc_name: Option<String>,
    /// **The frame the add-profile form draws on** — `None` until one
    /// is picked, which is the form's resting state in a document that
    /// holds no frame yet.
    ///
    /// A pick rather than a constant: a profile's plane is a document
    /// node, so the form names one that exists instead of minting one
    /// as a side effect of adding a profile. One submit, one node.
    pub(crate) profile_plane: Option<RecipeNodeId>,
    /// The add-datum form's kind choice.
    pub(crate) datum_kind: DatumKind,
    /// The add-datum form's origin/position, metres.
    pub(crate) datum_origin: [f64; 3],
    /// Its normal/direction (unitless; ignored by the point form).
    pub(crate) datum_direction: [f64; 3],
    /// The FRAME form's two in-plane axes, sketch +x then +y
    /// (unitless; ignored by every other kind).
    ///
    /// Their own fields rather than a reuse of `datum_direction`,
    /// because a form's default has to mean something: a plane opens
    /// facing +z, and a frame opens as the world xy frame — which the
    /// same buffer cannot say twice.
    pub(crate) datum_u: [f64; 3],
    /// The frame form's sketch +y axis.
    pub(crate) datum_v: [f64; 3],
    /// **The unit every creation form's LENGTH field is written in.**
    ///
    /// ONE choice for all the forms, not one per form. The panel's
    /// pickers are per literal because a literal is a thing in a
    /// document that remembers its own notation; a form's is a
    /// statement about how the person at the keyboard is working, and
    /// somebody who authors a datum in millimetres is not then
    /// authoring the extrude that consumes it in metres. The drafts
    /// behind the fields stay canonical either way ([`crate::widgets::unit_field`]),
    /// so moving the picker re-writes what is on screen and changes
    /// no value.
    /// Not optional: an authored value always names the notation it is
    /// written in (`quantity::written`'s module docs), so the field and
    /// the picker beside it read one fact rather than each resolving an
    /// absence its own way.
    pub(crate) length_unit: LengthUnit,
    /// The same for every ANGLE field. Defaults to half turns
    /// (`pi rad`), the notation this editor says angles in.
    pub(crate) angle_unit: AngleUnit,
    /// The add-profile form's shape choice — `None` until the user
    /// makes one.
    ///
    /// **Optional because the form is LIVE.** Every other draft here
    /// is a number sitting in a field, but this one decides what the
    /// viewport draws: the add-profile preview is taken from these
    /// drafts each frame, so a pre-selected shape meant that merely
    /// opening "Add feature" — to reach the datum form, say — put a
    /// circle in the picture that nobody had asked for. `None` is the
    /// form at rest: no shape fields, no preview, and the commit
    /// disabled until a shape is chosen.
    pub(crate) profile_shape: Option<ShapeKind>,
    /// The path form's verbs, in authoring order.
    ///
    /// Starts as the SQUARE a `line_to` chain spells — an empty list
    /// is a form with nothing to look at and no example of what a
    /// chain is supposed to look like, and this one is four verbs a
    /// reader can take apart. It is a draft like every other field
    /// here: nothing reaches a document until Add profile.
    pub(crate) profile_path: Vec<PathStep>,
    /// The circle form's centre, metres.
    pub(crate) profile_centre: [f64; 2],
    /// The circle form's radius, metres.
    pub(crate) profile_radius: f64,
    /// Whether the circle carries a concentric bore (a second loop
    /// inside the first) — what lets the chrome author the ring
    /// demo's annulus while staying a template, not a sketcher. The
    /// form guards bore < radius (see `add_profile_ui`): the loop
    /// ROLES come from the profile layer's containment forest, so a
    /// bore at or beyond the outer would not fail — it would swap
    /// which circle is the hole, silently defeating the form's own
    /// wording.
    pub(crate) profile_bored: bool,
    /// The bore's radius, metres.
    pub(crate) profile_bore: f64,
    /// The rectangle form's width and height, metres.
    pub(crate) profile_extent: [f64; 2],
    /// The extrude form's distance, metres.
    pub(crate) extrude_distance: f64,
    /// The revolve tool's angle, radians.
    pub(crate) revolve_angle: f64,
    /// The boolean tool's operation choice.
    pub(crate) boolean_op: BooleanOp,
    /// The transform tool's translation, metres.
    pub(crate) transform_translation: [f64; 3],
    /// Its rotation axis (unitless).
    pub(crate) transform_axis: [f64; 3],
    /// Its rotation angle, radians.
    pub(crate) transform_angle: f64,
    /// The pattern tool's rule choice.
    pub(crate) pattern_kind: PatternKindChoice,
    /// Its output choice — which of the two doors the commit button
    /// calls, and so whether the placements come out as separate
    /// instances or as one fused body.
    pub(crate) pattern_output: PatternOutputChoice,
    /// Its instance count — an INTEGER all the way from the field,
    /// because the slot it lands in is Count-typed and a number that
    /// was rounded on the way could differ from the one on screen.
    pub(crate) pattern_count: i64,
    /// The linear rule's step direction (unitless).
    pub(crate) pattern_direction: [f64; 3],
    /// The linear rule's spacing, metres.
    pub(crate) pattern_spacing: f64,
    /// The circular rule's angular step, radians.
    pub(crate) pattern_step: f64,
    /// The blend form's kind choice — which of the two doors the
    /// commit button calls.
    pub(crate) blend_kind: BlendKindChoice,
    /// Its one Length field, metres. ONE field for both kinds by the
    /// unit's spec: what the number means is the kind's to say
    /// ([`BlendKindChoice::size_label`]), and a second field would be
    /// a second place for the same quantity to be typed into.
    pub(crate) blend_size: f64,
}

impl Default for Drafts {
    /// The creation forms' sensible defaults (the GAUTH-1 spec):
    /// datum origin 0 with normal/direction +z, a 10 mm circle or
    /// rectangle, a 10 mm extrude, a full-turn revolve. Everything
    /// else starts empty.
    fn default() -> Self {
        Self {
            delta_mm: None,
            expr_target: None,
            expr_text: String::new(),
            new_param_name: String::new(),
            new_param_dimension: None,
            new_param_value: 0.0,
            new_param_offer: None,
            mate_class: 0,
            mate_primitive: 0,
            mate_opposed: false,
            new_doc_name: None,
            profile_plane: None,
            datum_kind: DatumKind::Plane,
            datum_origin: [0.0; 3],
            datum_direction: [0.0, 0.0, 1.0],
            datum_u: [1.0, 0.0, 0.0],
            datum_v: [0.0, 1.0, 0.0],
            length_unit: quantity::M,
            angle_unit: quantity::PI,
            profile_shape: None,
            profile_path: vec![
                PathStep::At([0.0, 0.0]),
                PathStep::LineTo(PathTarget::Point([0.01, 0.0])),
                PathStep::LineTo(PathTarget::Point([0.01, 0.01])),
                PathStep::LineTo(PathTarget::Point([0.0, 0.01])),
                PathStep::LineTo(PathTarget::Start),
            ],
            profile_centre: [0.0; 2],
            profile_radius: 0.01,
            profile_bored: false,
            profile_bore: 0.005,
            profile_extent: [0.01, 0.01],
            extrude_distance: 0.01,
            revolve_angle: core::f64::consts::TAU,
            boolean_op: BooleanOp::Union,
            transform_translation: [0.0; 3],
            transform_axis: [0.0, 0.0, 1.0],
            transform_angle: 0.0,
            pattern_kind: PatternKindChoice::Linear,
            pattern_output: PatternOutputChoice::Instances,
            pattern_count: 3,
            pattern_direction: [1.0, 0.0, 0.0],
            pattern_spacing: 0.02,
            pattern_step: core::f64::consts::FRAC_PI_2,
            blend_kind: BlendKindChoice::Fillet,
            blend_size: 0.001,
        }
    }
}

impl Drafts {
    /// **The loops the add-profile form would author right now.**
    ///
    /// One home, read twice: by the form's commit button and by the
    /// preview drawn under and around it. Two readings that built the
    /// loops separately could draw one shape and author another,
    /// which is the one way a live preview can lie.
    ///
    /// The bore is a second loop rather than a field on the first —
    /// which is what makes the ring demo's annulus a template rather
    /// than a special case — and the form's own guard on it lives at
    /// the commit ([`ViewerBehavior::add_profile_ui`](crate::app::ViewerBehavior::add_profile_ui)), because it is
    /// about what the loops MEAN, not about what they are.
    /// No shape chosen yet is the empty list, not a refusal: the form
    /// authors nothing and the preview draws nothing, which is what
    /// [`Drafts::profile_shape`] being `None` means.
    pub(crate) fn profile_loops(&self) -> Vec<ProfileShape> {
        match self.profile_shape {
            None => Vec::new(),
            Some(ShapeKind::Circle) => {
                let mut loops = vec![ProfileShape::Circle {
                    centre: self.profile_centre,
                    radius: self.profile_radius,
                }];
                if self.profile_bored {
                    loops.push(ProfileShape::Circle {
                        centre: self.profile_centre,
                        radius: self.profile_bore,
                    });
                }
                loops
            }
            Some(ShapeKind::Rectangle) => vec![ProfileShape::Rectangle {
                width: self.profile_extent[0],
                height: self.profile_extent[1],
            }],
            Some(ShapeKind::Path) => vec![ProfileShape::Path {
                steps: self.profile_path.clone(),
            }],
        }
    }

    /// **The notation these forms are authoring in** — the two pickers,
    /// as the lowering wants them.
    pub(crate) fn notation(&self) -> sketch::Notation {
        sketch::Notation {
            length: self.length_unit,
            angle: self.angle_unit,
        }
    }

    /// The loop PROGRAMS the add-profile form would author right now:
    /// [`Drafts::profile_loops`] lowered in this form's notation, which
    /// is what the op carries.
    ///
    /// # Errors
    ///
    /// A non-finite field (the literal door's refusal).
    pub(crate) fn profile_programs(&self) -> Result<Vec<LoopProgram>, DimensionError> {
        self.profile_loops()
            .iter()
            .map(|shape| sketch::loop_program(shape, self.notation()))
            .collect()
    }

    /// A `Length` literal from a draft field, remembering the form's
    /// notation. The draft is already canonical — a picker re-writes
    /// what is on screen and changes no value — so this attaches the
    /// unit without applying it.
    ///
    /// # Errors
    ///
    /// A non-finite draft (the literal door's refusal).
    pub(crate) fn length(&self, metres: f64) -> Result<Expr, DimensionError> {
        Expr::written_length(WrittenLength::canonical_in(metres, self.length_unit))
    }

    /// An `Angle` literal from a draft field — [`Drafts::length`]'s
    /// twin.
    ///
    /// # Errors
    ///
    /// A non-finite draft.
    pub(crate) fn angle(&self, radians: f64) -> Result<Expr, DimensionError> {
        Expr::written_angle(WrittenAngle::canonical_in(radians, self.angle_unit))
    }

    /// Three `Length` literals — a datum origin, a translation.
    ///
    /// # Errors
    ///
    /// A non-finite component.
    pub(crate) fn lengths(&self, v: [f64; 3]) -> Result<[Expr; 3], DimensionError> {
        Ok([self.length(v[0])?, self.length(v[1])?, self.length(v[2])?])
    }
}

/// Three dimensionless literals — a normal, a direction, a rotation
/// axis. Not a [`Drafts`] method, because there is no notation to
/// carry from the form: a dimensionless number has one spelling, and
/// `Expr::literal` stores that row itself.
///
/// # Errors
///
/// A non-finite component.
pub(crate) fn scalars(v: [f64; 3]) -> Result<[Expr; 3], DimensionError> {
    Ok([
        Expr::literal(v[0], Dimension::Scalar)?,
        Expr::literal(v[1], Dimension::Scalar)?,
        Expr::literal(v[2], Dimension::Scalar)?,
    ])
}

/// Why a creation form's commit did not produce an op.
///
/// Two faults reach one button: a seat a tool still needs, and a draft
/// the literal door refuses. They are separate types because they are
/// separate facts — one is about the picks, one about the numbers — and
/// this carries them to the status line without flattening either into
/// a string at the raising site.
#[derive(Debug)]
pub(crate) enum CommitFault {
    /// A tool seat is still empty.
    Seat(SeatError),
    /// A draft field is not a value a literal may hold.
    Dimension(DimensionError),
}

impl From<SeatError> for CommitFault {
    fn from(error: SeatError) -> Self {
        Self::Seat(error)
    }
}

impl From<DimensionError> for CommitFault {
    fn from(error: DimensionError) -> Self {
        Self::Dimension(error)
    }
}

impl std::fmt::Display for CommitFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seat(error) => error.fmt(f),
            Self::Dimension(error) => error.fmt(f),
        }
    }
}

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
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    BooleanOp, Dimension, DimensionError, Doc, Expr, LoopProgram, Node, ParamName, ProfileProgram,
    RecipeNodeId, RecordedProgramError, SlotId,
};
use pncad::geom_core::Point2;
use pncad::profile::{Step, Target};
use pncad::quantity::{self, AngleUnit, LengthUnit, WrittenAngle, WrittenLength};

use crate::blend::BlendKindChoice;
use crate::combine::PatternOutputChoice;
use crate::forms::{DatumKindChoice, PatternKindChoice, ShapeKind};
use crate::seats::SeatError;
use crate::session::{DatumSpec, ProfileShape, SessionOp};
use crate::sketch::{self, HeldRefusal};

/// Transient text a panel is mid-edit on.
///
/// Layer-3 state that never enters the document: an expression the
/// user is typing is not an edit until they commit it, and a draft
/// abandoned by selecting elsewhere leaves nothing behind.
#[derive(Debug)]
pub(crate) struct Drafts {
    /// The View pane's δ field, in millimetres AS TYPED: `Some` only
    /// once a keystroke has landed in it, and only while it holds the
    /// focus that keystroke arrived under. `None` otherwise, so a
    /// field nobody has typed into shows the δ actually in force —
    /// including one the triangle budget chose after this field last
    /// committed — and has nothing of its own to commit when the focus
    /// leaves it.
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
    pub(crate) datum_kind: DatumKindChoice,
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
    /// **The frame an axis-in-sketch is written in** — `None` until
    /// one is picked, for [`Self::profile_plane`]'s reason: the frame
    /// is a document node, so the form names one that exists.
    ///
    /// Its own pick rather than the profile form's, because the two
    /// forms are filled in separately. A revolve does need both nodes
    /// written against the SAME frame, which the form says beside the
    /// picker.
    pub(crate) datum_frame: Option<RecipeNodeId>,
    /// The axis-in-sketch form's point on the axis, metres, in the
    /// picked frame's 2-D coordinates.
    pub(crate) datum_in_frame_origin: Point2<f64>,
    /// Its direction in the same coordinates (unitless). Opens as the
    /// frame's +y, the axis a profile drawn beside it turns about.
    pub(crate) datum_in_frame_direction: [f64; 2],
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
    pub(crate) profile_path: Vec<Step<f64>>,
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
    /// **The path editor opened on a committed profile** — the
    /// add-profile form's step list, held for the edit door
    /// ([`Drafts::profile_edit`] loads it). `None` while no profile is
    /// selected, and dropped the moment the selection leaves the node
    /// it was loaded from ([`Drafts::abandon_profile_edit_off`]).
    pub(crate) profile_edit: Option<ProfileEdit>,
}

/// **One value per door of the profile editor** — the add-profile
/// form (`create`) and the same editor opened on a committed profile
/// (`edit`). What the frame loop takes per door — the loops, the
/// preview, whether the door drew — is one of these, so the two doors
/// run one pipeline rather than a pipeline and its twin.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ProfileDoors<T> {
    /// The add-profile form's.
    pub(crate) create: T,
    /// The edit door's.
    pub(crate) edit: T,
}

impl<T> ProfileDoors<T> {
    /// Each door's value through `f`.
    pub(crate) fn map<U>(self, mut f: impl FnMut(T) -> U) -> ProfileDoors<U> {
        ProfileDoors {
            create: f(self.create),
            edit: f(self.edit),
        }
    }

    /// Each door's value beside `other`'s for the same door.
    pub(crate) fn zip<U>(self, other: ProfileDoors<U>) -> ProfileDoors<(T, U)> {
        ProfileDoors {
            create: (self.create, other.create),
            edit: (self.edit, other.edit),
        }
    }

    /// Borrowed.
    pub(crate) fn as_ref(&self) -> ProfileDoors<&T> {
        ProfileDoors {
            create: &self.create,
            edit: &self.edit,
        }
    }

    /// Both doors' values, create first.
    pub(crate) fn into_array(self) -> [T; 2] {
        [self.create, self.edit]
    }
}

/// What one door's editor would preview: the frame it draws on (the
/// form's pick, `None` until one is made) and its loops.
#[derive(Clone, Debug)]
pub(crate) struct DoorLoops {
    /// The frame the loops are drawn on.
    pub(crate) frame: Option<RecipeNodeId>,
    /// The loops, in description order.
    pub(crate) loops: Vec<ProfileShape>,
}

impl DoorLoops {
    /// Whether `other` previews the same thing — the same frame and
    /// loops that lower alike ([`sketch::authors_same_loops`]).
    pub(crate) fn previews_as(&self, other: &Self) -> bool {
        self.frame == other.frame && sketch::authors_same_loops(&self.loops, &other.loops)
    }
}

/// **A committed profile's program, held in the add-profile form's
/// currency** — the kernel's [`Step`] at plain numbers, one list per
/// loop — so the one editor that authors a new profile edits this one.
///
/// It keeps the program it was loaded FROM, which is what an edit is
/// measured against ([`sketch::program_edits`]) and what tells the
/// holder the document has moved under it.
#[derive(Debug)]
pub(crate) struct ProfileEdit {
    /// The profile node.
    pub(crate) node: RecipeNodeId,
    /// The committed program the loops were loaded from.
    base: ProfileProgram,
    /// The loops as the editor holds them, in description order.
    pub(crate) loops: Vec<Vec<Step<f64>>>,
}

impl ProfileEdit {
    /// The frame the profile is drawn on — a reference the edit door
    /// does not rewrite.
    pub(crate) fn plane(&self) -> RecipeNodeId {
        self.base.plane
    }

    /// The committed program the loops were loaded from — what
    /// `SessionOp::EditProfile` carries so the door can refuse numbers
    /// loaded from a program the document no longer holds.
    pub(crate) fn base(&self) -> &ProfileProgram {
        &self.base
    }

    /// The held loops as the shapes the preview and the lowering take.
    pub(crate) fn shapes(&self) -> Vec<ProfileShape> {
        sketch::path_shapes(&self.loops)
    }

    /// The held loops lowered in `notation` — what
    /// `SessionOp::EditProfile` carries.
    ///
    /// # Errors
    ///
    /// [`sketch::loop_program`]'s: a non-finite field.
    pub(crate) fn programs(
        &self,
        notation: sketch::Notation,
    ) -> Result<Vec<LoopProgram>, RecordedProgramError> {
        sketch::loop_programs(&self.shapes(), notation)
    }

    /// **Whether applying would write anything** — the edit door's own
    /// question ([`sketch::program_edits`]) asked of the loaded
    /// program. A held state that does not lower, or does not have the
    /// committed program's structure, counts as moved: applying it is
    /// how the refusal is said.
    pub(crate) fn moved(&self) -> bool {
        let untouched = self
            .programs(sketch::Notation::CANONICAL)
            .ok()
            .and_then(|loops| sketch::program_edits(&self.base, &loops).ok())
            .is_some_and(|edits| edits.is_empty());
        !untouched
    }

    /// Put the loops back to the program they were loaded from.
    ///
    /// # Errors
    ///
    /// Never in practice — the base was loadable once — but typed
    /// rather than assumed: [`sketch::held_loops`]'s refusal.
    pub(crate) fn revert(&mut self, doc: &Doc<ProfileProgram>) -> Result<(), HeldRefusal> {
        self.loops = sketch::held_loops(doc, self.node)?;
        Ok(())
    }
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
            datum_kind: DatumKindChoice::Plane,
            datum_origin: [0.0; 3],
            datum_direction: [0.0, 0.0, 1.0],
            datum_u: [1.0, 0.0, 0.0],
            datum_v: [0.0, 1.0, 0.0],
            datum_frame: None,
            datum_in_frame_origin: Point2::origin(),
            datum_in_frame_direction: [0.0, 1.0],
            length_unit: quantity::M,
            angle_unit: quantity::PI,
            profile_shape: None,
            profile_path: vec![
                Step::At(Point2::origin()),
                Step::LineTo(Target::Point(Point2::new(0.01, 0.0))),
                Step::LineTo(Target::Point(Point2::new(0.01, 0.01))),
                Step::LineTo(Target::Point(Point2::new(0.0, 0.01))),
                Step::LineTo(Target::Start),
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
            profile_edit: None,
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

    /// **A form whose op the document ACCEPTED comes to rest** — called
    /// once per op a batch performed without a refusal.
    ///
    /// Only the add-profile form has anything to settle: its drafts ARE
    /// the viewport's preview (`sketch::preview` is replayed from them
    /// every frame), so a form left holding the shape it just committed
    /// would keep drawing that shape in the probe tint over the
    /// committed drawing of the node it became. Resting the shape
    /// (`None`) is what the form means by "nothing being composed"; the
    /// frame picked and the field values stay, so a second profile on
    /// the same frame starts from where the first one left off.
    ///
    /// A REFUSED add leaves the drafts alone, so correcting what was
    /// refused does not cost what was typed.
    ///
    /// The edit door needs nothing here: an accepted
    /// `SessionOp::EditProfile` changes the program its draft was
    /// loaded from, and the draft rebases onto the new one on its own
    /// (`Drafts::sync_profile_edit`, at the top of the next frame) —
    /// untouched again, and still open on the node, which is where the
    /// person is still working.
    pub(crate) fn accepted(&mut self, op: &SessionOp) {
        if matches!(op, SessionOp::AddProfile { .. }) {
            self.profile_shape = None;
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

    /// **The edit draft for the profile `node`** — the held one when
    /// it is this node's and the document still holds the program it
    /// was loaded from, a fresh load otherwise.
    ///
    /// A draft of another node, or of a program the document no longer
    /// holds (an undo, a redo, an apply, a slot edit from elsewhere),
    /// is replaced: its numbers were an edit of something that is not
    /// there any more, and applying them would write them over a
    /// program nobody was looking at. Compared by the program's VALUE,
    /// which is blind to notation, so a unit rewrite keeps the draft.
    ///
    /// # Errors
    ///
    /// [`HeldRefusal`] when the node is not a profile the editor can
    /// hold — and the held draft is dropped with it, so nothing stale
    /// survives the refusal.
    pub(crate) fn profile_edit(
        &mut self,
        doc: &Doc<ProfileProgram>,
        node: RecipeNodeId,
    ) -> Result<&mut ProfileEdit, HeldRefusal> {
        let current = match doc.node(node) {
            Some(Node::Profile(program)) => Some(program),
            _ => None,
        };
        let fresh = self
            .profile_edit
            .as_ref()
            .is_some_and(|held| held.node == node && current == Some(&held.base));
        if !fresh {
            self.profile_edit = None;
            let loops = sketch::held_loops(doc, node)?;
            let Some(base) = current else {
                unreachable!("`held_loops` loaded feature {} as a profile", node.0)
            };
            self.profile_edit = Some(ProfileEdit {
                node,
                base: base.clone(),
                loops,
            });
        }
        let Some(held) = self.profile_edit.as_mut() else {
            unreachable!("the edit draft was kept or loaded just above")
        };
        Ok(held)
    }

    /// **Bring the held edit draft up to the document, before anything
    /// reads it this frame**: dropped when the selection has left its
    /// node ([`Self::abandon_profile_edit_off`]), reloaded when the
    /// document no longer holds the program it was loaded from (an
    /// undo, a redo, an apply), dropped when that reload refuses. It
    /// never LOADS a draft that is not held — opening one is the
    /// pane's act ([`Self::profile_edit`]).
    pub(crate) fn sync_profile_edit(
        &mut self,
        doc: &Doc<ProfileProgram>,
        selected: Option<RecipeNodeId>,
    ) {
        self.abandon_profile_edit_off(selected);
        if let Some(node) = self.profile_edit.as_ref().map(|held| held.node) {
            // A refusal drops the draft (`profile_edit`'s contract) and
            // is said by the pane when it next draws.
            let _refused = self.profile_edit(doc, node).is_err();
        }
    }

    /// **What each door of the profile editor holds for its preview**:
    /// the add-profile form's frame pick and loops, and the loops of a
    /// committed profile opened for editing, on its own frame (`None`
    /// while none is open).
    pub(crate) fn door_loops(&self) -> ProfileDoors<Option<DoorLoops>> {
        ProfileDoors {
            create: Some(DoorLoops {
                frame: self.profile_plane,
                loops: self.profile_loops(),
            }),
            edit: self.profile_edit.as_ref().map(|edit| DoorLoops {
                frame: Some(edit.plane()),
                loops: edit.shapes(),
            }),
        }
    }

    /// **The committed profile the edit door is drawing in place of
    /// its committed loops** — the draft's node, when the edit door's
    /// preview this frame REPLAYED (`edit_preview`, taken from
    /// [`Self::door_loops`]'s `edit`). The committed-profile pass
    /// leaves it out (`sketch::committed`'s `except`), so the node
    /// shows only its live preview; a preview that refused draws
    /// nothing, and then the committed drawing stays up.
    pub(crate) fn edited_in_place(
        &self,
        edit_preview: Option<&Result<sketch::ProfilePreview, sketch::PreviewError>>,
    ) -> Option<RecipeNodeId> {
        self.profile_edit
            .as_ref()
            .filter(|_| matches!(edit_preview, Some(Ok(_))))
            .map(|edit| edit.node)
    }

    /// **Drop the edit draft unless `selected` is its node** — the
    /// selection moving off a profile abandons what was typed into it,
    /// as every draft here is abandoned by selecting elsewhere.
    pub(crate) fn abandon_profile_edit_off(&mut self, selected: Option<RecipeNodeId>) {
        if self
            .profile_edit
            .as_ref()
            .is_some_and(|held| Some(held.node) != selected)
        {
            self.profile_edit = None;
        }
    }

    /// The loop PROGRAMS the add-profile form would author right now:
    /// [`Drafts::profile_loops`] lowered in this form's notation, which
    /// is what the op carries.
    ///
    /// # Errors
    ///
    /// A non-finite field, or a path that is not a program's shape
    /// ([`sketch::loop_program`]'s refusals).
    pub(crate) fn profile_programs(&self) -> Result<Vec<LoopProgram>, RecordedProgramError> {
        sketch::loop_programs(&self.profile_loops(), self.notation())
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

    /// Two `Length` literals — a point in a sketch frame.
    ///
    /// # Errors
    ///
    /// A non-finite component.
    pub(crate) fn lengths2(&self, p: Point2<f64>) -> Result<[Expr; 2], DimensionError> {
        Ok([self.length(p.x)?, self.length(p.y)?])
    }

    /// **The add-datum form's drafts as a spec**, for the kind chosen:
    /// lengths in the form's notation, a normal or a direction
    /// dimensionless. `None` for an axis in a sketch whose frame is not
    /// picked yet — the form holds its button until it is.
    ///
    /// # Errors
    ///
    /// A non-finite component.
    pub(crate) fn datum_spec(&self) -> Result<Option<DatumSpec>, DimensionError> {
        Ok(Some(match self.datum_kind {
            DatumKindChoice::Plane => DatumSpec::Plane {
                origin: self.lengths(self.datum_origin)?,
                normal: scalars(self.datum_direction)?,
            },
            DatumKindChoice::Frame => DatumSpec::Frame {
                origin: self.lengths(self.datum_origin)?,
                u: scalars(self.datum_u)?,
                v: scalars(self.datum_v)?,
            },
            DatumKindChoice::Axis => DatumSpec::Axis {
                origin: self.lengths(self.datum_origin)?,
                direction: scalars(self.datum_direction)?,
            },
            DatumKindChoice::AxisInPlane => {
                let Some(plane) = self.datum_frame else {
                    return Ok(None);
                };
                DatumSpec::AxisInPlane {
                    plane,
                    origin: self.lengths2(self.datum_in_frame_origin)?,
                    direction: scalars2(self.datum_in_frame_direction)?,
                }
            }
            DatumKindChoice::Point => DatumSpec::Point {
                position: self.lengths(self.datum_origin)?,
            },
        }))
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

/// Two dimensionless literals — a direction in a sketch frame;
/// [`scalars`]' twin.
///
/// # Errors
///
/// A non-finite component.
pub(crate) fn scalars2(v: [f64; 2]) -> Result<[Expr; 2], DimensionError> {
    Ok([
        Expr::literal(v[0], Dimension::Scalar)?,
        Expr::literal(v[1], Dimension::Scalar)?,
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

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::document::{
        CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Expr, Node, ProfileProgram,
        RecipeNodeId, apply, evaluate,
    };
    use pncad::geom_core::{Point2, Tol};
    use pncad::profile::{Step, Target};

    use super::{Drafts, ProfileEdit};
    use crate::forms::{DatumKindChoice, ShapeKind};
    use crate::seats::Seat;
    use crate::session::SessionOp;
    use crate::session::author::datum_node;
    use crate::session::{NodeKindWanted, admits};
    use crate::sketch;

    /// **Every seat a datum fills can be filled from the add-datum
    /// form.** Each choice the form offers is lowered from its default
    /// drafts, with a frame picked, and every datum seat must admit at
    /// least one of the nodes that produces — the question the seat's
    /// own gate asks of a pick.
    ///
    /// The frame id is arbitrary: `admits` reads the node's kind, and
    /// whether the id names a frame is the add-datum door's question.
    #[test]
    fn every_datum_seat_is_fillable_from_the_add_datum_form() {
        let authorable: Vec<_> = DatumKindChoice::ALL
            .into_iter()
            .map(|(datum_kind, _)| {
                let drafts = Drafts {
                    datum_kind,
                    datum_frame: Some(RecipeNodeId(0)),
                    ..Drafts::default()
                };
                let spec = drafts.datum_spec().expect("the default drafts are finite");
                datum_node(spec.expect("a frame is picked"))
            })
            .collect();
        for seat in Seat::ALL {
            let wanted = seat.wants();
            match wanted {
                // Made by the add-profile form and by the ops that
                // produce bodies, not by this one.
                NodeKindWanted::Profile | NodeKindWanted::Body => continue,
                NodeKindWanted::Axis
                | NodeKindWanted::SketchAxis
                | NodeKindWanted::Plane
                | NodeKindWanted::Frame => {}
            }
            assert!(
                authorable.iter().any(|node| admits(Some(node), wanted)),
                "the {} seat wants {} and no add-datum choice authors one",
                seat.name(),
                wanted.name(),
            );
        }
    }

    /// **A profile the add form just committed is drawn once**, as the
    /// document's, and not a second time as the form's preview.
    ///
    /// The form's drafts are what the preview replays every frame, so
    /// a form still holding the shape it committed drew that shape in
    /// the probe tint over the committed drawing of its own node. This
    /// commits the form's own programs as the node `AddProfile` inserts,
    /// hands the op to `Drafts::accepted` as the app's batch does on an
    /// accepted op, and then asks both drawings the viewport makes: the
    /// committed pass holds the circle, and the preview replayed from
    /// the settled drafts holds nothing. The control is an op that adds
    /// no profile, which must leave the draft being composed.
    #[test]
    fn a_committed_profile_is_not_drawn_again_as_its_preview() {
        let tol = Tol::witness();
        let chord = 1.0e-4;
        let length = |v: f64| Expr::literal(v, Dimension::Length).expect("finite");
        let scalar = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");
        let frame = Node::Datum(Datum::Frame {
            origin: [length(0.0), length(0.0), length(0.0)],
            u: [scalar(1.0), scalar(0.0), scalar(0.0)],
            v: [scalar(0.0), scalar(1.0), scalar(0.0)],
        });
        let insert = |doc: &Doc<ProfileProgram>, node| {
            // A part-less fixture: no mate, no cluster, so the reach
            // is the refusing one and is never asked.
            let applied = apply(
                doc,
                &DocEdit::InsertNode { node },
                tol,
                &pncad::document::RefusingReach,
            )
            .expect("the fixture's edit applies");
            let id = applied.record.minted.expect("an insert mints an id");
            (applied.doc, id)
        };
        let (doc, plane) = insert(&Doc::empty_derived("drafts-accepted", tol), frame);
        let mut drafts = Drafts {
            profile_plane: Some(plane),
            profile_shape: Some(ShapeKind::Circle),
            ..Drafts::default()
        };

        // The control: an accepted op that adds no profile leaves the
        // form composing.
        drafts.accepted(&SessionOp::Hover(None));
        assert!(!drafts.profile_loops().is_empty(), "the draft was dropped");

        let loops = drafts
            .profile_programs()
            .expect("the default circle lowers");
        let (doc, _) = insert(
            &doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: loops.clone(),
            }),
        );
        drafts.accepted(&SessionOp::AddProfile { plane, loops });
        let evaluation = evaluate(
            &doc,
            None,
            &CancelToken::default(),
            &EvalOptions::default(),
            tol,
        );
        let placement =
            sketch::frame_placement(&doc, &evaluation, plane).expect("the frame has a placement");
        let committed = sketch::committed(&doc, &evaluation, chord, None);
        assert_eq!(
            committed.drawn.len(),
            1,
            "the circle is drawn as the document's"
        );
        let preview = sketch::preview(placement, &drafts.profile_loops(), tol, chord)
            .expect("an empty form previews");
        assert!(
            preview.loops.is_empty(),
            "the committed circle is drawn again as the form's preview"
        );
    }

    /// An axis in a sketch with no frame picked lowers to nothing,
    /// rather than to a spec naming some frame the person did not pick.
    #[test]
    fn an_axis_in_a_sketch_waits_for_its_frame() {
        let drafts = Drafts {
            datum_kind: DatumKindChoice::AxisInPlane,
            ..Drafts::default()
        };
        assert!(matches!(drafts.datum_spec(), Ok(None)));
    }

    /// A document holding one frame and the profile the add-profile
    /// form's DEFAULT path authors on it, lowered by the form's own
    /// `profile_programs` and inserted through the edit door the
    /// create button's op reaches. Answers the document, the drafts
    /// that authored it, and the profile node.
    fn authored_by_the_form() -> (Doc<ProfileProgram>, Drafts, RecipeNodeId) {
        use crate::forms::ShapeKind;
        use crate::session::DatumSpec;

        let len = |m: f64| Expr::literal(m, Dimension::Length).expect("finite");
        let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");
        let doc = Doc::empty_derived("drafts-edit", Tol::witness());
        let frame = datum_node(DatumSpec::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        });
        let doc = apply(
            &doc,
            &DocEdit::InsertNode { node: frame },
            Tol::witness(),
            &pncad::document::RefusingReach,
        )
        .expect("a frame inserts")
        .doc;
        let plane = *doc.order().last().expect("the frame");
        let drafts = Drafts {
            profile_shape: Some(ShapeKind::Path),
            profile_plane: Some(plane),
            ..Drafts::default()
        };
        let loops = drafts.profile_programs().expect("the default path lowers");
        let node = Node::Profile(ProfileProgram { plane, loops });
        let doc = apply(
            &doc,
            &DocEdit::InsertNode { node },
            Tol::witness(),
            &pncad::document::RefusingReach,
        )
        .expect("the form's default path is a profile")
        .doc;
        let profile = *doc.order().last().expect("the profile");
        (doc, drafts, profile)
    }

    /// `doc` with the slot writes the edit door would make for `edit`'s
    /// held loops — [`sketch::program_edits`], applied in order.
    fn applied(
        doc: &Doc<ProfileProgram>,
        edit: &ProfileEdit,
        notation: sketch::Notation,
    ) -> Doc<ProfileProgram> {
        let loops = edit.programs(notation).expect("finite");
        let Some(Node::Profile(current)) = doc.node(edit.node) else {
            panic!("the edited node is a profile")
        };
        let edits = sketch::program_edits(current, &loops).expect("same shape");
        assert_eq!(edits.len(), 1, "one argument moved: {edits:?}");
        edits.into_iter().fold(doc.clone(), |doc, (slot, expr)| {
            apply(
                &doc,
                &DocEdit::SetParam {
                    node: edit.node,
                    slot,
                    expr,
                },
                Tol::witness(),
                &pncad::document::RefusingReach,
            )
            .expect("the edit door takes it")
            .doc
        })
    }

    /// **The create form's profile opens in the edit door untouched**:
    /// the draft holds the form's own steps, says nothing moved, and
    /// what its Apply would send writes no argument.
    #[test]
    fn the_forms_profile_opens_untouched_and_would_write_nothing() {
        let (doc, mut drafts, profile) = authored_by_the_form();
        let authored = drafts.profile_loops();
        let notation = drafts.notation();
        let edit = drafts
            .profile_edit(&doc, profile)
            .expect("the editor holds the form's profile");
        assert!(!edit.moved(), "a fresh load has nothing to apply");
        assert!(sketch::authors_same_loops(&edit.shapes(), &authored));
        let loops = edit.programs(notation).expect("finite");
        let Some(Node::Profile(current)) = doc.node(profile) else {
            panic!("a profile")
        };
        assert_eq!(sketch::program_edits(current, &loops), Ok(Vec::new()));
    }

    /// **The draft follows the document, not the other way round.** A
    /// moved number is held until applied; the applied program is the
    /// new base, so the draft is untouched again; going back to the
    /// earlier document (an undo) replaces the draft with the program
    /// held there; and a selection that leaves the node drops it.
    #[test]
    fn the_edit_draft_reloads_on_undo_and_is_abandoned_off_selection() {
        let (before, mut drafts, profile) = authored_by_the_form();
        let notation = drafts.notation();
        let moved_to = Step::LineTo(Target::Point(Point2::new(0.02, 0.0)));
        let same_step = |a: Step<f64>, b: Step<f64>| {
            sketch::authors_same_loops(
                &sketch::path_shapes(&[vec![Step::At(Point2::origin()), a]]),
                &sketch::path_shapes(&[vec![Step::At(Point2::origin()), b]]),
            )
        };
        let edit = drafts.profile_edit(&before, profile).expect("held");
        edit.loops[0][1] = moved_to;
        assert!(edit.moved());
        // Held across frames while the document stands still.
        let edit = drafts.profile_edit(&before, profile).expect("held");
        assert!(edit.moved(), "the typed number survived a second read");
        let after = applied(&before, edit, notation);
        let edit = drafts.profile_edit(&after, profile).expect("held");
        assert!(!edit.moved(), "the applied program is the new base");
        assert!(same_step(edit.loops[0][1], moved_to));
        // Undo: the document the history steps back to.
        let edit = drafts.profile_edit(&before, profile).expect("held");
        assert!(!edit.moved());
        assert!(
            !same_step(edit.loops[0][1], moved_to),
            "the undone number is gone from the draft"
        );
        // Revert puts typed numbers back.
        edit.loops[0][2] = Step::LineTo(Target::Point(Point2::new(0.03, 0.03)));
        assert!(edit.moved());
        edit.revert(&before).expect("revertible");
        assert!(!edit.moved());
        drafts.abandon_profile_edit_off(Some(profile));
        assert!(drafts.profile_edit.is_some(), "still selected, still held");
        drafts.abandon_profile_edit_off(None);
        assert!(drafts.profile_edit.is_none(), "selection left, draft gone");
        // A node the editor cannot hold leaves nothing stale behind.
        assert!(drafts.profile_edit(&before, RecipeNodeId(0)).is_err());
        assert!(drafts.profile_edit.is_none());
    }

    /// **The profile being edited shows only its live preview.** The
    /// edit door's replayed preview stands in for the node, which the
    /// committed-profile pass then leaves out; a preview that refused
    /// stands in for nothing, and the committed drawing stays.
    #[test]
    fn the_edited_profile_is_drawn_only_as_its_preview() {
        let tol = Tol::witness();
        let chord = 1.0e-4;
        let (doc, mut drafts, profile) = authored_by_the_form();
        let evaluation = evaluate(
            &doc,
            None,
            &CancelToken::default(),
            &EvalOptions::default(),
            tol,
        );
        let drawn = |except| {
            sketch::committed(&doc, &evaluation, chord, except)
                .drawn
                .iter()
                .map(|profile| profile.node)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            drawn(None),
            vec![profile],
            "the control: drawn when nothing edits it"
        );
        drafts.profile_edit(&doc, profile).expect("held");
        let held = drafts.door_loops().edit.expect("the edit door holds loops");
        let plane = held.frame.expect("its own frame");
        let placement = sketch::frame_placement(&doc, &evaluation, plane).expect("placed");
        let preview = sketch::preview(placement, &held.loops, tol, chord);
        assert!(
            matches!(&preview, Ok(p) if !p.loops.is_empty()),
            "the live preview draws"
        );
        let except = drafts.edited_in_place(Some(&preview));
        assert_eq!(except, Some(profile));
        assert!(drawn(except).is_empty(), "the committed loops are left out");
        // A preview that refuses stands in for nothing.
        if let Some(edit) = drafts.profile_edit.as_mut() {
            edit.loops[0] = vec![Step::At(Point2::origin())];
        }
        let held = drafts.door_loops().edit.expect("held");
        let refused = sketch::preview(placement, &held.loops, tol, chord);
        assert!(refused.is_err(), "a one-point chain does not replay");
        assert_eq!(drafts.edited_in_place(Some(&refused)), None);
        assert_eq!(drafts.edited_in_place(None), None, "no preview taken");
    }
}

//! **The in-flight form state**: what a panel is mid-edit on, and its
//! lowering to the document's own types.
//!
//! A VOCABULARY module (`crates/viewer/README.md`, Module boundaries).
//! [`Drafts`] is layer-3 state that never enters the document — an
//! expression the user is typing is not an edit until they commit it —
//! and its methods turn typed field values into [`Expr`](editor_core::Expr)s and
//! [`LoopProgram`]s. Nothing here names `DocSession`, `ViewerApp` or
//! `egui`; the panels write these fields and read nothing back.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeMap;
use std::sync::{Arc, Weak};

use pncad::document::{
    Dimension, DimensionError, Doc, Evaluation, Formula, Label, LabelFault, LoopProgram,
    Maintenance, Node, ProfileProgram, RecipeNodeId, RecordedProgramError, SlotId, Speaker,
    StepArg, StepId, VarId, VarName,
};
use pncad::geom_core::{Point2, Tol};
use pncad::profile::{Step, Target};
use pncad::quantity::UnitDef;
use pncad::trace::TraceMarks;

use crate::blend::BlendKindChoice;
use crate::combine::PatternOutputChoice;
use crate::forms::{PatternLayoutChoice, ShapeKind};
use crate::history::HistoryId;
use crate::props::{self, Notation};
use crate::seats::SeatError;
use crate::session::{FaceSelection, ProfileShape, SessionOp, SketchTraceFault};
use crate::sketch::{self, HeldLoops, HeldRefusal};
use crate::tree::HeldPick;

/// Transient text a panel is mid-edit on.
///
/// Layer-3 state that never enters the document: an expression the
/// user is typing is not an edit until they commit it, and a draft
/// abandoned by selecting elsewhere leaves nothing behind.
#[derive(Debug)]
pub(crate) struct Drafts {
    /// The View pane's δ field, in the working length unit AS TYPED: `Some` only
    /// once a keystroke has landed in it, and only while it holds the
    /// focus that keystroke arrived under. `None` otherwise, so a
    /// field nobody has typed into shows the δ actually in force —
    /// including one the triangle budget chose after this field last
    /// committed — and has nothing of its own to commit when the focus
    /// leaves it.
    pub(crate) delta_text: Option<String>,
    /// The slot whose value field is holding REFUSED text.
    ///
    /// The field's text is egui's while it has focus and the
    /// document's afterwards, so there is only one thing this layer
    /// has to remember: text a parse refusal sent back
    /// ([`crate::frame::retype_draft`]). Acting on the refusal — going off to
    /// declare the variable it named — must not cost the text that
    /// raised it, so the field keeps showing it until an expression
    /// edit for that slot lands.
    pub(crate) expr_target: Option<(RecipeNodeId, SlotId)>,
    /// The refused text itself.
    pub(crate) expr_text: String,
    /// The naming field, while it is open ([`NameDraft`]).
    pub(crate) name_draft: Option<NameDraft>,
    /// **Whether the add-variable form is open** — its `New variable…`
    /// door clicked, or an unknown-variable refusal's offer landed
    /// ([`crate::frame::creation_offer`]). Closed, the form's fields
    /// below are not drawn.
    pub(crate) new_variable_open: bool,
    /// The add-variable form's name field.
    pub(crate) new_variable_name: String,
    /// Its chosen dimension — `None` until the user picks one, and
    /// the commit button waits for the pick. The offer path lands
    /// here from an expression whose context does not determine the
    /// new variable's dimension, and a silently-defaulted one would
    /// be a guess none of this program's doors make.
    pub(crate) new_variable_dimension: Option<Dimension>,
    /// Its value field.
    pub(crate) new_variable_value: f64,
    /// The name an unknown-variable refusal offered to create
    /// ([`crate::frame::creation_offer`]); shown over the form while the
    /// name field still says it.
    pub(crate) new_variable_offer: Option<VarName>,
    /// The mate tool's choice, as widget state: an index into
    /// [`crate::matetool::MATE_KINDS`], the flip toggle, and for each
    /// freedom of [`crate::matetool::MateFreedom::ALL`] whether it is set and its number in
    /// SI units. Draft chrome state only — the typed choice is minted
    /// at commit.
    pub(crate) mate_kind: usize,
    pub(crate) mate_flip: bool,
    /// Whether the mate moves its copy into the space its `to` reads,
    /// asked by name: off, a mate across two spaces refuses.
    pub(crate) mate_into_target_space: bool,
    pub(crate) mate_values: [(bool, f64); crate::matetool::MateFreedom::ALL.len()],
    /// The toolbar's New… form: `Some(text)` while the name field is
    /// open, `None` while it is not. The op is not emitted until
    /// its commit — a name in flight is a draft, not a document.
    pub(crate) new_doc_name: Option<String>,
    /// **Whether the add-profile form is open** — its `Add profile…`
    /// door clicked. Whether it is on screen is a different read, the
    /// frame's (`ProfileDoors::create`): a section folded over an open
    /// form hides it too, and what the viewport shows of the form
    /// follows that read ([`Self::marked_face`], [`Self::trace_wanted`]).
    pub(crate) profile_open: bool,
    /// **Whether the add-profile form draws on the selected face** —
    /// "sketch on this face" ([`SessionOp::SketchOnFace`]) — rather than
    /// on the profile's own plane alone.
    pub(crate) sketch_on_face: bool,
    /// **The face a sketch on a face is drawn on** — `None` until one
    /// is picked in the viewport.
    ///
    /// A LATCH of the viewport's face pick rather than a read of the
    /// live selection: a face is not a node a combo can list, so the
    /// selection IS this form's picker, and an author who picks a face
    /// and then clicks the feature tree to check something must not
    /// lose it. `crate::pane::create`'s on-face rows write it whenever
    /// the live selection is a face, so a second pick moves the seat.
    ///
    /// **Across documents it would be a hazard**: a `RecipeNodeId` is a
    /// small integer and a `StableName` carries no document identity,
    /// so a stale latch would commit against a same-numbered face of
    /// the next document. A document replacement therefore drops it
    /// ([`Self::document_replaced`]).
    ///
    /// Held as every face pick is ([`HeldPick`]), its nodes spoken again
    /// from each later document ([`Self::respeak`]).
    pub(crate) sketch_face: Option<HeldPick>,
    /// **Whether the path continues tangent to a face's trace** — a
    /// profile drawn on a body's plane ([`SessionOp::AddDrawnProfile`])
    /// whose chain writes [`Self::trace_gesture`] — rather than a
    /// profile on its own plane.
    pub(crate) trace_on: bool,
    /// The face whose body's plane the traced sketch is drawn on: a
    /// latch of the viewport's face pick, held as [`Self::sketch_face`] is.
    pub(crate) trace_plane: Option<HeldPick>,
    /// The face the sketch traces, latched the same way.
    pub(crate) trace_face: Option<HeldPick>,
    /// Where the trace steps go in the chain, and their branches.
    pub(crate) trace_gesture: sketch::TraceGesture,
    /// The trace the two latches show, as last read against the landed
    /// evaluation ([`Self::sync_trace_shown`]).
    pub(crate) trace_shown: Option<TraceShown>,
    /// The sketch on a face's depth, metres: how far it is extruded
    /// away from the face.
    pub(crate) sketch_depth: f64,
    /// The sketch on a face's spin, radians — its `+x`'s turn about the
    /// face's outward normal. Opens at zero.
    pub(crate) sketch_spin: f64,
    /// The split form's plane: a point on it, metres, in the target's
    /// own coordinates.
    pub(crate) split_origin: [f64; 3],
    /// The split form's plane normal (unitless).
    pub(crate) split_normal: [f64; 3],
    /// The split target the plane's point was last seeded for: a new
    /// target seeds it at that body's centre
    /// ([`crate::splitplane::centre`]), and the person's edits stand
    /// until the target changes.
    pub(crate) split_seeded: Option<pncad::document::Operand>,
    /// The add-profile form's shape choice — `None` until the user
    /// makes one.
    ///
    /// **Optional because the form is LIVE.** Every other draft here
    /// is a number sitting in a field, but this one decides what the
    /// viewport draws: the add-profile preview is taken from these
    /// drafts each frame, so a pre-selected shape meant that merely
    /// opening "Add feature" — to reach the extrude form, say — put a
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
    /// The revolve tool's point on its axis, metres, in the profile's
    /// own 2-D coordinates.
    pub(crate) revolve_axis_through: Point2<f64>,
    /// The revolve tool's axis heading from the profile's x axis,
    /// radians. Opens a quarter turn, along the profile's +y, the axis
    /// a profile drawn beside it turns about.
    pub(crate) revolve_axis_heading: f64,
    /// The transform tool's translation, metres.
    pub(crate) transform_translation: [f64; 3],
    /// Its rotation axis (unitless).
    pub(crate) transform_axis: [f64; 3],
    /// Its rotation angle, radians.
    pub(crate) transform_angle: f64,
    /// The pattern tool's rule choice.
    pub(crate) pattern_kind: PatternLayoutChoice,
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
    /// The circular rule's axis: a point on it, metres, in the
    /// patterned body's own coordinates.
    pub(crate) pattern_through: [f64; 3],
    /// The circular rule's axis direction (unitless).
    pub(crate) pattern_axis: [f64; 3],
    /// Which instance of a pattern it projects — an INTEGER all the
    /// way from the field, for [`Drafts::pattern_count`]'s reason: the
    /// slot it lands in is Count-typed and a number rounded on the way
    /// could differ from the one on screen.
    pub(crate) part_instance: i64,
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
    /// **The rename field's text, as typed**, and the node it renames:
    /// `Some` once a keystroke has landed, taken when the field
    /// commits. `None` shows the label the node has.
    pub(crate) label_text: Option<(RecipeNodeId, String)>,
    /// **A create form's label, as typed**, by the kind noun the form
    /// creates. A form nobody has typed into has no entry and shows
    /// the proposal ([`crate::tree::proposed_label`]) for that moment;
    /// the entry is taken when the form commits.
    pub(crate) creation_labels: BTreeMap<&'static str, String>,
}

/// **A label field's text, as the document's label**: a field that
/// shows nothing ([`Label::is_blank`]) clears (`None`); anything else
/// is held to the label rule.
///
/// # Errors
///
/// [`LabelFault`] for a text the rule refuses.
pub(crate) fn label_typed(text: &str) -> Result<Option<Label>, LabelFault> {
    if Label::is_blank(text) {
        return Ok(None);
    }
    Label::new(text).map(Some)
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

/// What one door's editor would preview: its loops, drawn on the
/// profile's own plane ([`sketch::profile_plane`]).
#[derive(Clone, Debug)]
pub(crate) struct DoorLoops {
    /// The loops, in description order.
    pub(crate) loops: Vec<ProfileShape>,
}

impl DoorLoops {
    /// Whether `other` previews the same thing — loops that lower
    /// alike ([`sketch::authors_same_loops`]).
    pub(crate) fn previews_as(&self, other: &Self) -> bool {
        sketch::authors_same_loops(&self.loops, &other.loops)
    }
}

/// **A committed profile's program, held in the add-profile form's
/// currency** — the kernel's [`Step`] at plain numbers, one list per
/// loop — so the one editor that authors a new profile edits this one.
///
/// It keeps the program it was loaded FROM, which is what tells the
/// holder the document has moved under it, and beside every held step
/// the committed step it was loaded as ([`Self::ids`]) — which is what
/// the edit door is told each step IS, so a name on a committed step
/// follows it through the reshaping or is reported stranded.
#[derive(Debug)]
pub(crate) struct ProfileEdit {
    /// The profile node.
    pub(crate) node: RecipeNodeId,
    /// The committed program the loops were loaded from, as written
    /// ([`sketch::written_program`]).
    base: ProfileProgram<Formula>,
    /// The loops as the editor holds them, in description order.
    loops: Vec<Vec<Step<f64>>>,
    /// Per held loop, per held step: the committed step it was loaded
    /// as, carried through every row edit ([`Self::edit_row`]); `None`
    /// for a row the editor made. The same shape as `loops`, always.
    loaded_as: Vec<Vec<Option<StepId>>>,
    /// The base program's steps as the editor first held them, by id
    /// — what [`Self::ids`] asks a held step whether it still draws.
    base_steps: BTreeMap<StepId, Step<f64>>,
    /// Every argument of the base program an expression drives, by
    /// its step's id and its role: the expression as written, which a
    /// kept step's lowering writes in place of the number it reads
    /// ([`Self::programs`]).
    driven: BTreeMap<StepId, Vec<(StepArg, Formula)>>,
    /// The edit door's report for the last held state it was asked
    /// about ([`Self::report`]).
    report: Option<HeldReport>,
}

/// What the edit door would report for one held state, and the state:
/// the history state it was asked over, the held program and its ids.
#[derive(Debug)]
struct HeldReport {
    at: HistoryId,
    loops: Vec<LoopProgram<Formula>>,
    ids: Vec<Vec<Option<StepId>>>,
    rows: Vec<Maintenance>,
}

impl ProfileEdit {
    /// The draft of `program`, held as `held` ([`sketch::held_loops`]
    /// of it), every step loaded as itself.
    fn load(node: RecipeNodeId, program: ProfileProgram<Formula>, held: HeldLoops) -> Self {
        let HeldLoops { loops, driven } = held;
        let loaded_as = program.kept_in_place();
        let shaped = program.ids.len() == loops.len()
            && program
                .ids
                .iter()
                .zip(&loops)
                .all(|(ids, steps)| ids.len() == steps.len());
        if !shaped {
            unreachable!(
                "a program at rest carries one id per authored step, and `held_loops` holds one \
                 step per authored step"
            )
        }
        let base_steps = program
            .ids
            .iter()
            .flatten()
            .copied()
            .zip(loops.iter().flatten().copied())
            .collect();
        let written = Node::Profile(program.clone());
        let mut by_step: BTreeMap<StepId, Vec<(StepArg, Formula)>> = BTreeMap::new();
        for slot in driven {
            let SlotId::Profile { loop_, step, arg } = slot else {
                unreachable!(
                    "a profile lists only profile slots, and {} is not one",
                    slot.label()
                )
            };
            let id = program.ids[loop_ as usize][step as usize];
            let Some(expr) = written.expr(slot) else {
                unreachable!(
                    "`held_loops` listed {} as driven, so it is written",
                    slot.label()
                )
            };
            by_step.entry(id).or_default().push((arg, expr.clone()));
        }
        Self {
            node,
            base: program,
            loops,
            loaded_as,
            base_steps,
            driven: by_step,
            report: None,
        }
    }

    /// The committed program the loops were loaded from — what
    /// `SessionOp::EditProfile` carries so the door can refuse numbers
    /// loaded from a program the document no longer holds.
    pub(crate) fn base(&self) -> &ProfileProgram<Formula> {
        &self.base
    }

    /// The held loops, in description order.
    pub(crate) fn loops(&self) -> &[Vec<Step<f64>>] {
        &self.loops
    }

    /// One held loop's steps, for their fields to write — a slice, so
    /// the list's SHAPE changes only through [`Self::edit_row`].
    pub(crate) fn steps_mut(&mut self, loop_: usize) -> &mut [Step<f64>] {
        &mut self.loops[loop_]
    }

    /// Apply a row edit to held loop `loop_`, and to what each of its
    /// rows was loaded as.
    pub(crate) fn edit_row(&mut self, loop_: usize, edit: RowEdit) {
        edit.shape(&mut self.loaded_as[loop_], |_| None);
        edit.apply(&mut self.loops[loop_]);
    }

    /// **Which committed step each held step is** — `SetProgram`'s
    /// `ids`: per loop, per step, the id it was loaded as while it
    /// still draws that step's pieces (`draws_as`), else `None`, so
    /// a kept id never hands a name a piece of another role.
    pub(crate) fn ids(&self) -> Vec<Vec<Option<StepId>>> {
        self.loops
            .iter()
            .zip(&self.loaded_as)
            .map(|(steps, loaded_as)| {
                steps
                    .iter()
                    .zip(loaded_as)
                    .map(|(step, id)| {
                        let id = (*id)?;
                        let was = self.base_steps.get(&id)?;
                        draws_as(was, step).then_some(id)
                    })
                    .collect()
            })
            .collect()
    }

    /// **What committing the held program would report**, asked of
    /// `door` — the edit door's own rows for the `SetProgram` it would
    /// be — once per held state: the answer is kept until the program,
    /// its ids or the history state `at` moves. A held state that does
    /// not lower reports nothing; its refusal is the preview's.
    pub(crate) fn report(
        &mut self,
        at: HistoryId,
        door: impl FnOnce(Vec<LoopProgram<Formula>>, Vec<Vec<Option<StepId>>>) -> Vec<Maintenance>,
    ) -> &[Maintenance] {
        let Ok(loops) = self.programs(Notation::CANONICAL) else {
            return &[];
        };
        let ids = self.ids();
        let fresh = self
            .report
            .as_ref()
            .is_some_and(|held| held.at == at && held.loops == loops && held.ids == ids);
        if !fresh {
            let rows = door(loops.clone(), ids.clone());
            self.report = Some(HeldReport {
                at,
                loops,
                ids,
                rows,
            });
        }
        self.report.as_ref().map_or(&[], |held| &held.rows)
    }

    /// The held loops as the shapes the preview and the lowering take.
    pub(crate) fn shapes(&self) -> Vec<ProfileShape> {
        sketch::path_shapes(&self.loops)
    }

    /// **Which committed step each held row is, and where it stands**:
    /// per loop, per row, the id it is kept as ([`Self::ids`]) and that
    /// step's committed `(loop, step)`; `None` for a row that is no
    /// committed step. An argument's committed slot is that position
    /// with its role, where the committed step has that role.
    fn kept(&self) -> Vec<Vec<Kept>> {
        let position: BTreeMap<StepId, (u32, u32)> = self
            .base
            .ids
            .iter()
            .enumerate()
            .flat_map(|(loop_, ids)| {
                ids.iter()
                    .enumerate()
                    .map(move |(step, id)| (*id, (program_index(loop_), program_index(step))))
            })
            .collect();
        self.ids()
            .iter()
            .map(|ids| {
                ids.iter()
                    .map(|id| {
                        let id = (*id)?;
                        Some((id, *position.get(&id)?))
                    })
                    .collect()
            })
            .collect()
    }

    /// [`Self::kept`]'s positions, for held loop `loop_`.
    pub(crate) fn kept_at(&self, loop_: usize) -> Vec<Option<(u32, u32)>> {
        self.kept()[loop_]
            .iter()
            .map(|kept| kept.map(|(_, at)| at))
            .collect()
    }

    /// **Every driven argument of a kept step that the held step no
    /// longer has** — a re-moded arc dropping the role an expression
    /// drives — as the committed slot. The edit door cannot write the
    /// program without writing over that expression, and its field is
    /// locked, so it says what to do instead.
    pub(crate) fn dropped_drivers(&self) -> Vec<SlotId> {
        let Ok(loops) = self.programs(Notation::CANONICAL) else {
            return Vec::new();
        };
        let held = Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        });
        let mut dropped = Vec::new();
        for (loop_, rows) in self.kept().iter().enumerate() {
            for (row, kept) in rows.iter().enumerate() {
                let Some((id, (at_loop, at_step))) = *kept else {
                    continue;
                };
                for (arg, _) in self.driven.get(&id).into_iter().flatten() {
                    let here = SlotId::Profile {
                        loop_: program_index(loop_),
                        step: program_index(row),
                        arg: *arg,
                    };
                    if held.expr(here).is_none() {
                        dropped.push(SlotId::Profile {
                            loop_: at_loop,
                            step: at_step,
                            arg: *arg,
                        });
                    }
                }
            }
        }
        dropped
    }

    /// **Read every driven argument off `landed`** — the document on
    /// screen, a gesture's scratch included — into the held number
    /// beside its expression. A driven field is locked, so the draft
    /// never holds a person's number there: its number is the
    /// expression's reading, and it moves when what the expression
    /// reads moves (a variable set, dragged, undone), which is what the
    /// preview draws. A reading that does not evaluate leaves the
    /// number as it was, and so does one with a value per member; the
    /// door says why it waits
    /// ([`Self::unread_drivers`]).
    pub(crate) fn read_drivers(&mut self, landed: &Doc<ProfileProgram>, tol: Tol) {
        let rows = props::slot_rows(landed, self.node);
        let reading = |slot: SlotId| {
            let row = rows.iter().find(|row| row.slot == slot)?;
            row.driver.is_driven().then_some(())?;
            row.value.as_ref().ok()?.one().map(|value| value.as_f64())
        };
        let kept = self.kept();
        for (loop_, rows) in kept.iter().enumerate() {
            for (row, kept) in rows.iter().enumerate() {
                let Some((id, (at_loop, at_step))) = *kept else {
                    continue;
                };
                let readings: Vec<(StepArg, f64)> = self
                    .driven
                    .get(&id)
                    .into_iter()
                    .flatten()
                    .filter_map(|(arg, _)| {
                        let slot = SlotId::Profile {
                            loop_: at_loop,
                            step: at_step,
                            arg: *arg,
                        };
                        Some((*arg, reading(slot)?))
                    })
                    .collect();
                if readings.is_empty() {
                    continue;
                }
                if let Some(step) = sketch::with_readings(self.loops[loop_][row], &readings, tol) {
                    self.loops[loop_][row] = step;
                }
            }
        }
    }

    /// **Every driven argument of a kept step that reads no one value
    /// in `landed`**, with why: Apply waits on them, since the number
    /// beside them reads nothing. Its expression did not evaluate (a
    /// variable it reads deleted, say), or it reads an index and has a
    /// value per member where a step holds one.
    pub(crate) fn unread_drivers(&self, landed: &Doc<ProfileProgram>) -> Vec<(SlotId, String)> {
        let rows = props::slot_rows(landed, self.node);
        self.kept()
            .iter()
            .flatten()
            .flatten()
            .flat_map(|(id, (loop_, step))| {
                self.driven
                    .get(id)
                    .into_iter()
                    .flatten()
                    .map(move |(arg, _)| SlotId::Profile {
                        loop_: *loop_,
                        step: *step,
                        arg: *arg,
                    })
            })
            .filter_map(|slot| {
                let row = rows.iter().find(|row| row.slot == slot)?;
                let why = match &row.value {
                    Ok(props::Reading::One(_)) => return None,
                    Ok(props::Reading::PerMember(members)) => {
                        let indices: Vec<String> =
                            members.over().iter().map(ToString::to_string).collect();
                        format!(
                            "it reads index {}, so it has a value per member and the editor holds one",
                            indices.join(", ")
                        )
                    }
                    Err(fault) => fault.to_string(),
                };
                Some((slot, why))
            })
            .collect()
    }

    /// The held loops lowered in `notation` — what
    /// `SessionOp::EditProfile` carries ([`Self::programs_noted`],
    /// noting nothing).
    ///
    /// # Errors
    ///
    /// [`sketch::loop_program`]'s: a non-finite field.
    pub(crate) fn programs(
        &self,
        notation: Notation,
    ) -> Result<Vec<LoopProgram<Formula>>, RecordedProgramError> {
        self.programs_noted(notation, &|_| None)
    }

    /// **The held loops lowered**, each argument of a kept step in the
    /// unit `noted` says its committed slot is written in, every other
    /// in `notation` — and every driven argument of a kept step written
    /// as its expression, which is what the number beside it reads.
    ///
    /// The expression is written wherever the kept step still has the
    /// role: a driven field holds its number locked, so there is no
    /// moved value it could be standing over. A step whose arc mode
    /// dropped the role holds no such argument
    /// ([`Self::dropped_drivers`]).
    ///
    /// # Errors
    ///
    /// [`sketch::loop_program`]'s: a non-finite field.
    pub(crate) fn programs_noted(
        &self,
        notation: Notation,
        noted: &dyn Fn(SlotId) -> Option<UnitDef>,
    ) -> Result<Vec<LoopProgram<Formula>>, RecordedProgramError> {
        let kept = self.kept();
        let loops = self
            .loops
            .iter()
            .zip(&kept)
            .map(|(steps, kept)| {
                sketch::path_program(steps, notation, &|step, arg| {
                    let (_, (loop_, step)) = (*kept.get(usize::try_from(step).ok()?)?)?;
                    noted(SlotId::Profile { loop_, step, arg })
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut held = Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        });
        for (loop_, rows) in kept.iter().enumerate() {
            for (step, kept) in rows.iter().enumerate() {
                let Some(driven) = kept.and_then(|(id, _)| self.driven.get(&id)) else {
                    continue;
                };
                for (arg, expr) in driven {
                    let slot = SlotId::Profile {
                        loop_: program_index(loop_),
                        step: program_index(step),
                        arg: *arg,
                    };
                    if let Some(written) = held.expr_mut(slot) {
                        *written = expr.clone();
                    }
                }
            }
        }
        let Node::Profile(program) = held else {
            unreachable!("the held program was built as a profile node")
        };
        Ok(program.loops)
    }

    /// **Whether the held state differs from the committed program at
    /// all**: it edits it, or it cannot be read as it. The first is
    /// the held program, every step kept where it was loaded, not
    /// being the committed one (by value, blind to notation — the
    /// door's own no-op); the second is a held state that does not
    /// lower, and applying it writes nothing — it is how its refusal
    /// is said.
    pub(crate) fn differs_from_base(&self) -> bool {
        let untouched = self
            .programs(Notation::CANONICAL)
            .is_ok_and(|loops| sketch::is_committed(&self.base, &loops, &self.ids()));
        !untouched
    }

    /// Put the loops back to the program they were loaded from.
    ///
    /// # Errors
    ///
    /// Never in practice — the base was loadable once — but typed
    /// rather than assumed: [`sketch::held_loops`]'s refusal.
    pub(crate) fn revert(&mut self, doc: &Doc<ProfileProgram>) -> Result<(), HeldRefusal> {
        let held = sketch::held_loops(doc, self.node)?;
        *self = Self::load(self.node, self.base.clone(), held);
        Ok(())
    }
}

/// A held row as a committed step: its id, and its committed
/// `(loop, step)` ([`ProfileEdit::kept`]); `None` for a row that is no
/// committed step.
type Kept = Option<(StepId, (u32, u32))>;

/// A position in a profile program as the slot vocabulary spells it.
fn program_index(i: usize) -> u32 {
    let Ok(narrowed) = u32::try_from(i) else {
        unreachable!(
            "position {i}: a profile program is addressed by `SlotId`'s u32 positions, and a held \
             program is one loaded from the document and edited a row per click"
        )
    };
    narrowed
}

/// Whether `now` draws the pieces `was` drew: the same verb, whose
/// role list it draws from, and the same piece count
/// ([`Step::pieces`]).
fn draws_as(was: &Step<f64>, now: &Step<f64>) -> bool {
    was.verb() == now.verb() && was.pieces() == now.pieces()
}

/// **A change to the SHAPE of one step list** — what a step row's
/// glyph controls, its verb picker and the list's own controls ask
/// for, applied once the list has been drawn (a list cannot be
/// reshaped while its rows are being iterated).
#[derive(Clone, Debug)]
pub(crate) enum RowEdit {
    /// Row `index` becomes `step`: a fresh step of the verb the row's
    /// picker chose, with no number carried across.
    Replace {
        /// The row.
        index: usize,
        /// The fresh step.
        step: Step<f64>,
    },
    /// Row `index` goes.
    Remove(usize),
    /// `step` is inserted at `at`.
    Insert {
        /// Where it lands.
        at: usize,
        /// The fresh step.
        step: Step<f64>,
    },
    /// Two rows trade places.
    Swap(usize, usize),
    /// Every row goes.
    Clear,
}

impl RowEdit {
    /// The edit, made to `steps`.
    pub(crate) fn apply(&self, steps: &mut Vec<Step<f64>>) {
        self.shape(steps, |step| *step);
    }

    /// The edit made to a list of one entry per row, where a replaced
    /// or inserted row holds `fresh` of its step.
    fn shape<T>(&self, rows: &mut Vec<T>, fresh: impl FnOnce(&Step<f64>) -> T) {
        match self {
            Self::Replace { index, step } => rows[*index] = fresh(step),
            Self::Remove(index) => {
                rows.remove(*index);
            }
            Self::Insert { at, step } => rows.insert(*at, fresh(step)),
            Self::Swap(a, b) => rows.swap(*a, *b),
            Self::Clear => rows.clear(),
        }
    }
}

/// **The naming field**: the slot it is drawn under, the variable it
/// was opened for, and the text in it — empty when it opens, since
/// nothing proposes a name (Ev, PR 4247), and stored in the document
/// only when the person commits it (VR2).
///
/// **It names the variable it was opened for, never the slot's reader
/// at commit.** The field stands only while that variable is still
/// unnamed and still the one it was opened for: the slot's own, or the
/// unnamed variable on offer there (`share`). An accepted offer, a
/// retype or an undo closes it, so a name typed for one variable can
/// never land on another.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NameDraft {
    /// The node whose row the field is drawn under.
    pub(crate) node: RecipeNodeId,
    /// The slot of that row.
    pub(crate) slot: SlotId,
    /// The variable the field was opened for, and the one it names.
    pub(crate) var: VarId,
    /// Whether the field was opened by accepting an offer of an unnamed
    /// variable: its commit names the variable and makes the slot read
    /// it, as one step, since a variable two slots share has a name
    /// (VR2).
    pub(crate) share: bool,
    /// The text in the field.
    pub(crate) text: String,
}

impl Default for Drafts {
    /// The creation forms' sensible defaults (the GAUTH-1 spec): a
    /// 10 mm circle or rectangle, a 10 mm extrude, a full-turn revolve,
    /// a split plane and a pattern axis through the origin along +z.
    /// Everything else starts empty.
    fn default() -> Self {
        Self {
            delta_text: None,
            expr_target: None,
            expr_text: String::new(),
            name_draft: None,
            new_variable_name: String::new(),
            new_variable_open: false,
            new_variable_dimension: None,
            new_variable_value: 0.0,
            new_variable_offer: None,
            mate_kind: 0,
            // Two picked faces rest against each other: the plain plane
            // mate equates outward normals, so a rest carries the flip.
            mate_flip: true,
            mate_into_target_space: false,
            mate_values: [(false, 0.0); crate::matetool::MateFreedom::ALL.len()],
            new_doc_name: None,
            profile_open: false,
            sketch_on_face: false,
            sketch_face: None,
            trace_on: false,
            trace_plane: None,
            trace_face: None,
            trace_gesture: sketch::TraceGesture::default(),
            trace_shown: None,
            sketch_depth: 0.01,
            sketch_spin: 0.0,
            split_origin: [0.0; 3],
            split_normal: [0.0, 0.0, 1.0],
            split_seeded: None,
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
            revolve_axis_through: Point2::origin(),
            revolve_axis_heading: core::f64::consts::FRAC_PI_2,
            transform_translation: [0.0; 3],
            transform_axis: [0.0, 0.0, 1.0],
            transform_angle: 0.0,
            pattern_kind: PatternLayoutChoice::Linear,
            pattern_output: PatternOutputChoice::Instances,
            pattern_count: 3,
            // The one home both stepping gestures open on.
            pattern_direction: crate::combine::STEP_DIRECTION,
            pattern_spacing: 0.02,
            pattern_step: core::f64::consts::FRAC_PI_2,
            pattern_through: [0.0; 3],
            pattern_axis: [0.0, 0.0, 1.0],
            // The FIRST instance, which is the copy rather than the
            // original: a projection of instance 0 is the master where
            // it already stands, so the index a person opening this
            // form wants is the one that selects something new.
            part_instance: 1,
            blend_kind: BlendKindChoice::Fillet,
            blend_size: 0.001,
            profile_edit: None,
            label_text: None,
            creation_labels: BTreeMap::new(),
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

    /// **The add-variable form closes**, at its Cancel or once its
    /// declaration is accepted ([`Self::accepted`]), and its fields go
    /// with it: the next `New variable…` starts empty.
    pub(crate) fn close_new_variable(&mut self) {
        self.new_variable_open = false;
        self.new_variable_name.clear();
        self.new_variable_dimension = None;
        self.new_variable_value = 0.0;
        self.new_variable_offer = None;
    }

    /// **A form whose op the document ACCEPTED comes to rest** — called
    /// once per op a batch performed without a refusal.
    ///
    /// The add-profile form settles its preview: its drafts ARE
    /// the viewport's preview (`sketch::preview` is replayed from them
    /// every frame), so a form left holding the shape it just committed
    /// would keep drawing that shape in the probe tint over the
    /// committed drawing of the node it became. Resting the shape
    /// (`None`) is what the form means by "nothing being composed", and
    /// the form closes with its one committed edit, as a tool does; the
    /// field values stay, so a second profile starts from where the
    /// first one left off. A sketch on a face settles the same way, and
    /// the add-variable form closes on its own declaration.
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
        // A labelled creation settles the form its creation came from.
        let op = match op {
            SessionOp::CreateLabelled { creation, .. } => creation.op(),
            op => op,
        };
        if matches!(
            op,
            SessionOp::AddProfile { .. }
                | SessionOp::SketchOnFace { .. }
                | SessionOp::AddDrawnProfile { .. }
        ) {
            self.profile_shape = None;
            self.profile_open = false;
        }
        // The add-variable form's own declaration landed: the one
        // naming the variable its field still says.
        if let SessionOp::DeclareVar { name, .. } = op
            && self.new_variable_open
            && name.as_str() == self.new_variable_name.trim()
        {
            self.close_new_variable();
        }
    }

    /// **The rename field is showing `shown`'s label** (`None`: no
    /// rename field is showing). A rename typed for any other node is
    /// dropped, so it can neither reappear on a later visit nor commit
    /// against a node it was not typed for.
    pub(crate) fn rename_shown_for(&mut self, shown: Option<RecipeNodeId>) {
        if self
            .label_text
            .as_ref()
            .is_some_and(|(typed_for, _)| Some(*typed_for) != shown)
        {
            self.label_text = None;
        }
    }

    /// **A creation landed**: the label its form held is spent, so the
    /// next creation of that kind proposes afresh. The form is found by
    /// the kind noun of the node the action made
    /// ([`crate::world::made`]) — the noun a
    /// form's label field is keyed by, which is that node's kind
    /// (`creation_nouns`). Called only for an op that committed, so a
    /// refused creation keeps what was typed.
    pub(crate) fn creation_landed(&mut self, doc: &Doc<ProfileProgram>, minted: &[RecipeNodeId]) {
        if let Some(node) = crate::world::made(doc, minted).and_then(|id| doc.node(id)) {
            self.creation_labels
                .remove(pncad::document::node_kind_noun(node));
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
            Some(Node::Profile(program)) => Some(sketch::written_program(doc, program)),
            _ => None,
        };
        let fresh = self
            .profile_edit
            .as_ref()
            .is_some_and(|held| held.node == node && current.as_ref() == Some(&held.base));
        if !fresh {
            self.profile_edit = None;
            let held = sketch::held_loops(doc, node)?;
            let Some(base) = current else {
                unreachable!("`held_loops` loaded node {} as a profile", node)
            };
            self.profile_edit = Some(ProfileEdit::load(node, base, held));
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
    /// pane's act ([`Self::profile_edit`]). Then every driven argument
    /// reads the `landed` document ([`ProfileEdit::read_drivers`]), so
    /// what the expression reads moving moves the held outline.
    pub(crate) fn sync_profile_edit(
        &mut self,
        doc: &Doc<ProfileProgram>,
        landed: &Doc<ProfileProgram>,
        tol: Tol,
        selected: Option<RecipeNodeId>,
    ) {
        self.abandon_profile_edit_off(selected);
        if let Some(node) = self.profile_edit.as_ref().map(|held| held.node) {
            // A refusal drops the draft (`profile_edit`'s contract) and
            // is said by the pane when it next draws.
            let _refused = self.profile_edit(doc, node).is_err();
        }
        if let Some(edit) = self.profile_edit.as_mut() {
            edit.read_drivers(landed, tol);
        }
    }

    /// **What each door of the profile editor holds for its preview**:
    /// the add-profile form's loops, and the loops of a committed
    /// profile opened for editing (`None` while none is open).
    pub(crate) fn door_loops(&self) -> ProfileDoors<Option<DoorLoops>> {
        ProfileDoors {
            create: Some(DoorLoops {
                loops: self.profile_loops(),
            }),
            edit: self.profile_edit.as_ref().map(|edit| DoorLoops {
                loops: edit.shapes(),
            }),
        }
    }

    /// **The committed profile the edit door is drawing in place of
    /// its committed loops** — the draft's node, whenever the edit
    /// door took a preview this frame (`edit_preview`, taken from
    /// [`Self::door_loops`]'s `edit`). The committed-profile pass
    /// leaves it out (`sketch::committed`'s `except`), so the node
    /// shows only the edit: what its preview draws, which for a
    /// refused step is the prefix before it and may be nothing, beside
    /// the form's sentence. Whether the committed shape shows does not
    /// turn on whether a prefix happened to replay.
    pub(crate) fn edited_in_place(
        &self,
        edit_preview: Option<&Result<sketch::ProfilePreview, sketch::PreviewError>>,
    ) -> Option<RecipeNodeId> {
        self.profile_edit
            .as_ref()
            .filter(|_| edit_preview.is_some())
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
    /// [`Drafts::profile_loops`] lowered in the working `notation`, which
    /// is what the op carries.
    ///
    /// # Errors
    ///
    /// A non-finite field, or a path that is not a program's shape
    /// ([`sketch::loop_program`]'s refusals).
    pub(crate) fn profile_programs(
        &self,
        notation: Notation,
    ) -> Result<Vec<LoopProgram<Formula>>, RecordedProgramError> {
        sketch::loop_programs(&self.profile_loops(), notation)
    }

    /// **Another document replaced the one the held face was picked
    /// in** (an open, a new document): the latch names nothing in it.
    pub(crate) fn document_replaced(&mut self) {
        self.trace_plane = None;
        self.trace_face = None;
        self.sketch_face = None;
    }

    /// **The held picks' nodes, spoken again from `doc`**, the session's
    /// shown document after an operation (`SpokenNode::respoken`'s
    /// rule): a node `doc` holds takes its label now, and one it no
    /// longer holds keeps the last it had.
    pub(crate) fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        for held in [
            &mut self.sketch_face,
            &mut self.trace_plane,
            &mut self.trace_face,
        ]
        .into_iter()
        .flatten()
        {
            *held = held.respoken(doc);
        }
    }

    /// The mate panel's chosen kind ([`crate::matetool::MATE_KINDS`]).
    pub(crate) fn mate_kind(&self) -> pncad::document::VarKind {
        crate::matetool::MATE_KINDS
            .get(self.mate_kind)
            .map_or(pncad::document::VarKind::Plane, |(kind, _)| *kind)
    }

    /// **The face this form holds**: [`Self::sketch_face`] while the
    /// add-profile form draws on a face, `None` otherwise.
    ///
    /// Off the face the latch is kept, not held — nothing the form
    /// commits reads it — so switching off releases the pick, and
    /// switching back holds it again.
    pub(crate) fn held_face(&self) -> Option<&HeldPick> {
        self.sketch_face.as_ref().filter(|_| self.sketch_on_face)
    }

    /// **The face the viewport marks as held** (`crate::marks::Held`):
    /// [`Self::held_face`] while the form was `drawn` on the last frame
    /// (`ProfileDoors::create`), the one read of whether the form is
    /// on screen — a form behind its door, in a folded section or in a
    /// pane not drawn holds nothing a reader can see.
    pub(crate) fn marked_face(&self, drawn: bool) -> Option<&FaceSelection> {
        self.held_face().filter(|_| drawn).map(HeldPick::face)
    }

    /// **The latches a trace is to be read for** — when the form was
    /// `drawn` on the last frame (the read [`Self::marked_face`] takes)
    /// and writes a traced path, both are held, and the trace shown was
    /// not read for them against `landed`. Drops the trace shown
    /// otherwise.
    pub(crate) fn trace_wanted(
        &mut self,
        drawn: bool,
        landed: Option<&Arc<Evaluation<f64>>>,
    ) -> Option<(FaceSelection, FaceSelection)> {
        let (true, Some(plane), Some(face)) = (
            drawn && self.traces(),
            self.trace_plane.as_ref().map(HeldPick::face),
            self.trace_face.as_ref().map(HeldPick::face),
        ) else {
            self.trace_shown = None;
            return None;
        };
        let current = self.trace_shown.as_ref().is_some_and(|shown| {
            shown.plane == *plane
                && shown.face == *face
                && landed.is_some_and(|ev| Weak::ptr_eq(&shown.against, &Arc::downgrade(ev)))
        });
        (!current).then(|| (plane.clone(), face.clone()))
    }

    /// **The trace of `plane` and `face`, read against `landed`**
    /// ([`crate::session::DocSession::sketch_trace`]), held for the
    /// form and the viewport to show.
    pub(crate) fn show_trace(
        &mut self,
        (plane, face): (FaceSelection, FaceSelection),
        landed: Option<&Arc<Evaluation<f64>>>,
        read: Result<TraceMarks, SketchTraceFault>,
    ) {
        self.trace_shown = Some(TraceShown {
            plane,
            face,
            against: landed.map_or_else(Weak::new, Arc::downgrade),
            read,
        });
    }

    /// **Whether the add-profile form writes a traced path**: a path,
    /// continuing tangent to a face's trace ([`Self::trace_on`]), and
    /// not on a picked face.
    pub(crate) fn traces(&self) -> bool {
        self.trace_on && !self.sketch_on_face && self.profile_shape == Some(ShapeKind::Path)
    }

    /// **The add-profile form's op**: the profile on its own plane, or
    /// — drawing on a face — the sketch on the held face, `Ok(None)`
    /// while no face is held.
    ///
    /// # Errors
    ///
    /// A non-finite field, or a path that is not a program's shape.
    pub(crate) fn profile_op(&self, notation: Notation) -> Result<Option<SessionOp>, CommitFault> {
        if self.traces() {
            let (Some(plane), Some(face)) = (&self.trace_plane, &self.trace_face) else {
                return Ok(None);
            };
            let chain = sketch::traced_loop(&self.profile_path, &self.trace_gesture, 0, notation)
                .map_err(CommitFault::Traced)?;
            return Ok(Some(SessionOp::AddDrawnProfile {
                plane: plane.face().clone(),
                traced: vec![face.face().clone()],
                chains: vec![chain],
            }));
        }
        let loops = self.profile_programs(notation)?;
        if !self.sketch_on_face {
            return Ok(Some(SessionOp::AddProfile { loops }));
        }
        let Some(face) = self.held_face() else {
            return Ok(None);
        };
        Ok(Some(SessionOp::SketchOnFace {
            face: face.face().clone(),
            loops,
            distance: notation.length_literal(self.sketch_depth)?,
            spin: notation.angle_literal(self.sketch_spin)?,
        }))
    }
}

/// Three dimensionless literals — a normal, a direction, a rotation
/// axis. Not a [`Drafts`] method, because there is no notation to
/// carry from the form: a dimensionless number has one spelling, and
/// `Formula::literal` stores that row itself.
///
/// # Errors
///
/// A non-finite component.
pub(crate) fn scalars(v: [f64; 3]) -> Result<[Formula; 3], DimensionError> {
    Ok([
        Formula::literal(v[0], Dimension::Scalar)?,
        Formula::literal(v[1], Dimension::Scalar)?,
        Formula::literal(v[2], Dimension::Scalar)?,
    ])
}

/// **A face's trace as the traced path's form shows it**: the latches
/// it was read for, the evaluation it was read against, and what the
/// read said.
#[derive(Debug)]
pub(crate) struct TraceShown {
    plane: FaceSelection,
    face: FaceSelection,
    against: Weak<Evaluation<f64>>,
    /// The trace's points, or why it shows none.
    pub(crate) read: Result<TraceMarks, SketchTraceFault>,
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
    /// A profile draft is not a program's shape.
    Program(RecordedProgramError),
    /// A traced path does not lower.
    Traced(sketch::TracedLoopError),
}

impl From<SeatError> for CommitFault {
    fn from(error: SeatError) -> Self {
        Self::Seat(error)
    }
}

impl From<RecordedProgramError> for CommitFault {
    fn from(error: RecordedProgramError) -> Self {
        Self::Program(error)
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
            Self::Program(error) => error.fmt(f),
            Self::Traced(error) => error.fmt(f),
        }
    }
}

/// **A form's face latch takes `face`**: held as a pick is
/// ([`HeldPick::new`]), its nodes as `doc` speaks them now. The face it already holds is kept as held, so its labels
/// stay the ones it kept while the selection stays on it (a later edit
/// may have taken a node `doc` no longer speaks).
pub(crate) fn latch(slot: &mut Option<HeldPick>, doc: &Doc<ProfileProgram>, face: &FaceSelection) {
    if slot.as_ref().is_some_and(|held| held.face() == face) {
        return;
    }
    *slot = Some(HeldPick::new(doc, face.clone()));
}

/// **A form's latch row, said**: the held pick by `by` ([`HeldPick`]), a
/// node `by` no longer holds as the latch kept it; the empty pick's one
/// spelling ([`crate::seats::NONE_PICKED`]) while nothing is held.
pub(crate) fn latch_said(held: Option<&HeldPick>, by: Speaker<'_>) -> String {
    held.map_or_else(
        || crate::seats::NONE_PICKED.to_owned(),
        |held| pncad::document::Said(held, by).to_string(),
    )
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::document::{
        CancelToken, Doc, DocEdit, EvalOptions, Formula, Node, ProfileProgram, RecipeNodeId,
        evaluate,
    };
    use pncad::geom_core::{Point2, Tol};
    use pncad::profile::{ArcData, Step, Target};

    use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};

    use super::{Drafts, ProfileEdit, RowEdit};
    use crate::forms::ShapeKind;
    use crate::props::Notation;
    use crate::session::FaceSelection;
    use crate::session::SessionOp;
    use crate::sketch;
    use crate::test_support::{inserted, try_edited, try_inserted};

    /// A rename draft survives only while its own node's field shows.
    #[test]
    fn a_rename_draft_is_dropped_when_the_pane_moves_off_its_node() {
        let (typed_for, other) = (RecipeNodeId::new(0, 3), RecipeNodeId::new(0, 4));
        let mut drafts = Drafts {
            label_text: Some((typed_for, "lid".to_owned())),
            ..Drafts::default()
        };
        drafts.rename_shown_for(Some(typed_for));
        assert!(drafts.label_text.is_some(), "kept while its node shows");
        drafts.rename_shown_for(Some(other));
        assert_eq!(drafts.label_text, None, "dropped on another node");
        drafts.label_text = Some((typed_for, "lid".to_owned()));
        drafts.rename_shown_for(None);
        assert_eq!(drafts.label_text, None, "dropped with no node shown");
    }

    /// A labelled creation settles its form as the bare creation does,
    /// and spends the label draft of the kind it minted — only once it
    /// has landed, and only that kind's.
    #[test]
    fn a_landed_creation_spends_its_kinds_label_draft_and_settles_its_form() {
        let tol = Tol::witness();
        let doc: Doc<ProfileProgram> = Doc::empty_derived("drafts-creation-landed", tol);
        let mut drafts = Drafts {
            profile_shape: Some(ShapeKind::Circle),
            ..Drafts::default()
        };
        drafts
            .creation_labels
            .insert("Profile", "sketch".to_owned());
        drafts.creation_labels.insert("Extrude", "plate".to_owned());
        let loops = drafts
            .profile_programs(Notation::DEFAULT)
            .expect("the default circle lowers");
        let (doc, profile) = inserted(
            &doc,
            Node::Profile(ProfileProgram {
                loops: loops.clone(),
                ids: Vec::new(),
                on: None,
            }),
            tol,
        );
        let creation = crate::session::Creation::of(SessionOp::AddProfile { loops })
            .expect("adding a profile creates a node");
        drafts.accepted(&SessionOp::CreateLabelled {
            creation,
            label: pncad::document::Label::new("sketch").expect("a label"),
        });
        assert_eq!(
            drafts.profile_shape, None,
            "the labelled add settles the form as the bare one does"
        );
        drafts.creation_landed(&doc, &[profile]);
        assert_eq!(
            drafts.creation_labels.keys().copied().collect::<Vec<_>>(),
            vec!["Extrude"],
            "the profile's draft is spent, the extrude's kept"
        );
    }

    /// Blank clears; a label is kept as typed; a text the rule refuses
    /// is refused.
    #[test]
    fn a_label_field_reads_blank_as_clear_and_holds_the_rest_to_the_rule() {
        assert_eq!(super::label_typed(" \t "), Ok(None));
        assert_eq!(
            super::label_typed("\u{200b}"),
            Ok(None),
            "a field of a zero-width space shows nothing, so it clears"
        );
        assert_eq!(
            super::label_typed(" lid ").map(|label| label.map(|l| l.as_str().to_owned())),
            Ok(Some(" lid ".to_owned()))
        );
        assert!(matches!(
            super::label_typed("a\u{7}b"),
            Err(pncad::document::LabelFault::Control { at: 1, .. })
        ));
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
        let doc = Doc::empty_derived("drafts-accepted", tol);
        let mut drafts = Drafts {
            profile_shape: Some(ShapeKind::Circle),
            ..Drafts::default()
        };

        // The control: an accepted op that adds no profile leaves the
        // form composing.
        drafts.accepted(&SessionOp::Hover(None));
        assert!(!drafts.profile_loops().is_empty(), "the draft was dropped");

        let loops = drafts
            .profile_programs(Notation::DEFAULT)
            .expect("the default circle lowers");
        let (doc, _) = inserted(
            &doc,
            Node::Profile(ProfileProgram {
                loops: loops.clone(),
                ids: Vec::new(),
                on: None,
            }),
            tol,
        );
        drafts.accepted(&SessionOp::AddProfile { loops });
        let evaluation = evaluate(
            &doc,
            None,
            &CancelToken::default(),
            &EvalOptions::default(),
            tol,
        );
        let placement = sketch::profile_plane();
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

    /// **A sketch on a face lowers the held pick, the depth and the
    /// spin in the form's notation**, and waits for a face while none is
    /// held.
    ///
    /// The face pick's node-valued answers are DIFFERENT ids here,
    /// deliberately: the ray met node 2 and the name was minted by node
    /// 1, so a lowering that carried the feature instead of the face as
    /// picked is red. The spin is not zero: a lowering that dropped it,
    /// or wrote it in canonical radians while the form said degrees, is
    /// red too.
    #[test]
    fn a_sketch_on_a_face_lowers_its_pick_its_depth_and_its_spin() {
        let face = FaceSelection {
            name: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, 1),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            },
            node: RecipeNodeId::new(0, 2),
            body: 0,
        };
        let notation = Notation {
            angle: pncad::quantity::DEG,
            ..Notation::DEFAULT
        };
        let waiting = Drafts {
            profile_shape: Some(ShapeKind::Circle),
            sketch_on_face: true,
            ..Drafts::default()
        };
        assert!(
            matches!(waiting.profile_op(notation), Ok(None)),
            "no face held: the form waits"
        );
        let doc = Doc::<ProfileProgram>::empty_derived("sketch-face", Tol::witness());
        let drafts = Drafts {
            sketch_face: Some(crate::tree::HeldPick::new(&doc, face.clone())),
            sketch_spin: core::f64::consts::FRAC_PI_2,
            ..waiting
        };
        let Ok(Some(SessionOp::SketchOnFace {
            face: held,
            distance,
            spin,
            ..
        })) = drafts.profile_op(notation)
        else {
            panic!("a held face lowers");
        };
        assert_eq!(held, face, "the face as picked");
        assert_eq!(distance.literal_value(), Some(drafts.sketch_depth));
        let want = Formula::written_angle(pncad::quantity::WrittenAngle::canonical_in(
            core::f64::consts::FRAC_PI_2,
            pncad::quantity::DEG,
        ))
        .expect("a finite angle");
        assert!(
            spin.bit_eq(&want),
            "the spin is written in degrees: {spin:?}"
        );
        let off = Drafts {
            sketch_on_face: false,
            ..drafts
        };
        assert!(
            matches!(
                off.profile_op(notation),
                Ok(Some(SessionOp::AddProfile { .. }))
            ),
            "off the face, the profile on its own plane"
        );
    }

    /// A document holding the profile the add-profile form's DEFAULT
    /// path authors, lowered by the form's own
    /// `profile_programs` and inserted through the edit door the
    /// create button's op reaches. Answers the document, the drafts
    /// that authored it, and the profile node.
    fn authored_by_the_form() -> (Doc<ProfileProgram>, Drafts, RecipeNodeId) {
        use crate::forms::ShapeKind;

        let tol = Tol::witness();
        let doc = Doc::empty_derived("drafts-edit", tol);
        let drafts = Drafts {
            profile_shape: Some(ShapeKind::Path),
            ..Drafts::default()
        };
        let loops = drafts
            .profile_programs(Notation::DEFAULT)
            .expect("the default path lowers");
        let node = Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        });
        let (doc, profile) =
            try_inserted(&doc, node, tol).expect("the form's default path is a profile");
        (doc, drafts, profile)
    }

    /// `doc` with the one `SetProgram` the edit door would make for
    /// `edit`'s held program.
    fn applied(
        doc: &Doc<ProfileProgram>,
        edit: &ProfileEdit,
        notation: Notation,
    ) -> Doc<ProfileProgram> {
        let edit = DocEdit::SetProgram {
            node: edit.node,
            loops: edit.programs(notation).expect("finite"),
            ids: edit.ids(),
            fresh: Vec::new(),
        };
        try_edited(doc, edit, Tol::witness())
            .expect("the edit door takes it")
            .0
    }

    /// **The create form's profile opens in the edit door untouched**:
    /// the draft holds the form's own steps, says nothing moved, and
    /// what its Apply would send writes no argument.
    #[test]
    fn the_forms_profile_opens_untouched_and_would_write_nothing() {
        let (doc, mut drafts, profile) = authored_by_the_form();
        let authored = drafts.profile_loops();
        let notation = Notation::DEFAULT;
        let edit = drafts
            .profile_edit(&doc, profile)
            .expect("the editor holds the form's profile");
        assert!(
            !edit.differs_from_base(),
            "a fresh load has nothing to apply"
        );
        assert!(sketch::authors_same_loops(&edit.shapes(), &authored));
        let Some(Node::Profile(current)) = doc.node(profile) else {
            panic!("a profile")
        };
        assert!(
            sketch::is_committed(
                &sketch::written_program(&doc, current),
                &edit.programs(notation).expect("finite"),
                &edit.ids()
            ),
            "the held program, every step its committed self, is the committed one"
        );
    }

    /// **The draft follows the document, not the other way round.** A
    /// moved number is held until applied; the applied program is the
    /// new base, so the draft is untouched again; going back to the
    /// earlier document (an undo) replaces the draft with the program
    /// held there; and a selection that leaves the node drops it.
    #[test]
    fn the_edit_draft_reloads_on_undo_and_is_abandoned_off_selection() {
        let (before, mut drafts, profile) = authored_by_the_form();
        let notation = Notation::DEFAULT;
        let moved_to = Step::LineTo(Target::Point(Point2::new(0.02, 0.0)));
        let same_step = |a: Step<f64>, b: Step<f64>| {
            sketch::authors_same_loops(
                &sketch::path_shapes(&[vec![Step::At(Point2::origin()), a]]),
                &sketch::path_shapes(&[vec![Step::At(Point2::origin()), b]]),
            )
        };
        let edit = drafts.profile_edit(&before, profile).expect("held");
        edit.steps_mut(0)[1] = moved_to;
        assert!(edit.differs_from_base());
        // Held across frames while the document stands still.
        let edit = drafts.profile_edit(&before, profile).expect("held");
        assert!(
            edit.differs_from_base(),
            "the typed number survived a second read"
        );
        let after = applied(&before, edit, notation);
        let edit = drafts.profile_edit(&after, profile).expect("held");
        assert!(
            !edit.differs_from_base(),
            "the applied program is the new base"
        );
        assert!(same_step(edit.loops()[0][1], moved_to));
        // Undo: the document the history steps back to.
        let edit = drafts.profile_edit(&before, profile).expect("held");
        assert!(!edit.differs_from_base());
        assert!(
            !same_step(edit.loops()[0][1], moved_to),
            "the undone number is gone from the draft"
        );
        // Revert puts typed numbers back.
        edit.steps_mut(0)[2] = Step::LineTo(Target::Point(Point2::new(0.03, 0.03)));
        assert!(edit.differs_from_base());
        edit.revert(&before).expect("revertible");
        assert!(!edit.differs_from_base());
        drafts.abandon_profile_edit_off(Some(profile));
        assert!(drafts.profile_edit.is_some(), "still selected, still held");
        drafts.abandon_profile_edit_off(None);
        assert!(drafts.profile_edit.is_none(), "selection left, draft gone");
        // A node the editor cannot hold leaves nothing stale behind.
        assert!(
            drafts
                .profile_edit(&before, RecipeNodeId::new(0, 0))
                .is_err()
        );
        assert!(drafts.profile_edit.is_none());
    }

    /// A draft of a one-loop profile of `steps`, committed on a fresh
    /// document; `(doc, drafts, profile)`.
    fn held_path(steps: Vec<Step<f64>>) -> (Doc<ProfileProgram>, Drafts, RecipeNodeId) {
        let tol = Tol::witness();
        let doc = Doc::empty_derived("drafts-ids", tol);
        let loops = vec![
            sketch::loop_program(
                &crate::session::ProfileShape::Path { steps },
                Notation::CANONICAL,
            )
            .expect("finite"),
        ];
        let node = Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        });
        let (doc, profile) = try_inserted(&doc, node, tol).expect("a profile");
        (doc, Drafts::default(), profile)
    }

    /// **A held step is its committed step through every row edit
    /// around it**: an insert is new, a removal is dropped, a reorder
    /// moves the id with its row, and a moved number or a changed
    /// target form is still the step. A verb re-picked in the row's
    /// combo is a fresh step, and so is a verb changed any other way.
    #[test]
    fn a_held_step_keeps_its_id_through_row_edits_and_loses_it_with_its_verb() {
        let (doc, mut drafts, profile) = authored_by_the_form();
        let edit = drafts.profile_edit(&doc, profile).expect("held");
        let &[a, b, c, d, e] = edit.base().ids[0].as_slice() else {
            panic!("the form's default chain is five steps")
        };
        let point = |x, y| Step::LineTo(Target::Point(Point2::new(x, y)));
        edit.edit_row(
            0,
            RowEdit::Insert {
                at: 2,
                step: point(0.02, 0.005),
            },
        );
        assert_eq!(
            edit.ids(),
            vec![vec![Some(a), Some(b), None, Some(c), Some(d), Some(e)]],
            "an inserted row is new"
        );
        edit.edit_row(0, RowEdit::Swap(3, 4));
        assert_eq!(
            edit.ids(),
            vec![vec![Some(a), Some(b), None, Some(d), Some(c), Some(e)]],
            "a reordered row carries its id"
        );
        edit.edit_row(0, RowEdit::Remove(1));
        assert_eq!(
            edit.ids(),
            vec![vec![Some(a), None, Some(d), Some(c), Some(e)]],
            "a removed row's id is dropped"
        );
        edit.steps_mut(0)[2] = point(0.0, 0.02);
        edit.steps_mut(0)[4] = Step::LineTo(Target::StartArriving);
        assert_eq!(
            edit.ids(),
            vec![vec![Some(a), None, Some(d), Some(c), Some(e)]],
            "a moved number and a changed target form are the same step"
        );
        edit.edit_row(
            0,
            RowEdit::Replace {
                index: 3,
                step: point(0.01, 0.01),
            },
        );
        edit.steps_mut(0)[2] = Step::ContinueTo(Target::Point(Point2::new(0.0, 0.02)));
        assert_eq!(
            edit.ids(),
            vec![vec![Some(a), None, None, None, Some(e)]],
            "a re-picked verb is a fresh step, even back to the same verb, and a changed verb is \
             one however it changed"
        );
        edit.revert(&doc).expect("revertible");
        assert_eq!(
            edit.ids(),
            vec![vec![Some(a), Some(b), Some(c), Some(d), Some(e)]],
            "a revert loads every step as itself again"
        );
        assert!(!edit.differs_from_base());
        let same = edit.loops()[0][1];
        edit.edit_row(
            0,
            RowEdit::Replace {
                index: 1,
                step: same,
            },
        );
        assert!(
            edit.differs_from_base(),
            "a step re-made with the numbers it had is still a new step, which Apply writes"
        );
    }

    /// **An arc's mode is the step; a split circle's count is not.** An
    /// arc drawn another way still draws its one leg. A circle split
    /// into another number of pieces draws other pieces — its
    /// `Piece(k)` is another arc — so it keeps nothing, while its
    /// radius and phase are numbers like any other.
    #[test]
    fn an_arc_mode_keeps_the_step_and_a_split_count_does_not() {
        let (doc, mut drafts, profile) = held_path(vec![
            Step::At(Point2::new(-0.01, 0.0)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(Point2::new(0.01, 0.0)),
                b: 1.0,
            }),
            Step::LineTo(Target::Start),
        ]);
        let edit = drafts.profile_edit(&doc, profile).expect("held");
        let kept = edit.base().kept_in_place();
        edit.steps_mut(0)[1] = Step::ArcTo(ArcData::Via {
            q: Point2::new(0.0, 0.01),
            target: Target::Point(Point2::new(0.01, 0.0)),
        });
        assert_eq!(edit.ids(), kept, "an arc mode moved");

        let split = |radius, n, phase| Step::CircleSplit {
            centre: Point2::origin(),
            radius,
            n,
            phase,
        };
        let (doc, mut drafts, profile) = held_path(vec![split(0.01, 3, 0.0)]);
        let edit = drafts.profile_edit(&doc, profile).expect("held");
        let kept = edit.base().kept_in_place();
        edit.steps_mut(0)[0] = split(0.02, 3, 0.5);
        assert_eq!(edit.ids(), kept, "a split circle's radius and phase moved");
        edit.steps_mut(0)[0] = split(0.02, 4, 0.5);
        assert_eq!(edit.ids(), vec![vec![None]], "its count moved");
    }

    /// **The profile being edited shows only its live preview.** The
    /// edit door's preview stands in for the node, which the
    /// committed-profile pass then leaves out; with no preview taken,
    /// nothing stands in and the committed drawing stays. A refused
    /// preview's rows are the viewport's
    /// (`a_refused_edit_hides_the_committed_profile_with_or_without_a_prefix`),
    /// where the lanes it paints can be read.
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
        let placement = sketch::profile_plane();
        let preview = sketch::preview(placement, &held.loops, tol, chord);
        assert!(
            matches!(&preview, Ok(p) if !p.loops.is_empty()),
            "the live preview draws"
        );
        let except = drafts.edited_in_place(Some(&preview));
        assert_eq!(except, Some(profile));
        assert!(drawn(except).is_empty(), "the committed loops are left out");
        assert_eq!(drafts.edited_in_place(None), None, "no preview taken");
    }
}

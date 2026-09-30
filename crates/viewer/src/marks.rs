//! What a frame MARKS, over an index someone else built.
//!
//! # The marks
//!
//! Each is a pure function of a built
//! [`crate::pickindex::PickIndex`] and what is selected or held, and
//! each answers *what should be lit* — a different question from
//! *what is under the cursor*, which is [`crate::pickindex`]'s and
//! stays there. Nothing here decides anything about picking, holds
//! state, or builds an index.
//!
//! - [`highlight`] — the patch ids a selection and a hover light, as
//!   a pure function of what is drawn and what is selected;
//! - [`edge_overlay`], with [`edge_segments`] and
//!   [`edge_id_segments`] under it — the same question for edges,
//!   which have no area to shade, so the answer is line geometry
//!   rather than a set of ids, and which therefore settles
//!   selected-over-hovered here rather than leaving it to the shader
//!   the way [`highlight`] does;
//! - [`compose`] — both of the above with the picks a form or tool
//!   HOLDS ([`Held`]) added: everything a frame marks about picks, as
//!   the one value the viewport draws them from;
//! - [`focus`] — **not a cursor question at all**: which drawn
//!   patches the side panel's selection is RESPONSIBLE for, which for
//!   a parameter means walking `doc.order()` for the nodes it drives.
//!   It reaches for an index because that is where the ids live, not
//!   because it is about a pick.
//!
//! **They read the index through its public doors only** —
//! `ids`, `name_of`, `ids_of_node`, `ids_of_target`, `edges_of_target`,
//! `edges_in`, `edge_name_of` and `edge_polyline_for` — so no layout inside `PickIndex` is
//! reachable from here and none of these answers can be tightened by
//! reaching past one. That property is what made the module separable,
//! and keeping it is what keeps the two files independent.
//!
//! # A mark is a value, recomputed, and never retained
//!
//! Every answer here is a plain value computed from state that lives
//! in exactly one place, so no widget and no renderer holds a
//! "currently highlighted" field to fall out of step with the
//! session. That is what lets a test assert which patches a selection
//! lights, and which segments, without a window or a GPU
//! (`tests/focus_highlight.rs`, `tests/edge_pick.rs`).
//!
//! **What a mark MEANS is here; what colour it comes out is
//! `crate::theme`'s.** The two enumerate DIFFERENT lists and neither
//! checks the other: `Theme::marks` answers the palette question for
//! four semantic marks — selected, hovered, probe, focus — and only
//! the last of those is also a door here. One door feeds several of
//! the theme's marks (an [`EdgeOverlay`] carries a selected lane, a
//! hovered lane, a held lane and a probe flag on each), so the two
//! lists correspond many-to-many and a reader should not expect to
//! line them up. The held mark's colour is `Theme::held`, which is not
//! one of the four.
//!
//! Module kind: **vocabulary** (`crates/viewer/README.md`, Module
//! boundaries). It names no driver type and no `app`-only crate.

use std::collections::BTreeSet;

use pncad::document::{Doc, ParamName, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Point3;
use pncad::prelude::{NameOrigin, StableName, attribute};

use crate::display::DisplayView;
use crate::narrowing::Narrow;
use crate::pickindex::{EdgeId, EdgeNamesRefused, IdMap, PickIndex};
use crate::session::{EdgeSelection, FaceSelection, Hovered, Selection};
use crate::vocab::vocabulary;

/// Which drawn patches the viewport should mark, and how.
///
/// **A pure function of (index, selection, hover)** — see
/// [`highlight`] — and of the held picks, through [`compose`]. Nothing
/// is retained: the value is recomputed each
/// frame from state that lives in exactly one place, which is the
/// discipline the panels established and the reason no widget here
/// holds a "currently highlighted" field.
///
/// Every field is [`IdMap::NOTHING`] when nothing is marked, so the
/// GPU consumes them as plain uniforms with no branch for absence.
///
/// **No field is narrowed against another.** One id may sit in two
/// fields — a hover on the selection, a held face that is also
/// selected — and which mark it wears is settled where the fragment
/// sees every lane at once, `crate::gpu`'s `fs_main`, in the order
/// [`Held`] states; so every producer of this value gets the same
/// ruling, and these fields are public, so this module is not the only
/// producer. [`EdgeOverlay`] carries the OPPOSITE convention;
/// [`edge_overlay`] states why.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Highlight {
    /// The selected patch's id, or [`IdMap::NOTHING`].
    pub selected: u32,
    /// The hovered patch's id, or [`IdMap::NOTHING`].
    pub hovered: u32,
    /// The held faces' patch ids, slot for slot with [`Held::faces`],
    /// each [`IdMap::NOTHING`] where its slot holds nothing drawn.
    pub held: [u32; HELD_FACES],
}

/// The highlight for a selection and a hover, against the index that
/// describes what is drawn.
///
/// **Scoped to the selection's own (node, body)**, not merely to its
/// name. A name can be drawn twice — two `Transform` roots over one
/// extrude carry the same names on both copies — and marking "the
/// first id of the name" then lights the OTHER placement, which is the
/// deliverable failing at exactly the shape it is hardest to notice.
/// [`PickIndex::ids_of_target`] does the narrowing, and it narrows to
/// at most one id because a node's name table is a bijection.
///
/// A selection whose name is not drawn in this index — the vanished
/// case — yields [`IdMap::NOTHING`], which is how "nothing lights up"
/// falls out of the resolution-failure semantics rather than being a
/// second implementation of them. So does a selection whose name IS
/// drawn but not on the body it was picked from, which is the same
/// statement said about a stale index.
pub fn highlight(index: &PickIndex, selection: &Selection, hover: Option<&Hovered>) -> Highlight {
    let mark = |face: &FaceSelection| face_id(index, face);
    Highlight {
        selected: selection.face().map_or(IdMap::NOTHING, mark),
        hovered: hover.and_then(Hovered::face).map_or(IdMap::NOTHING, mark),
        // Nothing a SELECTION implies: a held pick is held by a form
        // or a tool, so [`compose`] adds it from [`Held`].
        held: [IdMap::NOTHING; HELD_FACES],
    }
}

/// **One face pick's patch id**: the id [`PickIndex::ids_of_target`]
/// narrows it to on its own (node, body), or [`IdMap::NOTHING`] — the
/// rule [`highlight`]'s doc states.
fn face_id(index: &PickIndex, face: &FaceSelection) -> u32 {
    index
        .ids_of_target(face)
        .first()
        .copied()
        .unwrap_or(IdMap::NOTHING)
}

/// **The patch a face pick is drawn as, if the picture draws it**:
/// its id on its own (node, body) ([`highlight`]'s narrowing), on a
/// root the display does not hide — the rule
/// [`PickIndex::edge_polyline_for`] applies to an edge.
///
/// The one answer to "is this pick in the picture": the held mark
/// lights what this answers, and the add-datum form refuses a held
/// face it answers `None` for (`crate::session::face_frame_seat_drawn`),
/// so the button never commits against a face the viewport is not
/// marking.
pub fn drawn_patch(index: &PickIndex, display: &DisplayView, face: &FaceSelection) -> Option<u32> {
    if display.hidden_roots.contains(&face.node) {
        return None;
    }
    Some(face_id(index, face)).filter(|&id| id != IdMap::NOTHING)
}

/// **How many held face picks a frame can mark at once** — the length
/// of [`Held::faces`] and of [`Highlight::held`].
///
/// A count of held faces, not a tuning: the driver that gathers
/// [`Held`] writes its slots as one array literal, so a slot added
/// there without this count does not compile, and `crate::gpu` refuses
/// at compile time a count its uniform lane cannot carry.
pub const HELD_FACES: usize = 3;

/// **The picks a form or a tool HOLDS**: the second source of marks
/// beside the live selection.
///
/// A held pick outlives the selection that made it. The add-datum
/// form latches a face so that an author can click the feature tree
/// without losing it, and a modal tool keeps its picks until it
/// commits. Nothing in the selection says so, so a picture that marked
/// the selection alone would show no sign of a pick the next click
/// commits against.
///
/// **What the held mark means: a choice a form or tool is holding,
/// which is not the live selection.** It wears the selection's colour
/// (`Theme::held`) and is told from the selection by shape — stripes
/// on a face, a hollow line on an edge (`crate::gpu`).
///
/// **Precedence:** selected over hovered over held.
/// A held pick that is also the selection or the hover wears that
/// mark; `crate::gpu`'s `fs_main` rules it for faces and
/// [`EdgeLane::DRAW_ORDER`] for edges.
///
/// A value built per frame from the holders' own state and never
/// kept. Which holders there are is the viewport's to gather, in one
/// place; each held face is narrowed by [`drawn_patch`], so one whose
/// body is not in the picture marks nothing rather than another copy
/// of its name.
#[derive(Clone, Copy, Debug)]
pub struct Held<'a> {
    /// The held face picks, a slot per held face, `None` where that
    /// slot holds nothing.
    pub faces: [Option<&'a FaceSelection>; HELD_FACES],
    /// A held edge SET, if a tool holds one.
    pub edges: Option<HeldEdges<'a>>,
}

/// **A held edge set, as the marks read it**: the drawn body the edges
/// are on and their names — what a tool holding a set hands over
/// (`crate::blend::BlendTool::held_edges`).
#[derive(Clone, Copy, Debug)]
pub struct HeldEdges<'a> {
    /// The node whose drawn body the edges were picked on.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
    /// The held edges' names.
    pub names: &'a BTreeSet<StableName>,
}

impl HeldEdges<'_> {
    /// **The held mark**: the drawn segments of the whole set, from one
    /// pass over the body's drawn edges testing membership per edge —
    /// and the index's refusal when some of those edges have no name.
    ///
    /// Not one [`edge_segments`] search per held name, which scans the
    /// body's whole edge run each time — `O(E²)` name comparisons every
    /// frame on a body with `E` edges. The narrowing is a single
    /// selection's: scoped to (node, body), empty for a body this index
    /// does not draw or the display hides, and silent about a held name
    /// with no drawn edge.
    ///
    /// **The refusal is not silent**: a drawn edge with no name cannot
    /// be told held or not, so the segments may be short by it, and
    /// [`EdgeOverlay::held_refused`] carries it to the chrome.
    pub fn mark(
        &self,
        index: &PickIndex,
        display: &DisplayView,
    ) -> (Vec<[f32; 3]>, Option<EdgeNamesRefused>) {
        let drawn = index.edge_names_in(self.node, self.body);
        let mut out = Vec::new();
        for (id, name) in drawn.named {
            if self.names.contains(name) {
                out.extend(edge_id_segments(index, display, id));
            }
        }
        (out, drawn.refused)
    }
}

/// **Everything a frame marks about picks**: the selection and the
/// hover ([`highlight`], [`edge_overlay`]) with every [`Held`] pick
/// added — the held faces' ids in [`Highlight::held`] through
/// [`drawn_patch`], and the held edges in [`EdgeOverlay::held`]
/// through [`HeldEdges::mark`], flagged off the set's one body for
/// [`EdgeOverlay::selected_probed`]'s reason.
pub fn compose(
    index: &PickIndex,
    display: &DisplayView,
    selection: &Selection,
    hover: Option<&Hovered>,
    held: &Held<'_>,
) -> (Highlight, EdgeOverlay) {
    let highlight = Highlight {
        held: held.faces.map(|face| {
            face.and_then(|face| drawn_patch(index, display, face))
                .unwrap_or(IdMap::NOTHING)
        }),
        ..highlight(index, selection, hover)
    };
    let (held_segments, held_refused) = held
        .edges
        .map(|set| set.mark(index, display))
        .unwrap_or_default();
    let edges = EdgeOverlay {
        held: held_segments,
        held_refused,
        held_probed: held.edges.is_some_and(|set| moved(display, set.node)),
        ..edge_overlay(index, display, selection, hover)
    };
    (highlight, edges)
}

/// Whether `node` is a free-moved root — the one read behind every
/// probe flag on an [`EdgeOverlay`].
fn moved(display: &DisplayView, node: RecipeNodeId) -> bool {
    display.moved_roots.contains_key(&node)
}

/// The edge marks a frame draws: the drawn polylines of the marked
/// edges, and the other world-space lanes drawn over the solid, as
/// line-list segment pairs in world space.
///
/// **A value, so the marking is checkable without pixels.** A test
/// asserts which segments a selection lights and where they are; what
/// colour they come out is the theme's answer and the shader's, and
/// neither is asserted here.
///
/// The buffers are `f32` because that is what a GPU consumes and this
/// is the display seam. **The narrowing itself is not spelled here**
/// — [`crate::narrowing::Narrow`] is the one door every lane crosses,
/// so this doc no longer holds sites in correspondence by naming
/// them, which is a job a sentence cannot keep: the one it replaced
/// said *the same cast that `SceneMesh` makes* and there were three
/// such casts, not two.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EdgeOverlay {
    /// The selected edge's segments, two positions per segment.
    pub selected: Vec<[f32; 3]>,
    /// The hovered edge's segments, two positions per segment —
    /// **empty when the hovered edge is the selected one**, which is
    /// the opposite of [`Highlight`]'s convention. [`edge_overlay`]
    /// states why it is decided at this end.
    pub hovered: Vec<[f32; 3]>,
    /// Whether the selected edge belongs to a free-moved instance.
    ///
    /// **The base a mark composites over is the same base the shaded
    /// pass uses**, and on a probed part that base already carries the
    /// probe tint — G3's honesty requirement, which a mark drawn over
    /// the un-probed body colour would quietly undo on exactly the
    /// geometry it is about.
    ///
    /// One flag per lane rather than per segment, which is enough
    /// while every lane's edges are on one body: the selection and the
    /// hover are one edge each, and a held set is on one body
    /// ([`EdgeOverlay::held_probed`]). A lane spanning several
    /// instances would want a flag per member.
    pub selected_probed: bool,
    /// Whether the hovered edge belongs to a free-moved instance. See
    /// [`EdgeOverlay::selected_probed`].
    pub hovered_probed: bool,
    /// **The edges a tool HOLDS** ([`Held::edges`]), two positions per
    /// segment: the held mark. Not narrowed against the selected or
    /// hovered edge — [`Held`] states which wins.
    pub held: Vec<[f32; 3]>,
    /// **What the index could not name on the held body**
    /// ([`HeldEdges::mark`]): [`EdgeOverlay::held`] may be short by
    /// those edges. `None` when every drawn edge there is named, and
    /// when nothing is held.
    pub held_refused: Option<EdgeNamesRefused>,
    /// Whether the held edges belong to a free-moved instance — one
    /// flag, because a held set is on one body
    /// ([`crate::blend::BlendTarget`]). See
    /// [`EdgeOverlay::selected_probed`].
    pub held_probed: bool,
    /// **Segments that are not in the document at all**: the
    /// wireframe of something a form is composing, in the same
    /// line-list shape as the marks above.
    ///
    /// It rides here rather than in a pass of its own because it is
    /// the same drawing — world-space segments over the solid, depth
    /// tested, writing no depth — and a second pass would be a second
    /// place for that to be got right. What separates it is the MARK:
    /// a preview is drawn in the theme's probe tint, the mark that
    /// means "this placement is not committed" (G3's honesty
    /// requirement), so a wireframe can never be mistaken for a
    /// selection of something that exists.
    pub preview: Vec<[f32; 3]>,
    /// **Construction geometry that is in the document but is not
    /// material**: the datum wireframes `crate::datums` draws, in the
    /// same line-list shape as everything above.
    ///
    /// It rides this overlay for [`EdgeOverlay::preview`]'s reason —
    /// one pass for every world-space segment drawn over the solid —
    /// and it is a separate LANE for the same reason that one is: the
    /// mark is what differs. A datum is drawn in `Theme::datum`, the
    /// colour that means "not material", so it can never be mistaken
    /// for a marked face of something that is.
    pub datums: Vec<[f32; 3]>,
    /// **Profiles the document holds**: the loops of every profile
    /// node the landed evaluation validated, drawn where they lie.
    ///
    /// A lane of its own rather than the preview's, because the two
    /// say different things — this one is in the document and the
    /// preview is not. **No loop is meant to be in both**, and two
    /// things keep it so. The create form's preview is of a profile
    /// that is not a node yet, and the form comes to rest when its add
    /// is accepted (`crate::drafts::Drafts::accepted`), so the node
    /// it became is drawn here and nowhere else. A form that previews
    /// an edit of a COMMITTED profile has to name that node to
    /// `crate::sketch::committed`'s `except`, which leaves it out of
    /// this lane; nothing checks that it does.
    pub profiles: Vec<[f32; 3]>,
}

vocabulary! {
    /// **One lane of an [`EdgeOverlay`]**, named so that where it is
    /// drawn relative to the others is a property of the lane and not of
    /// the order some caller happened to fill the fields in.
    ///
    /// The edge pass writes no depth, so where two lanes cover one pixel
    /// the one drawn LATER is what the pixel shows. The variants are
    /// therefore declared in draw order, lowest priority first, and
    /// [`EdgeLane::DRAW_ORDER`] — projected from this declaration — is
    /// the list the renderer walks and nothing else:
    ///
    /// - The datum grid is first: it is the backdrop the rest is placed
    ///   against, and a plane rules its whole seen region, so anything
    ///   it could cover it would cover everywhere.
    /// - A committed profile is over the grid, which is the plane it
    ///   usually lies in.
    /// - A preview is over the committed profiles: it is what the person
    ///   is composing now, possibly on top of one.
    /// - The marks are last, in [`Held`]'s precedence: a mark is the
    ///   answer to "which one is that", and it is worthless where
    ///   something else covers it.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum EdgeLane {
        /// [`EdgeOverlay::datums`].
        Datum,
        /// [`EdgeOverlay::profiles`].
        Profile,
        /// [`EdgeOverlay::preview`].
        Preview,
        /// [`EdgeOverlay::held`].
        Held,
        /// [`EdgeOverlay::hovered`].
        Hovered,
        /// [`EdgeOverlay::selected`].
        Selected,
    }

    /// **Every lane, lowest priority first** — the order the edge pass
    /// draws them in, so each is drawn over every lane before it.
    pub const DRAW_ORDER;
}

impl EdgeOverlay {
    /// Whether there is nothing to draw.
    pub fn is_empty(&self) -> bool {
        EdgeLane::DRAW_ORDER
            .iter()
            .all(|lane| self.lane(*lane).is_empty())
    }

    /// How many line segments this overlay draws.
    pub fn segments(&self) -> usize {
        EdgeLane::DRAW_ORDER
            .iter()
            .map(|lane| self.lane(*lane).len() / 2)
            .sum()
    }

    /// One lane's segments, two positions per segment.
    pub fn lane(&self, lane: EdgeLane) -> &[[f32; 3]] {
        match lane {
            EdgeLane::Datum => &self.datums,
            EdgeLane::Profile => &self.profiles,
            EdgeLane::Preview => &self.preview,
            EdgeLane::Held => &self.held,
            EdgeLane::Hovered => &self.hovered,
            EdgeLane::Selected => &self.selected,
        }
    }

    /// Whether `lane`'s edges belong to a free-moved instance — the
    /// three marks carry the flag, and nothing else in the overlay
    /// belongs to an instance at all.
    pub fn probed(&self, lane: EdgeLane) -> bool {
        match lane {
            EdgeLane::Held => self.held_probed,
            EdgeLane::Hovered => self.hovered_probed,
            EdgeLane::Selected => self.selected_probed,
            EdgeLane::Datum | EdgeLane::Profile | EdgeLane::Preview => false,
        }
    }
}

/// **The edge half of the highlight**, as a pure function of (index,
/// display view, selection, hover) — the twin of [`highlight`], which
/// answers the face half by patch id.
///
/// Edges are drawn rather than tinted, so they cannot ride the id
/// comparison the face marks use: a face mark is a patch the shader
/// recognises, and an edge mark is geometry that has to be handed to
/// the renderer. What the two share is the RULE — the mark is scoped
/// to the selection's own (node, body), so an edge whose name is drawn
/// twice lights the copy it was picked from and not the other one, and
/// a selection whose name is not drawn at all lights nothing. That is
/// the resolution-failure semantics falling out of the same narrowing
/// rather than being implemented a second time.
///
/// A hover on the edge that is already selected draws only the
/// selected mark: selection is the state the user committed to. The
/// OUTCOME is the one the shader's face path states, and the value
/// states it too: the edge pass draws its lanes in
/// [`EdgeLane::DRAW_ORDER`], selected last, so a hovered lane holding
/// the selected edge's own geometry would lose to the selected mark on
/// screen anyway — but it would still be a value naming one edge in two
/// lanes, and what a test reads of this overlay is the value. So the
/// selection is settled here, and the draw order is what keeps it
/// settled for a producer that did not narrow (a held edge is drawn in
/// its own lane, [`EdgeOverlay::held`], without asking either mark).
/// [`Highlight`] can leave its pair to the shader because a fragment
/// sees both of its lanes.
pub fn edge_overlay(
    index: &PickIndex,
    display: &DisplayView,
    selection: &Selection,
    hover: Option<&Hovered>,
) -> EdgeOverlay {
    let mark = |edge: &EdgeSelection| edge_segments(index, display, edge);
    let selected_edge = selection.edge();
    let hovered_edge = hover.and_then(Hovered::edge).filter(|edge| {
        // The one already marked as selected is not marked twice.
        selected_edge != Some(*edge)
    });
    let probed = |edge: &EdgeSelection| moved(display, edge.node);
    EdgeOverlay {
        selected: selected_edge.map(mark).unwrap_or_default(),
        hovered: hovered_edge.map(mark).unwrap_or_default(),
        selected_probed: selected_edge.is_some_and(probed),
        hovered_probed: hovered_edge.is_some_and(probed),
        // Nothing a SELECTION implies: a held pick is added by
        // [`compose`], from whoever holds it; a preview is about
        // something that is not in the document, so it is added by
        // whoever is composing it. Datums are not derived from a pick
        // either — they are simply what the document holds — so the
        // same line covers them.
        held: Vec::new(),
        held_refused: None,
        held_probed: false,
        preview: Vec::new(),
        datums: Vec::new(),
        profiles: Vec::new(),
    }
}

/// **One edge selection's drawn segments**, as the line-list pairs a
/// renderer consumes — the conversion [`edge_overlay`] is built from.
///
/// Scoped to the selection's own (node, body), empty for a name this
/// index does not draw there. It SEARCHES the target's edge run for
/// the name, so a SET is marked through [`HeldEdges::mark`], which
/// walks the run once.
pub fn edge_segments(
    index: &PickIndex,
    display: &DisplayView,
    edge: &EdgeSelection,
) -> Vec<[f32; 3]> {
    index
        .edges_of_target(edge)
        .first()
        .map(|id| edge_id_segments(index, display, *id))
        .unwrap_or_default()
}

/// **One DRAWN edge's segments**, addressed by the id this index
/// assigned it — [`edge_segments`] without the name search, for a
/// caller already walking the index's own edge run.
///
/// Empty for an id this index did not assign and for an edge whose
/// part is hidden: [`PickIndex::edge_polyline_for`]'s rule unaltered,
/// so the two doors cannot disagree about what is in the picture.
pub fn edge_id_segments(index: &PickIndex, display: &DisplayView, id: EdgeId) -> Vec<[f32; 3]> {
    edge_id_lane(index, display, id).into_segments()
}

/// [`edge_id_segments`] with the display seam's own answer kept: the
/// same legs, and how many this edge lost to the narrowing.
///
/// The door [`edge_id_segments`] is written in terms of, public
/// because the drop is otherwise unobservable — an edge every leg of
/// which is past `f32::MAX` and an edge this index never drew both
/// answer the empty `Vec`.
#[must_use]
pub fn edge_id_lane(index: &PickIndex, display: &DisplayView, id: EdgeId) -> LegLane {
    LegLane::of_polyline(&index.edge_polyline_for(id, display))
}

/// **An overlay lane at the display seam**: the legs a GPU can hold,
/// as the line-list pairs it draws, and a COUNT of the legs it cannot.
///
/// # The rule, in one place
///
/// **A leg with an end the display seam refuses is not drawn, it is
/// not half drawn, and the rest of the lane is.** An overlay lane is
/// a list of independent pairs, so a leg the GPU cannot hold is the
/// one thing at this seam with an answer short of refusing the whole
/// mark: what [`crate::narrowing::Narrow`] declines is a coordinate
/// past `f32::MAX`, where the leg's two ends are not places on the
/// screen in the first place, and the alternative is a pair of
/// infinities the rasterizer smears across the pane.
///
/// Both of the crate's leg producers reach the rule here — the drawn
/// edges of an indexed body ([`edge_id_segments`]) and the
/// sketch-plane lanes the viewport composes — because a rule spelled
/// at two sites is two rules that agree today.
///
/// # Why the count is part of the value
///
/// A lane that dropped a leg and a lane that had none to draw are the
/// same `Vec`, and they are not the same picture: a leg refused
/// between two legs that were drawn leaves an outline with a gap in
/// it, and a gap in an outline reads to a person as an authoring
/// mistake rather than as a number too large to show. [`undrawn`] is
/// the only thing that can tell them apart, so it is held beside the
/// segments rather than recomputed by whoever wants to say so.
///
/// [`undrawn`]: LegLane::undrawn
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LegLane {
    segments: Vec<[f32; 3]>,
    undrawn: usize,
}

impl LegLane {
    /// The lane a polyline draws as: one leg per adjacent pair.
    #[must_use]
    pub fn of_polyline(polyline: &[Point3<f64>]) -> Self {
        let mut lane = Self::default();
        lane.polyline(polyline);
        lane
    }

    /// Append one leg, or refuse it and count it.
    ///
    /// Generic over the shape an end is written in, because the
    /// crate's producers hold their positions differently — an
    /// indexed edge's polyline is [`Point3<f64>`] and a datum mark's
    /// segment list is `[f64; 3]` — and which of those a lane was fed
    /// is not a difference the seam's rule has ever had.
    pub fn leg<T>(&mut self, from: T, to: T)
    where
        T: Narrow<Narrowed = [f32; 3]> + Copy,
    {
        match [from, to].narrow() {
            Some(pair) => self.segments.extend(pair),
            None => self.undrawn += 1,
        }
    }

    /// Append every leg of a polyline, in order.
    pub fn polyline(&mut self, polyline: &[Point3<f64>]) {
        for pair in polyline.windows(2) {
            self.leg(pair[0], pair[1]);
        }
    }

    /// The drawn legs, two positions per leg.
    #[must_use]
    pub fn segments(&self) -> &[[f32; 3]] {
        &self.segments
    }

    /// The drawn legs, taken.
    #[must_use]
    pub fn into_segments(self) -> Vec<[f32; 3]> {
        self.segments
    }

    /// **How many legs the display seam refused** — the state a
    /// reader would need to be told that an outline is missing a leg
    /// rather than ending where it looks like it ends.
    #[must_use]
    pub fn undrawn(&self) -> usize {
        self.undrawn
    }
}

/// **What the picture marks because it is what the side panel is
/// showing.** The ids of every drawn patch the selection MADE.
///
/// Distinct from [`highlight`], and the distinction is the point.
/// `highlight` marks the ONE patch a pick landed on — an answer about
/// the cursor. This marks the whole extent of the thing being EDITED,
/// which for a feature is every face it made, and for a document
/// parameter is every face of every feature that parameter drives.
/// Selecting an extrude in the feature tree lights its walls; clicking
/// one of those walls lights the same set, with the picked patch
/// additionally tinted by `highlight` — and that holds however many
/// features later carried the wall, because a click resolves to the
/// feature that MADE the face (`FaceSelection::feature`) rather than
/// to whichever root drew it.
///
/// **Made, not merely drawn under.** A patch belongs to the node that
/// MINTED the entity its name denotes, which
/// [`pncad::select::attribute`] reads off the name's own
/// carry-through segments: a fillet's `FromTarget(f)` face is still
/// the target's face `f`, so a fillet's extent is the blends and
/// corners it created and nothing else. Which node DRAWS a patch is a
/// different question with a different answer — on a body whose whole
/// history ends in one outer feature, that feature draws every face
/// and made almost none of them.
///
/// **A node that made nothing drawn still focuses something**, in two
/// steps. A `Transform`, or a tool body a boolean consumed, mints no
/// drawn entity but drawn entities pass THROUGH it, and those are its
/// extent — the geometry built on top of it. Passing through is read
/// off the name where the op re-named what it carried, and off the
/// recipe where it did not: a `Transform` contributes no role segment
/// by construction, so what it carries is what was minted below it
/// ([`crate::display::derives_from`]). Failing that, a node no
/// drawn name mentions at all — a profile, a datum plane, a sketch —
/// marks the drawn roots deriving from it
/// ([`crate::display::roots_deriving_from`]): a profile's line and the wall it
/// swept are one thing seen twice. That last step is also where a name
/// the vocabulary walk cannot classify degrades to, so an
/// unclassified role costs the whole-body picture rather than an empty
/// one.
///
/// **What it does NOT do yet (issue 1182)**, stated so the gap is not
/// mistaken for a decision: the marking is per NODE, so selecting a
/// profile lights the whole body built from it rather than the walls of
/// the one segment being edited. Per-segment marking is expressible in
/// this type — the answer is a set of patch ids and nothing about the
/// shape assumes a whole node's worth — and wants the profile-step ↔
/// `RoleSeg::Lateral(ProfileEdgeRef)` correspondence established rather
/// than guessed: a slot's `step` is an index in the AUTHORING chain and
/// the name's `segment` an index in the LOWERED one, and one authored
/// step can lower to several segments. A wrong guess there lights a
/// confidently wrong face, silently.
///
/// A selection whose referent is not drawn — vanished, unevaluated,
/// hidden, or a feature that produces no body at all — answers the
/// empty set, which is how "nothing lights up" falls out of the same
/// rule rather than being a case.
pub fn focus(index: &PickIndex, doc: &Doc<ProfileProgram>, selection: &Selection) -> BTreeSet<u32> {
    let nodes: Vec<RecipeNodeId> = match selection {
        Selection::None => Vec::new(),
        Selection::Node(node) => vec![*node],
        // The feature the face IS, not the root that drew it — the
        // same inversion the tree and the panel read
        // (`FaceSelection::feature`), so a click and a tree selection
        // of one feature mark one set.
        Selection::Face(face) => vec![face.feature()],
        // The feature the EDGE is, by the same inversion: an edge is
        // the same kind of picked entity a face is, and selecting one
        // shows the same feature's rows.
        Selection::Edge(edge) => vec![edge.feature()],
        // Every node the parameter drives. A parameter is the one
        // selection with no geometry of its own, and the useful
        // question about it is exactly "what does this number move".
        Selection::Param(name) => doc
            .order()
            .iter()
            .copied()
            .filter(|&id| drives(doc, id, name))
            .collect(),
    };
    if nodes.is_empty() {
        return BTreeSet::new();
    }
    // One walk of the names per call, not one per selected node: a
    // parameter selection asks the same question of every node it
    // drives.
    let made: Vec<(u32, NameOrigin)> = index
        .ids()
        .ids()
        .filter_map(|id| Some((id, attribute(index.name_of(id)?.as_ref().ok()?))))
        .collect();
    let mut out = BTreeSet::new();
    for node in nodes {
        out.extend(marked_for(index, doc, &made, node));
    }
    out
}

/// The patches ONE node is responsible for: what it minted, else what
/// passes through it, else the roots built from it (see [`focus`]).
fn marked_for(
    index: &PickIndex,
    doc: &Doc<ProfileProgram>,
    made: &[(u32, NameOrigin)],
    node: RecipeNodeId,
) -> BTreeSet<u32> {
    let pick = |keep: &dyn Fn(&NameOrigin) -> bool| -> BTreeSet<u32> {
        made.iter()
            .filter(|(_, at)| keep(at))
            .map(|(id, _)| *id)
            .collect()
    };
    let minted = pick(&|at| at.minted_by() == Some(node));
    if !minted.is_empty() {
        return minted;
    }
    // Passing through, by the name and by the recipe. A name records
    // the ops that RE-NAMED the entity, so an op that contributes no
    // role segment — a `Transform` — is invisible to the walk; the
    // entities it carries are the ones minted anywhere below it.
    let below: BTreeSet<RecipeNodeId> = made
        .iter()
        .filter_map(|(_, at)| at.minted_by())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|&minter| crate::display::derives_from(doc, node, minter))
        .collect();
    let through =
        pick(&|at| at.passes_through(node) || at.minted_by().is_some_and(|m| below.contains(&m)));
    if !through.is_empty() {
        return through;
    }
    crate::display::roots_deriving_from(doc, node)
        .into_iter()
        .flat_map(|root| index.ids_of_node(root))
        .collect()
}

/// Whether any of `node`'s slot expressions reads the parameter
/// `name` — through `Expr::param_refs`, the public read side, so a
/// reference nested inside arithmetic counts exactly as a bare one
/// does.
fn drives(doc: &Doc<ProfileProgram>, node: RecipeNodeId, name: &ParamName) -> bool {
    let Some(recipe_node) = doc.node(node) else {
        return false;
    };
    recipe_node.slots().into_iter().any(|slot| {
        recipe_node.expr(slot).is_some_and(|expr| {
            let mut refs = Vec::new();
            expr.param_refs(&mut refs);
            refs.iter().any(|(referenced, _)| referenced == name)
        })
    })
}

/// **What the display seam does to a drawn mark**, through the doors a
/// frame actually calls.
///
/// The rule under test is [`LegLane`]'s: a leg with an end the seam
/// refuses is dropped, the rest of the lane is drawn, and the drop is
/// counted. It is asserted here rather than at the arithmetic because
/// the question is REACHABILITY — whether the legs a frame asks for
/// come back short — and a row against a private helper answers a
/// different question. The held mark's other way of coming back short,
/// a drawn edge the index cannot name, is asked the same way.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::BTreeMap;

    use pncad::document::{Frame, RecipeNodeId};
    use pncad::geom_core::Point3;

    use std::collections::BTreeSet;

    use pncad::prelude::StableName;

    use super::{HELD_FACES, Held, HeldEdges, LegLane, compose, edge_id_lane, edge_id_segments};
    use crate::display::DisplayView;
    use crate::pickindex::{EdgeId, EdgeNameFault, EdgeNamesRefused, PickIndex};
    use crate::session::Selection;

    /// The spike plate's index — the picture a frame marks in.
    fn plate() -> (PickIndex, RecipeNodeId) {
        let (_, index, extrude) =
            crate::test_support::plate_indexed(pncad::geom_core::Tol::witness());
        (index, extrude)
    }

    /// The view that puts `node`'s drawn geometry `shift` metres out.
    fn moved(node: RecipeNodeId, shift: f64) -> DisplayView {
        DisplayView {
            moved_roots: BTreeMap::from([(node, Frame::translation([shift, 0.0, 0.0]))]),
            ..DisplayView::none()
        }
    }

    fn some_edge(index: &PickIndex, node: RecipeNodeId) -> EdgeId {
        *index
            .edges_in(node, 0)
            .first()
            .expect("the plate draws edges")
    }

    /// **The public door comes back short, and only the seam can have
    /// shortened it.**
    ///
    /// The same index and the same edge id, asked twice: once where
    /// the body is drawn and once where a probe frame has put it past
    /// `f32::MAX`. The first answer is the edge's legs; the second is
    /// no legs at all, with every one of them counted as refused. A
    /// caller that kept a leg whose end does not narrow would answer
    /// the same list both times.
    #[test]
    fn an_edge_placed_past_the_display_seam_is_not_drawn_at_all() {
        let (index, extrude) = plate();
        let id = some_edge(&index, extrude);
        let here = edge_id_lane(&index, &DisplayView::none(), id);
        assert!(
            !here.segments().is_empty(),
            "the plate's first drawn edge has legs to lose"
        );
        assert_eq!(here.undrawn(), 0, "nothing about the plate is past 3.4e38");
        assert!(
            here.segments().iter().flatten().all(|c| c.is_finite()),
            "a drawn leg is made of numbers"
        );

        let far = moved(extrude, 1.0e300);
        let out_there = edge_id_lane(&index, &far, id);
        assert!(
            out_there.segments().is_empty(),
            "every leg of this edge has both ends past f32::MAX"
        );
        assert_eq!(
            out_there.undrawn(),
            here.segments().len() / 2,
            "the legs that were dropped are the legs that were drawn"
        );
        assert!(
            edge_id_segments(&index, &far, id).is_empty(),
            "the door the frame calls answers the same"
        );
    }

    /// **The case a person can author and can see**: an outline most
    /// of which is ordinary and one corner of which is past the seam.
    ///
    /// `7e307` is a number the add-profile form takes, because it is a
    /// number. The lane draws the legs between the ordinary corners
    /// and drops the two that reach the far one, so what is on screen
    /// is a chain with a gap in it at a camera framed on the ordinary
    /// corners — which is why the count is part of the value rather
    /// than the emptiness of the `Vec` being the whole answer.
    #[test]
    fn a_lane_draws_the_legs_it_can_and_counts_the_ones_it_cannot() {
        let corner = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let authored = [
            corner(0.0, 0.0),
            corner(1.0, 0.0),
            corner(7.0e307, 0.0),
            corner(0.0, 1.0),
            corner(0.0, 0.0),
        ];
        let lane = LegLane::of_polyline(&authored);
        assert_eq!(
            lane.segments(),
            &[
                [0.0_f32, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0]
            ],
            "the two legs clear of the seam are drawn, in order, and nothing else is"
        );
        assert_eq!(lane.undrawn(), 2, "the two legs reaching the far corner");
        assert!(
            lane.segments().iter().flatten().all(|c| c.is_finite()),
            "no infinity reaches the vertex buffer"
        );
    }

    /// **A lane that lost every leg is not a lane that had none**, and
    /// the segments alone cannot tell them apart.
    #[test]
    fn an_empty_lane_and_a_refused_one_differ_only_in_the_count() {
        let far = Point3::new(7.0e307, 0.0, 0.0);
        let refused = LegLane::of_polyline(&[far, far]);
        let nothing = LegLane::of_polyline(&[Point3::new(0.0, 0.0, 0.0)]);
        assert_eq!(refused.segments(), nothing.segments());
        assert_eq!(nothing.undrawn(), 0);
        assert_eq!(refused.undrawn(), 1);
        assert_ne!(refused, nothing);
    }

    /// A single leg is offered and refused one at a time, which is the
    /// door the viewport's sketch lanes use.
    #[test]
    fn a_single_leg_is_refused_on_its_own() {
        let mut lane = LegLane::default();
        lane.leg(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 2.0, 3.0));
        lane.leg(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 7.0e307));
        assert_eq!(
            lane.clone().into_segments(),
            vec![[0.0_f32, 0.0, 0.0], [1.0, 2.0, 3.0]]
        );
        assert_eq!(lane.undrawn(), 1);
    }

    /// **A held mark on a body the index cannot wholly name carries the
    /// index's refusal** to the composed value, and draws the held edges
    /// it can name. The naming layer's refusal is planted on a held
    /// edge (`PickIndex::unname_edge`). A held set on a body the index
    /// does not draw — the ordinary arm's case — marks nothing and
    /// refuses nothing.
    #[test]
    fn a_held_mark_the_index_cannot_wholly_name_carries_its_refusal() {
        let (_, mut index, extrude) =
            crate::test_support::plate_indexed(pncad::geom_core::Tol::witness());
        let drawn = index.edges_in(extrude, 0).to_vec();
        let names: BTreeSet<StableName> = drawn[..2]
            .iter()
            .map(|id| index.edge_name_of(*id).expect("named").clone())
            .collect();
        let held = |body| Held {
            faces: [None; HELD_FACES],
            edges: Some(HeldEdges {
                node: extrude,
                body,
                names: &names,
            }),
        };
        let display = DisplayView::none();
        let first = index.unname_edge(drawn[1]);

        let (_, marked) = compose(&index, &display, &Selection::None, None, &held(0));
        assert_eq!(
            marked.held,
            edge_id_segments(&index, &display, drawn[0]),
            "the held edge the index names is drawn, and only it"
        );
        assert_eq!(
            marked.held_refused,
            Some(EdgeNamesRefused {
                first: EdgeNameFault::Unnamed(first),
                refused: 1,
                drawn: drawn.len(),
            }),
            "the held edge it cannot name is not dropped silently"
        );

        let (_, elsewhere) = compose(&index, &display, &Selection::None, None, &held(7));
        assert!(elsewhere.held.is_empty());
        assert_eq!(elsewhere.held_refused, None, "the ordinary arm stays quiet");
    }
}

//! What a frame MARKS, over an index someone else built.
//!
//! Every door here takes a [`crate::pickindex::PickIndex`] as an
//! ARGUMENT and answers *what should be lit*. That is a different
//! question from *what is under the cursor*, which is
//! [`crate::pickindex`]'s and stays there: this module decides nothing
//! about picking, holds no state, and builds no index.
//!
//! **It reads the index through its public doors only** —
//! `ids`, `name_of`, `ids_of_node`, `ids_of_target`, `edges_of_target`
//! and `edge_polyline_for` — so no layout inside `PickIndex` is
//! reachable from here and none of these answers can be tightened by
//! reaching past one. That property is what made the module
//! separable, and keeping it is what keeps the two files independent.
//!
//! # The four marks
//!
//! - [`highlight`] — the patch ids a selection and a hover light, as
//!   a pure function of what is drawn and what is selected;
//! - [`edge_overlay`], with [`edge_segments`] and
//!   [`edge_id_segments`] under it — the same question for edges,
//!   which have no area to shade, so the answer is line geometry
//!   rather than a set of ids;
//! - [`focus`] — **not a cursor question at all**: which drawn
//!   patches the side panel's selection is RESPONSIBLE for, which for
//!   a parameter means walking `doc.order()` for the nodes it drives.
//!   It reaches for an index because that is where the ids live, not
//!   because it is about a pick;
//! - [`cursor_projection`] — the id pass's 1×1 target matrix, kept
//!   out of the render module because it is the one part of that pass
//!   a machine with no GPU can check.
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
//! `crate::theme`'s**, and the split is deliberate — `Theme::marks`
//! names the same four (selected, hovered, probe, focus) and answers
//! only the palette question. Neither file can change the other's
//! answer.
//!
//! Module kind: **vocabulary** (`crates/viewer/README.md`, Module
//! boundaries). It names no driver type and no `app`-only crate.

use std::collections::BTreeSet;

use pncad::document::{Doc, ParamName, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Point3;
use pncad::prelude::{NameOrigin, attribute};

use crate::display::DisplayView;
use crate::pickindex::{EdgeId, IdMap, PickIndex};
use crate::session::{EdgeSelection, FaceSelection, Hovered, Selection};

/// Which drawn patches the viewport should mark, and how.
///
/// **A pure function of (index, selection, hover)** — see
/// [`highlight`]. Nothing is retained: the value is recomputed each
/// frame from state that lives in exactly one place, which is the
/// discipline the panels established and the reason no widget here
/// holds a "currently highlighted" field.
///
/// Both fields are [`IdMap::NOTHING`] when nothing is marked, so the
/// GPU consumes them as plain uniforms with no branch for absence.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Highlight {
    /// The selected patch's id, or [`IdMap::NOTHING`].
    pub selected: u32,
    /// The hovered patch's id, or [`IdMap::NOTHING`].
    pub hovered: u32,
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
    let mark = |face: &FaceSelection| {
        index
            .ids_of_target(face)
            .first()
            .copied()
            .unwrap_or(IdMap::NOTHING)
    };
    Highlight {
        selected: selection.face().map_or(IdMap::NOTHING, mark),
        hovered: hover.and_then(Hovered::face).map_or(IdMap::NOTHING, mark),
    }
}

/// The edge marks a frame draws: the drawn polylines of the selected
/// and hovered edges, as line-list segment pairs in world space.
///
/// **A value, so the marking is checkable without pixels.** A test
/// asserts which segments a selection lights and where they are; what
/// colour they come out is the theme's answer and the shader's, and
/// neither is asserted here.
///
/// The buffers are `f32` because that is what a GPU consumes and this
/// is the display seam — the same cast, at the same boundary, that
/// [`crate::scene::SceneMesh`] makes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EdgeOverlay {
    /// The selected edge's segments, two positions per segment.
    pub selected: Vec<[f32; 3]>,
    /// The hovered edge's segments, two positions per segment.
    pub hovered: Vec<[f32; 3]>,
    /// Whether the selected edge belongs to a free-moved instance.
    ///
    /// **The base a mark composites over is the same base the shaded
    /// pass uses**, and on a probed part that base already carries the
    /// probe tint — G3's honesty requirement, which a mark drawn over
    /// the un-probed body colour would quietly undo on exactly the
    /// geometry it is about.
    ///
    /// One flag per marked edge rather than per segment, which is
    /// enough while the marks are single-select. A consumer holding a
    /// SET spanning several instances wants a flag per member and
    /// should build its overlay through [`edge_segments`], asking the
    /// display view per edge as this function does.
    pub selected_probed: bool,
    /// Whether the hovered edge belongs to a free-moved instance. See
    /// [`EdgeOverlay::selected_probed`].
    pub hovered_probed: bool,
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
}

impl EdgeOverlay {
    /// Whether there is nothing to draw.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
            && self.hovered.is_empty()
            && self.preview.is_empty()
            && self.datums.is_empty()
    }

    /// How many line segments this overlay draws.
    pub fn segments(&self) -> usize {
        (self.selected.len() + self.hovered.len() + self.preview.len() + self.datums.len()) / 2
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
/// selected mark: selection is the state the user committed to, which
/// is the precedence the shader's face path already states.
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
    let probed = |edge: &EdgeSelection| display.moved_roots.contains_key(&edge.node);
    EdgeOverlay {
        selected: selected_edge.map(mark).unwrap_or_default(),
        hovered: hovered_edge.map(mark).unwrap_or_default(),
        selected_probed: selected_edge.is_some_and(probed),
        hovered_probed: hovered_edge.is_some_and(probed),
        // Nothing a SELECTION implies: a preview is about something
        // that is not in the document, so it is added by whoever is
        // composing it, not derived from what is picked. Datums are
        // not derived from a pick either — they are simply what the
        // document holds — so the same line covers both.
        preview: Vec::new(),
        datums: Vec::new(),
    }
}

/// **One edge selection's drawn segments**, as the line-list pairs a
/// renderer consumes — the conversion [`edge_overlay`] is built from,
/// public because the next consumer holds a SET.
///
/// A blend tool accumulates edges in tool state and wants all of them
/// marked; [`EdgeOverlay`]'s fields are public, so it concatenates
/// these instead of copying the polyline-to-line-list step. The
/// narrowing is the same one a single selection gets — scoped to the
/// selection's own (node, body), empty for a name this index does not
/// draw there — so a set marks exactly the members that still denote
/// something.
///
/// **A SET is marked through [`edge_id_segments`] instead.** This door
/// SEARCHES the target's whole edge run for the name, so one call per
/// held name costs `O(E²)` name comparisons on a body with `E` edges,
/// every frame. A set-held tool walks the run once and tests
/// membership — `crate::blend::BlendTool::mark_segments`.
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
    segments_of(&index.edge_polyline_for(id, display))
}

/// A polyline as the line-list pairs a GPU draws.
fn segments_of(polyline: &[Point3<f64>]) -> Vec<[f32; 3]> {
    let corner = |point: &Point3<f64>| [point.x as f32, point.y as f32, point.z as f32];
    polyline
        .windows(2)
        .flat_map(|pair| [corner(&pair[0]), corner(&pair[1])])
        .collect()
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
/// (`display::derives_from`). Failing that, a node no
/// drawn name mentions at all — a profile, a datum plane, a sketch —
/// marks the drawn roots deriving from it
/// (`display::roots_deriving_from`): a profile's line and the wall it
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

/// The view-projection that puts ONE source pixel over the whole 1×1
/// target the GPU id pass renders into.
///
/// A pixel centred at `cursor_ndc` spans `2 / width` by `2 / height` of
/// normalized device space, so translating that point to the origin and
/// scaling by the viewport's pixel dimensions maps exactly that pixel
/// onto the target's `[−1, 1]²`. In a column-major clip-space matrix
/// the translation is a subtraction of `cursor · w`, which is why the
/// `w` row participates.
///
/// **It lives here, out of the render module, because it is the one
/// part of the id pass a machine with no GPU can check**: composed
/// with [`crate::camera::Camera::project`] it says that the world
/// point the ray path un-projects to is the point the id pass
/// rasterizes at the centre of its target. That composition is the
/// headless half of "both picking paths answer the same question".
pub fn cursor_projection(
    view_projection: &[[f32; 4]; 4],
    cursor_ndc: [f32; 2],
    viewport_px: [f32; 2],
) -> [[f32; 4]; 4] {
    let [cx, cy] = cursor_ndc;
    let [sx, sy] = viewport_px;
    let mut out = *view_projection;
    for column in &mut out {
        let w = column[3];
        column[0] = (column[0] - cx * w) * sx;
        column[1] = (column[1] - cy * w) * sy;
    }
    out
}

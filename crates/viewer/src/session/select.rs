//! **What is selected, and whether it still denotes anything**: the
//! session's selection and hover values, and the standing of what they
//! name.
//!
//! A VOCABULARY. These are values the panels and the viewport read and
//! the session moves; nothing here names the session, and every
//! question they answer is answered from the value alone.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! toolkit type (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{ParamName, RecipeNodeId};
use pncad::prelude::{StableName, attribute};
use pncad::select::Resolution;

/// A picked face: the stable name it is, and the node whose body
/// carried it when it was picked.
///
/// **The name is the selection**; the node rides along because it is
/// what the feature tree highlights and what the property panel shows
/// slots for, and re-deriving it would mean resolving the name again
/// for a question the pick already answered. G1's rule is satisfied
/// exactly: a `StableName` and a `RecipeNodeId`, no arena key.
///
/// The node is the one whose evaluated body was hit, which is not the
/// node that MADE the face: a face swept by an extrude, cut by a
/// boolean and carried through a fillet is hit on the fillet's body
/// and made by the extrude. Both are true and they answer different
/// questions — this field answers "whose body did the ray meet", and
/// [`FaceSelection::feature`] answers "which feature is this face's".
/// Every consumer that means the second must call it: on a model whose
/// history ends in one outer feature, this field is that feature for
/// every face of the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceSelection {
    /// The picked face's stable name — what survives re-evaluation.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
}

impl FaceSelection {
    /// **The feature this face is**: the node whose operation minted
    /// the entity the name denotes, read off the name's own
    /// carry-through segments (`pncad::select::attribute`).
    ///
    /// A fillet's `FromTarget(f)` face is still the target's face `f`,
    /// so clicking a flat on a filleted body reaches the feature that
    /// swept the flat and not the fillet that shrank it. That is the
    /// question the feature tree's highlight, the property panel's
    /// rows and the picture's focus all ask; [`FaceSelection::node`] —
    /// whose body the ray met — is a different one, and the only
    /// consumers that want it are the ones addressing the DRAWN body
    /// (`PickIndex::ids_of_target`, and the resolution check, which
    /// looks the name up in that body's own table).
    ///
    /// Falls back to [`FaceSelection::node`] for a name the vocabulary
    /// walk cannot classify, so an unclassified role degrades to the
    /// drawn root rather than to no feature at all.
    pub fn feature(&self) -> RecipeNodeId {
        attribute(&self.name).minted_by().unwrap_or(self.node)
    }
}

/// A picked edge: the stable name it is, and the node whose body
/// carried it when it was picked.
///
/// The face selection's twin, field for field, and deliberately a
/// DISTINCT type rather than a kind tag on one struct: the consumers
/// differ in what they accept — a blend selects edges, a mate selects
/// faces — so a value that could be either defers a refusal to run
/// time for no gain. The name is still the selection and no arena key
/// appears, which is all G1 asks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeSelection {
    /// The picked edge's stable name — what survives re-evaluation.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
}

impl EdgeSelection {
    /// **The feature this edge is**: the node whose operation minted
    /// the entity the name denotes — [`FaceSelection::feature`]'s
    /// argument, unchanged. An edge carried through a later boolean is
    /// still the edge the earlier feature made.
    pub fn feature(&self) -> RecipeNodeId {
        attribute(&self.name).minted_by().unwrap_or(self.node)
    }
}

/// What the cursor is over — the one transient pick, whichever kind of
/// entity it landed on.
///
/// One value rather than a field per kind, because the cursor is over
/// AT MOST ONE thing: two fields could both be set, and then the
/// picture and the status line would disagree about what the pointer
/// means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Hovered {
    /// A face under the cursor.
    Face(FaceSelection),
    /// An edge under the cursor — within
    /// [`crate::pick::EDGE_PICK_RADIUS_PX`] of it, which is what makes
    /// an edge reachable at all where its own face fills the pixel.
    Edge(EdgeSelection),
}

impl Hovered {
    /// The hovered entity's stable name.
    pub fn name(&self) -> &StableName {
        match self {
            Self::Face(face) => &face.name,
            Self::Edge(edge) => &edge.name,
        }
    }

    /// The node whose drawn body the cursor is over.
    pub fn node(&self) -> RecipeNodeId {
        match self {
            Self::Face(face) => face.node,
            Self::Edge(edge) => edge.node,
        }
    }

    /// The feature the hovered entity belongs to.
    pub fn feature(&self) -> RecipeNodeId {
        match self {
            Self::Face(face) => face.feature(),
            Self::Edge(edge) => edge.feature(),
        }
    }

    /// The hovered face, when the cursor is over one.
    pub fn face(&self) -> Option<&FaceSelection> {
        match self {
            Self::Face(face) => Some(face),
            Self::Edge(_) => None,
        }
    }

    /// The hovered edge, when the cursor is over one.
    pub fn edge(&self) -> Option<&EdgeSelection> {
        match self {
            Self::Edge(edge) => Some(edge),
            Self::Face(_) => None,
        }
    }

    /// This hover as the selection a click on it would make.
    pub fn selection(&self) -> Selection {
        match self {
            Self::Face(face) => Selection::Face(face.clone()),
            Self::Edge(edge) => Selection::Edge(edge.clone()),
        }
    }
}

/// What the session has selected. A typed layer-3 value: stable
/// names, recipe node ids and parameter names, never an arena key.
///
/// **Single-select, by ratification** (the GUI plan's rulings): one
/// selection, and nothing here is shaped to grow a second. Multi-select
/// is GQ7 and deferred by design.
///
/// **ONE value for the viewport and the panels.** A face picked in the
/// viewport and a node clicked in the tree write the same field, which
/// is what makes "click a face, watch its feature highlight" a
/// property of the state rather than of two widgets agreeing —
/// [`Selection::node`] is the one inversion both read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Selection {
    /// Nothing selected.
    #[default]
    None,
    /// A recipe node, selected in the feature tree.
    Node(RecipeNodeId),
    /// A document parameter, selected in the property panel — where
    /// the expression-driven refusal's affordance navigates to.
    Param(ParamName),
    /// A face, picked in the viewport.
    Face(FaceSelection),
    /// An edge, picked in the viewport — what a blend is authored
    /// against.
    Edge(EdgeSelection),
}

impl Selection {
    /// The recipe node this selection is about, when it is about one:
    /// the node itself, or the feature a picked face belongs to
    /// ([`FaceSelection::feature`] — the node that MADE the face, not
    /// the root that drew it).
    ///
    /// **The one home for the viewport→tree inversion.** The feature
    /// tree's highlight and the property panel's slot rows both read
    /// it, so a face pick reaches them without either of them knowing
    /// what a face is.
    pub fn node(&self) -> Option<RecipeNodeId> {
        match self {
            Self::Node(id) => Some(*id),
            Self::Face(face) => Some(face.feature()),
            Self::Edge(edge) => Some(edge.feature()),
            Self::None | Self::Param(_) => None,
        }
    }

    /// The picked face, when the selection is one.
    pub fn face(&self) -> Option<&FaceSelection> {
        match self {
            Self::Face(face) => Some(face),
            Self::None | Self::Node(_) | Self::Param(_) | Self::Edge(_) => None,
        }
    }

    /// The picked edge, when the selection is one.
    pub fn edge(&self) -> Option<&EdgeSelection> {
        match self {
            Self::Edge(edge) => Some(edge),
            Self::None | Self::Node(_) | Self::Param(_) | Self::Face(_) => None,
        }
    }

    /// The selected entity's stable name, when the selection is a
    /// picked entity — the one question the resolution check asks that
    /// does not care which kind was picked.
    pub fn entity_name(&self) -> Option<&StableName> {
        match self {
            Self::Face(face) => Some(&face.name),
            Self::Edge(edge) => Some(&edge.name),
            Self::None | Self::Node(_) | Self::Param(_) => None,
        }
    }
}

/// Whether the selection still denotes something in the evaluation on
/// screen — the ratified resolution-failure semantics, as a value.
///
/// **A vanished reference is a STATE, not an event.** Nothing clears
/// the selection when the thing it names stops existing: the name
/// stays, this verdict changes, and the chrome renders the unresolved
/// state distinctly while the affordances that need a live entity
/// switch off. That is the whole of GQ7's recorded constraint (tools
/// survive the referenced entity vanishing) at v1's single-select
/// scope.
#[derive(Clone, Debug, PartialEq)]
pub enum Standing {
    /// There is nothing selected to resolve.
    Empty,
    /// A node selection, and whether the document still holds it.
    Node {
        /// The node.
        node: RecipeNodeId,
        /// Whether it is still in the recipe.
        present: bool,
    },
    /// A parameter selection, and whether the document still declares
    /// it.
    Param {
        /// The parameter.
        name: ParamName,
        /// Whether it is still declared.
        present: bool,
    },
    /// A face selection, and the resolution verdict its name got.
    Face {
        /// The selection.
        face: FaceSelection,
        /// What the shipped resolution machinery answered — `None`
        /// when there is no evaluation to answer against yet, which is
        /// neither "live" nor "vanished" and is not reported as
        /// either.
        ///
        /// Boxed for the reason [`super::Refusal::Edit`] is: a `Resolution`
        /// carrying a diagnosis and a tombstone is an order of
        /// magnitude wider than the other arms here, and this value is
        /// returned by value on every frame.
        resolution: Option<Box<Resolution>>,
    },
    /// An edge selection, and the resolution verdict its name got.
    ///
    /// The same shape as [`Standing::Face`] because it is the same
    /// question asked of the same machinery: `resolve` takes a stable
    /// name and does not care which kind of entity minted it, so an
    /// edge selection survives its referent vanishing by exactly the
    /// face arm's rule rather than by a second implementation of it.
    Edge {
        /// The selection.
        edge: EdgeSelection,
        /// What the shipped resolution machinery answered — `None`
        /// when there is no evaluation to answer against yet.
        resolution: Option<Box<Resolution>>,
    },
}

impl Standing {
    /// Whether the selection denotes something the chrome may edit
    /// against.
    ///
    /// The one predicate the dependent affordances read. A face whose
    /// name did not resolve is NOT live; so is one with no evaluation
    /// behind it yet, because "we cannot tell" and "yes" are not the
    /// same answer and only one of them may enable a button.
    pub fn live(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Node { present, .. } | Self::Param { present, .. } => *present,
            Self::Face { resolution, .. } | Self::Edge { resolution, .. } => {
                matches!(resolution.as_deref(), Some(Resolution::Resolved(_)))
            }
        }
    }

    /// The typed unresolved verdict, when the selection has one.
    ///
    /// `Some` exactly when a picked entity's name failed to resolve or
    /// the evaluation could not answer for it — the two arms that
    /// render distinctly, for a face and for an edge alike.
    pub fn unresolved(&self) -> Option<&Resolution> {
        match self {
            Self::Face {
                resolution: Some(resolution),
                ..
            }
            | Self::Edge {
                resolution: Some(resolution),
                ..
            } if !matches!(**resolution, Resolution::Resolved(_)) => Some(resolution),
            _ => None,
        }
    }
}

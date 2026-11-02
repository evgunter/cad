//! **The derived impls, kept as the reference the hand-written ones
//! answer to** (`names::nest`'s tests).
//!
//! [`StableName`], [`RoleSeg`] and [`Qualifier`] as they were before
//! any walk over a name was written by hand: the same fields and
//! variants, the same serde attributes, every impl derived; and
//! [`NameRef`] as the handle it was, rendered, compared and written as
//! the name it holds. `RoleSeg` and `Qualifier` are copied from
//! `role.rs` with their docs left out (leaf types — the piece
//! locators and the run of them, `PieceRun`, whose one-or-many wire is
//! specified rather than derived — are shared); the variant row builds one
//! segment of every variant in both families from one list, so a
//! variant missing here stops that row's build.
#![allow(dead_code, unreachable_pub)]

use std::sync::Arc;

pub(super) use super::role::{
    CapEnd, EntityKind, MeridianEnd, PieceRole, PieceRun, ProfileEdgeRef, ProfileVertexRef,
    RimSupport, SectionCircle,
};
pub(super) use crate::node::{RecipeNodeId, StepId};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub(super) struct StableName {
    pub(super) kind: EntityKind,
    pub(super) node: RecipeNodeId,
    pub(super) path: Vec<RoleSeg>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub(super) struct RunLine {
    pub(super) line: NameRef,
    pub(super) sense: Sense,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub(super) enum Sense {
    Enters,
    Leaves,
    Outside,
    Inside,
    Through { entering: Vec<NameRef> },
}

/// The shared handle: its name's rendering, order, hash and wire form.
#[derive(Clone)]
pub(super) struct NameRef(Arc<StableName>);

impl NameRef {
    pub(super) fn new(name: StableName) -> Self {
        Self(Arc::new(name))
    }
}

impl core::fmt::Debug for NameRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for NameRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}

impl Eq for NameRef {}

impl core::hash::Hash for NameRef {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialOrd for NameRef {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NameRef {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if Arc::ptr_eq(&self.0, &other.0) {
            return core::cmp::Ordering::Equal;
        }
        self.0.cmp(&other.0)
    }
}

impl serde::Serialize for NameRef {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(ser)
    }
}

impl<'de> serde::Deserialize<'de> for NameRef {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        StableName::deserialize(de).map(Self::new)
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub(super) enum Qualifier {
    Borders(Vec<StableName>),
    Keeps(Vec<StableName>),
    Ends(Vec<StableName>),
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub(super) enum RoleSeg {
    OutputBody,

    Cap(CapEnd),
    Lateral(PieceRun),
    RimEdge(CapEnd, PieceRun),
    LateralEdge(ProfileVertexRef),
    CapVertex(CapEnd, ProfileVertexRef),

    LoftWall(Vec<ProfileEdgeRef>),
    LoftSeam(Vec<ProfileVertexRef>),
    LoftSlabWall(u32, Vec<ProfileEdgeRef>),
    LoftSlabSeam(u32, Vec<ProfileVertexRef>),
    LoftSectionEdge(u32, ProfileEdgeRef),
    LoftSectionVertex(u32, ProfileVertexRef),

    Band(PieceRun),
    BandRim(ProfileVertexRef),
    BandRimPi(ProfileVertexRef),
    BandPi(PieceRun),
    Meridian(MeridianEnd, PieceRun),
    MeridianVertex(MeridianEnd, ProfileVertexRef),
    RevolveCap(MeridianEnd),
    Pole(ProfileVertexRef),
    AxisEdge(PieceRun),

    From {
        read: crate::VarId,
        of: NameRef,
    },
    Seam {
        a: NameRef,
        b: NameRef,
    },
    Crossing {
        edge: NameRef,
        face: NameRef,
        sense: Sense,
    },
    EdgeCrossing(Vec<RunLine>),
    Merged(Vec<StableName>),
    Fragment(Qualifier),

    SplitBody,
    SectionFace,
    SectionEdge {
        face: NameRef,
    },
    SplitFragment {
        parent: NameRef,
    },
    CrossingVertex {
        edge: NameRef,
        sense: Sense,
    },
    OnToolVertex {
        of: NameRef,
    },

    BlendFace(NameRef),
    CornerFace(NameRef),
    TrimEdge {
        edge: NameRef,
        support: NameRef,
    },
    FootVertex {
        vertex: NameRef,
        support: NameRef,
    },
    EndArc {
        vertex: NameRef,
        edge: NameRef,
    },
    Mitre {
        vertex: NameRef,
    },
    TurnFoot {
        vertex: NameRef,
    },
    BandFace(Vec<StableName>),
    BandTrim {
        edge: NameRef,
        support: RimSupport,
    },
    BandFoot(NameRef),
    BandCross {
        edge: NameRef,
        band: Vec<StableName>,
    },
    BandCut(NameRef),
    BandSlit {
        edge: NameRef,
        band: Vec<StableName>,
    },

    Inner(NameRef),
    Rim(NameRef),
    HoleRim {
        of: NameRef,
        hole: u32,
    },

    Member {
        at: Vec<(crate::VarId, u32)>,
        of: NameRef,
    },
}

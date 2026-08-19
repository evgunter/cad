//! **Mates** — the A3 declaration node and its constructive solve
//! (ASSEMBLY-DESIGN A3/A11/A12; ASM-R2a spec D-1…D-5).
//!
//! A mate states one relation between two instances: which frames
//! coincide, with which axis senses, at which clocking. That statement
//! is BOTH the placement constraint and the contact declaration — one
//! node kind, no second vocabulary to keep synced (A3).
//!
//! # What a mate is, structurally
//!
//! A mate is a **leaf**: its `a`/`b` are instance-qualified stable
//! names, and name references are not DAG edges (the shipped D3
//! carve-out, which `Declare` established). What A12 adds on top is
//! the *reading* edge: the instantiate node each name's head resolves
//! through, RECOMPUTED from the recipe at need
//! ([`reading_edges`]) and never stored beside it. The partitions
//! divide on that distinction — A9's relative-freedom components and
//! A11's placement clusters run over consuming ∪ reading edges, while
//! A10's coverage, ancestor-freedom, maintenance and product gather
//! run over consuming edges only. A mate is therefore an ordinary
//! non-body root: an isolated sink under consuming edges, listed like
//! any other, denoting no body, ignored by the gather.
//!
//! # What a mate says, geometrically
//!
//! [`Alignment`] carries two mate frames — one per side, in that
//! instance's OWN part coordinates — plus the primitive relating
//! them, the axis sense, and the clocking rider. Nothing here reads
//! geometry: the frames are authored data, so the whole solve is a
//! decided-predicate computation over the recipe (A11's "no geometry
//! inspection, no numerics beyond decided predicates").
//!
//! Each primitive pins the pair's relative pose to a COSET of an
//! SE(3) subgroup, and multiple mates on one pair fold by exact coset
//! intersection — [`coset`], whose table is the spec's binding one.
//!
//! # v1's admitted classes
//!
//! `class` is the KERNEL [`ContactClass`] (M9-1), re-exported rather
//! than re-minted, so a mate's declaration is already the currency the
//! boolean wrapper's records speak. v1 admits `Rest` and `Tangent`
//! structurally; every other class — `Fit { gap }` when it lands —
//! refuses typed at the solve door naming the deferral, because a
//! declared clearance changes what "coincide" means and this unit
//! solves coincidence only.

use crate::node::RecipeNodeId;
use geom_core::linalg::frame::FrameError;
use geom_core::linalg::{Affine3, Point3, Vec3};
use geom_core::predicate::{BandError, Indeterminate};

pub mod coset;
pub mod solve;

pub use coset::{Coset, Subgroup};
pub use solve::{
    ClusterMaintenance, MateRole, PairSolve, SolvedPoses, clusters, gauge_of, reading_edges,
    relative_freedom_components, solve_document,
};

/// The kernel's contact vocabulary, re-exported (M9-1 PR-1: one enum,
/// defined lowest). A mate's class IS a contact declaration.
pub use topo::ContactClass;

/// Which side of a mate a diagnostic is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MateSide {
    /// The `a` reference.
    A,
    /// The `b` reference.
    B,
}

impl MateSide {
    /// The side's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
        }
    }
}

/// One side's **mate frame**, in that instance's own part coordinates:
/// an origin, the primary axis (a planar rest's normal, a coaxial
/// mate's axis), and the clocking reference that fixes roll.
///
/// Authored data, not geometry read back: the solve is structural plus
/// decided predicates over exactly these numbers (A11). The frame is
/// built through [`geom_core::linalg::frame::point_at`] — U4B's frame
/// family, reused rather than reinvented — so a degenerate axis or a
/// reference on the axis line refuses through that ladder's typed
/// voice instead of silently producing a rank-deficient basis.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MateFrame {
    /// The frame's origin (part coordinates).
    pub origin: [f64; 3],
    /// The primary axis: a rest plane's normal, a coaxial axis. Need
    /// not be unit; only its direction is read.
    pub axis: [f64; 3],
    /// The clocking reference, fixing roll about `axis`. Need not be
    /// perpendicular to the axis; only its perpendicular part is read.
    pub reference: [f64; 3],
}

impl MateFrame {
    /// The rigid placement this frame denotes: local +Z is `axis`,
    /// local origin is `origin`, roll fixed by `reference` (U4B's
    /// `point_at` convention, verbatim).
    ///
    /// # Errors
    ///
    /// [`FrameError`] when the axis has no definite direction or the
    /// reference has no definite perpendicular offset from it.
    pub fn placement(&self) -> Result<Affine3<f64>, FrameError> {
        let eye = Point3::new(self.origin[0], self.origin[1], self.origin[2]);
        let axis = Vec3::new(self.axis[0], self.axis[1], self.axis[2]);
        let reference = Vec3::new(self.reference[0], self.reference[1], self.reference[2]);
        geom_core::linalg::frame::point_at(eye, eye + axis, reference)
    }
}

/// Which way the two sides' axes point at each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AxisSense {
    /// The axes point the same way (a shaft into a through-hole).
    Aligned,
    /// The axes point at each other (a rest: two outward normals
    /// meeting). This is what kills every π-flip ambiguity — the
    /// senses are authored, never inferred.
    Opposed,
}

/// The **mate primitive**: which coset of SE(3) this mate pins the
/// pair's relative pose to (A11 rule 1; the spec's coset table).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatePrimitive {
    /// The two mate frames coincide outright — residual trivial.
    FrameCoincidence,
    /// The two axes coincide as a LINE — residual cylindrical (rotation
    /// about + translation along it).
    Coaxial,
    /// `b`'s plane rests on `a`'s plane, displaced by `offset` along
    /// `a`'s axis — residual planar. A zero offset is the flush rest;
    /// a nonzero one is an authored standoff, not a `Fit`.
    PlanarRest {
        /// The signed standoff along `a`'s axis, in metres.
        offset: f64,
    },
    /// Clocking with NO carrying primitive. Representable precisely so
    /// it can be REFUSED: the table lacks the entry by design (a bare
    /// angular relation pins no coset of the shapes the table closes
    /// over), and an unrepresentable refusal is an untestable one.
    Clocking,
}

impl MatePrimitive {
    /// The primitive's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::FrameCoincidence => "frame-coincidence",
            Self::Coaxial => "coaxial",
            Self::PlanarRest { .. } => "planar-rest",
            Self::Clocking => "clocking",
        }
    }
}

/// The A3 alignment datum: which frames coincide, the axis senses, and
/// the clocking rider.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alignment {
    /// The `a` side's mate frame, in `a`'s part coordinates.
    pub a: MateFrame,
    /// The `b` side's mate frame, in `b`'s part coordinates.
    pub b: MateFrame,
    /// Which coset this mate pins.
    pub primitive: MatePrimitive,
    /// Which way the axes point at each other.
    pub sense: AxisSense,
    /// The clocking rider: the signed angle (radians) from `a`'s
    /// reference to `b`'s about the shared axis. A RIDER, never a
    /// primitive — on [`MatePrimitive::Coaxial`] it cuts the residual
    /// to prismatic along the axis; on
    /// [`MatePrimitive::FrameCoincidence`] it is redundant-or-
    /// contradictory and gets decided; on a planar rest the table has
    /// no entry and the solve refuses typed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clocking: Option<f64>,
}

impl Alignment {
    /// The **lever arm** this mate's angular decisions turn on: the
    /// largest distance in its own authored data over which an angular
    /// error accumulates into a gap (D4 ¶1 — an angle only means
    /// something through the displacement it induces at a length
    /// scale, and the scale is named at the call site).
    ///
    /// The unit-metre floor is what keeps a mate authored AT the origin
    /// from claiming an arbitrarily tight angular threshold: with no
    /// length in the datum there is no lever, and a metre is the
    /// session box's own order of magnitude (D4 ¶4).
    pub fn lever_arm(&self) -> f64 {
        let norm = |v: [f64; 3]| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        let offset = match self.primitive {
            MatePrimitive::PlanarRest { offset } => offset.abs(),
            _ => 0.0,
        };
        norm(self.a.origin)
            .max(norm(self.b.origin))
            .max(offset)
            .max(1.0)
    }

    /// Whether every authored coordinate is finite — the edit door's
    /// admission test, the placement registry's rule applied one level
    /// out (a non-finite alignment could never decide anything).
    pub fn is_finite(&self) -> bool {
        let finite = |v: &[f64; 3]| v.iter().all(|x| x.is_finite());
        let frame = |f: &MateFrame| finite(&f.origin) && finite(&f.axis) && finite(&f.reference);
        frame(&self.a)
            && frame(&self.b)
            && self.clocking.is_none_or(f64::is_finite)
            && match self.primitive {
                MatePrimitive::PlanarRest { offset } => offset.is_finite(),
                _ => true,
            }
    }
}

/// **The A11 rule-4 recourse**, verbatim: what an author does about an
/// under-determined tree mate.
pub const UNDER_RECOURSE: &str = "add the complementary mate, or delete the mate if free relative \
                                  motion was intended";

/// **The v1 class restriction, named.** `Fit` is specified and not
/// built; a mate cannot declare a designed clearance until the kernel
/// variant lands with its first consumer, and AQ6 is where the
/// cross-document detail is still open.
pub const CLASS_DEFERRAL: &str = "v1 mates admit Rest and Tangent; the cross-document detail of \
                                  a designed clearance is undischarged";

/// A typed mate refusal (D9: fail loud, never a guess). Every arm names
/// its subject — the mate, the pair, the predicate, or the residual.
#[derive(Debug, Clone, PartialEq)]
pub enum MateFault {
    /// A mate frame's authored data has no definite frame.
    Frame {
        /// The mate whose datum refused.
        mate: RecipeNodeId,
        /// Which side.
        side: MateSide,
        /// The frame ladder's own refusal.
        error: FrameError,
    },
    /// A class outside v1's admitted vocabulary — `Fit { gap }` today,
    /// any later addition tomorrow. The message carries the kernel's
    /// own deferral sentence VERBATIM ([`topo::FIT_DEFERRAL`]) rather
    /// than a paraphrase of it.
    ClassNotAdmitted {
        /// The mate that declared it.
        mate: RecipeNodeId,
    },
    /// The coset table has no entry for this combination, and the
    /// refuse-any-missing-pair rule applies.
    TableLacks {
        /// The mate whose datum has no entry.
        mate: RecipeNodeId,
        /// What was asked for, in the table's own words.
        what: &'static str,
    },
    /// A case-split predicate landed in the ambiguity band — the typed
    /// escalation, never a silent pick.
    Indeterminate {
        /// The mate being folded when it fired.
        mate: RecipeNodeId,
        /// The predicate's diagnostics (it names itself).
        diag: Box<Indeterminate>,
    },
    /// The run's tolerance could not yield a band.
    Band {
        /// The band constructor's refusal.
        error: BandError,
    },
    /// The pair's mates intersect to the EMPTY coset (A11 rule 1's
    /// CONTRADICTORY): names both mates, the predicate that failed,
    /// and the measured clash.
    Contradictory {
        /// The mate already folded.
        held: RecipeNodeId,
        /// The mate whose intersection died against it.
        added: RecipeNodeId,
        /// The predicate that decided against them.
        predicate: &'static str,
        /// The measured clash, in metres (the margin that should have
        /// been zero and was not).
        clash: f64,
    },
    /// A tree mate left a positive-dimensional residual (A11 rule 4's
    /// UNDER): names the pair, the residual subgroup, and its
    /// parameters.
    Under {
        /// The tree mate that failed to determine.
        mate: RecipeNodeId,
        /// The instance the tree was extending from.
        parent: RecipeNodeId,
        /// The instance it failed to place.
        child: RecipeNodeId,
        /// What survived the fold.
        residual: Subgroup,
    },
    /// A mate's name head does not resolve to a live instantiate node
    /// (N5's dangling reference). It contributes no reading edge; the
    /// solve refuses typed rather than pretending the mate is absent.
    DanglingHead {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side dangles.
        side: MateSide,
        /// The head the name claims.
        head: RecipeNodeId,
    },
    /// A mate names ONE instance on both sides. A pair is two
    /// instances; a self-mate constrains nothing and is a recipe
    /// mistake, refused rather than folded into a tautology.
    SelfMate {
        /// The mate.
        mate: RecipeNodeId,
        /// The instance it names twice.
        instance: RecipeNodeId,
    },
}

impl core::fmt::Display for MateFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Frame { mate, side, error } => write!(
                f,
                "mate {}'s {} frame has no definite placement: {error}",
                mate.0,
                side.name()
            ),
            Self::ClassNotAdmitted { mate } => write!(
                f,
                "mate {}'s contact class is not admitted in v1 — {} ({CLASS_DEFERRAL})",
                mate.0,
                topo::FIT_DEFERRAL
            ),
            Self::TableLacks { mate, what } => write!(
                f,
                "mate {}: the coset table has no entry for {what} — the table refuses every pair \
                 it lacks rather than inventing one",
                mate.0
            ),
            Self::Indeterminate { mate, diag } => write!(
                f,
                "mate {}: a case split could not be decided — {}",
                mate.0,
                diag.payload()
            ),
            Self::Band { error } => write!(f, "the mate solve could not build a band: {error}"),
            Self::Contradictory {
                held,
                added,
                predicate,
                clash,
            } => write!(
                f,
                "mates {} and {} cannot both hold: predicate `{predicate}` measured a clash of \
                 {clash} m where their cosets would have had to meet",
                held.0, added.0
            ),
            Self::Under {
                mate,
                parent,
                child,
                residual,
            } => write!(
                f,
                "mate {} does not determine instance {} from instance {}: {} survives — \
                 {UNDER_RECOURSE}",
                mate.0,
                child.0,
                parent.0,
                residual.describe()
            ),
            Self::DanglingHead { mate, side, head } => write!(
                f,
                "mate {}'s {} reference resolves through node {}, which is not a live instance — \
                 rebind it",
                mate.0,
                side.name(),
                head.0
            ),
            Self::SelfMate { mate, instance } => write!(
                f,
                "mate {} names instance {} on both sides; a mate relates a PAIR",
                mate.0, instance.0
            ),
        }
    }
}

impl core::error::Error for MateFault {}

/// The wire spelling of a mate's [`ContactClass`] — the SAME stable
/// strings a `Declare` pair's class uses
/// ([`crate::node::declare_pairs_wire`]'s table, reused rather than
/// re-spelled). One contact vocabulary, one spelling of it: a mate's
/// `rest` and a declaration's `rest` are the same six bytes because
/// they are the same table.
///
/// The enum is `#[non_exhaustive]` and additive, so this is where a
/// class outside v1's vocabulary meets the file format. BOTH
/// directions refuse — the serialize direction too, since a kernel
/// newer than this writer can present a class with no spelling here
/// and a guessed tag would be read back as a different class. The
/// refusal quotes [`topo::FIT_DEFERRAL`] verbatim, which is the same
/// sentence [`MateFault::ClassNotAdmitted`] says at the solve door.
pub mod class_wire {
    use super::ContactClass;
    use crate::node::declare_pairs_wire::{tag, untag};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes the class as its stable lowercase spelling.
    ///
    /// # Errors
    ///
    /// A class with no spelling in this build refuses rather than
    /// inventing one.
    pub fn serialize<S: Serializer>(class: &ContactClass, ser: S) -> Result<S::Ok, S::Error> {
        let spelling = tag(*class).ok_or_else(|| {
            serde::ser::Error::custom(format!(
                "persist: this build has no wire spelling for the mate's contact class (a newer \
                 kernel's vocabulary) — refusing to write a guessed tag. {}",
                topo::FIT_DEFERRAL
            ))
        })?;
        ser.serialize_str(spelling)
    }

    /// Reads a stable lowercase spelling back.
    ///
    /// # Errors
    ///
    /// An unknown spelling refuses typed, naming the deferral.
    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<ContactClass, D::Error> {
        let spelling = String::deserialize(de)?;
        untag(&spelling).ok_or_else(|| {
            serde::de::Error::custom(format!(
                "unknown contact class '{spelling}' — v1 mates spell `rest` and `tangent`; {}",
                topo::FIT_DEFERRAL
            ))
        })
    }
}

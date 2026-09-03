//! **Geometric selector predicates** (LIB-SEL1; `docs/SELECT-DESIGN.md`
//! §1, ratified #286) — the vocabulary [`select_where`](super::select_where) filters a
//! structural selection with.
//!
//! # One implementation, two doors
//!
//! The geometric tests themselves are the KERNEL's ([`topo::query`],
//! the kernel query seat): kind mirrors, kind-set comparands, the
//! per-key predicates and the decided distance funnel are defined
//! there, beside `Body`, and re-exported here — the `ContactClass`
//! layering shape (defined lowest, re-exported upward), so the
//! document door and a kernel-direct caller run the same one
//! implementation. What this module OWNS is everything name-flavored:
//! the [`GeomPred`] atom vocabulary (whose datum is a recipe
//! reference), its resolution against an evaluation ([`prepare`]),
//! and the [`SelectRefusal`]s that name the candidate a refusal is
//! about.
//!
//! # The load-bearing split: EXACT vs DECIDED
//!
//! The design's central observation is that the predicate alphabet
//! splits in two, and only one half is a margins-discipline site.
//!
//! - **EXACT** — [`GeomPred::CurveKind`], [`GeomPred::SurfaceKind`],
//!   [`GeomPred::AdjacentKinds`] read the carrier's enum TAG. Post-#256
//!   (always-promote; "exact analytic geometry has exactly one native
//!   representation") the tag IS the semantic kind: no plane-shaped
//!   NURBS hides from a tag match on the main path. These atoms are
//!   total, deterministic, trivially equivariant (kinds are
//!   motion-invariant) and scalar-independent. They go through NO
//!   funnel and carry NO margin, deliberately: minting a fake margin
//!   for a tag match would be dimension-laundering in the other
//!   direction. Documented residual — geometry that is value-analytic
//!   but declared/structured as a spline does not tag-match; that is
//!   #256's intended semantics, not a defect (the recipe said spline,
//!   the selector believes the recipe).
//! - **DECIDED** — [`GeomPred::DatumDistance`] is a real numeric
//!   comparison and therefore a `k_stats::decide` site with a named
//!   `sel_*` predicate, an honest [`geom_core::Margin`] door,
//!   and a typed refusal on an in-band candidate. It participates in
//!   the K census exactly like a kernel site (GS-Q1 ruled (a): the
//!   naming convention does the separating, not a second funnel).
//!
//! # Position is DATUM-RELATIVE, never world-frame (GS-Q6)
//!
//! A raw "z ≈ 1" filter bakes an absolute frame into a selection rule,
//! which the margins discipline's intrinsic-quantities preference
//! forbids. [`GeomPred::DatumDistance`] instead references a datum
//! NODE like every other input, so the rule commutes with rigid
//! motions: move the datum with the part and the selection is
//! unchanged. Every document has an origin datum, so the cost is one
//! explicit argument.
//!
//! # Reserved, unbuilt: convexity (GS-Q2)
//!
//! No 3D edge-convexity predicate exists in the kernel (the only
//! shipped convex/reflex classification is the 2D vertex-sector one in
//! `topo::boolean::sectors`), its honest comparand is not obviously a
//! length (likely a flagged-lane ledger conversation), and the demand
//! evidence is a COMMENT, not a call site — P10 got "concave rim" via
//! an adjacent-kind pair, which this module ships. The names
//! `GeomPred::Convex` / `GeomPred::Reflex` are reserved by this note
//! and deliberately NOT built: a wrong margin design shipped into a
//! public API is far more expensive than a follow-up unit.

use geom_core::{Band, Decide, Sign};
use topo::{Body, query};

use crate::eval::{DatumValue, Evaluation, NodeResult, ValuePayload};
use crate::expr::{Dimension, Expr, ParamEnv};
use crate::names::InterrogateError;
use crate::names::role::StableName;
use crate::names::table::EntityKey;
use crate::node::RecipeNodeId;

// The kernel query seat's vocabulary, re-exported at its historical
// home so this crate's public surface is unchanged (see the module
// docs' layering note).
pub use topo::query::{
    ALL_SURFACE_KINDS, CurveKind, CurveKindSet, SEL_DATUM_DISTANCE, SurfaceKindSet,
};

/// The comparison a [`GeomPred::DatumDistance`] makes against its
/// stated value: the SIGN trilean, never a bare float equality.
///
/// `Approx` is the funnel's `Zero` arm — "within the document's
/// ε-band of the stated value" — which is the only honest reading of
/// "equals" for a measured length. `Greater`/`Less` are the strict
/// arms; a candidate whose margin lands INSIDE the band answers
/// neither and refuses (see [`SelectRefusal::InBand`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cmp {
    /// Within the ε-band of the value.
    Approx,
    /// Definitely greater than the value.
    Greater,
    /// Definitely less than the value.
    Less,
}

impl Cmp {
    /// The lowercase symbol, for refusal rendering.
    #[must_use]
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Approx => "~=",
            Self::Greater => ">",
            Self::Less => "<",
        }
    }
}

/// One geometric atom. A `&[GeomPred]` is their CONJUNCTION — the
/// algebra stays union-of-conjunctions end to end (run
/// [`select_where`](super::select_where) twice and concatenate for a
/// geometric union, exactly as the structural selector's union works).
///
/// Every atom is POINTWISE: there is no ordering, no "nearest", no
/// "first". That is deliberate — the S8 selection-ladder problem
/// (choosing ONE of several) never arises, so v1 needs no tie-break
/// design; a future "nearest entity to X" door is where that
/// precedent would bind.
#[derive(Debug, Clone, PartialEq)]
pub enum GeomPred {
    /// EXACT: the edge's certified carrier is one of these kinds.
    /// Non-edge names never match; an edge whose carrier is not
    /// certified (a null scaffold) never matches — exact atoms are
    /// TOTAL, so "no carrier" is an honest no, not a refusal.
    CurveKind(CurveKindSet),
    /// EXACT: the face's surface is one of these kinds. Non-face names
    /// never match.
    SurfaceKind(SurfaceKindSet),
    /// EXACT: the two faces across an EDGE have kinds drawn one from
    /// each set — UNORDERED, so `(Plane, Sphere)` matches a rim
    /// whichever half-edge carries which. The unordered reading is
    /// what makes the atom equivariant under a reflection that swaps
    /// the sides. Non-edge names never match.
    AdjacentKinds(SurfaceKindSet, SurfaceKindSet),
    /// DECIDED: the entity's distance to a datum, compared against a
    /// stated length.
    ///
    /// `datum` is a [`crate::Node::Datum`] node reference like every
    /// other input (GS-Q6), which is what keeps the rule equivariant:
    /// move the datum with the part and the selection commutes. The
    /// distance is SIGNED against a datum plane (along its normal) and
    /// UNSIGNED to a datum axis or point.
    ///
    /// **Where an entity IS** (the design under-specifies this; the
    /// reading here is the smallest one that invents no convention):
    /// the point measured is the one LIB-U5's shipped read-back doors
    /// already answer "where is it?" with — a vertex's stored
    /// position, an edge's certified carrier frame origin, a face's
    /// carrier frame origin. Nothing new is nominated, and the U5
    /// refusal travels with it: a NURBS face has no canonical frame,
    /// so it REFUSES rather than being silently dropped from the
    /// result. (The corpus demand agrees — the boss-plate's
    /// hand-written find tests a CIRCLE edge carrier's centre `z`,
    /// which is exactly this point.)
    DatumDistance {
        /// The datum node measured against.
        datum: RecipeNodeId,
        /// Which side of the value a candidate must land on.
        cmp: Cmp,
        /// The stated length. `Dimension::Length` — any other
        /// dimension refuses.
        value: Expr,
    },
    // RESERVED, unbuilt (GS-Q2): `Convex` / `Reflex`. See the module
    // docs — the slot is named there so the door is visibly open.
}

/// Why [`select_where`](super::select_where) could not answer.
///
/// The structural [`select`](fn@super::select) is infallible; adding
/// DECIDED predicates adds two honesty obligations it never had —
/// an indeterminate margin must not silently include OR exclude a
/// candidate, and a tied name whose candidates DISAGREE cannot be
/// half-selected. Both are refusals here rather than filter outcomes,
/// because silence in either direction lies about the result set.
///
/// A purely-EXACT filter produces none of these on a well-formed name
/// table: the exact atoms are total, so [`InBand`](Self::InBand) and
/// [`TiedDisagrees`](Self::TiedDisagrees) are unreachable without a
/// decided atom. The one residual is [`Unreadable`](Self::Unreadable),
/// which `select_where` raises if a table entry points at a body index
/// its node's payload does not have — an emitter invariant violation
/// rather than a query outcome, and reported rather than swallowed.
/// One door with one contract was preferred over splitting into an
/// infallible and a fallible materializer.
#[derive(Debug)]
#[non_exhaustive]
pub enum SelectRefusal {
    /// A candidate's decided margin landed strictly inside the
    /// ambiguity band: neither side of the comparison is certified.
    /// Including it would be a razor-thin selection cliff, excluding
    /// it would be a silent drop — so the query refuses and NAMES the
    /// candidate.
    InBand {
        /// The candidate whose margin was indeterminate.
        name: Box<StableName>,
        /// The funnel site ([`SEL_DATUM_DISTANCE`]).
        predicate: &'static str,
        /// The funnel's own diagnostic (margin, band, recourse).
        source: geom_core::Indeterminate,
    },
    /// A TIED name's candidates disagreed: some matched the
    /// conjunction and some did not. All-match includes the name
    /// (still tied — downstream still owns the ambiguity refusal),
    /// none-match excludes it; MIXED cannot be half-selected (GS-Q4).
    TiedDisagrees {
        /// The tied name.
        name: Box<StableName>,
        /// How many of its candidates matched.
        matched: usize,
        /// How many candidates the tie has.
        candidates: usize,
    },
    /// A DECIDED atom could not read the candidate's position — the
    /// U5 read-back refusal, surfaced rather than swallowed (a NURBS
    /// face has no canonical frame; a whole-body name has no point).
    Unreadable {
        /// The candidate that could not be read.
        name: Box<StableName>,
        /// Why.
        error: InterrogateError,
    },
    /// The referenced datum node is not an evaluated datum.
    NotADatum {
        /// The node referenced by [`GeomPred::DatumDistance`].
        datum: RecipeNodeId,
        /// What that node produced instead (`ValuePayload::kind_name`),
        /// or why it has no value at all.
        found: &'static str,
    },
    /// The stated value is not a length (`Dimension::Length`) — the
    /// comparand of a distance must be a distance.
    NotALength {
        /// What dimension the expression actually has.
        dim: crate::expr::Dimension,
    },
    /// A candidate PAIR's verify-door margin landed inside the
    /// ambiguity band, or the door could not decide it: the flush
    /// detector reports only DEFINITE findings, and an in-band pair
    /// must be neither reported nor silently dropped (SELECT-DESIGN
    /// §3a) — so the query refuses and NAMES the pair. The detector's
    /// pair-shaped sibling of [`InBand`](Self::InBand).
    PairInBand {
        /// The face-name pair whose margin was indeterminate.
        pair: Box<(StableName, StableName)>,
        /// The verify-door funnel site (a `bool_plane_*` predicate —
        /// detection reuses the C4 verifier's own sites and mints
        /// none of its own).
        predicate: &'static str,
        /// The funnel's own diagnostic (margin, band, recourse).
        source: geom_core::Indeterminate,
    },
    /// The stated value expression did not evaluate.
    BadValue(crate::expr::EvalError),
    /// The ambiguity band itself could not be built (a broken ambient
    /// tolerance — the same fail-loud rung `names::discriminate` uses).
    Band,
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in the query's own vocabulary — candidate, tie, band,
// datum, comparand — and NAMES the candidate the refusal is about, as
// the refusals themselves promise to. A name renders as its kind plus
// its minting node, the product layer's spelling: a role path is a
// derivation, not something a person reads mid-sentence.
//
// The in-band arms forward the funnel's whole `Indeterminate` Display,
// recourse tail included, rather than its bare payload: a selection
// margin IS a decidability question, so the three-lever coincidence
// sentence is the right one here (unlike a contact site, where it is
// not). The arms that wrap another layer's refusal forward that
// layer's words.
impl core::fmt::Display for SelectRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let named = |f: &mut core::fmt::Formatter<'_>, name: &StableName| {
            write!(f, "the {} minted by node {}", name.kind.noun(), name.node.0)
        };
        match self {
            Self::InBand {
                name,
                predicate,
                source,
            } => {
                f.write_str("select: ")?;
                named(f, name)?;
                write!(
                    f,
                    " is neither certified in nor out — '{predicate}' left it inside the \
                     ambiguity band, and a query neither drops a candidate silently nor \
                     selects on a razor-thin cliff: {source}"
                )
            }
            Self::TiedDisagrees {
                name,
                matched,
                candidates,
            } => {
                f.write_str("select: ")?;
                named(f, name)?;
                write!(
                    f,
                    " is a tied name whose {candidates} candidates disagree — {matched} match \
                     the query and the rest do not, and a tie cannot be half-selected; \
                     disambiguate the name, or ask something all its candidates answer alike"
                )
            }
            Self::Unreadable { name, error } => {
                f.write_str("select: ")?;
                named(f, name)?;
                write!(f, " could not be read to decide the query: {error}")
            }
            Self::NotADatum { datum, found } => write!(
                f,
                "select: the query measures from node {}, which produced {found} rather than \
                 a datum — point a distance query at an evaluated datum",
                datum.0
            ),
            Self::NotALength { dim } => write!(
                f,
                "select: the comparand of a distance is a distance, and this expression has \
                 dimension {dim}"
            ),
            Self::PairInBand {
                pair,
                predicate,
                source,
            } => {
                f.write_str("select: the pair (")?;
                named(f, &pair.0)?;
                f.write_str(", ")?;
                named(f, &pair.1)?;
                write!(
                    f,
                    ") is neither certified in nor out — '{predicate}' left its margin inside \
                     the ambiguity band, and detection reports only definite findings: {source}"
                )
            }
            Self::BadValue(error) => {
                write!(f, "select: the stated value did not evaluate: {error}")
            }
            Self::Band => f.write_str(
                "select: the ambiguity band itself could not be built — the ambient tolerance \
                 is broken, so no comparison below it can be trusted",
            ),
        }
    }
}

impl core::error::Error for SelectRefusal {}

// ---------------------------------------------------------------
// Evaluating the atoms against one resolved candidate.
//
// Split out from `select_where` so the materializer reads as the
// doctrine it implements (evaluate → resolve → store) and the
// EXACT/DECIDED asymmetry lives in one place: every arm below except
// `Distance` is a total tag read that cannot fail, and `Distance` is
// the only one that touches the funnel.
// ---------------------------------------------------------------

/// An atom with its recipe references already resolved against the
/// evaluation: the datum read once and the value expression evaluated
/// once, rather than per candidate.
pub(crate) enum Prepared<'a, T: Decide> {
    /// [`GeomPred::CurveKind`].
    Curve(CurveKindSet),
    /// [`GeomPred::SurfaceKind`].
    Surface(SurfaceKindSet),
    /// [`GeomPred::AdjacentKinds`].
    Adjacent(SurfaceKindSet, SurfaceKindSet),
    /// [`GeomPred::DatumDistance`], resolved.
    Distance {
        /// The datum's evaluated geometry.
        datum: &'a DatumValue<T>,
        /// Which side of `value` a candidate must land on.
        cmp: Cmp,
        /// The stated length, evaluated.
        value: T,
    },
}

/// Resolves every atom's recipe references once, before any candidate
/// is looked at.
///
/// # Errors
///
/// [`SelectRefusal::NotADatum`], [`SelectRefusal::NotALength`],
/// [`SelectRefusal::BadValue`] — all three are STATIC faults of the
/// query itself, so they surface before a single margin is taken.
pub(crate) fn prepare<'a, T: Decide>(
    ev: &'a Evaluation<T>,
    geom: &[GeomPred],
    params: &ParamEnv<T>,
) -> Result<Vec<Prepared<'a, T>>, SelectRefusal> {
    geom.iter()
        .map(|atom| match atom {
            GeomPred::CurveKind(s) => Ok(Prepared::Curve(*s)),
            GeomPred::SurfaceKind(s) => Ok(Prepared::Surface(*s)),
            GeomPred::AdjacentKinds(a, b) => Ok(Prepared::Adjacent(*a, *b)),
            GeomPred::DatumDistance { datum, cmp, value } => {
                if value.dim() != Dimension::Length {
                    return Err(SelectRefusal::NotALength { dim: value.dim() });
                }
                let found = match ev.nodes.get(datum) {
                    Some(NodeResult::Ok(v)) => match &v.payload {
                        ValuePayload::Datum(d) => {
                            return Ok(Prepared::Distance {
                                datum: d,
                                cmp: *cmp,
                                value: crate::expr::eval(value, params)
                                    .map_err(SelectRefusal::BadValue)?,
                            });
                        }
                        other => other.kind_name(),
                    },
                    Some(NodeResult::Failed(_)) => "a failed node",
                    Some(NodeResult::Poisoned { .. }) => "a poisoned node",
                    None => "an unevaluated node",
                };
                Err(SelectRefusal::NotADatum {
                    datum: *datum,
                    found,
                })
            }
        })
        .collect()
}

/// Whether ONE resolved candidate satisfies the whole conjunction.
///
/// `name` rides along only so a refusal can name the candidate it is
/// about — nothing here matches on it.
///
/// # Errors
///
/// [`SelectRefusal::InBand`] when a decided margin is indeterminate,
/// [`SelectRefusal::Unreadable`] when a decided atom cannot read the
/// candidate's position. The EXACT atoms never error: a missing
/// carrier, a wrong entity kind or an unreadable adjacency is an
/// honest NO, which is what makes a purely-exact filter total.
pub(crate) fn candidate_matches<T: Decide>(
    body: &Body<T>,
    key: EntityKey,
    atoms: &[Prepared<'_, T>],
    band: Band,
    name: &StableName,
) -> Result<bool, SelectRefusal> {
    for atom in atoms {
        let ok = match atom {
            // The kinds an atom does not apply to answer an honest NO,
            // and they are LISTED rather than swept up: an entity kind
            // added to the model is a decision about which atoms can
            // read it, and it is owed here rather than defaulted to no.
            Prepared::Curve(set) => match key {
                EntityKey::Edge(e) => query::edge_carrier_matches(body, e, *set),
                EntityKey::Body | EntityKey::Face(_) | EntityKey::Vertex(_) => false,
            },
            Prepared::Surface(set) => match key {
                EntityKey::Face(f) => query::face_surface_matches(body, f, *set),
                EntityKey::Body | EntityKey::Edge(_) | EntityKey::Vertex(_) => false,
            },
            Prepared::Adjacent(a, b) => match key {
                EntityKey::Edge(e) => query::edge_adjacent_matches(body, e, *a, *b),
                EntityKey::Body | EntityKey::Face(_) | EntityKey::Vertex(_) => false,
            },
            Prepared::Distance { datum, cmp, value } => {
                let point = super::interrogate::entity_point(body, key).map_err(|error| {
                    SelectRefusal::Unreadable {
                        name: Box::new(name.clone()),
                        error,
                    }
                })?;
                match query::datum_distance_sign(datum, point, *value, band) {
                    Ok(sign) => match cmp {
                        Cmp::Approx => sign == Sign::Zero,
                        Cmp::Greater => sign == Sign::Positive,
                        Cmp::Less => sign == Sign::Negative,
                    },
                    Err(source) => {
                        return Err(SelectRefusal::InBand {
                            name: Box::new(name.clone()),
                            predicate: SEL_DATUM_DISTANCE,
                            source,
                        });
                    }
                }
            }
        };
        if !ok {
            return Ok(false);
        }
    }
    Ok(true)
}

//! **Geometric selector predicates** (LIB-SEL1; `docs/SELECT-DESIGN.md`
//! §1, ratified #286) — the vocabulary [`select_where`](super::select_where) filters a
//! structural selection with.
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

use geom_brep::SurfaceKind;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign};
use topo::{Body, Curve3, CurveGeom, EdgeKey, HalfEdgeKey, Surface};

use crate::eval::{DatumValue, Evaluation, NodeResult, ValuePayload};
use crate::expr::{Dimension, Expr, ParamEnv};
use crate::names::InterrogateError;
use crate::names::role::StableName;
use crate::names::table::EntityKey;
use crate::node::RecipeNodeId;

/// Which [`Curve3`] variant a carrier is: the fieldless mirror of the
/// curve enum, and the edge-side twin of [`SurfaceKind`].
///
/// The mirror is hand-written and [`CurveKind::of`]'s match is
/// EXHAUSTIVE with no wildcard arm, so adding a `Curve3` variant fails
/// to compile here rather than silently classifying as something else
/// — the same fail-loud tripwire `SegTag::of` uses for role segments.
///
/// (Placement: `SurfaceKind` lives in `geom-brep` rather than beside
/// `Surface` in `geom`, so "the mirror lives where it is
/// used" is the shipped precedent; this one lives in the crate that
/// selects with it. Moving it down beside `Curve3` later is additive.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CurveKind {
    /// [`Curve3::Line`].
    Line,
    /// [`Curve3::Circle`].
    Circle,
    /// [`Curve3::Ellipse`].
    Ellipse,
    /// [`Curve3::Nurbs`].
    Nurbs,
}

impl CurveKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Line, Self::Circle, Self::Ellipse, Self::Nurbs];

    /// The kind of a carrier (exhaustive by construction — type docs).
    #[must_use]
    pub fn of<T: geom_core::Real>(c: &Curve3<T>) -> Self {
        match c {
            Curve3::Line { .. } => Self::Line,
            Curve3::Circle { .. } => Self::Circle,
            Curve3::Ellipse { .. } => Self::Ellipse,
            Curve3::Nurbs(_) => Self::Nurbs,
        }
    }

    /// This kind's bit position in a [`CurveKindSet`].
    const fn bit(self) -> u8 {
        match self {
            Self::Line => 0,
            Self::Circle => 1,
            Self::Ellipse => 2,
            Self::Nurbs => 3,
        }
    }

    /// The lowercase name, for refusal rendering.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Circle => "circle",
            Self::Ellipse => "ellipse",
            Self::Nurbs => "nurbs",
        }
    }
}

/// A SET of [`CurveKind`]s — the atom's comparand, so "a line or an
/// arc" is one predicate rather than a union of two selections.
///
/// A bitset, not a `Vec`: the mirror is closed and tiny, so the value
/// is `Copy`, `Ord` and canonical (no ordering or duplicate freedom to
/// disagree about between two equal sets).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CurveKindSet(u8);

impl CurveKindSet {
    /// The set of exactly these kinds. An EMPTY set matches nothing
    /// (the same posture as an empty [`Selector`](super::Selector)).
    #[must_use]
    pub fn of(kinds: impl IntoIterator<Item = CurveKind>) -> Self {
        Self(kinds.into_iter().fold(0, |acc, k| acc | (1 << k.bit())))
    }

    /// The singleton set — the common case.
    #[must_use]
    pub fn just(kind: CurveKind) -> Self {
        Self::of([kind])
    }

    /// Whether `kind` is a member.
    #[must_use]
    pub fn contains(self, kind: CurveKind) -> bool {
        self.0 & (1 << kind.bit()) != 0
    }

    /// Whether the set is empty (matches nothing).
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The members, in [`CurveKind::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = CurveKind> {
        CurveKind::ALL
            .into_iter()
            .filter(move |k| self.contains(*k))
    }
}

/// The [`SurfaceKind`] bit position in a [`SurfaceKindSet`].
///
/// EXHAUSTIVE with no wildcard arm: a new `SurfaceKind` variant fails
/// to compile here (and `ALL_SURFACE_KINDS` below is pinned against
/// this function by a unit test, so the pair cannot drift apart).
const fn surface_bit(kind: SurfaceKind) -> u8 {
    match kind {
        SurfaceKind::Plane => 0,
        SurfaceKind::Cylinder => 1,
        SurfaceKind::Cone => 2,
        SurfaceKind::Sphere => 3,
        SurfaceKind::Torus => 4,
        SurfaceKind::Nurbs => 5,
    }
}

/// Every [`SurfaceKind`], in declaration order — the iteration order of
/// a [`SurfaceKindSet`].
pub const ALL_SURFACE_KINDS: [SurfaceKind; 6] = [
    SurfaceKind::Plane,
    SurfaceKind::Cylinder,
    SurfaceKind::Cone,
    SurfaceKind::Sphere,
    SurfaceKind::Torus,
    SurfaceKind::Nurbs,
];

/// A SET of [`SurfaceKind`]s — [`CurveKindSet`]'s face-side twin, and
/// the comparand of both [`GeomPred::SurfaceKind`] and each side of
/// [`GeomPred::AdjacentKinds`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SurfaceKindSet(u8);

impl SurfaceKindSet {
    /// The set of exactly these kinds. An EMPTY set matches nothing.
    #[must_use]
    pub fn of(kinds: impl IntoIterator<Item = SurfaceKind>) -> Self {
        Self(
            kinds
                .into_iter()
                .fold(0, |acc, k| acc | (1 << surface_bit(k))),
        )
    }

    /// The singleton set — the common case.
    #[must_use]
    pub fn just(kind: SurfaceKind) -> Self {
        Self::of([kind])
    }

    /// Whether `kind` is a member.
    #[must_use]
    pub fn contains(self, kind: SurfaceKind) -> bool {
        self.0 & (1 << surface_bit(kind)) != 0
    }

    /// Whether the set is empty (matches nothing).
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The members, in [`ALL_SURFACE_KINDS`] order.
    pub fn iter(self) -> impl Iterator<Item = SurfaceKind> {
        ALL_SURFACE_KINDS
            .into_iter()
            .filter(move |k| self.contains(*k))
    }

    /// The kind of a carrier — the tag read the EXACT atoms are.
    #[must_use]
    pub fn kind_of<T: geom_core::Real>(s: &Surface<T>) -> SurfaceKind {
        SurfaceKind::of(s)
    }
}

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

/// **The funnel site name** of the decided position predicate — the
/// `sel_*` prefix SELECT-DESIGN §1 proposes, so any K-census consumer
/// can tell selector margins from kernel ones by name alone (GS-Q1's
/// separation mechanism). Its comparand is a genuine length (the
/// signed/unsigned distance minus the stated value), so it goes
/// through the plain [`Margin::of`](geom_core::Margin::of) door and
/// owes NO `docs/predicate-dimension-audit.md` row — the flagged lane
/// is for comparands that cannot honestly be lengths.
///
/// A K row name reaching the funnel through a const, not a literal at
/// the decide site, so it is a roster carrier (`docs/K-REPORT.md`,
/// "The inventory method, restated").
pub const SEL_DATUM_DISTANCE: &str = "sel_datum_distance";

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

/// The surface kind on one side of an edge, or `None` where the
/// adjacency or its geometry is not there to read.
fn face_kind_across<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<SurfaceKind> {
    let h = body.get_half_edge(he)?;
    let f = body.get_loop(h.parent_loop)?.face;
    body.get_surface(body.get_face(f)?.surface)
        .map(SurfaceKind::of)
}

/// The certified carrier kind of an edge, or `None` for an
/// uncertified (null-scaffold) one.
fn carrier_kind<T: Decide>(body: &Body<T>, e: EdgeKey) -> Option<CurveKind> {
    let curve = body.get_edge(e)?.curve;
    body.get_curve_geom(curve)
        .and_then(CurveGeom::certified)
        .map(|c| CurveKind::of(c.carrier()))
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
            Prepared::Curve(set) => match key {
                EntityKey::Edge(e) => carrier_kind(body, e).is_some_and(|k| set.contains(k)),
                _ => false,
            },
            Prepared::Surface(set) => match key {
                EntityKey::Face(f) => body
                    .get_face(f)
                    .and_then(|face| body.get_surface(face.surface))
                    .is_some_and(|s| set.contains(SurfaceKind::of(s))),
                _ => false,
            },
            Prepared::Adjacent(a, b) => match key {
                EntityKey::Edge(e) => body.get_edge(e).is_some_and(|edge| {
                    match (
                        face_kind_across(body, edge.he_plus),
                        face_kind_across(body, edge.he_minus),
                    ) {
                        // UNORDERED (the atom's equivariance): the
                        // pair matches whichever side carries which.
                        (Some(p), Some(m)) => {
                            (a.contains(p) && b.contains(m)) || (a.contains(m) && b.contains(p))
                        }
                        _ => false,
                    }
                }),
                _ => false,
            },
            Prepared::Distance { datum, cmp, value } => {
                let point = super::interrogate::entity_point(body, key).map_err(|error| {
                    SelectRefusal::Unreadable {
                        name: Box::new(name.clone()),
                        error,
                    }
                })?;
                let margin = datum_margin(datum, point) - *value;
                match decide(SEL_DATUM_DISTANCE, Margin::of(margin), band) {
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

/// The distance of `p` from a datum: SIGNED along a plane's normal
/// (which is stored unit, so the dot product is already a length),
/// UNSIGNED to an axis or a point.
///
/// The comparand handed to the funnel is this MINUS the stated value —
/// a length minus a length, so [`Margin::of`] is the honest door and
/// the site owes no dimension-audit row.
fn datum_margin<T: Decide>(datum: &DatumValue<T>, p: Point3<T>) -> T {
    match datum {
        DatumValue::Plane { origin, normal } => (p - *origin).dot(*normal),
        DatumValue::Axis { origin, dir } => {
            let v = p - *origin;
            (v - *dir * v.dot(*dir)).norm()
        }
        DatumValue::Point { position } => (p - *position).norm(),
    }
}

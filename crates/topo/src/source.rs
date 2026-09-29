//! **`GeomSource`** — N6's recipe-source identity for geometric
//! descriptions (NAMING-DESIGN N6, ratified #74; M4 PR 5).
//!
//! Every surface/curve/point description minted by *evaluation*
//! carries the recipe expression that produced its parameters, as a
//! side record parallel to the geometry arenas (the D5 provenance
//! pattern: identity bookkeeping beside the data, never inside it).
//! Same-source is **syntactic identity** of the whole
//! `(node, expr, orient)` triple — a provenance lookup, zero numerics.
//!
//! **The retirement theorem (N6)**: same `GeomSource` ⇒ bit-identical
//! descriptions, by D9 determinism of expression evaluation. The
//! converse is deliberately NOT claimed — equal bits without shared
//! source stay unglued (the coincidence ladder's ratified rung (b)).
//! The bit comparison survives only as debug assertions, read through
//! `surface_bits_witness` and `data_bits_witness` — which answer
//! nothing at a scalar with no bit channel, so the theorem is asserted
//! exactly where its premise can be read. Its home is
//! [`crate::Body::set_surface_source`], against every key of the body
//! as a stamp lands. Two readers assert it again, each over pairs that
//! door never compared: the face merge, whose body may hold stamps a
//! graft carried in (`Body::carry_surface_rows` copies rows unread),
//! and `oriented_plane_eq`'s rung 1, whose planes may come from two
//! bodies and whose mirrored pair no stamp states. Chart-region's
//! cross-body read needs a verdict rather than an assertion, and
//! decides it through its own exact-bracket comparator.
//!
//! # Two questions called "same chart"
//!
//! - **Do two keys hold one description?** Row carry asks it: a pcurve
//!   row moves to another surface key only if it is about the value that
//!   key holds. `Body::same_chart` answers from identity — one
//!   key, or one shared payload `Arc`.
//! - **Did the recipe declare two keys one surface?** Gluing and merging
//!   ask it. `surface_declaration` answers from one key or one
//!   `GeomSource` (N6); the face merge adds the faces' `sense`, since it
//!   asks whether two faces are one region, and chart-region adds its
//!   bracketed read of the descriptions.
//!
//! Neither reads the other's evidence. A stamp declares intent and
//! proves no value, so a row carried on one could land on a surface it
//! is not about; and identity is not a declaration, so gluing on it would
//! glue what the recipe never said was one — the coincidence ladder's
//! rung (b), turned from equal bits to a shared payload.
//!
//! **Layering**: the recipe vocabulary (node ids, expression paths)
//! lives in `editor-core`, which depends on this crate — so the
//! fields here are the *lowered* pure-data forms (`u64` node ids,
//! structural expression addresses). `editor-core` constructs them
//! from its typed `RecipeNodeId`/`ExprPath`; this crate only ever
//! compares them for identity and flips orientation.
//!
//! **Absence is not an origin.** A description with no `GeomSource`
//! is not thereby "un-sourced": it may have been imported, built by
//! hand, derived by a kernel op, or had its source CLEARED by
//! [`crate::Body::clear_geom_sources`] with the re-stamp that door
//! expects never running — a defect. [`GeomOrigin`] is the total
//! record a body keeps per description: `Imported`, `Cleared` and the
//! recipe stamp are each their own arm, and hand-built and
//! kernel-derived are one arm, `KernelDirect`, because no reader asks
//! which of the two a description is.
//!
//! A `GeomSource` is the payload of that record's `Recipe` arm, never
//! a spelling inside `GeomSource` itself: a source meaning "not from a
//! recipe" would compare equal to every other such spelling, and rung
//! 1 of the coincidence ladder is `GeomSource` equality, so two
//! unrelated imported surfaces would glue. N6 decides on `GeomSource`
//! and only on `GeomSource`; the other three arms decide nothing.
//!
//! **One level finer, for axes.** [`AxisSource`] is the same
//! discipline at the granularity of one COMPONENT of a description —
//! its axis line — kept in its own opt-in rows ([`AxisRecord`]) beside
//! the origin record rather than inside it: an axis is shared across
//! descriptions a `GeomSource` tells apart, and an imported axis can
//! have an identity no recipe gave it. It serves the design's LINE
//! reading (coaxiality) only: its "same point" (concentricity) and
//! "same direction" (parallelism) readings are not a line token's to
//! answer, and each would be a further component table beside this one.
//!
//! **Scope of the identity claim (PR 1 review ruling, binding)**:
//! ExprPath same-slot ancestor replacement silently re-points stale
//! paths, so a `GeomSource` must NOT be assumed re-point-detectable —
//! identity claims hold per evaluation against the current document,
//! never across unaudited document mutations.

/// N6's orientation tag: whether the description is the source
/// expression's value (`Id`) or its orientation-reversal (`Rev`,
/// minted by `revert`; `rev ∘ rev = id`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Or {
    /// The expression's value as evaluated.
    Id,
    /// The orientation-reversed description (negated plane).
    Rev,
}

impl Or {
    /// `rev ∘ rev = id` (N6).
    pub fn flip(self) -> Self {
        match self {
            Self::Id => Self::Rev,
            Self::Rev => Self::Id,
        }
    }
}

/// The (lowered) recipe expression that produced a description's
/// parameters (N6: composition through transforms wraps the base).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceExpr {
    /// The minting op's `index`-th description of its geometric kind,
    /// in deterministic mint (arena) order. Per-evaluation identity:
    /// D9 determinism makes the index a function of the recipe, which
    /// is exactly the scope the identity claim holds at (module docs).
    Minted {
        /// Arena-order mint index within the minting node's output.
        index: u32,
    },
    /// A placement node's rigid map applied to an upstream source
    /// (N6: "the transform node composes into `expr`"). Equal chains
    /// ⇒ equal maps applied to equal descriptions ⇒ equal bits (D9).
    Placed {
        /// The placing recipe node, lowered id.
        node: u64,
        /// The placed body's OUTPUT ordinal in the placing node's value
        /// (a pattern's flat `j·M + i`, a Transform's body index), so
        /// that distinct bodies of one node never share a source. Not
        /// the placement index an [`AxisPlacement`] keys on.
        instance: u32,
        /// The source expression being placed.
        inner: Box<SourceExpr>,
    },
}

/// N6's `GeomSource { node, expr, orient }`: the recipe source of one
/// geometric description. Identity (including `orient`) is the
/// declared-coincidence rung's entire test.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeomSource {
    /// The recipe node whose evaluation minted the description
    /// (lowered `RecipeNodeId`).
    pub node: u64,
    /// The expression that produced the description's parameters.
    pub expr: SourceExpr,
    /// Orientation relative to the expression's value.
    pub orient: Or,
}

impl GeomSource {
    /// A freshly minted base source: `node`'s `index`-th description.
    pub fn minted(node: u64, index: u32) -> Self {
        Self {
            node,
            expr: SourceExpr::Minted { index },
            orient: Or::Id,
        }
    }

    /// This source placed by rigid-transform node `placed_by` as its
    /// output body `instance` ([`SourceExpr::Placed`]'s ordinal):
    /// the placing node composes into `expr` (N6), `node` and
    /// `orient` are untouched (a rigid placement neither re-mints nor
    /// reverses the description).
    pub fn placed(&self, placed_by: u64, instance: u32) -> Self {
        Self {
            node: self.node,
            expr: SourceExpr::Placed {
                node: placed_by,
                instance,
                inner: Box::new(self.expr.clone()),
            },
            orient: self.orient,
        }
    }

    /// The orientation-reversed source (`revert`; N6: `orient` flips).
    pub fn reverted(&self) -> Self {
        Self {
            node: self.node,
            expr: self.expr.clone(),
            orient: self.orient.flip(),
        }
    }

    /// Same base (node + expr), ignoring orientation — the
    /// SameOriented/SameOpposite discriminator's premise.
    pub fn same_base(&self, other: &Self) -> bool {
        self.node == other.node && self.expr == other.expr
    }
}

/// A refused GeomSource attachment (closed enum, D3 style).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceAttachError {
    /// The key does not resolve in its arena — attaching identity to
    /// nothing is a caller bug, refused loudly.
    StaleKey,
}

impl core::fmt::Display for SourceAttachError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StaleKey => write!(f, "geom-source attachment: stale geometry key"),
        }
    }
}

impl std::error::Error for SourceAttachError {}

/// What the recipe declared about two surface keys — the module docs'
/// second question, asked of keys on one body or on two. The key rung
/// exists only when `body_a` and `body_b` are one body: arena keys mean
/// nothing across arenas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceDeclaration {
    /// One key of one body.
    SameKey,
    /// Two keys stamped with one [`GeomSource`], orientation included.
    SameSource,
    /// One source base under opposite orientations: the recipe declared
    /// the one surface and its reversal, whose charts mirror.
    Mirrored,
    /// Two different sources: equal-but-independent descriptions do not
    /// glue.
    DistinctSources,
    /// A key with no source: nothing is declared.
    Unsourced,
}

impl SurfaceDeclaration {
    /// The recipe declared the two keys one surface.
    pub(crate) fn one_surface(self) -> bool {
        matches!(self, Self::SameKey | Self::SameSource)
    }
}

/// [`SurfaceDeclaration`] of surface `a` on `body_a` and `b` on
/// `body_b`: a provenance lookup that reads no scalar.
pub(crate) fn surface_declaration<T: geom_core::Real>(
    body_a: &crate::Body<T>,
    a: crate::SurfaceKey,
    body_b: &crate::Body<T>,
    b: crate::SurfaceKey,
) -> SurfaceDeclaration {
    if core::ptr::eq(body_a, body_b) && a == b {
        return SurfaceDeclaration::SameKey;
    }
    source_declaration(body_a.surface_source(a), body_b.surface_source(b))
}

/// [`SurfaceDeclaration`] of two descriptions by their sources alone —
/// [`surface_declaration`] past its key rung, and the whole question
/// where the descriptions have no keys (a face's outward plane, whose
/// source has the face's `sense` composed into `orient`). Never
/// [`SurfaceDeclaration::SameKey`].
pub(crate) fn source_declaration(
    a: Option<&GeomSource>,
    b: Option<&GeomSource>,
) -> SurfaceDeclaration {
    match (a, b) {
        (Some(x), Some(y)) if x == y => SurfaceDeclaration::SameSource,
        (Some(x), Some(y)) if x.same_base(y) => SurfaceDeclaration::Mirrored,
        (Some(_), Some(_)) => SurfaceDeclaration::DistinctSources,
        _ => SurfaceDeclaration::Unsourced,
    }
}

/// Assertion-build agreement of two surface descriptions of any kind:
/// the analytic kinds through [`geom::Surface::paired_with`]'s one walk
/// of their data, the spline kinds through their payloads. Two
/// different kinds, or two NURBS nets of different shape, disagree at
/// any scalar; a shared payload `Arc` agrees unread; otherwise every
/// part is compared, and a part that differs decides the answer even
/// where another part has no bit channel to read.
///
/// **Tri-state, because the theorem's premise is not readable at
/// every scalar.** `eq_bits` answers `None` where the scalar has no
/// bit channel (`Dual`, `Sym`), and that is the right never-equal
/// DEFAULT for a verdict that must decide — but an ASSERTION reading
/// `None` as "the bits disagree" asserts the unknowable, and fires on
/// a same-source pair whose bits are identical at `f64`. So this
/// answers `None` there: no evidence, nothing to assert. The
/// assertion sites assert only on `Some`.
///
/// This file's witnesses are this crate's only bit-identity call
/// sites: `cfg(debug_assertions)`-gated, never a production consumer
/// (the CI tripwire allowlists this file on exactly that
/// justification).
#[cfg(debug_assertions)]
pub(crate) fn surface_bits_witness<T: geom_core::Real>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
) -> Option<bool> {
    use geom::SurfacePairing as P;
    match a.paired_with(b) {
        P::KindsDiffer => Some(false),
        P::Analytic(data) => data_bits_witness(data.pairs().map(|(_, x, y)| (x, y))),
        P::Nurbs(x, y) if std::sync::Arc::ptr_eq(x, y) => Some(true),
        P::Nurbs(x, y) => nurbs_surface_bits_witness(x, y),
        P::Approx(x, y) if std::sync::Arc::ptr_eq(x, y) => Some(true),
        P::Approx(x, y) => {
            let (wx, wy) = (x.window(), y.window());
            let window = bits_witness([
                (wx.u.0, wy.u.0),
                (wx.u.1, wy.u.1),
                (wx.v.0, wy.v.0),
                (wx.v.1, wy.v.1),
            ]);
            joined([window, nurbs_surface_bits_witness(x.fit(), y.fit())])
        }
    }
}

/// Assertion-build agreement of paired data, datum by datum, with
/// [`surface_bits_witness`]'s tri-state.
#[cfg(debug_assertions)]
pub(crate) fn data_bits_witness<T: geom_core::Real>(
    pairs: impl IntoIterator<Item = (geom::DatumValue<T>, geom::DatumValue<T>)>,
) -> Option<bool> {
    joined(
        pairs
            .into_iter()
            .map(|(x, y)| bits_witness(x.scalars().zip(y.scalars()))),
    )
}

/// Assertion-build agreement of two NURBS surfaces: degrees and counts
/// decide unread; then knots, weights and control net are each
/// compared, so knots or weights that differ answer `Some(false)` even
/// where the net's scalar has no bit channel.
#[cfg(debug_assertions)]
fn nurbs_surface_bits_witness<T: geom_core::Real>(
    x: &geom::NurbsSurface<T>,
    y: &geom::NurbsSurface<T>,
) -> Option<bool> {
    let shape = x.knots_u().degree() == y.knots_u().degree()
        && x.knots_v().degree() == y.knots_v().degree()
        && x.knots_u().knots().len() == y.knots_u().knots().len()
        && x.knots_v().knots().len() == y.knots_v().knots().len()
        && x.control_counts() == y.control_counts();
    if !shape {
        return Some(false);
    }
    let reals = |a: &[f64], b: &[f64]| bits_witness(a.iter().copied().zip(b.iter().copied()));
    joined([
        reals(x.knots_u().knots(), y.knots_u().knots()),
        reals(x.knots_v().knots(), y.knots_v().knots()),
        reals(x.weights(), y.weights()),
        bits_witness(
            x.control()
                .iter()
                .zip(y.control())
                .flat_map(|(a, b)| [(a.x, b.x), (a.y, b.y), (a.z, b.z)]),
        ),
    ])
}

/// Folds part verdicts: any part that differs decides `Some(false)`;
/// otherwise a part with no bit channel leaves `None`.
#[cfg(debug_assertions)]
fn joined(parts: impl IntoIterator<Item = Option<bool>>) -> Option<bool> {
    parts
        .into_iter()
        .try_fold(Some(true), |acc, part| match part {
            Some(false) => Err(()),
            Some(true) => Ok(acc),
            None => Ok(None),
        })
        .unwrap_or(Some(false))
}

/// `Some(all pairs bit-equal)` where the scalar has a bit channel,
/// `None` where it has none — the one fold both witnesses share, so
/// a channel-less scalar cannot read as disagreement at either.
///
/// The signature carries no `;` before its brace: the debug-only gate
/// (`scripts/gates/bit-identity-debug-only.sh`) ends a gated item's
/// read at one, so a `[T; N]` parameter here would report this use
/// ungated (`work/issues/bit-identity-debug-only-gate-ends-an-item-at-a-semicolon`).
#[cfg(debug_assertions)]
fn bits_witness<T: geom_core::Real>(pairs: impl IntoIterator<Item = (T, T)>) -> Option<bool> {
    pairs.into_iter().try_fold(true, |agree, (a, b)| {
        geom_core::bit_identity::eq_bits(&a, &b).map(|eq| agree && eq)
    })
}

/// **Where a geometric description came from** — the total answer to
/// the question `Option<&GeomSource>` could not answer, and the one
/// ORIGIN row a [`crate::Body`] keeps per geometric description. A
/// surface's finer rows — its per-field [`crate::ParamSource`]s and its
/// [`AxisRecord`] — sit beside it, opt-in, and never stand in for it.
///
/// It tells apart what a missing `GeomSource` row cannot: an imported
/// description, one minted by a kernel door, and one
/// [`crate::Body::clear_geom_sources`] dropped whose re-stamp never
/// ran — the last a DEFECT, which absence would make
/// indistinguishable from the legitimate states.
///
/// **No absence arm, and no second map.** The recipe source IS the
/// `Recipe` arm's payload, so a description cannot carry a source and
/// a non-recipe origin at once — the exclusion two parallel maps would
/// have had to maintain by hand is unrepresentable, which is D9's
/// taxonomy row 0 answered rather than deferred to a `debug_assert`.
/// [`crate::Body::surface_source`] and its siblings are the projection
/// of this arm, and answer exactly what they answered before the other
/// three existed.
///
/// **Total over live keys.** The mint doors write
/// [`GeomOrigin::KernelDirect`] as a description enters its arena, so
/// a live key ALWAYS has a row and a missing one is a kernel bug the
/// origin readers announce (D9 row 4) rather than an origin. Two
/// obligations follow, stated once here instead of at each site that
/// carries them: a door that transplants a description carries its row
/// (the graft), and a door that drops one from an arena drops its row
/// (the orphan doors, `carve`'s sweeps). A surface's rows are several,
/// and `Body::carry_surface_rows` / `Body::drop_surface_rows` are the
/// one spelling of both for all of them. The second is hygiene rather
/// than a guarded invariant — generational keys mean a re-minted key
/// can never read a stranded row — but the OLD key would otherwise go
/// on answering for a description the body no longer holds.
///
/// **It decides nothing N6 decides.** The recipe-source identity the
/// coincidence ladder's rung 1 tests is [`GeomSource`] equality and
/// stays exactly that: `Recipe` is the only arm carrying one, the
/// other three carry no source at all, and two descriptions on the
/// same non-recipe arm are no more glued than two absences were.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeomOrigin {
    /// The recipe layer stamped this description with the expression
    /// that produced it — N6's identity channel, and the only arm any
    /// coincidence rung reads.
    Recipe(GeomSource),
    /// Adopted from an exchange file (D7), stamped by the importer at
    /// the door it ships from. Carries no recipe expression because
    /// there is no recipe: the file is the source.
    Imported,
    /// Minted through a kernel door with no recipe context — **written
    /// at the mint**, not inferred from a missing row.
    ///
    /// A hand-built body's description and one a kernel op derived are
    /// both this arm. They enter the arenas through the same
    /// crate-internal mint doors, so telling them apart would take a
    /// stamp decided at every public door that mints, and no reader
    /// asks the question: the importer's consumers need `Imported`
    /// told from the rest, and N6 reads only `Recipe`.
    KernelDirect,
    /// [`crate::Body::clear_geom_sources`] dropped a recipe source this
    /// description carried, and the re-stamp that door expects has not
    /// run. **This is the defect arm**: the clearing door is half of a
    /// pair, and a description resting here is the other half missing.
    /// A re-stamp through [`crate::Body::set_surface_source`] and its
    /// siblings overwrites it; [`crate::Body::mark_imported`]
    /// deliberately does not.
    Cleared,
}

impl GeomOrigin {
    /// The recipe source this origin carries, if it is `Recipe` — the
    /// projection [`crate::Body::surface_source`] and its siblings
    /// answer, and the whole of what N6's readers ever see.
    pub fn source(&self) -> Option<&GeomSource> {
        match self {
            Self::Recipe(source) => Some(source),
            Self::Imported | Self::KernelDirect | Self::Cleared => None,
        }
    }
}

/// **The recipe-level axis a description's AXIS COMPONENT came from**,
/// composed through every rigid placement applied to it since — the
/// per-component token of the axis channel
/// (`docs/AXIS-DECLARATION-DESIGN.md`).
///
/// A [`GeomSource`] identifies a whole description, so two cylinders
/// sharing one axis carry two different sources and the shared axis is
/// not derivable from them. This token identifies the axis alone: two
/// descriptions carry equal tokens exactly when the recipe layer
/// derived both axes from one recipe-level axis AND the same chain of
/// rigid MAPS has moved both since. Equality is the whole reading —
/// token comparison, zero numerics.
///
/// **What it claims is a LINE the description's axis lies on**, not
/// any stored component: two coaxial cylinders may store different
/// `origin`s along the line, so no bit agreement follows from equal
/// tokens, and none is asserted. For a cylinder, cone or torus the line
/// is the stored axis. For a sphere it is **a line through the
/// centre**: the stored `axis` is the pole of the sphere's chart, which
/// no reading of this token consults, so the recipe layer stamps a
/// sphere with the token of any recipe line its centre lies on,
/// whatever its pole.
///
/// **Two halves, two readabilities.** The base is an opaque byte string
/// the recipe layer lowered: the recipe vocabulary stays above the
/// layering line, as [`crate::ParamSource`]'s does, and this crate has
/// no decoder. The placement chain is readable, and has to be: a
/// declaration made stale by a placement applied to one carrier and not
/// the other refuses NAMING that placement.
///
/// It is placement data, so — unlike a `ParamSource`, whose stored
/// scalar is motion-invariant — rigid placement composes it:
/// `transform_rigid` marks the row [`AxisRecord::Cleared`] and the
/// recipe layer re-stamps [`AxisSource::placed`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AxisSource {
    base: std::sync::Arc<[u8]>,
    placements: Vec<AxisPlacement>,
}

/// One rigid MAP in an [`AxisSource`]'s chain: the placing recipe node
/// (lowered id) and which of that node's maps it applied.
///
/// This is not [`SourceExpr::Placed`]'s pair: that one keys on the
/// placed body's output ordinal, because distinct bodies must not share
/// a description source, while an axis's identity is the map — every
/// body one map places carries the axis to the same line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AxisPlacement {
    /// The placing recipe node, lowered id.
    pub node: u64,
    /// Which of the node's maps: 0 for a node with one (a Transform,
    /// an instantiation), the placement index for a pattern or a
    /// group boolean.
    pub index: u32,
}

/// The base's bytes stay out of every print — `Body` derives `Debug`,
/// and a derived impl here would spell them into every body dump. The
/// chain is printed: it is the readable half.
impl core::fmt::Debug for AxisSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { base, placements } = self;
        write!(
            f,
            "AxisSource(<{} bytes>, placed {placements:?})",
            base.len()
        )
    }
}

impl AxisSource {
    /// The token for an already-lowered recipe-level axis, unplaced.
    #[must_use]
    pub fn from_lowered(lowered: &[u8]) -> Self {
        Self {
            base: std::sync::Arc::from(lowered),
            placements: Vec::new(),
        }
    }

    /// This axis moved by node `node`'s map `index` ([`AxisPlacement`]):
    /// the new outermost placement.
    #[must_use]
    pub fn placed(&self, node: u64, index: u32) -> Self {
        let mut placements = self.placements.clone();
        placements.push(AxisPlacement { node, index });
        Self {
            base: self.base.clone(),
            placements,
        }
    }

    /// The placements applied since the axis was lowered, innermost
    /// first.
    #[must_use]
    pub fn placements(&self) -> &[AxisPlacement] {
        &self.placements
    }

    /// Same recipe-level axis, whatever has placed it since. Between
    /// unequal tokens this separates the stale case — one axis moved by
    /// different chains — from two unrelated axes.
    #[must_use]
    pub fn same_base(&self, other: &Self) -> bool {
        self.base == other.base
    }

    /// Whether the channel can name a line for `surface`'s kind: the
    /// four analytic kinds of revolution (a sphere's line being one
    /// through its centre, above). A plane stores a normal and a point
    /// on itself, not a line; the spline arms store a net.
    pub fn admits<T: geom_core::Real>(surface: &geom::Surface<T>) -> bool {
        use geom::Surface as S;
        match surface {
            S::Cylinder { .. } | S::Cone { .. } | S::Sphere { .. } | S::Torus { .. } => true,
            S::Plane { .. } | S::Nurbs(_) | S::Approx(_) => false,
        }
    }
}

/// **A surface's axis-channel row.** Opt-in: a surface the recipe layer
/// attached no axis to has no row, and which origin it has is
/// [`GeomOrigin`]'s answer. A row that exists is one of two states, so
/// a re-stamp that never ran is nameable rather than a silence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AxisRecord {
    /// The axis component's source.
    Source(AxisSource),
    /// [`crate::Body::clear_geom_sources`] moved the axis this row
    /// named and the recipe layer's re-stamp has not run — the defect
    /// arm, as [`GeomOrigin::Cleared`] is.
    Cleared,
}

impl AxisRecord {
    /// The source this row carries, unless it is cleared.
    pub fn source(&self) -> Option<&AxisSource> {
        match self {
            Self::Source(source) => Some(source),
            Self::Cleared => None,
        }
    }
}

/// A refused axis attachment (closed enum, D3 style). Both are caller
/// bugs, refused rather than recorded where nothing reads them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisAttachError {
    /// The key does not resolve in the surface arena.
    StaleKey,
    /// The channel names no line for the surface's kind
    /// ([`AxisSource::admits`]).
    NoAxisOnKind,
}

impl core::fmt::Display for AxisAttachError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StaleKey => write!(f, "axis-source attachment: stale surface key"),
            Self::NoAxisOnKind => write!(
                f,
                "axis-source attachment: the surface stores no axis (a plane or a spline)"
            ),
        }
    }
}

impl std::error::Error for AxisAttachError {}

#[cfg(test)]
mod tests {
    use geom::{Surface, SurfaceDatum as D};
    use geom_core::{Point3, Vec3};

    /// One analytic kind: a builder over a flat scalar list, the list,
    /// and which scalars each datum owns — written against the
    /// variant's fields, not read off the walk under test.
    type Kind = (
        fn(&[f64]) -> Surface<f64>,
        Vec<f64>,
        Vec<(D, core::ops::Range<usize>)>,
    );

    fn kinds() -> Vec<Kind> {
        fn pt(x: &[f64]) -> Point3<f64> {
            Point3::new(x[0], x[1], x[2])
        }
        fn dir(x: &[f64]) -> Vec3<f64> {
            Vec3::new(x[0], x[1], x[2])
        }
        vec![
            (
                |x| Surface::Plane {
                    origin: pt(&x[0..3]),
                    normal: dir(&x[3..6]),
                    u_ref: dir(&x[6..9]),
                },
                vec![1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
                vec![(D::Origin, 0..3), (D::Normal, 3..6), (D::URef, 6..9)],
            ),
            (
                |x| Surface::Cylinder {
                    origin: pt(&x[0..3]),
                    axis: dir(&x[3..6]),
                    radius: x[6],
                    u_ref: dir(&x[7..10]),
                },
                vec![1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0],
                vec![
                    (D::Origin, 0..3),
                    (D::Axis, 3..6),
                    (D::Radius, 6..7),
                    (D::URef, 7..10),
                ],
            ),
            (
                |x| Surface::Cone {
                    apex: pt(&x[0..3]),
                    axis: dir(&x[3..6]),
                    half_angle: x[6],
                    u_ref: dir(&x[7..10]),
                },
                vec![1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 0.5, 1.0, 0.0, 0.0],
                vec![
                    (D::Apex, 0..3),
                    (D::Axis, 3..6),
                    (D::HalfAngle, 6..7),
                    (D::URef, 7..10),
                ],
            ),
            (
                |x| Surface::Sphere {
                    center: pt(&x[0..3]),
                    radius: x[3],
                    axis: dir(&x[4..7]),
                    u_ref: dir(&x[7..10]),
                },
                vec![1.0, 2.0, 3.0, 2.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
                vec![
                    (D::Center, 0..3),
                    (D::Radius, 3..4),
                    (D::Axis, 4..7),
                    (D::URef, 7..10),
                ],
            ),
            (
                |x| Surface::Torus {
                    center: pt(&x[0..3]),
                    axis: dir(&x[3..6]),
                    major_radius: x[6],
                    minor_radius: x[7],
                    u_ref: dir(&x[8..11]),
                },
                vec![1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 3.0, 1.0, 1.0, 0.0, 0.0],
                vec![
                    (D::Center, 0..3),
                    (D::Axis, 3..6),
                    (D::MajorRadius, 6..7),
                    (D::MinorRadius, 7..8),
                    (D::URef, 8..11),
                ],
            ),
        ]
    }

    /// Every field-by-field reader of a surface reads every scalar:
    /// each scalar of each analytic kind, changed alone, is a
    /// difference to both comparators (the assertion build's bit
    /// witness and chart-region's bracketed read), and made NaN alone,
    /// is named by check 1's poison read. A walk that skips a field, at
    /// any of them, leaves that field's rows green where they must be
    /// red.
    #[test]
    fn every_surface_reader_reads_every_scalar() {
        for (build, base, fields) in kinds() {
            let at_rest = build(&base);
            let kind = format!("{at_rest:?}");
            assert!(
                crate::chart_region::surface_bits_equal(&at_rest, &build(&base)),
                "{kind}: a surface must read bit-equal to its own copy"
            );
            #[cfg(debug_assertions)]
            assert_eq!(
                super::surface_bits_witness(&at_rest, &build(&base)),
                Some(true),
                "{kind}: the witness must read a copy as agreeing"
            );
            assert!(
                crate::validate::poisoned_datums(&at_rest).is_empty(),
                "{kind}: a finite surface has no poisoned datum"
            );
            assert_eq!(
                fields.last().map(|(_, r)| r.end),
                Some(base.len()),
                "{kind}: the fixture's fields cover its scalars"
            );
            for (datum, scalars) in fields {
                for i in scalars {
                    let mut moved = base.clone();
                    moved[i] += 0.5;
                    let moved = build(&moved);
                    assert!(
                        !crate::chart_region::surface_bits_equal(&at_rest, &moved),
                        "{kind}: chart-region's read missed a change to {} (scalar {i})",
                        datum.name()
                    );
                    #[cfg(debug_assertions)]
                    assert_eq!(
                        super::surface_bits_witness(&at_rest, &moved),
                        Some(false),
                        "{kind}: the bit witness missed a change to {} (scalar {i})",
                        datum.name()
                    );
                    let mut poisoned = base.clone();
                    poisoned[i] = f64::NAN;
                    assert_eq!(
                        crate::validate::poisoned_datums(&build(&poisoned)),
                        vec![datum],
                        "{kind}: check 1 missed a NaN in {} (scalar {i})",
                        datum.name()
                    );
                }
            }
        }
    }

    /// **What each declaration says, and how it is reached.** Only one
    /// key or one source (orientation included) declares one surface;
    /// a source and its reversal declare a surface and its mirror,
    /// whose outward sides face apart, so a face on each does not glue.
    #[test]
    fn a_declaration_is_one_surface_only_for_one_key_or_one_source() {
        use super::{GeomSource, SurfaceDeclaration as S, surface_declaration};
        let plane = || Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let mut body = crate::Body::<f64>::new();
        let [a, b, c, d, e] = [(); 5].map(|()| body.add_surface(plane()));
        let source = GeomSource::minted(7, 0);
        for (key, stamp) in [
            (a, source.clone()),
            (b, source.clone()),
            (c, source.reverted()),
            (d, GeomSource::minted(7, 1)),
        ] {
            assert_eq!(body.set_surface_source(key, stamp), Ok(()));
        }
        let other = crate::Body::<f64>::new();
        for (x, y, on, declared, one) in [
            (a, a, &body, S::SameKey, true),
            (a, b, &body, S::SameSource, true),
            (a, c, &body, S::Mirrored, false),
            (a, d, &body, S::DistinctSources, false),
            (a, e, &body, S::Unsourced, false),
            // One key on two bodies is two arena slots, not one key.
            (e, e, &other, S::Unsourced, false),
        ] {
            let answer = surface_declaration(&body, x, on, y);
            assert_eq!(answer, declared, "{x:?} against {y:?}");
            assert_eq!(answer.one_surface(), one, "{declared:?}.one_surface()");
        }
    }
}

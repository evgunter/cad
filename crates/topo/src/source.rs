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
//! The bit comparison survives only as the debug assertions behind
//! the lookup, read through [`plane_bits_witness`] — which answers
//! nothing at a scalar with no bit channel, so the theorem is asserted
//! exactly where its premise can be read.
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
//! have an identity no recipe gave it.
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
        /// The placing recipe node (Transform or Pattern), lowered id.
        node: u64,
        /// The pattern instance index (0 for a plain Transform).
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

    /// This source placed by rigid-transform node `placed_by`
    /// (instance `instance` for patterns; 0 for a plain transform):
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

/// Debug-only bit agreement of two same-source plane descriptions —
/// the DESIGN.md M4 "records agree with bits" assertion, N6's
/// `debug_assert!(same_source ⇒ eq_bits)`. `opposite` = the sources'
/// orients differ (a `revert` pair): the normal must then be the
/// exact IEEE negation, the origin unchanged (see `revert`'s map).
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
pub(crate) fn plane_bits_witness<T: geom_core::Real>(
    o1: geom_core::Point3<T>,
    n1: geom_core::Vec3<T>,
    o2: geom_core::Point3<T>,
    n2: geom_core::Vec3<T>,
    opposite: bool,
) -> Option<bool> {
    let n2 = if opposite { -n2 } else { n2 };
    let pairs = [
        (o1.x, o2.x),
        (o1.y, o2.y),
        (o1.z, o2.z),
        (n1.x, n2.x),
        (n1.y, n2.y),
        (n1.z, n2.z),
    ];
    bits_witness(pairs)
}

/// Debug-only bit agreement of two vectors (the `u_ref` leg of the
/// merge-site assertion; same posture and same tri-state as
/// [`plane_bits_witness`]).
#[cfg(debug_assertions)]
pub(crate) fn vec3_bits_witness<T: geom_core::Real>(
    a: geom_core::Vec3<T>,
    b: geom_core::Vec3<T>,
) -> Option<bool> {
    bits_witness([(a.x, b.x), (a.y, b.y), (a.z, b.z)])
}

/// Assertion-build agreement of two surface descriptions of any kind — the
/// check behind [`crate::Body::set_surface_source`], with the tri-state
/// of [`plane_bits_witness`]. Two different kinds, or two NURBS nets of
/// different shape, disagree at any scalar; a shared payload `Arc`
/// agrees unread; otherwise every part is compared, and a part that
/// differs decides the answer even where another part has no bit
/// channel to read.
#[cfg(debug_assertions)]
pub(crate) fn surface_bits_witness<T: geom_core::Real>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
) -> Option<bool> {
    use geom::Surface as S;
    let p = |x: geom_core::Point3<T>, y: geom_core::Point3<T>| {
        bits_witness([(x.x, y.x), (x.y, y.y), (x.z, y.z)])
    };
    let v = |x: geom_core::Vec3<T>, y: geom_core::Vec3<T>| {
        bits_witness([(x.x, y.x), (x.y, y.y), (x.z, y.z)])
    };
    let s = |x: T, y: T| bits_witness([(x, y)]);
    match *a {
        S::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let S::Plane {
                origin: o,
                normal: n,
                u_ref: r,
            } = *b
            else {
                return Some(false);
            };
            joined([p(origin, o), v(normal, n), v(u_ref, r)])
        }
        S::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let S::Cylinder {
                origin: o,
                axis: ax,
                radius: r,
                u_ref: u,
            } = *b
            else {
                return Some(false);
            };
            joined([p(origin, o), v(axis, ax), s(radius, r), v(u_ref, u)])
        }
        S::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let S::Cone {
                apex: o,
                axis: ax,
                half_angle: h,
                u_ref: u,
            } = *b
            else {
                return Some(false);
            };
            joined([p(apex, o), v(axis, ax), s(half_angle, h), v(u_ref, u)])
        }
        S::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            let S::Sphere {
                center: c,
                radius: r,
                axis: ax,
                u_ref: u,
            } = *b
            else {
                return Some(false);
            };
            joined([p(center, c), s(radius, r), v(axis, ax), v(u_ref, u)])
        }
        S::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let S::Torus {
                center: c,
                axis: ax,
                major_radius: big,
                minor_radius: small,
                u_ref: u,
            } = *b
            else {
                return Some(false);
            };
            joined([
                p(center, c),
                v(axis, ax),
                s(major_radius, big),
                s(minor_radius, small),
                v(u_ref, u),
            ])
        }
        S::Nurbs(ref x) => {
            let S::Nurbs(ref y) = *b else {
                return Some(false);
            };
            if std::sync::Arc::ptr_eq(x, y) {
                return Some(true);
            }
            nurbs_surface_bits_witness(x, y)
        }
        S::Approx(ref x) => {
            let S::Approx(ref y) = *b else {
                return Some(false);
            };
            if std::sync::Arc::ptr_eq(x, y) {
                return Some(true);
            }
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
/// the question `Option<&GeomSource>` could not answer, and the ONE
/// row a [`crate::Body`] keeps per geometric description.
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
/// (the orphan doors, `carve`'s sweeps). The second is hygiene rather
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
/// placements has moved both since. Equality is the whole reading —
/// token comparison, zero numerics.
///
/// **What it claims is the axis LINE**, not the stored anchor point on
/// it: two coaxial cylinders may store different `origin`s along the
/// line, and a sphere's `center` is one point of its axis. So no bit
/// agreement follows from equal tokens, and none is asserted.
///
/// **Two halves, two readabilities.** The base is an opaque byte string
/// the recipe layer lowered: the recipe vocabulary stays above the
/// layering line, as [`crate::ParamSource`]'s does, and this crate has
/// no decoder. The placement chain is readable, and has to be: a
/// declaration made stale by a placement applied to one carrier and not
/// the other refuses NAMING that placement. The chain is the
/// `(node, instance)` spelling [`SourceExpr::Placed`] composes with,
/// not a second one.
///
/// It is placement data, so — unlike a `ParamSource`, whose stored
/// scalar is motion-invariant — rigid placement composes it:
/// `transform_rigid` marks the row [`AxisRecord::Cleared`] and the
/// recipe layer re-stamps [`AxisSource::placed`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AxisSource {
    base: std::sync::Arc<[u8]>,
    placements: Vec<SourcePlacement>,
}

/// One rigid placement in an [`AxisSource`]'s chain: the placing
/// recipe node (lowered id) and the pattern instance (0 for a plain
/// transform) — the pair [`SourceExpr::Placed`] wraps with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourcePlacement {
    /// The placing recipe node (Transform or Pattern), lowered id.
    pub node: u64,
    /// The pattern instance index (0 for a plain Transform).
    pub instance: u32,
}

/// The base's bytes stay out of every print — `Body` derives `Debug`,
/// and a derived impl here would spell them into every body dump. The
/// chain is printed: it is the readable half.
impl core::fmt::Debug for AxisSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "AxisSource(<{} bytes>, placed {:?})",
            self.base.len(),
            self.placements
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

    /// This axis placed by rigid-transform node `node` (instance
    /// `instance` for patterns): the new outermost placement, as
    /// [`GeomSource::placed`] composes.
    #[must_use]
    pub fn placed(&self, node: u64, instance: u32) -> Self {
        let mut placements = self.placements.clone();
        placements.push(SourcePlacement { node, instance });
        Self {
            base: self.base.clone(),
            placements,
        }
    }

    /// The placements applied since the axis was lowered, innermost
    /// first.
    #[must_use]
    pub fn placements(&self) -> &[SourcePlacement] {
        &self.placements
    }

    /// Same recipe-level axis, whatever has placed it since. Between
    /// unequal tokens this separates the stale case — one axis moved by
    /// different chains — from two unrelated axes.
    #[must_use]
    pub fn same_base(&self, other: &Self) -> bool {
        self.base == other.base
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

/// Whether `surface` stores an axis the channel can name: the four
/// analytic kinds of revolution. A plane stores a normal and a point on
/// itself, not a line; the spline arms store a net.
pub fn has_axis<T: geom_core::Real>(surface: &geom::Surface<T>) -> bool {
    use geom::Surface as S;
    match surface {
        S::Cylinder { .. } | S::Cone { .. } | S::Sphere { .. } | S::Torus { .. } => true,
        S::Plane { .. } | S::Nurbs(_) | S::Approx(_) => false,
    }
}

/// A refused axis attachment (closed enum, D3 style). Both are caller
/// bugs, refused rather than recorded where nothing reads them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisAttachError {
    /// The key does not resolve in the surface arena.
    StaleKey,
    /// The surface at the key stores no axis ([`has_axis`]).
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

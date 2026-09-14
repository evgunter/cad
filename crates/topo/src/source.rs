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
//! record a body keeps per description, and it separates THREE of
//! those four: `Imported`, `Cleared` and the recipe stamp are each
//! their own arm, while hand-built and kernel-derived share
//! `KernelDirect` because nothing inside the kernel can tell them
//! apart — the reason is on that arm, and the fourth separation is
//! `work/topo/kernel-direct-origin-does-not-separate-hand-built-from-derived`.
//!
//! A `GeomSource` is the payload of that record's `Recipe` arm, never
//! a spelling inside `GeomSource` itself: a source meaning "not from a
//! recipe" would compare equal to every other such spelling, and rung
//! 1 of the coincidence ladder is `GeomSource` equality, so two
//! unrelated imported surfaces would glue. N6 decides on `GeomSource`
//! and only on `GeomSource`; the other three arms decide nothing.
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
/// This is the ONE remaining bit-identity call site in this crate:
/// `cfg(debug_assertions)`-gated, never a production consumer (the
/// CI tripwire allowlists this file on exactly that justification).
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
    bits_witness(&pairs)
}

/// Debug-only bit agreement of two vectors (the `u_ref` leg of the
/// merge-site assertion; same posture and same tri-state as
/// [`plane_bits_witness`]).
#[cfg(debug_assertions)]
pub(crate) fn vec3_bits_witness<T: geom_core::Real>(
    a: geom_core::Vec3<T>,
    b: geom_core::Vec3<T>,
) -> Option<bool> {
    bits_witness(&[(a.x, b.x), (a.y, b.y), (a.z, b.z)])
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
fn bits_witness<T: geom_core::Real>(pairs: &[(T, T)]) -> Option<bool> {
    pairs.iter().try_fold(true, |agree, (a, b)| {
        geom_core::bit_identity::eq_bits(a, b).map(|eq| agree && eq)
    })
}


/// **Where a geometric description came from** — the total answer to
/// the question `Option<&GeomSource>` could not answer, and the ONE
/// row a [`crate::Body`] keeps per geometric description.
///
/// A missing `GeomSource` row conflated four origins: an imported
/// description, a hand-built one, one a kernel op derived, and one
/// [`crate::Body::clear_geom_sources`] dropped whose re-stamp never
/// ran — the last a DEFECT, indistinguishable from the three
/// legitimate states. Three of the four separate by name here; the
/// fourth separation is [`GeomOrigin::KernelDirect`]'s own doc.
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
    /// BOTH this arm today, because nothing inside the kernel can tell
    /// them apart: they enter the arenas through the same
    /// crate-internal mint doors, and what separates them is which
    /// caller opened the public door above
    /// (`work/topo/kernel-direct-origin-does-not-separate-hand-built-from-derived`).
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

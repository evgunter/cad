//! **THE C5 dispatch table** (M5 PR 5, C12.1): one total
//! `(SurfaceKind, SurfaceKind)` routing table for surface×surface
//! intersection, plus the rung-2 closed forms this PR lands.
//!
//! # The table rules (C5, verbatim-binding)
//!
//! - **Compile-time routing, no runtime fallback.** [`route`] is an
//!   exhaustive match with **no wildcard arms**: adding a
//!   [`SurfaceKind`] breaks the build (D3), and every arm is a
//!   documented decision into C1's rung 1 (closed form), rung 2
//!   (conic), or rung 3 (march + fit). "Try closed-form, else march"
//!   is a silent semantic downgrade and does not exist here: an
//!   unimplemented arm refuses **typed**, naming its routing.
//! - **Within-pair degeneracy trileans run BEFORE any rung**: each
//!   implemented arm's section function classifies its configuration
//!   invariants through named Q1 predicates (axis parallelism at
//!   derived angular thresholds with named lever arms, center/axis
//!   distances vs radii, all K-funnel registered) — definitely-generic
//!   ⇒ the arm's rung; exactly-degenerate ⇒ the degenerate closed
//!   form; in-band ⇒ the F6 escalated typed error
//!   ([`SectionError::Escalated`] — an ill-conditioned operand pair at
//!   this ε), whose Display composes the shared two-tolerance recourse
//!   through [`geom_core::Indeterminate`]'s own Display.
//! - **The M2 pairs enter unchanged**: plane×plane stays the existing
//!   splitting/boolean seam (rung 1, implemented — the table names it,
//!   the pipelines execute it bit-identically); plane×cylinder's rim
//!   case stays the rung-1 `Circle`.
//! - **R1 is permanent until a PR moves it**: plane×cone generic tilt
//!   routes to rung 3 *explicitly and permanently* — the conic trio
//!   (parabola/hyperbola) does NOT land in M5, so the arm's generic
//!   verdict is [`SectionError::RoutesToGeneralRung`], a documented
//!   decision, not a TODO.
//!
//! # What executes in this PR (spec §3)
//!
//! 1. [`plane_cylinder_section`] — tilted ⇒ exact `Ellipse`
//!    (zero-residual-by-construction: every constructed point satisfies
//!    both implicit forms exactly in ℝ); axis ∥ normal ⇒ the M2 rim
//!    `Circle`; axis in-plane ⇒ line pair / tangent line / empty.
//! 2. [`cylinder_cylinder_section`] — equal radii (**structural or
//!    declared ONLY, never inferred from values** — the caller passes
//!    [`RadiusEvidence`] resolved through the coincidence ladder; the
//!    declaration is then *verified*, D5-style) with intersecting axes
//!    ⇒ two `Ellipse` carriers in the two axis-bisector planes;
//!    parallel axes ⇒ line pair / tangent line / empty; skew or
//!    undeclared ⇒ typed rung-3 refusal.
//! 3. [`plane_cone_section`] — exact-degenerate cases only (R1):
//!    apex-through plane (two generator lines / tangent line / apex
//!    point), axis-normal cut (`Circle`); generic tilt refuses typed as
//!    permanently routed to rung 3.
//!
//! Tangential outcomes (`TangentLine` variants) are **classification
//! data**, not constructible edges: a pair whose transversality margin
//! dies along the locus is C7 (`TangentIntersection`) territory — M5
//! PR 9 builds those; consumers here must refuse to construct from
//! them (the split/boolean lanes do, typed).

use geom_core::{Band, Indeterminate, Point3, Real, Sign, Vec3};
use geom_curves::{Curve3, EllipseInvalid};
use geom_surfaces::Surface;

use crate::dihedral::decide;
use geom_core::Decide;

// ---------------------------------------------------------------------
// Kinds, rungs, routing
// ---------------------------------------------------------------------

/// The closed kind tag of a [`Surface`] variant — the table's index
/// set. Mirrors the enum exactly (D3: closed, compiler-enumerated).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceKind {
    /// [`Surface::Plane`].
    Plane,
    /// [`Surface::Cylinder`].
    Cylinder,
    /// [`Surface::Cone`].
    Cone,
    /// [`Surface::Sphere`].
    Sphere,
    /// [`Surface::Torus`].
    Torus,
    /// [`Surface::Nurbs`] — the universal fallback kind.
    Nurbs,
}

impl SurfaceKind {
    /// The kind of a surface value.
    pub fn of<T: Real>(s: &Surface<T>) -> Self {
        match s {
            Surface::Plane { .. } => Self::Plane,
            Surface::Cylinder { .. } => Self::Cylinder,
            Surface::Cone { .. } => Self::Cone,
            Surface::Sphere { .. } => Self::Sphere,
            Surface::Torus { .. } => Self::Torus,
            Surface::Nurbs(_) => Self::Nurbs,
        }
    }

    /// The kind's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Plane => "plane",
            Self::Cylinder => "cylinder",
            Self::Cone => "cone",
            Self::Sphere => "sphere",
            Self::Torus => "torus",
            Self::Nurbs => "nurbs",
        }
    }
}

/// C1's three-rung intersection-locus ladder — where a pair's locus
/// representation lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rung {
    /// Rung 1: closed-form `Line`/`Circle` carriers (the M2 pairs).
    Closed,
    /// Rung 2: exact conic carriers (`Ellipse`, M5 PR 5).
    Conic,
    /// Rung 3: march + fit (SSI, M5 PR 7) — the general rung.
    General,
}

impl Rung {
    /// The rung's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Closed => "rung 1 (closed form)",
            Self::Conic => "rung 2 (exact conic)",
            Self::General => "rung 3 (march + fit / SSI)",
        }
    }
}

/// One arm of the table: the pair's **documented routing decision**.
/// `implemented` is per-arm retirement state (C12.1: the curved-boolean
/// refusal retires arm by arm, never wholesale); an unimplemented arm's
/// consumers refuse typed with [`PairRoute::refusal`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PairRoute {
    /// The rung this pair's locus is routed to.
    pub rung: Rung,
    /// Whether this build executes the arm (closed forms exist and are
    /// wired). `false` arms refuse typed — never a runtime fallback.
    pub implemented: bool,
    /// The arm's decision note (cited by refusals).
    pub note: &'static str,
}

impl PairRoute {
    /// The typed-refusal sentence for an unimplemented arm — names the
    /// routing per C5 ("this pair routes to …, unimplemented until …").
    pub fn refusal(&self, a: SurfaceKind, b: SurfaceKind) -> String {
        format!(
            "{}×{} routes to {} — {}",
            a.name(),
            b.name(),
            self.rung.name(),
            self.note
        )
    }
}

/// **THE table** (C5): every unordered kind pair's routing, written as
/// the exhaustive ordered match — no wildcard arm anywhere, so adding a
/// `SurfaceKind` breaks this build at compile time (D3). Symmetric: the
/// two orders of a pair share one arm via explicit `|` alternation.
pub fn route(a: SurfaceKind, b: SurfaceKind) -> PairRoute {
    use SurfaceKind::{Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    match (a, b) {
        // ---- Rung 1, implemented: the M2 pair, executed by the
        // existing splitting/boolean seam bit-identically. ----
        (Plane, Plane) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "the M2/M3 planar seam (lines; splitting + boolean execute it)",
        },
        // ---- Rung 2, implemented HERE (M5 PR 5 §3.1): tilted ⇒
        // Ellipse; the rim case stays the rung-1 Circle; axis-parallel
        // degenerates are closed forms. ----
        (Plane, Cylinder) | (Cylinder, Plane) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "tilted cut is the exact Ellipse (plane_cylinder_section); the \
                   perpendicular cut stays the rung-1 rim Circle",
        },
        // ---- Rung 2, exact-degenerates only (R1, PERMANENT): generic
        // tilt routes to rung 3 until a future PR adds the conic trio.
        // The routing itself is the decision — not a TODO. ----
        (Plane, Cone) | (Cone, Plane) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "exact-degenerate cases only (apex-through lines/tangent/point, \
                   axis-normal Circle); generic tilt routes to the general rung \
                   PERMANENTLY (R1: parabola/hyperbola do not land in M5), \
                   unimplemented until SSI (M5 PR 7)",
        },
        // ---- Rung 1 routing, unimplemented: the closed form (a
        // circle) exists but no PR has landed it; refuses typed — the
        // general rung is not a fallback (C5). ----
        (Plane, Sphere) | (Sphere, Plane) => PairRoute {
            rung: Rung::Closed,
            implemented: false,
            note: "a plane×sphere cut is a closed-form Circle; unimplemented in this \
                   build — refuses typed, no runtime fallback (C5)",
        },
        (Sphere, Sphere) => PairRoute {
            rung: Rung::Closed,
            implemented: false,
            note: "distinct spheres cut in a closed-form Circle; unimplemented in \
                   this build — refuses typed, no runtime fallback (C5)",
        },
        // ---- Rung 2, implemented HERE (M5 PR 5 §3.2) for the
        // equal-radius intersecting-axes configuration; everything else
        // in the pair routes to rung 3. ----
        (Cylinder, Cylinder) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "equal radii (structural/declared ONLY — never inferred from \
                   values) with intersecting axes split into two Ellipses \
                   (cylinder_cylinder_section); unequal, undeclared, or skew routes \
                   to the general rung, unimplemented until SSI (M5 PR 7)",
        },
        // ---- Rung 3: quartic-and-worse loci; the general rung is not
        // ready until PR 7, so these refuse typed naming the routing.
        (Plane, Torus) | (Torus, Plane) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "a plane×torus section is quartic (special Villarceau/profile \
                   circles are not classified here); this pair routes to the \
                   general rung, unimplemented until SSI (M5 PR 7)",
        },
        (Cylinder, Cone) | (Cone, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7)",
        },
        (Cylinder, Sphere) | (Sphere, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7); the coaxial circle special case is not classified \
                   here",
        },
        (Cylinder, Torus) | (Torus, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7)",
        },
        (Cone, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7); the common-apex line-pair special case is not \
                   classified here",
        },
        (Cone, Sphere) | (Sphere, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7)",
        },
        (Cone, Torus) | (Torus, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7)",
        },
        (Sphere, Torus) | (Torus, Sphere) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7)",
        },
        (Torus, Torus) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung, unimplemented until SSI \
                   (M5 PR 7)",
        },
        // ---- Nurbs × anything: the universal general-rung route. ----
        (Nurbs, Plane | Cylinder | Cone | Sphere | Torus | Nurbs)
        | (Plane | Cylinder | Cone | Sphere | Torus, Nurbs) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "NURBS pairs route to the general rung, unimplemented until SSI \
                   (M5 PR 7)",
        },
    }
}

// ---------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------

/// Typed refusal of a section function — D4 ¶3: actionable, closed.
#[derive(Clone, Debug, PartialEq)]
pub enum SectionError {
    /// The caller routed a pair to the wrong arm (kind mismatch) — a
    /// caller bug, loudly typed rather than assumed away.
    WrongLane {
        /// What the arm expected.
        expected: &'static str,
    },
    /// A within-pair degeneracy trilean landed in the ambiguity band
    /// or poisoned (F6): the operand pair is ill-conditioned at this ε.
    Escalated(Indeterminate),
    /// The configuration routes to the general rung — a documented arm
    /// decision (no runtime fallback exists; C5): unimplemented until
    /// SSI (M5 PR 7), or permanently routed (plane×cone generic tilt,
    /// R1).
    RoutesToGeneralRung {
        /// The pair, for the message.
        pair: &'static str,
        /// The routing grounds.
        why: &'static str,
    },
    /// The declared radius equality is contradicted by the geometry
    /// (|r₁ − r₂| definitely nonzero): declarations are verified, never
    /// trusted (the M3 verified-at-use posture).
    RadiusDeclarationContradicted,
    /// The two surfaces are coincident (coaxial equal-radius cylinders):
    /// a same-surface locus is a coincidence to declare/merge, never an
    /// intersection curve.
    CoincidentSurfaces,
    /// The conic carrier constructor refused (near-circular tilt or a
    /// degenerate axis) — the constructor is the one deciding door for
    /// axis ordering (spec §1) and its verdict stands.
    Carrier(EllipseInvalid),
}

impl From<EllipseInvalid> for SectionError {
    fn from(e: EllipseInvalid) -> Self {
        Self::Carrier(e)
    }
}

impl core::fmt::Display for SectionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongLane { expected } => {
                write!(
                    f,
                    "section: wrong dispatch lane — this arm classifies {expected} (caller bug)"
                )
            }
            // The Indeterminate Display carries the shared two-tolerance
            // recourse (S6) exactly once.
            Self::Escalated(diag) => write!(
                f,
                "section: configuration trilean escalated — an ill-conditioned \
                 operand pair at this tolerance (F6): {diag}"
            ),
            Self::RoutesToGeneralRung { pair, why } => {
                write!(f, "section: {pair}: {why}")
            }
            Self::RadiusDeclarationContradicted => write!(
                f,
                "section: the declared equal-radius coincidence is contradicted by the \
                 geometry (|r1 - r2| definitely nonzero) — declarations are verified at \
                 use, never trusted"
            ),
            Self::CoincidentSurfaces => write!(
                f,
                "section: the surfaces are coincident (coaxial equal-radius cylinders) — \
                 a same-surface locus is not an intersection; {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::Carrier(e) => write!(f, "section: {e}"),
        }
    }
}

impl std::error::Error for SectionError {}

// ---------------------------------------------------------------------
// plane × cylinder (spec §3.1)
// ---------------------------------------------------------------------

/// The classified plane×cylinder section — every configuration's
/// closed form (the trileans run before any rung, C5).
#[derive(Clone, Debug)]
pub enum PlaneCylinderSection<T: Real> {
    /// Generic tilt: the exact `Ellipse` (rung 2) — semi-major
    /// `r/|cos φ|` along the axis' in-plane shadow, semi-minor `r`
    /// along `axis × normal`, centered where the axis pierces the
    /// plane. Zero-residual-by-construction (both implicit forms
    /// vanish identically in ℝ on the constructed locus).
    TiltedEllipse(Curve3<T>),
    /// Axis ∥ normal: the rung-1 rim `Circle` (the M2 case, unchanged —
    /// carrier axis is the CYLINDER axis, u_ref the cylinder's seam).
    Rim(Curve3<T>),
    /// Axis in-plane, axis-to-plane gap definitely < r: two rulings.
    ParallelLines {
        /// The ruling on the `+axis×normal`-ish side.
        l1: Curve3<T>,
        /// The other ruling.
        l2: Curve3<T>,
    },
    /// Axis in-plane, gap coincident with r: the tangency ruling —
    /// **classification data, not a constructible edge** (C7: tangent
    /// loci are `TangentIntersection` territory, M5 PR 9; consumers
    /// refuse typed).
    TangentLine(Curve3<T>),
    /// Axis in-plane, gap definitely > r: no intersection.
    Empty,
}

/// Classifies and constructs the plane×cylinder section (spec §3.1).
///
/// Trileans, in order (named lever arms per D4 ¶1):
///
/// 1. `pc_axis_plane_parallel` — margin `(axis·normal)·extent` (the
///    axis' angle off the plane, metered at the operand extent
///    `extent`): Zero ⇒ the axis lies in the plane (the parallel
///    degenerate lane, step 2); definite ⇒ a bounded cut (step 3).
/// 2. `pc_parallel_gap` — margin `r − |signed axis-to-plane gap|`
///    (meters): Positive ⇒ [`PlaneCylinderSection::ParallelLines`],
///    Zero ⇒ [`PlaneCylinderSection::TangentLine`], Negative ⇒
///    [`PlaneCylinderSection::Empty`].
/// 3. `pc_rim_alignment` — margin `‖axis×normal‖·r` (the tilt angle's
///    sine, metered at the rim radius): Zero ⇒
///    [`PlaneCylinderSection::Rim`]; definite ⇒
///    [`PlaneCylinderSection::TiltedEllipse`] through the ellipse
///    constructor (whose own `ellipse_axes_distinct` gate is the final
///    word on near-circular tilts — the documented double gate: the
///    constructor is the one deciding door for the kind).
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, in-band escalations (F6), or a
/// carrier-constructor refusal.
pub fn plane_cylinder_section<T: Decide>(
    plane: &Surface<T>,
    cylinder: &Surface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneCylinderSection<T>, SectionError> {
    let &Surface::Plane {
        origin: q,
        normal: n,
        ..
    } = plane
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cylinder",
        });
    };
    let &Surface::Cylinder {
        origin: o,
        axis: a,
        radius: r,
        u_ref: cyl_u,
    } = cylinder
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cylinder",
        });
    };

    let c = a.dot(n);
    match decide("pc_axis_plane_parallel", c * extent, band).map_err(SectionError::Escalated)? {
        Sign::Zero => {
            // The axis lies in the plane: line pair / tangent / empty
            // by the axis-to-plane gap vs the radius.
            let gap_signed = (o - q).dot(n);
            let margin = r - gap_signed.abs();
            match decide("pc_parallel_gap", margin, band).map_err(SectionError::Escalated)? {
                Sign::Positive => {
                    // Cross-section chord: the plane cuts the circle at
                    // foot ± w·half, foot the axis' plane projection.
                    let foot = o - n * gap_signed;
                    let half = (r * r - gap_signed * gap_signed).sqrt();
                    let w = a.cross(n).normalize();
                    Ok(PlaneCylinderSection::ParallelLines {
                        l1: Curve3::Line {
                            origin: foot + w * half,
                            dir: a,
                        },
                        l2: Curve3::Line {
                            origin: foot - w * half,
                            dir: a,
                        },
                    })
                }
                Sign::Zero => Ok(PlaneCylinderSection::TangentLine(Curve3::Line {
                    origin: o - n * gap_signed,
                    dir: a,
                })),
                Sign::Negative => Ok(PlaneCylinderSection::Empty),
            }
        }
        Sign::Positive | Sign::Negative => {
            // Bounded cut: rim circle vs tilted ellipse.
            let sin_vec = a.cross(n);
            let sin_norm = sin_vec.norm();
            let t_star = (q - o).dot(n) / c;
            let center = o + a * t_star;
            match decide("pc_rim_alignment", sin_norm * r, band).map_err(SectionError::Escalated)? {
                Sign::Zero => Ok(PlaneCylinderSection::Rim(Curve3::Circle {
                    center,
                    axis: a,
                    radius: r,
                    u_ref: cyl_u,
                })),
                // The margin is a norm: Negative is unreachable; both
                // definite verdicts take the tilted lane.
                Sign::Positive | Sign::Negative => {
                    let v_minor = sin_vec / sin_norm;
                    let u_major = v_minor.cross(n);
                    let e = Curve3::ellipse(center, n, r / c.abs(), r, u_major, band)?;
                    Ok(PlaneCylinderSection::TiltedEllipse(e))
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// cylinder × cylinder, equal radii (spec §3.2)
// ---------------------------------------------------------------------

/// The coincidence-ladder evidence for radius equality (C5: structural
/// or declared ONLY — **never inferred from values**). The caller —
/// who owns provenance/declaration data — resolves the ladder; this
/// module only consumes its verdict, then *verifies* a declaration
/// against the geometry (declared ≠ unchecked).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadiusEvidence {
    /// Radius equality is structural or declared through the ladder.
    Declared,
    /// No ladder evidence: the pair routes to the general rung even if
    /// the radius VALUES happen to coincide (the never-infer rule).
    None,
}

/// The classified equal-radius cylinder×cylinder section.
#[derive(Clone, Debug)]
pub enum EqualCylinderSection<T: Real> {
    /// Intersecting axes: the two exact `Ellipse` carriers, one in each
    /// axis-bisector plane through the axes' intersection point
    /// (normals ∝ `a1 − a2` and `a1 + a2`). Each is exactly the tilted
    /// plane×cylinder ellipse of cylinder 1 with that bisector plane —
    /// on cylinder 2 by the mirror symmetry, so the residual is
    /// identically zero in ℝ against both.
    TwoEllipses {
        /// The ellipse in the `a1 − a2`-normal bisector plane.
        e1: Curve3<T>,
        /// The ellipse in the `a1 + a2`-normal bisector plane.
        e2: Curve3<T>,
    },
    /// Parallel axes, gap definitely < 2r: two parallel rulings.
    ParallelLines {
        /// One ruling.
        l1: Curve3<T>,
        /// The other.
        l2: Curve3<T>,
    },
    /// Parallel axes, gap coincident with 2r: the tangency ruling —
    /// classification data, not a constructible edge (C7 / M5 PR 9).
    TangentLine(Curve3<T>),
    /// Parallel axes, gap definitely > 2r: no intersection.
    Empty,
}

/// Classifies and constructs the equal-radius cylinder×cylinder
/// section (spec §3.2).
///
/// Trileans, in order:
///
/// 1. [`RadiusEvidence`] gate — **structural, not numeric**: without
///    ladder evidence the pair routes to the general rung
///    ([`SectionError::RoutesToGeneralRung`]), radius values never
///    consulted.
/// 2. `cc_declared_radius_equality` — margin `r₁ − r₂` (meters):
///    the declaration is verified — Zero required; definite ⇒
///    [`SectionError::RadiusDeclarationContradicted`]; in-band ⇒
///    escalated.
/// 3. `cc_axes_parallel` — margin `‖a1×a2‖·extent`: Zero ⇒ the
///    parallel lane (step 4); definite ⇒ the crossing lane (step 5).
/// 4. `cc_coaxial` / `cc_parallel_gap` — axis-to-axis distance `d`:
///    coincident-with-zero ⇒ [`SectionError::CoincidentSurfaces`];
///    then margin `2r − d`: Positive ⇒ two rulings, Zero ⇒ tangent
///    ruling, Negative ⇒ empty.
/// 5. `cc_axes_coplanar` — margin the signed axis-to-axis gap
///    `(o2−o1)·(a1×a2)/‖a1×a2‖` (meters): Zero ⇒ intersecting axes ⇒
///    the two bisector-plane ellipses; definite ⇒ skew ⇒ typed rung-3
///    refusal.
///
/// # Errors
///
/// [`SectionError`] — see the trilean list.
pub fn cylinder_cylinder_section<T: Decide>(
    c1: &Surface<T>,
    c2: &Surface<T>,
    evidence: RadiusEvidence,
    extent: T,
    band: Band,
) -> Result<EqualCylinderSection<T>, SectionError> {
    let &Surface::Cylinder {
        origin: o1,
        axis: a1,
        radius: r1,
        ..
    } = c1
    else {
        return Err(SectionError::WrongLane {
            expected: "cylinder×cylinder",
        });
    };
    let &Surface::Cylinder {
        origin: o2,
        axis: a2,
        radius: r2,
        ..
    } = c2
    else {
        return Err(SectionError::WrongLane {
            expected: "cylinder×cylinder",
        });
    };

    // 1. The ladder gate: never inferred from values.
    if evidence == RadiusEvidence::None {
        return Err(SectionError::RoutesToGeneralRung {
            pair: "cylinder×cylinder",
            why: "radius equality is not structural/declared — never inferred from \
                  values (the coincidence ladder); the undeclared pair routes to the \
                  general rung, unimplemented until SSI (M5 PR 7)",
        });
    }
    // 2. Verify the declaration (declared ≠ unchecked).
    match decide("cc_declared_radius_equality", r1 - r2, band).map_err(SectionError::Escalated)? {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(SectionError::RadiusDeclarationContradicted);
        }
    }

    let cross = a1.cross(a2);
    let cross_norm = cross.norm();
    match decide("cc_axes_parallel", cross_norm * extent, band).map_err(SectionError::Escalated)? {
        Sign::Zero => {
            // Parallel axes: the cross-section is two equal circles at
            // center distance d.
            let w0 = o2 - o1;
            let d_vec = w0 - a1 * w0.dot(a1);
            let d = d_vec.norm();
            match decide("cc_coaxial", d, band).map_err(SectionError::Escalated)? {
                Sign::Zero => return Err(SectionError::CoincidentSurfaces),
                Sign::Positive | Sign::Negative => {}
            }
            let two = T::from_f64(2.0);
            match decide("cc_parallel_gap", two * r1 - d, band).map_err(SectionError::Escalated)? {
                Sign::Positive => {
                    let mid = o1 + d_vec * T::from_f64(0.5);
                    let half = (r1 * r1 - (d / two) * (d / two)).sqrt();
                    let h = a1.cross(d_vec / d);
                    Ok(EqualCylinderSection::ParallelLines {
                        l1: Curve3::Line {
                            origin: mid + h * half,
                            dir: a1,
                        },
                        l2: Curve3::Line {
                            origin: mid - h * half,
                            dir: a1,
                        },
                    })
                }
                Sign::Zero => Ok(EqualCylinderSection::TangentLine(Curve3::Line {
                    origin: o1 + d_vec * T::from_f64(0.5),
                    dir: a1,
                })),
                Sign::Negative => Ok(EqualCylinderSection::Empty),
            }
        }
        Sign::Positive | Sign::Negative => {
            // Crossing lane: coplanarity (intersecting vs skew).
            let w0 = o2 - o1;
            let gap = w0.dot(cross) / cross_norm;
            match decide("cc_axes_coplanar", gap, band).map_err(SectionError::Escalated)? {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => {
                    return Err(SectionError::RoutesToGeneralRung {
                        pair: "cylinder×cylinder",
                        why: "skew axes have no conic section; this configuration routes \
                              to the general rung, unimplemented until SSI (M5 PR 7)",
                    });
                }
            }
            // The axes' intersection point (closest point on axis 1;
            // the coplanarity verdict bounds the residual by ε).
            let t1 = w0.cross(a2).dot(cross) / (cross_norm * cross_norm);
            let p = o1 + a1 * t1;
            // The two bisector planes through p. Each ellipse is the
            // tilted plane×cylinder cut of cylinder 1 (by symmetry it
            // lies on cylinder 2 as well).
            let n1 = (a1 - a2).normalize();
            let n2 = (a1 + a2).normalize();
            let mk = |n: Vec3<T>| -> Result<Curve3<T>, SectionError> {
                let c = a1.dot(n);
                let sin_vec = a1.cross(n);
                let v_minor = sin_vec.normalize();
                let u_major = v_minor.cross(n);
                Ok(Curve3::ellipse(p, n, r1 / c.abs(), r1, u_major, band)?)
            };
            Ok(EqualCylinderSection::TwoEllipses {
                e1: mk(n1)?,
                e2: mk(n2)?,
            })
        }
    }
}

// ---------------------------------------------------------------------
// plane × cone, exact-degenerates only (spec §3.3, R1)
// ---------------------------------------------------------------------

/// The classified plane×cone exact-degenerate section. Generic tilt is
/// deliberately NOT a variant: it refuses typed
/// ([`SectionError::RoutesToGeneralRung`]) — R1's permanent routing.
#[derive(Clone, Debug)]
pub enum PlaneConeSection<T: Real> {
    /// Apex on the plane, plane cutting inside the cone: two generator
    /// lines through the apex.
    ApexLinePair {
        /// The generator at azimuth `φ + δ`.
        l1: Curve3<T>,
        /// The generator at azimuth `φ − δ`.
        l2: Curve3<T>,
    },
    /// Apex on the plane, plane tangent along one generator —
    /// classification data, not a constructible edge (C7 / M5 PR 9).
    ApexTangentLine(Curve3<T>),
    /// Apex on the plane, no other contact: the apex point alone.
    ApexPoint(Point3<T>),
    /// Axis ∥ normal, apex off the plane: the rung-1 `Circle` cut.
    AxisNormalCircle(Curve3<T>),
}

/// Classifies and constructs the plane×cone exact-degenerate sections
/// (spec §3.3). Generic tilt refuses typed — **permanently routed to
/// rung 3** (R1: the conic trio does not land in M5; a future PR that
/// adds parabola/hyperbola moves the arm, nothing else does).
///
/// Trileans, in order:
///
/// 1. `pn_apex_on_plane` — margin `(apex − q)·normal` (meters): Zero ⇒
///    the apex lane (step 2); definite ⇒ step 3.
/// 2. `pn_apex_section` — margin `sin α·‖axis×normal‖ −
///    cos α·|axis·normal|` metered at `extent` (the two-generator
///    discriminant: positive exactly when the plane dips inside the
///    cone): Positive ⇒ [`PlaneConeSection::ApexLinePair`], Zero ⇒
///    [`PlaneConeSection::ApexTangentLine`], Negative ⇒
///    [`PlaneConeSection::ApexPoint`].
/// 3. `pn_axis_normal` — margin `‖axis×normal‖·arm`, arm the would-be
///    circle radius `|h|·tan α` (h the apex-to-plane distance along
///    the axis): Zero ⇒ [`PlaneConeSection::AxisNormalCircle`];
///    definite ⇒ the R1 refusal.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, escalations (F6), or the R1
/// generic-tilt routing refusal.
pub fn plane_cone_section<T: Decide>(
    plane: &Surface<T>,
    cone: &Surface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneConeSection<T>, SectionError> {
    let &Surface::Plane {
        origin: q,
        normal: n,
        ..
    } = plane
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cone",
        });
    };
    let &Surface::Cone {
        apex,
        axis: a,
        half_angle,
        u_ref: cone_u,
    } = cone
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cone",
        });
    };

    let (sin_a, cos_a) = half_angle.sin_cos();
    let c = a.dot(n);
    let s_vec = a.cross(n);
    let s = s_vec.norm();

    let apex_gap = (apex - q).dot(n);
    match decide("pn_apex_on_plane", apex_gap, band).map_err(SectionError::Escalated)? {
        Sign::Zero => {
            // Apex lane: generators g(u) = a·cosα + radial(u)·sinα with
            // g·n = 0 ⇔ cos(u − φ) = −cosα·c / (sinα·s).
            let discr = sin_a * s - cos_a * c.abs();
            let verdict =
                decide("pn_apex_section", discr * extent, band).map_err(SectionError::Escalated)?;
            match verdict {
                Sign::Positive | Sign::Zero => {
                    let v_ref = a.cross(cone_u);
                    let (nu, nv) = (cone_u.dot(n), v_ref.dot(n));
                    let phi = nv.atan2(nu);
                    // Clamped acos argument (outward-rounding can push
                    // it a hair past ±1 at the tangency boundary —
                    // min/max are Real lattice ops, evaluation-legal).
                    let arg = ((T::zero() - cos_a * c) / (sin_a * s))
                        .min(T::one())
                        .max(T::zero() - T::one());
                    let delta = arg.acos();
                    let gen_at = |u: T| -> Curve3<T> {
                        let (su, cu) = u.sin_cos();
                        Curve3::Line {
                            origin: apex,
                            dir: a * cos_a + (cone_u * cu + v_ref * su) * sin_a,
                        }
                    };
                    if verdict == Sign::Zero {
                        Ok(PlaneConeSection::ApexTangentLine(gen_at(phi + delta)))
                    } else {
                        Ok(PlaneConeSection::ApexLinePair {
                            l1: gen_at(phi + delta),
                            l2: gen_at(phi - delta),
                        })
                    }
                }
                Sign::Negative => Ok(PlaneConeSection::ApexPoint(apex)),
            }
        }
        Sign::Positive | Sign::Negative => {
            // Apex definitely off the plane: axis-normal circle or the
            // R1 permanent routing.
            let h = (q - apex).dot(a);
            let rim_r = h.abs() * (sin_a / cos_a);
            match decide("pn_axis_normal", s * rim_r, band).map_err(SectionError::Escalated)? {
                Sign::Zero => Ok(PlaneConeSection::AxisNormalCircle(Curve3::Circle {
                    center: apex + a * h,
                    axis: a,
                    radius: rim_r,
                    u_ref: cone_u,
                })),
                Sign::Positive | Sign::Negative => Err(SectionError::RoutesToGeneralRung {
                    pair: "plane×cone",
                    why: "generic tilt routes to the general rung PERMANENTLY (R1: the \
                          conic trio does not land in M5 — a documented arm decision, \
                          moved only by a future PR that adds parabola/hyperbola); \
                          unimplemented until SSI (M5 PR 7)",
                }),
            }
        }
    }
}

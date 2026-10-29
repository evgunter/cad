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
//! - **A division by a cone's aperture is decided first.** Every arm
//!   that divides by `sin α` or `cos α` — or by a product carrying one
//!   — decides that clause of the cone's convention `α ∈ (0, π/2)` as
//!   a named trilean metered at the arm's `extent` before the lane that
//!   divides runs, and refuses [`SectionError::DegenerateOperand`] when
//!   it is not definitely positive (`pn_aperture_*`, `coc_aperture_*`).
//!   This is the file's BAND posture on an operand convention — the one
//!   `torus_tube_positive` takes on the torus — and it is a posture, not a
//!   numerical necessity: `sin α` of a stored `α` is relatively exact,
//!   and a quotient by it is the correctly rounded value for the datum
//!   as stored. A body at rest holds the convention against ZERO (tier-3
//!   check 1), so a cone that validates can be refused here. Whether the
//!   convention is a band question or a datum-sign question is open
//!   (`work/germcert/the-tube-and-radius-guards-decide-on-the-band-where-check-1-reads-lo.md`),
//!   and these trileans follow the band side until that is ruled.
//! - **The M2 pairs enter unchanged**: plane×plane stays the existing
//!   splitting/boolean seam (rung 1, implemented — the table names it,
//!   the pipelines execute it bit-identically); plane×cylinder's rim
//!   case stays the rung-1 `Circle`.
//! - **A plane×cone parabola or hyperbola has no curve kind** (C1): the
//!   section is served in closed form ([`OpenConicSection`]) and each
//!   arc of it is minted per window as an exact rational-quadratic
//!   `Curve3::Nurbs` ([`ConicBranch::arc`]), never marched.
//!
//! # The section arms
//!
//! Every arm classifies its configuration through named trileans
//! first, mints the closed form only for the configurations it names,
//! and refuses typed for the rest. Each is
//! zero-residual-by-construction where it mints: every constructed
//! point satisfies both implicit forms exactly in ℝ.
//!
//! 1. [`plane_cylinder_section`] — tilted ⇒ exact `Ellipse`;
//!    axis ∥ normal ⇒ the M2 rim `Circle`; axis in-plane ⇒ line pair /
//!    tangent line / empty.
//! 2. [`plane_sphere_section`] — the `Circle`; the tangency is a POINT,
//!    classification data refused as a carrier.
//! 3. [`plane_cone_section`] — apex-through plane (two generator
//!    lines / tangent line / apex point); axis-normal cut (`Circle`);
//!    a tilt meeting every generator ⇒ exact `Ellipse`; a parabolic or
//!    hyperbolic tilt ⇒ the open conic in closed form, minted per window
//!    as an exact rational-quadratic `Nurbs` arc.
//! 4. [`plane_torus_section`] — the axis-aligned poses: an
//!    axis-CONTAINING plane's two meridian `Circle`s, an axis-PARALLEL
//!    plane off the axis its two `Spiric` ovals (short of the inner
//!    equator), an axis-NORMAL plane's two concentric circles (or the
//!    tangency circle, as classification data); every tilt — the
//!    Villarceau bitangent included — refuses typed.
//! 5. [`cylinder_cylinder_section`] — equal radii, decided by their
//!    margin (D10: a margin decided Zero is a coincidence, recorded by
//!    the caller), with intersecting axes ⇒ two `Ellipse` carriers in
//!    the two axis-bisector planes; parallel axes ⇒ line pair / tangent
//!    line / empty; skew axes or unequal radii ⇒ typed rung-3 refusal.
//! 6. cylinder × sphere — built by the section value
//!    ([`crate::pair_section()`]) over its pose: the coaxial pose, its
//!    axis-to-centre distance decided Zero by its margin, is two
//!    circles, the tangent circle as classification data, or empty.
//! 7. [`sphere_sphere_section`] — the radical-plane `Circle`; either
//!    tangency is a POINT, and one sphere given twice is a coincidence
//!    to declare rather than a section.
//! 8. [`cone_cylinder_section`] — the COAXIAL pose only: one `Circle`
//!    per nappe at `±R·cot α`, admitted by the station's own reach;
//!    tilted and parallel-but-offset refuse typed.
//!
//! # Rung 3
//!
//! These arms retired their refusals — per-arm, never wholesale (C12.1),
//! each with its trace shape as a **compile-time** decision documented
//! at the arm (C5: no runtime fallback, so an arm's shape is not
//! something a caller can influence):
//!
//! - **plane × NURBS** — the ℝ⁴ parametric×parametric trace (3×4 SVD).
//! - **cylinder × sphere** and the three torus pairs (**sphere × torus**,
//!   **cylinder × torus**, **torus × torus**) — the ℝ³ implicit-pair
//!   march (2×3 SVD), each past its pose's own tangency decision.
//!
//! Each goes through `geom_brep::ssi`, which marches (untrusted), fits
//! (PR 4), certifies all three C2 limbs, and proves the domain
//! exhausted or refuses typed. Every other rung-3 arm keeps its typed
//! refusal and **cites the trace shape it would use** and what is
//! actually missing: for the cone pairs, the meters conversion of the
//! cone's composite.
//!
//! Tangential outcomes (`TangentLine` variants) are **classification
//! data**, not constructible edges: a pair whose transversality margin
//! dies along the locus is C7 (`TangentIntersection`) territory — M5
//! PR 9 builds those; consumers here must refuse to construct from
//! them (the split/boolean lanes do, typed).

use geom::Surface;
use geom::{Curve3, EllipseInvalid, SpiricInvalid, SquareFrame, SurfaceKind, square_frame};
use geom_core::{Band, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Vec3};

use crate::dihedral::{
    decide, decide_magnitude, decide_magnitude_reported, decide_nonzero, decide_reported,
};
use crate::extent::{LeverBounds, Reach};
use crate::recourse::{Reading, RefusedArm, RefusedSide, SizedDecision, SizedPass};
use crate::torus_convention::TorusRefusal;
use geom_core::Decide;

mod conic_arc;
mod section_bounds;
use conic_arc::ConeTilt;
pub use conic_arc::{
    ConicArcError, ConicBranch, OpenConic, OpenConicSection, SPAN_WEIGHT_MAX, WindowEnd,
};
use geom_core::k_stats::{Magnitude, NonzeroSign};
pub use section_bounds::{SectionBounds, Unstated, tube_bounds};

// ---------------------------------------------------------------------
// Kinds, rungs, routing
// ---------------------------------------------------------------------

/// C1's three-rung intersection-locus ladder — where a pair's locus
/// representation lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rung {
    /// Rung 1: closed-form `Line`/`Circle` carriers (the M2 pairs).
    Closed,
    /// Rung 2: exact conic and quartic carriers (`Ellipse`, M5 PR 5;
    /// the axis-parallel plane×torus `Spiric`, C1).
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
    /// routing per C5 ("this pair routes to …") and what that arm is
    /// still missing.
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
///
/// A variant added to [`Surface`] is a `SurfaceKind` by derivation, so
/// it reaches this match as E0004 non-exhaustive-patterns; the
/// no-wildcard grep row in `tests/pcurve_conic.rs` keeps the property
/// pinned in CI.
pub fn route(a: SurfaceKind, b: SurfaceKind) -> PairRoute {
    use SurfaceKind::{Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    match (a, b) {
        // ---- Rung 1, implemented: the M2 pair, executed by the
        // existing splitting/boolean seam bit-identically. ----
        (Plane, Plane) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "the planar seam (lines; splitting + boolean execute it)",
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
        // ---- Rung 2: the apex-through degenerates, the axis-normal
        // Circle, the tilted Ellipse, and the parabola or hyperbola in
        // closed form, its arcs minted per window. ----
        (Plane, Cone) | (Cone, Plane) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "apex-through lines/tangent/point, the axis-normal Circle, the \
                   tilted Ellipse, and a parabola or hyperbola whose arcs are minted \
                   per window as exact rational-quadratic NURBS (plane_cone_section)",
        },
        // ---- Rung 1, implemented (M5 S13): the closed-form Circle —
        // never a fitted chord (the die-pips premise). ----
        (Plane, Sphere) | (Sphere, Plane) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "a plane×sphere cut is the closed-form Circle \
                   (plane_sphere_section); the tangent gap is a POINT — \
                   classification data, refused as a carrier",
        },
        (Sphere, Sphere) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "distinct spheres cut in the closed-form radical-plane Circle \
                   (sphere_sphere_section); either tangency is a POINT — \
                   classification data, refused as a carrier — and one sphere \
                   given twice is a COINCIDENCE to declare, never a section",
        },
        // ---- Rung 2, implemented HERE (M5 PR 5 §3.2) for the
        // equal-radius intersecting-axes configuration; everything else
        // in the pair routes to rung 3. ----
        (Cylinder, Cylinder) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "equal radii (decided Zero by their margin) with intersecting axes \
                   split into two Ellipses (cylinder_cylinder_section); unequal or skew \
                   routes to the general rung, whose cylinder×cylinder arm has not \
                   retired (arms retire one at a time, each with its proof)",
        },
        // ---- Rung 2, axis-aligned poses only: the containing and
        // normal planes cut closed-form Circles, the parallel one off
        // the axis the exact Spiric; every tilted configuration keeps
        // its general-rung routing, named at the arm's own refusal. ----
        (Plane, Torus) | (Torus, Plane) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "axis-aligned cases only: an axis-containing plane cuts the two \
                   meridian Circles, an axis-parallel plane off the axis the two \
                   Spiric ovals (short of the inner equator), an axis-normal plane \
                   the two concentric Circles (or the tangency circle — \
                   classification data, not a carrier; no production consumer takes \
                   it yet) (plane_torus_section); everything tilted — the bitangent \
                   Villarceau pair included — routes to the general rung with the \
                   ℝ³ IMPLICIT-PAIR trace shape, whose tilted plane×torus arm has \
                   not retired (arms retire one at a time, each with its proof)",
        },
        // ---- Rung 1, the COAXIAL configuration only: two closed-form
        // Circles, one per nappe. Every other pose keeps its
        // general-rung routing, named at the arm's own refusal. ----
        (Cylinder, Cone) | (Cone, Cylinder) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "the coaxial configuration only: a cylinder sharing the cone's axis \
                   cuts each nappe in a closed-form Circle of the CYLINDER's own \
                   radius, at ±R·cot α from the apex (cone_cylinder_section) — no \
                   tangency sub-case exists, a coaxial cylinder always cuts \
                   transversally; a tilted cylinder and a parallel-but-OFFSET one \
                   both cut a QUARTIC and route to the general rung, whose \
                   cone×cylinder arm has not retired — the cone's composite has no \
                   meters conversion, so its certificate, not its trace, is what is \
                   missing (arms retire one at a time, each with its proof)",
        },
        // ---- Rung 3, IMPLEMENTED (M5 PR 7): the ℝ³ implicit-pair
        // march. Both operands' C9 composites convert to meters
        // exactly, so all three C2 limbs certify without an invented
        // scale factor — which is why this is the pair the milestone's
        // planted small-loop fixture is built on. ----
        (Cylinder, Sphere) | (Sphere, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "built by the section value (geom_brep::pair_section) over its pose, \
                   decided once: the coaxial pose, the axis-to-centre distance decided \
                   Zero by its margin, is two closed-form circles, the tangent circle as \
                   classification data, or empty, and every transversal pose still \
                   marches in ℝ³ on the IMPLICIT PAIR (2×3 SVD, Hoffmann §6.2) and is \
                   fitted, with the full three-limb certificate and in-op exhaustiveness \
                   (geom_brep::ssi::march_cylinder_sphere)",
        },
        // ---- Rung 3: quartic-and-worse loci. The general rung is
        // implemented, but it retires per arm (C12.1), so these still
        // refuse typed, naming the routing AND what each one lacks.
        (Cone, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on the cone's exact \
                   meters conversion; the common-apex line-pair special case is \
                   not classified here",
        },
        (Cone, Sphere) | (Sphere, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on the cone's exact \
                   meters conversion",
        },
        (Cone, Torus) | (Torus, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on the cone's exact \
                   meters conversion",
        },
        // ---- Rung 3, IMPLEMENTED: the ℝ³ implicit-pair march, each
        // torus read through its quartic. ----
        (Sphere, Torus) | (Torus, Sphere) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "marched in ℝ³ on the IMPLICIT PAIR and fitted, with the full \
                   three-limb certificate and in-op exhaustiveness \
                   (geom_brep::ssi::sphere_torus_ssi), the torus read through its \
                   quartic; a torus outside its ring convention is refused as a shape",
        },
        (Cylinder, Torus) | (Torus, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "marched in ℝ³ on the IMPLICIT PAIR and fitted, with the full \
                   three-limb certificate and in-op exhaustiveness \
                   (geom_brep::ssi::cylinder_torus_ssi), the torus read through its \
                   quartic, past the pose's critical gap; a torus outside its ring \
                   convention is refused as a shape",
        },
        (Torus, Torus) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "marched in ℝ³ on the IMPLICIT PAIR and fitted, with the full \
                   three-limb certificate and in-op exhaustiveness \
                   (geom_brep::ssi::torus_torus_ssi), both tori read through their \
                   quartics, past the pose's critical gap; two tori decided one are a \
                   coincidence, and a torus outside its ring convention is refused as a \
                   shape",
        },
        // ---- Rung 3, IMPLEMENTED (M5 PR 7): the ℝ⁴ trace. The
        // plane and the wall are both charts, so the state is
        // (u₁,v₁,u₂,v₂) on G₁ − G₂ = 0 (3×4 SVD, Hoffmann §6.3.2) and
        // BOTH pcurves fall out as coordinate projections of the one
        // traced object — the shared parameter PR 6's cache contract
        // wants, which is how OQ4 discharged. ----
        // ---- Rung 3, RETIRED 2026-07-31 (M5 PR 7b): the ℝ⁴ arm's
        // last limb landed. PR 7 shipped the trace, the
        // shared-parameter fit of the carrier and BOTH pcurves, the
        // certified foot points, the chart uniqueness tube and the
        // UV-domain exhaustiveness, and refused at C2.2's
        // between-samples sup bound against the NURBS operand (the
        // per-span first-order enclosure was sound but scaled like
        // the span width). PR 7b landed the tensor-product Bernstein
        // composition (`geom_core::spline::compose::tensor`): the
        // residual S(P(t)) − C(t) is enclosed as ONE composite so the
        // cancellation survives, limb 2 flipped to the tight bound,
        // and the arm retired by deleting nothing (C12.1: per-arm,
        // WITH its proof). ----
        (Plane, Nurbs) | (Nurbs, Plane) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "traced in ℝ⁴ on the PARAMETRIC PAIR (3×4 SVD, Hoffmann §6.3.2) by \
                   geom_brep::ssi::PlaneSection, read at a seed or whole, which \
                   certifies the whole chain: \
                   the trace, the shared-parameter fit of the carrier and BOTH \
                   pcurves, the certified foot points, the chart uniqueness tube, \
                   the UV-domain exhaustiveness, and the between-samples SUP bound \
                   against the NURBS operand. That last bound is the \
                   tensor-product Bernstein composition of the surface with the \
                   pcurve (geom_core::spline::compose::tensor): the residual \
                   S(P(t)) − C(t) is enclosed as a SINGLE composite, so the \
                   cancellation a per-span first-order enclosure throws away \
                   survives into the bound. Practical breadth: gentle single-cell \
                   walls — an interior-knot wall refuses at the march/fit limb, \
                   and multi-cell/rational span windows hull neighbor-cell \
                   extensions into the bound or poison — loud and typed, never \
                   silent",
        },
        // ---- Rung 3, IMPLEMENTED: the ℝ⁴ trace on two charts, one
        // complete operation read at a seed or whole. ----
        (Nurbs, Nurbs) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "traced in ℝ⁴ on the PARAMETRIC PAIR (3×4 SVD) by \
                   geom_brep::ssi::Section, one complete operation over the product \
                   of the two knot rectangles: a boundary pass over every side of \
                   both rectangles, branches traced between its crossings and from \
                   the subdivision's seeds, each certified by both charts' \
                   tensor-composite sup bounds and the TWO-CHART uniqueness tube, \
                   and the accounting of every cell; a consumer reads the branch \
                   through a seed (Section::branch_at) or all of them \
                   (Section::all). A section touching or running along a side, or \
                   through a corner, of either rectangle refuses at the boundary \
                   pass — loud and typed, never silent",
        },
        // ---- Nurbs × a curved analytic: the general-rung route. ----
        (Nurbs, Cylinder | Cone | Sphere | Torus) | (Cylinder | Cone | Sphere | Torus, Nurbs) => {
            PairRoute {
                rung: Rung::General,
                implemented: false,
                note: "a NURBS operand routes to the general rung with the ℝ⁴ \
                       PARAMETRIC-PAIR trace shape; the general-rung marcher traces \
                       it and the tensor-composite sup bound that certifies \
                       plane×NURBS is the substrate these arms reuse. What is \
                       missing is the rest of the CERTIFICATE: the analytic partner \
                       needs its chart-form uniqueness tube (written for the plane) \
                       — arms retire one at a time, each with its proof",
            }
        }
        // ---- Approx: an intersection operand is its fit, as it is for
        // evaluation, boxes and pcurves. The fit's distance from its
        // description is the face's claim (O3, re-derived at rest by
        // O5), not the edge's, so nothing is composed into the section's
        // bound: each pair takes the arm its fit's kind takes, and
        // plane×Approx is the plane×NURBS arm over the fit. ----
        (Plane, Approx) | (Approx, Plane) => route(Plane, Nurbs),
        (Approx, Approx) => route(Nurbs, Nurbs),
        (Approx, other @ (Cylinder | Cone | Sphere | Torus | Nurbs))
        | (other @ (Cylinder | Cone | Sphere | Torus | Nurbs), Approx) => route(Nurbs, other),
    }
}

/// **The table asked about a POSE**, not a kind pair: [`route`]'s arm
/// for the two surfaces' kinds, with `implemented` narrowed to whether
/// THAT arm serves THESE two surfaces as they stand.
///
/// [`route`] answers per kind pair, and an implemented arm is
/// configuration-scoped: coaxial cone×cylinder, axis-containing or
/// axis-normal plane×torus, apex-through or axis-normal plane×cone,
/// equal-radius crossing or parallel cylinder×cylinder. A consumer that
/// gates on the kind pair admits every other pose of those pairs as
/// though a closed form were wired for it. This asks the arm itself —
/// the pair's own section function, which runs its configuration
/// trileans before any rung — and reads its routing verdict back:
///
/// - the arm classifies the pose (any `Ok`) ⇒ `implemented` stands;
/// - the arm refuses [`SectionError::RoutesToGeneralRung`] ⇒ the pose
///   routes to an arm that has not retired, so `implemented` is
///   `false` and `note` is the arm's own grounds;
/// - an in-band trilean ⇒ [`SectionError::Escalated`], returned: the
///   pose cannot be told from its neighbour at this ε.
///
/// **Only ROUTING is read, and only a CLASSIFIED pose is served.** An
/// arm that classifies the pose and then refuses its mint — a locus
/// past `extent`, a coincidence, a carrier the conic constructor
/// declines — has still said the pose is its own, and `implemented`
/// stands; what it would not mint is that arm's refusal to give, not
/// this question's. An arm that refuses BEFORE its pose trileans run —
/// an operand guard ([`SectionError::DegenerateOperand`]) — has
/// classified nothing, and the pose is not served: a gate must not
/// admit what it cannot classify. `note` is then the guard's own text,
/// or this function's statement where the arm's refusal carries none.
/// A torus outside its ring convention is refused as a shape, not as a
/// pose ([`SectionError::DegenerateTorus`], returned): its refusal ends
/// in the half's lever, which no routing note carries.
///
/// Every other implemented pair serves every pose: plane×plane,
/// plane×cylinder, plane×sphere, sphere×sphere, and the general-rung
/// arms that march (cylinder×sphere and sphere×torus on their decided
/// poses, plane×NURBS with plane×Approx over its fit, and NURBS×NURBS
/// with every Approx pairing over its fits). Cylinder×torus and
/// torus×torus read the section value ([`crate::pair_section()`]),
/// which serves every pose its pose decision classifies, a general one
/// through its march. Every unimplemented pair answers
/// [`route`] unchanged. The match is
/// exhaustive with no wildcard, as [`route`]'s is, so a kind added to
/// the table is a compile-time visit here too.
///
/// `reach` is what the consumer needs the pose read over: an edge's
/// span ([`Reach::Span`]), the one variant its callers hand it, levered
/// by its per-carrier farthest distance from a pivot ([`Reach::lever_from`]:
/// exact at a line's endpoints and a NURBS net's control points, at
/// most √2 over for a conic). The arms that take a scalar extent read
/// their pose at an ANCHOR (a cone's apex, a sphere's or torus's
/// centre) and get its lever from there (a tilt `θ` displaces the locus
/// by `θ·extent` there); the plane×cone arm takes the reach and levers
/// it from its apex the same way. A cylinder has no anchor:
/// its origin is any point of its axis, so a lever from it overstates
/// the reach without bound, and the cone×cylinder arm reads its pose at
/// the apex. The cylinder pair reads it from its axes' feet
/// ([`cylinder_cylinder_section`]). No lever is a ball around the edge:
/// on a two-sided trilean whose definite side is the SERVED class, a
/// lever past the consumed extent decides an in-band reading as served
/// (a near-parabola read as an ellipse).
///
/// # Errors
///
/// [`SectionError::Escalated`] — a pose trilean (or an operand guard)
/// in the band; [`SectionError::RadiusEscalated`], a radius guard in
/// the band; [`SectionError::DegenerateTorus`], a torus not shown
/// inside its ring convention. [`SectionError::WrongLane`] only if this dispatch
/// itself is wrong — it names each arm's seats in the arm's order — so
/// it is a kernel bug, returned typed rather than read as a verdict.
/// Nothing else is returned.
pub fn route_pose<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    reach: &Reach<T>,
    band: Band,
) -> Result<PairRoute, SectionError> {
    use SurfaceKind::{Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    let extent = [a, b].into_iter().fold(T::zero(), |lever, s| match *s {
        Surface::Cone { apex: anchor, .. }
        | Surface::Sphere { center: anchor, .. }
        | Surface::Torus { center: anchor, .. } => lever.max(reach.lever_from(anchor)),
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Nurbs(_)
        | Surface::Approx(_) => lever,
    });
    let (ka, kb) = (a.kind(), b.kind());
    let arm = route(ka, kb);
    let verdict = match (ka, kb) {
        (Plane, Cone) => plane_cone_section(a, b, reach, band).map(drop),
        (Cone, Plane) => plane_cone_section(b, a, reach, band).map(drop),
        (Plane, Torus) => plane_torus_section(a, b, extent, band).map(drop),
        (Torus, Plane) => plane_torus_section(b, a, extent, band).map(drop),
        (Cone, Cylinder) => cone_cylinder_section(a, b, extent, band).map(drop),
        (Sphere, Torus) | (Torus, Sphere) => torus_pose(crate::SphereTorusPose::decide(a, b, band)),
        (Cylinder, Torus) | (Torus, Cylinder) | (Torus, Torus) => {
            crate::pair_section(a, b, band).map(drop)
        }
        (Cylinder, Cone) => cone_cylinder_section(b, a, extent, band).map(drop),
        (Cylinder, Sphere) | (Sphere, Cylinder) => {
            match crate::CylinderSpherePose::decide(a, b, band) {
                Some(pose) => pose.map(drop).map_err(SectionError::from),
                None => Err(SectionError::WrongLane {
                    expected: "a cylinder and a sphere",
                }),
            }
        }
        (Cylinder, Cylinder) => match cylinder_cylinder_section(a, b, reach, band) {
            Err(SectionError::UnequalRadii) => Err(SectionError::RoutesToGeneralRung {
                pair: "cylinder×cylinder",
                why: "unequal radii are outside every closed form the equal-radius arm \
                      classifies, and route to the general rung, whose cylinder×cylinder arm \
                      has not retired",
            }),
            other => other.map(drop),
        },
        // Every pose served, by a closed form or by a general-rung arm
        // that marches.
        (Plane, Plane | Cylinder | Sphere | Nurbs | Approx)
        | (Cylinder | Sphere | Nurbs | Approx, Plane)
        | (Sphere, Sphere)
        | (Nurbs | Approx, Nurbs | Approx) => Ok(()),
        // Unimplemented at the kind level: `route`'s answer stands.
        (Cylinder, Nurbs)
        | (Torus, Cone | Nurbs)
        | (Cone, Cone | Sphere | Torus | Nurbs)
        | (Sphere, Cone | Nurbs)
        | (Nurbs, Cylinder | Cone | Sphere | Torus)
        | (Approx, Cylinder | Cone | Sphere | Torus)
        | (Cylinder | Cone | Sphere | Torus, Approx) => Ok(()),
    };
    let refused = |note| {
        Ok(PairRoute {
            implemented: false,
            note,
            ..arm
        })
    };
    match verdict {
        // Classified: the arm names the pose as its own, whatever it
        // then says about minting it.
        Ok(())
        | Err(
            SectionError::BeyondOperandExtent { .. }
            | SectionError::CoincidentSurfaces
            | SectionError::Carrier(_)
            | SectionError::Spiric(_)
            | SectionError::Pinched { .. }
            | SectionError::Unordered { .. },
        ) => Ok(arm),
        Err(SectionError::RoutesToGeneralRung { why, .. }) => refused(why),
        // Refused BEFORE the pose was classified: an operand guard the
        // arm runs ahead of its pose trileans. The pose is unknown, and
        // a gate must not admit what it cannot classify.
        Err(SectionError::DegenerateOperand { what }) => refused(what),
        Err(
            e @ (SectionError::Escalated(_)
            | SectionError::RadiusEscalated { .. }
            | SectionError::DegenerateTorus(_)
            | SectionError::WrongLane { .. }
            | SectionError::UnequalRadii),
        ) => Err(e),
    }
}

/// A torus pair's pose, decided ([`crate::pair_section::torus`]): any
/// decided pose is classified, and its refusal is the value's.
fn torus_pose<P>(decided: Option<Result<P, crate::TorusPoseRefusal>>) -> Result<(), SectionError> {
    match decided {
        Some(pose) => pose.map(drop).map_err(SectionError::from),
        None => Err(SectionError::WrongLane {
            expected: "a torus pair in the torus arm's seats",
        }),
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
    /// An operand's radius guard landed in the band or poisoned: whether
    /// that radius is positive, the question [`SectionRadius`] names, is
    /// undecided. Its decided sibling is [`Self::DegenerateOperand`].
    RadiusEscalated {
        /// Whose radius the guard read.
        radius: SectionRadius,
        /// The guard's diagnostics.
        diag: Indeterminate,
    },
    /// The configuration routes to the general rung — a documented arm
    /// decision (no runtime fallback exists; C5). The general rung is
    /// implemented; its arms retire one at a time, so a pair reaching
    /// here is one whose arm has not retired.
    RoutesToGeneralRung {
        /// The pair, for the message.
        pair: &'static str,
        /// The routing grounds.
        why: &'static str,
    },
    /// The two cylinders' radii are decided unequal: outside every
    /// closed form the equal-radius arm classifies, so the pair routes
    /// to the general rung, whose cylinder×cylinder arm has not retired.
    UnequalRadii,
    /// An operand violates the surface convention its arm is written
    /// against. Asked per QUESTION rather than through a relation
    /// between two quantities: a relation-only guard admits `r = 0` and
    /// `r = R = 0` and mints a radius-zero locus from them. `what`
    /// carries the whole finding — which clause failed and what it was
    /// measured on — because the clauses are not all about radii.
    DegenerateOperand {
        /// Which clause of the convention failed, and on what.
        what: &'static str,
    },
    /// The closed form's own locus stands where the caller's metered
    /// reach does not: a section curve outside `extent` carries an
    /// absolute position error that scales with the LOCUS rather than
    /// with the operands, so the arm's zero-residual claim would not
    /// hold there. Refused rather than minted.
    BeyondOperandExtent {
        /// What the arm was about to mint, and against which reach.
        what: &'static str,
    },
    /// The two surfaces are coincident (coaxial equal-radius cylinders):
    /// a same-surface locus is a coincidence to declare/merge, never an
    /// intersection curve.
    CoincidentSurfaces,
    /// The torus operand is not shown inside the ring convention
    /// `R > r > 0`, in band or decided, a half at a time
    /// ([`TorusRefusal`]): a horn or spindle torus's meridian circles
    /// meet or cross on the axis, so no closed form here is well-posed.
    /// Doors that read unvalidated bodies (`topo::replace_face_offset`)
    /// reach it, so it ends in the half's lever, as tier 3's
    /// `DegenerateTorus` does.
    DegenerateTorus(TorusRefusal),
    /// The conic carrier constructor refused (near-circular tilt or a
    /// degenerate axis) — the constructor is the one deciding door for
    /// axis ordering (spec §1) and its verdict stands.
    Carrier(EllipseInvalid),
    /// The spiric carrier constructor refused a configuration the arm
    /// classified as its own — the constructor is the one deciding door
    /// for the spiric's regime and its verdict stands.
    Spiric(SpiricInvalid),
    /// A closed-form count read a margin `Zero`, or took a reading as a
    /// tangency, where the pose was decided transverse: the count's
    /// components pinch there, named, and are not counted.
    Pinched {
        /// The reading.
        predicate: &'static str,
    },
    /// A crossing of the carriers the closed form cannot place on either
    /// side of a line it reads the section by, named.
    Unordered {
        /// The reading.
        predicate: &'static str,
    },
}

impl From<EllipseInvalid> for SectionError {
    fn from(e: EllipseInvalid) -> Self {
        Self::Carrier(e)
    }
}

impl core::fmt::Display for SectionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongLane { expected } => write!(
                f,
                "the section was dispatched to the arm for {expected}, not this pair's. {}",
                crate::ssi::WRONG_LANE_RECOURSE
            ),
            // The Indeterminate Display carries the shared two-tolerance
            // recourse (S6) exactly once.
            Self::Escalated(diag) => write!(
                f,
                "the two surfaces' configuration is ill-conditioned at this tolerance: {diag}"
            ),
            Self::RadiusEscalated { radius, diag } => diag
                .undecided(
                    radius.subject(),
                    radius
                        .sized()
                        .recourse(RefusedArm::Undecided(diag), Reading::Build),
                )
                .fmt(f),
            Self::RoutesToGeneralRung { pair, why } => write!(f, "the {pair} section: {why}"),
            Self::UnequalRadii => write!(
                f,
                "the cylinder×cylinder section: unequal radii are outside every closed form \
                 the equal-radius arm classifies, and route to the general rung, whose \
                 cylinder×cylinder arm has not retired"
            ),
            Self::DegenerateOperand { what } => {
                write!(f, "a section operand is degenerate: {what}")
            }
            Self::BeyondOperandExtent { what } => write!(
                f,
                "{what}, outside the extent the section was sized for, where its closed \
                 form is not exact. Recourse: size the section to an extent that covers \
                 the curve"
            ),
            // Raised by the cylinder×cylinder and sphere×sphere tables
            // only. The chord join reads plane-inclusive pairs and never
            // meets it; where a Boolean could (coincident operands), a
            // declaration is exactly the lever, so the shared recourse
            // is the true one.
            Self::CoincidentSurfaces => write!(
                f,
                "the surfaces are coincident (the same cylinder or the same sphere), so \
                 they share a surface rather than meet along a curve. Recourse: {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::DegenerateTorus(refusal) => refusal.fmt(f),
            Self::Carrier(e) => write!(f, "the section's curve refused: {e}"),
            Self::Spiric(e) => write!(f, "the section's spiric refused: {e}"),
            Self::Pinched { predicate } => write!(
                f,
                "the section's components pinch where its closed form counts them \
                 (`{predicate}`), so their count is not certified. Recourse: move one surface \
                 so the two clearly cross or clearly part there"
            ),
            Self::Unordered { predicate } => write!(
                f,
                "the section's closed form cannot place a crossing of the two surfaces on \
                 either side of the line it reads the section by (`{predicate}`). Recourse: \
                 move one surface a little so the crossing stands clear of that line"
            ),
        }
    }
}

impl std::error::Error for SectionError {}

/// **Whose radius a section arm's operand guard reads** (D4 ¶1 (i)): an
/// arm refuses an operand whose radius is not definitely positive
/// before it classifies any pose, so the question is the operand's own
/// size, which no declaration between the two faces names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum SectionRadius {
    /// The cylinder's radius (`cs_cylinder_radius`,
    /// `coc_cylinder_radius`).
    Cylinder,
    /// The sphere's radius (`cs_sphere_radius`).
    Sphere,
}

impl SectionRadius {
    /// What the guard decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Cylinder => "whether a cylinder's radius is positive",
            Self::Sphere => "whether a sphere's radius is positive",
        }
    }

    /// The guard's lever and the size its margin measures: it passes on
    /// a definitely positive radius, which the lever edits.
    #[must_use]
    pub const fn sized(self) -> SizedDecision {
        let lever = match self {
            Self::Cylinder => "make the cylinder's radius clearly larger than the tolerance",
            Self::Sphere => "make the sphere's radius clearly larger than the tolerance",
        };
        SizedDecision {
            lever,
            size: "radius",
            passes: SizedPass::Positive,
            side: RefusedSide::Lever,
            no_size: None,
        }
    }
}

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
    /// loci are `TangentIntersection` territory, M5 PR 9; section
    /// consumers refuse typed). The same ruling is the witness
    /// [`crate::tangent_locus`] mints for a declared `Tangent` pair: a
    /// line the declaration is verified along, never an edge.
    TangentLine(Curve3<T>),
    /// Axis in-plane, gap definitely > r: no intersection.
    Empty,
}

/// Classifies and constructs the plane×cylinder section (spec §3.1).
///
/// `reach` is where the section is consumed ([`Reach`]). The axis-in-plane
/// rows read at the foot of its point on the cylinder's axis, so where
/// the carrier's origin is stored does not enter them.
///
/// Trileans, in order (named lever arms per D4 ¶1):
///
/// 1. `pc_axis_plane_parallel` — margin `c·lever`, `c = axis·normal`,
///    the axis' angle off the plane levered over `reach` from the
///    rulings' hinge, the line through the foot's projection on the
///    plane: the consumed region's axial distance from the hinge's
///    station ([`Reach::hinge_lever`]), plus its distance across the
///    wall from the hinge at second order ([`Reach::turn_lever`]): Zero
///    ⇒ the axis lies in the plane
///    (the parallel degenerate lane, step 2); definite ⇒ a bounded cut
///    (step 3).
/// 2. `pc_parallel_gap` — margin `r − |signed axis-to-plane gap|` at
///    the foot (meters), read across the reach with step 1's tilt
///    displacement beside it ([`decide_across`]; `pc_parallel_gap_floor`
///    decides the definite side): Positive ⇒
///    [`PlaneCylinderSection::ParallelLines`], Zero ⇒
///    [`PlaneCylinderSection::TangentLine`], Negative ⇒
///    [`PlaneCylinderSection::Empty`].
/// 3. `pc_rim_alignment` — margin `‖axis×normal‖·r` (the tilt angle's
///    sine, metered at the rim radius): Zero ⇒
///    [`PlaneCylinderSection::Rim`]; definite ⇒
///    [`PlaneCylinderSection::TiltedEllipse`], however near-circular:
///    this row is the one decision of the section's kind.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, in-band escalations (F6), or a
/// carrier-constructor refusal.
pub fn plane_cylinder_section<T: Decide>(
    plane: &impl SectionPlane<T>,
    cylinder: &Surface<T>,
    reach: &Reach<T>,
    band: Band,
) -> Result<PlaneCylinderSection<T>, SectionError> {
    plane_cylinder_section_read(plane, cylinder, reach, band).map(|(section, _)| section)
}

/// [`plane_cylinder_section`], with `pc_axis_plane_parallel`'s verdict
/// at `reach`: Zero on the rulings' lane, definite on the conic lane. It
/// is the one row that reads the reach and parts the two lanes, for a
/// caller that reads the section at two reaches bracketing what it
/// consumes and must say what the two readings were where they part.
///
/// # Errors
///
/// As [`plane_cylinder_section`].
pub fn plane_cylinder_section_read<T: Decide>(
    plane: &impl SectionPlane<T>,
    cylinder: &Surface<T>,
    reach: &Reach<T>,
    band: Band,
) -> Result<(PlaneCylinderSection<T>, Decided), SectionError> {
    let (pc, cyl_u) = plane_cylinder_data(plane, cylinder)?;
    let (parallel, ruled) =
        plane_cylinder_ruled_read(&pc, reach, band).map_err(SectionError::Escalated)?;
    if let Some(ruled) = ruled {
        return Ok((
            match ruled {
                RuledSection::ParallelLines { l1, l2 } => {
                    PlaneCylinderSection::ParallelLines { l1, l2 }
                }
                RuledSection::TangentLine { origin, dir, .. } => {
                    PlaneCylinderSection::TangentLine(Curve3::Line { origin, dir })
                }
                RuledSection::Empty => PlaneCylinderSection::Empty,
            },
            parallel,
        ));
    }
    let PlaneCylinder { q, n, o, a, r } = pc;
    // Bounded cut: rim circle vs tilted ellipse.
    let c = a.dot(n);
    let sin_vec = a.cross(n);
    let sin_norm = sin_vec.norm();
    let t_star = (q - o).dot(n) / c;
    let center = o + a * t_star;
    let section = match decide("pc_rim_alignment", Margin::levered(sin_norm, r), band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero => PlaneCylinderSection::Rim(Curve3::Circle {
            center,
            axis: a,
            radius: r,
            u_ref: cyl_u,
        }),
        // The margin is a norm: Negative is unreachable; both
        // definite verdicts take the tilted lane.
        Sign::Positive | Sign::Negative => {
            let u_major = tilted_section_major(n, sin_vec);
            let e = Curve3::ellipse(center, n, r / c.abs(), r, u_major, band)?;
            PlaneCylinderSection::TiltedEllipse(e)
        }
    };
    Ok((section, parallel))
}

/// The major direction of the ellipse a plane of unit normal `n` cuts
/// from a cylinder or cone whose unit axis `a` it meets off square,
/// `sin_vec = a × n` (its norm `s` the tilt's sine, definitely
/// positive): `v × n`, `v` the minor direction `sin_vec / s`. The frame
/// is orthonormal to a few `u` at any tilt and in any frame: `sin_vec`
/// carries about `u` of rounding at norm `s`, so `sin_vec / s` would
/// stand up to `u / s` off the plane and its product with `n` fall short
/// of unit by `(u / s)² / 2`; projecting `sin_vec` onto the plane before
/// normalising leaves `v` within a few `u` of the plane, so `v × n` is
/// unit to a few `u` as well. What is left is the in-plane heading's `u / s`, which turns the
/// ellipse about its centre: a point moves by at most the semi-axes'
/// difference times that angle. The difference is of order `A·s²` on a
/// cylinder and `A·tan²α·s²` on a cone of half-angle `α` (`A` the major
/// semi-axis), so the carrier stands within about `A·s·u` of a
/// cylinder's wall, and `tan²α` times that of a cone's: a few `u` of
/// `A` near square, more on a flat cone tilted toward its parabola.
pub fn tilted_section_major<T: Real>(n: Vec3<T>, sin_vec: Vec3<T>) -> Vec3<T> {
    (sin_vec - n * sin_vec.dot(n)).normalize().cross(n)
}

/// The axis-in-plane answers of [`plane_cylinder_section`]: its
/// `ParallelLines`, `TangentLine` and `Empty`, the only three steps 1–2
/// can reach.
#[derive(Clone, Debug)]
pub(crate) enum RuledSection<T: Real> {
    /// The gap is definitely under `r`: two rulings.
    ParallelLines { l1: Curve3<T>, l2: Curve3<T> },
    /// The gap is coincident with `r`: the tangency ruling
    /// `origin + t·dir`, `origin` the axis' foot on the plane, with the
    /// margin that decided the gap Zero.
    TangentLine {
        origin: Point3<T>,
        dir: Vec3<T>,
        gap: MarginDiag,
    },
    /// The gap is definitely over `r`.
    Empty,
}

/// Steps 1–2 of [`plane_cylinder_section`]: the axis-in-plane lane, at
/// `reach`'s frame ([`ruled_frame`]): the tilt row
/// ([`ruled_parallel`]), then, with the axis in the plane, the gap row
/// ([`ruled_section`]). The tangent-locus lane reads its ruling
/// tangency through the same two rows on the same frame, so the
/// section and the witness never decide it apart.
fn plane_cylinder_ruled_read<T: Decide>(
    pc: &PlaneCylinder<T>,
    reach: &Reach<T>,
    band: Band,
) -> Result<(Decided, Option<RuledSection<T>>), Indeterminate> {
    let frame = ruled_frame(pc, reach);
    let parallel = ruled_parallel(&frame, band)?;
    Ok((parallel, ruled_section(pc, &frame, parallel, band)?))
}

/// `pc_axis_plane_parallel`: the axis' normal component levered from
/// the frame's hinge.
pub(crate) fn ruled_parallel<T: Decide>(
    frame: &RuledFrame<T>,
    band: Band,
) -> Result<Decided, Indeterminate> {
    decide_reported(
        "pc_axis_plane_parallel",
        Margin::levered(frame.c, frame.lever),
        band,
    )
}

/// The axis-in-plane lane's answer once the tilt row has read
/// `parallel`: `None` where the axis definitely leaves the plane (step
/// 3 not run); otherwise the gap row ([`ruled_gap`]) mints the rulings,
/// the tangency ruling or none.
pub(crate) fn ruled_section<T: Decide>(
    pc: &PlaneCylinder<T>,
    frame: &RuledFrame<T>,
    parallel: Decided,
    band: Band,
) -> Result<Option<RuledSection<T>>, Indeterminate> {
    // A tangent line's Zero is one only with the tilt's, so its margin
    // stands for both.
    if parallel.sign != Sign::Zero {
        return Ok(None);
    }
    let &RuledFrame { o, gap_signed, .. } = frame;
    let (n, a, r) = (pc.n, pc.a, pc.r);
    Ok(Some(match ruled_gap(pc, frame, band)? {
        (Sign::Positive, _) => {
            // Cross-section chord: the plane cuts the circle at
            // foot ± w·half, foot the axis' plane projection.
            let foot = o - n * gap_signed;
            let half = (r.powi(2) - gap_signed.powi(2)).sqrt();
            let w = a.cross(n).normalize();
            RuledSection::ParallelLines {
                l1: Curve3::Line {
                    origin: foot + w * half,
                    dir: a,
                },
                l2: Curve3::Line {
                    origin: foot - w * half,
                    dir: a,
                },
            }
        }
        (Sign::Zero, gap) => RuledSection::TangentLine {
            origin: o - n * gap_signed,
            dir: a,
            gap: gap.jointly(parallel.margin),
        },
        (Sign::Negative, _) => RuledSection::Empty,
    }))
}

/// The axis-in-plane lane's frame: the foot of `reach`'s point on the
/// axis, the axis' normal component `c`, the foot's signed height over
/// the plane, and the lever the tilt is read at.
pub(crate) struct RuledFrame<T: Real> {
    pub(crate) o: Point3<T>,
    pub(crate) c: T,
    pub(crate) gap_signed: T,
    pub(crate) lever: T,
}

/// [`RuledFrame`] of the pair at `reach`.
pub(crate) fn ruled_frame<T: Decide>(pc: &PlaneCylinder<T>, reach: &Reach<T>) -> RuledFrame<T> {
    let &PlaneCylinder { q, n, o, a, .. } = pc;
    let o = reach.foot_on(o, a);
    let c = a.dot(n);
    let gap_signed = (o - q).dot(n);
    // The rulings this lane mints stand on the hinge through `o − n·gap`,
    // `−gap·c` along the axis from the foot; the reach is levered from
    // there (`geom_brep::extent`'s module docs).
    let hinge = o - n * gap_signed;
    let cos = (T::one() - c.powi(2)).max(T::zero()).sqrt();
    let lever = reach.hinge_lever(o, -(gap_signed * c)) + reach.turn_lever(hinge, (n, a), c, cos);
    RuledFrame {
        o,
        c,
        gap_signed,
        lever,
    }
}

/// **The axis-in-plane lane's gap row**, `pc_parallel_gap`: a consumed
/// point of a ruling stands off the plane by the tilt levered from the
/// hinge, and the wall stands `r − |gap|` off the plane at the foot, so
/// the ruling stands at most their sum off the section and the gap is
/// read across the reach with the tilt beside it. Positive crosses,
/// Negative clears, Zero touches along the ruling.
///
/// It reads whatever `pc_axis_plane_parallel` decides: a definite sign
/// here is a crossing or a clearance no tilt within the swing closes,
/// which is how the tangent-locus lane refuses a pair the axis row
/// leaves in band ([`crate::tangent_locus`]).
pub(crate) fn ruled_gap<T: Decide>(
    pc: &PlaneCylinder<T>,
    frame: &RuledFrame<T>,
    band: Band,
) -> Result<(Sign, MarginDiag), Indeterminate> {
    decide_across_reported(
        ["pc_parallel_gap", "pc_parallel_gap_floor"],
        pc.r - frame.gap_signed.abs(),
        Margin::levered(frame.c, frame.lever).value(),
        band,
    )
}

/// A plane×cylinder pair: the plane's origin `q` and unit normal `n`,
/// the cylinder's origin `o`, unit axis `a` and radius `r`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PlaneCylinder<T: Real> {
    pub(crate) q: Point3<T>,
    pub(crate) n: Vec3<T>,
    pub(crate) o: Point3<T>,
    pub(crate) a: Vec3<T>,
    pub(crate) r: T,
}

/// The pair's data and the cylinder's seam, or the wrong-lane refusal.
fn plane_cylinder_data<T: Real>(
    plane: &impl SectionPlane<T>,
    cylinder: &Surface<T>,
) -> Result<(PlaneCylinder<T>, Vec3<T>), SectionError> {
    let wrong = || SectionError::WrongLane {
        expected: "plane×cylinder",
    };
    let Some((q, n)) = plane.site() else {
        return Err(wrong());
    };
    let &Surface::Cylinder {
        origin: o,
        axis: a,
        radius: r,
        u_ref,
    } = cylinder
    else {
        return Err(wrong());
    };
    Ok((PlaneCylinder { q, n, o, a, r }, u_ref))
}

// ---------------------------------------------------------------------
// the plane a plane×carrier section reads
// ---------------------------------------------------------------------

/// What a plane×carrier section reads of its plane: a point of it and
/// its unit normal, and never its chart. A stored [`Surface::Plane`]
/// serves, and so does a [`PlaneSite`], a plane no output keeps, which
/// has no `u_ref` to pick.
pub trait SectionPlane<T: Real> {
    /// The plane's `(origin, normal)`, or `None` for a surface that is
    /// not a plane (the section's wrong-lane refusal).
    fn site(&self) -> Option<(Point3<T>, Vec3<T>)>;
}

impl<T: Real> SectionPlane<T> for Surface<T> {
    fn site(&self) -> Option<(Point3<T>, Vec3<T>)> {
        match *self {
            Surface::Plane { origin, normal, .. } => Some((origin, normal)),
            _ => None,
        }
    }
}

/// A plane as a section reads it: a point of it and its unit normal.
#[derive(Clone, Copy, Debug)]
pub struct PlaneSite<T: Real> {
    /// A point of the plane.
    pub origin: Point3<T>,
    /// The plane's unit normal.
    pub normal: Vec3<T>,
}

impl<T: Real> SectionPlane<T> for PlaneSite<T> {
    fn site(&self) -> Option<(Point3<T>, Vec3<T>)> {
        Some((self.origin, self.normal))
    }
}

// ---------------------------------------------------------------------
// plane × sphere (M5 S13: the die-pips join lane's C5 row)
// ---------------------------------------------------------------------

/// The classified plane×sphere section — every configuration's closed
/// form (rung 1: the trileans run before any rung, C5; **no fitted
/// chord anywhere in this pair** — the locus is an exact `Circle`).
#[derive(Clone, Debug)]
pub enum PlaneSphereSection<T: Real> {
    /// Definite cut: the exact `Circle` — centered at the sphere
    /// center's foot on the plane, radius `√((r−|s|)(r+|s|))` for `s`
    /// the signed center-to-plane gap, carrier axis the plane normal,
    /// `u_ref` read off the sphere's chart frame (`n̂ × û`, or `n̂ × â`
    /// where that is short; in-plane by construction).
    /// Zero-residual-by-construction against both surfaces.
    Circle(Curve3<T>),
    /// Gap coincident with r: the tangency **point** — classification
    /// data, not a constructible edge (C7 lineage: tangent loci are
    /// not minted as section carriers; consumers refuse typed).
    TangentPoint(Point3<T>),
    /// Gap definitely > r: no intersection.
    Empty,
}

/// Classifies and constructs the plane×sphere section (C5 rung 1,
/// M5 S13).
///
/// One trilean: `ps_center_gap` — margin `r − |s|` (meters), `s` the
/// signed sphere-center-to-plane gap: Positive ⇒
/// [`PlaneSphereSection::Circle`], Zero ⇒
/// [`PlaneSphereSection::TangentPoint`], Negative ⇒
/// [`PlaneSphereSection::Empty`]. The in-band twin of both definite
/// verdicts escalates through the same named predicate (F6) — the
/// two-tolerance shape.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds or the in-band escalation.
pub fn plane_sphere_section<T: Decide>(
    plane: &impl SectionPlane<T>,
    sphere: &Surface<T>,
    band: Band,
) -> Result<PlaneSphereSection<T>, SectionError> {
    let Some((q, n)) = plane.site() else {
        return Err(SectionError::WrongLane {
            expected: "plane×sphere",
        });
    };
    let &Surface::Sphere {
        center: c,
        radius: r,
        axis: sph_axis,
        u_ref: sph_u,
    } = sphere
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×sphere",
        });
    };
    let s = (c - q).dot(n);
    let foot = c - n * s;
    // The circle's `u_ref` is a PLACEMENT convention (D2): every
    // downstream margin is a frame DIFFERENCE, so any in-plane unit
    // vector serves — but it must be genuinely in-plane. Derived from
    // the sphere's chart frame, an input, so it moves with the inputs
    // and not with the computing axes:
    // `n̂ × û` unless that sine is small, then `n̂ × â` — with
    // `û ⊥ â` the two sines satisfy sin²(û,n̂) + sin²(â,n̂) ≥ 1, so
    // the second candidate is definitely nonzero whenever the first
    // is not chosen. The selection trilean's degenerate and in-band
    // arms both take the second candidate: near the threshold BOTH
    // are valid placements, so the arm is a deterministic tie-break
    // (D9), not a verdict — no downstream VERDICT moves with it
    // (derived f64 parameter bits may differ across code versions;
    // within-run replay and strategy identity are what D9 pins).
    let seam_cand = n.cross(sph_u);
    let u_ref = match decide(
        "ps_frame_seam",
        Margin::levered(seam_cand.norm() - T::from_f64(0.5), r),
        band,
    ) {
        Ok(Sign::Positive) => seam_cand / seam_cand.norm(),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {
            let polar_cand = n.cross(sph_axis);
            polar_cand / polar_cand.norm()
        }
    };
    match decide("ps_center_gap", Margin::of(r - s.abs()), band).map_err(SectionError::Escalated)? {
        Sign::Positive => Ok(PlaneSphereSection::Circle(Curve3::Circle {
            center: foot,
            axis: n,
            // The interval-square tripwire does not bite: both factors
            // are definitely positive after the trilean (never a
            // spuriously negative bracket under a sqrt).
            radius: ((r - s.abs()) * (r + s.abs())).sqrt(),
            u_ref,
        })),
        Sign::Zero => Ok(PlaneSphereSection::TangentPoint(foot)),
        Sign::Negative => Ok(PlaneSphereSection::Empty),
    }
}

// ---------------------------------------------------------------------
// sphere × sphere
// ---------------------------------------------------------------------

/// The classified sphere×sphere section (rung 1: the trileans run
/// before any rung, C5; **no fitted chord anywhere in this pair** — the
/// locus is an exact `Circle`).
#[derive(Clone, Debug)]
pub enum SphereSphereSection<T: Real> {
    /// Definite cut: the exact radical-plane `Circle`, centred on the
    /// centre line at `a = (d² + r₁² − r₂²)/2d` from sphere 1, radius
    /// `√((r₁−a)(r₁+a))`, carrier axis the unit centre direction.
    /// Zero-residual-by-construction against both surfaces.
    Circle(Curve3<T>),
    /// The spheres touch — externally (`d = r₁ + r₂`) or internally
    /// (`d = |r₁ − r₂|`) — at the one **point** `c₁ + n̂·a`: the
    /// degenerate section circle, at the SAME signed offset
    /// `a = (d² + r₁² − r₂²)/2d` the [`SphereSphereSection::Circle`]
    /// arm uses, with radius zero. One formula, three branches.
    ///
    /// The sign is load-bearing and is why `a` is not written `r₁`.
    /// External tangency and internal tangency with `r₁ > r₂` both give
    /// `a = +r₁`; internal tangency with `r₂ > r₁` — sphere A strictly
    /// inside sphere B — gives `a = −r₁`, and the contact is
    /// diametrically opposite, on the `−n̂` side. Substituting `d` into
    /// `a` gives `+r₁` for two of the three configurations and `−r₁`
    /// for the third; `|a| = r₁` is what all three share.
    ///
    /// Classification data, not a constructible edge (C7 lineage:
    /// tangent loci are not minted as section carriers; consumers
    /// refuse typed).
    TangentPoint(Point3<T>),
    /// Definitely separated (`d > r₁ + r₂`) or definitely strictly
    /// nested (`d < |r₁ − r₂|`): no intersection.
    Empty,
}

/// Classifies and constructs the sphere×sphere section (C5 rung 1).
///
/// The 2-D skeleton is `profile`'s arc/arc carrier gate, and the three
/// trileans are its three, in the same gate order and with the same
/// per-branch division argument — the algebra is identical, only the
/// perpendicular direction differs (a 2-D normal there, a whole circle
/// here). All margins are LENGTHS in metres:
///
/// - **`ss_carrier_identity`** — margin `d + |r₁ − r₂|`, the carrier
///   distance in "same sphere" terms. Zero ⇒ the two surfaces are the
///   same sphere, refused as [`SectionError::CoincidentSurfaces`]: a
///   same-surface locus is a whole-sphere coincidence to declare or
///   merge, never a section circle, and CONTACT-DESIGN forbids
///   inferring the gluing at any ε.
/// - **`ss_carrier_external`** — margin `d − (r₁ + r₂)`. Positive ⇒
///   [`SphereSphereSection::Empty`] (separated); Zero ⇒
///   [`SphereSphereSection::TangentPoint`] (external tangency).
/// - **`ss_carrier_internal`** — margin `d − |r₁ − r₂|`. Negative ⇒
///   [`SphereSphereSection::Empty`] (strictly nested); Zero ⇒
///   [`SphereSphereSection::TangentPoint`] (internal tangency);
///   Positive ⇒ [`SphereSphereSection::Circle`], of radius
///   [`geom_core::Circles::half_chord`]: its square's four factors are
///   the two clearances this branch has pinned definite (one negated)
///   and two sums of non-negatives, so it is definitely positive here
///   at every scalar and the interval-square tripwire cannot bite.
///   Each factor rounds at the inputs' scale, so the circle sits on
///   the smaller sphere at radii orders of magnitude apart.
///
/// Divisions by `d` occur only where a preceding trilean bounds it away
/// from zero, per branch:
/// the secant branch has `d > |r₁ − r₂| ≥ 0` definitely; the internal
/// tangency has `d = |r₁ − r₂|` with the identity margin `2d` definite;
/// the external tangency has `d = r₁ + r₂ ≥` the identity margin.
///
/// The in-band twin of every definite verdict escalates through the
/// same named predicate (F6) — the two-tolerance shape.
///
/// The form is `atan2`-free and branch-cut-free by construction, so the
/// `Interval` lane takes it unchanged: there is no lane fork here.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, coincident surfaces, or the
/// in-band escalation.
pub fn sphere_sphere_section<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    band: Band,
) -> Result<SphereSphereSection<T>, SectionError> {
    let wrong = || SectionError::WrongLane {
        expected: "sphere×sphere",
    };
    let &Surface::Sphere {
        center: c1,
        radius: r1,
        axis: axis1,
        u_ref: u1,
    } = a
    else {
        return Err(wrong());
    };
    let &Surface::Sphere {
        center: c2,
        radius: r2,
        ..
    } = b
    else {
        return Err(wrong());
    };
    // The margins and the SIGNED radical-plane foot, from one pair
    // (`geom_core::Circles`). Every branch that places the foot has
    // already decided `d` definitely positive, so its division is safe.
    let at = geom_core::Circles::new((c2 - c1).norm(), r1, r2).at(c1, c2);
    let pair = at.pair();
    match decide("ss_carrier_identity", Margin::of(pair.identity()), band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero | Sign::Negative => Err(SectionError::CoincidentSurfaces),
        Sign::Positive => {
            match decide("ss_carrier_external", Margin::of(pair.external()), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Positive => Ok(SphereSphereSection::Empty),
                // Externally tangent: `d = r₁ + r₂ ≥` the definite
                // identity margin, so the division is safe. The contact
                // is the degenerate section circle, `c₁ + n̂·a` at
                // radius zero — and `a` must be the SIGNED offset, not
                // `r₁`. Substituting `d = r₁ + r₂` does give `a = +r₁`
                // here; the internal branch below is where the sign
                // matters, and one formula serves both.
                Sign::Zero => Ok(SphereSphereSection::TangentPoint(at.foot())),
                Sign::Negative => {
                    match decide("ss_carrier_internal", Margin::of(pair.internal()), band)
                        .map_err(SectionError::Escalated)?
                    {
                        Sign::Negative => Ok(SphereSphereSection::Empty),
                        // Internally tangent: `d = |Δr|` and the
                        // identity margin `d + |Δr| = 2d` is definite,
                        // so `d` is bounded away from zero. Here the
                        // SIGN of `a` is the whole answer: substituting
                        // `d = |r₁ − r₂|` gives `a = +r₁` when `r₁ > r₂`
                        // (B inside A, contact on the `+n̂` side) and
                        // `a = −r₁` when `r₂ > r₁` (A inside B, contact
                        // on the `−n̂` side, diametrically opposite).
                        // Writing `+r₁` in both orders puts the contact
                        // point INSIDE the larger ball — for the unit
                        // sphere at the origin against radius 3 at
                        // `x = 2` it returns `(1,0,0)`, which is B's own
                        // centre, and the true contact is `(−1,0,0)`.
                        Sign::Zero => Ok(SphereSphereSection::TangentPoint(at.foot())),
                        // Proper secant: `d > |Δr|` definitely, so
                        // `d > 0`. The radical plane's foot and half-chord.
                        Sign::Positive => {
                            let n = at.direction();
                            Ok(SphereSphereSection::Circle(Curve3::Circle {
                                center: at.foot(),
                                axis: n,
                                radius: pair.half_chord(),
                                u_ref: ss_frame_seam(n, axis1, u1, r1, band),
                            }))
                        }
                    }
                }
            }
        }
    }
}

/// The section circle's `u_ref` PLACEMENT (D2/D9), the sphere sibling
/// of the plane×sphere lane's identical tie-break.
///
/// The section axis is the centre direction `n̂`, and the candidate
/// `n̂ × û` collapses exactly when the centre line runs along the
/// operand's own seam direction — the same degeneracy, reached by a
/// different route. Derived from operand A's chart: `n̂ × û` unless
/// that sine is small, then `n̂ × â` — with `û ⊥ â` the two sines
/// satisfy sin²(û,n̂) + sin²(â,n̂) ≥ 1, so the second candidate is
/// definitely nonzero whenever the first is not chosen. The selection
/// trilean's degenerate and in-band arms both take the second
/// candidate: near the threshold BOTH are valid placements, so the arm
/// is a deterministic tie-break, not a verdict — **no downstream
/// VERDICT moves with it**.
fn ss_frame_seam<T: Decide>(
    n: Vec3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    r: T,
    band: Band,
) -> Vec3<T> {
    let seam_cand = n.cross(u_ref);
    match decide(
        "ss_frame_seam",
        Margin::levered(seam_cand.norm() - T::from_f64(0.5), r),
        band,
    ) {
        Ok(Sign::Positive) => seam_cand / seam_cand.norm(),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {
            let polar_cand = n.cross(axis);
            polar_cand / polar_cand.norm()
        }
    }
}

// ---------------------------------------------------------------------
// cylinder × cylinder, equal radii (spec §3.2)
// ---------------------------------------------------------------------

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
        /// The two coincidences decided Zero to reach them, read off
        /// their decisions: the radii equal (`cc_radius_equality`), then
        /// the axes meeting (`cc_axes_coplanar`).
        margins: [MarginDiag; 2],
        /// The two NODES, where the ellipses cross: `p ∓ r·û`, `p` the
        /// axes' meeting point and `û = unit(a1 × a2)`. The first is
        /// `e1` at parameter `π/2` and `e2` at `−π/2`, the second `e1`
        /// at `−π/2` and `e2` at `π/2` (the minor vertices of both).
        /// The walls are tangent there, their common tangent plane
        /// spanned by the axes, and the ellipses cross square, along
        /// `a1 + a2` (`e1`) and `a1 − a2` (`e2`). Round a node each
        /// wall's four sectors between the four arcs alternate sides of
        /// the other CYLINDER: on wall 1 the sectors about `±a1` lie
        /// outside cylinder 2 and those about `±a2` inside it; on wall 2
        /// the same sectors, the other way round. Which side of a
        /// cylinder is its solid's material is the face's, not the
        /// surface's.
        nodes: [Point3<T>; 2],
    },
    /// Parallel axes, gap definitely < r₁ + r₂: two parallel rulings.
    ParallelLines {
        /// One ruling.
        l1: Curve3<T>,
        /// The other.
        l2: Curve3<T>,
    },
    /// Parallel axes, gap coincident with r₁ + r₂: the tangency ruling —
    /// classification data, not a constructible edge (C7 / M5 PR 9).
    TangentLine(Curve3<T>),
    /// Parallel axes, gap definitely > r₁ + r₂: no intersection.
    Empty,
}

/// Classifies and constructs the equal-radius cylinder×cylinder
/// section (spec §3.2).
///
/// Trileans, in order:
///
/// 1. `cc_radius_equality` — margin `r₁ − r₂` (meters): Zero is the
///    equal-radius pose (D10: a coincidence decided by its margin);
///    definite ⇒ [`SectionError::UnequalRadii`]; in-band ⇒ escalated.
/// 2. `cc_axes_parallel` — margin `‖a1×a2‖·lever`, the lever
///    [`Reach::lever_between`] the axes by `reach`
///    ([`cylinder_axes_parallel`]): Zero ⇒ the parallel lane (step 3);
///    definite ⇒ the crossing lane (step 4).
/// 3. `cc_coaxial` / `cc_parallel_gap` — axis-to-axis distance `d`,
///    read between the feet of `reach`'s point on the two axes
///    ([`Reach::foot_on`]), the same in either order, each read across
///    the reach with the axes' distance's own range beside it
///    ([`parallel_swing`], [`decide_across`]; the `_floor` rows decide
///    the definite side):
///    coincident-with-zero ⇒
///    [`SectionError::CoincidentSurfaces`]; then margin `r₁ + r₂ − d`:
///    Positive ⇒ two rulings, Zero ⇒ tangent ruling, Negative ⇒ empty.
/// 4. `cc_axes_coplanar` — margin the signed axis-to-axis gap
///    `w·(a1×a2)/‖a1×a2‖` (meters), `w` between the axes' feet at
///    `reach` ([`cylinder_axes_coplanar`]): Zero ⇒ intersecting axes ⇒
///    the two bisector-plane ellipses; definite ⇒ skew ⇒ typed rung-3
///    refusal.
///
/// # Errors
///
/// [`SectionError`] — see the trilean list.
pub fn cylinder_cylinder_section<T: Decide>(
    c1: &Surface<T>,
    c2: &Surface<T>,
    reach: &Reach<T>,
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

    // 1. Equal radii, decided by their margin.
    let radii = decide_reported("cc_radius_equality", Margin::of(r1 - r2), band)
        .map_err(SectionError::Escalated)?;
    match radii.sign {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => return Err(SectionError::UnequalRadii),
    }

    match cylinder_axes_parallel(reach, (o1, a1), (o2, a2), band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero => {
            // Parallel axes: the cross-section is two equal circles at
            // center distance d.
            let ParallelAxes { foot1, d_vec, d } = parallel_axes_at(reach, (o1, a1), (o2, a2));
            // The axes stand within `d ± swing` of each other across the
            // reach, so a coincidence stands at most their sum off.
            let swing = parallel_swing(reach, (o1, a1), (o2, a2), d);
            match decide_across(["cc_coaxial", "cc_coaxial_floor"], d, swing, band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Zero => return Err(SectionError::CoincidentSurfaces),
                Sign::Positive | Sign::Negative => {}
            }
            let two = T::from_f64(2.0);
            let mid = foot1 + d_vec * T::from_f64(0.5);
            match parallel_cylinder_gap((r1, r2), d, swing, band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Positive => {
                    // The rulings stand `±half` off `mid` along `h`, both
                    // ⊥ `a1`, so `half` is read from the perpendicular gap
                    // `|d_vec|` as `mid` and `h` are: then each ruling lies
                    // on the first wall exactly. `|d_vec| ≤ d < 2·r1`
                    // here, so the root is real.
                    let half = (r1.powi(2) - (d_vec.norm() / two).powi(2)).sqrt();
                    let h = a1.cross(d_vec).normalize();
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
                    origin: mid,
                    dir: a1,
                })),
                Sign::Negative => Ok(EqualCylinderSection::Empty),
            }
        }
        Sign::Positive | Sign::Negative => {
            // Crossing lane: coplanarity (intersecting vs skew).
            let meet = cylinder_axes_coplanar_reported(reach, (o1, a1), (o2, a2), band)
                .map_err(SectionError::Escalated)?;
            match meet.sign {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => {
                    return Err(SectionError::RoutesToGeneralRung {
                        pair: "cylinder×cylinder",
                        why: "skew axes have no conic section; this configuration routes \
                              to the general rung, whose cylinder×cylinder arm has not \
                              retired",
                    });
                }
            }
            // The axes' intersection point (closest point on axis 1;
            // the coplanarity verdict bounds the residual by ε).
            let (foot1, w0) = axes_feet(reach, (o1, a1), (o2, a2));
            let cross = a1.cross(a2);
            let t1 = w0.cross(a2).dot(cross) / cross.norm().powi(2);
            let p = foot1 + a1 * t1;
            // The two bisector planes through p. Each ellipse is the
            // tilted plane×cylinder cut of cylinder 1 (by symmetry it
            // lies on cylinder 2 as well).
            //
            // The normals are `a1 ∓ a2`, whose halved lengths are the
            // half-angle's sine and cosine read without cancellation
            // (`a·n` would read the sine off `1 − cos φ`, to `u / φ²`).
            // The shorter is made ⊥ the longer: an axis a hair off unit
            // turns `a1 − a2` toward `a1 + a2` by `u / φ`, and the long
            // ellipse's plane must not tilt by that.
            let two = T::from_f64(2.0);
            let (d, s) = (a1 - a2, a1 + a2);
            let off_unit = |v: Vec3<T>, w: Vec3<T>| v - w * v.dot(w);
            let (dn, sn) = (d.normalize(), s.normalize());
            let d_shorter = d.norm() - s.norm();
            let pick = |when_le: Vec3<T>, when_gt: Vec3<T>| {
                Vec3::new(
                    d_shorter.select_le_zero(when_le.x, when_gt.x),
                    d_shorter.select_le_zero(when_le.y, when_gt.y),
                    d_shorter.select_le_zero(when_le.z, when_gt.z),
                )
            };
            let (d, s) = (pick(off_unit(d, sn), d), pick(s, off_unit(s, dn)));
            let (n1, n2) = (d.normalize(), s.normalize());
            let (sin_half, cos_half) = (d.norm() / two, s.norm() / two);
            let mk = |n: Vec3<T>, c: T| -> Result<Curve3<T>, SectionError> {
                let u_major = tilted_section_major(n, a1.cross(n));
                Ok(Curve3::ellipse(p, n, r1 / c, r1, u_major, band)?)
            };
            let off = cross * (r1.abs() / cross.norm());
            Ok(EqualCylinderSection::TwoEllipses {
                e1: mk(n1, sin_half)?,
                e2: mk(n2, cos_half)?,
                margins: [radii.margin, meet.margin],
                nodes: [p - off, p + off],
            })
        }
    }
}

/// `cc_axes_parallel`: whether the cylinder axes `oᵢ + s·aᵢ` (`aᵢ`
/// unit) are parallel by `reach`, their sine `‖a1×a2‖` levered at
/// [`Reach::lever_between`]. Zero ⇒ parallel.
///
/// The ONE reading of the fact: the section table's step 3, the
/// tangent-locus lane and topo's germ frame all ask it here, so the
/// frame and the table it re-enters cannot disagree.
///
/// # Errors
///
/// The trilean's diagnostics where it lands in the band or poisons.
pub fn cylinder_axes_parallel<T: Decide>(
    reach: &Reach<T>,
    line1: (Point3<T>, Vec3<T>),
    line2: (Point3<T>, Vec3<T>),
    band: Band,
) -> Result<Sign, Indeterminate> {
    cylinder_axes_parallel_reported(reach, line1, line2, band).map(|d| d.sign)
}

/// [`cylinder_axes_parallel`], keeping the reading it decided on.
///
/// # Errors
///
/// As [`cylinder_axes_parallel`].
pub fn cylinder_axes_parallel_reported<T: Decide>(
    reach: &Reach<T>,
    line1: (Point3<T>, Vec3<T>),
    line2: (Point3<T>, Vec3<T>),
    band: Band,
) -> Result<Decided, Indeterminate> {
    decide_reported(
        "cc_axes_parallel",
        Margin::levered(
            line1.1.cross(line2.1).norm(),
            reach.lever_between(line1, line2),
        ),
        band,
    )
}

/// **How far two near-parallel axes' distance moves across the reach**
/// from the `d` the parallel lane's rows read between the feet: the
/// `swing` [`decide_across`] carries beside each of them.
///
/// From the foot `p` of one line (`p + s·a`, the consumed stations
/// within that foot's lever `L` of it, [`Reach::foot_lever`]) to the
/// other line `o + t·b` the distance is EXACTLY `|u + s·τ|`, `u` and `τ`
/// the parts of `p − o` and `a` off `b` (`separation_range`). Its range
/// over `|s| ≤ L` is read exactly, so a tilt's component across the
/// offset moves it only by its second-order lift and its component along
/// the offset by the full first-order travel. Every distance in that
/// range stands within `max(|dmax − d|, |d − dmin|)` of `d`. Read from
/// either foot it bounds the same separation across the reach (the
/// argument [`Reach::lever_between`] makes), so the lesser holds and
/// the answer does not depend on operand order.
pub(crate) fn parallel_swing<T: Real>(
    reach: &Reach<T>,
    line1: (Point3<T>, Vec3<T>),
    line2: (Point3<T>, Vec3<T>),
    d: T,
) -> T {
    let from = |this: (Point3<T>, Vec3<T>), other: (Point3<T>, Vec3<T>)| {
        let (foot, lever) = reach.foot_lever(this);
        let (dmin, dmax) = separation_range((foot, this.1), other, lever);
        (dmax - d).abs().max((d - dmin).abs())
    };
    from(line1, line2).min(from(line2, line1))
}

/// The least and greatest distance from the points `p + s·a`,
/// `|s| ≤ lever`, to the line `o + t·b` (`a`, `b` unit): exactly
/// `|u + s·τ|` with `u = (p − o)` and `τ = a` each less its part along
/// `b`, a convex function of `s`, so its greatest is at an end and its
/// least at the clamped stationary point `−(u·τ)/(τ·τ)`.
fn separation_range<T: Real>(
    (p, a): (Point3<T>, Vec3<T>),
    (o, b): (Point3<T>, Vec3<T>),
    lever: T,
) -> (T, T) {
    let off = |v: Vec3<T>| v - b * v.dot(b);
    let (u, tau) = (off(p - o), off(a));
    let dmax = (u + tau * lever).norm().max((u - tau * lever).norm());
    let at = (T::zero() - u.dot(tau)) / tau.dot(tau).max(T::from_f64(f64::MIN_POSITIVE));
    let dmin = (u + tau * at.max(T::zero() - lever).min(lever)).norm();
    (dmin, dmax)
}

/// `cc_axes_coplanar`: whether two crossing cylinder axes `oᵢ + s·aᵢ`
/// (`aᵢ` unit) meet, the margin their signed gap along the common
/// perpendicular, `w·(a1×a2)/‖a1×a2‖`, `w` between their feet at `reach`
/// (`axes_feet`). Zero ⇒ they meet; definite ⇒ skew.
///
/// Sliding either point along its axis leaves the gap unchanged, so the
/// feet name the same gap as any stored origins, with less of the
/// rounding a far-stored origin's coordinates carry. The division by
/// `‖a1×a2‖` makes the margin the axis-to-axis LENGTH the band is
/// denominated in.
///
/// The ONE reading, as [`cylinder_axes_parallel`] is: the section
/// table's step 5 and topo's germ frame ask it here.
///
/// # Errors
///
/// The trilean's diagnostics where it lands in the band or poisons.
pub fn cylinder_axes_coplanar<T: Decide>(
    reach: &Reach<T>,
    line1: (Point3<T>, Vec3<T>),
    line2: (Point3<T>, Vec3<T>),
    band: Band,
) -> Result<Sign, Indeterminate> {
    cylinder_axes_coplanar_reported(reach, line1, line2, band).map(|d| d.sign)
}

/// [`cylinder_axes_coplanar`], keeping the margin it decided on.
///
/// # Errors
///
/// As [`cylinder_axes_coplanar`].
pub fn cylinder_axes_coplanar_reported<T: Decide>(
    reach: &Reach<T>,
    line1: (Point3<T>, Vec3<T>),
    line2: (Point3<T>, Vec3<T>),
    band: Band,
) -> Result<Decided, Indeterminate> {
    let (_, w) = axes_feet(reach, line1, line2);
    let cross = line1.1.cross(line2.1);
    decide_reported(
        "cc_axes_coplanar",
        Margin::of(w.dot(cross) / cross.norm()),
        band,
    )
}

/// The feet of `reach`'s point on two axes `oᵢ + s·aᵢ` ([`Reach::foot_on`]):
/// the first foot, and the offset from it to the second's. The one
/// place the cylinder pair's rows read where its axes stand.
fn axes_feet<T: Real>(
    reach: &Reach<T>,
    (o1, a1): (Point3<T>, Vec3<T>),
    (o2, a2): (Point3<T>, Vec3<T>),
) -> (Point3<T>, Vec3<T>) {
    let foot1 = reach.foot_on(o1, a1);
    (foot1, reach.foot_on(o2, a2) - foot1)
}

/// Two parallel axes read at their feet ([`axes_feet`]). Decisions read
/// `d`, which is the same in either operand order; constructions read
/// `d_vec`, which is ⊥ the first axis so a ruling laid across it lies on
/// the first wall. They differ by the feet's axial offset, which the
/// band's tilt allows.
///
/// A [`Reach::Span`] picks its pivot per axis, so the two feet are each
/// axis's least-lever point rather than the feet of one point. "The same
/// in either order" holds for the edges that reach this arm: an edge on
/// two parallel walls of equal radius is a ruling, so it stands at one
/// distance from both axes and its least-lever points are the feet of
/// one axial station. Geometry that would split them (an edge on
/// neither wall) does not come through a consumer.
pub struct ParallelAxes<T: Real> {
    /// The first axis's foot.
    pub foot1: Point3<T>,
    /// The feet's offset ⊥ the first axis.
    pub d_vec: Vec3<T>,
    /// The distance between the feet.
    pub d: T,
}

/// [`ParallelAxes`] of the lines `oᵢ + s·aᵢ` (`aᵢ` unit) at `reach`.
/// Shared with the tangent-locus lane and topo's radical plane of a
/// parallel cylinder pair.
#[must_use]
pub fn parallel_axes_at<T: Real>(
    reach: &Reach<T>,
    line1: (Point3<T>, Vec3<T>),
    line2: (Point3<T>, Vec3<T>),
) -> ParallelAxes<T> {
    let (foot1, w) = axes_feet(reach, line1, line2);
    let a1 = line1.1;
    ParallelAxes {
        foot1,
        d_vec: w - a1 * w.dot(a1),
        d: w.norm(),
    }
}

/// `cc_parallel_gap`: the external-tangency margin `r1 + r2 − d` of two
/// parallel cylinders whose axes stand `d` apart at their feet, read
/// across the reach with `swing` beside it ([`decide_across`],
/// [`parallel_swing`]): the axes stand within `d ± swing` of each other
/// at every consumed station, so a ruling the verdict serves stands at
/// most `|r1 + r2 − d| + swing` off either wall there. Positive ⇒ the walls cross along two rulings, Zero ⇒
/// they touch along one, Negative ⇒ they clear. Symmetric in the pair,
/// so the verdict does not depend on which cylinder is first. Shared
/// with the tangent-locus lane.
pub(crate) fn parallel_cylinder_gap<T: Decide>(
    radii: (T, T),
    d: T,
    swing: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    parallel_cylinder_gap_reported(radii, d, swing, band).map(|(sign, _)| sign)
}

/// [`parallel_cylinder_gap`] with the margin it was decided on
/// ([`decide_across_reported`]).
pub(crate) fn parallel_cylinder_gap_reported<T: Decide>(
    (r1, r2): (T, T),
    d: T,
    swing: T,
    band: Band,
) -> Result<(Sign, MarginDiag), Indeterminate> {
    decide_across_reported(
        ["cc_parallel_gap", "cc_parallel_gap_floor"],
        r1 + r2 - d,
        swing,
        band,
    )
}

/// **A two-sided row read across the consumed region**
/// (`crate::extent`'s module docs): `datum` the row's signed margin
/// where its pivot reads it, and `swing` a LENGTH, the most the term
/// its routing row admitted (a tilt levered at the row's own reach, or
/// a position the row is read beside) moves that reading at a consumed
/// point; each caller derives its own. Across the region the reading stands inside
/// `datum ± swing`, so each verdict is decided on its own side of that
/// bracket, each the point deviation that flips it:
///
/// - `far`, `|datum| + swing`, the farthest a consumed point reads from
///   the flip: Zero ⇒ [`Sign::Zero`] at every consumed point;
/// - otherwise `floor`, the bracket's reading nearest the flip (`datum`
///   shrunk toward zero by `swing`, zero where the bracket holds it): a
///   definite sign holds at every consumed point. A floor decided Zero
///   beside a definite far straddles the band, and escalates through
///   the gate.
///
/// The two terms are never decided one at a time: each just inside the
/// band would sum to nearly twice it. `topo`'s section certificate reads
/// its axis rows through this door too.
///
/// # Errors
///
/// The [`Indeterminate`] of the row left undecided, naming it: the far
/// row in its band, or a floor decided Zero beside a definite far row.
pub fn decide_across<T: Decide>(
    names: [&'static str; 2],
    datum: T,
    swing: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    decide_across_reported(names, datum, swing, band).map(|(sign, _)| sign)
}

/// [`decide_across`] with the margin the `far` row was decided on: the
/// Zero verdict's own margin where the row reads Zero, the margin the
/// floor stood past otherwise.
pub(crate) fn decide_across_reported<T: Decide>(
    [far, floor]: [&'static str; 2],
    datum: T,
    swing: T,
    band: Band,
) -> Result<(Sign, MarginDiag), Indeterminate> {
    let swing = swing.abs();
    let (far_side, margin) = decide_magnitude_reported(far, Margin::of(datum.abs() + swing), band)?;
    match far_side {
        Magnitude::Zero => Ok((Sign::Zero, margin)),
        Magnitude::Positive => {
            let near = (datum - swing).max(T::zero()) + (datum + swing).min(T::zero());
            Ok(match decide_nonzero(floor, Margin::of(near), band)? {
                NonzeroSign::Positive => (Sign::Positive, margin),
                NonzeroSign::Negative => (Sign::Negative, margin),
            })
        }
    }
}

// ---------------------------------------------------------------------
// plane × cone (spec §3.3, R1)
// ---------------------------------------------------------------------

/// The classified plane×cone section.
#[derive(Clone, Debug)]
pub enum PlaneConeSection<T: Real> {
    /// Apex on the plane, plane cutting inside the cone, and the real
    /// plane's section starting within the zero band of the apex on both
    /// nappes: two generator lines through the apex.
    ApexLinePair {
        /// The generator at azimuth `φ + δ`.
        l1: Curve3<T>,
        /// The generator at azimuth `φ − δ`.
        l2: Curve3<T>,
    },
    /// Apex on the plane, plane tangent along one generator, and the
    /// real plane's section starting within the zero band of the apex —
    /// classification data, not a constructible edge (C7 / M5 PR 9).
    ApexTangentLine(Curve3<T>),
    /// Apex on the plane, plane through it clearing the cone, and the
    /// real plane's section within the zero band of the apex: the apex
    /// point alone.
    ApexPoint(Point3<T>),
    /// Axis ∥ normal, the section reaching past the zero band of the
    /// apex: the rung-1 `Circle` cut.
    AxisNormalCircle(Curve3<T>),
    /// Plane tilted but meeting every generator, the section reaching
    /// past the zero band of the apex:
    /// the closed-form `Ellipse` (rung 2), carrier axis the plane normal.
    /// Its points stand off both surfaces by the rounding its stored
    /// fields carry, a few `u` of `max(major, |centre|, |apex|)`. That
    /// bounds the distance off the cut, not the fields: near the
    /// parabola the section is ill-conditioned along the plane, and the
    /// semi-axes carry the discriminant's relative rounding `u/|D|`
    /// while the points stay on both surfaces.
    ///
    /// With `c = axis·n`, `s = ‖axis × n‖`, `δ = (apex − q)·n`, the
    /// discriminant `D = sin α·s − cos α·|c|` and
    /// `K = c² − sin²α = −D·(sin α·s + cos α·|c|)` (positive exactly on
    /// this lane, and read in the product form), the Dandelin
    /// construction gives semi-major `|δ|·sin α·cos α / K` along
    /// `û = (axis × n)/s × n`, semi-minor `|δ|·sin α / √K` along
    /// `axis × n`, and centre `apex − δ·n + (c·δ·s/K)·û`: the apex's foot
    /// on the plane moved along the major axis. At `c² = 1` both
    /// semi-axes are the axis-normal circle's `|h|·tan α` and the centre
    /// is its centre — the circle is this form's boundary case.
    TiltedEllipse(Curve3<T>),
    /// Plane tilted to a generator or past it, the section reaching
    /// past the zero band of the apex: a parabola or a hyperbola, which
    /// no curve kind holds (C1). It is served in closed form, and each
    /// arc of it is minted per window as an exact rational-quadratic
    /// `Curve3::Nurbs` ([`OpenConicSection::branch_at`],
    /// [`ConicBranch::arc`]).
    OpenConic(OpenConicSection<T>),
}

/// A cone's rulings out of its apex, by azimuth about its squared axis
/// from its seam: the apex lane's one frame ([`plane_cone_section`],
/// [`plane_cone_apex_extrema`]).
struct ApexFan<T: Real> {
    a: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    sin_a: T,
    cos_a: T,
}

impl<T: Real> ApexFan<T> {
    /// The fan of `cone`'s rulings on the nappe of its axis.
    fn of(axis: Vec3<T>, u_ref: Vec3<T>, half_angle: T) -> Self {
        let SquareFrame {
            axis: a, u_ref: u, ..
        } = square_frame(axis, u_ref);
        let (sin_a, cos_a) = half_angle.sin_cos();
        Self {
            a,
            u,
            v: a.cross(u),
            sin_a,
            cos_a,
        }
    }

    /// The azimuth of `n`'s part square to the axis: the ruling there
    /// reads `g·n` greatest round the fan.
    fn azimuth(&self, n: Vec3<T>) -> T {
        self.v.dot(n).atan2(self.u.dot(n))
    }

    /// The ruling at azimuth `phi`.
    fn ruling(&self, phi: T) -> Vec3<T> {
        let (s, c) = phi.sin_cos();
        self.a * self.cos_a + (self.u * c + self.v * s) * self.sin_a
    }
}

/// **The rulings a plane through a cone's apex reads its extremes
/// along**: `[g(φ), g(φ + π)]`, the rulings (on the nappe of the cone's
/// axis) at the azimuth `φ` of `normal`'s part square to the axis and
/// opposite it, where `g·normal` is greatest and least round the fan.
/// [`plane_cone_section`]'s apex lane builds its section from the same
/// `φ`: a tangent generator is one of these two, and a line pair stands
/// at `φ ± δ`, symmetric about the first. So a reader that subdivides
/// the cone's apex at these rulings and reads the lane's section between
/// them reads one trace. A normal along the axis has no `φ`: every
/// ruling reads it alike, and the caller decides that before asking.
///
/// `None` for a surface that is not a cone.
#[must_use]
pub fn plane_cone_apex_extrema<T: Real>(
    cone: &Surface<T>,
    normal: Vec3<T>,
) -> Option<[Vec3<T>; 2]> {
    let &Surface::Cone {
        axis,
        half_angle,
        u_ref,
        ..
    } = cone
    else {
        return None;
    };
    let fan = ApexFan::of(axis, u_ref, half_angle);
    let phi = fan.azimuth(normal);
    Some([fan.ruling(phi), fan.ruling(phi + T::pi())])
}

/// Classifies and constructs the plane×cone section (spec §3.3): the
/// apex-through degenerates, the axis-normal circle, the tilted ellipse,
/// and the parabola or hyperbola in closed form.
///
/// `reach` is where the section is consumed ([`Reach`]). The arm reads
/// its pose at the apex: `extent` below is [`Reach::lever_from`] the
/// apex, and the line pair is bounded over the reach's window along the
/// axis ([`Reach::range_along`], step 2b).
///
/// Trileans, in order:
///
/// 0. `pn_aperture_sin` and `pn_aperture_cos`, each metered at
///    `extent` — the two clauses of the cone's convention
///    `α ∈ (0, π/2)`, on the file's band posture (module docs), at the
///    two lanes below that DIVIDE by them (the apex lane by
///    `sin α·‖a×n‖`, the axis-normal lane by `cos α`); either failing
///    refuses [`SectionError::DegenerateOperand`] before any pose is
///    classified.
/// 1. `pn_apex_on_plane` — margin `(apex − q)·normal` (meters): Zero ⇒
///    the apex lane (step 2); definite ⇒ step 3.
/// 2. `pn_apex_section` — margin `D = sin α·‖axis×normal‖ −
///    cos α·|axis·normal|` metered at `extent` (the conic-type
///    discriminant, here at its degenerate column: positive exactly when
///    the plane dips inside the cone), read with step 1's apex gap
///    beside it ([`decide_across`]; `pn_apex_section_floor` decides the
///    definite side): Positive ⇒ step 2b, Zero ⇒ step 2c, Negative ⇒
///    step 2a.
///
///    **Where the real plane's section lies from the apex.** A unit
///    generator `g` meets the real plane `−δ/(g·n)` from the apex (δ
///    the apex gap of step 1), on the nappe the sign names, and `g·n`
///    runs over `[−D, M]` with `M = sin α·‖axis×normal‖ +
///    cos α·|axis·normal|` (mirrored where `axis·normal < 0`). So where
///    `D > 0` one nappe's section starts `|δ|/M` from the apex and the
///    other's `|δ|/D`, with `D ≤ M`; where `D < 0` the section is an
///    ellipse on one nappe, whose farthest point stands `|δ|/|D|` off.
///    Each step below decides one of these distances as a magnitude,
///    levered by `|δ|` read with its own rounding: the gap `δ̂` is a
///    rounded dot product of the apex's offset from the plane's origin,
///    `|δ̂ − δ| ≤ e_δ = 5u·Σ|(apex − origin)ᵢ·nᵢ|` (`u = f64::EPSILON/2`:
///    `u` for the difference, `γ₃` for the sum), and `|δ̂| + e_δ` is the
///    lever. A plane built through the apex reads a `δ̂` that is that
///    rounding alone, and the `1/D` above amplifies it. Zero serves the
///    apex outcome, Positive leaves the section to steps 3–4.
///
///    2a. `pn_apex_point_reach` — margin `|δ|/|D|`, the farthest the
///    ellipse reaches: Zero ⇒ [`PlaneConeSection::ApexPoint`].
///
///    2b. `pn_apex_pair_reach` — the pair runs through the apex on
///    both nappes. Its margin is the lesser of two bounds on how far it
///    stands from the real plane's section over the consumed points:
///    the farther start `|δ|/D`, and the bound over the reach, whose
///    derivation is [`pair_bound`]'s. Zero ⇒
///    [`PlaneConeSection::ApexLinePair`].
///
///    2c. `pn_apex_tangent_reach` — margin `|δ|/M`: the tangent
///    generator stands in for a touching whose clearing nappe may carry
///    no section, so it is bounded by the nearer start. Zero ⇒
///    [`PlaneConeSection::ApexTangentLine`].
/// 3. `pn_axis_normal` — margin `‖axis×normal‖·arm`, arm the larger of
///    the would-be circle's radius `|δ/c|·tan α` (δ the apex's distance
///    from the plane, `c = axis·normal`) and `extent`, the reach over
///    which the circle stands in for the section; neither depends on
///    where the plane's origin sits. Zero ⇒
///    [`PlaneConeSection::AxisNormalCircle`], built where the axis meets
///    the plane; definite ⇒ step 4.
/// 4. `pn_conic_type` — the same margin `D`, metered at `extent`, off
///    the apex: Negative ⇒ the plane meets every generator once and the
///    section is [`PlaneConeSection::TiltedEllipse`], however
///    near-circular (step 3 is the one decision of circle against
///    ellipse); Zero ⇒
///    a parabola (`D` taken as zero), Positive ⇒ a hyperbola, each
///    [`PlaneConeSection::OpenConic`]. An in-band `D` (a near-parabola)
///    escalates; it is never snapped to either side.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, the aperture guards, escalations
/// (F6), or a carrier-constructor refusal.
pub fn plane_cone_section<T: Decide>(
    plane: &impl SectionPlane<T>,
    cone: &Surface<T>,
    reach: &Reach<T>,
    band: Band,
) -> Result<PlaneConeSection<T>, SectionError> {
    let Some((q, n)) = plane.site() else {
        return Err(SectionError::WrongLane {
            expected: "plane×cone",
        });
    };
    let &Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref,
    } = cone
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cone",
        });
    };
    // The cone's frame, squared once: nothing holds a stored cone's axis
    // unit or its seam square to it, and the arm reads the axis and
    // half-angle as the geometric cone, so the seam only places azimuth
    // zero ([`square_frame`]).
    let SquareFrame {
        axis: a,
        u_ref: cone_u,
        width: seam_width,
    } = square_frame(axis, u_ref);

    let extent = reach.lever_from(apex);
    let (sin_a, cos_a) = half_angle.sin_cos();
    // The aperture guards (the module's band posture on the cone's
    // convention): the apex lane divides by `sin α·‖a×n‖` and the
    // axis-normal lane by `cos α`, so each clause is decided before
    // either lane runs. They run BEFORE the pose trileans, so a refusal
    // here has classified no pose — `route_pose` reads it as unserved.
    for (name, margin, what) in [
        (
            "pn_aperture_sin",
            sin_a,
            "the cone's half-angle does not definitely open off its axis, so the \
             apex lane's division by sin α is not decided",
        ),
        (
            "pn_aperture_cos",
            cos_a,
            "the cone's half-angle is not definitely under a right angle, so the \
             axis-normal circle's division by cos α is not decided",
        ),
        (
            "pn_seam_off_axis",
            seam_width,
            "the cone's seam does not definitely leave its axis, so where its \
             azimuth zero lies is not decided",
        ),
    ] {
        match decide(name, Margin::levered(margin, extent), band)
            .map_err(SectionError::Escalated)?
        {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => return Err(SectionError::DegenerateOperand { what }),
        }
    }
    let c = a.dot(n);
    let s_vec = a.cross(n);
    let s = s_vec.norm();
    // The conic-type discriminant: its sign is the conic's type off the
    // apex and the generator count through it.
    let discr = sin_a * s - cos_a * c.abs();

    let apex_gap = (apex - q).dot(n);
    if decide("pn_apex_on_plane", Margin::of(apex_gap), band).map_err(SectionError::Escalated)?
        == Sign::Zero
    {
        // Apex lane: generators g(u) = a·cosα + radial(u)·sinα with
        // g·n = 0 ⇔ cos(u − φ) = −cosα·c / (sinα·s). The datum is
        // the discriminant levered at the extent: how far the plane
        // through the apex dips into (or clears) the cone at the
        // extent's generators. The real plane stands `apex_gap` off
        // the apex, which moves its stand-off from every generator
        // point by that much, so `apex_gap` is the swing: the
        // tangent generator is served only where the two together
        // stand inside the zero band, the line pair only where the
        // plane dips in by more than the band past its offset, and
        // the apex point only where it clears by as much.
        let verdict = decide_across(
            ["pn_apex_section", "pn_apex_section_floor"],
            Margin::levered(discr, extent).value(),
            apex_gap,
            band,
        )
        .map_err(SectionError::Escalated)?;
        // Where the real plane's section starts or reaches from the apex
        // (step 2's doc): the apex outcome stands in for it only within
        // the zero band.
        let steep = sin_a * s + cos_a * c.abs();
        // Each read's rounding off the stored `f64`s (step 2): `u` for
        // the offset's difference and `γ₃` for each sum.
        let rounding = |x: Vec3<T>, y: Vec3<T>, per_unit: T| {
            per_unit * (x.x.abs() * y.x.abs() + x.y.abs() * y.y.abs() + x.z.abs() * y.z.abs())
        };
        let gap_err = rounding(apex - q, n, T::from_f64(2.5 * f64::EPSILON));
        let size = apex_gap.abs() + gap_err;
        let (name, bound) = match verdict {
            Sign::Positive => (
                "pn_apex_pair_reach",
                pair_bound(
                    reach,
                    (apex, a, cos_a, sin_a, seam_width),
                    (discr, steep, s),
                    (apex_gap, gap_err),
                    (
                        c,
                        rounding(a, n, T::from_f64(2.0 * f64::EPSILON))
                            + T::from_f64(0.5 * f64::EPSILON) * s,
                    ),
                ),
            ),
            Sign::Zero => (
                "pn_apex_tangent_reach",
                Margin::levered(T::one() / steep, size),
            ),
            Sign::Negative => (
                "pn_apex_point_reach",
                Margin::levered(T::one() / discr.abs(), size),
            ),
        };
        if decide_magnitude(name, bound, band).map_err(SectionError::Escalated)? == Magnitude::Zero
        {
            if verdict == Sign::Negative {
                return Ok(PlaneConeSection::ApexPoint(apex));
            }
            let fan = ApexFan {
                a,
                u: cone_u,
                v: a.cross(cone_u),
                sin_a,
                cos_a,
            };
            let phi = fan.azimuth(n);
            let gen_at = |u: T| -> Curve3<T> {
                Curve3::Line {
                    origin: apex,
                    dir: fan.ruling(u),
                }
            };
            return Ok(if verdict == Sign::Zero {
                // A graze's one root is the extremum of g·n over the
                // azimuth: φ where the normal leans down the axis
                // (c < 0), φ + π where it leans up. The residue's
                // quotient sits up to the band off ∓1 here, and its acos
                // √(2·band) off the extremum. `c` is definite on this
                // arm: D Zero puts |c| at tan α·s, and the aperture
                // guard decided sin α.
                let half_turn = T::pi() * T::from_f64(0.5);
                PlaneConeSection::ApexTangentLine(gen_at(phi + half_turn.copysign(c) + half_turn))
            } else {
                // Clamped acos argument (outward-rounding can push
                // it a hair past ±1 at the tangency boundary —
                // min/max are Real lattice ops, evaluation-legal).
                let delta = ((T::zero() - cos_a * c) / (sin_a * s))
                    .min(T::one())
                    .max(T::zero() - T::one())
                    .acos();
                PlaneConeSection::ApexLinePair {
                    l1: gen_at(phi + delta),
                    l2: gen_at(phi - delta),
                }
            });
        }
    }
    // The section of a plane off the apex, or of one whose section
    // starts or reaches past the band from it (steps 2a–2c): the
    // axis-normal circle, else the conic the tilt makes. A Zero here
    // stands the circle in for the plane's true section, so the tilt's
    // sine is levered at the longest reach over which that stand-in is
    // read: the circle's own radius, and the extent the conic type is
    // metered at. Tilting the plane by `s` about the circle's centre
    // moves it by at most `s` times the reach, so a Zero at the larger
    // of the two keeps every point the section is read at within the
    // zero band. Both are read off the plane's NORMAL, never its stored
    // origin: a plane along the axis (`s = 1`) reads definite at the
    // extent wherever its origin sits, however close it passes the
    // apex.
    //
    // The circle is built where the axis meets the plane, at
    // `t = −δ/c` from the apex (`c` is ±1 within the band here).
    let t = (T::zero() - apex_gap) / c;
    let rim_r = t.abs() * (sin_a / cos_a);
    let arm = rim_r.max(extent);
    match decide("pn_axis_normal", Margin::levered(s, arm), band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero => Ok(PlaneConeSection::AxisNormalCircle(Curve3::Circle {
            center: apex + a * t,
            axis: a,
            radius: rim_r,
            u_ref: cone_u,
        })),
        Sign::Positive | Sign::Negative => {
            let tilt = ConeTilt::new(c, s, (sin_a, cos_a), discr);
            match decide("pn_conic_type", Margin::levered(discr, extent), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Negative => {
                    let major = tilt.half_axis(apex_gap.abs());
                    let minor = apex_gap.abs() * sin_a / tilt.k.sqrt();
                    // The axis-normal trilean above made `s`
                    // definite, so the minor direction is.
                    let u_major = tilted_section_major(n, s_vec);
                    // The centre is the apex's foot on the plane moved
                    // along the major axis: in the plane by
                    // construction, where `apex − (c·a − sin²α·n)·δ/K`
                    // reads a vector that cancels to K along the normal
                    // and then divides it by K.
                    let center = apex - n * apex_gap + u_major * tilt.centre(apex_gap);
                    let e = Curve3::ellipse(center, n, major, minor, u_major, band)?;
                    Ok(PlaneConeSection::TiltedEllipse(e))
                }
                verdict @ (Sign::Zero | Sign::Positive) => {
                    Ok(PlaneConeSection::OpenConic(OpenConicSection::new(
                        if verdict == Sign::Zero {
                            OpenConic::Parabola
                        } else {
                            OpenConic::Hyperbola
                        },
                        (q, n),
                        apex,
                        tilt,
                        tilted_section_major(n, s_vec),
                    )))
                }
            }
        }
    }
}

/// **How far [`PlaneConeSection::ApexLinePair`] stands from the real
/// plane's section over what `reach` consumes** (step 2b): the lesser of
/// the far start `|δ|/D`, read at the apex, and the bound over the
/// consumed span and nappes, `max(t₀ − ρ, 0) + H` per nappe the reach
/// touches, beside the served lines' own rounding. `|δ|` is read as
/// `|δ̂| + e_δ` throughout (step 2), and `gap`, `c` arrive with their
/// rounding `(δ̂, e_δ)`, `(ĉ, e_c)`.
///
/// **The derivation.** Write `c = a·n`, `s = |a×n|`, `D = sin α·s −
/// cos α·|c| > 0` on this lane, `M = sin α·s + cos α·|c|`. A unit
/// generator at azimuth `u` has `g·n = s·sin α·(cos(u − φ) − cos γ)`,
/// `cos γ = −c·cos α/(s·sin α)`; the served lines `L±` lie at `u = φ ± γ`,
/// and `s·sin γ = σ = √(D·M)/sin α`, the sine of the dihedral angle
/// between the plane and the cone along `L±`.
///
/// 1. *A section point's distance from a served line.* The section
///    point on `g` is `X = −δ/(g·n)·g`. `X` and `L+` lie in the plane
///    through `g` and `L+`, which meets the real plane in a line parallel
///    to `L+`, so `dist(X, L+) = |δ|·sin∠(g, L+)/|g·n|`, with
///    `sin∠ ≤ |g − g+| = 2·sin α·|sin(Δu/2)|`. With `w = (u − φ)/2`,
///    `p = |sin(w + γ/2)|`, `q = |sin(w − γ/2)|`: `|g·n| = 2s·sin α·p·q`,
///    `dist(X, L+) ≤ |δ|/(s·p)` and `dist(X, L−) ≤ |δ|/(s·q)`.
/// 2. *Over the consumed points.* A consumed point stands `ρ` or more
///    from the apex, so `p·q ≤ |δ|/(2ρ·s·sin α)`; the two angles differ
///    by `γ`, so `p + q ≥ sin γ`. With `P = max(p, q)`, `Q = min(p, q)`:
///    `P ≥ sin γ − Q ≥ sin γ − p·q/P`, so `P² − sin γ·P +
///    |δ|/(2ρ·s·sin α) ≥ 0`, and `P ≥ sin γ/2` is no less than its
///    smaller root; so `P` clears the larger root wherever the
///    discriminant is non-negative, and
///    `dist(X, pair) ≤ |δ|/(s·P) ≤ H = 2|δ|/(σ + √max(σ² − 2s|δ|/(ρ·sin α), 0))`:
///    `2|δ|/σ` anywhere, the asymptote's offset `|δ|/σ` far out.
/// 3. *A consumed point `τ·g+` of a served line,* on the nappe whose
///    section starts `t₀` from the apex. That nappe's branch has its
///    vertex at its start (`u = φ`, or `φ + π`, where `|g·n|` peaks), and
///    the half-branch from there to `L+`'s asymptote has `p ≥ q`: there
///    `s·p = s·P`, so step 2 bounds its points' distance from `L+` itself
///    by `H`. Its feet on `L+` run from at most `t₀` out to infinity, so
///    one of its points has its foot at `max(|τ|, t₀)`, and the line point
///    stands within `max(t₀ − |τ|, 0) + H ≤ max(t₀ − ρ, 0) + H`. A
///    window holding the apex consumes the apex, which stands the nearer
///    start off. A consumer reads a chord between two points of the
///    window, and the axial coordinate is linear along a segment, so the
///    window is all a served line's consumed points can reach.
/// 4. *The starts.* The nappe `+a` starts `|δ|/M` from the apex where
///    `δ·c < 0`, else `|δ|/D`; the nappe `−a` the other. That sign is
///    read off `δ̂` and `ĉ` only where each clears its rounding
///    (`|δ̂| > e_δ`, `|ĉ| > e_c`); a plane built through the apex reads a
///    `δ̂` that is rounding alone, whose sign is noise. Where either is
///    undecided both nappes take `|δ|/D`, which bounds either start
///    (`D ≤ M`), and step 3 holds for any `t₀` past the true start.
/// 5. *The lines' own rounding*, which near tangency amplifies, read
///    with `u` per operation on unit vectors. The frame is the stored
///    one squared ([`square_frame`]): its unit axis stands `u` off the
///    stored axis' direction, and its seam `τ` off square to it, `τ < 6u`
///    where the seam's `width ≥ 16u` and charged whole (`τ = 1`) below.
///    `ĉ` is `3u` off and `s` `4u` off, each a further `u` for the axis,
///    so `cos γ` is `4u·cot α/s + (5u/s + 6u)·|cos γ|` off; `acos` turns
///    that by `s/σ` (`sin γ = σ/s`) and adds `2u`. `φ`, the `atan2` of
///    the normal's two components across the axis, each `3u` off and
///    together `s` long, is `5u/s` off, and the frame's lean off the
///    stored axis turns it by `(u + τ)·|c|/s` more; the sum and its sine
///    and cosine add `12u`. A line turned `Δ` in azimuth moves `sin α·Δ`
///    per unit of its distance from the apex; its direction leans
///    `τ·sin α` with the seam and `u·cos α` with the axis, and its own
///    sum adds `3u`: so `u·((4·cos α + 5·sin α + 6s·sin α)/σ +
///    (6 + τ/u)·sin α/s + (12 + τ/u)·sin α + 4)` per unit of the
///    farthest consumed point's distance from the apex, `max(|lo|,
///    |hi|)/cos α` over the window.
/// 6. *The window's own rounding* is not charged. Each `(x − apex)·a`
///    is plain arithmetic, as is a caller's `below`/`above`, and is off
///    by about `3u·|x − apex|`. That moves `ρ`, and `start` through it,
///    by about `3u·|x − apex|/cos α`, which is `3u·ρ/cos²α` for a point
///    on the cone. Step 5's term less what it charges the frame, at
///    least `(12·sin α + 3)·u·ρ/cos α`, covers that up to `α ≈ 78°`;
///    the frame's terms charge errors the window rounding is not.
///
/// `ρ` is the least `|(x − apex)·a|` over the reach's window along the
/// axis ([`Reach::range_along`]) over `cos α`, zero where the window
/// holds the apex; a nappe counts where the window reaches past the apex
/// into it. Every term is a `min`, `max`, sum, product or quotient of
/// non-negative lengths, each read through its door, so an `Interval`
/// run encloses the margin and its sign choices hull both arms where
/// undecided. `D > 0` on this lane, so every divisor is positive; `ρ` is
/// floored at the least positive `f64`, which leaves `H` at its `ρ = 0`
/// value `2|δ|/σ`.
#[allow(clippy::many_single_char_names)] // the derivation's own symbols
fn pair_bound<T: Real>(
    reach: &Reach<T>,
    (apex, a, cos_a, sin_a, seam_width): (Point3<T>, Vec3<T>, T, T, T),
    (d, m, s): (T, T, T),
    (gap, gap_err): (T, T),
    (c, c_err): (T, T),
) -> Margin<T> {
    let zero = T::zero();
    let size = gap.abs() + gap_err;
    let far = Margin::levered(T::one() / d, size).value();
    let near = Margin::levered(T::one() / m, size).value();
    let (lo, hi) = reach.range_along(apex, a);
    let rho = lo.max(zero - hi).max(zero) / cos_a;
    // `δ·c < 0` puts the near start on `+a`; read only where both signs
    // clear their rounding (step 4).
    let undecided = (gap.abs() - gap_err).min(c.abs() - c_err);
    let unlike = |when_unlike: T, when_like: T| {
        undecided.select_le_zero(
            far,
            gap.select_le_zero(
                c.select_le_zero(when_like, when_unlike),
                c.select_le_zero(when_unlike, when_like),
            ),
        )
    };
    let (plus, minus) = (unlike(near, far), unlike(far, near));
    let start = hi
        .select_le_zero(zero, (plus - rho).max(zero))
        .max((zero - lo).select_le_zero(zero, (minus - rho).max(zero)))
        .max(lo.max(zero - hi).select_le_zero(near, zero));
    let sigma_sq = d * m / sin_a.powi(2);
    let floor = T::from_f64(f64::MIN_POSITIVE);
    let lean = T::from_f64(2.0) * s * size / (sin_a * rho.max(floor));
    let offset = Margin::levered(
        T::from_f64(2.0) / (sigma_sq.sqrt() + (sigma_sq - lean).max(zero).sqrt()),
        size,
    )
    .value();
    // The seam's tilt `τ/u` (step 5): `6` where `width ≥ 16u`, else whole.
    let tau = (seam_width - T::from_f64(8.0 * f64::EPSILON))
        .select_le_zero(T::from_f64(2.0 / f64::EPSILON), T::from_f64(6.0));
    let turn = (T::from_f64(4.0) * cos_a + T::from_f64(5.0) * sin_a + T::from_f64(6.0) * s * sin_a)
        / sigma_sq.sqrt()
        + (T::from_f64(6.0) + tau) * sin_a / s
        + (T::from_f64(12.0) + tau) * sin_a
        + T::from_f64(4.0);
    let azimuth = Margin::levered(
        T::from_f64(0.5 * f64::EPSILON) * turn,
        lo.abs().max(hi.abs()) / cos_a,
    )
    .value();
    Margin::of(far.min(start + offset) + azimuth)
}

// ---------------------------------------------------------------------
// plane × torus
// ---------------------------------------------------------------------

/// The classified plane×torus section — the axis-aligned
/// configurations' closed forms (the trileans run before any rung, C5;
/// **no fitted chord anywhere in this arm** — every constructible locus
/// is an exact `Circle` or `Spiric`). Everything tilted refuses typed
/// as routed to the general rung; the bitangent (Villarceau) two-circle
/// configuration is deliberately unclassified — a classification no
/// consumer configuration reaches.
#[derive(Clone, Debug)]
pub enum PlaneTorusSection<T: Real> {
    /// Axis-containing plane: the TWO meridian circles — radius `r`,
    /// carrier axis the plane normal, centres `c ± m·R` for
    /// `m = (n × a)`, the in-plane radial; `u_ref` is the torus axis
    /// `a` (in-plane and ⊥ the carrier axis in the decided
    /// configuration). Zero-residual-by-construction against both
    /// implicit forms in ℝ. The `+m`/`−m` assignment is a
    /// deterministic placement (D9), not a verdict.
    MeridianCircles {
        /// The circle centred at `c + m·R`.
        c1: Curve3<T>,
        /// The circle centred at `c − m·R`.
        c2: Curve3<T>,
    },
    /// Axis-parallel plane OFF the axis, short of the inner equator
    /// (`0 < |d| < R − r` for the stand-off `d = n·(q − c)`): the
    /// spiric's TWO ovals, each the [`Curve3::Spiric`]
    /// [`Curve3::spiric`] mints from the torus and the plane's normal. Zero-residual-by-construction
    /// against both implicit forms in ℝ.
    SpiricOvals {
        /// The oval on the `+a × n` side of the plane's trace of the
        /// axis: `u_ref = n`, `offset = d`.
        s1: Curve3<T>,
        /// The oval on the `−a × n` side: `u_ref = −n`, `offset = −d`.
        s2: Curve3<T>,
    },
    /// Axis-normal plane cutting the tube (`|h| < r` for
    /// `h = (q − c)·a`): TWO concentric circles — radii
    /// `R ± √((r−|h|)(r+|h|))`, common centre `c + a·h` on the axis,
    /// carrier axis the torus axis, `u_ref` the torus's own seam
    /// direction as stored: ⊥ the axis only to the torus's at-rest
    /// frame margin (`Surface::representability_margins`, a tilt under
    /// `ε/(R + r)`), so the circles stand within `ε` of their plane.
    /// Zero-residual-by-construction against both implicit forms in ℝ.
    ConcentricCircles {
        /// The outer circle, radius `R + √(r² − h²)`.
        c1: Curve3<T>,
        /// The inner circle, radius `R − √(r² − h²)`.
        c2: Curve3<T>,
    },
    /// Axis-normal plane at `|h| = r`: the tangency circle of radius
    /// `R` — **classification data, not a constructible edge** (the
    /// `TangentLine`/`TangentPoint` lineage, C7: the transversality
    /// margin dies along the whole locus). No production consumer of
    /// this arm exists yet (tests only); when one arrives, the
    /// pattern is the sphere-pair join dispatch's — match the
    /// tangency variant and refuse typed (`topo::boolean/join.rs`'s
    /// "is not a locus" refusal).
    TangentCircle(Curve3<T>),
    /// Axis-normal plane at `|h| > r`: no intersection.
    Empty,
}

/// Classifies and constructs the plane×torus axis-aligned sections.
///
/// Trileans, in order (named lever arms per D4 ¶1):
///
/// 1. The ring convention `R > r > 0`, decided before any
///    classification as TWO inequalities, a half at a time
///    ([`TorusRefusal::require`]): `torus_tube_positive` — margin `r`
///    (meters) — then `ring_torus_convention` — margin `R − r` (meters).
///    `R − r` alone waves through a nonpositive tube radius whenever the
///    difference stays positive (`r = −0.3` against `R = 0.75` has
///    `R − r = 1.05`, and the unguarded arm minted a negative-radius
///    meridian circle). A spindle/horn torus's meridian circles meet or
///    cross on the axis, so no closed form below is well-posed on one;
///    either half's refusal, in band or decided, is
///    [`SectionError::DegenerateTorus`].
/// 2. `pt_axis_in_plane` — margin `(a·n)·extent` (the axis' angle off
///    the plane, metered at the operand extent): Zero ⇒ the axis
///    DIRECTION lies in the plane; then `pt_axis_plane_gap` — margin
///    the stand-off `d = n·(q − c)` (meters, the plane's origin less
///    the centre, read along the normal), read with the meridian
///    circles' lean `r·(a·n)` beside it ([`decide_across`]): Zero ⇒ the plane CONTAINS the
///    axis ⇒ [`PlaneTorusSection::MeridianCircles`], each standing at
///    most that sum off the plane; definite (`pt_axis_plane_gap_floor`) ⇒ the
///    axis-parallel plane OFF the axis, and then `pt_spiric_two_ovals`
///    — margin `(R − r) − |d|` (meters, `d` that same gap), read across
///    the extent with that tilt displacement beside it
///    ([`decide_across`], `pt_spiric_two_ovals_floor` deciding the
///    definite side): Positive ⇒
///    [`PlaneTorusSection::SpiricOvals`], minted through
///    [`Curve3::spiric`]; otherwise the section is a node, a folded
///    loop or empty, and refuses as routed to the general rung.
/// 3. `pt_axis_normal` — margin `‖a×n‖·(R + r)` (the tilt angle's
///    sine, metered at the largest radius a circle of the section can
///    have, which it stands off the plane by that sine times): Zero ⇒
///    the plane is
///    perpendicular to the axis; then `pt_cap_gap` — margin `r − |h|`
///    (meters, `h = d/(a·n)` the depth of the plane's crossing of the
///    axis, `d` the stand-off, so independent of where the plane's
///    origin is stored), read with that tilt displacement beside it
///    ([`decide_across`]); each circle is minted at that crossing and
///    stands at most the levered tilt off the plane:
///    Positive ⇒ [`PlaneTorusSection::ConcentricCircles`], Zero ⇒
///    [`PlaneTorusSection::TangentCircle`] (classification data),
///    Negative ⇒ [`PlaneTorusSection::Empty`].
/// 4. Everything else ⇒ [`SectionError::RoutesToGeneralRung`], with
///    the bitangent (Villarceau) two-circle case NAMED as deliberately
///    unclassified — exactly as the cylinder×cylinder arm names skew
///    and the plane×cone arm names the parabola and the hyperbola.
///
/// The form is `atan2`-free and branch-cut-free by construction, so the
/// `Interval` lane takes it unchanged: there is no lane fork here.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, the ring-convention guard
/// ([`SectionError::DegenerateTorus`], in band or decided),
/// in-band escalations (F6), the general-rung routing refusal, or the
/// spiric constructor's own refusal ([`SectionError::Spiric`]).
pub fn plane_torus_section<T: Decide>(
    plane: &impl SectionPlane<T>,
    torus: &Surface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneTorusSection<T>, SectionError> {
    let wrong = || SectionError::WrongLane {
        expected: "plane×torus",
    };
    let Some((q, n)) = plane.site() else {
        return Err(wrong());
    };
    let &Surface::Torus {
        center: c,
        axis: a,
        major_radius: big_r,
        minor_radius: r,
        u_ref: tor_u,
    } = torus
    else {
        return Err(wrong());
    };

    TorusRefusal::require(big_r, r, band).map_err(SectionError::DegenerateTorus)?;

    // The axis tilt is levered at the extent for the fork alone. Each
    // served row below decides the displacement it puts on its minted
    // curves beside its own position datum, so no served circle leans on
    // the extent being long.
    let tilt = Margin::levered(a.dot(n), extent);
    match decide("pt_axis_in_plane", tilt, band).map_err(SectionError::Escalated)? {
        Sign::Zero => {
            // The axis direction lies in the plane: containing vs
            // offset, by the stand-off `d`. A meridian circle is minted
            // on the torus about its centre, which stands `d` off the
            // plane, and its points lean off the plane by up to
            // `r·|a·n|` more, so the two are decided as one sum. An oval
            // is minted in a plane that the tilt moves off the real one
            // by up to the levered tilt, read beside its own gap.
            let d = n.dot(q - c);
            match decide_across(
                ["pt_axis_plane_gap", "pt_axis_plane_gap_floor"],
                d,
                r * a.dot(n),
                band,
            )
            .map_err(SectionError::Escalated)?
            {
                Sign::Zero => {
                    // The plane contains the axis: the two meridian
                    // circles. `n ⊥ a` in the decided configuration,
                    // so `n × a` is unit up to rounding; normalized
                    // for the frame all the same.
                    let m = n.cross(a).normalize();
                    let circle_at = |center: Point3<T>| Curve3::Circle {
                        center,
                        axis: n,
                        radius: r,
                        u_ref: a,
                    };
                    Ok(PlaneTorusSection::MeridianCircles {
                        c1: circle_at(c + m * big_r),
                        c2: circle_at(c - m * big_r),
                    })
                }
                Sign::Positive | Sign::Negative => {
                    // Off the axis: the spiric, two ovals while the
                    // plane stays short of the inner equator — a
                    // length, decided before any root is taken.
                    match decide_across(
                        ["pt_spiric_two_ovals", "pt_spiric_two_ovals_floor"],
                        (big_r - r) - d.abs(),
                        tilt.value(),
                        band,
                    )
                    .map_err(SectionError::Escalated)?
                    {
                        Sign::Positive => {}
                        Sign::Zero | Sign::Negative => {
                            return Err(SectionError::RoutesToGeneralRung {
                                pair: "plane×torus",
                                why: "an axis-parallel plane at or past the torus's inner \
                                      equator cuts a node, one folded loop or nothing — not \
                                      the two ovals the spiric carries — and the pair's \
                                      general-rung arm has not retired (arms retire one at \
                                      a time, each with its proof)",
                            });
                        }
                    }
                    // `(a, n, d)` names the oval on the `+a × n` side
                    // of the plane's trace, and `(a, −n, −d)` the other
                    // (the variant's docs).
                    let oval = |u: Vec3<T>, offset: T| {
                        Curve3::spiric(c, a, u, big_r, r, offset, band).map_err(|e| match e {
                            geom::SpiricInvalid::Escalated(diag) => SectionError::Escalated(diag),
                            other => SectionError::Spiric(other),
                        })
                    };
                    Ok(PlaneTorusSection::SpiricOvals {
                        s1: oval(n, d)?,
                        s2: oval(-n, -d)?,
                    })
                }
            }
        }
        Sign::Positive | Sign::Negative => {
            let tilt = Margin::levered(a.cross(n).norm(), big_r + r);
            match decide("pt_axis_normal", tilt, band).map_err(SectionError::Escalated)? {
                Sign::Zero => {
                    // The plane is perpendicular to the axis: the
                    // concentric-circle lane, by the depth `h` of the
                    // plane's crossing of the axis, wherever its origin
                    // is stored. The circles are minted there, so a
                    // circle of radius `ρ ≤ R + r` stands at most
                    // `ρ·‖a×n‖`, under the levered tilt, off the plane,
                    // and the plane's depth into the tube moves by at
                    // most as much across them: the cap gap is decided
                    // beside it.
                    let h = n.dot(q - c) / a.dot(n);
                    let center = c + a * h;
                    match decide_across(
                        ["pt_cap_gap", "pt_cap_gap_floor"],
                        r - h.abs(),
                        tilt.value(),
                        band,
                    )
                    .map_err(SectionError::Escalated)?
                    {
                        Sign::Positive => {
                            // The interval-square tripwire does not
                            // bite: both factors are definitely
                            // positive after the trilean (never a
                            // spuriously negative bracket under a
                            // sqrt).
                            let w = ((r - h.abs()) * (r + h.abs())).sqrt();
                            let circle_at = |radius: T| Curve3::Circle {
                                center,
                                axis: a,
                                radius,
                                u_ref: tor_u,
                            };
                            Ok(PlaneTorusSection::ConcentricCircles {
                                c1: circle_at(big_r + w),
                                c2: circle_at(big_r - w),
                            })
                        }
                        Sign::Zero => Ok(PlaneTorusSection::TangentCircle(Curve3::Circle {
                            center,
                            axis: a,
                            radius: big_r,
                            u_ref: tor_u,
                        })),
                        Sign::Negative => Ok(PlaneTorusSection::Empty),
                    }
                }
                Sign::Positive | Sign::Negative => Err(SectionError::RoutesToGeneralRung {
                    pair: "plane×torus",
                    why: "generic tilt cuts a spiric quartic — the bitangent \
                          (Villarceau) two-circle configuration is deliberately \
                          unclassified, a third classification no consumer \
                          configuration reaches — and the pair's general-rung arm \
                          has not retired (arms retire one at a time, each with its \
                          proof)",
                }),
            }
        }
    }
}

// ---------------------------------------------------------------------
// cone × cylinder, the coaxial configuration only
// ---------------------------------------------------------------------

/// The classified cone×cylinder section — the one exact-degenerate
/// configuration's closed form (rung 1: the trileans run before any
/// rung, C5; **no fitted chord anywhere in this arm** — both loci are
/// exact `Circle`s). Every other pose cuts a QUARTIC and refuses typed
/// as routed to the general rung.
///
/// There is deliberately no tangency variant: a coaxial cylinder meets
/// each nappe transversally at every half-angle in the cone's own
/// convention `α ∈ (0, π/2)`, so the `TangentCircle`/`TangentPoint`
/// classification-data lineage has nothing to classify here. That is
/// what makes this arm strictly simpler than [`plane_cone_section`].
#[derive(Clone, Debug)]
pub enum ConeCylinderSection<T: Real> {
    /// Coaxial cylinder: ONE circle per nappe — both of the CYLINDER's
    /// radius `R`, both centred on the shared axis at
    /// `apex ± axis·(R·cot α)`, carrier axis the CONE's axis and
    /// `u_ref` the cone's own seam direction, squared to the axis
    /// ([`geom::square_frame`]). Zero-residual-by-construction against
    /// both implicit forms in ℝ.
    ///
    /// The `+`/`−` assignment is the cone's own nappe convention, not a
    /// verdict: `c1` is on the `v > 0` nappe (the one opening along
    /// `axis`) and `c2` on its mirror.
    CoaxialCircles {
        /// The circle on the `v > 0` nappe, centred at `apex + a·R·cot α`.
        c1: Curve3<T>,
        /// The circle on the mirror nappe, centred at `apex − a·R·cot α`.
        c2: Curve3<T>,
    },
}

/// Classifies and constructs the cone×cylinder coaxial section.
///
/// Trileans, in order (named lever arms per D4 ¶1):
///
/// **`extent` is measured from the apex**: the farthest the consumed
/// region stands from it, which also bounds where the minted circles
/// may stand (`coc_station_reach`). Every row reads the pose at the
/// apex and levers its angles from there: the axes stand within
/// `d ± θ·extent` of each other across the region, `d` the apex's
/// distance from the cylinder's axis and `θ` the tilt
/// `coc_axes_parallel` admits, and `coc_coaxial` decides that sum.
///
/// **A caller that knows the extent only from both sides hands both**
/// ([`LeverBounds`]; a plain length is exact), and each row reads the
/// side on which a wrong length only refuses. The rows that act on
/// their Zero, `coc_axes_parallel` and `coc_coaxial`'s swing
/// (`separation_range` over the extent), read the bound from above: a
/// longer arm only reads more tilt and more swing. The rows that act on
/// a Positive, the three convention guards and `coc_station_reach`'s
/// admission, read the bound from below: a longer arm would certify a
/// guard and admit a station the region does not reach.
///
/// 1. `coc_cylinder_radius` — margin `R` (meters): the arm states both
///    circles at exactly that radius, so it must be a positive length.
/// 2. `coc_aperture_sin` and `coc_aperture_cos`, each metered at
///    `extent` — the two clauses of the cone's own convention
///    `α ∈ (0, π/2)`, asked as separate questions (the
///    `CylinderSpherePose::decide` radius-guard shape). **Neither is the admission
///    criterion**: `sin α` is decided because the station DIVIDES by
///    it, and `cos α` because `cot α`'s sign is what puts `c1` on the
///    nappe this arm's docs promise. Both refuse
///    [`SectionError::DegenerateOperand`].
/// 3. `coc_axes_parallel` — margin `‖a×b‖·extent` (the axes' angle
///    off parallel, metered at the operand extent): definite ⇒ the
///    general-rung refusal, a tilted cylinder cutting a quartic. Zero
///    covers the antiparallel pose too, which is the same
///    configuration read through the cylinder's opposite orientation.
/// 4. `coc_coaxial` — margin the apex's distance from the cylinder's
///    axis, `‖(apex − o) − b·((apex − o)·b)‖` (meters), read across the
///    extent with the axes' distance's range over it and the circles'
///    turn beside it ([`decide_across`]): Zero ⇒ coaxial, the circles
///    standing at most that sum off the cylinder; `coc_coaxial_floor` definite ⇒ the
///    general-rung refusal, a parallel-but-OFFSET cylinder cutting a
///    quartic. A norm is never negative, so the floor has one live
///    definite verdict by construction.
/// 5. `coc_station_reach` — margin `extent − |R·cot α|` (meters), the
///    ADMISSION criterion and the last decision before the mint:
///    Positive ⇒ [`ConeCylinderSection::CoaxialCircles`], otherwise
///    [`SectionError::BeyondOperandExtent`]. **Why the station and not
///    the angle.** Each centre carries the absolute error of
///    `R·cot α`, which diverges as the half-angle closes, whereas an
///    angular guard levered at `extent` LOOSENS as the operands grow —
///    so metering the aperture alone would let a bigger operand admit
///    a worse mint (measured: `α = 1e-10`, `extent = 100`, `ε = 1e-9`
///    put a "zero-residual" circle ~101 ε off BOTH surfaces). Inside
///    the caller's own reach the position error is `O(ε · extent)`,
///    which is what makes the exactness claim above a statement about
///    this arm.
///
/// The axis distance is decided here by its margin, as every pose
/// question of an exact-degenerate arm is (`pt_axis_plane_gap`,
/// `pn_apex_on_plane`, `cs_coaxial`): where a surface stands relative
/// to another's frame, at the committed tolerance, with the in-band
/// case escalating rather than being guessed.
///
/// The form is `atan2`-free and branch-cut-free by construction, so the
/// `Interval` lane takes it unchanged: there is no lane fork here.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, the convention guards, the
/// extent refusal, in-band escalations (F6), or the general-rung
/// routing refusal.
pub fn cone_cylinder_section<T: Decide>(
    cone: &Surface<T>,
    cyl: &Surface<T>,
    extent: impl Into<LeverBounds<T>>,
    band: Band,
) -> Result<ConeCylinderSection<T>, SectionError> {
    let extent = extent.into();
    let (below, above) = (extent.below(), extent.above());
    let wrong = || SectionError::WrongLane {
        expected: "cone×cylinder (cone first)",
    };
    let &Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref,
    } = cone
    else {
        return Err(wrong());
    };
    // The cone's frame, squared once ([`square_frame`]).
    let SquareFrame {
        axis: a,
        u_ref: cone_u,
        width: seam_width,
    } = square_frame(axis, u_ref);
    let &Surface::Cylinder {
        origin: o,
        axis: b,
        radius: big_r,
        ..
    } = cyl
    else {
        return Err(SectionError::WrongLane {
            expected: "cone×cylinder (cylinder second)",
        });
    };

    let (sin_a, cos_a) = half_angle.sin_cos();
    match decide("coc_cylinder_radius", Margin::of(big_r), band).map_err(|diag| {
        SectionError::RadiusEscalated {
            radius: SectionRadius::Cylinder,
            diag,
        }
    })? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(SectionError::DegenerateOperand {
                what: "the cylinder's radius is not definitely positive, and this arm \
                       states both circles at exactly that radius",
            });
        }
    }
    // **These three are NOT the admission criterion**; `coc_station_reach`
    // below is. `coc_aperture_sin` exists because the station DIVIDES by
    // `sin α` and a division needs its divisor decided; `coc_aperture_cos`
    // because a half-angle at or past a right angle turns `cot α`'s sign
    // and with it the `c1`/`c2` nappe assignment this arm documents, so
    // it is the cone convention's own clause rather than a conditioning
    // question; `coc_seam_off_axis` because the circles' seam is the
    // cone's squared to the axis. All three meter through the operand extent, the lever every
    // angular margin here takes.
    for (name, margin, what) in [
        (
            "coc_aperture_sin",
            sin_a,
            "the cone's half-angle does not definitely open off its axis, so the \
             station's division by sin α is not decided",
        ),
        (
            "coc_aperture_cos",
            cos_a,
            "the cone's half-angle is not definitely under a right angle, so cot α's \
             sign — and with it which nappe each circle is on — is not decided",
        ),
        (
            "coc_seam_off_axis",
            seam_width,
            "the cone's seam does not definitely leave its axis, so the circles' \
             azimuth zero is not decided",
        ),
    ] {
        match decide(name, Margin::levered(margin, below), band).map_err(SectionError::Escalated)? {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => return Err(SectionError::DegenerateOperand { what }),
        }
    }

    match decide(
        "coc_axes_parallel",
        Margin::levered(a.cross(b).norm(), above),
        band,
    )
    .map_err(SectionError::Escalated)?
    {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(SectionError::RoutesToGeneralRung {
                pair: "cone×cylinder",
                why: "a cylinder tilted off the cone's axis cuts a QUARTIC, not a \
                      circle, and the pair's general-rung arm has not retired — the \
                      cone's composite has no meters conversion (arms retire one at a \
                      time, each with its proof)",
            });
        }
    }

    // The axes are parallel: coaxial or merely parallel, by the apex's
    // distance from the CYLINDER's axis, read at the pivot every row
    // here is levered from. Against its own axis the cylinder's origin
    // may stand anywhere on it. A point of a circle minted at `R` about
    // the cone's axis, at station `s` from the apex, stands off the
    // cylinder by at most the cone's axis point's distance from the
    // cylinder's, `|u + s·τ|` (`separation_range`, read exactly over
    // `|s| ≤ extent`), plus the circle's own turn off the cylinder's
    // cross-section, `R·(1 − cos θ) ≤ R·sin²θ`. The swing is that range's
    // reach from `d`, plus the turn.
    let d = (apex - o).reject_from(b).norm();
    let (near, far) = separation_range((apex, a), (o, b), above);
    let swing = (far - d).abs().max((d - near).abs()) + big_r * a.cross(b).norm().powi(2);
    match decide_across(["coc_coaxial", "coc_coaxial_floor"], d, swing, band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero => {
            // Coaxial. On the cone `S(u, v) = apex + a·(v·cos α) +
            // radial(u)·(v·sin α)`, so the circle of radius `R` sits at
            // `v = ±R/sin α`, i.e. at station `±R·cot α` along the
            // axis. `sin α` is definitely positive by the guard above,
            // so the division is decided.
            let station = big_r * (cos_a / sin_a);
            // **The admission criterion, and the reason it is the
            // STATION rather than the angle.** Every centre this arm
            // mints carries the ABSOLUTE error of `R·cot α`, which grows
            // without bound as the half-angle closes — while an angular
            // guard levered at `extent` gets LOOSER as the operands get
            // larger, so a bigger operand would admit a worse mint. A
            // station the caller's own reach does not cover is refused
            // instead: inside it the position error is O(ε·extent),
            // which is what makes the zero-residual claim above a claim
            // about the arm and not about the fixture.
            match decide("coc_station_reach", Margin::of(below - station.abs()), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(SectionError::BeyondOperandExtent {
                        what: "the coaxial cone×cylinder circles stand at ±R·cot α from \
                               the apex, which is not definitely inside the extent the \
                               caller metered against",
                    });
                }
            }
            let circle_at = |center: Point3<T>| Curve3::Circle {
                center,
                axis: a,
                radius: big_r,
                u_ref: cone_u,
            };
            Ok(ConeCylinderSection::CoaxialCircles {
                c1: circle_at(apex + a * station),
                c2: circle_at(apex - a * station),
            })
        }
        Sign::Positive | Sign::Negative => Err(SectionError::RoutesToGeneralRung {
            pair: "cone×cylinder",
            why: "a cylinder parallel to the cone's axis but OFF it cuts a QUARTIC, \
                  not a circle, and the pair's general-rung arm has not retired — the \
                  cone's composite has no meters conversion (arms retire one at a time, \
                  each with its proof)",
        }),
    }
}

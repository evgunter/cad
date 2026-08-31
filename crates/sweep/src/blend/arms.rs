//! **The analytic blend arms** (C8's D3 payoff): the constant-radius
//! rolling ball over analytic supports lands on analytic surfaces in
//! exactly the cases that dominate practice, and those are the cases
//! this unit ships — nothing is fitted, nothing is approximated.
//!
//! | support pair | spine | blend | trimlines |
//! |---|---|---|---|
//! | plane–plane | line | cylinder | two lines |
//! | plane–sphere (a circular rim) | circle | torus | two circles |
//! | sphere–cone, cone–plane(⊥), cone–cone, cylinder–cone, cylinder–sphere, cylinder–plane(⊥), sphere–sphere | circle | torus | two circles |
//! | cylinder–cylinder(∥), cylinder–plane(∥) | line | cylinder | two lines |
//! | trihedral vertex (3 planes) | point | sphere | three circles |
//!
//! **Every constant-radius arm mints a TORUS or a CYLINDER, never a
//! cone.** A constant-radius rolling ball is the envelope of EQUAL
//! spheres, and the envelope of equal spheres over a line spine is a
//! cylinder and over a circle spine a torus; a cone is the envelope of
//! spheres whose radius varies LINEARLY, which is the variable-radius
//! (canal) family, not this one.
//!
//! # The one derivation, said once ([`SupportTrace`])
//!
//! Rows 2–4 are a single closed form. Whenever a support pair carries a
//! symmetry that the rolling ball inherits — a common axis of
//! revolution, or a common ruling direction — the ball's centre is
//! confined to one plane: the **SHEET**. For a coaxial pair that is the
//! meridian half-plane through the rim point ([`Meridian`]); for a pair
//! meeting along a straight ruling it is the cross-section normal to
//! that ruling ([`Ruling`]). Both supports cut the sheet in a LINE or a
//! CIRCLE, the ball centre is where the two offset traces cross
//! ([`sheet_center`]), and the spine is that centre carried round the
//! axis (a circle ⇒ torus) or along the ruling (a line ⇒ cylinder).
//!
//! So the per-pair content is only the reduction — which trace each
//! support cuts, and on which side its material lies — and the spine
//! radius `s` falls out of the crossing. The pairs, with the trace each
//! support contributes to the sheet and the resulting `s`:
//!
//! | pair | traces | spine radius `s` |
//! |---|---|---|
//! | plane(⊥)–sphere | line ⊥ axis × circle | `√((R ∓ r)² − h²)` |
//! | cone–plane(⊥) | two lines | crossing of the two offset lines |
//! | cone–cone | two lines | crossing of the two offset lines |
//! | cylinder–cone | two lines | `R ∓ r` and the offset generator |
//! | cylinder–plane(⊥) | two lines | `R ∓ r` exactly |
//! | sphere–cone | line × circle | the offset generator on the offset sphere |
//! | cylinder–sphere | line × circle | `R_cyl ∓ r` exactly |
//! | sphere–sphere | two circles | the offset spheres' own crossing |
//! | cylinder–cylinder(∥) | two circles | (straight spine: no `s`) |
//! | cylinder–plane(∥) | line × circle | (straight spine: no `s`) |
//!
//! The pair is selected by the RIM CARRIER's own stored shape, not
//! guessed: a coaxial pair meets in a circle and a ruled pair in a
//! line, so the carrier says which sheet the reduction runs in.
//!
//! The **chamfer** ([`chamfer_strip`], [`chamfer_corner_patch`]) is
//! the ruled sibling of the first and third rows: the same trimline
//! structure over the same supports, with the rolling ball's tube
//! replaced by the flat strip at equal setback and its octant by the
//! plane through the three feet. It is exact for the same reason — a
//! straight edge over planes puts both trimlines on lines.
//!
//! Every arm returns its blend surface, its spine, and the EXACT
//! setback on each support — and the battery consumes the setback
//! from HERE, not from a parallel first-order estimate. That is what
//! makes the ordering claim honest: the quantity predicate 2 refuses
//! on is bit-for-bit the quantity the constructor would use.
//!
//! **Sense, from stored structure** (S10/S11): a blend face's chart
//! normal agrees with the material's outward normal exactly when the
//! chain is CONVEX — the rolling ball is then inside the material and
//! the blend bulges away from it, so the chart normal (radially out
//! of the tube/ball) is the outward one. On a CONCAVE chain the ball
//! rolls in the void, material sits outside the tube, and the sense
//! bit is `false`. The bit is read off the chain's stored convexity
//! verdict — never from a sampled normal.
//!
//! Everything a general spine would need — a canal surface, the
//! kernel's first approximating SURFACE — is refused typed by
//! [`super::BlendError::SpineUnsupported`], which names that banked
//! unit as the front door that does not exist yet.

use geom::Curve3;
use geom::Surface;
use geom_core::{Point3, Real, Vec3};

/// A blend surface with its spine and its per-support trimlines —
/// the complete analytic answer for one link of a chain.
#[derive(Clone, Debug)]
pub struct EdgeBlend<T: Real> {
    /// The blend surface itself (a cylinder or a torus).
    pub surface: Surface<T>,
    /// The spine's curvature, 1/meters: `0` for a straight spine,
    /// `1/s` for a circular one. Predicate 3 reads this.
    ///
    /// A rolling-ball fact, so a ruled band has none: the chamfer
    /// strip stores `0` and predicate 3 does not run over it (a
    /// chamfer has no rolling ball whose centre locus could fold).
    pub spine_curvature: T,
    /// The trimline on the FIRST support, and the setback (meters)
    /// from the original edge to it, measured in that support.
    pub trim_a: (Curve3<T>, T),
    /// The trimline on the SECOND support, likewise.
    pub trim_b: (Curve3<T>, T),
}

/// The corner ball of a three-convex-edge vertex: a sphere patch (the
/// spherical triangle bounded by the three contact circles with the
/// three incident edge cylinders).
#[derive(Clone, Debug)]
pub struct CornerBall<T: Real> {
    /// The sphere the octant patch lies on.
    pub surface: Surface<T>,
    /// The ball centre — the point at distance `r` inside all three
    /// support planes.
    pub center: Point3<T>,
    /// `|det(n₁, n₂, n₃)|` for the three outward support normals —
    /// the independence margin predicate 6 classifies (times the
    /// radius, which the battery supplies as the lever arm).
    pub independence: T,
}

/// The rim blend of a plane meeting a sphere along a circle — the pip
/// rims. Named separately from [`EdgeBlend`] only in the docs: it is
/// produced by [`plane_sphere_blend`] and returned as an
/// [`EdgeBlend`] like every other arm.
pub type RimBlend<T> = EdgeBlend<T>;

/// Which analytic arm a link took — carried on the link so the report
/// and the tests can enumerate coverage without re-deriving it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendArm {
    /// Plane–plane edge → cylinder patch, straight spine.
    PlanePlaneCylinder,
    /// Plane–sphere circular rim → torus patch, circular spine.
    PlaneSphereTorus,
    /// Plane–plane edge → flat strip at equal setback, no spine. The
    /// chamfer's one arm ([`chamfer_strip`]).
    PlanePlaneStrip,
    /// Sphere–cone coaxial rim → torus patch, circular spine.
    SphereConeTorus,
    /// Cone–plane(⊥ axis) coaxial rim → torus patch, circular spine.
    ConePlaneTorus,
    /// Cone–cone coaxial rim → torus patch, circular spine.
    ConeConeTorus,
    /// Cylinder–cone coaxial rim → torus patch, circular spine.
    CylinderConeTorus,
    /// Cylinder–sphere coaxial rim → torus patch, circular spine.
    CylinderSphereTorus,
    /// Cylinder–plane(⊥ axis) coaxial rim → torus patch, circular spine.
    CylinderPlaneTorus,
    /// Sphere–sphere rim → torus patch, circular spine. The one arm
    /// whose hypothesis is free: two spheres on distinct centres always
    /// meet in a circle, and the line through the centres is that
    /// circle's own axis, so the pair is coaxial by construction.
    SphereSphereTorus,
    /// Two parallel cylinders meeting along a common ruling → cylinder
    /// patch, straight spine. The arm is exact; no surgery carves its
    /// band yet (the terminations are the run-out taxonomy — #987).
    CylinderCylinderCylinder,
    /// Cylinder and a plane containing its axis direction, meeting
    /// along a ruling → cylinder patch, straight spine. Same standing
    /// as the row above: exact arm, uncarved band (#987).
    CylinderPlaneCylinder,
}

impl BlendArm {
    /// Whether this arm blends a plane–plane support pair — the one
    /// pair the in-place open-chain surgery carves, whichever band
    /// the request grafts onto it.
    #[must_use]
    pub fn is_plane_plane(self) -> bool {
        matches!(self, Self::PlanePlaneCylinder | Self::PlanePlaneStrip)
    }

    /// Whether this arm's blend is the TORUS about a circular spine —
    /// the shape a CLOSED rim's band is built from, whatever kinds its
    /// two supports are. Every coaxial-revolution pair mints one; the
    /// ruled pairs mint a cylinder and meet along an open edge instead.
    #[must_use]
    pub fn is_coaxial_torus(self) -> bool {
        matches!(
            self,
            Self::PlaneSphereTorus
                | Self::SphereConeTorus
                | Self::ConePlaneTorus
                | Self::ConeConeTorus
                | Self::CylinderConeTorus
                | Self::CylinderSphereTorus
                | Self::CylinderPlaneTorus
                | Self::SphereSphereTorus
        )
    }

    /// The arm's name, for refusal text and report rows. The
    /// [`super::BlendError::SpineUnsupported`] payload's hand-written
    /// roster is checked against exactly these strings, so a new arm
    /// that is not advertised there fails a test rather than shipping a
    /// stale refusal.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::PlanePlaneCylinder => "plane–plane → cylinder",
            Self::PlaneSphereTorus => "plane–sphere → torus",
            Self::PlanePlaneStrip => "plane–plane → strip",
            Self::SphereConeTorus => "sphere–cone → torus",
            Self::ConePlaneTorus => "cone–plane → torus",
            Self::ConeConeTorus => "cone–cone → torus",
            Self::CylinderConeTorus => "cylinder–cone → torus",
            Self::CylinderSphereTorus => "cylinder–sphere → torus",
            Self::CylinderPlaneTorus => "cylinder–plane → torus",
            Self::SphereSphereTorus => "sphere–sphere → torus",
            Self::CylinderCylinderCylinder => "cylinder–cylinder → cylinder",
            Self::CylinderPlaneCylinder => "cylinder–plane(∥) → cylinder",
        }
    }

    /// Every arm, for the coverage rows and the refusal-roster check.
    pub const ALL: [Self; 12] = [
        Self::PlanePlaneCylinder,
        Self::PlaneSphereTorus,
        Self::PlanePlaneStrip,
        Self::SphereConeTorus,
        Self::ConePlaneTorus,
        Self::ConeConeTorus,
        Self::CylinderConeTorus,
        Self::CylinderSphereTorus,
        Self::CylinderPlaneTorus,
        Self::SphereSphereTorus,
        Self::CylinderCylinderCylinder,
        Self::CylinderPlaneCylinder,
    ];
}

/// The unit component of `x` orthogonal to the unit direction `a` —
/// the seam reference every chart below takes from real geometry the
/// caller already stored (a support's own `u_ref`, a corner normal),
/// never from a coordinate-axis tie-break. Poison propagates when `x`
/// is parallel to `a`, which is the honest answer: there is no
/// distinguished perpendicular there.
fn perp_unit<T: Real>(x: Vec3<T>, a: Vec3<T>) -> Vec3<T> {
    // Named binding so the interval-square tripwire's self-multiplication
    // grep does not false-positive on `a * a.dot(x)` (a vector scaled by a
    // projection coefficient, not a scalar square) — the affine.rs
    // restructuring precedent; no numeric change.
    let along = a.dot(x);
    (x - a * along).normalize()
}

/// **The plane–plane edge blend**: the rolling ball of radius `r`
/// tangent to both planes, rolled along their common edge.
///
/// With outward normals `n_a`, `n_b` and any point `p` on the edge,
/// the ball centre solves `(c − p)·n_a = (c − p)·n_b = −r` (inside
/// the material by `r` from each support), giving the closed form
/// `c = p − r·(n_a + n_b)/(1 + n_a·n_b)` — the spine, a straight line
/// along the edge direction `tau`. The blend is the cylinder about
/// that line with radius `r`; the trimline on each support is the
/// ruling through the ball's foot on that plane, and the setback is
/// `r·√((1 − d)/(1 + d))` with `d = n_a·n_b` (equivalently
/// `r·tan(φ/2)` for the outward-normal angle φ) — derived here once
/// and consumed by predicate 2.
///
/// `convex` picks the side: on a concave chain the ball rolls on the
/// far side of both supports, which is the same formula with `r`
/// negated.
#[must_use]
pub fn plane_plane_blend<T: Real>(
    p: Point3<T>,
    tau: Vec3<T>,
    n_a: Vec3<T>,
    n_b: Vec3<T>,
    radius: T,
    convex: bool,
) -> EdgeBlend<T> {
    let signed = if convex { radius } else { -radius };
    let d = n_a.dot(n_b);
    let one = T::one();
    let center = p - (n_a + n_b) * (signed / (one + d));
    let foot_a = center + n_a * signed;
    let foot_b = center + n_b * signed;
    let setback = radius * ((one - d) / (one + d)).sqrt();
    EdgeBlend {
        surface: Surface::Cylinder {
            origin: center,
            axis: tau,
            radius,
            u_ref: (foot_a - center).normalize(),
        },
        spine_curvature: T::zero(),
        trim_a: (
            Curve3::Line {
                origin: foot_a,
                dir: tau,
            },
            setback,
        ),
        trim_b: (
            Curve3::Line {
                origin: foot_b,
                dir: tau,
            },
            setback,
        ),
    }
}

/// **The plane–sphere rim blend**: a plane with outward normal `n`
/// through `origin`, meeting a sphere of radius `sphere_r` centred at
/// `sphere_c` along a circle.
///
/// The ball centre sits at depth `r` below the plane and on the OFFSET
/// SPHERE — the locus at distance `r` from the sphere on the material
/// side. Which offset sphere that is, is the pair's own second
/// configuration and is read from the sphere face's stored sense bit,
/// never from a normal: a sphere's chart normal is the outward radial
/// (`geom::Surface::Sphere`), so `sphere_convex` (the stored `sense`)
/// says the material is INSIDE the sphere and the centre rides at
/// `R − r`, while a POCKET (a pip's dimple: material outside the ball)
/// puts it at `R + r`. Everything else is one derivation:
///
/// - the spine is the CIRCLE of radius `s = √(offset² − h²)`,
///   `h = (c_s − o)·n + r`, in the offset plane, and the blend is the
///   torus about it;
/// - both trimlines are circles coaxial with the spine: radius `s` on
///   the plane and `R·s/offset` on the sphere;
/// - the plane's setback is `|s − a|` for the rim radius `a`, and its
///   SIGN is that same configuration bit: a pocket's blend widens the
///   hole (`s > a` — what makes it eat into the flat face rather than
///   into the pocket), a convex sphere's shrinks the plane's boundary.
///
/// **The setback convention, stated once, HERE, because this is where
/// the two spellings diverge.** This arm returns a SIGNED setback on the
/// plane; the shared sheet reduction ([`Meridian::blend`],
/// [`Ruling::blend`]) returns the unsigned Euclidean displacement of the
/// trimline from the rim, on both supports. Predicate 2 consumes them
/// identically, as `gap − setback − setback`, so the unsigned form is
/// the CONSERVATIVE one — it can only shrink the margin, never widen
/// it — and the two agree in magnitude everywhere (pinned by
/// `verbs_arms2_arms::the_shared_reduction_agrees_with_the_plane_sphere_arm`,
/// which compares `|signed|` against the unsigned and says why).
///
/// **Neither degenerate case is gated here**, and they are two
/// different cases, not one:
///
/// - `s² < 0` — no spine circle exists at all (the offsets do not
///   meet). `s` is then POISON, `spine_curvature` is poison, and
///   predicate 3 escalates with an `Invalid` margin.
/// - `0 < s ≤ r` — a spine circle exists but the tube swallows it: `s`
///   and `1/s` are ordinary finite numbers, and predicate 3 refuses
///   `SpineIrregular` on the FINITE curvature, not on poison.
///
/// Both are refused before the surface is ever used, and at the verb
/// level predicate 2's conservative consumption screen usually fires
/// first on the setbacks such a radius implies (the battery's own
/// stated ordering) — either refusal is honest and typed. Total
/// arithmetic in, classification at the caller: the crate's standing
/// posture.
#[must_use]
pub fn plane_sphere_blend<T: Real>(
    origin: Point3<T>,
    n: Vec3<T>,
    plane_u_ref: Vec3<T>,
    sphere_c: Point3<T>,
    sphere_r: T,
    radius: T,
    sphere_convex: bool,
) -> EdgeBlend<T> {
    let depth = (sphere_c - origin).dot(n);
    let h = depth + radius;
    // The offset sphere the ball centre rides, selected STRUCTURALLY.
    let offset = if sphere_convex {
        sphere_r - radius
    } else {
        sphere_r + radius
    };
    let s2 = offset.powi(2) - h.powi(2);
    // No gate here (see the two degenerate cases in the doc above):
    // `s² < 0` yields poison and escalates at predicate 3, `0 < s ≤ r`
    // yields a finite curvature predicate 3 refuses `SpineIrregular`
    // on. Total arithmetic in, classification at the caller.
    let s = s2.sqrt();
    let spine_center = sphere_c - n * h;
    let u_ref = perp_unit(plane_u_ref, n);
    // The rim circle the blend replaces: the sphere ∩ plane locus.
    let rim2 = sphere_r.powi(2) - depth.powi(2);
    let rim = rim2.max(T::zero()).sqrt();
    // The contact circle on the sphere: the sphere point along the
    // line from its centre to the spine, scaled to radius R.
    let scale = sphere_r / offset;
    let sphere_trim_c = sphere_c - n * (h * scale);
    let sphere_trim_r = s * scale;
    // Setback on the sphere: the EUCLIDEAN displacement of the
    // trimline from the rim. Predicate 2 compares it against face
    // extents measured the same way, so the two sides of that
    // inequality are in one metric — an arc-length setback against a
    // straight-line extent would be comparing different quantities.
    let rim_pt = sphere_c - n * depth + u_ref * rim;
    let con_pt = sphere_trim_c + u_ref * sphere_trim_r;
    EdgeBlend {
        surface: Surface::Torus {
            center: spine_center,
            axis: n,
            major_radius: s,
            minor_radius: radius,
            u_ref,
        },
        spine_curvature: T::one() / s,
        trim_a: (
            Curve3::Circle {
                center: spine_center + n * radius,
                axis: n,
                radius: s,
                u_ref,
            },
            if sphere_convex { rim - s } else { s - rim },
        ),
        trim_b: (
            Curve3::Circle {
                center: sphere_trim_c,
                axis: n,
                radius: sphere_trim_r,
                u_ref,
            },
            (con_pt - rim_pt).norm(),
        ),
    }
}

// ------------------------------------------------------------------
// The shared sheet reduction: one derivation for every curved-support
// constant-radius arm.
// ------------------------------------------------------------------

/// One support reduced to its **trace in the blend sheet** — the plane
/// the pair's own symmetry confines the rolling ball's centre to (the
/// module docs' one derivation).
///
/// Both traces and the rim point lie in that sheet, so the centre
/// problem is two-dimensional and closed-form: a line and a circle are
/// the only traces an analytic surface of revolution cuts in its own
/// meridian, and the same two are all a cylinder or a plane cuts in a
/// cross-section normal to a ruling.
///
/// **`side` is the material side, and it is the only place one enters
/// an arm.** It is `+1` when the face's stored sense bit says its
/// outward normal IS its surface's chart normal and `−1` when it is the
/// negation, and it enters as a factor on the radius — the `R ∓ r`
/// fold, spelled once here instead of once per pair. Read from stored
/// structure, never from a sampled normal (S10/S11).
#[derive(Clone, Copy, Debug)]
pub enum SupportTrace<T: Real> {
    /// A STRAIGHT trace: the line through the rim point whose unit
    /// normal in the sheet is the support's own chart normal there. A
    /// plane ⊥ the axis, a coaxial cylinder and a coaxial cone all cut
    /// their meridian in one, as does a plane containing a ruling in the
    /// cross-section normal to it.
    ///
    /// The line is anchored at the RIM, not at the support's stored
    /// origin — every such support passes through the rim by
    /// construction, and anchoring there keeps the reduction free of the
    /// support's own axis anchor (a cylinder's `origin`, a cone's
    /// `apex`), which carries placement round-off the rim does not.
    Straight {
        /// The support's unit chart normal at the rim, which lies in the
        /// sheet.
        normal: Vec3<T>,
        /// The material side, `±1` (type docs).
        side: T,
    },
    /// A ROUND trace: the circle the support cuts in the sheet — a
    /// sphere centred on the axis in its meridian, a cylinder about the
    /// ruling in the cross-section.
    Round {
        /// The trace circle's centre, in the sheet.
        center: Point3<T>,
        /// Its radius (positive by convention).
        radius: T,
        /// The material side, `±1` (type docs).
        side: T,
    },
}

impl<T: Real> SupportTrace<T> {
    /// Where the ball at `center` touches this support: back off the
    /// centre by `radius` along the trace's own outward direction.
    fn contact(self, center: Point3<T>, radius: T) -> Point3<T> {
        match self {
            Self::Straight { normal, side } => center + normal * (radius * side),
            Self::Round {
                center: c,
                radius: rr,
                side,
            } => c + (center - c) * (rr / (rr - radius * side)),
        }
    }
}

/// **The rolling ball's centre in the sheet** — the crossing of the two
/// OFFSET traces, in closed form, for each of the three trace pairings.
///
/// `rim` is the point of the original edge the sheet passes through and
/// `sheet_normal` the sheet's unit normal. Each closed form is written
/// so it **selects the branch that returns the rim as `radius → 0`**,
/// which is the structural answer to "which of the two circles the
/// offsets meet in is MY edge" — the same question the plane–sphere
/// arm answers by the meridian's `ρ ≥ 0` and never has to ask.
///
/// - **line × line**: `δ = −r[(σ_a − σ_b d)n̂_a + (σ_b − σ_a d)n̂_b]/(1 − d²)`
///   for `d = n̂_a·n̂_b` — [`plane_plane_blend`]'s own centre formula with
///   the two material sides folded in, and poison exactly where the two
///   traces are parallel (`d = ±1`), which is a tangential pair.
/// - **line × circle**: along the line's own direction `t̂`, the offset
///   parameter solves `λ² + 2λb + D = 0` with `b = R(t̂·û)` and
///   `D = 2Rr(σ_circle − σ_line(n̂·û))` for the circle's outward unit `û`
///   at the rim. The rim branch is `λ = b(√(1 − D/b²) − 1)`, written
///   that way so the `√` carries `b`'s own sign and no comparison
///   appears; `b = 0` is the tangential pair and poisons.
/// - **circle × circle**: the standard two-circle crossing along the
///   centre line, with the transverse half-chord carried by
///   `μ·√((R'² − x²)/μ²)` for `μ` the rim's own transverse coordinate —
///   again the rim branch, branch-free, poisoned at a tangential pair.
///
/// Total arithmetic in, classification at the caller: nothing is gated
/// here, exactly as [`plane_sphere_blend`] gates nothing.
///
/// **Where the poison goes differs by family, and only one of the two
/// paths is predicate 3's.** A COAXIAL pair carries the poison into the
/// spine radius `s` and so into `spine_curvature = 1/s`, which predicate
/// 3 escalates. A RULED pair has a straight spine and
/// [`Ruling::blend`] stores `spine_curvature = 0` unconditionally, so
/// predicate 3 saturates and cannot see it; there the poisoned centre
/// reaches the CYLINDER's `origin` and its `u_ref`, and the refusal
/// arrives one step later — at the open-chain admission door today
/// (a ruled pair meets along an open edge, which the surgery does not
/// carve), and at the certification of any band that door ever mints.
#[must_use]
pub fn sheet_center<T: Real>(
    rim: Point3<T>,
    sheet_normal: Vec3<T>,
    a: SupportTrace<T>,
    b: SupportTrace<T>,
    radius: T,
) -> Point3<T> {
    use SupportTrace::{Round, Straight};
    let one = T::one();
    match (a, b) {
        (
            Straight {
                normal: n_a,
                side: s_a,
            },
            Straight {
                normal: n_b,
                side: s_b,
            },
        ) => {
            let d = n_a.dot(n_b);
            rim - (n_a * (s_a - s_b * d) + n_b * (s_b - s_a * d)) * (radius / (one - d.powi(2)))
        }
        (
            Straight { normal, side },
            Round {
                center,
                radius: rr,
                side: sr,
            },
        )
        | (
            Round {
                center,
                radius: rr,
                side: sr,
            },
            Straight { normal, side },
        ) => {
            // The circle's outward unit at the rim, and the line's own
            // direction in the sheet (orientation-free: `lambda` flips
            // with `t`, so `t * lambda` does not).
            let u = (rim - center) / rr;
            let t = sheet_normal.cross(normal);
            let b_coef = rr * t.dot(u);
            let d_coef = (rr + rr) * radius * (sr - side * normal.dot(u));
            let lambda = b_coef * ((one - d_coef / b_coef.powi(2)).sqrt() - one);
            rim + t * lambda - normal * (radius * side)
        }
        (
            Round {
                center: c_a,
                radius: r_a,
                side: s_a,
            },
            Round {
                center: c_b,
                radius: r_b,
                side: s_b,
            },
        ) => {
            let span = c_b - c_a;
            let dist = span.norm();
            let along = span / dist;
            let across = sheet_normal.cross(along);
            let off_a = r_a - radius * s_a;
            let off_b = r_b - radius * s_b;
            let x = (dist.powi(2) + off_a.powi(2) - off_b.powi(2)) / (dist + dist);
            let mu = (rim - c_a).dot(across);
            let y = mu * (((off_a.powi(2) - x.powi(2)) / mu.powi(2)).sqrt());
            c_a + along * x + across * y
        }
    }
}

/// **The meridian half-plane of a coaxial pair** at one rim point: the
/// sheet in which a pair of surfaces of revolution about one axis
/// reduces to two plane curves.
///
/// The axis is the RIM CIRCLE's own stored frame — a coaxial pair meets
/// in a circle about the common axis, so the edge the caller asked to
/// blend already carries the axis, and nothing is fitted or guessed.
#[derive(Clone, Copy, Debug)]
pub struct Meridian<T: Real> {
    /// A point of the common axis (the rim circle's centre).
    pub origin: Point3<T>,
    /// The unit common axis (the rim circle's axis).
    pub axis: Vec3<T>,
    /// The rim point the sheet passes through.
    pub rim: Point3<T>,
}

impl<T: Real> Meridian<T> {
    /// The sheet's radial unit at the rim — poison on an on-axis rim,
    /// where no meridian is distinguished ([`perp_unit`]).
    #[must_use]
    pub fn radial(&self) -> Vec3<T> {
        perp_unit(self.rim - self.origin, self.axis)
    }

    /// The unit normal of the sheet (the meridian half-plane's own).
    #[must_use]
    pub fn sheet_normal(&self) -> Vec3<T> {
        self.axis.cross(self.radial())
    }

    /// The rim's radius — the **named lever arm** every angular
    /// departure from coaxiality is metered through, so the coaxiality
    /// margin is a length in the same metres as every other one.
    #[must_use]
    pub fn lever(&self) -> T {
        (self.rim - self.origin).dot(self.radial())
    }

    /// Reduce one support to its meridian trace, with the **departure
    /// from coaxiality** it contributes (meters, at [`Self::lever`]).
    ///
    /// `None` for a surface kind this family does not cover; the caller
    /// refuses [`super::BlendError::SpineUnsupported`] on it.
    ///
    /// The departure is what makes the coaxiality hypothesis checkable
    /// rather than assumed: a plane and a cone contribute their axis
    /// misalignment `|n̂ × k̂|` at the lever arm, a sphere the distance of
    /// its centre from the axis. A definite non-zero departure means the
    /// pair is not a coaxial one, so its spine is neither line nor
    /// circle — the canal family, refused.
    #[must_use]
    pub fn trace(&self, s: &Surface<T>, sense: bool) -> Option<(SupportTrace<T>, T)> {
        let side = if sense { T::one() } else { -T::one() };
        let radial = self.radial();
        let lever = self.lever();
        match *s {
            // A plane ⊥ the axis: its chart normal IS the meridian
            // trace's normal, and it must be the axis itself.
            Surface::Plane { normal, .. } => Some((
                SupportTrace::Straight { normal, side },
                normal.cross(self.axis).norm() * lever,
            )),
            // A coaxial cylinder: the meridian trace is the line through
            // the rim with the radially outward normal. The stored
            // radius never enters — the rim's own radius IS it.
            Surface::Cylinder { axis, .. } => Some((
                SupportTrace::Straight {
                    normal: radial,
                    side,
                },
                axis.cross(self.axis).norm() * lever,
            )),
            // A coaxial cone: the generator through the rim, whose chart
            // normal is `radial·cos α − k̂·(ω sin α)` for the NAPPE sign
            // ω — which nappe the rim sits on, read off the apex by sign
            // alone (a sign the apex's own placement round-off cannot
            // move).
            Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            } => {
                let (sin_a, cos_a) = half_angle.sin_cos();
                let nappe = T::one().copysign((self.rim - apex).dot(axis) * axis.dot(self.axis));
                Some((
                    SupportTrace::Straight {
                        normal: radial * cos_a - self.axis * (nappe * sin_a),
                        side,
                    },
                    axis.cross(self.axis).norm() * lever,
                ))
            }
            // A sphere centred on the axis: its meridian trace is the
            // great circle of the same radius, centred at the axis point
            // level with the sphere's centre. The departure is the
            // centre's own distance off the axis.
            Surface::Sphere { center, radius, .. } => {
                let off = center - self.origin;
                let along = self.axis * off.dot(self.axis);
                Some((
                    SupportTrace::Round {
                        center: self.origin + along,
                        radius,
                        side,
                    },
                    (off - along).norm(),
                ))
            }
            _ => None,
        }
    }

    /// **The coaxial arm**: the torus about the circular spine, with
    /// both trimlines as circles coaxial with it.
    ///
    /// The spine radius `s` is the ball centre's own radial coordinate
    /// and is stored SIGNED: a centre that has crossed the axis yields a
    /// negative major radius, which is a torus the surface inventory
    /// reports rather than one this arm quietly repairs (the `minor ≥
    /// major` net). `spine_curvature` is `1/s`, so a spine that does not
    /// exist at all reaches predicate 3 as poison and a spine the tube
    /// swallows reaches it as a finite curvature — the two degenerate
    /// cases [`plane_sphere_blend`] distinguishes, unchanged here.
    #[must_use]
    pub fn blend(&self, a: SupportTrace<T>, b: SupportTrace<T>, radius: T) -> EdgeBlend<T> {
        let radial = self.radial();
        let center = sheet_center(self.rim, self.sheet_normal(), a, b, radius);
        let level = |q: Point3<T>| self.origin + self.axis * (q - self.origin).dot(self.axis);
        let s = (center - self.origin).dot(radial);
        // The setback is the EUCLIDEAN displacement of the trimline from
        // the rim, the metric predicate 2 measures face extents in.
        let trim = |t: SupportTrace<T>| {
            let q = t.contact(center, radius);
            (
                Curve3::Circle {
                    center: level(q),
                    axis: self.axis,
                    radius: (q - self.origin).dot(radial),
                    u_ref: radial,
                },
                (q - self.rim).norm(),
            )
        };
        EdgeBlend {
            surface: Surface::Torus {
                center: level(center),
                axis: self.axis,
                major_radius: s,
                minor_radius: radius,
                u_ref: radial,
            },
            spine_curvature: T::one() / s,
            trim_a: trim(a),
            trim_b: trim(b),
        }
    }
}

/// **The cross-section of a ruled pair** at one point of the common
/// ruling: the sheet in which two supports that are translational along
/// one direction reduce to two plane curves.
///
/// The ruling is the EDGE's own stored direction — a pair meeting along
/// a straight line carries it on the carrier, so nothing is fitted.
#[derive(Clone, Copy, Debug)]
pub struct Ruling<T: Real> {
    /// The unit ruling direction (the rim line's own).
    pub tau: Vec3<T>,
    /// The rim point the cross-section passes through.
    pub rim: Point3<T>,
    /// The link's own extent — the named lever arm the departure from
    /// the shared-ruling hypothesis is metered through.
    pub lever: T,
}

impl<T: Real> Ruling<T> {
    /// Reduce one support to its cross-section trace, with the
    /// **departure from the shared-ruling hypothesis** it contributes
    /// (meters, at [`Self::lever`]).
    ///
    /// `None` for a surface kind this family does not cover.
    #[must_use]
    pub fn trace(&self, s: &Surface<T>, sense: bool) -> Option<(SupportTrace<T>, T)> {
        let side = if sense { T::one() } else { -T::one() };
        match *s {
            // A plane containing the ruling: its chart normal is already
            // ⊥ the ruling, so it is the cross-section line's normal.
            Surface::Plane { normal, .. } => Some((
                SupportTrace::Straight { normal, side },
                normal.dot(self.tau).abs() * self.lever,
            )),
            // A cylinder about the ruling: the cross-section circle,
            // centred where the cylinder's axis crosses this sheet.
            Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => {
                let on_axis = origin + axis * (self.rim - origin).dot(axis);
                Some((
                    SupportTrace::Round {
                        center: on_axis - self.tau * (on_axis - self.rim).dot(self.tau),
                        radius,
                        side,
                    },
                    axis.cross(self.tau).norm() * self.lever,
                ))
            }
            _ => None,
        }
    }

    /// **The ruled arm**: the cylinder of radius `r` about the straight
    /// spine, with both trimlines as lines along the ruling.
    ///
    /// `spine_curvature` is zero — a straight spine never folds, so
    /// predicate 3 saturates exactly as it does on [`plane_plane_blend`].
    /// That is also why a degenerate ruled crossing does NOT reach
    /// predicate 3: the zero is unconditional, so poison from
    /// [`sheet_center`] rides the cylinder's `origin` and `u_ref`
    /// instead (see that function's family note).
    #[must_use]
    pub fn blend(&self, a: SupportTrace<T>, b: SupportTrace<T>, radius: T) -> EdgeBlend<T> {
        let center = sheet_center(self.rim, self.tau, a, b, radius);
        let (q_a, q_b) = (a.contact(center, radius), b.contact(center, radius));
        EdgeBlend {
            surface: Surface::Cylinder {
                origin: center,
                axis: self.tau,
                radius,
                u_ref: (q_a - center).normalize(),
            },
            spine_curvature: T::zero(),
            trim_a: (
                Curve3::Line {
                    origin: q_a,
                    dir: self.tau,
                },
                (q_a - self.rim).norm(),
            ),
            trim_b: (
                Curve3::Line {
                    origin: q_b,
                    dir: self.tau,
                },
                (q_b - self.rim).norm(),
            ),
        }
    }
}

/// **The corner ball** of a three-convex-edge vertex: the sphere of
/// radius `r` tangent to all three support planes from inside the
/// material.
///
/// The centre solves `(c − p_i)·n_i = −r` for the three outward
/// normals; the solution is `c = p + N⁻¹·(…)` written here as the
/// Cramer expansion so `|det(n₁, n₂, n₃)|` — the independence margin
/// predicate 6 classifies — falls out of the same computation.
///
/// The patch is bounded by three CIRCLES, one per incident edge
/// cylinder: the ball centre lies ON each cylinder's axis (the axis
/// is the locus at distance `r` from two of the planes, and the
/// centre is at distance `r` from all three), and both have radius
/// `r`, so sphere and cylinder are tangent along a full circle. That
/// is the configuration the jet certificate's circle arm certifies,
/// and its `κ_rel` is `1/r` — the sphere curves transverse to the
/// contact circle where the cylinder is flat along its ruling.
#[must_use]
pub fn corner_ball<T: Real>(
    verts: [Point3<T>; 3],
    normals: [Vec3<T>; 3],
    radius: T,
    convex: bool,
) -> CornerBall<T> {
    let [n1, n2, n3] = normals;
    let signed = if convex { -radius } else { radius };
    // Right-hand sides: c·n_i = p_i·n_i + signed.
    let rhs = [
        Vec3::new(verts[0].x, verts[0].y, verts[0].z).dot(n1) + signed,
        Vec3::new(verts[1].x, verts[1].y, verts[1].z).dot(n2) + signed,
        Vec3::new(verts[2].x, verts[2].y, verts[2].z).dot(n3) + signed,
    ];
    let det = n1.dot(n2.cross(n3));
    // Cramer: c = (rhs₁·(n₂×n₃) + rhs₂·(n₃×n₁) + rhs₃·(n₁×n₂)) / det.
    let num = n2.cross(n3) * rhs[0] + n3.cross(n1) * rhs[1] + n1.cross(n2) * rhs[2];
    let c = Point3::new(num.x / det, num.y / det, num.z / det);
    CornerBall {
        surface: Surface::Sphere {
            center: c,
            radius,
            axis: (n1 + n2 + n3).normalize(),
            u_ref: perp_unit(n1, (n1 + n2 + n3).normalize()),
        },
        center: c,
        independence: det.abs(),
    }
}

// ------------------------------------------------------------------
// The chamfer's arm: the flat strip, and the flat corner patch.
// ------------------------------------------------------------------

/// A support's **inward in-plane unit** at one of its boundary edges:
/// the unit ⊥ the edge, in the support plane, pointing into the face.
///
/// It is read off the TRAVERSAL, never off a convexity verdict. Under
/// the kernel's counterclockwise-outer-loop convention
/// (`topo::entity`) a face's interior lies to the LEFT of each of its
/// half-edges seen from outside the shell, and "left of `τ̂` seen from
/// outside" is the rotation of `τ̂` by a quarter turn about the
/// OUTWARD normal — `n × τ̂`. `τ̂` is the half-edge's own direction, so
/// the second support of an edge (which carries `he_minus`, running
/// against the stored carrier) takes `−τ̂`.
///
/// This is what makes the chamfer's geometry convexity-free: on a
/// concave edge the two supports lie on the other sides of the edge
/// and their half-edges run the other way round, so the same
/// expression still points into the face. Convexity enters the
/// chamfer at its ADMISSION doors and nowhere in this file.
#[must_use]
pub fn inward_unit<T: Real>(n: Vec3<T>, tau: Vec3<T>) -> Vec3<T> {
    n.cross(tau).normalize()
}

/// **The plane–plane chamfer strip**: the flat band that replaces a
/// straight edge at equal setback `distance` along both supports.
///
/// With `p` a point of the edge, `tau` the `he_plus` traversal
/// direction and `n_a`/`n_b` the two supports' OUTWARD normals, the
/// two trimlines are `p + m·distance + τ̂ s` for the supports' own
/// inward units `m_a = n_a × τ̂` and `m_b = −(n_b × τ̂)`
/// ([`inward_unit`]) — lines parallel to the edge, so the strip is an
/// exact [`Surface::Plane`] and nothing is fitted.
///
/// **Its chart normal is `n_a + n_b`, and that is a derivation, not a
/// convention.** The strip's normal is
/// `(foot_b − foot_a) × τ̂ = −[(n_a + n_b) × τ̂] × τ̂ = n_a + n_b`
/// (both normals are ⊥ `τ̂`). A positive combination of two OUTWARD
/// normals is outward, so the strip face mints with sense `true` — on
/// a concave edge as much as on a convex one, which is why the
/// chamfer never reads [`super::Convexity::blend_sense`]. For a
/// symmetric setback the strip plane is the supports' bisector plane,
/// which is the same statement.
#[must_use]
pub fn chamfer_strip<T: Real>(
    p: Point3<T>,
    tau: Vec3<T>,
    n_a: Vec3<T>,
    n_b: Vec3<T>,
    distance: T,
) -> EdgeBlend<T> {
    let m_a = inward_unit(n_a, tau);
    let m_b = -inward_unit(n_b, tau);
    let foot_a = p + m_a * distance;
    let foot_b = p + m_b * distance;
    EdgeBlend {
        surface: Surface::Plane {
            origin: foot_a,
            normal: (n_a + n_b).normalize(),
            u_ref: tau,
        },
        spine_curvature: T::zero(),
        trim_a: (
            Curve3::Line {
                origin: foot_a,
                dir: tau,
            },
            distance,
        ),
        trim_b: (
            Curve3::Line {
                origin: foot_b,
                dir: tau,
            },
            distance,
        ),
    }
}

/// Where two coplanar lines of one support face meet, in that face's
/// plane of normal `n` — the chamfer's FOOT at a corner, which is the
/// point where the two incident strips' trimlines on this support
/// cross.
///
/// Closed form: `s = ((o₂ − o₁) × d₂)·n / ((d₁ × d₂)·n)`, exact and
/// total. Two parallel trimlines (a "corner" whose two edges are
/// collinear) drive the denominator to zero and poison the foot, which
/// is the honest answer — there is no crossing there.
#[must_use]
pub fn line_meet<T: Real>(
    o1: Point3<T>,
    d1: Vec3<T>,
    o2: Point3<T>,
    d2: Vec3<T>,
    n: Vec3<T>,
) -> Point3<T> {
    let s = (o2 - o1).cross(d2).dot(n) / d1.cross(d2).dot(n);
    o1 + d1 * s
}

/// **The chamfer's corner patch**: the plane through the three feet at
/// a trivalent corner — the sphere octant's flat analog.
///
/// Each strip meets the patch along the segment between the two feet
/// on that strip's own supports, and both of those feet lie on that
/// strip's trimlines, which the strip plane contains — so the patch
/// closes the corner exactly, with no fitting and no tolerance.
///
/// **The chart normal is outward, derived.** The patch plane's normal
/// is `g = (f₁ − f₀) × (f₂ − f₀)` up to sign, and the sign is fixed by
/// folding `g` against the supports' outward normal SUM: the patch is
/// the face that truncates (convex) or fills (concave) the corner, and
/// in both cases its outward normal lies on the positive side of
/// `n₀ + n₁ + n₂`. Folding it in as a positive multiple —
/// `normalize(g·(g·Σn))` — keeps the derivation branch-free and total;
/// a patch plane orthogonal to `Σn` poisons rather than picking a
/// side. The face therefore mints with sense `true`, like the strips,
/// and this module has no convexity parameter to leave half-derived.
#[must_use]
pub fn chamfer_corner_patch<T: Real>(feet: [Point3<T>; 3], normals: [Vec3<T>; 3]) -> Surface<T> {
    let g = (feet[1] - feet[0]).cross(feet[2] - feet[0]);
    let outward = normals[0] + normals[1] + normals[2];
    let normal = (g * g.dot(outward)).normalize();
    Surface::Plane {
        origin: feet[0],
        normal,
        u_ref: perp_unit(feet[1] - feet[0], normal),
    }
}

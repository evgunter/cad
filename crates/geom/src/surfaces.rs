//! Analytic surfaces: the [`Surface`] closed enum and its evaluators.
//!
//! Surface kinds form a **closed enum** per D3 (`docs/DESIGN.md`):
//! intersection needs pairwise dispatch (plane×cylinder, cylinder×torus,
//! …), and a closed enum makes every dispatch site exhaustively checked
//! at compile time. The [`Surface::Nurbs`] variant is the universal
//! fallback — it carries a validated [`NurbsSurface`] payload (see
//! [`nurbs`]) and its evaluator arms are real; the "no description yet"
//! state is [`Surface::nurbs_placeholder`].
//!
//! # Surface conventions (normative; the surface half)
//!
//! The crate docs carry the conventions curves and surfaces share —
//! units, complete loci (here: the infinite plane, the full cylinder,
//! both cone nappes), the no-range-reduction rule and its bit-identity
//! policy, conventional-and-unchecked frame fields, totality and
//! poison, and the evaluation-code discipline. These are the
//! surface-specific ones:
//!
//! - **The reference frame.** Every axisymmetric variant carries a unit
//!   `axis` and a unit `u_ref ⊥ axis`; the third frame vector is always
//!   `v_ref = axis × u_ref`, **computed, never stored** — the frame
//!   `(u_ref, v_ref, axis)` is right-handed by construction. The
//!   plane's frame is the same shape with `normal` in the axis role.
//!   `u_ref` carries the **seam** (where u = 0 lives).
//! - **The shared helper.** `radial(u) = u_ref·cos u + v_ref·sin u` is
//!   the unit vector at azimuth `u`; its derivative
//!   `tangential(u) = u_ref·(−sin u) + v_ref·cos u`. Every azimuthal
//!   evaluator is written in terms of these two, with one `sin_cos`
//!   call per parameter.
//! - **Normals are derived, not stored**: `normal(u, v)` is
//!   `(∂S/∂u × ∂S/∂v).normalize()` — the orientation is the
//!   parameterization's, full stop. There is no "outward" concept at
//!   this layer: a face's material side is topology's to carry (D1's
//!   interior-left rule); the surface only promises which way its
//!   *chart* orients. (For the closed surfaces below, the chosen charts
//!   happen to give the intuitive out-of-the-material direction —
//!   radially out of the cylinder, out of the sphere and torus — a
//!   convenience, not a contract.)
//! - **Parameterization singularities are chart defects, documented
//!   per variant, and they poison the derived normal honestly**: where
//!   `∂S/∂u = 0` (sphere poles, cone apex) the cross product vanishes
//!   and `normalize` yields the scalar's poison — no branch, no
//!   fabricated limit direction. (Exactly AT the singularity, that is;
//!   in the underflow band just off it — cone: `v ≲ 3e-162` — the cross
//!   product's `norm_squared` underflows to 0 and `normalize` yields
//!   `±∞` components instead of NaN, ∞ not being f64 poison, with a
//!   degraded-precision band just above; `Vec3::normalize`'s docs carry
//!   the band boundaries. Both bands are far outside the session box,
//!   D4 ¶4.) The SURFACE is regular at a sphere
//!   pole (the chart isn't); the cone apex is a genuine surface
//!   singularity (no tangent plane exists). Pole handling (tessellation
//!   fans, revolve's pole vertices) is downstream case analysis on
//!   *topology*, never a special path inside these evaluators.
//! - **The conventional fields** here are `axis`, `normal` and `u_ref`
//!   (unit; `u_ref ⊥ axis`, ⊥ `normal`), unchecked per the crate
//!   docs' rule.

pub mod approx;
pub mod boxes;
pub mod nurbs;
pub mod projection;

use std::sync::Arc;

use geom_core::spline::SpanLocate;
use geom_core::{Point3, Real, Vec3};

use crate::azimuth;
pub use approx::{ApproxSurface, ChartWindow, OffsetCertificate, SurfaceDescription, SurfaceSpec};
pub use nurbs::{NurbsSurface, SurfaceJet, SurfaceJet3, SurfaceWindow};
pub use projection::{SurfaceProjection, SurfaceProjectionInconclusive};

/// An analytic surface — a **complete locus**. Units, the
/// no-range-reduction rule and its bit-identity policy, and the
/// conventional-and-unchecked field rule are the crate docs'; the
/// reference frame, seam placement, derived normals and the chart
/// singularities are this module's.
///
/// Fields are public data (D2: conventions are carried by data);
/// construction is by struct-literal variant syntax.
///
/// **`Clone`, not `Copy` (M5 PR 3, accepted and binding):** the
/// [`Surface::Nurbs`] payload is an [`Arc`]-shared [`NurbsSurface`], so
/// the enum is cheap to clone (one refcount) but no longer `Copy`. The
/// payload is immutable after validated construction — sharing is
/// D9-clean (no address-dependent behavior, no interior mutability).
#[derive(Clone, Debug)]
pub enum Surface<T: Real> {
    /// The infinite plane `S(u, v) = origin + u_ref·u + v_ref·v` with
    /// `v_ref = normal × u_ref`.
    ///
    /// - `u`, `v` in meters (unit frame ⇒ arc-length parameters);
    ///   domain ℝ², not periodic. `normal ⊥ u_ref`, both unit
    ///   (conventional, unchecked).
    /// - The chart normal `∂u × ∂v` is exactly the stored `normal` (in
    ///   exact arithmetic): `u_ref × v_ref = normal` for the
    ///   right-handed frame.
    Plane {
        /// The point at `(u, v) = (0, 0)` — conventional data.
        origin: Point3<T>,
        /// The unit plane normal (conventional, unchecked).
        normal: Vec3<T>,
        /// The unit direction of the `u` axis, ⊥ `normal` — carries the
        /// in-plane frame convention.
        u_ref: Vec3<T>,
    },

    /// The infinite cylinder
    /// `S(u, v) = origin + radial(u)·radius + axis·v`.
    ///
    /// - `u` in radians, 2π-periodic (the azimuth; seam at `u_ref`);
    ///   `v` in meters along `axis` (`origin` is the `v = 0` point on
    ///   the axis); domain ℝ².
    /// - Chart normal: `radial(u)` — radially outward (for
    ///   `radius > 0`), since `∂u × ∂v = tangential·r × axis =
    ///   radial·r`.
    /// - No chart singularities (`radius > 0` by convention).
    Cylinder {
        /// The `v = 0` point on the cylinder's axis.
        origin: Point3<T>,
        /// The unit axis direction; increasing `v` runs along it.
        axis: Vec3<T>,
        /// The radius in meters (positive by convention).
        radius: T,
        /// The unit reference direction ⊥ `axis` where u = 0 lives —
        /// the seam.
        u_ref: Vec3<T>,
    },

    /// The (double) cone with apex `apex`, axis `axis`, and
    /// **half-angle** `half_angle` ∈ (0, π/2) between axis and
    /// generators:
    /// `S(u, v) = apex + axis·(v·cos α) + radial(u)·(v·sin α)`.
    ///
    /// - `u` in radians, 2π-periodic (azimuth; seam at `u_ref`).
    /// - `v` in meters **along the slant** (arc length of the
    ///   generator lines; `|∂S/∂v| = 1`): the (u, v) convention chosen
    ///   so both parameters are honest lengths/angles. `v = 0` is the
    ///   apex; `v > 0` is the nappe opening along `+axis`; `v < 0`
    ///   evaluates the mirror nappe (the complete locus — kernel faces
    ///   will bound a single nappe by their vertices, as everywhere).
    /// - **The apex, honestly**: at `v = 0` the parameterization is
    ///   singular (`∂S/∂u = 0`, every azimuth maps to the apex) AND the
    ///   surface itself is singular — the cone has no tangent plane at
    ///   its apex; this is a true geometric singularity, unlike the
    ///   sphere's pole, which is purely a chart artifact. The derived
    ///   normal at `v = 0` is therefore poison (0-vector normalized),
    ///   which is the correct answer, not a limitation: any definite
    ///   vector would fabricate a tangent plane that does not exist.
    ///   (NaN-poison holds exactly at `v = 0`; in the underflow band
    ///   `0 < |v| ≲ 3e-162` the normalization instead yields `±∞`
    ///   components — ∞ is not f64 poison — see this module's
    ///   singularity bullet and `Vec3::normalize`'s band notes.)
    ///   Away from the apex the chart normal is
    ///   `radial(u)·cos α − axis·sin α` for `v > 0` (tilted outward,
    ///   perpendicular to the generator) and its negation for `v < 0`.
    /// - `half_angle` strictly inside (0, π/2) by convention: 0
    ///   degenerates to a line, π/2 to a plane — both rejected upstream
    ///   (evaluated as-is here, like every conventional invariant).
    Cone {
        /// The apex point (`v = 0`).
        apex: Point3<T>,
        /// The unit axis direction; the `v > 0` nappe opens along it.
        axis: Vec3<T>,
        /// The half-angle α ∈ (0, π/2), radians, between `axis` and the
        /// generator lines.
        half_angle: T,
        /// The unit reference direction ⊥ `axis` where u = 0 lives —
        /// the seam.
        u_ref: Vec3<T>,
    },

    /// The sphere
    /// `S(u, v) = center + (radial(u)·cos v + axis·sin v)·radius`.
    ///
    /// - `u` in radians, 2π-periodic: **longitude** (azimuth; seam at
    ///   `u_ref`). `v` in radians: **latitude**, canonical domain
    ///   `[−π/2, π/2]`, `v = 0` the equator through
    ///   `center + u_ref·radius` — chosen (over colatitude) so the
    ///   azimuth/`u_ref`/seam pattern is identical across cylinder,
    ///   cone, sphere, and torus.
    /// - **Poles, honestly**: at `v = ±π/2` (the points
    ///   `center ± axis·radius`) the SURFACE is perfectly regular —
    ///   the chart is not: `∂S/∂u = tangential·(r·cos v) → 0`, all
    ///   longitudes map to one point, and the derived normal is poison
    ///   there (in exact arithmetic; at `f64`, `cos(fl(π/2)) ≈ 6e-17`
    ///   is not exactly zero, so evaluation near the pole returns
    ///   finite values with catastrophically ill-conditioned `u`
    ///   derivatives — same defect, different clothes). Downstream pole
    ///   machinery (revolve's pole vertices, tessellation fans) owns
    ///   the case analysis; the evaluator stays branch-free.
    /// - Chart normal: `radial(u)·cos v + axis·sin v` — radially
    ///   outward — since `∂u × ∂v = (that unit vector)·(r²·cos v)` and
    ///   `cos v ≥ 0` on the canonical domain.
    Sphere {
        /// The sphere's center.
        center: Point3<T>,
        /// The radius in meters (positive by convention).
        radius: T,
        /// The unit pole axis: the poles lie at `center ± axis·radius`
        /// (`v = ±π/2`).
        axis: Vec3<T>,
        /// The unit reference direction ⊥ `axis` where u = 0 lives —
        /// the seam meridian.
        u_ref: Vec3<T>,
    },

    /// The torus
    /// `S(u, v) = center + radial(u)·(R + r·cos v) + axis·(r·sin v)`
    /// with `R` = `major_radius`, `r` = `minor_radius`.
    ///
    /// - `u` in radians, 2π-periodic: the **major** azimuth around
    ///   `axis` (seam at `u_ref`). `v` in radians, 2π-periodic: the
    ///   **minor** angle around the tube; `v = 0` on the outer equator
    ///   (radially farthest from the axis), `v = π/2` on the top circle
    ///   (toward `+axis`).
    /// - Convention `R > r > 0` (a ring torus — no self-intersection);
    ///   spindle/horn configurations are degenerate data. `sweep::revolve`
    ///   refuses them at construction, but the other door that can mint a
    ///   torus (`step-import`'s `TOROIDAL_SURFACE`) reads both radii
    ///   verbatim — so the net covering BOTH is `topo::validate`'s tier-3
    ///   check 1, which reports `DegenerateTorus` on any face carrying one
    ///   at rest.
    /// - Chart normal: `radial(u)·cos v + axis·sin v` — out of the tube
    ///   — since `∂u × ∂v = (that)·(r·(R + r·cos v))` and
    ///   `R + r·cos v > 0` for a ring torus. No chart singularities on
    ///   a ring torus.
    Torus {
        /// The torus center (on the axis, in the tube's midplane).
        center: Point3<T>,
        /// The unit axis of revolution.
        axis: Vec3<T>,
        /// The major radius R in meters: center-to-tube-center
        /// distance (`R > r` by convention).
        major_radius: T,
        /// The minor radius r in meters: the tube radius (positive by
        /// convention).
        minor_radius: T,
        /// The unit reference direction ⊥ `axis` where u = 0 lives —
        /// the seam meridian.
        u_ref: Vec3<T>,
    },

    /// The NURBS fallback (D3: representable from day one; evaluators
    /// implemented at M5 PR 3). The payload is a validated
    /// [`NurbsSurface`] behind an [`Arc`] (immutable, cheap to clone —
    /// see the enum docs on the `Copy` loss). The "no description yet"
    /// state the former unit variant carried is
    /// [`Surface::nurbs_placeholder`] — a poison-valued payload with
    /// the same all-poison evaluation behavior.
    Nurbs(Arc<NurbsSurface<T>>),

    /// An **approximating** surface: a fitted NURBS standing in for a
    /// surface the kernel cannot represent exactly, carrying the
    /// intensional description of what it approximates and a
    /// certificate bounding the distance between them
    /// ([`ApproxSurface`]).
    ///
    /// The fit **is** the geometry: every evaluator, box and mesh arm
    /// delegates to it, and the certificate says how far that stands
    /// from the intent. The description is what re-certification
    /// measures against — the validator re-derives per face and never
    /// trusts the stored certificate.
    ///
    /// The payload is certified by construction ([`ApproxSurface`] has
    /// no other door), so unlike [`Surface::Nurbs`] there is no
    /// placeholder state in this variant: an `Approx` surface is always
    /// described.
    Approx(Arc<ApproxSurface<T>>),
}

impl<T: Real> Surface<T> {
    /// The "no description yet" NURBS state (the former unit
    /// placeholder variant, as data): a structurally valid payload
    /// whose control points are all-poison, so evaluation yields the
    /// all-poison point and every downstream certification fails
    /// loudly (D4 ¶2) — representable ≠ described.
    pub fn nurbs_placeholder() -> Self {
        Surface::Nurbs(Arc::new(NurbsSurface::placeholder()))
    }

    /// The **spline chart** a surface evaluates through, when it has
    /// one: the payload for [`Surface::Nurbs`], the fit for
    /// [`Surface::Approx`], `None` for the analytic kinds.
    ///
    /// The invariant this expresses is the `Approx` variant's own — the
    /// fit IS the geometry — so every consumer whose question is about
    /// the surface's `(u, v)` chart (evaluation, stretch bounds, the
    /// spline pcurve lanes, tessellation) asks here rather than
    /// matching `Nurbs` and dropping `Approx` on the floor. A consumer
    /// whose question is about what the surface *means* (the boolean
    /// operand gate, the offset mint, STEP export) must NOT: it reads
    /// the description, or refuses.
    pub fn spline_chart(&self) -> Option<&NurbsSurface<T>> {
        match self {
            Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Sphere { .. }
            | Surface::Torus { .. } => None,
            Surface::Nurbs(n) => Some(n),
            Surface::Approx(a) => Some(a.fit()),
        }
    }
}

impl<T: SpanLocate> Surface<T> {
    /// The point at parameters `(u, v)` — each variant's formula and
    /// conventions are on the variant (this module's docs carry the
    /// frame and seam rules, the crate docs the units). Evaluation
    /// order per variant is exactly the documented formula with the
    /// shared azimuthal frame's fixed associations
    /// (`radial`/`tangential` from one `sin_cos`, `crate::azimuth`);
    /// [`Surface::Nurbs`] routes to the payload's evaluator
    /// (span selection via the sealed [`SpanLocate`] seam — the
    /// `impl`-block bound, a sealed `Real` subtrait; see the crate
    /// docs' evaluation-code discipline note).
    ///
    /// **Each analytic arm's point expression is written twice** — once
    /// here and once in [`Surface::jet`] — and that is a duplication
    /// this method's own collapse of the derivative accessors created.
    /// The alternative is making `eval` a projection of the jet, which
    /// costs [`Surface::Nurbs`] an order-2 basis pass on the workspace's
    /// hottest evaluation door to spare five short analytic
    /// expressions; the copies are the cheaper trade. They are not
    /// unguarded: `eval_agrees_bitwise_with_the_jets_point` compares
    /// the two, bit for bit, over every chart in the corpus, so a
    /// divergence between the copies is a red test rather than a silent
    /// fork.
    pub fn eval(&self, u: T, v: T) -> Point3<T> {
        match self {
            &Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                let v_ref = normal.cross(u_ref);
                origin + u_ref * u + v_ref * v
            }
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                let (radial, _) = azimuth::frame(axis, u_ref, u);
                origin + radial * radius + axis * v
            }
            &Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => {
                let (s_a, c_a) = half_angle.sin_cos();
                let (radial, _) = azimuth::frame(axis, u_ref, u);
                apex + axis * (v * c_a) + radial * (v * s_a)
            }
            &Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => {
                let (s_v, c_v) = v.sin_cos();
                let (radial, _) = azimuth::frame(axis, u_ref, u);
                center + (radial * c_v + axis * s_v) * radius
            }
            &Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => {
                let (s_v, c_v) = v.sin_cos();
                let (radial, _) = azimuth::frame(axis, u_ref, u);
                center + radial * (major_radius + minor_radius * c_v) + axis * (minor_radius * s_v)
            }
            Surface::Nurbs(n) => n.eval(u, v),
            // The fit IS the geometry (the variant's docs): evaluation
            // delegates to it, and the certificate — not this arm —
            // carries how far it stands from the description.
            Surface::Approx(a) => a.fit().eval(u, v),
        }
    }

    /// The whole second-order jet at `(u, v)` in **one** evaluation:
    /// the point and every partial with `k + l ≤ 2`.
    ///
    /// This is the enum's primitive derivative query: the five
    /// single-partial accessors below and [`Surface::normal`] are
    /// defined as its projections, so each field here is, by
    /// construction, exactly what the corresponding accessor returns,
    /// bit for bit (pinned by test, as [`SurfaceJet3`]'s common fields
    /// are). A caller wanting more than one partial at a point asks
    /// once: the analytic arms build the azimuthal frame and
    /// `sin_cos(v)` a single time, and [`Surface::Nurbs`] makes a
    /// single [`NurbsSurface::ders`] pass rather than one per partial.
    ///
    /// `point` is the payload jet's own point for [`Surface::Nurbs`] —
    /// [`Surface::eval`] keeps its dedicated pass and is **not** a
    /// projection of this jet. That pass really is the cheaper one:
    /// order-0 basis against this jet's order-2 tensor, measured on a
    /// rational patch at 2 heap allocations against the jet's 40.
    ///
    /// Note what these arms cost the other way: on the analytic
    /// variants the single-partial projections are now *more*
    /// expensive than the bodies they replaced — `deriv_vv` on a
    /// [`Surface::Plane`] was `Vec3::zero()` and now builds a whole
    /// jet. That is deliberate, and it is affordable because no
    /// production path calls the single-partial doors; `Surface::eval`
    /// and `Surface::jet` are the only two that any does.
    pub fn jet(&self, u: T, v: T) -> SurfaceJet<T> {
        match self {
            &Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                let v_ref = normal.cross(u_ref);
                SurfaceJet {
                    point: origin + u_ref * u + v_ref * v,
                    du: u_ref,
                    dv: v_ref,
                    duu: Vec3::zero(),
                    duv: Vec3::zero(),
                    dvv: Vec3::zero(),
                }
            }
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                let (radial, tangential) = azimuth::frame(axis, u_ref, u);
                SurfaceJet {
                    point: origin + radial * radius + axis * v,
                    du: tangential * radius,
                    dv: axis,
                    duu: radial * (-radius),
                    duv: Vec3::zero(),
                    dvv: Vec3::zero(),
                }
            }
            &Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => {
                let (s_a, c_a) = half_angle.sin_cos();
                let (radial, tangential) = azimuth::frame(axis, u_ref, u);
                SurfaceJet {
                    point: apex + axis * (v * c_a) + radial * (v * s_a),
                    du: tangential * (v * s_a),
                    dv: axis * c_a + radial * s_a,
                    duu: radial * (-(v * s_a)),
                    duv: tangential * s_a,
                    dvv: Vec3::zero(),
                }
            }
            &Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => {
                let (s_v, c_v) = v.sin_cos();
                let (radial, tangential) = azimuth::frame(axis, u_ref, u);
                SurfaceJet {
                    point: center + (radial * c_v + axis * s_v) * radius,
                    du: tangential * (radius * c_v),
                    dv: (radial * (-s_v) + axis * c_v) * radius,
                    duu: radial * (-(radius * c_v)),
                    duv: tangential * (-(radius * s_v)),
                    dvv: (radial * c_v + axis * s_v) * (-radius),
                }
            }
            &Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => {
                let (s_v, c_v) = v.sin_cos();
                let (radial, tangential) = azimuth::frame(axis, u_ref, u);
                SurfaceJet {
                    point: center
                        + radial * (major_radius + minor_radius * c_v)
                        + axis * (minor_radius * s_v),
                    du: tangential * (major_radius + minor_radius * c_v),
                    dv: (radial * (-s_v) + axis * c_v) * minor_radius,
                    duu: radial * (-(major_radius + minor_radius * c_v)),
                    duv: tangential * (-(minor_radius * s_v)),
                    dvv: (radial * c_v + axis * s_v) * (-minor_radius),
                }
            }
            Surface::Nurbs(n) => n.ders(u, v),
            // As `eval`: the fit is the geometry, so every derivative
            // (and therefore `normal`) is the fit's.
            Surface::Approx(a) => a.fit().ders(u, v),
        }
    }

    /// The first partial `∂S/∂u`.
    ///
    /// Plane: `u_ref` (constant). Cylinder: `tangential(u)·radius`.
    /// Cone: `tangential(u)·(v·sin α)` — **zero at the apex** (`v = 0`,
    /// the chart singularity). Sphere: `tangential(u)·(radius·cos v)` —
    /// **zero at the poles**. Torus:
    /// `tangential(u)·(R + r·cos v)`. Nurbs: the payload jet's `du`
    /// (all-poison for the placeholder state).
    ///
    /// [`Surface::jet`]'s `du`, projected — the enum evaluates its jet
    /// once, and a caller wanting a second partial at the same `(u, v)`
    /// asks for the jet rather than for two projections.
    pub fn deriv_u(&self, u: T, v: T) -> Vec3<T> {
        self.jet(u, v).du
    }

    /// The first partial `∂S/∂v`.
    ///
    /// Plane: `v_ref = normal × u_ref` (constant). Cylinder: `axis`
    /// (constant). Cone: `axis·cos α + radial(u)·sin α` — the unit
    /// generator direction (slant parameterization). Sphere:
    /// `(radial(u)·(−sin v) + axis·cos v)·radius` — the meridian
    /// tangent. Torus: `(radial(u)·(−sin v) + axis·cos v)·r`. Nurbs:
    /// the payload jet's `dv` (all-poison for the placeholder state).
    ///
    /// [`Surface::jet`]'s `dv`, projected (see [`Surface::deriv_u`]).
    pub fn deriv_v(&self, u: T, v: T) -> Vec3<T> {
        self.jet(u, v).dv
    }

    /// The unit chart normal: `du × dv` normalized, off ONE
    /// [`Surface::jet`] — the same value as
    /// `self.deriv_u(u, v).cross(self.deriv_v(u, v)).normalize()`, whose
    /// two fields it takes from a single evaluation instead of two. The
    /// derived normal of this module's docs. Orientation is the
    /// parameterization's; at chart singularities (sphere poles in
    /// exact arithmetic, the cone apex) the cross vanishes and the
    /// result is honest poison, per the singularity bullet there (no
    /// fabricated limit directions; downstream pole machinery owns
    /// those points).
    pub fn normal(&self, u: T, v: T) -> Vec3<T> {
        let j = self.jet(u, v);
        j.du.cross(j.dv).normalize()
    }

    /// The second partial `∂²S/∂u²`.
    ///
    /// Plane: zero. Cylinder: `radial(u)·(−radius)`. Cone:
    /// `radial(u)·(−(v·sin α))`. Sphere: `radial(u)·(−(radius·cos v))`.
    /// Torus: `radial(u)·(−(R + r·cos v))`. Nurbs: the payload jet's
    /// `duu` (all-poison for the placeholder state). (Each is the
    /// azimuthal-rotation second derivative: `radial″ = −radial`.)
    ///
    /// [`Surface::jet`]'s `duu`, projected (see [`Surface::deriv_u`]).
    pub fn deriv_uu(&self, u: T, v: T) -> Vec3<T> {
        self.jet(u, v).duu
    }

    /// The mixed second partial `∂²S/∂u∂v` (= `∂²S/∂v∂u` — smooth
    /// charts).
    ///
    /// Plane, cylinder: zero. Cone: `tangential(u)·sin α`. Sphere:
    /// `tangential(u)·(−(radius·sin v))`. Torus:
    /// `tangential(u)·(−(r·sin v))`. Nurbs: the payload jet's `duv`
    /// (all-poison for the placeholder state).
    ///
    /// [`Surface::jet`]'s `duv`, projected (see [`Surface::deriv_u`]).
    pub fn deriv_uv(&self, u: T, v: T) -> Vec3<T> {
        self.jet(u, v).duv
    }

    /// The second partial `∂²S/∂v²`.
    ///
    /// Plane, cylinder, cone: zero (rulings/generators are straight).
    /// Sphere: `(radial(u)·cos v + axis·sin v)·(−radius)` — the inward
    /// radial. Torus: `(radial(u)·cos v + axis·sin v)·(−r)` — into the
    /// tube. Nurbs: the payload jet's `dvv` (all-poison for the
    /// placeholder state).
    ///
    /// [`Surface::jet`]'s `dvv`, projected (see [`Surface::deriv_u`]).
    pub fn deriv_vv(&self, u: T, v: T) -> Vec3<T> {
        self.jet(u, v).dvv
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI, TAU};

    use geom_core::{Dual, Dual64};
    use proptest::prelude::*;

    use super::*;

    // ------------------------------------------------------------------
    // Fixtures: an exactly orthonormal tilted frame (integer Pythagorean
    // triple over 3), so frame invariants hold to a few ulps, plus
    // axis-aligned variants for exact closed-form checks.
    // ------------------------------------------------------------------

    fn t_axis() -> Vec3<f64> {
        Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
    }

    fn t_uref() -> Vec3<f64> {
        Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
    }

    fn t_center() -> Point3<f64> {
        Point3::new(-0.5, 4.0, 1.25)
    }

    fn all_surfaces() -> Vec<(&'static str, Surface<f64>)> {
        vec![
            (
                "plane",
                Surface::Plane {
                    origin: t_center(),
                    normal: t_axis(),
                    u_ref: t_uref(),
                },
            ),
            (
                "cylinder",
                Surface::Cylinder {
                    origin: t_center(),
                    axis: t_axis(),
                    radius: 2.5,
                    u_ref: t_uref(),
                },
            ),
            (
                "cone",
                Surface::Cone {
                    apex: t_center(),
                    axis: t_axis(),
                    half_angle: FRAC_PI_6,
                    u_ref: t_uref(),
                },
            ),
            (
                "sphere",
                Surface::Sphere {
                    center: t_center(),
                    radius: 2.5,
                    axis: t_axis(),
                    u_ref: t_uref(),
                },
            ),
            (
                "torus",
                Surface::Torus {
                    center: t_center(),
                    axis: t_axis(),
                    major_radius: 3.0,
                    minor_radius: 1.25,
                    u_ref: t_uref(),
                },
            ),
        ]
    }

    /// Lifts an f64 surface to `Surface<Dual64>` with constant geometry
    /// (only evaluation parameters become variables).
    fn lift_dual(s: &Surface<f64>) -> Surface<Dual64> {
        match *s {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => Surface::Plane {
                origin: crate::scalar_lift::dual_point(origin),
                normal: crate::scalar_lift::dual_vec(normal),
                u_ref: crate::scalar_lift::dual_vec(u_ref),
            },
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Surface::Cylinder {
                origin: crate::scalar_lift::dual_point(origin),
                axis: crate::scalar_lift::dual_vec(axis),
                radius: Dual::constant(radius),
                u_ref: crate::scalar_lift::dual_vec(u_ref),
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Surface::Cone {
                apex: crate::scalar_lift::dual_point(apex),
                axis: crate::scalar_lift::dual_vec(axis),
                half_angle: Dual::constant(half_angle),
                u_ref: crate::scalar_lift::dual_vec(u_ref),
            },
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Surface::Sphere {
                center: crate::scalar_lift::dual_point(center),
                radius: Dual::constant(radius),
                axis: crate::scalar_lift::dual_vec(axis),
                u_ref: crate::scalar_lift::dual_vec(u_ref),
            },
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Surface::Torus {
                center: crate::scalar_lift::dual_point(center),
                axis: crate::scalar_lift::dual_vec(axis),
                major_radius: Dual::constant(major_radius),
                minor_radius: Dual::constant(minor_radius),
                u_ref: crate::scalar_lift::dual_vec(u_ref),
            },
            // The corpus below is analytic; neither spline kind lifts
            // to a `Dual` (there is no `NurbsSurface<Dual>` fixture).
            Surface::Nurbs(_) | Surface::Approx(_) => Surface::nurbs_placeholder(),
        }
    }

    fn close3(a: Vec3<f64>, b: Vec3<f64>, tol: f64) -> bool {
        (a.x - b.x).abs() <= tol && (a.y - b.y).abs() <= tol && (a.z - b.z).abs() <= tol
    }

    // ------------------------------------------------------------------
    // Closed-form loci (axis-aligned exact fixtures)
    // ------------------------------------------------------------------

    #[test]
    fn plane_evaluates_exactly() {
        let p = Surface::Plane {
            origin: Point3::new(1.0, 2.0, 3.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        // v_ref = z × x = y: S(2, -3) = (3, -1, 3), exactly.
        let q = p.eval(2.0, -3.0);
        assert_eq!((q.x, q.y, q.z), (3.0, -1.0, 3.0));
        // The chart normal is the stored normal, exactly (0/1 products).
        let n = p.normal(2.0, -3.0);
        assert_eq!((n.x, n.y, n.z), (0.0, 0.0, 1.0));
        // Derivatives: ∂u = u_ref, ∂v = v_ref, all seconds zero.
        let du = p.deriv_u(2.0, -3.0);
        assert_eq!((du.x, du.y, du.z), (1.0, 0.0, 0.0));
        let dv = p.deriv_v(2.0, -3.0);
        assert_eq!((dv.x, dv.y, dv.z), (0.0, 1.0, 0.0));
        assert_eq!(p.deriv_uu(2.0, -3.0).norm_squared(), 0.0);
        assert_eq!(p.deriv_uv(2.0, -3.0).norm_squared(), 0.0);
        assert_eq!(p.deriv_vv(2.0, -3.0).norm_squared(), 0.0);
    }

    #[test]
    fn cylinder_cardinal_points_and_normal() {
        let c = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 2.0,
            u_ref: Vec3::unit_x(),
        };
        // u = 0, v = 5: (2, 0, 5) exactly.
        let q = c.eval(0.0, 5.0);
        assert_eq!((q.x, q.y, q.z), (2.0, 0.0, 5.0));
        // Chart normal at u = 0 is +x (radially outward), exactly.
        let n = c.normal(0.0, 5.0);
        assert_eq!((n.x, n.y, n.z), (1.0, 0.0, 0.0));
        // u = π/2: (0, 2, 5) to rounding.
        let q = c.eval(FRAC_PI_2, 5.0);
        assert!((q.x - 0.0).abs() <= 1e-15 && (q.y - 2.0).abs() <= 1e-15);
    }

    #[test]
    fn sphere_cardinal_points() {
        let s = Surface::Sphere {
            center: Point3::new(1.0, 1.0, 1.0),
            radius: 2.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        // Equator seam: center + x·r.
        let q = s.eval(0.0, 0.0);
        assert_eq!((q.x, q.y, q.z), (3.0, 1.0, 1.0));
        // The normal there is +x, exactly.
        let n = s.normal(0.0, 0.0);
        assert_eq!((n.x, n.y, n.z), (1.0, 0.0, 0.0));
        // North pole (v = π/2): center + z·r to rounding.
        let q = s.eval(0.7, FRAC_PI_2);
        assert!((q.x - 1.0).abs() <= 1e-15);
        assert!((q.y - 1.0).abs() <= 1e-15);
        assert!((q.z - 3.0).abs() <= 1e-15);
    }

    #[test]
    fn torus_cardinal_points() {
        let t = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 3.0,
            minor_radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        // Outer equator at the seam: (R + r, 0, 0) exactly.
        let q = t.eval(0.0, 0.0);
        assert_eq!((q.x, q.y, q.z), (4.0, 0.0, 0.0));
        // Top of the tube at the seam: (R, 0, r) to rounding.
        let q = t.eval(0.0, FRAC_PI_2);
        assert!((q.x - 3.0).abs() <= 1e-15 && (q.z - 1.0).abs() <= 1e-15);
        // Inner equator: (R − r, 0, 0) to rounding.
        let q = t.eval(0.0, PI);
        assert!((q.x - 2.0).abs() <= 1e-15);
        // Outward normal on the outer equator: +x exactly.
        let n = t.normal(0.0, 0.0);
        assert_eq!((n.x, n.y, n.z), (1.0, 0.0, 0.0));
    }

    #[test]
    fn cone_slant_parameterization() {
        // Half-angle π/6: cos = √3/2, sin = 1/2.
        let c = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: FRAC_PI_6,
            u_ref: Vec3::unit_x(),
        };
        // v is slant length: |S(u, v) − apex| = |v| (unit generator).
        for v in [0.5, 2.0, -1.5] {
            let q = c.eval(0.3, v);
            assert!((q.distance(Point3::origin()) - v.abs()).abs() <= 1e-14);
        }
        // ∂v is unit (the generator direction).
        assert!((c.deriv_v(0.3, 2.0).norm() - 1.0).abs() <= 1e-15);
        // v > 0 nappe opens along +axis: z-component = v·cos α > 0.
        assert!(c.eval(0.3, 2.0).z > 0.0);
        assert!(c.eval(0.3, -2.0).z < 0.0);
        // The apex: parameterization singular AND surface singular —
        // ∂u = 0 exactly at v = 0, so the derived normal is poison.
        let du = c.deriv_u(0.9, 0.0);
        assert_eq!(du.norm_squared(), 0.0);
        let n = c.normal(0.9, 0.0);
        assert!(n.x.is_nan() && n.y.is_nan() && n.z.is_nan());
        // Away from the apex the normal is the documented closed form
        // radial·cos α − axis·sin α (v > 0).
        let n = c.normal(0.0, 2.0);
        let expect = Vec3::new(FRAC_PI_6.cos(), 0.0, -FRAC_PI_6.sin());
        assert!(close3(n, expect, 1e-15), "n = {n:?}");
    }

    // ------------------------------------------------------------------
    // Residual properties on the tilted frame (proptest)
    // ------------------------------------------------------------------

    proptest! {
        /// Every surface point satisfies its defining implicit residual
        /// on the tilted fixture (the locus checks).
        #[test]
        fn points_lie_on_their_loci(u in -10.0..10.0f64, v in -1.4..1.4f64) {
            let center = t_center();
            let axis = t_axis();
            for (name, s) in all_surfaces() {
                let p = s.eval(u, v);
                let d = p - center;
                let along = d.dot(axis);
                let radial_part = d - axis * along;
                let rho = radial_part.norm();
                let residual = match s {
                    Surface::Plane { .. } => along, // in-plane ⇔ no axis component
                    Surface::Cylinder { radius, .. } => rho - radius,
                    Surface::Cone { half_angle, .. } =>
                        // slant |v|: axis component v·cosα, radial |v|·sinα
                        rho - d.norm() * half_angle.sin(),
                    Surface::Sphere { radius, .. } => d.norm() - radius,
                    Surface::Torus { major_radius, minor_radius, .. } => {
                        let dr = rho - major_radius;
                        (dr.powi(2) + along.powi(2)).sqrt() - minor_radius
                    }
                    Surface::Nurbs(_) | Surface::Approx(_) => 0.0,
                };
                prop_assert!(
                    residual.abs() <= 1e-12,
                    "{name}: residual {residual:e} at (u, v) = ({u}, {v})"
                );
            }
        }

        /// Normals are unit and orthogonal to both first partials
        /// (away from chart singularities — v is kept off the poles and
        /// the apex by the generator ranges).
        #[test]
        fn normals_are_unit_and_orthogonal(
            u in -10.0..10.0f64,
            v in 0.1..1.4f64,
            flip in any::<bool>(),
        ) {
            let v = if flip { -v } else { v };
            for (name, s) in all_surfaces() {
                let n = s.normal(u, v);
                prop_assert!((n.norm() - 1.0).abs() <= 1e-12, "{name}: |n|");
                prop_assert!(n.dot(s.deriv_u(u, v)).abs() <= 1e-11, "{name}: n·∂u");
                prop_assert!(n.dot(s.deriv_v(u, v)).abs() <= 1e-11, "{name}: n·∂v");
            }
        }

        /// Derivative-vs-Dual consistency across ALL variants and both
        /// parameters — the M2 test axis: seeding u (then v) as the dual
        /// variable through `eval` reproduces `deriv_u` (then `deriv_v`)
        /// in the tangent channel, and the value channel is bit-identical
        /// to the f64 evaluation.
        #[test]
        fn first_derivatives_match_duals(u in -10.0..10.0f64, v in -1.4..1.4f64) {
            for (name, s) in all_surfaces() {
                let sd = lift_dual(&s);
                let pf = s.eval(u, v);

                let pu = sd.eval(Dual::variable(u), Dual::constant(v));
                for (channel, f64_val) in [(pu.x, pf.x), (pu.y, pf.y), (pu.z, pf.z)] {
                    prop_assert_eq!(channel.value.to_bits(), f64_val.to_bits(), "{}", name);
                }
                let du = s.deriv_u(u, v);
                prop_assert!((pu.x.deriv - du.x).abs() <= 1e-12, "{name}: ∂u x");
                prop_assert!((pu.y.deriv - du.y).abs() <= 1e-12, "{name}: ∂u y");
                prop_assert!((pu.z.deriv - du.z).abs() <= 1e-12, "{name}: ∂u z");

                let pv = sd.eval(Dual::constant(u), Dual::variable(v));
                let dv = s.deriv_v(u, v);
                prop_assert!((pv.x.deriv - dv.x).abs() <= 1e-12, "{name}: ∂v x");
                prop_assert!((pv.y.deriv - dv.y).abs() <= 1e-12, "{name}: ∂v y");
                prop_assert!((pv.z.deriv - dv.z).abs() <= 1e-12, "{name}: ∂v z");
            }
        }

        /// Second derivatives vs duals of the first derivatives — all
        /// three, plus the mixed-partial symmetry check (dual-of-∂u in v
        /// equals dual-of-∂v in u equals `deriv_uv`).
        #[test]
        fn second_derivatives_match_duals(u in -10.0..10.0f64, v in -1.4..1.4f64) {
            for (name, s) in all_surfaces() {
                let sd = lift_dual(&s);

                let duu = sd.deriv_u(Dual::variable(u), Dual::constant(v));
                let uu = s.deriv_uu(u, v);
                prop_assert!((duu.x.deriv - uu.x).abs() <= 1e-12, "{name}: ∂uu");
                prop_assert!((duu.y.deriv - uu.y).abs() <= 1e-12, "{name}: ∂uu");
                prop_assert!((duu.z.deriv - uu.z).abs() <= 1e-12, "{name}: ∂uu");

                let duv = sd.deriv_u(Dual::constant(u), Dual::variable(v));
                let vu = sd.deriv_v(Dual::variable(u), Dual::constant(v));
                let uv = s.deriv_uv(u, v);
                for (a, b) in [(duv.x, vu.x), (duv.y, vu.y), (duv.z, vu.z)] {
                    // Mixed-partial symmetry between the two dual routes.
                    prop_assert!((a.deriv - b.deriv).abs() <= 1e-12, "{name}: symmetry");
                }
                prop_assert!((duv.x.deriv - uv.x).abs() <= 1e-12, "{name}: ∂uv");
                prop_assert!((duv.y.deriv - uv.y).abs() <= 1e-12, "{name}: ∂uv");
                prop_assert!((duv.z.deriv - uv.z).abs() <= 1e-12, "{name}: ∂uv");

                let dvv = sd.deriv_v(Dual::constant(u), Dual::variable(v));
                let vv = s.deriv_vv(u, v);
                prop_assert!((dvv.x.deriv - vv.x).abs() <= 1e-12, "{name}: ∂vv");
                prop_assert!((dvv.y.deriv - vv.y).abs() <= 1e-12, "{name}: ∂vv");
                prop_assert!((dvv.z.deriv - vv.z).abs() <= 1e-12, "{name}: ∂vv");
            }
        }

        /// Periodicity in u at the value level (the honest statement —
        /// see the crate docs; never bitwise).
        #[test]
        fn azimuthal_periodicity_value_level(
            u in -3.0..3.0f64,
            v in -1.4..1.4f64,
            k in -20i32..20,
        ) {
            for (name, s) in all_surfaces() {
                if matches!(s, Surface::Plane { .. }) {
                    continue; // the plane is not periodic in u
                }
                let p = s.eval(u, v);
                let q = s.eval(u + f64::from(k) * TAU, v);
                let slack = 1e-14 + 2e-14 * f64::from(k).abs();
                prop_assert!(
                    (p.x - q.x).abs() <= slack
                        && (p.y - q.y).abs() <= slack
                        && (p.z - q.z).abs() <= slack,
                    "{name}: k = {k}"
                );
            }
        }
    }

    // ------------------------------------------------------------------
    // Chart singularities, honestly
    // ------------------------------------------------------------------

    #[test]
    fn sphere_pole_chart_defect() {
        let s = Surface::Sphere {
            center: Point3::origin(),
            radius: 2.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        // In exact arithmetic ∂u vanishes at v = π/2; fl(π/2) is not
        // π/2, so at f64 the pole is *approached*, not hit: ∂u is tiny
        // but nonzero, and the normal is still computable and outward.
        let du = s.deriv_u(0.7, FRAC_PI_2);
        assert!(du.norm() <= 1e-15, "|∂u| = {}", du.norm());
        assert!(du.norm() > 0.0, "fl(π/2) does not hit the exact pole");
        let n = s.normal(0.7, FRAC_PI_2);
        assert!((n.z - 1.0).abs() <= 1e-12);
        // The exact chart singularity is reachable with an exactly-zero
        // ∂u factor via a v with cos v = 0 unreachable at f64 — but the
        // POISON path is exact at the cone apex (tested there). Here we
        // pin the honest near-pole conditioning: |∂u| collapses while
        // |∂v| stays r.
        assert!((s.deriv_v(0.7, FRAC_PI_2).norm() - 2.0).abs() <= 1e-14);
    }

    // ------------------------------------------------------------------
    // Totality and poison
    // ------------------------------------------------------------------

    #[test]
    fn nurbs_placeholder_evaluates_to_poison() {
        let n: Surface<f64> = Surface::nurbs_placeholder();
        assert!(n.eval(0.5, 0.5).x.is_nan());
        assert!(n.deriv_u(0.5, 0.5).x.is_nan());
        assert!(n.deriv_v(0.5, 0.5).x.is_nan());
        assert!(n.normal(0.5, 0.5).x.is_nan());
        assert!(n.deriv_uu(0.5, 0.5).x.is_nan());
        assert!(n.deriv_uv(0.5, 0.5).x.is_nan());
        assert!(n.deriv_vv(0.5, 0.5).x.is_nan());
    }

    #[test]
    fn poison_parameters_poison_points_and_no_panic_on_extremes() {
        for (name, s) in all_surfaces() {
            let p = s.eval(f64::NAN, 0.5);
            assert!(
                p.x.is_nan() || p.y.is_nan() || p.z.is_nan(),
                "{name}: NaN u must poison"
            );
            let p = s.eval(0.5, f64::NAN);
            assert!(
                p.x.is_nan() || p.y.is_nan() || p.z.is_nan(),
                "{name}: NaN v must poison"
            );
            // Totality on extremes: no panic, values or poison.
            for e in [f64::INFINITY, f64::NEG_INFINITY, 1e300, f64::MAX] {
                let _ = s.eval(e, e);
                let _ = s.normal(e, e);
                let _ = s.deriv_uu(e, e);
            }
        }
    }

    // ------------------------------------------------------------------
    // Interval instantiation (feature-gated)
    // ------------------------------------------------------------------

    #[cfg(feature = "interval")]
    mod interval {
        use geom_core::{Bounds, Interval};

        use super::*;

        fn lift(s: &Surface<f64>) -> Surface<Interval> {
            match *s {
                Surface::Plane {
                    origin,
                    normal,
                    u_ref,
                } => Surface::Plane {
                    origin: crate::scalar_lift::interval_point(origin),
                    normal: crate::scalar_lift::interval_vec(normal),
                    u_ref: crate::scalar_lift::interval_vec(u_ref),
                },
                Surface::Cylinder {
                    origin,
                    axis,
                    radius,
                    u_ref,
                } => Surface::Cylinder {
                    origin: crate::scalar_lift::interval_point(origin),
                    axis: crate::scalar_lift::interval_vec(axis),
                    radius: Interval::from_f64(radius),
                    u_ref: crate::scalar_lift::interval_vec(u_ref),
                },
                Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    u_ref,
                } => Surface::Cone {
                    apex: crate::scalar_lift::interval_point(apex),
                    axis: crate::scalar_lift::interval_vec(axis),
                    half_angle: Interval::from_f64(half_angle),
                    u_ref: crate::scalar_lift::interval_vec(u_ref),
                },
                Surface::Sphere {
                    center,
                    radius,
                    axis,
                    u_ref,
                } => Surface::Sphere {
                    center: crate::scalar_lift::interval_point(center),
                    radius: Interval::from_f64(radius),
                    axis: crate::scalar_lift::interval_vec(axis),
                    u_ref: crate::scalar_lift::interval_vec(u_ref),
                },
                Surface::Torus {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    u_ref,
                } => Surface::Torus {
                    center: crate::scalar_lift::interval_point(center),
                    axis: crate::scalar_lift::interval_vec(axis),
                    major_radius: Interval::from_f64(major_radius),
                    minor_radius: Interval::from_f64(minor_radius),
                    u_ref: crate::scalar_lift::interval_vec(u_ref),
                },
                // As the dual lift: the corpus is analytic, and neither
                // spline kind has an interval fixture to lift.
                Surface::Nurbs(_) | Surface::Approx(_) => Surface::nurbs_placeholder(),
            }
        }

        fn contains(e: Interval, x: f64) -> bool {
            e.lo() <= x && x <= e.hi()
        }

        /// EVERY evaluator instantiates at `Interval`, and the defining
        /// residuals enclose zero (truth containment through identities
        /// — the transcendental-safe assertion form). Uses the sphere
        /// (|P − c|² − r²) and cylinder (ρ² − r²) as the spot checks,
        /// and exercises eval/deriv_u/deriv_v/normal/seconds on all
        /// variants for instantiation coverage.
        #[test]
        fn evaluators_instantiate_and_residuals_enclose_zero() {
            for (name, s) in all_surfaces() {
                let si = lift(&s);
                let (u, v) = (Interval::from_f64(0.7), Interval::from_f64(0.4));
                // Instantiation coverage: every method at interval type.
                let _ = si.eval(u, v);
                let _ = si.deriv_u(u, v);
                let _ = si.deriv_v(u, v);
                let _ = si.normal(u, v);
                let _ = si.deriv_uu(u, v);
                let _ = si.deriv_uv(u, v);
                let _ = si.deriv_vv(u, v);
                // Normal ⊥ partials, as an enclosure of 0.
                let n = si.normal(u, v);
                let orth = n.dot(si.deriv_u(u, v));
                assert!(
                    contains(orth, 0.0),
                    "{name}: n·∂u = [{}, {}]",
                    orth.lo(),
                    orth.hi()
                );
            }

            let sphere = lift(&all_surfaces()[3].1);
            let (center, r) = match sphere {
                Surface::Sphere { center, radius, .. } => (center, radius),
                _ => panic!("fixture order"),
            };
            for (uu, vv) in [(0.0, 0.0), (0.7, 0.4), (-3.0, 1.2), (100.0, -0.9)] {
                let p = sphere.eval(Interval::from_f64(uu), Interval::from_f64(vv));
                let res = (p - center).norm_squared() - r.powi(2);
                assert!(contains(res, 0.0), "sphere residual at ({uu}, {vv})");
                assert!(res.hi() - res.lo() < 1e-12);
            }

            let cyl = lift(&all_surfaces()[1].1);
            let (origin, axis, r) = match cyl {
                Surface::Cylinder {
                    origin,
                    axis,
                    radius,
                    ..
                } => (origin, axis, radius),
                _ => panic!("fixture order"),
            };
            for (uu, vv) in [(0.0, 0.0), (2.9, -4.0), (-11.0, 7.5)] {
                let p = cyl.eval(Interval::from_f64(uu), Interval::from_f64(vv));
                let d = p - origin;
                let rho2 = d.norm_squared() - d.dot(axis).powi(2);
                let res = rho2 - r.powi(2);
                assert!(contains(res, 0.0), "cylinder residual at ({uu}, {vv})");
            }
        }

        /// The plane evaluator is exact-ops only, so the f64 evaluation
        /// is contained in the interval evaluation.
        #[test]
        fn plane_encloses_f64_evaluation() {
            let p = all_surfaces()[0].1.clone();
            let pi_ = lift(&p);
            for (uu, vv) in [(0.0, 0.0), (1.75, -3.5), (1234.5, 0.125)] {
                let q = p.eval(uu, vv);
                let qi = pi_.eval(Interval::from_f64(uu), Interval::from_f64(vv));
                assert!(contains(qi.x, q.x) && contains(qi.y, q.y) && contains(qi.z, q.z));
            }
        }

        /// An input box maps to an output box containing the images of
        /// sampled inner parameters — enclosure containment through a
        /// genuinely wide box (inclusion monotonicity at the evaluator
        /// level; f64 sample images are asserted against a box widened
        /// by the box arithmetic itself, using exact-op variants only).
        #[test]
        fn wide_boxes_enclose_sampled_images() {
            let p = all_surfaces()[0].1.clone(); // plane: exact ops, assertable
            let pi_ = lift(&p);
            let ub = Interval::from_bounds(-1.0, 2.0);
            let vb = Interval::from_bounds(0.5, 0.75);
            let img = pi_.eval(ub, vb);
            for uu in [-1.0, -0.3, 0.9, 2.0] {
                for vv in [0.5, 0.6, 0.75] {
                    let q = p.eval(uu, vv);
                    assert!(
                        contains(img.x, q.x) && contains(img.y, q.y) && contains(img.z, q.z),
                        "sample ({uu}, {vv}) escapes the box image"
                    );
                }
            }
        }

        /// NaI in → NaI out, and the Nurbs placeholder poisons at
        /// interval type.
        #[test]
        fn poison_propagates_at_interval() {
            let si = lift(&all_surfaces()[1].1);
            let p = si.eval(Interval::from_f64(f64::NAN), Interval::zero());
            assert!(p.x.lo().is_nan());
            let n: Surface<Interval> = Surface::nurbs_placeholder();
            assert!(n.eval(Interval::zero(), Interval::zero()).x.lo().is_nan());
        }
    }
}

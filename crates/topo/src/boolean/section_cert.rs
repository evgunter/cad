//! **The section certificate: a face pair's section, classified, and
//! every component of it either evidenced by the crossing layer or
//! proved not to lie inside both faces.**
//!
//! # The class it closes
//!
//! A face `F` of one operand can meet a face `G` of the other in a
//! closed loop interior to both faces: a loop that touches no edge of
//! either body. No edge event marks it, the join cuts nothing along it,
//! and on the crossings path face-region propagation carries each face's
//! side across it from regions the crossings elsewhere did cut. On the
//! no-crossings path the vertex probe sees each shell outside the other.
//! Either way the result is a valid body that is wrong: the overlap is
//! counted twice under ∪ and dropped under ∩ and ∖.
//!
//! # The evidence lemma (L1)
//!
//! Let `Σ = carrier(F) ∩ carrier(G)` and `γ` a connected component of
//! it. If `γ ∩ F ∩ G ≠ ∅` and `γ ⊄ int F ∩ int G`, some point `x` of
//! `γ ∩ F ∩ G` lies on an edge of one face inside the other: either
//! `γ ⊂ F ∩ G` and `x` is a point of `γ` on a boundary, or `x` is a
//! relative boundary point of the proper closed subset `γ ∩ F ∩ G` of
//! the connected `γ`. The sweep examines every box-overlapping
//! edge × face pair and records the contact there, or refuses (premise
//! **S**). So the components no event can evidence are exactly those
//! with `γ ⊂ int F ∩ int G`, and the certificate's job is to prove, per
//! component, that `γ ∩ F ∩ G = ∅` or `γ ⊄ int F ∩ int G`.
//!
//! An event on a pair is a point of SOME component: with `Σ = γ₁ ∪ γ₂`
//! an event on `γ₁` says nothing of `γ₂`. So events are read in two
//! places only: W4, where the section has exactly one component, and
//! the no-event decision.
//!
//! # The witnesses, per component
//!
//! - **W0, apart**: the classification returns no component.
//! - **W1, unbounded**: faces are compact, so `γ ⊄ F`.
//! - **W2, essential**: `γ` is essential on the carrier of `X` (it winds
//!   about the axis, or around the tube) and `chart_boundary(X)` returns
//!   `Ok`: `X`'s loops lift into one sheet of the chart's cover, so every
//!   closed curve in `X` lifts to a closed curve with zero winding in
//!   every periodic channel, and `γ ⊄ X`. This is checked per FACE, never
//!   assumed of a kind: a seamless periodic band — one face closing on
//!   itself with no meridian edge — fails the check and loses W2.
//! - **W3, a witness point `Out`**: one closed-form point of `γ`,
//!   certified outside `F` or outside `G` (by the chart trim on a curved
//!   face, by `contfp` on a plane), so `γ ⊄ F ∩ G`.
//! - **W4, a lone evented component**: `Σ` has exactly one component and
//!   the pair has an event. The event's vertex lies on `Σ` within the
//!   band, hence on `γ`, and on an edge of `F` or `G`, so
//!   `γ ⊄ int F ∩ int G`.
//! - **The no-event decision**: with no event on the pair,
//!   `γ ∩ F ∩ G` has no relative boundary point, so it is empty or all
//!   of `γ`, and one point decides. A witness strictly inside both faces
//!   is a **definite interior loop** (R-loop). Anything else, a witness
//!   on a boundary or with no verdict, refuses undecided (R-undec).
//!
//! One witness point per component is complete on the arms below: a
//! lone component is cleared by W4 or decided by its point, and every
//! multi-component section is essential or unbounded on a carrier, which
//! W1 and W2 clear. A pair that has an event and a multi-component
//! section with a component no witness clears refuses R-undec: no arm
//! below produces one on faces that describe, and the refusal is the
//! guard for the day an arm does.
//!
//! # The refusals
//!
//! - **R-reach**: a kind pair or pose with no arm (torus against an
//!   oblique cylinder, a non-coaxial torus, any cone or NURBS face).
//! - **R-tan**: a classification margin `Zero` or undecided — a
//!   tangency, where components pinch and the count is not certified.
//! - **R-loop** and **R-undec**: the no-event decision, above.
//! - **Lone-vertex loops** refuse per pair (below).
//!
//! # Premises, each at its site
//!
//! - **S**, sweep completeness. It holds with the conic × plane lane
//!   examining every root (`reduce.rs` `sweep_direction`), not only the
//!   first.
//! - **The ring torus**, `R > r > 0` (`geom::require_ring_torus`): the
//!   component counts of the torus arms need genus 1.
//! - **Event vertices lie on `Σ` within the band**, the posture every
//!   contact record takes.
//! - **Witness points are within rounding of `Σ`** on `f64`, and
//!   enclosures of it on the interval lane; containment is decided on the
//!   band, and a point it cannot decide is not a witness.
//! - **Lone-vertex loops.** The sweep is edge-driven, so a vertex of a
//!   `LoopBoundary::Empty` loop is never examined. That does not break
//!   L1 on its own: `F` surrounds its lone vertex on every side, so the
//!   vertex is a relative boundary point of `γ ∩ F ∩ G` only where it
//!   also lies on `∂G`, whose edges the sweep does examine; and
//!   `chart_boundary` reads the face's region as its cycles bound it, so
//!   W2 covers the vertex too. What the sweep cannot see is a component
//!   in `F ∩ G` whose only boundary point is the lone vertex. That is an
//!   interior loop to every witness here except that it touches a
//!   vertex; the no-event decision would refuse it anyway, but nothing
//!   builds such a face at rest (tier 2 bans empty loops), so a pair
//!   whose face carries one refuses outright rather than resting an
//!   answer on that argument.
//!
//! # The angular margins (the lever)
//!
//! Two margins are angles: the tilt between two axes, decided before a
//! coaxial or parallel-axis arm is taken. `decide` meters in metres, so
//! each is levered by the pair's **extent**: the diagonal of the overlap
//! of the two faces' certified boxes, the region where the section can
//! matter. An axis tilted by `ε` moves the section by at most `ε·extent`
//! there.
//!
//! # What the arms say (component structure per kind pair)
//!
//! - **Torus × plane, torus × sphere** (one classification). In the
//!   meridian plane `Π` holding the axis and the plane's normal (or the
//!   sphere's centre), let `T₊`, `T₋` be the tube circles at the near
//!   and far core points. The section has roots at `u' = 0` iff the
//!   partner cuts `T₊`, and at `u' = π` iff it cuts `T₋`. Both cut: two
//!   `(1,0)` curves (essential on the torus). One cut: a single null
//!   oval, the scrape, whose points on `Π` are the partner's crossings
//!   with that tube circle (one square root). Neither: two `(0,1)` curves
//!   when the partner separates `T₊` from `T₋`, else none. A
//!   two-component section is therefore always essential.
//! - **Torus × coaxial {cylinder, torus}**: parallels, essential on both.
//! - **Torus × parallel-axis cylinder** (axis offset `e`): with
//!   `ρ ∈ [|e − ρc|, e + ρc]` against `[R − r, R + r]`, inside gives two
//!   loops encircling the cylinder, across one bound a single null loop
//!   (witness at the extreme ruling), across both two `(0,1)` loops.
//! - **Cylinder × cylinder** (common-perpendicular offset `δ₀`):
//!   `|δ₀| + r_min < r_max` gives two loops encircling the thinner
//!   axis; `|r₁ − r₂| < |δ₀| < r₁ + r₂` a single null saddle loop,
//!   witnessed on the arc's middle ruling; parallel axes give rulings
//!   (lines).
//! - **Cylinder × plane**: an ellipse encircling the wall, or rulings.
//!   Either way the section cannot lie in a wall that describes. The
//!   count is not certified (two rulings, or one ellipse), so W4 is
//!   never read here and the pair clears by W2 or refuses.
//! - **Sphere × {plane, sphere}**: one circle. **Sphere × cylinder**
//!   (axis offset `e`): one loop when `|e − ρc| < ρs < e + ρc`,
//!   witnessed on the nearest ruling; two loops encircling the cylinder
//!   when `e + ρc < ρs`.
//! - **Cone pairs** have no arm yet and refuse on reach; the arms land
//!   with the cone's operand admission, in [`classify`]'s match.
//!
//! # What this replaced
//!
//! The no-crossings path's two blanket gates refused a torus face, and
//! a pair of cylinder walls, whose boxes overlapped a partner's. Their
//! arguments are these arms: a plane section of a wall is essential or
//! unbounded (the plane exemption the wall gate made, which rested on
//! every wall carrying a meridian edge — now checked per face by W2); a
//! torus meets a plane in a scrape oval interior to both faces, which a
//! blanket gate could only refuse. The conservative refusal they were
//! known for — a cube standing in a donut's hole — is W0 or W2 here.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::contain::FaceContainment;
use crate::validate::decide;

/// Which face of the pair a statement is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    /// The pair's first face.
    F,
    /// The pair's second face.
    G,
}

/// One connected component of a pair's section.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Component<T: Real> {
    /// The component is unbounded (a line).
    pub unbounded: bool,
    /// It cannot lie inside `F` if `F` describes: essential on `F`'s
    /// carrier (or, on a cylinder × plane pair, essential or unbounded).
    pub essential_f: bool,
    /// The same for `G`.
    pub essential_g: bool,
    /// One point of the component, in closed form.
    pub witness: Option<Point3<T>>,
}

/// A pair's section, classified.
#[derive(Clone, Debug)]
pub(crate) enum Section<T: Real> {
    /// The components, and whether their count is CERTIFIED to be one
    /// (`single`), which W4 needs.
    Components {
        /// Every component.
        parts: Vec<Component<T>>,
        /// The section has exactly one component.
        single: bool,
    },
    /// A classification margin is `Zero` or undecided: R-tan, naming the
    /// predicate.
    Tangent(&'static str),
    /// No arm for the kind pair or the pose: R-reach.
    Intractable,
}

/// Why a component was cleared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cleared {
    /// W1.
    Unbounded,
    /// W2, on the named face.
    Essential(Side),
    /// W3, on the named face.
    Out(Side),
    /// W4.
    LoneEvented,
}

/// Why a pair refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Refusal {
    /// R-reach: no arm.
    Reach,
    /// R-tan, with the margin's predicate.
    Tangent(&'static str),
    /// R-loop: a component certified inside both faces, no event.
    Loop,
    /// R-undec: no witness cleared a component and none proved it
    /// interior.
    Undecided,
    /// A face of the pair carries a lone-vertex loop.
    LoneVertex,
}

impl Refusal {
    /// The sentence the no-crossings path's refusal carries.
    pub(crate) fn what(self) -> &'static str {
        match self {
            Self::Reach => {
                "a curved face's box overlaps a face of the other solid, no crossing layer saw \
                 an event, and the pair has no section classification (an oblique or \
                 non-coaxial pose, a cone or a spline face) to prove the two faces do not meet \
                 in a closed loop interior to both"
            }
            Self::Tangent(_) => {
                "a curved face and a face of the other solid have tangent or near-tangent \
                 carriers and no crossing layer saw an event; the section's component count is \
                 not certified there"
            }
            Self::Loop => {
                "a curved face meets a face of the other solid in a closed loop interior to \
                 both faces, which no edge event and no vertex probe can see; the loop is \
                 certified, and cutting it in is not built"
            }
            Self::Undecided => {
                "a curved face's section with a face of the other solid has a component no \
                 witness could place in or out of both faces, and no crossing layer saw an \
                 event on it"
            }
            Self::LoneVertex => {
                "a face of the pair carries a lone-vertex loop, which the crossing layer does \
                 not examine"
            }
        }
    }
}

/// The unit vector along `v`.
fn unit<T: Real>(v: Vec3<T>) -> Vec3<T> {
    v / v.norm()
}

/// A margin's sign, `None` when undecided.
fn sign<T: Decide>(name: &'static str, m: Margin<T>, band: Band) -> Option<Sign> {
    decide(name, m, band).ok()
}

/// Decides every margin in `ms`; the first `Zero` or undecided one is
/// the pair's tangency.
fn signs<T: Decide, const N: usize>(
    ms: [(&'static str, T); N],
    band: Band,
) -> Result<[bool; N], Section<T>> {
    let mut out = [false; N];
    for (slot, (name, m)) in out.iter_mut().zip(ms) {
        match sign(name, Margin::of(m), band) {
            Some(Sign::Positive) => *slot = true,
            Some(Sign::Negative) => {}
            Some(Sign::Zero) | None => return Err(Section::Tangent(name)),
        }
    }
    Ok(out)
}

fn none<T: Real>() -> Section<T> {
    Section::Components {
        parts: Vec::new(),
        single: false,
    }
}

fn lone<T: Real>(witness: Point3<T>) -> Section<T> {
    Section::Components {
        parts: vec![Component {
            unbounded: false,
            essential_f: false,
            essential_g: false,
            witness: Some(witness),
        }],
        single: true,
    }
}

fn essential_pair<T: Real>(on_f: bool, on_g: bool) -> Section<T> {
    let c = Component {
        unbounded: false,
        essential_f: on_f,
        essential_g: on_g,
        witness: None,
    };
    Section::Components {
        parts: vec![c, c],
        single: false,
    }
}

/// The same classification with the roles of `F` and `G` exchanged.
fn swapped<T: Real>(s: Section<T>) -> Section<T> {
    match s {
        Section::Components { parts, single } => Section::Components {
            parts: parts
                .into_iter()
                .map(|c| Component {
                    essential_f: c.essential_g,
                    essential_g: c.essential_f,
                    ..c
                })
                .collect(),
            single,
        },
        other => other,
    }
}

/// **The classifier: two carriers in, the section's components out.**
/// Pure: surfaces, the angular lever (module docs) and the band.
pub(crate) fn classify<T: Decide>(
    f: &geom::Surface<T>,
    g: &geom::Surface<T>,
    lever: T,
    band: Band,
) -> Section<T> {
    use geom::Surface as S;
    match (f, g) {
        (
            S::Torus { .. },
            S::Plane { .. } | S::Sphere { .. } | S::Cylinder { .. } | S::Torus { .. },
        ) => torus_pair(f, g, lever, band),
        (S::Plane { .. } | S::Sphere { .. } | S::Cylinder { .. }, S::Torus { .. }) => {
            swapped(torus_pair(g, f, lever, band))
        }
        (
            &S::Cylinder {
                origin: o1,
                axis: d1,
                radius: r1,
                ..
            },
            &S::Cylinder {
                origin: o2,
                axis: d2,
                radius: r2,
                ..
            },
        ) => cylinder_cylinder(o1, unit(d1), r1, o2, unit(d2), r2, lever, band),
        (S::Cylinder { .. }, S::Plane { .. }) => cylinder_plane(),
        (S::Plane { .. }, S::Cylinder { .. }) => swapped(cylinder_plane()),
        (&S::Sphere { center, radius, .. }, &S::Plane { origin, normal, .. }) => {
            sphere_plane(center, radius, origin, unit(normal), band)
        }
        (&S::Plane { origin, normal, .. }, &S::Sphere { center, radius, .. }) => {
            sphere_plane(center, radius, origin, unit(normal), band)
        }
        (
            &S::Sphere {
                center: c1,
                radius: r1,
                ..
            },
            &S::Sphere {
                center: c2,
                radius: r2,
                ..
            },
        ) => sphere_sphere(c1, r1, c2, r2, band),
        (
            &S::Sphere { center, radius, .. },
            &S::Cylinder {
                origin,
                axis,
                radius: rc,
                ..
            },
        ) => sphere_cylinder(center, radius, origin, unit(axis), rc, band),
        (
            &S::Cylinder {
                origin,
                axis,
                radius: rc,
                ..
            },
            &S::Sphere { center, radius, .. },
        ) => swapped(sphere_cylinder(
            center,
            radius,
            origin,
            unit(axis),
            rc,
            band,
        )),
        // The cone arms (plane: every component essential or unbounded;
        // sphere: one null loop or encircling ones; coaxial and
        // parallel-axis partners: encircling) land here with the cone's
        // operand admission. Until then a cone pair, like every pair
        // with no arm, refuses on reach.
        _ => Section::Intractable,
    }
}

/// The torus arms: `t` is a torus, `p` its partner.
fn torus_pair<T: Decide>(
    t: &geom::Surface<T>,
    p: &geom::Surface<T>,
    lever: T,
    band: Band,
) -> Section<T> {
    let &geom::Surface::Torus {
        center: c,
        axis,
        major_radius: big_r,
        minor_radius: r,
        ..
    } = t
    else {
        return Section::Intractable;
    };
    if geom::require_ring_torus(big_r, r, band).is_err() {
        return Section::Intractable;
    }
    let a = unit(axis);
    match *p {
        geom::Surface::Plane { origin, normal, .. } => {
            torus_plane(c, a, big_r, r, origin, unit(normal), band)
        }
        geom::Surface::Sphere { center, radius, .. } => {
            torus_sphere(c, a, big_r, r, center, radius, band)
        }
        geom::Surface::Cylinder {
            origin,
            axis: d,
            radius: rc,
            ..
        } => match axis_pose(c, a, origin, unit(d), lever, band) {
            Pose::Coaxial => {
                // The cylinder's meridian is the line ρ = ρc against the
                // tube circle.
                match signs(
                    [("section_torus_coaxial_wall", r - (rc - big_r).abs())],
                    band,
                ) {
                    Ok([true]) => essential_pair(true, true),
                    Ok([false]) => none(),
                    Err(tan) => tan,
                }
            }
            Pose::Parallel { e, toward } => {
                torus_parallel_cylinder(c, a, big_r, r, e, toward, rc, band)
            }
            Pose::Other => Section::Intractable,
        },
        geom::Surface::Torus {
            center: c2,
            axis: a2,
            major_radius: big_r2,
            minor_radius: r2,
            ..
        } => {
            if geom::require_ring_torus(big_r2, r2, band).is_err() {
                return Section::Intractable;
            }
            match axis_pose(c, a, c2, unit(a2), lever, band) {
                Pose::Coaxial => {
                    let z2 = (c2 - c).dot(a);
                    let d = Vec3::new(big_r2 - big_r, z2, T::zero()).norm();
                    match signs(
                        [
                            ("section_torus_coaxial_tube_reach", r + r2 - d),
                            ("section_torus_coaxial_tube_nest", d - (r - r2).abs()),
                        ],
                        band,
                    ) {
                        Ok([true, true]) => essential_pair(true, true),
                        Ok(_) => none(),
                        Err(tan) => tan,
                    }
                }
                Pose::Parallel { .. } | Pose::Other => Section::Intractable,
            }
        }
        _ => Section::Intractable,
    }
}

/// How a partner's axis stands to the torus axis.
enum Pose<T: Real> {
    /// One axis line.
    Coaxial,
    /// Parallel, at offset `e`, with `toward` the unit direction from the
    /// torus axis to the partner's.
    Parallel { e: T, toward: Vec3<T> },
    /// Tilted, or undecided.
    Other,
}

/// Decides the axis pose: tilt levered by the pair's extent, offset in
/// metres. An undecided margin is not a pose any arm here claims.
fn axis_pose<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    o: Point3<T>,
    d: Vec3<T>,
    lever: T,
    band: Band,
) -> Pose<T> {
    let tilt = a.cross(d).norm();
    if sign("section_axes_tilt", Margin::levered(tilt, lever), band) != Some(Sign::Zero) {
        return Pose::Other;
    }
    let w = o - c;
    let perp = w - a * w.dot(a);
    let e = perp.norm();
    match sign("section_axes_offset", Margin::of(e), band) {
        Some(Sign::Zero) => Pose::Coaxial,
        Some(Sign::Positive) => Pose::Parallel {
            e,
            toward: perp / e,
        },
        _ => Pose::Other,
    }
}

/// **Torus × plane**, the plane through `p0` with unit normal `n`.
fn torus_plane<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    big_r: T,
    r: T,
    p0: Point3<T>,
    n: Vec3<T>,
    band: Band,
) -> Section<T> {
    let h = n.dot(p0 - c);
    let na = n.dot(a);
    let n_perp = n - a * na;
    let s = n_perp.norm();
    let [near, far] = match signs(
        [
            ("section_torus_plane_near_tube", r - (h - s * big_r).abs()),
            ("section_torus_plane_far_tube", r - (h + s * big_r).abs()),
        ],
        band,
    ) {
        Ok(x) => x,
        Err(tan) => return tan,
    };
    match (near, far) {
        (true, true) => essential_pair(true, false),
        (false, false) => {
            // The plane separates the two tube circles (two `(0,1)`
            // curves) or misses the torus.
            match signs(
                [("section_torus_plane_between", s * big_r - r - h.abs())],
                band,
            ) {
                Ok([true]) => essential_pair(true, false),
                Ok([false]) => none(),
                Err(tan) => tan,
            }
        }
        (true, false) | (false, true) => {
            let sigma = if near { T::one() } else { -T::one() };
            // In Π, with x along ê = n⊥/s and y along a: the tube circle
            // T_σ is centred (σR, 0), radius r, and the plane is the line
            // s·x + n_a·y = h, whose unit normal is (s, n_a).
            let e_hat = n_perp / s;
            let d = h - sigma * s * big_r;
            let (fx, fy) = (sigma * big_r + d * s, d * na);
            let half = ((r - d.abs()) * (r + d.abs())).sqrt();
            let (x, y) = (fx - na * half, fy + s * half);
            lone(c + e_hat * x + a * y)
        }
    }
}

/// **Torus × sphere**, the ball `(cs, rho)`.
fn torus_sphere<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    big_r: T,
    r: T,
    cs: Point3<T>,
    rho: T,
    band: Band,
) -> Section<T> {
    let w = cs - c;
    let ca = w.dot(a);
    let w_perp = w - a * ca;
    let s = w_perp.norm();
    let dist = |x: T| Vec3::new(x, ca, T::zero()).norm();
    let (d_near, d_far) = (dist(s - big_r), dist(s + big_r));
    let cuts = |d: T| (d + r - rho).min(rho - (d - r).abs());
    let [near, far] = match signs(
        [
            ("section_torus_sphere_near_tube", cuts(d_near)),
            ("section_torus_sphere_far_tube", cuts(d_far)),
        ],
        band,
    ) {
        Ok(x) => x,
        Err(tan) => return tan,
    };
    match (near, far) {
        (true, true) => essential_pair(true, false),
        (false, false) => match signs(
            [(
                "section_torus_sphere_between",
                (rho - d_near - r).min(d_far - r - rho),
            )],
            band,
        ) {
            Ok([true]) => essential_pair(true, false),
            Ok([false]) => none(),
            Err(tan) => tan,
        },
        (true, false) | (false, true) => {
            let sigma = if near { T::one() } else { -T::one() };
            let d = if near { d_near } else { d_far };
            // Circle against circle in Π: T_σ centred (σR, 0) radius r,
            // the sphere's great circle centred (s, c_a) radius ρ.
            let e_hat = w_perp / s;
            let (ex, ey) = ((s - sigma * big_r) / d, ca / d);
            let along = (d * d + r * r - rho * rho) / (d + d);
            let half = ((r - along) * (r + along)).sqrt();
            let (x, y) = (
                sigma * big_r + ex * along - ey * half,
                ey * along + ex * half,
            );
            lone(c + e_hat * x + a * y)
        }
    }
}

/// **Torus × a cylinder whose axis is parallel to the torus's**, at
/// offset `e` in the direction `toward`.
#[allow(clippy::too_many_arguments)]
fn torus_parallel_cylinder<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    big_r: T,
    r: T,
    e: T,
    toward: Vec3<T>,
    rc: T,
    band: Band,
) -> Section<T> {
    let (rho_min, rho_max) = ((e - rc).abs(), e + rc);
    let (lo, hi) = (big_r - r, big_r + r);
    let [min_over_lo, min_under_hi, max_over_lo, max_under_hi] = match signs(
        [
            ("section_torus_offset_wall_near_inner", rho_min - lo),
            ("section_torus_offset_wall_near_outer", hi - rho_min),
            ("section_torus_offset_wall_far_inner", rho_max - lo),
            ("section_torus_offset_wall_far_outer", hi - rho_max),
        ],
        band,
    ) {
        Ok(x) => x,
        Err(tan) => return tan,
    };
    let height = |rho: T| {
        let dr = rho - big_r;
        ((r - dr.abs()) * (r + dr.abs())).sqrt()
    };
    match (min_over_lo, min_under_hi, max_over_lo, max_under_hi) {
        // The whole ρ-range inside the tube's shadow: two loops, each
        // encircling the cylinder.
        (true, true, true, true) => essential_pair(false, true),
        // Wholly inside the hole, or wholly outside the torus.
        (_, _, false, _) | (_, false, _, _) => none(),
        // Across both bounds: two `(0,1)` loops on the torus.
        (false, true, true, false) => essential_pair(true, false),
        // Across the inner bound only: the null loop at θ = 0.
        (false, true, true, true) => lone(c + toward * rho_max + a * height(rho_max)),
        // Across the outer bound only: the null loop at θ = π.
        (true, true, true, false) => lone(c + toward * (e - rc) + a * height(rho_min)),
    }
}

/// **Cylinder × cylinder.**
#[allow(clippy::too_many_arguments)]
fn cylinder_cylinder<T: Decide>(
    o1: Point3<T>,
    d1: Vec3<T>,
    r1: T,
    o2: Point3<T>,
    d2: Vec3<T>,
    r2: T,
    lever: T,
    band: Band,
) -> Section<T> {
    let cross = d1.cross(d2);
    let sin = cross.norm();
    match sign(
        "section_cylinder_axes_tilt",
        Margin::levered(sin, lever),
        band,
    ) {
        // Parallel axes: common rulings, or nothing.
        Some(Sign::Zero) => {
            return Section::Components {
                parts: vec![Component {
                    unbounded: true,
                    essential_f: false,
                    essential_g: false,
                    witness: None,
                }],
                single: false,
            };
        }
        Some(Sign::Positive) => {}
        // A near-parallel pair can carry a long null saddle loop.
        Some(Sign::Negative) | None => return Section::Tangent("section_cylinder_axes_tilt"),
    }
    let m = cross / sin;
    let delta = (o2 - o1).dot(m);
    let [reach, nest] = match signs(
        [
            ("section_cylinder_pair_reach", r1 + r2 - delta.abs()),
            ("section_cylinder_pair_nest", delta.abs() - (r1 - r2).abs()),
        ],
        band,
    ) {
        Ok(x) => x,
        Err(tan) => return tan,
    };
    match (reach, nest) {
        (false, _) => none(),
        // Two loops, each encircling the thinner wall's axis.
        (true, false) => match signs([("section_cylinder_pair_thin", r1 - r2)], band) {
            Ok([true]) => essential_pair(false, true),
            Ok([false]) => essential_pair(true, false),
            Err(tan) => tan,
        },
        // The single null saddle loop, witnessed on the arc's middle
        // ruling of cylinder 2, `cos(θ − φ) = −sign(δ₀)`.
        (true, true) => {
            let q = o2 - m * (r2 * T::one().copysign(delta));
            let (av, bv) = (d2.cross(d1), (q - o1).cross(d1));
            let (qa, qb, qc) = (av.dot(av), av.dot(bv), bv.dot(bv) - r1 * r1);
            let t = (-qb + (qb * qb - qa * qc).sqrt()) / qa;
            lone(q + d2 * t)
        }
    }
}

/// **Cylinder × plane**: an ellipse encircling the wall, or rulings.
/// Neither lies inside a wall that describes, and the count is not
/// certified, so W4 is never read.
fn cylinder_plane<T: Real>() -> Section<T> {
    Section::Components {
        parts: vec![Component {
            unbounded: false,
            essential_f: true,
            essential_g: false,
            witness: None,
        }],
        single: false,
    }
}

/// **Sphere × plane**: one circle.
fn sphere_plane<T: Decide>(
    cs: Point3<T>,
    rho: T,
    p0: Point3<T>,
    n: Vec3<T>,
    band: Band,
) -> Section<T> {
    let d = (cs - p0).dot(n);
    match signs([("section_sphere_plane_reach", rho - d.abs())], band) {
        Ok([true]) => {
            let rad = ((rho - d.abs()) * (rho + d.abs())).sqrt();
            let (b1, _) = n.orthonormal_basis();
            lone(cs - n * d + b1 * rad)
        }
        Ok([false]) => none(),
        Err(tan) => tan,
    }
}

/// **Sphere × sphere**: one circle.
fn sphere_sphere<T: Decide>(c1: Point3<T>, r1: T, c2: Point3<T>, r2: T, band: Band) -> Section<T> {
    let dd = (c2 - c1).norm();
    match signs(
        [
            ("section_sphere_pair_reach", r1 + r2 - dd),
            ("section_sphere_pair_nest", dd - (r1 - r2).abs()),
        ],
        band,
    ) {
        Ok([true, true]) => {
            let k = (c2 - c1) / dd;
            let x = (dd * dd + r1 * r1 - r2 * r2) / (dd + dd);
            let rad = ((r1 - x) * (r1 + x)).sqrt();
            let (b1, _) = k.orthonormal_basis();
            lone(c1 + k * x + b1 * rad)
        }
        Ok(_) => none(),
        Err(tan) => tan,
    }
}

/// **Sphere × cylinder** (the sphere is `F`).
fn sphere_cylinder<T: Decide>(
    cs: Point3<T>,
    rho: T,
    o: Point3<T>,
    d: Vec3<T>,
    rc: T,
    band: Band,
) -> Section<T> {
    let w = cs - o;
    let foot = o + d * w.dot(d);
    let perp = cs - foot;
    let e = perp.norm();
    let q_min = (e - rc).abs();
    let [reach, girdle] = match signs(
        [
            ("section_sphere_cylinder_reach", rho - q_min),
            ("section_sphere_cylinder_girdle", e + rc - rho),
        ],
        band,
    ) {
        Ok(x) => x,
        Err(tan) => return tan,
    };
    match (reach, girdle) {
        (false, _) => none(),
        // The ball holds every ruling's nearest point: two loops, each
        // encircling the cylinder.
        (true, false) => essential_pair(false, true),
        // One loop, witnessed on the ruling nearest the centre.
        (true, true) => {
            let t = ((rho - q_min) * (rho + q_min)).sqrt();
            lone(foot + perp / e * rc + d * t)
        }
    }
}

/// **The per-pair rule** over a classified section.
///
/// `evented` says whether the reduction recorded an event on this pair;
/// `describes` asks `chart_boundary` of a face (cached by the caller);
/// `place` places a witness point in `[F, G]`, `None` for no verdict.
///
/// # Errors
///
/// The pair's [`Refusal`].
pub(crate) fn certify<T: Decide>(
    section: &Section<T>,
    evented: bool,
    mut describes: impl FnMut(Side) -> bool,
    mut place: impl FnMut(Point3<T>) -> [Option<FaceContainment>; 2],
) -> Result<Vec<Cleared>, Refusal> {
    let (parts, single) = match section {
        Section::Components { parts, single } => (parts, *single),
        Section::Tangent(name) => return Err(Refusal::Tangent(name)),
        Section::Intractable => return Err(Refusal::Reach),
    };
    let mut out = Vec::with_capacity(parts.len());
    for c in parts {
        if c.unbounded {
            out.push(Cleared::Unbounded);
            continue;
        }
        if c.essential_f && describes(Side::F) {
            out.push(Cleared::Essential(Side::F));
            continue;
        }
        if c.essential_g && describes(Side::G) {
            out.push(Cleared::Essential(Side::G));
            continue;
        }
        if single && evented {
            out.push(Cleared::LoneEvented);
            continue;
        }
        let Some(p) = c.witness else {
            return Err(Refusal::Undecided);
        };
        let [at_f, at_g] = place(p);
        if at_f == Some(FaceContainment::Out) {
            out.push(Cleared::Out(Side::F));
            continue;
        }
        if at_g == Some(FaceContainment::Out) {
            out.push(Cleared::Out(Side::G));
            continue;
        }
        if !evented && at_f == Some(FaceContainment::In) && at_g == Some(FaceContainment::In) {
            return Err(Refusal::Loop);
        }
        return Err(Refusal::Undecided);
    }
    Ok(out)
}

#[cfg(test)]
#[path = "section_cert_rows.rs"]
mod section_cert_rows;

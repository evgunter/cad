//! Certified-conservative boxes for candidate generation (M5 PR 8,
//! C10) and its siblings.
//!
//! **The contract, in one sentence: every box this module returns
//! contains the entity's whole locus, or is the poison box.** A box
//! that is not a superset silently loses whatever its door would
//! otherwise have examined; a poison box overlaps everything, which
//! is the honest answer when no cheap superset is known. No Q1
//! predicate runs on a box; classification is untouched (`reduce`
//! module docs).
//!
//! **The contract is not fully held today, and no consumer may recite
//! it as though it were.** Issue **#862** holds the cases: the slab
//! arm's axial projection reads single bracket endpoints, and the
//! conic arm takes `hi().abs()`, which is not an upper bound on `|x|`
//! when the lower endpoint is larger in magnitude — so under an
//! `Interval` scalar there are loci this module does not enclose. The
//! `f64` lane is unaffected. Cite the sentence above with #862
//! attached or not at all; `separation`'s door does, and it is the
//! one where a box-level non-overlap is a GRANT.
//!
//! # Which way LOOSENESS runs is the door's property, not the box's
//!
//! A box bigger than it needs to be is free only where the box
//! PRUNES. That is **one** of the four doors that read a box from
//! here; at the other three, box NON-overlap is the answer being
//! sought, so a bigger box is a REFUSAL:
//!
//! - `boolean::reduce`'s C10 tree PRUNES. Loose costs a candidate
//!   pair's worth of exact work and can never change a verdict.
//! - `separation` GRANTS on non-overlap — `Ok(())` IS the
//!   disjointness certificate — so a bigger box refuses a placement
//!   pair that is genuinely separated.
//! - `boolean::ops`'s sphere-extent fallback refuses typed unless the
//!   ball's certified extent CLEARS the face's box, so a bigger box
//!   turns a separated cyl×sphere pair into
//!   `FallbackExtentUnsupported`.
//! - `census`'s arm 2 clears an instance pair only on a definitely
//!   negative margin against a CONTAINING box, so a bigger box turns
//!   a genuinely-outside instance into `CensusUndecidable` — the
//!   interference class.
//!
//! So nothing here may say "loose is free" about a BOX. It is a claim
//! about a door, and the door has to be named. The four are not
//! recited: `every_door_that_reads_a_box_is_inventoried` below walks
//! `topo/src` and pins them per file — both rules, face and edge — so
//! a fifth door cannot land unargued. **It pins WHERE the doors are
//! and not which way each reads**, which is the column that carries
//! the argument above; that gap is `S234` and has an owner rather
//! than a disclosure.
//!
//! [`sweep_pad`] is sized so the padding can never lose an accepted
//! pair at the sweep's door (its derivation below).
//!
//! [`FaceBoxRule`] is the ONE statement of which surface kinds have a
//! cheap sound box and by what construction, and [`face_box`] is its
//! `f64`-bracket instantiation. `census`'s `reach_box` reads the same
//! rule and re-derives the ARITHMETIC in the scalar lane — **and
//! nothing compares the two derivations.** Why the duplication
//! survives is an OPEN question, not a settled one: the argument that
//! module used to carry (the census validates `Dual` bodies and
//! `Dual` has no bracket, so that lane cannot join the allowlist)
//! lapsed at the D1 ruling, its own comment says so in as many words,
//! and what is left is the allowlist gate itself. **#700 is where
//! that is decided** — read the census comment as the open question
//! it records, not as this module's justification.
//!
//! An allowlisted [`geom_core::Bounds`] seam (ratified 2026-07-29 —
//! see geom-core `real.rs`, Bounds scope rule; the C10 tree is the
//! subdivision driver): coordinates enter as `[lo(), hi()]` brackets,
//! and poison flows to the poison box, which each door reads in its
//! own fail-loud direction — never prunes at the sweep, refuses at
//! the other three.

use bvh::Aabb;
use geom::Surface;
use geom::surfaces::nurbs::NurbsSurface;
use geom_core::{Band, Bounds, Decide, Point3, Real, Vec3};

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey, VertexKey};

/// The sweep's box pad in meters — what candidate generation must add
/// so pruning can never lose an accepted pair. Derivation (each term
/// against the sweep's accept conditions in `reduce`):
///
/// - an event point classifies ON the face plane / boundary only
///   within `band.zero()` (`bool_vertex_face_side`, `contfp`'s
///   `bool_contact_*` sites) — one `zero` for the point-to-face gap;
/// - vertices sit on their carriers only up to attachment-time
///   certification (`Certificate::max_residual ≤ ε`, the same run
///   tolerance the linear band's `zero` is built from) — one more
///   `zero` per side for vertex-extent honesty;
/// - `band.escalate()` on top dominates every remaining f64 slop
///   (crossing-point interpolation and `eval` rounding are
///   session-box-scale ulps, orders below it) and keeps near-boundary
///   escalation zones inside candidate range.
///
/// The sum is deliberately generous, and what that buys differs by
/// door (module docs). At the sweep's it only admits more candidate
/// pairs and never changes an answer (the differential suite pins
/// that). At `separation`'s and at `boolean::ops`'s fallback it can
/// only make a certificate harder to obtain, never wrongly grant one
/// — the same direction, but a cost rather than a free one.
pub(crate) fn sweep_pad(band: Band) -> f64 {
    band.escalate() + 2.0 * band.zero()
}

fn corrupt(what: &'static str) -> BooleanError {
    BooleanError::ClassificationInvariant { what }
}

/// **The one soundness rule for a face's box**, stated per surface
/// kind: which cheap construction yields a genuine SUPERSET of the
/// face's locus. Every consumer that bounds a face reads its arm from
/// here, so no two of them can quietly disagree about which kinds are
/// boxable.
///
/// The variants carry the surface payload the construction needs, so
/// the kind is matched ONCE and each lane only performs its own
/// arithmetic. The soundness argument per arm:
///
/// - [`BoundaryHull`](Self::BoundaryHull) — **Plane.** A planar face
///   lies in the convex hull of its boundary, so the hull of the
///   boundary's own certified boxes contains it whatever the boundary
///   curves are. The hull of the boundary VERTICES alone does not: a
///   circular rim bulges past its endpoints, and this engine's
///   plane×cylinder lane mints exactly that face.
/// - [`CylinderSlab`](Self::CylinderSlab) — **Cylinder.** The wall's
///   belly bulges past its chords, so the box is the whole cylinder
///   slab over the face's axial range (the axial coordinate is linear
///   along the surface, so the face's axial extremes lie on its
///   boundary), widened by the full radius in every coordinate.
///
///   **The axial coordinate carries the pad TWICE**, and this is the
///   rule rather than an accident of the code: the arm pads the axial
///   RANGE before building the slab, and the final `padded` pads
///   every coordinate again. The second application is dominated by
///   the first — an axial offset of `pad` is at most `pad` in any
///   world coordinate, which `padded` already adds — so it is
///   redundant width. Redundant width is not free here (see the door
///   list above), and removing it is a TIGHTENING that changes which
///   pairs three of the four doors refuse, so it is **#862**'s
///   alongside the two over-widths and not a style pass's.
///
///   **Widening the AXIAL coordinate by the radius claims more than
///   the construction needs**, and this doc is the only place that
///   has ever said so: the boundary already bounds the axial extent
///   exactly, so a radius-`r` cylinder about `z` over `z ∈ [0, 1]` is
///   claimed over `z ∈ [−r, 1 + r]`. Over-width, not unsoundness —
///   but three of the four doors above read over-width as a refusal.
///   **Issue #862** holds it, together with the axial projection's
///   single-endpoint bracket reads. The
///   `the_cylinder_arms_box_is_exactly_…` row below allows exactly
///   this much and no more, and goes red the day #862 lands.
/// - [`WholeBall`](Self::WholeBall) — **Sphere.** A band's belly
///   bulges past its poles and seam arcs, so the box is the whole ball
///   `center ± r`; every surface point is within `r` of the center.
/// - [`ControlNet`](Self::ControlNet) — **NURBS.** The patch bulges
///   past the hull of its boundary exactly as the sphere does, but it
///   lies in the hull of its CONTROL NET (nonnegative basis, strictly
///   positive weights — `geom::surfaces::boxes::nurbs_surface_aabb`
///   carries the citation), over the whole KNOT DOMAIN and a fortiori
///   over any trim inside it.
///
///   **The premise that carries: the face's trim lies inside the knot
///   domain.** The convex-hull property is a statement about the
///   domain the basis is defined on; this kernel's evaluator
///   EXTRAPOLATES outside it, and an extrapolated point can leave the
///   control hull by any amount. Nothing in the type system enforces
///   trim ⊆ domain today — `pcurves.rs`'s chart window is built from
///   the boundary's own chart boxes and is never compared against
///   `knots_u().domain()`. What holds it up is construction: every
///   kernel-minted NURBS wall is iso-parameter bounded at the domain
///   edges. State the premise when you add a constructor that is not.
/// - [`NoSoundBox`](Self::NoSoundBox) — **Cone, Torus.** No cheap
///   superset is known, so nothing is claimed: the box is poison
///   (never prunes) where a box is built, and a refusal where a
///   certificate is wanted. A face whose surface key does not RESOLVE
///   is a different answer — that is arena corruption, and the callers
///   here report it as such rather than folding it in here.
///
/// [`WholeBall`](Self::WholeBall) and the conic-fed
/// [`BoundaryHull`](Self::BoundaryHull) claim more than the trimmed
/// face occupies on purpose — a cheap SUPERSET is what the contract
/// asks for, and no cheaper one is known per kind. That looseness is
/// not free (module docs: three of four doors read it as a refusal),
/// so it is bounded rather than open-ended:
/// the four `the_*_arms_box_is_exactly_the_construction_its_rule_states`
/// rows below pin every arm to exactly the construction stated here.
pub(crate) enum FaceBoxRule<'a, T: Real> {
    /// Hull the boundary's certified loci — see the type docs.
    BoundaryHull,
    /// The axial slab widened by the radius — see the type docs.
    CylinderSlab {
        /// The `v = 0` point on the axis.
        origin: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The cylinder's radius.
        radius: T,
    },
    /// The whole ball `center ± r` — see the type docs.
    WholeBall {
        /// The sphere's center.
        center: Point3<T>,
        /// The sphere's radius.
        radius: T,
    },
    /// The control net's hull — see the type docs.
    ControlNet(&'a NurbsSurface<T>),
    /// No cheap superset exists — see the type docs.
    NoSoundBox,
}

/// The [`FaceBoxRule`] for a surface — the single kind→rule mapping. A
/// kind added to [`Surface`] lands on [`FaceBoxRule::NoSoundBox`] only
/// by being written here, never by falling through a wildcard in some
/// consumer. Takes a RESOLVED surface: a missing one is corruption,
/// which is the caller's to report and not a rule.
pub(crate) fn face_box_rule<T: Real>(surface: &Surface<T>) -> FaceBoxRule<'_, T> {
    match surface {
        Surface::Plane { .. } => FaceBoxRule::BoundaryHull,
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => FaceBoxRule::CylinderSlab {
            origin: *origin,
            axis: *axis,
            radius: *radius,
        },
        Surface::Sphere { center, radius, .. } => FaceBoxRule::WholeBall {
            center: *center,
            radius: *radius,
        },
        Surface::Nurbs(patch) => FaceBoxRule::ControlNet(patch),
        Surface::Cone { .. } | Surface::Torus { .. } => FaceBoxRule::NoSoundBox,
    }
}

/// The face's certified box, padded — [`FaceBoxRule`]'s
/// `f64`-bracket instantiation, and therefore a genuine superset of
/// the face's locus for every kind that has one, the poison box for
/// every kind that does not.
///
/// Poison here is not a refusal: it overlaps everything, so a face
/// whose kind has no cheap superset is simply never pruned and reaches
/// the exact predicates — which is where a `Cone`/`Torus` operand
/// meets its typed refusal anyway.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when the face's topology
/// is corrupt (a lost entity, an unwalkable loop). A face whose
/// surface key does not resolve is corruption, NOT a kind without a
/// box — the two are separate answers here.
pub(crate) fn face_box<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let f = body.get_face(face).ok_or(corrupt("face box: face lost"))?;
    let surface = body
        .get_surface(f.surface)
        .ok_or(corrupt("face box: surface lost"))?;
    let boxed = match face_box_rule(surface) {
        FaceBoxRule::NoSoundBox => return Ok(Aabb::poison()),
        FaceBoxRule::ControlNet(patch) => geom::surfaces::boxes::nurbs_surface_aabb(patch),
        FaceBoxRule::WholeBall { center, radius } => {
            let r = radius.hi();
            Aabb {
                min_x: center.x.lo() - r,
                min_y: center.y.lo() - r,
                min_z: center.z.lo() - r,
                max_x: center.x.hi() + r,
                max_y: center.y.hi() + r,
                max_z: center.z.hi() + r,
            }
        }
        FaceBoxRule::CylinderSlab {
            origin,
            axis,
            radius,
        } => {
            let radius = radius.hi();
            let Some(bnd) = boundary_hull(body, f)? else {
                return Ok(Aabb::poison());
            };
            // The axial range over the boundary's own hull. Taking the
            // hull first (rather than each edge box separately) is a
            // superset of every per-edge range because the projection
            // is linear — looser, the conservative direction — and it
            // is what makes poison PROPAGATE: `Aabb::hull` carries NaN,
            // and a NaN projection returns the poison box below rather
            // than being dropped by an `f64::min` that ignores it.
            let mut h_min = f64::INFINITY;
            let mut h_max = f64::NEG_INFINITY;
            for &(x, y, z) in &[
                (bnd.min_x, bnd.min_y, bnd.min_z),
                (bnd.max_x, bnd.max_y, bnd.max_z),
                (bnd.min_x, bnd.min_y, bnd.max_z),
                (bnd.min_x, bnd.max_y, bnd.min_z),
                (bnd.max_x, bnd.min_y, bnd.min_z),
                (bnd.min_x, bnd.max_y, bnd.max_z),
                (bnd.max_x, bnd.min_y, bnd.max_z),
                (bnd.max_x, bnd.max_y, bnd.min_z),
            ] {
                let h = (x - origin.x.lo()) * axis.x.lo()
                    + (y - origin.y.lo()) * axis.y.lo()
                    + (z - origin.z.lo()) * axis.z.lo();
                if !h.is_finite() {
                    return Ok(Aabb::poison());
                }
                h_min = h_min.min(h);
                h_max = h_max.max(h);
            }
            if !(h_min.is_finite() && h_max.is_finite()) {
                return Ok(Aabb::poison());
            }
            // Pad the axial range too (interval-lane bracket slop is
            // dominated by the pad; conservative direction).
            h_min -= pad;
            h_max += pad;
            let along = |c: f64, a: f64| (c + a * h_min, c + a * h_max);
            let (x0, x1) = along(origin.x.lo(), axis.x.lo());
            let (y0, y1) = along(origin.y.lo(), axis.y.lo());
            let (z0, z1) = along(origin.z.lo(), axis.z.lo());
            Aabb {
                min_x: x0.min(x1) - radius,
                min_y: y0.min(y1) - radius,
                min_z: z0.min(z1) - radius,
                max_x: x0.max(x1) + radius,
                max_y: y0.max(y1) + radius,
                max_z: z0.max(z1) + radius,
            }
        }
        FaceBoxRule::BoundaryHull => boundary_hull(body, f)?.unwrap_or_else(Aabb::poison),
    };
    Ok(boxed.padded(pad))
}

/// A face's loop keys, outer first — the walk order every arm here
/// shares (D9: fixed, so two boxes of one face fold identically).
fn loops_of(f: &crate::entity::Face) -> impl Iterator<Item = LoopKey> + '_ {
    core::iter::once(f.outer).chain(f.rings.iter().copied())
}

/// The hull of the face boundary's own certified boxes — every
/// boundary edge's [`edge_box`], plus the isolated-vertex loops, which
/// have no edge to speak for them. `None` for a face with no boundary
/// at all.
///
/// Poison propagates: [`Aabb::hull`] carries NaN by construction, so
/// one unboxable boundary edge poisons the face rather than quietly
/// contributing nothing.
///
/// This is the [`FaceBoxRule::BoundaryHull`] arm, and it is also what
/// the [`FaceBoxRule::CylinderSlab`] arm reads its axial range from —
/// the axial coordinate is linear along the surface, so the face's
/// axial extremes lie on the boundary, but not necessarily at a
/// boundary VERTEX.
fn boundary_hull<T: Decide + Bounds>(
    body: &Body<T>,
    f: &crate::entity::Face,
) -> Result<Option<Aabb>, BooleanError> {
    let mut acc: Option<Aabb> = None;
    let mut grow = |x: Aabb| acc = Some(acc.map_or(x, |a: Aabb| a.hull(&x)));
    for lk in loops_of(f) {
        let l = body.get_loop(lk).ok_or(corrupt("face box: loop lost"))?;
        match l.boundary {
            LoopBoundary::Empty { vertex } => {
                let p = vertex_point(body, vertex)?;
                grow(Aabb::from_points([p]).unwrap_or_else(Aabb::poison));
            }
            LoopBoundary::Cycle { first } => {
                for he in body
                    .loop_cycle(first)
                    .ok_or(corrupt("face box: unwalkable loop"))?
                {
                    let ek = body
                        .get_half_edge(he)
                        .ok_or(corrupt("face box: half-edge lost"))?
                        .edge;
                    grow(edge_box(body, ek, 0.0)?);
                }
            }
        }
    }
    Ok(acc)
}

/// **The one soundness rule for an edge's box** — [`FaceBoxRule`]'s
/// curve-side twin, and read by the same consumers for the same
/// reason: which cheap construction yields a genuine SUPERSET of the
/// edge's locus over its span.
///
/// - [`Chord`](Self::Chord) — **Line.** The locus IS the chord between
///   the endpoints, up to the certification residual the pad covers.
/// - [`ConicAmplitude`](Self::ConicAmplitude) — **Circle, Ellipse.**
///   The FULL conic's centre-±-amplitude box (per coordinate
///   `|û_i|·a + |v̂_i|·b`, with `v̂ = axis × û`) hulled with the chord.
///   A superset of any arc of the conic, reflex spans included — an
///   arc's belly bulges past its chord, so the chord alone is not a
///   bound. The full turn is deliberately loose: a superset is what
///   the contract asks for and the arc's own extremes are not cheap.
///   What that looseness costs is the door's, not the box's — the
///   module docs list the four and their directions.
///
///   **`|û_i|·a + |v̂_i|·b` is not a function of the LOCUS**, and that
///   is a sharper defect than being loose. The conic's own
///   half-extent is `√((û_i·a)² + (v̂_i·b)²)`; this sum is the
///   triangle-inequality bound over it, and what it returns depends
///   on how the carrier is NAMED rather than on where the conic is.
///   Two `Curve3::Circle` values describing the SAME circle — same
///   centre, axis and radius, `u_ref` rotated within the plane — get
///   boxes as far apart as `r√2` against `r`. The module contract is
///   one-sided (*"contains the entity's whole locus"*), so nothing
///   here is unsound; but a box rule that reads its own
///   parameterization is not a rule about the entity, and three of
///   the four doors pay the difference in refusals rather than in
///   work.
///
///   **In-tree bodies already take the wide branch** — this is not
///   reachable-but-unexercised. The plane×cylinder rim inherits the
///   cylinder surface's own `u_ref`, and the plane×sphere circle
///   derives one from the seam or polar candidate; neither is
///   axis-aligned in general, and an extruded arc profile mints
///   rotated ones directly.
///
///   **The exact box already exists one crate down and is unused.**
///   `geom::curves::boxes::circle_arc_aabb` (and its ellipse twin)
///   computes `Aᵢ = √(uᵢ² + vᵢ²)` outward-bracketed AND restricts to
///   the certified span, so it is tighter on both counts; it takes
///   the two params [`geom_brep::EdgeCurve::params`] already has here,
///   and today its only callers are `geom`'s own tests. Taking it is
///   a TIGHTENING, so it carries the same obligation the NURBS arm
///   states: it would start pruning pairs that are examined today.
///   The correctness half is **#862**'s and the *two constructions,
///   the correct one unused* half is **`S235`**; the
///   `the_planar_arms_box_is_exactly_…` row below spells the current
///   formula out so tightening it is loud.
/// - [`NoSoundBox`](Self::NoSoundBox) — **NURBS carriers**, and an
///   edge whose carrier is null scaffolding. Nothing is certified
///   about the locus, so nothing is claimed; the chord is NOT a bound
///   for either.
///
///   The NURBS arm is the one place a sound cheap box exists and is
///   deliberately not taken: `geom::curves::boxes::nurbs_curve_aabb`
///   would give the control-net hull, exactly as
///   [`FaceBoxRule::ControlNet`] does one dimension up. Taking it
///   would TIGHTEN this box — it would start pruning pairs that are
///   examined today — and tightening is a different obligation from
///   soundness: a rung-3 operand gate has to admit the kind first.
///   Claiming nothing is already the conservative answer, so nothing
///   is unsound while it waits. (It also carries the same trim ⊆ knot
///   domain premise the surface arm states.)
pub(crate) enum EdgeBoxRule<T: Real> {
    /// The chord between the endpoints — see the type docs.
    Chord,
    /// The full conic's amplitude box, hulled with the chord — see the
    /// type docs.
    ConicAmplitude {
        /// The conic's centre.
        center: Point3<T>,
        /// The plane normal of the conic.
        axis: Vec3<T>,
        /// The semi-axis along `u_ref`.
        semi_u: T,
        /// The semi-axis along `axis × u_ref`.
        semi_v: T,
        /// The in-plane reference direction.
        u_ref: Vec3<T>,
    },
    /// No cheap superset exists — see the type docs.
    NoSoundBox,
}

/// The [`EdgeBoxRule`] for a carrier — the single kind→rule mapping,
/// with `None` standing for the null-scaffolding state (no carrier by
/// type). A kind added to [`geom::Curve3`] lands on
/// [`EdgeBoxRule::NoSoundBox`] only by being written here.
pub(crate) fn edge_box_rule<T: Real>(carrier: Option<&geom::Curve3<T>>) -> EdgeBoxRule<T> {
    match carrier {
        Some(geom::Curve3::Line { .. }) => EdgeBoxRule::Chord,
        Some(geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        }) => EdgeBoxRule::ConicAmplitude {
            center: *center,
            axis: *axis,
            semi_u: *radius,
            semi_v: *radius,
            u_ref: *u_ref,
        },
        Some(geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        }) => EdgeBoxRule::ConicAmplitude {
            center: *center,
            axis: *axis,
            semi_u: *major,
            semi_v: *minor,
            u_ref: *u_ref,
        },
        Some(geom::Curve3::Nurbs(_)) | None => EdgeBoxRule::NoSoundBox,
    }
}

/// The edge's certified box, padded — [`EdgeBoxRule`]'s `f64`-bracket
/// instantiation, and therefore a superset of the edge's locus or the
/// poison box.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when the edge's topology
/// is corrupt.
pub(crate) fn edge_box<T: Decide + Bounds>(
    body: &Body<T>,
    edge: EdgeKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let e = body.get_edge(edge).ok_or(corrupt("edge box: edge lost"))?;
    let start_of = |he| -> Result<Point3<T>, BooleanError> {
        let vk = body
            .get_half_edge(he)
            .ok_or(corrupt("edge box: half-edge lost"))?
            .start;
        vertex_point(body, vk)
    };
    let (a, b) = (start_of(e.he_plus)?, start_of(e.he_minus)?);
    let chord = Aabb::from_points([a, b]).unwrap_or_else(Aabb::poison);
    let carrier = body
        .get_curve_geom(e.curve)
        .and_then(crate::null::CurveGeom::certified)
        .map(geom_brep::EdgeCurve::carrier);
    let boxed = match edge_box_rule(carrier) {
        EdgeBoxRule::NoSoundBox => return Ok(Aabb::poison()),
        EdgeBoxRule::Chord => chord,
        EdgeBoxRule::ConicAmplitude {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => {
            let v_ref = axis.cross(u_ref);
            let reach = |ui: f64, vi: f64| ui.abs() * semi_u.hi() + vi.abs() * semi_v.hi();
            let rx = reach(u_ref.x.hi(), v_ref.x.hi());
            let ry = reach(u_ref.y.hi(), v_ref.y.hi());
            let rz = reach(u_ref.z.hi(), v_ref.z.hi());
            let full = Aabb {
                min_x: center.x.lo() - rx,
                min_y: center.y.lo() - ry,
                min_z: center.z.lo() - rz,
                max_x: center.x.hi() + rx,
                max_y: center.y.hi() + ry,
                max_z: center.z.hi() + rz,
            };
            // `Aabb::hull`, not a raw min/max fold: a poisoned centre
            // or semi-axis must survive the hull, and `f64::min`
            // RETURNS the non-NaN operand.
            full.hull(&chord)
        }
    };
    Ok(boxed.padded(pad))
}

/// Either empty lookup means the same thing here — a corrupt body —
/// so the read-back door's discriminated reference collapses to one
/// verdict.
fn vertex_point<T: Decide + Bounds>(
    body: &Body<T>,
    v: VertexKey,
) -> Result<Point3<T>, BooleanError> {
    crate::readback::vertex_point_ref(body, v)
        .map_err(|_| corrupt("face/edge box: vertex point lost"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! **Two contracts, and they need opposite assertions.**
    //!
    //! The **locus** rows sample the face's true locus and assert the
    //! box contains every sample. The spans are swept rather than
    //! chosen, so a rule that drops part of the bulge reds and not
    //! only one that drops all of it, and no single fixture can be
    //! the reason a row passes.
    //!
    //! **That family degrades in ONE direction only, and saying
    //! otherwise is the thing S66 exists to refute.** `holds(&b, p)`
    //! is satisfied by any bigger box, so a `face_box` returning
    //! `[-1e300, 1e300]` on every arm that has one passes every locus
    //! row here — and passes the whole `topo` lib suite. Over-width
    //! is not slower work at three of this module's four doors
    //! (module docs); it is a refusal, and therefore an answer. A
    //! suite of locus rows is **not** an adequate guard for this
    //! module, and a reader who leaves this header believing it is
    //! has been told the wrong thing.
    //!
    //! So the **ceiling** rows —
    //! `the_*_arms_box_is_exactly_the_construction_its_rule_states` —
    //! state each arm's box as a formula in the fixture's own
    //! parameters and pin it on all six faces in BOTH directions.
    //! Neither family subsumes the other: the locus rows check the
    //! rule against the geometry, the ceiling rows check the code
    //! against the rule.
    //!
    //! `every_door_that_reads_a_box_is_inventoried` is neither — it
    //! reads source, not geometry, and guards the module docs' door
    //! list rather than any box.

    use geom_core::Tol;
    use super::*;
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use geom::Curve3;
    use geom::Surface;
    use geom_brep::{EdgeCurveSpec, EdgeGeometry};
    use geom_core::{Point3, Vec3};

    /// The pad every row boxes with — the sweep's own, so a row that
    /// only passes because of a generous pad would have to say so.
    fn pad() -> f64 {
        sweep_pad(Band::linear(Tol::witness()).unwrap())
    }

    /// `p` is inside `b` — the containment the contract promises.
    fn holds(b: &Aabb, p: Point3<f64>) -> bool {
        p.x >= b.min_x
            && p.x <= b.max_x
            && p.y >= b.min_y
            && p.y <= b.max_y
            && p.z >= b.min_z
            && p.z <= b.max_z
    }

    fn plane_z0() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
    }

    fn cyl_r(r: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }
    }

    /// A PLANAR face whose rim is a circular arc: the sector of radius
    /// `r` spanning `[0, span]` in azimuth, closed by two radii. This
    /// is the shape the plane×cylinder lane mints as a cylinder's cap,
    /// and the shape whose locus leaves its boundary-vertex hull.
    ///
    /// Returns the body and the sector face.
    fn arc_sector(r: f64, span: f64) -> (Body<f64>, FaceKey) {
        arc_sector_from(r, span, 0.0)
    }

    /// [`arc_sector`] with the rim carrier's `u_ref` rotated by `phi`
    /// in the plane: the SAME circle and the same arc, named from a
    /// different reference direction, with the parameter range shifted
    /// so the endpoints are unchanged. That is not a contrivance — the
    /// split lane mints rims this way, and `cyl_wall`'s descending rim
    /// below is one — and [`EdgeBoxRule::ConicAmplitude`]'s
    /// per-coordinate `|û_i|·a + |v̂_i|·b` is not invariant under it.
    fn arc_sector_from(r: f64, span: f64, phi: f64) -> (Body<f64>, FaceKey) {
        let on = |t: f64| Point3::new(r * t.cos(), r * t.sin(), 0.0);
        let (a, b, c) = (on(0.0), on(span), Point3::origin());
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(a).unwrap();
        let plane = body.add_surface(plane_z0());
        let cyl = body.add_surface(cyl_r(r));
        let arc = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: plane,
                s2: cyl,
                witness: on(span * 0.5),
            },
            carrier: Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: r,
                u_ref: Vec3::new(phi.cos(), phi.sin(), 0.0),
            },
            param_start: -phi,
            param_end: span - phi,
        };
        let e_ab = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                b,
                arc,
                Tol::witness(),
            )
            .unwrap();
        let e_bc = body
            .mev_line(
                MevSite::Fan {
                    he1: e_ab.he_minus,
                    he2: e_ab.he_minus,
                },
                c,
                Tol::witness(),
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_bc.vertex, e_ab.vertex)
            .unwrap();
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_ab.he_plus,
                },
                EdgeCurveSpec::line_between(c, a),
                FaceSurface::Shared(plane),
                Tol::witness(),
            )
            .unwrap()
            .face;
        (body, face)
    }

    /// **The reported defect.** A planar face's rim bulges past its
    /// boundary VERTICES, so a vertex-hull box is not a superset and
    /// `Bvh::overlapping` can prune a pair the exact predicates would
    /// have accepted.
    ///
    /// The span is swept from a shallow arc to a reflex one, and the
    /// radius with it: the miss grows with the sagitta, so a rule that
    /// covers only part of the bulge fails at the larger spans while
    /// passing the small ones. A single fixture cannot be the reason
    /// this row is green.
    #[test]
    fn a_planar_faces_circular_rim_is_inside_its_box() {
        for &r in &[0.001, 1.0, 250.0] {
            for span_deg in [10.0_f64, 90.0, 179.0, 181.0, 300.0, 359.0] {
                for phi_deg in [0.0_f64, 45.0, 137.0] {
                    let span = span_deg.to_radians();
                    let (body, face) = arc_sector_from(r, span, phi_deg.to_radians());
                    let b = face_box(&body, face, pad()).unwrap();
                    // The locus is the convex hull of its boundary and
                    // the box is convex, so sampling the boundary
                    // settles it.
                    for i in 0..=512 {
                        let t = span * f64::from(i) / 512.0;
                        let p = Point3::new(r * t.cos(), r * t.sin(), 0.0);
                        assert!(
                            holds(&b, p),
                            "rim point at {t} rad left the box (r = {r}, \
                             span = {span_deg}°, u_ref at {phi_deg}°): {b:?}"
                        );
                    }
                }
            }
        }
    }

    /// The same claim stated as a margin: the box must reach the arc's
    /// extreme in the direction the vertex hull cannot see. A
    /// half-turn sector's rim tops out at `y = r` while both its
    /// vertices sit at `y ≤ 0`, so a vertex-hull box misses by `r` —
    /// this row measures that gap rather than trusting a sample to
    /// land on it.
    #[test]
    fn the_boxs_reach_beyond_the_vertex_hull_is_the_whole_bulge() {
        let r = 2.0;
        let (body, face) = arc_sector(r, core::f64::consts::PI);
        let b = face_box(&body, face, pad()).unwrap();
        // Both boundary vertices and the sector's centre are at y ≤ 0;
        // the rim reaches y = r.
        assert!(
            b.max_y >= r,
            "the box must reach the rim's extreme y = {r}, got {}",
            b.max_y
        );
    }

    /// A cylinder WALL face: the patch `u ∈ [u0, u1] × z ∈ [z0, z1]` on
    /// the radius-`r` cylinder about the z axis, bounded below and
    /// above by circular rims and on the sides by axial lines.
    fn cyl_wall(r: f64, u0: f64, u1: f64, z0: f64, z1: f64) -> (Body<f64>, FaceKey) {
        let on = |u: f64, z: f64| Point3::new(r * u.cos(), r * u.sin(), z);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(on(u0, z0)).unwrap();
        let cyl = body.add_surface(cyl_r(r));
        // A rim at height `z`: the cylinder cut by the plane there.
        // The descending rim runs on the reversed axis so its own
        // parameters increase, exactly as the split lane mints them.
        let rim = |body: &mut Body<f64>, z: f64, ccw: bool| {
            let plane = body.add_surface(Surface::Plane {
                origin: Point3::new(0.0, 0.0, z),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            });
            let (carrier, t0, t1) = if ccw {
                (
                    Curve3::Circle {
                        center: Point3::new(0.0, 0.0, z),
                        axis: Vec3::unit_z(),
                        radius: r,
                        u_ref: Vec3::unit_x(),
                    },
                    u0,
                    u1,
                )
            } else {
                (
                    Curve3::Circle {
                        center: Point3::new(0.0, 0.0, z),
                        axis: Vec3::new(0.0, 0.0, -1.0),
                        radius: r,
                        u_ref: Vec3::new(u1.cos(), u1.sin(), 0.0),
                    },
                    0.0,
                    u1 - u0,
                )
            };
            EdgeCurveSpec {
                description: EdgeGeometry::Intersection {
                    s1: cyl,
                    s2: plane,
                    witness: on((u0 + u1) * 0.5, z),
                },
                carrier,
                param_start: t0,
                param_end: t1,
            }
        };
        let bottom = rim(&mut body, z0, true);
        let e_b = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                on(u1, z0),
                bottom,
                Tol::witness(),
            )
            .unwrap();
        let e_r = body
            .mev_line(
                MevSite::Fan {
                    he1: e_b.he_minus,
                    he2: e_b.he_minus,
                },
                on(u1, z1),
                Tol::witness(),
            )
            .unwrap();
        let top = rim(&mut body, z1, false);
        let e_t = body
            .mev(
                MevSite::Fan {
                    he1: e_r.he_minus,
                    he2: e_r.he_minus,
                },
                on(u0, z1),
                top,
                Tol::witness(),
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
            .unwrap();
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_b.he_plus,
                },
                EdgeCurveSpec::line_between(on(u0, z1), on(u0, z0)),
                FaceSurface::Shared(cyl),
                Tol::witness(),
            )
            .unwrap()
            .face;
        (body, face)
    }

    /// The cylinder arm, against the wall it bounds. The belly bulges
    /// past every chord of the boundary, and the axial range must cover
    /// the whole patch — both swept over radii and over azimuth spans
    /// including a reflex one, so a rule that recovers the extent only
    /// for short spans goes red.
    #[test]
    fn a_cylinder_walls_locus_is_inside_its_box() {
        for &r in &[0.002, 1.0, 40.0] {
            for span_deg in [30.0_f64, 170.0, 200.0, 350.0] {
                let span = span_deg.to_radians();
                let (z0, z1) = (-0.25 * r, 0.75 * r);
                let (body, face) = cyl_wall(r, 0.0, span, z0, z1);
                let b = face_box(&body, face, pad()).unwrap();
                for i in 0..=64 {
                    let u = span * f64::from(i) / 64.0;
                    for j in 0..=8 {
                        let z = z0 + (z1 - z0) * f64::from(j) / 8.0;
                        let p = Point3::new(r * u.cos(), r * u.sin(), z);
                        assert!(
                            holds(&b, p),
                            "wall point (u = {u}, z = {z}) left the box \
                             (r = {r}, span = {span_deg}°): {b:?}"
                        );
                    }
                }
            }
        }
    }

    /// The sphere arm, against the sphere. This arm claims the WHOLE
    /// ball and reads nothing from the boundary, so the honest locus to
    /// sample is the whole sphere — every point of it, at any trim, is
    /// what the box promises to contain.
    #[test]
    fn a_spheres_whole_locus_is_inside_its_box() {
        for &r in &[0.002, 1.0, 40.0] {
            let center = Point3::new(0.3 * r, -0.2 * r, 0.1 * r);
            let (mut body, face) = arc_sector(r, core::f64::consts::PI);
            body.set_face_surface(
                face,
                FaceSurface::New(Surface::Sphere {
                    center,
                    radius: r,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                }),
            )
            .unwrap();
            let b = face_box(&body, face, pad()).unwrap();
            for i in 0..=32 {
                let theta = core::f64::consts::PI * f64::from(i) / 32.0;
                for j in 0..=32 {
                    let phi = 2.0 * core::f64::consts::PI * f64::from(j) / 32.0;
                    let p = Point3::new(
                        center.x + r * theta.sin() * phi.cos(),
                        center.y + r * theta.sin() * phi.sin(),
                        center.z + r * theta.cos(),
                    );
                    assert!(holds(&b, p), "sphere point left the box (r = {r}): {b:?}");
                }
            }
        }
    }

    /// A biquadratic patch whose boundary lies entirely in `z = 0`
    /// while its centre control point lifts the surface to `z = 1/4`,
    /// carried on an arc sector's topology. Returns the body, the
    /// face, and the control net's own hull — which is the unit cube,
    /// and is what [`FaceBoxRule::ControlNet`] claims.
    fn nurbs_bulge_face() -> (Body<f64>, FaceKey, (Point3<f64>, Point3<f64>)) {
        use geom::surfaces::nurbs::NurbsSurface;
        use geom_core::spline::KnotVector;
        let kv = KnotVector::unit_segment(2);
        let p = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
        let control = vec![
            p(0.0, 0.0, 0.0),
            p(0.0, 0.5, 0.0),
            p(0.0, 1.0, 0.0),
            p(0.5, 0.0, 0.0),
            p(0.5, 0.5, 1.0),
            p(0.5, 1.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.0, 0.5, 0.0),
            p(1.0, 1.0, 0.0),
        ];
        let patch = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap();
        let surface = Surface::Nurbs(std::sync::Arc::new(patch));
        let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
        body.set_face_surface(face, FaceSurface::New(surface))
            .unwrap();
        (body, face, (p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)))
    }

    /// **The NURBS half of the same defect.** A patch's interior
    /// bulges past the hull of its boundary — here the biquadratic
    /// whose boundary lies entirely in `z = 0` while its centre
    /// control point lifts the surface to `z = 1/4`. The control-net
    /// hull contains it; the boundary hull does not.
    #[test]
    fn a_nurbs_patchs_interior_bulge_is_inside_its_box() {
        let (body, face, _) = nurbs_bulge_face();
        let surface = body
            .get_surface(body.get_face(face).unwrap().surface)
            .unwrap()
            .clone();
        // The lifted interior, on the surface itself — its boundary
        // curves all lie in `z = 0`, so no hull of the BOUNDARY can
        // contain this point.
        let mid = surface.eval(0.5, 0.5);
        assert!(mid.z > 0.2, "the fixture must actually bulge, got {mid:?}");
        assert!(
            surface.eval(0.0, 0.5).z.abs() < 1e-15,
            "the fixture's boundary must lie in z = 0"
        );
        let b = face_box(&body, face, pad()).unwrap();
        assert!(
            holds(&b, mid),
            "the patch's own interior point left its box: {b:?}"
        );
    }

    /// **The ceiling side of the contract, shared by the four rows
    /// below: an arm's box is EXACTLY the construction its rule
    /// states.**
    ///
    /// Each row states its arm's box as a FORMULA in the fixture's
    /// own parameters and pins [`face_box`] to it on all six faces in
    /// **both** directions — widening beyond the stated construction
    /// reds it, and so does narrowing, which the locus rows see only
    /// once it crosses the locus. One row per arm, so a red names its
    /// arm before the message does.
    ///
    /// **Three terms below are deviations rather than rule, and each
    /// is named as one**: `axial_overwidth`, `redundant_axial_pad`
    /// and `overwide_half_extent`, all **#862**'s. They are spelled
    /// as their own terms rather than folded into the construction so
    /// a reader can tell which quantities are the rule and which are
    /// the bug — a term whose name did not say it was wrong would
    /// ratify the very thing these rows exist to flag. The day any of
    /// them lands, its row reds at a named line and whoever fixes it
    /// deletes the term and its sentence together. Pinned here rather
    /// than fixed here: a correctness defect does not get fixed in a
    /// style pass.
    ///
    /// The six faces of `got` agree with `want` to within the
    /// arithmetic's own rounding. [`Aabb::padded`] alone moves each
    /// face by an ulp, and the fixture's trigonometry by a few more;
    /// the tolerance is relative to the fixture's scale and is orders
    /// below every term it has to separate — the pad, and the radius
    /// each arm might wrongly add.
    fn agrees_with_the_rule(got: &Aabb, want: &Aabb, scale: f64, what: &str) {
        let tol = 1e-12 * (1.0 + scale);
        for (name, g, w) in [
            ("min_x", got.min_x, want.min_x),
            ("min_y", got.min_y, want.min_y),
            ("min_z", got.min_z, want.min_z),
            ("max_x", got.max_x, want.max_x),
            ("max_y", got.max_y, want.max_y),
            ("max_z", got.max_z, want.max_z),
        ] {
            assert!(
                (g - w).abs() <= tol,
                "{what}: {name} is {g}, the construction its rule states gives {w} \
                 (off by {}, tolerance {tol})",
                g - w
            );
        }
    }

    /// **`BoundaryHull`, conic-fed** — the rim is a circle, so its
    /// [`EdgeBoxRule::ConicAmplitude`] box is the FULL turn's
    /// whatever the span (the arc's own extremes are deliberately not
    /// recovered) and the two radii chords lie inside it. Flat in z.
    ///
    /// The circle's TRUE half extent is `r` in every coordinate.
    /// `overwide_half_extent` is what the arm claims instead —
    /// `|û_i|·a + |v̂_i|·b`, which for a circle named from a `u_ref`
    /// at φ is `r·(|cos φ| + |sin φ|)`: equal to `r` only at an
    /// axis-aligned φ, and `r√2` at 45°. A function of the NAME, not
    /// of the locus, and #862's. **The φ sweep is what makes this row
    /// see it at all** — at φ = 0 the two agree and the arm looks
    /// tight.
    #[test]
    fn the_planar_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.001, 1.0, 250.0] {
            for span_deg in [10.0_f64, 90.0, 179.0, 181.0, 300.0, 359.0] {
                for phi_deg in [0.0_f64, 45.0, 137.0] {
                    let phi = phi_deg.to_radians();
                    let overwide_half_extent = r * (phi.cos().abs() + phi.sin().abs());
                    assert!(
                        overwide_half_extent >= r,
                        "the over-width term must never claim less than the true half extent"
                    );
                    let (body, face) = arc_sector_from(r, span_deg.to_radians(), phi);
                    let b = face_box(&body, face, pad).unwrap();
                    agrees_with_the_rule(
                        &b,
                        &Aabb {
                            min_x: -overwide_half_extent - pad,
                            min_y: -overwide_half_extent - pad,
                            min_z: -pad,
                            max_x: overwide_half_extent + pad,
                            max_y: overwide_half_extent + pad,
                            max_z: pad,
                        },
                        r,
                        &format!(
                            "the planar arm (r = {r}, span = {span_deg}°, u_ref at {phi_deg}°)"
                        ),
                    );
                }
            }
        }
    }

    /// **`CylinderSlab`** — the axial range is the boundary's own and
    /// the radial half-width is the radius.
    ///
    /// Two of the three axial terms are the rule's stated deviations
    /// rather than its construction, and both are #862's.
    /// `axial_overwidth` is the full radius added along the axis,
    /// where the boundary already bounds the extent exactly.
    /// `redundant_axial_pad` is the arm's own pad on the axial RANGE,
    /// applied before the slab is built and then again by the final
    /// [`Aabb::padded`] — so the axial coordinate carries the pad
    /// twice and the radial ones once. Both are stated at
    /// [`FaceBoxRule::CylinderSlab`], so this row transcribes the
    /// RULE and not an implementation detail: removing the inner pad
    /// reds this row at the term that names it.
    #[test]
    fn the_cylinder_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.002, 1.0, 40.0] {
            for span_deg in [30.0_f64, 170.0, 200.0, 350.0] {
                let (z0, z1) = (-0.25 * r, 0.75 * r);
                let (body, face) = cyl_wall(r, 0.0, span_deg.to_radians(), z0, z1);
                let b = face_box(&body, face, pad).unwrap();
                let axial_overwidth = r;
                let redundant_axial_pad = pad;
                agrees_with_the_rule(
                    &b,
                    &Aabb {
                        min_x: -r - pad,
                        min_y: -r - pad,
                        min_z: z0 - pad - redundant_axial_pad - axial_overwidth,
                        max_x: r + pad,
                        max_y: r + pad,
                        max_z: z1 + pad + redundant_axial_pad + axial_overwidth,
                    },
                    r,
                    &format!("the cylinder arm (r = {r}, span = {span_deg}°)"),
                );
            }
        }
    }

    /// **`WholeBall`** — `center ± r`, reading nothing from the
    /// boundary, so the trim the fixture carries must not appear in
    /// the box at all. No deviation terms: this arm claims exactly
    /// what its rule states.
    #[test]
    fn the_sphere_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.002, 1.0, 40.0] {
            let c = Point3::new(0.3 * r, -0.2 * r, 0.1 * r);
            let (mut body, face) = arc_sector(r, core::f64::consts::PI);
            body.set_face_surface(
                face,
                FaceSurface::New(Surface::Sphere {
                    center: c,
                    radius: r,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                }),
            )
            .unwrap();
            let b = face_box(&body, face, pad).unwrap();
            agrees_with_the_rule(
                &b,
                &Aabb {
                    min_x: c.x - r - pad,
                    min_y: c.y - r - pad,
                    min_z: c.z - r - pad,
                    max_x: c.x + r + pad,
                    max_y: c.y + r + pad,
                    max_z: c.z + r + pad,
                },
                r,
                &format!("the sphere arm (r = {r})"),
            );
        }
    }

    /// **`ControlNet`** — the hull of the net and nothing else. The
    /// fixture's net spans the unit cube while its own boundary lies
    /// in `z = 0`, so a box read off the BOUNDARY would be visibly
    /// different from one read off the net. No deviation terms.
    #[test]
    fn the_nurbs_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        let (body, face, net) = nurbs_bulge_face();
        let b = face_box(&body, face, pad).unwrap();
        agrees_with_the_rule(
            &b,
            &Aabb {
                min_x: net.0.x - pad,
                min_y: net.0.y - pad,
                min_z: net.0.z - pad,
                max_x: net.1.x + pad,
                max_y: net.1.y + pad,
                max_z: net.1.z + pad,
            },
            1.0,
            "the NURBS arm",
        );
    }

    /// **The door inventory** — the module docs' door list, computed
    /// rather than recited.
    ///
    /// The header states which direction each consumer reads a loose
    /// box in. That is a roster, and a roster is right only until the
    /// next door lands. This row walks `topo/src` and counts every
    /// CALL of [`face_box`], [`face_box_rule`], [`edge_box`] and
    /// [`edge_box_rule`] in code — comments and literals blanked by
    /// [`crate::source_walk::CodeOnly`], the more competent of this
    /// crate's two readers — pinned per file.
    ///
    /// **Both rules, not just the face one.** The header's claim is
    /// about *a box*, and [`EdgeBoxRule`]'s conic arm is where the
    /// sharper of the two over-widths lives (#862). A walk that
    /// matched only `face_box` would attribute that arm's cost to a
    /// door list computed for a different function, and an
    /// edge-box-only door would land green.
    ///
    /// - `boolean/reduce.rs` — the C10 candidate tree, face and edge.
    ///   **Prunes**: loose is slower work, never a different answer.
    ///   The only door for which that is true.
    /// - `boolean/ops.rs` — the sphere-extent fallback, face and
    ///   edge: the cylinder-face arm clears a [`face_box`] against
    ///   the ball's extent, and the scan's near-boundary test walks
    ///   the face's [`edge_box`]es against the germ circle's box.
    ///   **Refuses**: whichever box fails to clear turns the pair
    ///   into `FallbackExtentUnsupported`.
    /// - `separation.rs` — the placement certificate. **Refuses**:
    ///   non-overlap IS the grant.
    /// - `census.rs` — `reach_box` and `edge_reach`, which read the
    ///   RULES here and re-derive the arithmetic (#700). **Refuses**:
    ///   arm 2 clears only on a definitely negative margin against a
    ///   CONTAINING box, so over-width is a false `CensusUndecidable`.
    ///
    /// `boolean/boxes.rs` is excluded by path: it is the definition
    /// site, every call in it is this suite's own or one arm calling
    /// another, and its count would churn on each row added here
    /// while pinning nothing.
    ///
    /// **This row is a member of S117's population** — a
    /// source-text guard over `.rs` in this workspace. It reuses the
    /// shared reader rather than minting a sixth, which answers the
    /// half of S117 about how they lex; it does not answer the half
    /// about how many there are, and it makes that number larger.
    ///
    /// **What this cannot match**, stated rather than implied:
    ///
    /// 1. A door in another CRATE. The walk is `topo/src` because
    ///    that is the tree this crate can see. All four functions are
    ///    `pub(crate)`, so no such door can exist today — that, and
    ///    not a survey, is what carries the claim out of crate.
    /// 2. A door reading a box through a helper defined here: a
    ///    wrapper's own callers are invisible to a textual walk.
    /// 3. A call spelled through an alias or a re-export, or one a
    ///    macro assembles.
    ///
    /// **What it cannot match that is a FINDING rather than a
    /// disclosure: the direction itself.** This pins where the doors
    /// are, not what each does with looseness — and the direction
    /// column is the whole content of the header's argument, so a
    /// roster without it is a grep result. A door that changes its
    /// reading without moving leaves the dispositions above stale and
    /// this row green. That is this module's own defect one level up,
    /// recorded as **S232**'s sibling `S234` rather than left in the
    /// list above, because a disclosure with no owner is how the
    /// fifth instance gets found by accident.
    #[test]
    fn every_door_that_reads_a_box_is_inventoried() {
        const PINNED: [(&str, usize); 4] = [
            ("boolean/ops.rs", 2),
            ("boolean/reduce.rs", 3),
            ("census.rs", 2),
            ("separation.rs", 1),
        ];
        const HOME: &str = "boolean/boxes.rs";
        const DOORS: [&str; 4] = ["face_box(", "face_box_rule(", "edge_box(", "edge_box_rule("];
        let root = crate::source_walk::src_root();
        let mut found: Vec<(String, usize)> = Vec::new();
        for path in crate::source_walk::crate_sources() {
            let rel = path
                .strip_prefix(&root)
                .expect("a walked file lies under topo/src")
                .to_string_lossy()
                .replace('\\', "/");
            if rel == HOME {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            let code = crate::source_walk::CodeOnly::of(&text);
            let calls: usize = DOORS.iter().map(|d| code.as_str().matches(d).count()).sum();
            if calls > 0 || PINNED.iter().any(|(pinned, _)| *pinned == rel) {
                found.push((rel, calls));
            }
        }
        found.sort();
        let pinned: Vec<(String, usize)> = PINNED
            .iter()
            .map(|(path, calls)| ((*path).to_string(), *calls))
            .collect();
        assert_eq!(
            found, pinned,
            "a door that reads a box from this module arrived, left or moved. What the \
             module docs owe is not this count but the DIRECTION the new door reads \
             looseness in — pruning, or refusing — and nothing computes that (S234). \
             Update both, and read S234 before trusting the list you are updating."
        );
    }

    /// **A DISPATCH row, not a locus row.** It asserts only that a
    /// kind with no cheap sound box claims NOTHING — the poison box,
    /// which overlaps everything and therefore prunes nothing. The
    /// face's boundary is an arc sector's and has nothing to do with a
    /// cone or a torus, deliberately: the point is that the arm never
    /// looks at the boundary, because any hull of it would be a claim
    /// the kernel cannot make for these kinds.
    #[test]
    fn kinds_without_a_sound_box_dispatch_to_poison_and_never_prune() {
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let torus = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        for s in [cone, torus] {
            let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
            body.set_face_surface(face, FaceSurface::New(s)).unwrap();
            let b = face_box(&body, face, pad()).unwrap();
            assert!(b.min_x.is_nan(), "an unboxable kind must poison: {b:?}");
            assert!(
                b.overlaps(&Aabb {
                    min_x: 1e6,
                    min_y: 1e6,
                    min_z: 1e6,
                    max_x: 2e6,
                    max_y: 2e6,
                    max_z: 2e6,
                }),
                "poison must never prune"
            );
        }
    }
}

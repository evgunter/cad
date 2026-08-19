//! **The assembly half of the fillet unit** (M5 PR 12): turning a
//! [`BatteryVerdict`](super::BatteryVerdict) into a rounded solid.
//!
//! # The ordering contract, structurally
//!
//! [`fillet_edges`] calls [`run_battery`] as its FIRST statement and
//! propagates the refusal unchanged. Nothing here mints a surface, a
//! point, or a topology entity before a verdict exists — the C8 claim
//! ("if the battery returns `Ok`, construction cannot fail for a
//! geometric reason") is kept by construction order, not by hope.
//!
//! # The two assembly front doors
//!
//! The kernel has no face-soup constructor: bodies come only from
//! Euler operators. This module ships the **whole-body** rebuild —
//! every face of the result minted fresh:
//!
//! > fillet EVERY edge of a convex, planar-faced, trivalent-vertex
//! > polyhedron.
//!
//! Every other admissible request routes to the in-place edge-blend
//! **composition surgery** ([`super::surgery`], M6 unit 1): split the
//! supports along the stored trimlines, excise the strips, graft the
//! blends — which is what carries a face's RINGS through and what
//! replaces a pip rim with a torus band. The whole-body path is kept
//! (not subsumed) so its M5 outputs stay bit-preserved. What neither
//! door covers refuses typed through
//! [`FilletError::AssemblyUnsupported`], naming the remaining gap
//! (the `FullRevolveHoles` precedent).
//!
//! # Naming (M6-5 PR-2)
//!
//! Both doors now emit per-entity birth records
//! ([`super::naming::FilletNaming`]). Here they are read straight off
//! the [`Plan`] — the same provenance payloads the assembly built
//! from, paired with the key each plan slot minted. This door mints
//! into a FRESH arena, so it has no survivors: a shrunk support face
//! is a new key with a `supports` row, where the surgery's shrunk
//! support is the source key itself. Both name it the same thing.
//!
//! # What the record-writing does and does not guarantee
//!
//! Reading the payloads off the plan buys **locality**: the source
//! entity is stated where the mint is decided, so there is no second
//! pass that could go looking for it, and nothing is recovered by
//! matching geometry — which is what N4 forbids and the whole reason
//! this shape was chosen.
//!
//! It does NOT make the binding checkable. The provenance payload in
//! [`FaceKind`] is consumed by naming ALONE — the coverage buckets
//! read it blind (`matches!(k, FaceKind::Blend(_))`), and nothing
//! downstream re-derives which source edge a blend rounds. So a plan
//! that named the WRONG source — two blends handed each other's edge
//! — would emit two wrong names silently: the table stays total,
//! injective, and covariant, and every test in this crate passes.
//! (Executed by the PR-2 review, exactly so.) The correctness of the
//! payload rests on it being written at the one site that knows it,
//! three lines from the geometry it describes; that is a discipline,
//! not a proof, and it is identical to the surgery door's. Recording
//! elsewhere would be strictly worse, but it is worth saying plainly
//! that "birth-derived" bounds how a name can go wrong, not whether.
//!
//! # The derived result, stated once
//!
//! For a polyhedron with `V` vertices, `E` edges and `F` faces (so
//! `E = 3V/2` at trivalence) the rounded solid has
//!
//! - `3V` vertices — one per (original vertex, incident face) pair,
//!   `p(V, F) = c_V + r·n_F`, the foot of the corner ball's centre on
//!   that support plane;
//! - `2E + 3V` edges — two straight TRIMLINES per original edge (one
//!   per support) and three corner ARCS per original vertex;
//! - `F + E + V` faces — a shrunk planar face per original face, a
//!   quarter-cylinder blend per original edge, and a sphere patch per
//!   original vertex.
//!
//! For the die (`V = 8`, `E = 12`, `F = 6`) that is 24 / 48 / 26, and
//! `24 − 48 + 26 = 2`.
//!
//! Every trimline and every corner arc is born
//! [`EdgeGeometry::TangentIntersection`] over its two adjacent faces'
//! surfaces (prefer-intrinsic, D2/OQ7) — the blend cylinder IS tangent
//! to its support plane along the trimline, and the corner ball IS
//! tangent to each incident cylinder along the arc, because the ball
//! centre lies on that cylinder's axis and both have radius `r`.
//! Descriptions are attached in a final pass, once every face's
//! surface exists (the [`Body::set_edge_curve`] seam extrude already
//! uses for its rim upgrades).
//!
//! # Sense, from stored structure
//!
//! Blend and corner faces read their bit from the chain's stored
//! [`Convexity`] ([`Convexity::blend_sense`]) — never from a sampled
//! normal (S10/S11). Shrunk planar faces inherit the ORIGINAL face's
//! bit: same plane, same chart, same material side.
//!
//! # KNOWN LIMITATION — the octant's chart, and oblique trihedra
//!
//! The result is a tier-1/2 valid closed shell for every input the
//! front door admits. Tier **3** additionally needs the sphere
//! octant's three contact circles to be an iso-parameter rectangle in
//! its chart, because check 7's `+V` invariant runs the closed-form
//! mass properties, whose curved-face inventory is rims and meridians
//! (`geom_brep::props::curved_face`). The chart chosen below —
//! aimed along one incident edge — achieves that exactly when the
//! THIRD support's normal is parallel to that edge, i.e. at a RIGHT
//! trihedron (every vertex of a box). At an oblique trihedron no
//! chart makes a spherical triangle an iso-rectangle, so
//! `topo::mass_properties` refuses `NotIsoRectangle` and tier 3
//! reports `VolumeUncomputable` — a gap in the props inventory (a
//! spherical-triangle closed form, or the certified-quadrature lane
//! extended to sphere faces), not in the body. The assembly does not
//! pretend otherwise: it neither gates on it nor asserts tier 3.

use geom_brep::{EdgeCurveSpec, EdgeGeometry};
use geom_core::{Band, Bounds, Decide, Point3, Real, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;
use topo::{
    Body, EdgeKey, FaceKey, FaceSurface, HalfEdgeKey, LoopBoundary, MefSite, MevSite, ShellKey,
    SolidKey, VertexKey,
};

use super::FilletError;
use super::battery::{Chain, Convexity, FilletRequest, Link, run_battery};
use super::blend::corner_ball;

/// A filleted body: the rounded solid plus the keys of the faces the
/// blend introduced.
#[derive(Clone, Debug)]
pub struct Filleted<T: Real> {
    /// The rounded solid.
    pub body: Body<T>,
    /// Its (only) solid.
    pub solid: SolidKey,
    /// Its (only) shell.
    pub shell: ShellKey,
    /// The blend faces — the quarter-cylinder patches, one per
    /// original edge, in original-edge order.
    pub blend_faces: Vec<FaceKey>,
    /// The corner faces — the sphere patches, one per original
    /// vertex, in original-vertex order.
    pub corner_faces: Vec<FaceKey>,
    /// The torus band faces — one per CLOSED chain (the rim blends),
    /// in first-link-edge order. Empty on the whole-body path, which
    /// admits no closed chains (M6 surgery unit).
    pub band_faces: Vec<FaceKey>,
    /// **Per-entity birth records** (M6-5): what the fillet minted and
    /// which source entity each mint was made for.
    ///
    /// BOTH doors fill this since PR-2 — the composition surgery
    /// writes rows as it mutates, the whole-body rebuild reads them
    /// off its [`Plan`]. The `Option` survives only because
    /// `Filleted` is public and a future third door would have to say
    /// so explicitly rather than ship an empty table by default;
    /// `None` is a kernel bug today and `editor-core` refuses it as
    /// one, never falling back to unnamed geometry.
    pub naming: Option<super::naming::FilletNaming>,
}

/// **Fillet every edge of a convex, planar-faced, trivalent-vertex
/// polyhedron** at constant radius `radius`.
///
/// The battery runs FIRST (module docs): a refusal from
/// [`run_battery`] propagates unchanged, and no entity is minted
/// before its verdict. Only then is the request checked against the
/// one assembly front door, and only then is anything built.
///
/// # Errors
///
/// Any [`FilletError`] the battery produces;
/// [`FilletError::AssemblyUnsupported`] when the request is not the
/// whole-body case above; [`FilletError::Op`] when an Euler operator
/// refuses; [`FilletError::Certify`] when a blend description does not
/// certify as stored.
pub fn fillet_edges<T: Decide + Bounds>(
    body: &Body<T>,
    edges: &[EdgeKey],
    radius: T,
    band: Band,
) -> Result<Filleted<T>, FilletError> {
    // A repeated edge is malformed for BOTH doors (the chain walk
    // would double a link), so it refuses before the battery samples
    // anything.
    let mut requested = edges.to_vec();
    requested.sort_unstable();
    requested.dedup();
    if requested.len() != edges.len() {
        return Err(FilletError::AssemblyUnsupported {
            detail: "the request repeats an edge",
        });
    }

    // ---- The ordering contract: verdict first, unchanged. ----
    let request = FilletRequest {
        body,
        edges: edges.to_vec(),
        radius,
    };
    let verdict = run_battery(&request, band)?;

    // ---- Then the front doors: the whole-body rebuild when the
    // request is exactly that shape (bit-preserved — the M5 path),
    // otherwise the in-place composition surgery (M6 unit 1). ----
    match whole_body_links(body, edges, &verdict.chains) {
        Ok(links) => {
            let plan = Plan::derive(body, &links, radius)?;
            plan.assemble()
        }
        Err(FilletError::AssemblyUnsupported { .. }) => {
            super::surgery::fillet_surgery(body, &verdict, band)
        }
        Err(other) => Err(other),
    }
}

/// The assembly front door, stated as a predicate over the request
/// (module docs). Returns the per-original-edge links in EDGE-ARENA
/// order — the deterministic order every derived list inherits (D9).
fn whole_body_links<'a, T: Decide>(
    body: &Body<T>,
    edges: &[EdgeKey],
    chains: &'a [Chain<T>],
) -> Result<Vec<&'a Link<T>>, FilletError> {
    let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };
    if body.solids().count() != 1 || body.shells().count() != 1 {
        return Err(unsupported(
            "the body is not a single solid with a single shell",
        ));
    }
    let mut requested = edges.to_vec();
    requested.sort_unstable();
    requested.dedup();
    if requested.len() != edges.len() {
        return Err(unsupported("the request repeats an edge"));
    }
    let mut all: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    all.sort_unstable();
    if requested != all {
        return Err(unsupported(
            "the request is not EVERY edge of the body (in-place edge-blend surgery)",
        ));
    }
    for (fk, face) in body.faces() {
        if !matches!(body.get_surface(face.surface), Some(Surface::Plane { .. })) {
            return Err(unsupported("a face of the body is not planar"));
        }
        if !face.rings.is_empty() {
            return Err(unsupported("a face of the body carries rings"));
        }
        let cycle =
            face_cycle(body, fk).ok_or_else(|| unsupported("a face has no boundary cycle"))?;
        if cycle.len() < 3 {
            return Err(unsupported("a face boundary is shorter than a triangle"));
        }
    }
    for (vk, _) in body.vertices() {
        let faces =
            vertex_faces(body, vk).ok_or_else(|| unsupported("a vertex orbit does not resolve"))?;
        if faces.len() != 3 {
            return Err(unsupported(
                "a vertex is not trivalent (the sphere octant is the only corner built)",
            ));
        }
    }
    let mut links: Vec<&Link<T>> = Vec::new();
    for chain in chains {
        for link in &chain.links {
            if !matches!(link.convexity, Convexity::Convex) {
                return Err(unsupported(
                    "a requested edge is concave (the corner ball is not a sphere octant there)",
                ));
            }
            links.push(link);
        }
    }
    links.sort_by_key(|l| l.edge);
    if links.len() != all.len() {
        return Err(unsupported(
            "the resolved links do not cover the edges once each",
        ));
    }
    Ok(links)
}

/// A face's boundary cycle (outer loop, cycle order).
pub(super) fn face_cycle<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<HalfEdgeKey>> {
    let f = body.get_face(face)?;
    let LoopBoundary::Cycle { first } = body.get_loop(f.outer)?.boundary else {
        return None;
    };
    body.loop_cycle(first)
}

/// The distinct faces around a vertex, in orbit order.
pub(super) fn vertex_faces<T: Decide>(body: &Body<T>, vertex: VertexKey) -> Option<Vec<FaceKey>> {
    let he = body.get_vertex(vertex)?.emanating?;
    let mut faces = Vec::new();
    for h in body.vertex_orbit(he)? {
        let f = body.get_loop(body.get_half_edge(h)?.parent_loop)?.face;
        if !faces.contains(&f) {
            faces.push(f);
        }
    }
    Some(faces)
}

/// The octant's chart pick at one trivalent corner (fix pass F2,
/// extracted at the M6 surgery unit — one implementation for both
/// assembly doors). The criterion: the octant is an iso-parameter
/// rectangle exactly when the chart is aimed along an incident edge
/// whose axis `n_a × n_b` the THIRD support's normal is parallel to,
/// so the pick minimizes `|n_c × axis|` over the incident requested
/// links, ORDER-FREE — it finds the admitting edge whenever one
/// exists and degrades to "no chart admits this trihedron" (the
/// genuinely oblique case, tier-3 `VolumeUncomputable`) only when
/// none does. Returns `(u_ref, axis)` — the seam is the picked
/// link's first support normal.
pub(super) fn octant_chart<T: Decide + Bounds>(
    body: &Body<T>,
    vertex: VertexKey,
    faces: &[FaceKey],
    links: &[&Link<T>],
) -> Result<(Vec3<T>, Vec3<T>), FilletError> {
    let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };
    let mut best: Option<(f64, Vec3<T>, Vec3<T>)> = None;
    for l in links
        .iter()
        .filter(|l| l.start == vertex || l.end == vertex)
    {
        let (Some(n_a), Some(n_b)) = (outward_of(body, l.face_a), outward_of(body, l.face_b))
        else {
            return Err(unsupported("a corner edge has a non-planar support"));
        };
        let axis = n_a.cross(n_b).normalize();
        // The third support of the corner — the one this edge does
        // not touch. `faces` is the vertex's three, so the complement
        // is a single face.
        let Some(&f_c) = faces.iter().find(|f| **f != l.face_a && **f != l.face_b) else {
            continue;
        };
        let Some(n_c) = outward_of(body, f_c) else {
            return Err(unsupported("a corner edge has a non-planar support"));
        };
        let score = n_c.cross(axis).norm().lo().abs();
        if best.as_ref().is_none_or(|(s, _, _)| score < *s) {
            best = Some((score, n_a, axis));
        }
    }
    let (_, n_a, axis) = best.ok_or_else(|| unsupported("a vertex has no requested edge"))?;
    Ok((n_a, axis))
}

/// The octant's ORIENTATION BIT at one trivalent corner, extracted so
/// both assembly doors mint it from one implementation (the shape
/// [`octant_chart`] already has).
///
/// A corner patch is a sphere about the rolling ball's rest centre and
/// its chart normal is the outward radial, exactly as a blend's
/// cylinder chart normal is. The centre lies on the material side
/// precisely when the corner is convex, so the octant's sense is the
/// same bit its blends take, off the same stored verdict
/// ([`Convexity::blend_sense`]) — never a sampled normal.
///
/// `links` are the requested links already filtered to this corner.
/// **Neither refusal below is a reachable door**: both assembly doors
/// admit only convex chains, and a corner exists only where an open
/// link terminates, so the links are always present and always agree.
/// They are typed guards on the invariant those doors hold — kept so
/// the bit cannot rot when a door moves, in the shape
/// [`super::surgery`]'s closure guard uses.
pub(super) fn corner_convexity<T: Real>(links: &[&Link<T>]) -> Result<Convexity, FilletError> {
    let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };
    let mut convexity: Option<Convexity> = None;
    for l in links {
        if *convexity.get_or_insert(l.convexity) != l.convexity {
            return Err(unsupported(
                "a corner's incident links disagree on convexity (the corner ball is \
                 not a sphere octant there)",
            ));
        }
    }
    convexity.ok_or_else(|| unsupported("a corner has no requested incident link"))
}

/// A planar face's OUTWARD normal: the stored plane normal folded
/// through the stored sense bit (S10 category A — never sampled).
pub(super) fn outward_of<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec3<T>> {
    let f = body.get_face(face)?;
    match body.get_surface(f.surface)? {
        Surface::Plane { normal, .. } => Some(*normal * f.sense_sign::<T>()),
        _ => None,
    }
}

// ------------------------------------------------------------------
// The plan: the whole rounded solid, derived before anything is minted.
// ------------------------------------------------------------------

/// A planned edge's carrier kind — the geometry the final description
/// pass rebuilds from the body's own vertices.
#[derive(Clone, Copy, Debug)]
enum EdgeCarrier<T: Real> {
    /// A trimline: the straight contact ruling on a support plane.
    Trim,
    /// A corner arc: the contact circle of the corner ball centred at
    /// `center` with one incident blend cylinder.
    Arc {
        /// The corner ball's centre — the circle's centre.
        center: Point3<T>,
    },
}

/// One planned edge of the result.
#[derive(Clone, Copy, Debug)]
struct PlannedEdge<T: Real> {
    /// Its two planned vertices.
    ends: (usize, usize),
    /// Its carrier kind (geometry — what the description pass
    /// rebuilds).
    carrier: EdgeCarrier<T>,
    /// What it was minted for (provenance — what the naming records
    /// are written from). Kept apart from `carrier` on purpose: the
    /// two answer different questions and must be free to differ.
    origin: EdgeOrigin,
}

/// What a planned face is a patch OF — the result's coverage buckets
/// AND its birth provenance (M6-5 PR-2: the payload is what the
/// naming records are written from, so the two can never disagree).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaceKind {
    /// A shrunk planar support face, over the source face it shrinks.
    Support(FaceKey),
    /// A quarter-cylinder blend, over the original edge it rounds.
    Blend(EdgeKey),
    /// A sphere patch, over the original vertex it rounds.
    Corner(VertexKey),
}

/// What a planned EDGE was minted for (M6-5 PR-2).
#[derive(Clone, Copy, Debug)]
enum EdgeOrigin {
    /// A trimline: the source edge blended, and the support it lies
    /// in (one source edge yields two trimlines).
    Trim {
        /// The source edge.
        edge: EdgeKey,
        /// The support face.
        support: FaceKey,
    },
    /// A corner arc: the source vertex rounded, and the source edge
    /// whose blend the arc bounds.
    Arc {
        /// The source corner vertex.
        vertex: VertexKey,
        /// The source edge.
        edge: EdgeKey,
    },
}

/// One planned face of the result.
#[derive(Clone, Debug)]
struct PlannedFace<T: Real> {
    /// The boundary cycle's planned vertices, CCW about the face's
    /// OUTWARD normal.
    verts: Vec<usize>,
    /// The planned edge joining `verts[i]` to `verts[i + 1 mod k]`.
    edges: Vec<usize>,
    /// The face's surface.
    surface: Surface<T>,
    /// Its sense bit, from stored structure.
    sense: bool,
    /// Which bucket it belongs to.
    kind: FaceKind,
}

/// The complete derived result, before any entity exists.
struct Plan<T: Real> {
    points: Vec<Point3<T>>,
    /// Per planned point: the source (corner vertex, support face)
    /// whose foot it is — parallel to `points` (M6-5 PR-2).
    point_of: Vec<(VertexKey, FaceKey)>,
    edges: Vec<PlannedEdge<T>>,
    faces: Vec<PlannedFace<T>>,
    /// Every source entity the rebuild retires — which, on this door,
    /// is every edge and every vertex of the input (M6-5 PR-2).
    dead: super::naming::Retired,
    radius: T,
}

impl<T: Decide + Bounds> Plan<T> {
    /// Derive the whole rounded solid from the body and the verdict's
    /// links. Nothing here decides anything numerically — the battery
    /// already did, and every quantity below is a closed form off its
    /// stored output.
    fn derive(body: &Body<T>, links: &[&Link<T>], radius: T) -> Result<Self, FilletError> {
        let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };

        // ---- Corner balls, and the new vertices they place. ----
        let mut vindex: Vec<(VertexKey, FaceKey, usize)> = Vec::new();
        let mut points: Vec<Point3<T>> = Vec::new();
        let mut point_of: Vec<(VertexKey, FaceKey)> = Vec::new();
        let mut corners: Vec<(VertexKey, Point3<T>, Surface<T>)> = Vec::new();
        for (v, _) in body.vertices() {
            let faces = vertex_faces(body, v)
                .ok_or_else(|| unsupported("a vertex orbit does not resolve"))?;
            let p = *body
                .get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .ok_or_else(|| unsupported("a vertex point does not resolve"))?;
            let mut normals = [Vec3::new(T::zero(), T::zero(), T::zero()); 3];
            for (slot, &f) in normals.iter_mut().zip(faces.iter()) {
                *slot = outward_of(body, f)
                    .ok_or_else(|| unsupported("a support face is not planar"))?;
            }
            let ball = corner_ball([p; 3], normals, radius, true);
            for (n, &f) in normals.iter().zip(faces.iter()) {
                vindex.push((v, f, points.len()));
                point_of.push((v, f));
                points.push(ball.center + *n * radius);
            }
            // The octant's CHART, chosen so the patch is an
            // iso-parameter rectangle: `corner_ball`'s own chart aims
            // at the trihedral diagonal, under which all three
            // contact circles are oblique — neither rims nor
            // meridians — and the closed-form mass properties refuse
            // (`props_rim_axis_parallel`). Aiming the chart along ONE
            // incident edge instead makes that edge's contact circle
            // the equator RIM and — when the third support's normal is
            // parallel to that edge, as at every right trihedron —
            // the other two polar MERIDIANS, with the third foot at
            // the pole. The turn from the first support's foot to the
            // second gives the axis its sign (so `u` sweeps forward
            // from the seam) and the first foot IS the seam.
            //
            // **Which incident edge (fix pass F2).** See
            // [`octant_chart`] — extracted at the M6 surgery unit so
            // both assembly doors take the SAME order-free pick, with
            // the arithmetic unchanged.
            let (u_ref, axis) = octant_chart(body, v, &faces, links)?;
            corners.push((
                v,
                ball.center,
                Surface::Sphere {
                    center: ball.center,
                    radius,
                    axis,
                    u_ref,
                },
            ));
        }
        let foot = |v: VertexKey, f: FaceKey| -> Option<usize> {
            vindex
                .iter()
                .find(|(vv, ff, _)| *vv == v && *ff == f)
                .map(|(_, _, i)| *i)
        };

        // ---- Edges: two trimlines per original edge, three corner
        // arcs per original vertex. ----
        let mut edges: Vec<PlannedEdge<T>> = Vec::new();
        let mut trims: Vec<(EdgeKey, FaceKey, usize)> = Vec::new();
        for link in links {
            for face in [link.face_a, link.face_b] {
                let (Some(a), Some(b)) = (foot(link.start, face), foot(link.end, face)) else {
                    return Err(unsupported(
                        "an edge's support is not one of its vertices' faces",
                    ));
                };
                trims.push((link.edge, face, edges.len()));
                edges.push(PlannedEdge {
                    ends: (a, b),
                    carrier: EdgeCarrier::Trim,
                    origin: EdgeOrigin::Trim {
                        edge: link.edge,
                        support: face,
                    },
                });
            }
        }
        let mut arcs: Vec<(VertexKey, EdgeKey, usize)> = Vec::new();
        for (v, center, _) in &corners {
            for link in links {
                if link.start != *v && link.end != *v {
                    continue;
                }
                let (Some(a), Some(b)) = (foot(*v, link.face_a), foot(*v, link.face_b)) else {
                    return Err(unsupported(
                        "a corner's support is not one of its vertex's faces",
                    ));
                };
                arcs.push((*v, link.edge, edges.len()));
                edges.push(PlannedEdge {
                    ends: (a, b),
                    carrier: EdgeCarrier::Arc { center: *center },
                    origin: EdgeOrigin::Arc {
                        vertex: *v,
                        edge: link.edge,
                    },
                });
            }
        }
        let trim_of = |e: EdgeKey, f: FaceKey| -> Option<usize> {
            trims
                .iter()
                .find(|(ee, ff, _)| *ee == e && *ff == f)
                .map(|(_, _, i)| *i)
        };
        let arc_of = |v: VertexKey, e: EdgeKey| -> Option<usize> {
            arcs.iter()
                .find(|(vv, ee, _)| *vv == v && *ee == e)
                .map(|(_, _, i)| *i)
        };

        // ---- Faces. ----
        let mut faces: Vec<PlannedFace<T>> = Vec::new();

        // (a) The shrunk supports: the original loop order verbatim,
        // foot by foot, with each original edge's trimline ON THIS
        // FACE joining consecutive feet. Preserving the loop order
        // preserves the winding about the (unchanged) outward normal,
        // so tier 3's check 6 holds by construction.
        for (fk, face) in body.faces() {
            let cycle =
                face_cycle(body, fk).ok_or_else(|| unsupported("a face has no boundary cycle"))?;
            let mut verts = Vec::with_capacity(cycle.len());
            let mut ring = Vec::with_capacity(cycle.len());
            for he in &cycle {
                let h = body
                    .get_half_edge(*he)
                    .ok_or_else(|| unsupported("a half-edge does not resolve"))?;
                let (Some(i), Some(t)) = (foot(h.start, fk), trim_of(h.edge, fk)) else {
                    return Err(unsupported("a support face's trimline does not resolve"));
                };
                verts.push(i);
                ring.push(t);
            }
            faces.push(PlannedFace {
                verts,
                edges: ring,
                surface: body
                    .get_surface(face.surface)
                    .ok_or_else(|| unsupported("a face surface does not resolve"))?
                    .clone(),
                sense: face.sense,
                kind: FaceKind::Support(fk),
            });
        }

        // (b) The blends. `face_a` traverses the original edge
        // start → end (`he_plus` lies in its loop), so the shrunk
        // support traverses trim_a start → end and the BLEND — the
        // other side of that trimline — traverses it end → start;
        // symmetrically `face_b` (`he_minus`) fixes the other side.
        // That pins the quad without reading a single normal:
        //   trim_a(end→start), arc@start, trim_b(start→end), arc@end.
        for link in links {
            let (Some(a_end), Some(a_start), Some(b_start), Some(b_end)) = (
                foot(link.end, link.face_a),
                foot(link.start, link.face_a),
                foot(link.start, link.face_b),
                foot(link.end, link.face_b),
            ) else {
                return Err(unsupported("a blend's feet do not resolve"));
            };
            let (Some(ta), Some(sa), Some(tb), Some(sb)) = (
                trim_of(link.edge, link.face_a),
                arc_of(link.start, link.edge),
                trim_of(link.edge, link.face_b),
                arc_of(link.end, link.edge),
            ) else {
                return Err(unsupported("a blend's boundary edges do not resolve"));
            };
            faces.push(PlannedFace {
                verts: vec![a_end, a_start, b_start, b_end],
                edges: vec![ta, sa, tb, sb],
                surface: link.blend.surface.clone(),
                sense: link.convexity.blend_sense(),
                kind: FaceKind::Blend(link.edge),
            });
        }

        // (c) The corners. Each arc is traversed opposite to the blend
        // that shares it, which pins all three directions; the three
        // then chain into the octant's cycle with no geometry read.
        for (v, _, surface) in &corners {
            let mut directed: Vec<(usize, usize, usize)> = Vec::new();
            let mut incident: Vec<&Link<T>> = Vec::new();
            for link in links {
                let at_start = link.start == *v;
                if !at_start && link.end != *v {
                    continue;
                }
                incident.push(link);
                let (Some(a), Some(b), Some(arc)) = (
                    foot(*v, link.face_a),
                    foot(*v, link.face_b),
                    arc_of(*v, link.edge),
                ) else {
                    return Err(unsupported("a corner's boundary edges do not resolve"));
                };
                directed.push(if at_start { (b, a, arc) } else { (a, b, arc) });
            }
            let convexity = corner_convexity(&incident)?;
            let Some(&seed) = directed.first().filter(|_| directed.len() == 3) else {
                return Err(unsupported("a corner does not have exactly three arcs"));
            };
            let mut verts = Vec::with_capacity(3);
            let mut ring = Vec::with_capacity(3);
            let mut cur = seed;
            for _ in 0..3 {
                verts.push(cur.0);
                ring.push(cur.2);
                let Some(next) = directed.iter().find(|(f, _, _)| *f == cur.1).copied() else {
                    return Err(unsupported("a corner's three arcs do not close a cycle"));
                };
                cur = next;
            }
            if cur != seed {
                return Err(unsupported("a corner's three arcs do not close a cycle"));
            }
            faces.push(PlannedFace {
                verts,
                edges: ring,
                surface: surface.clone(),
                sense: convexity.blend_sense(),
                kind: FaceKind::Corner(*v),
            });
        }

        Ok(Self {
            points,
            point_of,
            edges,
            faces,
            // The door admits only the every-edge request over a
            // trivalent polyhedron, and the rebuild lands in a FRESH
            // arena: every source entity is retired by construction,
            // so this is a statement of the door's contract, not a
            // tally kept during mutation.
            dead: super::naming::Retired {
                edges: body.edges().map(|(k, _)| k).collect(),
                vertices: body.vertices().map(|(k, _)| k).collect(),
            },
            radius,
        })
    }
}

// ------------------------------------------------------------------
// Assembly: Euler operators only (D1), face by face.
// ------------------------------------------------------------------

/// The assembly's running state: which planned entities exist yet, and
/// the hole face whose loop is the built patch's boundary.
struct Build<T: Real> {
    body: Body<T>,
    hole: FaceKey,
    vkey: Vec<Option<VertexKey>>,
    ekey: Vec<Option<EdgeKey>>,
    fkey: Vec<Option<FaceKey>>,
}

/// The already-built run of one candidate face's boundary.
struct Runway {
    /// Index into the face's cycle where the run starts.
    start: usize,
    /// How many of the face's edges already exist.
    len: usize,
}

impl<T: Real> Plan<T> {
    /// The planned edge joining two planned vertices, if any.
    fn edge_between(&self, a: usize, b: usize) -> Option<usize> {
        self.edges
            .iter()
            .position(|e| e.ends == (a, b) || e.ends == (b, a))
    }
}

impl<T: Decide + Bounds> Plan<T> {
    /// Build the body: seed the first face, grow the patch greedily,
    /// and let the last planned face BE the seed's surviving hole.
    fn assemble(self) -> Result<Filleted<T>, FilletError> {
        let op = |e: topo::EulerOpError| FilletError::Op {
            detail: e.to_string(),
        };
        let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };
        let (Some(first), true) = (self.faces.first(), self.faces.len() >= 2) else {
            return Err(unsupported("fewer than two faces to assemble"));
        };

        // ---- Seed: mvfs at the first face's first vertex, then one
        // `mev(Lone)` so the general routine has a run to hang on. ----
        let (Some(&w0), Some(&w1)) = (first.verts.first(), first.verts.get(1)) else {
            return Err(unsupported("the seed face has fewer than two vertices"));
        };
        let (Some(&p0), Some(&p1)) = (self.points.get(w0), self.points.get(w1)) else {
            return Err(unsupported("the seed face's points do not resolve"));
        };
        let mut body = Body::<T>::new();
        let seed = body.mvfs(p0).map_err(op)?;
        let opening = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p1,
                EdgeCurveSpec::line_between(p0, p1),
            )
            .map_err(op)?;
        let mut state = Build {
            body,
            hole: seed.face,
            vkey: vec![None; self.points.len()],
            ekey: vec![None; self.edges.len()],
            fkey: vec![None; self.faces.len()],
        };
        state.vkey[w0] = Some(seed.vertex);
        state.vkey[w1] = Some(opening.vertex);
        let Some(&seam) = first.edges.first() else {
            return Err(unsupported("the seed face has no edges"));
        };
        state.ekey[seam] = Some(opening.edge);

        // ---- Grow: one face per step, the face sharing the most
        // boundary first (which keeps the hole's loop simple). The
        // LAST planned face is never minted — it is the seed's hole,
        // which by then bounds exactly that face's cycle. ----
        let mut built = vec![false; self.faces.len()];
        let mut remaining = self.faces.len();
        while remaining > 1 {
            let mut best: Option<(usize, Runway)> = None;
            for (i, face) in self.faces.iter().enumerate() {
                if built[i] {
                    continue;
                }
                let Some(run) = self.runway(&state, face) else {
                    continue;
                };
                if best.as_ref().is_none_or(|(_, b)| run.len > b.len) {
                    best = Some((i, run));
                }
            }
            let Some((i, run)) = best else {
                return Err(unsupported(
                    "the face order left no completable face (the patch boundary pinched)",
                ));
            };
            self.mint_face(&mut state, i, &run)?;
            built[i] = true;
            remaining -= 1;
        }

        // ---- The surviving hole IS the last planned face. ----
        let last = built
            .iter()
            .position(|b| !b)
            .ok_or_else(|| unsupported("no face left for the hole"))?;
        self.check_hole(&state, last)?;
        let Some(surface) = self.faces.get(last).map(|f| f.surface.clone()) else {
            return Err(unsupported("the hole's planned face does not resolve"));
        };
        state
            .body
            .set_face_surface(state.hole, FaceSurface::New(surface))
            .map_err(op)?;
        state.fkey[last] = Some(state.hole);

        // ---- Sense bits, from stored structure. ----
        for (i, face) in self.faces.iter().enumerate() {
            let Some(fk) = state.fkey[i] else {
                return Err(unsupported("a planned face was never minted"));
            };
            state.body.set_face_sense(fk, face.sense).map_err(op)?;
        }

        // ---- The prefer-intrinsic pass: every trimline and every
        // corner arc re-describes as the tangential contact locus of
        // its two adjacent faces' surfaces, now that both exist. ----
        for (i, planned) in self.edges.iter().enumerate() {
            let Some(ek) = state.ekey[i] else {
                return Err(unsupported("a planned edge was never minted"));
            };
            let kind = match planned.carrier {
                EdgeCarrier::Trim => "trimline",
                EdgeCarrier::Arc { .. } => "corner arc",
            };
            let spec = self.contact_spec(&state.body, ek, planned.carrier)?;
            state.body.set_edge_curve(ek, spec).map_err(|e| match e {
                topo::EulerOpError::Certification { error } => FilletError::Certify {
                    detail: format!("{kind} {i}: {error}"),
                },
                other => FilletError::Op {
                    detail: format!("{kind} {i}: {other}"),
                },
            })?;
        }

        // The structural postcondition, exactly as extrude's (tier 3
        // is NOT asserted here — see the module docs' known
        // limitation on oblique trihedra).
        #[cfg(debug_assertions)]
        debug_assert_eq!(
            topo::validate_closed(&state.body),
            Ok(()),
            "fillet postcondition: the result is not tier-2 valid (kernel bug)",
        );

        let bucket = |want: fn(&FaceKind) -> bool| -> Vec<FaceKey> {
            self.faces
                .iter()
                .enumerate()
                .filter(|(_, f)| want(&f.kind))
                .filter_map(|(i, _)| state.fkey[i])
                .collect()
        };

        // ---- Birth records (M6-5 PR-2). Every row is read off the
        // PLAN's own provenance — the same payloads the assembly built
        // from — paired with the key that plan slot minted. Nothing is
        // recovered afterwards by matching geometry, which is exactly
        // what N4 forbids; the surgery door's discipline, verbatim.
        //
        // The payloads are trusted, not checked: nothing outside
        // naming consumes them (module docs). A swapped payload emits
        // a wrong name silently, so the guard is that each is written
        // at the site that decided the mint — see `Plan::derive`.
        //
        // This door has no survivors: the rebuild lands in a fresh
        // arena, so even a shrunk support face is a NEW key and gets
        // its own `supports` row. The one thing the plan cannot state
        // is a slot that was never minted — `assemble` has already
        // refused ("a planned face was never minted") above, so every
        // `fkey`/`ekey`/`vkey` below resolves. ----
        let mut rec = super::naming::FilletNaming {
            dead: self.dead.clone(),
            ..Default::default()
        };
        for (i, planned) in self.faces.iter().enumerate() {
            let Some(fk) = state.fkey[i] else {
                return Err(unsupported("a planned face was never minted"));
            };
            match planned.kind {
                FaceKind::Support(src) => rec.supports.push((fk, src)),
                FaceKind::Blend(src) => rec.blends.push((fk, src)),
                FaceKind::Corner(src) => rec.corners.push((fk, src)),
            }
        }
        for (i, planned) in self.edges.iter().enumerate() {
            let Some(ek) = state.ekey[i] else {
                return Err(unsupported("a planned edge was never minted"));
            };
            match planned.origin {
                EdgeOrigin::Trim { edge, support } => rec.trims.push((ek, edge, support)),
                EdgeOrigin::Arc { vertex, edge } => rec.arcs.push((ek, vertex, edge)),
            }
        }
        for (i, (v, f)) in self.point_of.iter().enumerate() {
            let Some(vk) = state.vkey[i] else {
                return Err(unsupported("a planned vertex was never minted"));
            };
            rec.feet.push((vk, *v, *f));
        }
        // Final pass (M6-3, walk row 4): every fillet chart mints —
        // the quarter-cylinders since M5 PR 6, the corner sphere
        // octants since the analytic-chart completion — so a
        // whole-body fillet result carries its stored certified
        // pcurves at rest, the same posture as the composition
        // surgery's (`surgery.rs`) and every other constructor's.
        let mut body = state.body;
        topo::mint_pcurves(&mut body).map_err(|e| FilletError::Certify {
            detail: format!("pcurve mint after the whole-body rebuild: {e:?}"),
        })?;
        Ok(Filleted {
            blend_faces: bucket(|k| matches!(k, FaceKind::Blend(_))),
            corner_faces: bucket(|k| matches!(k, FaceKind::Corner(_))),
            // This door admits no closed chains (`whole_body_links`),
            // so there is no rim to band.
            band_faces: Vec::new(),
            naming: Some(rec),
            body,
            solid: seed.solid,
            shell: seed.shell,
        })
    }

    /// Can `face` be completed from the current hole boundary? It can
    /// exactly when its already-built edges form ONE contiguous run
    /// (in the face's cycle AND in the hole's loop) and every vertex
    /// the missing path passes through is still unbuilt.
    fn runway(&self, state: &Build<T>, face: &PlannedFace<T>) -> Option<Runway> {
        let k = face.verts.len();
        let mut hes: Vec<Option<HalfEdgeKey>> = Vec::with_capacity(k);
        for i in 0..k {
            let found = (|| {
                let a = state.vkey[*face.verts.get(i)?]?;
                let b = state.vkey[*face.verts.get((i + 1) % k)?]?;
                state.body.find_half_edge(state.hole, a, b)
            })();
            hes.push(found);
        }
        let count = hes.iter().filter(|h| h.is_some()).count();
        // A face with no boundary yet cannot be hung anywhere; a face
        // with ALL of it is the hole itself (handled at the end).
        if count == 0 || count == k {
            return None;
        }
        let starts: Vec<usize> = (0..k)
            .filter(|&i| hes[i].is_some() && hes[(i + k - 1) % k].is_none())
            .collect();
        let &start = starts.first().filter(|_| starts.len() == 1)?;
        // Contiguity in the HOLE's loop, not only in the face's cycle:
        // a boundary that pinches at a vertex can present a run that
        // is not one arc of the hole.
        for step in 0..count.saturating_sub(1) {
            let here = hes[(start + step) % k]?;
            let next = hes[(start + step + 1) % k]?;
            if state.body.get_half_edge(here)?.next != next {
                return None;
            }
        }
        // Every vertex on the missing path must still be unbuilt.
        for step in 1..(k - count) {
            if state.vkey[*face.verts.get((start + count + step) % k)?].is_some() {
                return None;
            }
        }
        Some(Runway { start, len: count })
    }

    /// Mint one face: `mev` the missing path's interior vertices as a
    /// strut chain off the run's far end, then one `mef` to close.
    fn mint_face(
        &self,
        state: &mut Build<T>,
        index: usize,
        run: &Runway,
    ) -> Result<(), FilletError> {
        let op = |e: topo::EulerOpError| FilletError::Op {
            detail: e.to_string(),
        };
        let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };
        let stale = || unsupported("the patch boundary went stale during assembly");
        let face = self.faces.get(index).ok_or_else(stale)?;
        let k = face.verts.len();
        let at = |i: usize| -> Result<usize, FilletError> {
            face.verts.get(i % k).copied().ok_or_else(stale)
        };
        let point = |i: usize| -> Result<Point3<T>, FilletError> {
            self.points.get(at(i)?).copied().ok_or_else(stale)
        };
        let vertex = |i: usize| -> Result<VertexKey, FilletError> {
            state.vkey.get(at(i)?).copied().flatten().ok_or_else(stale)
        };

        // `he1` opens the run (its start is the new face's first
        // vertex); the run's far end names the splice site.
        let he1 = state
            .body
            .find_half_edge(state.hole, vertex(run.start)?, vertex(run.start + 1)?)
            .ok_or_else(stale)?;
        let tail = state
            .body
            .find_half_edge(
                state.hole,
                vertex(run.start + run.len - 1)?,
                vertex(run.start + run.len)?,
            )
            .ok_or_else(stale)?;
        let mut site = state.body.get_half_edge(tail).ok_or_else(stale)?.next;

        // The strut chain: each `mev` splices its two halves in just
        // before the current site, so the chain grows outward along
        // the missing path and the site walks to the new minus half.
        let mut he2 = site;
        for step in 0..(k - run.len - 1) {
            let from = run.start + run.len + step;
            let created = state
                .body
                .mev(
                    MevSite::Fan {
                        he1: site,
                        he2: site,
                    },
                    point(from + 1)?,
                    EdgeCurveSpec::line_between(point(from)?, point(from + 1)?),
                )
                .map_err(op)?;
            let target = at(from + 1)?;
            if let Some(slot) = state.vkey.get_mut(target) {
                *slot = Some(created.vertex);
            }
            let e = self
                .edge_between(at(from)?, target)
                .ok_or_else(|| unsupported("a face cycle uses an edge the plan does not have"))?;
            if let Some(slot) = state.ekey.get_mut(e) {
                *slot = Some(created.edge);
            }
            site = created.he_minus;
            he2 = created.he_minus;
        }

        // The closing `mef`: `he_plus` runs start(he1) → start(he2),
        // and `he1`'s side of the split becomes the NEW face's outer
        // loop — which is exactly this face's cycle.
        let last = run.start + k - 1;
        let created = state
            .body
            .mef(
                MefSite::Chords { he1, he2 },
                EdgeCurveSpec::line_between(point(run.start)?, point(last)?),
                FaceSurface::New(face.surface.clone()),
            )
            .map_err(op)?;
        let e = self
            .edge_between(at(last)?, at(run.start)?)
            .ok_or_else(|| unsupported("a face cycle uses an edge the plan does not have"))?;
        if let Some(slot) = state.ekey.get_mut(e) {
            *slot = Some(created.edge);
        }
        if let Some(slot) = state.fkey.get_mut(index) {
            *slot = Some(created.face);
        }
        Ok(())
    }

    /// The surviving hole must bound exactly the last planned face's
    /// cycle (a rotation of it) — checked, never assumed.
    fn check_hole(&self, state: &Build<T>, last: usize) -> Result<(), FilletError> {
        let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };
        let mismatch = || unsupported("the surviving hole is not the last planned face");
        let face = self.faces.get(last).ok_or_else(mismatch)?;
        let cycle = face_cycle(&state.body, state.hole)
            .ok_or_else(|| unsupported("the hole has no boundary cycle"))?;
        if cycle.len() != face.verts.len() {
            return Err(mismatch());
        }
        let found: Vec<Option<VertexKey>> = cycle
            .iter()
            .map(|he| state.body.get_half_edge(*he).map(|h| h.start))
            .collect();
        let want: Vec<Option<VertexKey>> = face
            .verts
            .iter()
            .map(|&i| state.vkey.get(i).copied().flatten())
            .collect();
        let k = want.len();
        let aligned = (0..k).any(|shift| (0..k).all(|i| found[i] == want[(i + shift) % k]));
        if aligned { Ok(()) } else { Err(mismatch()) }
    }

    /// The intrinsic description a trimline / corner arc is born with
    /// (module docs): `TangentIntersection` over the edge's two
    /// adjacent faces' surfaces, with the carrier rebuilt from the
    /// edge's OWN `he_plus` endpoints so the witness is exactly the
    /// mid-parameter point the certification contract pins.
    fn contact_spec(
        &self,
        body: &Body<T>,
        edge: EdgeKey,
        carrier: EdgeCarrier<T>,
    ) -> Result<EdgeCurveSpec<T>, FilletError> {
        let unsupported = |detail: &'static str| FilletError::AssemblyUnsupported { detail };
        let e = body
            .get_edge(edge)
            .ok_or_else(|| unsupported("an edge went stale before its description"))?;
        let face_surface = |he: HalfEdgeKey| -> Option<geom_brep::SurfaceKey> {
            let h = body.get_half_edge(he)?;
            Some(body.get_face(body.get_loop(h.parent_loop)?.face)?.surface)
        };
        let (Some(s1), Some(s2)) = (face_surface(e.he_plus), face_surface(e.he_minus)) else {
            return Err(unsupported("an edge's adjacent surfaces do not resolve"));
        };
        let point = |v: Option<VertexKey>| -> Option<Point3<T>> {
            body.get_point(body.get_vertex(v?)?.point).copied()
        };
        let (Some(p0), Some(p1)) = (
            point(body.get_half_edge(e.he_plus).map(|h| h.start)),
            point(body.half_edge_end(e.he_plus)),
        ) else {
            return Err(unsupported("an edge's endpoints do not resolve"));
        };
        let (curve, t0, t1) = match carrier {
            EdgeCarrier::Trim => {
                let len = p0.distance(p1);
                (
                    Curve3::Line {
                        origin: p0,
                        dir: (p1 - p0) / len,
                    },
                    T::zero(),
                    len,
                )
            }
            // The contact circle is the ball's equator in the plane
            // normal to the incident cylinder's axis, so the two feet
            // and the ball centre determine it outright: `u_ref` is
            // the start foot's radial, the axis their turn direction,
            // and the swept angle their subtended angle.
            EdgeCarrier::Arc { center } => {
                let u = (p0 - center).normalize();
                let w = (p1 - center).normalize();
                let turn = u.cross(w);
                (
                    Curve3::Circle {
                        center,
                        axis: turn.normalize(),
                        radius: self.radius,
                        u_ref: u,
                    },
                    T::zero(),
                    turn.norm().atan2(u.dot(w)),
                )
            }
        };
        let witness = curve.eval((t0 + t1) * T::from_f64(0.5));
        Ok(EdgeCurveSpec {
            description: EdgeGeometry::TangentIntersection { s1, s2, witness },
            carrier: curve,
            param_start: t0,
            param_end: t1,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use topo::Body;

    use super::super::battery::{Convexity, Link};
    use super::{FaceKind, Plan};
    use crate::test_support::{L, R, all_links, cube};

    /// The corner octants' sense bits, in plan order.
    fn corner_senses(body: &Body<f64>, links: &[Link<f64>]) -> Vec<bool> {
        let refs: Vec<&Link<f64>> = links.iter().collect();
        Plan::derive(body, &refs, R)
            .expect("the plan derives")
            .faces
            .iter()
            .filter(|f| matches!(f.kind, FaceKind::Corner(_)))
            .map(|f| f.sense)
            .collect()
    }

    /// **The corner octant's sense is DERIVED, never assumed.** The
    /// ball centre lies on the material side exactly when the corner
    /// is convex, so the sphere chart's outward radial is the solid's
    /// outward normal there and inverts with the corner — the same bit
    /// a blend takes, from the same stored verdict.
    ///
    /// **The concave half of this probe is not a body.** No concave
    /// corner can be built or admitted today, so the fixture instead
    /// FALSIFIES the battery's stored verdict on a cube whose geometry
    /// is untouched — a lie about a convex body, not a concave one.
    /// It is the only probe that reaches the mint site, because
    /// `whole_body_links` refuses a concave link at the door and
    /// `Plan::derive` sits below it. What it pins is exactly the
    /// dependency: the octant reads the verdict and nothing else, so
    /// the day the door admits concave chains the octants follow
    /// without a second edit.
    #[test]
    fn a_corner_octant_takes_its_links_sense() {
        let body = cube(L);
        let mut links = all_links(&body);
        assert_eq!(corner_senses(&body, &links), vec![true; 8]);
        for l in &mut links {
            l.convexity = Convexity::Concave;
        }
        assert_eq!(corner_senses(&body, &links), vec![false; 8]);
    }

    /// A corner whose incident links disagree on convexity is not a
    /// ball octant at all: the derivation refuses rather than picking
    /// one of the three.
    #[test]
    fn a_corner_whose_links_disagree_refuses() {
        let body = cube(L);
        let mut links = all_links(&body);
        links[0].convexity = Convexity::Concave;
        let refs: Vec<&Link<f64>> = links.iter().collect();
        assert!(
            Plan::derive(&body, &refs, R).is_err(),
            "a mixed-convexity corner must refuse, not pick a link"
        );
    }
}

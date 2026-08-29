//! `offset_planes_together` — the SIMULTANEOUS offset door, planar
//! corners.
//!
//! # Why this exists, and why the per-face door could not be widened
//!
//! [`crate::replace_faces_offset`] moves ONE chart and re-describes its
//! boundary against neighbours that did not move. Composing it over
//! every chart of a body — which is what `shell` did — cannot produce
//! an offset body at an OBLIQUE junction, and the reason is arithmetic
//! rather than a posture:
//!
//! A corner where planes `n₁, n₂, n₃` meet is visited once per chart,
//! and each visit transports it rigidly by that chart's own `d·nᵢ`, so
//! it accumulates `Σ dᵢ·nᵢ`. The corner an offset body needs is the
//! point satisfying `nᵢ·x = nᵢ·oᵢ + dᵢ` for every `i` at once. Those
//! agree exactly when the normals are mutually perpendicular — which is
//! why a box has always been right and is bit-identical here — and
//! diverge otherwise: on a regular hexagonal prism at `t = 0.02` the
//! accumulation lands 11.5 mm from the true corner and leaves 30 mm of
//! wall where 20 mm was asked for.
//!
//! `ReanchorOffCarrier` is the gate that has been PREVENTING that body,
//! and it stays load-bearing for everything this door does not cover.
//! What this door does instead is solve the corner ONCE, against every
//! moved plane meeting it, and re-derive each edge as the intersection
//! of its TWO MOVED planes.
//!
//! # Scope, stated as a gate rather than as a hope
//!
//! **Every face of the body must be planar and must be in the moving
//! set.** Both are checked and refused typed
//! ([`ReplaceFaceError::TogetherNonPlanar`],
//! [`ReplaceFaceError::TogetherPartialSet`]) — a body with one curved
//! face has corners this door cannot solve, and a partially moving set
//! has corners whose answer depends on faces it was not told about.
//! Curved corners are the C5-table work that follows this unit; until
//! it lands they refuse where they always did.
//!
//! # What every step is, exactly
//!
//! Two planes translate, so their intersection line keeps its direction
//! and translates too — every motion here is a rigid translation, and
//! that is what keeps the door closed-form:
//!
//! 1. **the corner** — `nᵢ·x = cᵢ` over the distinct moved planes at
//!    the vertex, solved by Cramer on the first well-conditioned
//!    triple in ORBIT order — the order the vertex's own fan is walked,
//!    which is what makes the choice reproducible, not the face
//!    arena's; any further plane is VERIFIED against the
//!    solution rather than assumed
//!    ([`ReplaceFaceError::TogetherCorner`] when it disagrees, which is
//!    the valence-past-3 shape);
//! 2. **the edge** — its own line, translated perpendicular to itself
//!    by however far the two moved planes carried it, with the
//!    endpoints re-read because the third plane at each corner is
//!    moving too and slides them ALONG it. The old carrier's
//!    conventional data survives that (a `t = 0` anchor on a line is
//!    conventional, D2), which is what keeps an unmoved corner's edge
//!    bit-identical; a seam between two faces of ONE chart is not an
//!    intersection at all and translates with its own plane;
//! 3. **the description** — an intrinsic one re-points at the new
//!    surface keys and re-states its witness at the new mid-parameter;
//!    a mapped one translates by the edge's own displacement, which is
//!    the same rigid vector the carrier moved by.
//!
//! The conditioning of step 1 is metered, not assumed. A corner's
//! solve amplifies each plane's position error by `1/|det|`, and
//! `|det|` — a triple product of UNIT normals — is a pure number, which
//! no band in meters can classify. Its arm is the corner's OWN
//! geometry: the EDGES that end there ([`Margin::levered`]'s
//! documented shape, a dimensionless quantity times a length). The
//! question the margin asks is whether the displacement `ε/|det|`
//! induced by a plane's own tolerance stays below the lengths that make
//! this a corner at all.
//!
//! **It is deliberately NOT levered by the offset.** That would make
//! the verdict a statement about the REQUEST wearing the words of a
//! statement about the geometry: a cube's corner would read "singular"
//! at a small enough thickness, and a near-degenerate prism would build
//! at a large enough one. A corner asked to move nothing is answered
//! before any meter runs — it does not move — so the only shape that
//! reaches the word "singular" is one that is.

use geom::{Curve3, Surface};
use geom_brep::{EdgeAuthority, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Tol, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::euler::FaceSurface;
use crate::geometry::SurfaceKey;
use crate::props::PropsQuadLane;
use crate::replace_face::ReplaceFaceError;

/// One chart's move: the faces wearing it, and the signed distance
/// along its stored normal.
///
/// Both structural preconditions — the faces share ONE surface key,
/// and no face is named twice across the whole call — are ENFORCED at
/// the door ([`ReplaceFaceError::TogetherChartMixed`],
/// [`ReplaceFaceError::TogetherFaceRepeated`]). They were documented
/// and unenforced once, and a violation then arrived as a loud but
/// MISATTRIBUTED refusal from somewhere downstream; an arena scan is
/// cheap and names the true cause.
pub struct ChartMove<T: Real> {
    /// The faces wearing the chart. They share one surface key.
    pub faces: Vec<FaceKey>,
    /// The signed offset along the chart's stored normal.
    pub distance: T,
}

/// A moved plane, resolved once and read many times.
struct MovedPlane<T: Real> {
    old_key: SurfaceKey,
    normal: Vec3<T>,
    /// `n · x = c` for the MOVED plane.
    c: T,
    /// The rigid displacement the plane itself underwent.
    delta: Vec3<T>,
}

/// **Offset every chart of `body` at once** (module docs).
///
/// `moves` names each chart and its signed distance; every face of the
/// body must appear exactly once across them.
///
/// # Errors
///
/// [`ReplaceFaceError`], the body untouched on every one: the whole
/// plan is decided before anything is written, and the writes go to a
/// clone that replaces `body` only on success.
pub fn offset_planes_together<T: Decide + PropsQuadLane>(
    body: &mut Body<T>,
    moves: &[ChartMove<T>],
    band: Band,
    tol: Tol,
) -> Result<(), ReplaceFaceError<T>> {
    // ---- Decide: the chart moves are well formed. ----
    //
    // One surface key per chart and no face named twice: both are
    // structural, both are one arena scan, and both are named HERE so
    // a caller's mistake does not surface as a downstream refusal
    // about something else.
    let mut seen: Vec<FaceKey> = Vec::new();
    for m in moves {
        let Some(&first) = m.faces.first() else {
            return Err(ReplaceFaceError::EmptyGroup);
        };
        let key = body
            .get_face(first)
            .ok_or(ReplaceFaceError::StaleFace { face: first })?
            .surface;
        for &face in &m.faces {
            let data = body
                .get_face(face)
                .ok_or(ReplaceFaceError::StaleFace { face })?;
            if data.surface != key {
                return Err(ReplaceFaceError::TogetherChartMixed { face, other: first });
            }
            if seen.contains(&face) {
                return Err(ReplaceFaceError::TogetherFaceRepeated { face });
            }
            seen.push(face);
        }
    }

    // ---- Decide: the scope gate. ----
    let mut planes: Vec<(FaceKey, MovedPlane<T>)> = Vec::new();
    for m in moves {
        for &face in &m.faces {
            let data = body
                .get_face(face)
                .ok_or(ReplaceFaceError::StaleFace { face })?;
            let surface = body
                .get_surface(data.surface)
                .ok_or(ReplaceFaceError::Corrupt)?;
            let Surface::Plane { origin, normal, .. } = surface else {
                return Err(ReplaceFaceError::TogetherNonPlanar {
                    face,
                    kind: geom_brep::SurfaceKind::of(surface),
                });
            };
            let delta = *normal * m.distance;
            planes.push((
                face,
                MovedPlane {
                    old_key: data.surface,
                    normal: *normal,
                    c: normal.dot(radius(*origin + delta)),
                    delta,
                },
            ));
        }
    }
    for (face, _) in body.faces() {
        if !planes.iter().any(|(k, _)| *k == face) {
            return Err(ReplaceFaceError::TogetherPartialSet { face });
        }
    }
    let plane_of = |face: FaceKey| planes.iter().find(|(k, _)| *k == face).map(|(_, p)| p);

    // ---- Decide: every corner, before anything is written. ----
    let mut moved: Vec<(VertexKey, Point3<T>)> = Vec::new();
    for (vertex, _) in body.vertices() {
        let mut at: Vec<&MovedPlane<T>> = Vec::new();
        for face in faces_at_vertex(body, vertex)? {
            let p = plane_of(face).ok_or(ReplaceFaceError::Corrupt)?;
            if !at.iter().any(|q| q.old_key == p.old_key) {
                at.push(p);
            }
        }
        let here = body
            .get_vertex(vertex)
            .and_then(|v| body.get_point(v.point).copied())
            .ok_or(ReplaceFaceError::Corrupt)?;
        let arms = corner_arms(body, vertex, here)?;
        moved.push((vertex, solve_corner(vertex, here, &at, &arms, band)?));
    }
    let point_at = |v: VertexKey| moved.iter().find(|(k, _)| *k == v).map(|(_, p)| *p);

    // ---- Decide: every edge's carrier and description. ----
    let mut specs: Vec<(EdgeKey, EdgeCurveSpec<T>)> = Vec::new();
    for (edge, edge_data) in body.edges() {
        let (fa, fb) =
            crate::replace_face::edge_faces(body, edge).ok_or(ReplaceFaceError::Corrupt)?;
        let (pa, pb) = (
            plane_of(fa).ok_or(ReplaceFaceError::Corrupt)?,
            plane_of(fb).ok_or(ReplaceFaceError::Corrupt)?,
        );
        let start = body
            .get_half_edge(edge_data.he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?
            .start;
        let end = body
            .half_edge_end(edge_data.he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let (p_start, p_end) = (
            point_at(start).ok_or(ReplaceFaceError::Corrupt)?,
            point_at(end).ok_or(ReplaceFaceError::Corrupt)?,
        );
        let curve = body
            .get_curve_geom(edge_data.curve)
            .and_then(crate::null::CurveGeom::certified)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let old_carrier = curve.carrier().clone();
        let (t0_old, t1_old) = curve.params();
        let description = curve.description().clone();
        let authority = curve.authority();
        let old_start = old_carrier.eval(t0_old);

        // A SEAM between two faces of one chart is not an intersection
        // of two planes — the two are the same plane — so it moves with
        // that plane and keeps its parameters. Every other edge is the
        // line the two moved planes meet in.
        let (carrier, t0, t1) = if pa.old_key == pb.old_key {
            (
                crate::replace_face::translate_curve(&old_carrier, pa.delta)
                    .map_err(|error| ReplaceFaceError::Structure { edge, error })?,
                t0_old,
                t1_old,
            )
        } else {
            // The new line is the old one TRANSLATED. Two planes
            // translate, so the line they meet in keeps its direction
            // and moves PERPENDICULAR to it — which means the old
            // carrier's conventional data (its `t = 0` anchor, its
            // direction, signed zeros and all) is still valid data, and
            // carrying it rather than re-deriving an origin from a
            // cross product is what keeps a body whose corners did not
            // move bit-identical. The endpoints still SLIDE along the
            // line, because the third plane at each corner is moving
            // too, so the parameters are read afresh.
            let Curve3::Line { origin, dir } = old_carrier else {
                return Err(ReplaceFaceError::CarrierLaneUnsupported {
                    edge,
                    what: "an edge between two distinct planes whose carrier is not a line",
                });
            };
            let shift = p_start - old_start;
            let origin = origin + (shift - dir * shift.dot(dir));
            // The other endpoint is VERIFIED onto that line rather than
            // assumed onto it: it is a different corner solve, and two
            // solves agreeing is the claim being made.
            let t1 = (p_end - origin).dot(dir);
            let gap = (origin + dir * t1).distance(p_end);
            match decide("offset_together_edge_agreement", Margin::of(gap), band) {
                Ok(Sign::Zero) => {}
                Ok(_) => return Err(ReplaceFaceError::TogetherEdgeDisagreement { edge, gap }),
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
            (
                Curve3::Line { origin, dir },
                (p_start - origin).dot(dir),
                t1,
            )
        };

        let mid = carrier.eval((t0 + t1) * T::from_f64(0.5));
        let displacement = p_start - old_start;
        specs.push((
            edge,
            EdgeCurveSpec {
                description: restate(description, authority, mid, displacement, edge)?,
                carrier,
                param_start: t0,
                param_end: t1,
            },
        ));
    }

    // ---- Mutation, on a clone (every decision is done). ----
    let mut work = body.clone();
    let mut minted: Vec<(SurfaceKey, SurfaceKey)> = Vec::new();
    for m in moves {
        let Some(&first) = m.faces.first() else {
            return Err(ReplaceFaceError::EmptyGroup);
        };
        let p = plane_of(first).ok_or(ReplaceFaceError::Corrupt)?;
        let Some(Surface::Plane { origin, u_ref, .. }) = work
            .get_face(first)
            .and_then(|f| work.get_surface(f.surface))
            .cloned()
        else {
            return Err(ReplaceFaceError::Corrupt);
        };
        let new_key = work
            .set_face_surface(
                first,
                FaceSurface::New(Surface::Plane {
                    origin: origin + p.delta,
                    normal: p.normal,
                    u_ref,
                }),
            )
            .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
        for &member in &m.faces[1..] {
            work.set_face_surface(member, FaceSurface::Shared(new_key))
                .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
        }
        minted.push((p.old_key, new_key));
    }
    for (vertex, point) in &moved {
        let old_point = work
            .get_vertex(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point;
        let new_point = work.add_point(*point);
        work.get_vertex_mut(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point = new_point;
        work.remove_point_if_orphaned(old_point);
    }
    for (edge, mut spec) in specs {
        for (old, new) in &minted {
            spec.description = crate::replace_face::remap_description(spec.description, *old, *new);
        }
        work.set_edge_curve(edge, spec, tol)
            .map_err(|error| ReplaceFaceError::Op {
                edge: Some(edge),
                error,
            })?;
    }
    // Every edge of the body was just re-described, so every stored
    // pcurve row is stale — re-minted here for the same reason
    // `replace_faces_offset` re-mints, and before the tier-2 gate that
    // adopts the clone.
    //
    // **KEPT although it is inert today, deliberately.** Only MINTING
    // charts carry pcurve rows and a plane is not one, so on every body
    // this door currently accepts the pass clears nothing and cannot go
    // red — which means it is also not covered by any row here, and
    // that is said rather than left for a reader to discover. It stays
    // because the door's scope gate is the only thing making it inert:
    // the curved corners that follow bring charts that DO mint, and a
    // door that re-describes every edge in the body without re-minting
    // would be storing stale rows the moment they arrive. The census
    // posture (`Maintains`, by re-minting) is therefore honest now and
    // stays honest then.
    crate::pcurves::mint_pcurves(&mut work, tol)
        .map_err(|source| ReplaceFaceError::Pcurve { source })?;
    if let Err(errors) = crate::validate::validate_closed(&work) {
        return Err(ReplaceFaceError::ResultNotClosed { errors });
    }
    *body = work;
    Ok(())
}

/// `description` re-stated for the moved edge: an intrinsic one keeps
/// its (about to be remapped) surfaces with the witness at the new
/// mid-parameter; a mapped one translates by the edge's own rigid
/// displacement.
///
/// **A near-twin of `replace_face::plan_edge`'s description arm, and
/// the difference is why it is not shared.** That one re-states a
/// description in which exactly ONE named surface moved, so it must
/// pick out the moved key and route the pair through the C5 table;
/// here EVERY named surface moves, the pair is unchanged, and the
/// remap is a bulk pass at the end. Sharing them would mean a
/// parameter selecting which of two different obligations to
/// discharge. The duplication is one `match` over five variants and
/// this note is its disclosure.
fn restate<T: Real>(
    description: EdgeDescription<T>,
    authority: EdgeAuthority<T>,
    mid: Point3<T>,
    displacement: Vec3<T>,
    edge: EdgeKey,
) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>> {
    // **The pushforward is carried, wherever it lives** (PCURVE P-1b,
    // at the merge). This function was written against the
    // pre-collapse taxonomy, where a conventional locus WAS a
    // `MappedCurve` and translating the description was the whole job.
    // U2 restated such loci as chart images and moved the pushforward
    // beside them as the authority record, so the job splits in two:
    // the image is in the chart's own coordinates and a rigid
    // displacement of the chart leaves it alone, while the declaration
    // is 3-space sketch data and still has to be translated.
    //
    // Getting only the first half right is exactly the defect this
    // unit shipped and had to fix in `replace_face`'s offset lane
    // (`declared: None` silently destroying the record); the same
    // question is answered the same way here rather than rediscovered.
    // Unlike that lane, `offset_together` moves planes RIGIDLY by
    // construction, so a displacement always exists and only the
    // rotation-family refusal remains reachable.
    let carried = |mc: geom_brep::MappedCurve<T>| {
        crate::replace_face::translate_mapped(mc, displacement).ok_or(
            ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "a rotation-family mapped description (its trajectory does not \
                       translate)",
            },
        )
    };
    Ok(match description {
        EdgeDescription::Intersection { s1, s2, .. } => EdgeDescriptionSpec::Intersection {
            s1,
            s2,
            witness: mid,
        },
        EdgeDescription::TangentIntersection { s1, s2, .. } => {
            EdgeDescriptionSpec::TangentIntersection {
                s1,
                s2,
                witness: mid,
            }
        }
        EdgeDescription::Chart(c) => EdgeDescriptionSpec::Chart {
            surface: c.surface,
            image: Some(c.pcurve),
            seam: c.seam,
            declared: match authority {
                EdgeAuthority::Derived => None,
                EdgeAuthority::Declared(mc) => Some(carried(mc)?),
            },
        },
        EdgeDescription::Scaffold(m) => EdgeDescriptionSpec::Scaffold(carried(m)?),
    })
}

/// The corner: `nᵢ·x = cᵢ` over the distinct moved planes at a vertex.
fn solve_corner<T: Decide>(
    vertex: VertexKey,
    here: Point3<T>,
    at: &[&MovedPlane<T>],
    arms: &[T],
    band: Band,
) -> Result<Point3<T>, ReplaceFaceError<T>> {
    // **A corner that is not asked to move does not move**, and it is
    // answered before any meter runs. Metering a motion of zero would
    // classify every corner of a stationary body as unsolvable and say
    // "singular" about geometry that is nothing of the kind — the
    // refusal's own words have to stay true.
    let requested = at.iter().fold(T::zero(), |acc, p| acc + p.delta.norm());
    match decide("offset_together_request", Margin::of(requested), band) {
        Ok(Sign::Zero) => return Ok(here),
        Ok(_) => {}
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }
    solve_planar_corner(
        vertex,
        here,
        &at.iter().map(|p| (p.normal, p.c)).collect::<Vec<_>>(),
        arms,
        band,
    )
}

/// The corner solve itself, over `(n̂, c)` plane equations — shared with
/// the axial door, whose all-planar corners are exactly this problem.
///
/// The zero-move short-circuit is the CALLER's: only the caller knows
/// what motion was asked for, and metering a motion of zero here would
/// call a stationary body's every corner singular.
pub(crate) fn solve_planar_corner<T: Decide>(
    vertex: VertexKey,
    here: Point3<T>,
    at: &[(Vec3<T>, T)],
    arms: &[T],
    band: Band,
) -> Result<Point3<T>, ReplaceFaceError<T>> {
    let _ = here;
    let non_simple = |what: &'static str| ReplaceFaceError::TogetherCorner {
        vertex,
        planes: at.len(),
        what,
    };
    if at.len() < 3 {
        return Err(non_simple(
            "fewer than three distinct planes meet here, so no point is determined",
        ));
    }
    // The first well-conditioned triple in ORBIT order (the order
    // `faces_at_vertex` walks the vertex's own fan, which is what makes
    // the choice reproducible — not the face arena's).
    //
    // **The conditioning arm is the corner's OWN geometry, never the
    // request.** A triple product of unit normals is dimensionless and
    // the solve amplifies each plane's ε by `1/|det|`, so the question
    // is whether that induced displacement stays below a length at
    // which this is still a corner — and the lengths that answer it are
    // the EDGES that end here. Levering by the offset instead would
    // make the verdict depend on how far the body was asked to move: a
    // cube's corner would read "singular" at a small enough thickness
    // and a near-degenerate prism would build at a large enough one,
    // which is a statement about the request wearing the words of a
    // statement about the geometry.
    let mut solved = None;
    'triples: for (i, a) in at.iter().enumerate() {
        for (j, b) in at.iter().enumerate().skip(i + 1) {
            for c in at.iter().skip(j + 1) {
                let det = a.0.dot(b.0.cross(c.0));
                let mut resolvable = true;
                for &arm in arms {
                    match decide(
                        "offset_together_corner",
                        Margin::levered(det.abs(), arm),
                        band,
                    ) {
                        Ok(Sign::Positive) => {}
                        Ok(_) => {
                            resolvable = false;
                            break;
                        }
                        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                    }
                }
                if resolvable {
                    solved = Some(cramer(a, b, c, det));
                    break 'triples;
                }
            }
        }
    }
    let point = solved.ok_or_else(|| {
        non_simple(
            "no triple of the planes here resolves this corner against the edges that end at \
             it — they share a line or a plane",
        )
    })?;
    // Any further plane is VERIFIED, never assumed: a valence-past-3
    // corner whose planes do not concur has no offset point at all,
    // and guessing one is how a wrong body gets built.
    for p in at {
        let residual = p.0.dot(radius(point)) - p.1;
        match decide("offset_together_concurrence", Margin::of(residual), band) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(non_simple(
                    "the planes meeting here do not concur after the offset, so this corner has \
                     no offset point",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }
    Ok(point)
}

/// Cramer's rule on three plane equations with a known determinant.
fn cramer<T: Real>(a: &(Vec3<T>, T), b: &(Vec3<T>, T), c: &(Vec3<T>, T), det: T) -> Point3<T> {
    let v = (b.0.cross(c.0) * a.1 + c.0.cross(a.0) * b.1 + a.0.cross(b.0) * c.1) / det;
    Point3::new(v.x, v.y, v.z)
}

/// A point read as the vector from the origin — the form a plane
/// equation's `n · x` needs.
fn radius<T: Real>(p: Point3<T>) -> Vec3<T> {
    Vec3::new(p.x, p.y, p.z)
}

/// The chord length of every edge ending at a vertex — the lengths the
/// corner's conditioning is levered by (see [`solve_corner`]).
pub(crate) fn corner_arms<T: Real>(
    body: &Body<T>,
    vertex: VertexKey,
    here: Point3<T>,
) -> Result<Vec<T>, ReplaceFaceError<T>> {
    let Some(emanating) = body
        .get_vertex(vertex)
        .ok_or(ReplaceFaceError::Corrupt)?
        .emanating
    else {
        return Ok(Vec::new());
    };
    let orbit = body
        .vertex_orbit(emanating)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let mut out = Vec::new();
    for he in orbit {
        let far = body.half_edge_end(he).ok_or(ReplaceFaceError::Corrupt)?;
        let there = body
            .get_vertex(far)
            .and_then(|v| body.get_point(v.point).copied())
            .ok_or(ReplaceFaceError::Corrupt)?;
        out.push((there - here).norm());
    }
    Ok(out)
}

/// Every face incident to a vertex, in orbit order.
pub(crate) fn faces_at_vertex<T: Real>(
    body: &Body<T>,
    vertex: VertexKey,
) -> Result<Vec<FaceKey>, ReplaceFaceError<T>> {
    let Some(emanating) = body
        .get_vertex(vertex)
        .ok_or(ReplaceFaceError::Corrupt)?
        .emanating
    else {
        return Ok(Vec::new());
    };
    let orbit = body
        .vertex_orbit(emanating)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let mut out = Vec::new();
    for he in orbit {
        let face = body
            .get_loop(
                body.get_half_edge(he)
                    .ok_or(ReplaceFaceError::Corrupt)?
                    .parent_loop,
            )
            .ok_or(ReplaceFaceError::Corrupt)?
            .face;
        if !out.contains(&face) {
            out.push(face);
        }
    }
    Ok(out)
}

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
//! **Every face of every SOLID the moves touch must be planar and must
//! be in the moving set, and no solid may be touched in part.** Both
//! are checked and refused typed
//! ([`ReplaceFaceError::TogetherNonPlanar`],
//! [`ReplaceFaceError::TogetherPartialSet`]) — a planar face this door
//! moves has corners it cannot solve without every other chart at that
//! corner, and a partially moving solid has corners whose answer
//! depends on faces it was not told about.
//! Curved corners are the C5-table work that follows this unit; until
//! it lands they refuse where they always did.
//!
//! **The unit is the SOLID, not the body.** A corner is the meeting of
//! charts that belong to one solid — two solids share no vertex, no
//! edge and no face — so a body of several solids is several
//! independent corner problems, and this door solves the ones it was
//! given. Every entity it reads and every entity it writes lies on a
//! solid the moves name; the rest of the body is bitwise untouched.
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

use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, SolidKey, VertexKey};
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
    /// The signed offset along the chart's normal AT THESE FACES. On a
    /// cone's mirror nappe that is the negation of the stored `v > 0`
    /// field, and the door turns it (`crate::offset_nappe`) before the
    /// mint sees it.
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
/// `moves` names each chart and its signed distance. Every face of
/// every SOLID the moves touch must appear exactly once across them,
/// and no solid may be touched in part; a solid the moves do not name
/// is not offset and its geometry is not written.
///
/// **What it READS, as tightly as what it writes.** The scope is built
/// by walking the named solids' shells alone, so a solid the moves do
/// not name is not walked and its structure cannot refuse this call;
/// and the closing pcurve pass re-derives the rows of the scope's faces
/// alone ([`crate::pcurves::mint_pcurves_of`]) — a row outside it
/// belongs to an edge this door did not touch and stays as it was
/// found. The one whole-body read left is the closing tier-2 check on
/// the door's own clone, whose passes are arena-global by construction
/// (`Scope`'s docs).
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
    //
    // The scope is the SOLIDS the moves touch, and every face of each
    // of them must be in the set: a corner belongs to one solid, so a
    // solid named in part has corners whose answer depends on faces
    // the door was not told about, while a solid named not at all has
    // no corner this call can disturb.
    let scope = scope_of_moves(body, moves)?;
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
        if scope.holds_face(face) && !planes.iter().any(|(k, _)| *k == face) {
            return Err(ReplaceFaceError::TogetherPartialSet { face });
        }
    }
    let plane_of = |face: FaceKey| planes.iter().find(|(k, _)| *k == face).map(|(_, p)| p);

    // ---- Decide: every corner, before anything is written. ----
    let mut moved: Vec<(VertexKey, Point3<T>)> = Vec::new();
    for (vertex, _) in body.vertices() {
        if !scope.holds_vertex(vertex) {
            continue;
        }
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
        if !scope.holds_edge(edge) {
            continue;
        }
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
    // Every edge OF THE SCOPE was just re-described, so its stored
    // pcurve rows are stale — re-minted here for the same reason
    // `replace_faces_offset` re-mints, and before the tier-2 gate that
    // adopts the clone. A row outside the scope is not re-derived: the
    // door did not touch its edge, so the row is exactly as fresh as it
    // was found, which is what the `Maintains` posture claims.
    //
    // **KEPT although it is inert today, deliberately.** Only MINTING
    // charts carry pcurve rows and a plane is not one, so on every body
    // this door currently accepts the pass clears nothing and cannot go
    // red — which means it is also not covered by any row here, and
    // that is said rather than left for a reader to discover. It stays
    // because the door's scope gate is the only thing making it inert:
    // the curved corners that follow bring charts that DO mint, and a
    // door that re-describes every edge in the scope without re-minting
    // would be storing stale rows the moment they arrive. The census
    // posture (`Maintains`, by re-minting) is therefore honest now and
    // stays honest then.
    let minting = scope
        .faces_in_scope(&work)
        .ok_or(ReplaceFaceError::Corrupt)?;
    crate::pcurves::mint_pcurves_of(&mut work, &minting, tol)
        .map_err(|source| ReplaceFaceError::Pcurve { source })?;
    // Tier 2 over the WHOLE clone, and deliberately: tier 1's passes
    // are arena-global (ownership partitions, the edge <-> half-edge
    // bijection, orphan geometry, edge-adjacency shell coherence), so
    // there is no shell-subset reading of them that is the same check
    // narrowed rather than a different check. The clone differs from
    // the operand only inside the scope, so what this can report about
    // an out-of-scope solid is a defect the operand already had.
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
    at: &[(Vec3<T>, T)],
    arms: &[T],
    band: Band,
) -> Result<Point3<T>, ReplaceFaceError<T>> {
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
///
/// **The axial door keeps its own copy, levered by ARC LENGTH, and the
/// difference is load-bearing rather than a duplication to collapse.**
/// A chord is the arc length here because this door's bodies are
/// all-planar and a planar edge is a straight segment: no closed edge
/// exists for a chord to read zero on. The axial door's do — a chart
/// with ONE seam closes on itself — and a zero arm makes every meter
/// read `Zero` and call a perfectly transversal corner degenerate,
/// which is what it measured on the revolved tube.
fn corner_arms<T: Real>(
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

/// **The solids a simultaneous door works over.**
///
/// A corner is the meeting of charts that belong to ONE solid — two
/// solids of a body share no vertex, no edge and no face — so the
/// coherent unit for a simultaneous solve is the solid, not the body.
/// A scope names the solids the move set touches, and it is what both
/// doors SOLVE and WRITE over: the coverage gate asks that every face
/// of every named solid appear in the moves, the corner walk visits
/// the named solids' vertices, and the edge walk their edges, so no
/// entity outside the scope is offset or re-authored.
///
/// **Constructing one walks the named solids and nothing else.** The
/// maps hold the entities of the solids that have been walked, so a
/// shell, face, loop or half-edge that does not resolve refuses the
/// construction only when it belongs to a solid this scope was asked
/// about: a call about one solid is not refused for another solid's
/// corruption. The closing pcurve pass is scope-sized for the same
/// reason ([`crate::pcurves::mint_pcurves_of`], over
/// [`Scope::faces_in_scope`]).
///
/// The closure check each door ends with is NOT: `validate_closed` is
/// whole-body on the door's own clone, because tier 1's passes are
/// global by construction — ownership partitions, the edge ↔ half-edge
/// bijection, orphan geometry refcounts and edge-adjacency shell
/// coherence are statements about the whole arena, and evaluating them
/// over a shell subset reports every entity outside the subset as
/// unowned or orphaned. That is a read on a clone, and it is stated
/// here rather than left for a reader to find.
///
/// The scope is total on the entities a shell owns, lone vertices
/// included: an empty loop's vertex is reached through the loop's own
/// face rather than through an orbit it has no half-edge for.
#[derive(Clone)]
pub(crate) struct Scope {
    solids: Vec<SolidKey>,
    /// The solids whose shells have been walked into the maps. A
    /// superset of `solids` after a re-scope down, and what makes a
    /// re-scope UP a walk of the difference rather than a lie.
    built: Vec<SolidKey>,
    faces: SecondaryMap<FaceKey, SolidKey>,
    edges: SecondaryMap<EdgeKey, SolidKey>,
    vertices: SecondaryMap<VertexKey, SolidKey>,
}

impl Scope {
    /// The whole body: every solid it holds, in slot order.
    pub(crate) fn whole<T: Real>(body: &Body<T>) -> Option<Self> {
        let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
        Self::of_solids(body, &solids)
    }

    /// The named solids. `None` when one of THEIR shells, faces, loops
    /// or half-edges does not resolve; a solid not named is not walked
    /// and its state cannot refuse this construction.
    pub(crate) fn of_solids<T: Real>(body: &Body<T>, solids: &[SolidKey]) -> Option<Self> {
        let mut scope = Self {
            solids: solids.to_vec(),
            built: Vec::new(),
            faces: SecondaryMap::new(),
            edges: SecondaryMap::new(),
            vertices: SecondaryMap::new(),
        };
        scope.walk(body, solids)?;
        Some(scope)
    }

    /// Walks the shells of every solid of `solids` the maps do not
    /// already hold, extending them. Idempotent: a solid already in
    /// `built` costs nothing.
    fn walk<T: Real>(&mut self, body: &Body<T>, solids: &[SolidKey]) -> Option<()> {
        for &solid in solids {
            if self.built.contains(&solid) {
                continue;
            }
            for &shell in &body.get_solid(solid)?.shells {
                for &face in &body.get_shell(shell)?.faces {
                    self.faces.insert(face, solid);
                    let f = body.get_face(face)?;
                    for r#loop in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                        match body.get_loop(r#loop)?.boundary {
                            crate::entity::LoopBoundary::Empty { vertex } => {
                                self.vertices.insert(vertex, solid);
                            }
                            crate::entity::LoopBoundary::Cycle { first } => {
                                for he in body.loop_cycle(first)? {
                                    let half = body.get_half_edge(he)?;
                                    self.vertices.insert(half.start, solid);
                                    self.edges.insert(half.edge, solid);
                                }
                            }
                        }
                    }
                }
            }
            self.built.push(solid);
        }
        Some(())
    }

    /// Every face of every solid this scope names, in solid-then-shell
    /// order — the faces a door's closing pcurve pass is entitled to
    /// re-mint. `None` on a solid or shell of the scope that does not
    /// resolve.
    pub(crate) fn faces_in_scope<T: Real>(&self, body: &Body<T>) -> Option<Vec<FaceKey>> {
        let mut out: Vec<FaceKey> = Vec::new();
        for &solid in &self.solids {
            for &shell in &body.get_solid(solid)?.shells {
                out.extend_from_slice(&body.get_shell(shell)?.faces);
            }
        }
        Some(out)
    }

    /// The solid `face` belongs to as recorded by the walk: `Some` for
    /// any face of a solid this scope has walked — which for a
    /// [`Scope::whole`] is every face of the body, in or out of scope —
    /// and `None` for a face of a solid it never visited.
    pub(crate) fn solid_of(&self, face: FaceKey) -> Option<SolidKey> {
        self.faces.get(face).copied()
    }

    /// The same partition, re-aimed at `solids`, walking any of them
    /// the maps do not already hold. Re-scoping DOWN — the shell verb's
    /// case, a scope built over every solid and then narrowed — is a
    /// swap of one `Vec`; re-scoping up to a solid never walked extends
    /// the maps rather than answering `false` about entities that are
    /// in scope. `None` on a structural failure in what it had to walk.
    pub(crate) fn re_scope<T: Real>(&mut self, body: &Body<T>, solids: &[SolidKey]) -> Option<()> {
        self.walk(body, solids)?;
        self.solids.clear();
        self.solids.extend_from_slice(solids);
        Some(())
    }

    /// Is `face` on a solid this scope names?
    pub(crate) fn holds_face(&self, face: FaceKey) -> bool {
        self.faces
            .get(face)
            .is_some_and(|s| self.solids.contains(s))
    }

    /// Is `edge` on a solid this scope names?
    pub(crate) fn holds_edge(&self, edge: EdgeKey) -> bool {
        self.edges
            .get(edge)
            .is_some_and(|s| self.solids.contains(s))
    }

    /// Is `vertex` on a solid this scope names?
    pub(crate) fn holds_vertex(&self, vertex: VertexKey) -> bool {
        self.vertices
            .get(vertex)
            .is_some_and(|s| self.solids.contains(s))
    }
}

/// The scope a move set names: the solids its faces lie on, in the
/// order the moves first reach them.
pub(crate) fn scope_of_moves<T: Real>(
    body: &Body<T>,
    moves: &[ChartMove<T>],
) -> Result<Scope, ReplaceFaceError<T>> {
    // A face names its solid in two pointer hops, so the solids are
    // read off the moves themselves and the ONE structural walk that
    // follows covers those solids alone.
    let mut solids: Vec<SolidKey> = Vec::new();
    for m in moves {
        for &face in &m.faces {
            let shell = body
                .get_face(face)
                .ok_or(ReplaceFaceError::StaleFace { face })?
                .shell;
            let solid = body
                .get_shell(shell)
                .ok_or(ReplaceFaceError::Corrupt)?
                .solid;
            if !solids.contains(&solid) {
                solids.push(solid);
            }
        }
    }
    Scope::of_solids(body, &solids).ok_or(ReplaceFaceError::Corrupt)
}

#[cfg(test)]
mod scope_walks {
    //! The two walks SHELL-10 narrowed, pinned on a two-solid body:
    //! the scope's construction and the doors' closing pcurve pass.

    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{ChartMove, Scope, offset_planes_together, scope_of_moves};
    use crate::body::Body;
    use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, SolidKey};
    use crate::splitting::reassembly::quad_prism;
    use geom_core::{Affine3, Band, Point3, Tol, Vec3};

    const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

    /// Two unit boxes, ten apart, as two solids of one body — grafted
    /// through the public disjoint-graft door, so the operand is one a
    /// caller could hold.
    fn two_boxes() -> (Body<f64>, SolidKey, SolidKey) {
        let tol = Tol::witness();
        let mut body = quad_prism(&SQUARE, 1.0, tol);
        let first = body.solids().next().unwrap().0;
        let other = quad_prism(&SQUARE, 1.0, tol);
        let placed = crate::transform_rigid(
            &other,
            &Affine3::translation(Vec3::new(10.0, 0.0, 0.0)),
            tol,
        )
        .unwrap();
        let second = crate::graft_disjoint(&mut body, &placed, tol).unwrap();
        assert!(crate::validate::validate_closed(&body).is_ok());
        (body, first, second)
    }

    fn faces_of(body: &Body<f64>, solid: SolidKey) -> Vec<FaceKey> {
        body.faces()
            .filter(|(_, d)| body.get_shell(d.shell).unwrap().solid == solid)
            .map(|(k, _)| k)
            .collect()
    }

    /// Every chart of `solid`, as a move of `distance`. A zero distance
    /// is a legal move set — the corner solve answers an unmoved corner
    /// before any meter runs — and it is what these rows use, so that
    /// what they measure is the BOOKKEEPING around the solve.
    fn moves_of(body: &Body<f64>, solid: SolidKey, distance: f64) -> Vec<ChartMove<f64>> {
        let mut out: Vec<(crate::geometry::SurfaceKey, Vec<FaceKey>)> = Vec::new();
        for face in faces_of(body, solid) {
            let key = body.get_face(face).unwrap().surface;
            match out.iter_mut().find(|(k, _)| *k == key) {
                Some((_, v)) => v.push(face),
                None => out.push((key, vec![face])),
            }
        }
        out.into_iter()
            .map(|(_, faces)| ChartMove { faces, distance })
            .collect()
    }

    /// Breaks `solid`'s first loop cycle: one half-edge's `next` is
    /// re-pointed at a key no arena holds, so the walk is `Broken`.
    fn break_a_loop(body: &mut Body<f64>, solid: SolidKey) {
        let face = faces_of(body, solid)[0];
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("a box face bounds a cycle");
        };
        body.get_half_edge_mut(first).unwrap().next = HalfEdgeKey::default();
    }

    /// **The scope is the named solids' entities and no others.** The
    /// maps answer about the solid the moves name and are silent about
    /// the other — which is what `holds_*` already reported, and is now
    /// also what was walked.
    #[test]
    fn a_scope_holds_only_the_solids_it_names() {
        let (body, first, second) = two_boxes();
        let scope = Scope::of_solids(&body, &[first]).unwrap();
        for f in faces_of(&body, first) {
            assert_eq!(scope.solid_of(f), Some(first));
            assert!(scope.holds_face(f));
        }
        for f in faces_of(&body, second) {
            assert_eq!(scope.solid_of(f), None, "an unwalked solid's face");
            assert!(!scope.holds_face(f));
        }
        // The whole body, for contrast: one walk, both solids.
        let whole = Scope::whole(&body).unwrap();
        for f in faces_of(&body, second) {
            assert_eq!(whole.solid_of(f), Some(second));
        }
    }

    /// **An out-of-scope solid's structural corruption is not this
    /// call's to find.** `Scope::whole` — the walk the doors used to
    /// take, and still the shell verb's — refuses this body; the walk a
    /// move set naming the SOUND solid takes accepts it.
    ///
    /// **The door as a whole is not** — and the row stops at the scope
    /// deliberately. A structurally corrupt body is refused by two
    /// arena-global reads this unit did not narrow, both downstream of
    /// the scope: the asserting setters' tier-1 postcondition
    /// (`set_face_surface`, `set_edge_curve` — a panic, not a refusal,
    /// and compiled into this workspace's release profile too) and the
    /// closing tier-2 check. Driving the door here would measure those,
    /// not this. The narrowing is the pair of walks above.
    #[test]
    fn an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk() {
        let (mut body, first, second) = two_boxes();
        break_a_loop(&mut body, second);

        assert!(
            Scope::whole(&body).is_none(),
            "the whole-body walk still refuses a corrupt solid"
        );
        let moves = moves_of(&body, first, 0.0);
        let scope = scope_of_moves(&body, &moves).expect("the sound solid's scope builds");
        assert_eq!(scope.faces_in_scope(&body).unwrap(), faces_of(&body, first));
        for f in faces_of(&body, second) {
            assert!(!scope.holds_face(f));
        }
    }

    /// **An out-of-scope face the pcurve lane cannot chart is not this
    /// call's either** — the same shape as the row above, in the walk
    /// that IS fully narrowed, so the door builds.
    ///
    /// The out-of-scope box wears a cylinder on one face: structurally
    /// sound (tier 2 is clean), and a whole-body mint refuses it. The
    /// door's scope-sized pass never reaches it.
    #[test]
    fn an_out_of_scope_faces_unmintable_chart_does_not_refuse_the_door() {
        let tol = Tol::witness();
        let (mut body, first, second) = two_boxes();
        let victim = faces_of(&body, second)[0];
        body.set_face_surface(
            victim,
            crate::euler::FaceSurface::New(geom::Surface::Cylinder {
                origin: Point3::new(10.5, 0.5, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 0.5,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
        assert!(
            crate::validate::validate_closed(&body).is_ok(),
            "the operand is structurally sound; only its charting is not"
        );
        let mut whole = body.clone();
        assert!(
            crate::pcurves::mint_pcurves(&mut whole, tol).is_err(),
            "a whole-body mint refuses this body — the base's pass, and the base's refusal"
        );

        let moves = moves_of(&body, first, 0.0);
        let mut work = body.clone();
        offset_planes_together(&mut work, &moves, Band::linear(tol).unwrap(), tol)
            .expect("the door reads its scope, and its scope charts");
    }
}

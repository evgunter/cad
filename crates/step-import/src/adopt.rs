//! D7 adoption (Leg C, phases B–C): loop→face designation, surface
//! attachment with key sharing, `same_sense` honored verbatim, and the
//! certification of every edge's built description by the kernel's
//! own gates.
//!
//! # Phase B — faces
//!
//! Phase A realizes the file's loops exactly, but the transient `mef`
//! partition owns them arbitrarily. Normalization promotes every
//! ring-designated loop to its own face (`mfkrh`), then each file
//! face's rings are demoted onto its outer loop's face (`kfmrh`) —
//! pure topology, no certification, ending with exactly the file's
//! loop→face partition. Surfaces then attach: **bitwise-identical
//! surface records share one kernel surface key** (`FaceSurface::
//! Shared`), restoring the writer-side sharing that the per-face
//! record emission flattened — this is what makes a seam edge's
//! same-surface-both-sides state visible to the build. `same_sense`
//! is stated beside the surface exactly as read (the corpus's reversed
//! faces are deliberate kernel output — honored, never healed).
//!
//! # Phase C — the built edges (D7 stage 2)
//!
//! Every edge arrives with one description and its carrier already
//! built by the kernel's own constructions ([`crate::build`]): the
//! carrier is never the file's curve record, only interpreted from it
//! at the file band. Phase C states that one description and certifies
//! it once through [`topo::Body::set_edge_curve`], at the kernel's ε:
//!
//! - `Intersection` and `Iso` as built;
//! - `Tangent` through the must-carry rule ([`must_carry_rung`]): the
//!   rule's own refusal stands, and a join the rule answers with the
//!   pair rather than a carrier refuses not-yet-supported, naming the
//!   under-determined smooth join's lane;
//! - `Wrap` where one face of a periodic surface bounds both uses;
//! - `Chart` as the image in one surface's chart, declared by its mapped
//!   self-description (an arc rim beside a cap plane, or a curve in one
//!   surface on both sides).
//!
//! There is no ladder and no fallback: a description that does not
//! certify refuses as [`StepImportError::Adoption`] with its one typed
//! attempt. Certification pins endpoints, the mid-parameter witness,
//! transversality/tangency margins and seam-side residuals at ε.

use geom::Curve3;
use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, MappedSource, SurfacePair};
use geom_core::{Affine3, FileCoincidence, Point2, Point3};
use topo::construction::Construct;
use topo::construction::FaceSurface;
use topo::{Body, FaceKey, HalfEdgeKey, LoopKey};

use crate::assemble::Assembled;
use crate::build::{Description, Shell, Side, surface_sig};
use crate::error::{AdoptionAttempt, AdoptionCandidate, StepImportError};
use geom_core::Tol;

/// Runs phases B and C for one assembled solid (module docs), and
/// returns the body face per file face, in `CLOSED_SHELL` order.
pub(crate) fn finish(
    body: &mut Body<f64>,
    solid: &Shell<'_>,
    asm: &Assembled,
    tol: Tol,
    band: geom_core::Band,
    file: FileCoincidence,
) -> Result<Vec<FaceKey>, StepImportError> {
    let face_keys = designate_faces(body, solid, asm, file)?;
    rotate_loop_firsts(body, solid, asm, tol, file)?;
    attach_surfaces(body, solid, &face_keys, file)?;
    adopt_edges(body, solid, asm, tol, band, file)?;
    Ok(face_keys)
}

/// The body half-edge realizing target loop `l`'s first use.
fn first_he(asm: &Assembled, l: usize) -> HalfEdgeKey {
    asm.use_he[asm.target.loops[l][0]]
}

/// The body loop realizing target loop `l`.
fn body_loop(
    body: &Body<f64>,
    asm: &Assembled,
    l: usize,
    solid_id: u64,
) -> Result<LoopKey, StepImportError> {
    Ok(body
        .get_half_edge(first_he(asm, l))
        .ok_or(StepImportError::Topology {
            id: solid_id,
            what: "internal: a realized half-edge does not resolve in adoption",
        })?
        .parent_loop)
}

/// The face currently owning body loop `lk`.
fn loop_face(body: &Body<f64>, lk: LoopKey, solid_id: u64) -> Result<FaceKey, StepImportError> {
    Ok(body
        .get_loop(lk)
        .ok_or(StepImportError::Topology {
            id: solid_id,
            what: "internal: a realized loop does not resolve in adoption",
        })?
        .face)
}

/// The face currently owning target loop `l`'s body loop.
fn owning_face(
    body: &Body<f64>,
    asm: &Assembled,
    l: usize,
    solid_id: u64,
) -> Result<FaceKey, StepImportError> {
    body.face_of_half_edge(first_he(asm, l))
        .ok_or(StepImportError::Topology {
            id: solid_id,
            what: "internal: a realized half-edge's face does not resolve in adoption",
        })
}

/// Phase B's partition surgery: normalize (every loop an outer), then
/// demote each file face's rings onto its outer's face. Returns the
/// body face per file face, in `CLOSED_SHELL` order.
fn designate_faces(
    body: &mut Body<f64>,
    solid: &Shell<'_>,
    asm: &Assembled,
    file: FileCoincidence,
) -> Result<Vec<FaceKey>, StepImportError> {
    let op_err = |source: topo::EulerOpError| StepImportError::Assembly {
        id: solid.id,
        source: source.from_driver(),
        file,
    };
    // Normalize: promote every ring-designated realized loop.
    for l in 0..asm.target.loops.len() {
        let lk = body_loop(body, asm, l, solid.id)?;
        let f = loop_face(body, lk, solid.id)?;
        let outer = body
            .get_face(f)
            .ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: a loop's face does not resolve in adoption",
            })?
            .outer;
        if outer != lk {
            body.mfkrh_plug(lk, true).map_err(op_err)?;
        }
    }
    // Re-mint outer faces in FILE order (fixed-point discipline): the
    // writer walks `Shell::faces` stored order, so the imported
    // shell's face-list order must BE the file's `CLOSED_SHELL` order
    // or one adoption pass is not a fixed point of export. Phase A's
    // transient `mef` partition creates faces in insertion order; this
    // cycle parks each file face's outer loop as a ring of any other
    // live face (`kfmrh` — the parked face dies) and immediately
    // re-promotes it (`mfkrh` — a fresh face APPENDED to the shell's
    // list), so the surviving outer faces sit in exactly file order.
    // A single-loop shell's order is trivial and skips the cycle.
    if asm.target.loops.len() > 1 {
        for (outer_l, _) in &asm.target.face_loops {
            let lk = body_loop(body, asm, *outer_l, solid.id)?;
            let f_cur = loop_face(body, lk, solid.id)?;
            // Park anchor: the first realized loop living on another
            // face (deterministic scan; one always exists with ≥ 2
            // loops, since every face here is single-loop).
            let mut park = None;
            for other in 0..asm.target.loops.len() {
                let of = owning_face(body, asm, other, solid.id)?;
                if of != f_cur {
                    park = Some(of);
                    break;
                }
            }
            let park = park.ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: no parking face for the face-order re-mint",
            })?;
            body.kfmrh(park, f_cur).map_err(op_err)?;
            body.mfkrh_plug(lk, true).map_err(op_err)?;
        }
    }
    // Designate: each file face's rings become rings of its outer's
    // face (`kfmrh` demotes the ring's transient face wholesale),
    // appended in file ring order — the writer's ring emission order.
    let mut face_keys = Vec::with_capacity(asm.target.face_loops.len());
    for (outer_l, ring_ls) in &asm.target.face_loops {
        let f0 = owning_face(body, asm, *outer_l, solid.id)?;
        for &rl in ring_ls {
            let fr = owning_face(body, asm, rl, solid.id)?;
            body.kfmrh(f0, fr).map_err(op_err)?;
        }
        face_keys.push(f0);
    }
    Ok(face_keys)
}

/// Rotates every realized loop's `Cycle::first` to the half-edge of
/// its file loop's FIRST oriented edge (fixed-point discipline: the
/// writer emits a loop starting at `Cycle::first`, so the anchor must
/// be the file's). The rotation is a public-op identity: a scaffold
/// strut spliced immediately before the target half-edge, then `kev`
/// — whose documented re-anchor rule sets the survivor loop's
/// `Cycle::first` to the first survivor after the killed halves,
/// which is exactly the target.
fn rotate_loop_firsts(
    body: &mut Body<f64>,
    solid: &Shell<'_>,
    asm: &Assembled,
    tol: Tol,
    file: FileCoincidence,
) -> Result<(), StepImportError> {
    let op_err = |source: topo::EulerOpError| StepImportError::Assembly {
        id: solid.id,
        source: source.from_driver(),
        file,
    };
    for l in 0..asm.target.loops.len() {
        let t = first_he(asm, l);
        let current_first = match body
            .get_loop(body_loop(body, asm, l, solid.id)?)
            .ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: a realized loop does not resolve in rotation",
            })?
            .boundary
        {
            topo::LoopBoundary::Cycle { first } => first,
            topo::LoopBoundary::Empty { .. } => {
                return Err(StepImportError::Topology {
                    id: solid.id,
                    what: "internal: an empty loop survived to rotation",
                });
            }
        };
        if current_first == t {
            continue;
        }
        let p = body
            .half_edge_start_point(t)
            .ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: a realized vertex does not resolve in rotation",
            })?;
        let offset = Point3::new(p.x + 1.0, p.y, p.z);
        let strut = body
            .mev_line(
                topo::construction::MevSite::Fan { he1: t, he2: t },
                offset,
                tol,
            )
            .map_err(op_err)?;
        body.kev(strut.he_plus).map_err(op_err)?;
    }
    Ok(())
}

/// Phase B's attachment: surfaces (deduped to shared keys) and the
/// `same_sense` bit, in `CLOSED_SHELL` face order.
fn attach_surfaces(
    body: &mut Body<f64>,
    solid: &Shell<'_>,
    face_keys: &[FaceKey],
    file: FileCoincidence,
) -> Result<(), StepImportError> {
    let op_err = |source: topo::EulerOpError| StepImportError::Assembly {
        id: solid.id,
        source: source.from_driver(),
        file,
    };
    let mut seen: std::collections::BTreeMap<Vec<u64>, topo::SurfaceKey> =
        std::collections::BTreeMap::new();
    for (spec, &fk) in solid.faces.iter().zip(face_keys) {
        let sig = surface_sig(&spec.surface);
        let surface = match seen.get(&sig) {
            Some(&key) => FaceSurface::Shared {
                key,
                sense: spec.sense,
            },
            None => FaceSurface::New {
                surface: spec.surface.clone(),
                sense: spec.sense,
            },
        };
        let attached = body.set_face_surface(fk, surface).map_err(op_err)?;
        seen.insert(sig, attached);
    }
    Ok(())
}

/// Phase C: each edge's built description, keyed by the surfaces the
/// adoption attached, certified once at the run's ε with its built
/// carrier and interval (`crate::build`).
fn adopt_edges(
    body: &mut Body<f64>,
    solid: &Shell<'_>,
    asm: &Assembled,
    tol: Tol,
    band: geom_core::Band,
    file: FileCoincidence,
) -> Result<(), StepImportError> {
    for (&edge_id, edge) in &solid.edges {
        let (fwd, rev) = asm
            .target
            .edge_uses_of(edge_id)
            .ok_or(StepImportError::Topology {
                id: edge_id,
                what: "internal: an edge without realized uses in adoption",
            })?;
        let he_plus = asm.use_he[fwd];
        let he_minus = asm.use_he[rev];
        let resolve = |what| StepImportError::Topology { id: edge_id, what };
        let edge_key = body
            .get_half_edge(he_plus)
            .ok_or(resolve("internal: a realized half-edge does not resolve"))?
            .edge;
        let sides = topo::readback::edge_sides(body, edge_key).unwrap_or_else(|_| {
            unreachable!(
                "{he_plus:?}'s edge names {edge_key:?}, which does not resolve: every public \
                 door keeps the body tier-1-valid"
            )
        });
        // Assembly realizes a file edge's forward use as the edge's
        // `he_plus`; the build's sides are the file's uses.
        if (sides.plus.half_edge, sides.minus.half_edge) != (he_plus, he_minus) {
            return Err(resolve(
                "internal: a realized edge's two uses are not its he_plus and he_minus",
            ));
        }
        let (fs_plus, fs_minus) = sides.surfaces();
        let key = |side| match side {
            Side::Plus => fs_plus,
            Side::Minus => fs_minus,
        };
        let pair = SurfacePair::new(fs_plus, fs_minus);
        let interval = (edge.t0, edge.t1);
        let witness = edge.carrier.mid_point(edge.t0, edge.t1);
        let (candidate, mut description) = match &edge.description {
            Description::Intersection => (
                AdoptionCandidate::Intersection,
                EdgeDescriptionSpec::Intersection {
                    pair,
                    selector: geom_brep::CurveSelector::Witness(witness),
                },
            ),
            Description::Tangent => {
                let surface = |k| {
                    body.get_surface(k).unwrap_or_else(|| {
                        unreachable!("{k:?} names a face's surface `attach_surfaces` attached")
                    })
                };
                let reach = topo::wedge_reach(
                    body,
                    [sides.plus.face, sides.minus.face],
                    &edge.carrier,
                    interval,
                );
                match must_carry_rung(
                    [(fs_plus, surface(fs_plus)), (fs_minus, surface(fs_minus))],
                    &edge.carrier,
                    interval,
                    reach,
                    None,
                    band,
                    edge_key,
                ) {
                    Some(Ok(_)) => {
                        return Err(StepImportError::NotYetSupported {
                            entity: crate::BuiltEntity::Edge,
                            id: edge_id,
                            what: "a smooth join its two surfaces under-determine (the \
                                   relative normal curvature across it is not certified \
                                   apart from zero, so the tangency has no construction \
                                   of its own)"
                                .to_owned(),
                            lane: "the construction of an under-determined smooth join \
                                   (D2's conventional pair, C7; \
                                   `work/importgate/an-imported-under-determined-smooth-join-refuses-not-yet-supported.md`)",
                            file,
                        });
                    }
                    Some(Err(refusal)) => {
                        return Err(StepImportError::Adoption {
                            id: edge_id,
                            attempts: vec![AdoptionAttempt {
                                candidate: AdoptionCandidate::TangentIntersection,
                                refusal,
                            }],
                            file,
                        });
                    }
                    None => {}
                }
                (
                    AdoptionCandidate::TangentIntersection,
                    EdgeDescriptionSpec::TangentIntersection { pair, witness },
                )
            }
            Description::Iso { wall, image } => (
                AdoptionCandidate::IsoCurve,
                EdgeDescriptionSpec::chart_image(key(*wall), image.clone()),
            ),
            Description::Wrap => (AdoptionCandidate::Wrap, EdgeDescriptionSpec::wrap(fs_plus)),
            Description::Chart { chart } => {
                let p_start = solid.vertices[&edge.start];
                let p_end = solid.vertices[&edge.end];
                let spec = if fs_plus == fs_minus {
                    topo::rest_locus([fs_plus, fs_minus]).spec()
                } else {
                    EdgeDescriptionSpec::chart(key(*chart))
                };
                let declared = mapped_self_description(
                    &edge.carrier,
                    p_start,
                    p_end,
                    edge.t0,
                    edge.t1,
                    fs_plus != fs_minus,
                );
                (
                    AdoptionCandidate::MappedCurve,
                    match declared {
                        Some(mapped) => spec.declared_by(mapped),
                        None => spec,
                    },
                )
            }
        };
        // An edge both of whose uses bound one face is that face's wrap
        // edge (D1), whichever description holds it — a closed spline
        // wall's boundary column as much as an analytic generator — and
        // tier 3 refuses it described otherwise.
        let (f_plus, f_minus) = sides.faces();
        if f_plus == f_minus
            && let EdgeDescriptionSpec::Chart { wrap, .. } = &mut description
        {
            *wrap = true;
        }
        let attempt = EdgeCurveSpec {
            description,
            carrier: edge.carrier.clone(),
            param_start: edge.t0,
            param_end: edge.t1,
        };
        body.set_edge_curve(edge_key, attempt, tol)
            .map_err(|refusal| StepImportError::Adoption {
                id: edge_id,
                attempts: vec![AdoptionAttempt {
                    candidate,
                    refusal: refusal.from_driver(),
                }],
                file,
            })?;
    }
    Ok(())
}

/// **The must-carry rung** for an edge between two distinct surface
/// records: what the rule over the pair says the edge stores
/// ([`geom_brep::reask_pair`], levered at its faces' `reach` as tier 3
/// levers it, at the kernel's band). `Some(Ok)` is
/// the conventional pair the rule answers for an under-determined join,
/// which the import refuses not-yet-supported (that join's lane). `Some(Err)`
/// is the rule's typed refusal of an in-band or mixed join, which is
/// certifiable as neither. `None` where the rule demands an intrinsic
/// description, the built tangent carrier that is then certified, or where
/// the rule reads no pair ([`geom_brep::must_carry_reads`]: a spline record
/// on either side).
fn must_carry_rung(
    surfaces: [(geom_brep::SurfaceKey, &Surface<f64>); 2],
    carrier: &Curve3<f64>,
    interval: (f64, f64),
    reach: geom_core::Lever<f64>,
    declared: Option<MappedCurve<f64>>,
    band: geom_core::Band,
    edge: topo::EdgeKey,
) -> Option<Result<EdgeDescriptionSpec<f64>, topo::EulerOpError>> {
    let [(_, a), (_, b)] = surfaces;
    if !geom_brep::must_carry_reads(a, b) {
        return None;
    }
    match geom_brep::reask_pair(surfaces, carrier, interval, reach, declared, band) {
        Ok(pair @ EdgeDescriptionSpec::OnPair { .. }) => Some(Ok(pair)),
        Ok(_) => None,
        Err(geom_brep::PairRefusal::InBand(escalation)) => {
            Some(Err(topo::EulerOpError::RestJoinUndecided {
                edge,
                diag: escalation.diag(),
            }))
        }
        Err(geom_brep::PairRefusal::Mixed(stations)) => {
            Some(Err(topo::EulerOpError::RestJoinMixed { edge, stations }))
        }
    }
}

/// The conventional self-description for carriers the surfaces
/// under-determine: a line is its start point's trajectory under the
/// translation to its end, a circular arc its start point's
/// trajectory under the rotation through the derived angle — honest
/// pushforward data (the same shapes native constructions store),
/// exact for the parsed carrier except a Nurbs-adjacent line rim's
/// (below). Ellipse/NURBS carriers have no
/// mapped form (none exists in `geom_brep::MappedCurve`'s vocabulary,
/// and no exported body puts one inside a single surface); the edge
/// then refuses typed.
///
/// A **Nurbs-adjacent LINE rim** (`nurbs_rim`, M7-3 item 4) the
/// `Intersection` rung did not certify — that rung, offered first,
/// takes a straight rim whose chart image is straight — takes the
/// `PlacedSegment` shape instead of `ExtrudedPoint`, for two reasons
/// with one root: `PlacedSegment { Line }` is the description CLASS
/// the native loft builder stores for exactly this edge class, and it
/// is the shape the NURBS chart's pcurve derivation accepts
/// (`topo`'s `nurbs_iso_derive` — its `ExtrudedPoint` mint from a
/// LINE carrier would refuse there, the measured mint blocker; the
/// alternative, an `ExtrudedPoint` arm in `nurbs_iso_derive` itself,
/// would widen the kernel's own certification vocabulary to spare
/// the importer a description it can synthesize exactly). Same class
/// and same certification surface — but NOT the same payload bits:
/// the native builder's segment lives in sketch coordinates under the
/// cap placement, while the synthesized one is the carrier's own
/// interval on its own axis (`a = (t0, 0)`, `b = (t1, 0)` under a
/// rigid frame whose x-axis is the carrier direction normalized) — so
/// the point at `t` is `origin + normalize(dir)·t`, not the carrier's
/// `origin + dir·t` bit for bit: for the unit-within-rounding direction
/// the reader keeps, the two differ by about `2.8e-16·|t|`, an
/// equivalent parameterization of the same locus up to that (the sketch plane never crosses the STEP
/// wire, so the native payload is not recoverable; review F5 pinned
/// the honest statement to class level). Arc rims stay
/// `RevolvedPoint` in both cases — on a rational wall nothing mints
/// (the native rational body's own state), and a non-rational wall
/// has no arc rims to mint (its profile was a polyline). Arc rims on
/// a rational wall are held on the wall's boundary by the build's
/// `step_rim_on_wall` check before this description is stated.
fn mapped_self_description(
    carrier: &Curve3<f64>,
    p_start: Point3<f64>,
    p_end: Point3<f64>,
    t0: f64,
    t1: f64,
    nurbs_rim: bool,
) -> Option<MappedCurve<f64>> {
    match carrier {
        Curve3::Line(geom::Line { origin, dir }) if nurbs_rim => {
            line_frame(*origin, *dir).map(|place| {
                MappedCurve::whole(MappedSource::PlacedSegment {
                    segment: geom_brep::SketchSegment::Line {
                        a: Point2::new(t0, 0.0),
                        b: Point2::new(t1, 0.0),
                    },
                    place,
                })
            })
        }
        Curve3::Line(_) => Some(MappedCurve::whole(MappedSource::ExtrudedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p_start - Point3::origin()),
            vec: p_end - p_start,
        })),
        Curve3::Circle(geom::Circle {
            frame:
                geom::Frame {
                    origin: center,
                    axis,
                    ..
                },
            ..
        }) => Some(MappedCurve::whole(MappedSource::RevolvedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p_start - Point3::origin()),
            axis: geom::Line {
                origin: *center,
                dir: *axis,
            },
            angle: t1 - t0,
        })),
        Curve3::Ellipse(..) | Curve3::Spiric { .. } | Curve3::Nurbs(_) | Curve3::Periodic(_) => {
            None
        }
    }
}

/// A rigid frame whose x-axis is the line direction normalized, placed
/// at the line's origin — the `PlacedSegment` placement for a Nurbs-adjacent
/// rim ([`mapped_self_description`]). The y/z columns complete an
/// orthonormal frame (they never move a segment point — every sketch
/// point has `y = 0` — but a placement claims rigidity as conventional
/// data, so honest perpendiculars are minted).
///
/// The frame is [`geom_core::RigidMotion::gram_schmidt`] of the
/// direction and the coordinate axis of its SMALLEST component
/// magnitude, which is at least `1/√3` from parallel to a unit-ish
/// direction, so the residual is well conditioned. A direction is no
/// distance for the file's band to read, so both lengths are decided at
/// the least band: a frame that still cannot be completed (a vanishing
/// or non-finite direction) answers `None`, the conventional rung is
/// WITHHELD, and the edge refuses typed — never a silent degenerate
/// placement.
fn line_frame(origin: Point3<f64>, dir: geom_core::Vec3<f64>) -> Option<Affine3<f64>> {
    let (ax, ay, az) = (dir.x.abs(), dir.y.abs(), dir.z.abs());
    let candidate = if ax <= ay && ax <= az {
        geom_core::Vec3::new(1.0, 0.0, 0.0)
    } else if ay <= az {
        geom_core::Vec3::new(0.0, 1.0, 0.0)
    } else {
        geom_core::Vec3::new(0.0, 0.0, 1.0)
    };
    geom_core::RigidMotion::gram_schmidt(
        origin,
        dir,
        candidate,
        crate::interpret::STEP_RIM_LINE_FRAME,
        geom_core::Band::least(),
    )
    .ok()
    .map(geom_core::RigidMotion::to_affine)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]
mod tests {
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;

    use super::*;

    /// A degree-1×2 loft-wall-shaped surface whose control net is a
    /// function of `dx` — two nets differing in one control coordinate.
    fn wall(dx: f64) -> Surface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = vec![
            Point3::new(dx, 0.0, 0.0),
            Point3::new(dx, 0.0, 1.0),
            Point3::new(dx, 0.0, 2.0),
            Point3::new(dx + 1.0, 0.0, 0.0),
            Point3::new(dx + 1.0, 0.0, 1.0),
            Point3::new(dx + 1.0, 0.0, 2.0),
        ];
        let payload = NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap();
        Surface::Nurbs(std::sync::Arc::new(payload))
    }

    /// **The M7-3 surface_sig pin (spec §1 item 2).** Two DISTINCT
    /// NURBS walls must get distinct signatures — the tag-only arm
    /// (`vec![5u64]`) would silently share one surface key across
    /// every wall of a body (the silent-wrong-body class) — while a
    /// bitwise-identical record must still share (the dedup that
    /// restores writer-side key sharing).
    #[test]
    fn distinct_nurbs_walls_get_distinct_signatures() {
        let a = surface_sig(&wall(0.0));
        let b = surface_sig(&wall(1.0));
        assert_ne!(a, b, "distinct NURBS walls must not share a surface key");
        assert_eq!(
            a,
            surface_sig(&wall(0.0)),
            "bitwise-identical NURBS records must share one key"
        );
    }

    /// The signature is a function of every stated field family:
    /// weights and knots move it too, not just control points (a
    /// rational wall differing only in weights is a different
    /// surface).
    #[test]
    fn weights_and_knots_reach_the_signature() {
        let base = wall(0.0);
        let Surface::Nurbs(payload) = &base else {
            unreachable!()
        };
        let mut weights = payload.weights().to_vec();
        weights[1] = 2.0;
        let reweighted = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(
                payload.knots_u_f64_interim().clone(),
                payload.knots_v_f64_interim().clone(),
                payload.control().to_vec(),
                weights,
            )
            .unwrap(),
        ));
        assert_ne!(surface_sig(&base), surface_sig(&reweighted));
        let kv3 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let refined = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(
                payload.knots_u_f64_interim().clone(),
                kv3,
                vec![Point3::new(0.0, 0.0, 0.0); 8],
                vec![1.0; 8],
            )
            .unwrap(),
        ));
        assert_ne!(surface_sig(&base), surface_sig(&refined));
    }

    /// REVIEW PROBE (V1): transposed nets — shape (2,3) at degrees
    /// (1,2) vs shape (3,2) at degrees (2,1) with the identical
    /// control multiset must not collide.
    #[test]
    fn probe_transposed_nets_distinct() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let pts = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 2.0),
        ];
        let a = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku.clone(), kv.clone(), pts.to_vec(), vec![1.0; 6]).unwrap(),
        ));
        let transposed = vec![pts[0], pts[3], pts[1], pts[4], pts[2], pts[5]];
        let b = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(kv, ku, transposed, vec![1.0; 6]).unwrap(),
        ));
        assert_ne!(
            surface_sig(&a),
            surface_sig(&b),
            "transposed nets with identical multisets must not collide"
        );
    }

    /// REVIEW PROBE (V1): a single interior knot moved, every count
    /// equal.
    #[test]
    fn probe_single_knot_value_distinct() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kva = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kvb = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.625, 1.0, 1.0, 1.0], 2).unwrap();
        let pts = vec![Point3::new(0.0, 0.0, 0.0); 8];
        let a = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku.clone(), kva, pts.clone(), vec![1.0; 8]).unwrap(),
        ));
        let b = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(ku, kvb, pts, vec![1.0; 8]).unwrap(),
        ));
        assert_ne!(
            surface_sig(&a),
            surface_sig(&b),
            "one knot value apart must not collide"
        );
    }

    /// REVIEW PROBE (V1): u/v knot vectors swapped between two square
    /// nets with equal lengths and degrees — the concatenated knot
    /// stream carries the same values in a different order.
    #[test]
    fn probe_uv_knot_swap_distinct() {
        let k01 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let k02 = KnotVector::clamped(vec![0.0, 0.0, 2.0, 2.0], 1).unwrap();
        let pts = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        let a = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(k01.clone(), k02.clone(), pts.clone(), vec![1.0; 4]).unwrap(),
        ));
        let b = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(k02, k01, pts, vec![1.0; 4]).unwrap(),
        ));
        assert_ne!(
            surface_sig(&a),
            surface_sig(&b),
            "swapped u/v knot vectors must not collide"
        );
    }

    /// **The must-carry rung offers only what the rule asks for.** A unit
    /// cylinder resting on the plane `y = 0` along the `z` axis, its wedge
    /// levered at `reach`, so the join's sagitta is `reach²/2`: definitely
    /// positive offers nothing (the intrinsic rungs carry the tangency),
    /// definitely zero offers the pair, in band refuses typed rather than
    /// offering it. A crossing offers nothing, and neither does a pair
    /// with a spline record on one side, where the rule would answer in
    /// band for want of an implicit form.
    #[test]
    fn the_must_carry_rung_offers_only_what_the_rule_asks_for() {
        let tol = Tol::witness();
        let band = geom_core::Band::linear(tol).unwrap();
        let mut keys = slotmap::SlotMap::<geom_brep::SurfaceKey, ()>::with_key();
        let (kp, kc) = (keys.insert(()), keys.insert(()));
        let plane = |normal| Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal,
            u_ref: geom_core::Vec3::new(0.0, 0.0, 1.0),
        };
        let cylinder = Surface::Cylinder {
            origin: Point3::new(0.0, 1.0, 0.0),
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        };
        let floor = plane(geom_core::Vec3::new(0.0, 1.0, 0.0));
        let carrier = Curve3::line(
            Point3::new(0.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
        );
        let rung = |a: &Surface<f64>, b: &Surface<f64>, sagitta: f64| {
            let reach = geom_brep::points_lever_from(
                Point3::new(0.0, 0.0, 0.5),
                [Point3::new((2.0 * sagitta).sqrt(), 0.0, 0.5)],
            );
            must_carry_rung(
                [(kp, a), (kc, b)],
                &carrier,
                (0.0, 1.0),
                reach,
                None,
                band,
                topo::EdgeKey::default(),
            )
        };
        let middle = (band.zero() * band.escalate()).sqrt();
        assert!(
            rung(&floor, &cylinder, 0.125).is_none(),
            "a jet-determinate join is not offered the pair"
        );
        assert!(
            matches!(
                rung(&floor, &cylinder, band.zero() / 100.0),
                Some(Ok(EdgeDescriptionSpec::OnPair { pair, declared: None }))
                    if pair == SurfacePair::new(kp, kc)
            ),
            "an under-determined join is offered the pair"
        );
        assert!(
            matches!(
                rung(&floor, &cylinder, middle),
                Some(Err(topo::EulerOpError::RestJoinUndecided { .. }))
            ),
            "an in-band join refuses typed: {:?}",
            rung(&floor, &cylinder, middle)
        );
        let crossing = plane(geom_core::Vec3::new(1.0, 0.0, 0.0));
        assert!(
            rung(&crossing, &cylinder, band.zero() / 100.0).is_none(),
            "a crossing is not offered the pair"
        );
        assert!(
            rung(&wall(0.0), &cylinder, band.zero() / 100.0).is_none(),
            "a spline record is not read by the rule"
        );
    }
}

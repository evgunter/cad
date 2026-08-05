//! D7 adoption (Leg C, phases B–C): loop→face designation, surface
//! attachment with key sharing, `same_sense` honored verbatim, and the
//! **edge-adoption ladder** — every edge's intensional description
//! rebuilt and certified by the kernel's own gates.
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
//! same-surface-both-sides state visible to the ladder. `same_sense`
//! lands via [`topo::Body::set_face_sense`] exactly as read (the
//! corpus's reversed faces are deliberate kernel output — honored,
//! never healed).
//!
//! # Phase C — the edge ladder (D7 stage 2)
//!
//! For each edge, candidates in preference order (intrinsic before
//! conventional — the kernel's own at-rest preference), each attempt a
//! full [`topo::Body::set_edge_curve`] certification of the **parsed
//! carrier** over the derived interval:
//!
//! - distinct adjacent surfaces: `Intersection` (transverse), then
//!   `TangentIntersection` (the fillet-trimline contact class);
//! - one surface on both sides: `Seam` (periodic charts only), then
//!   the conventional `MappedCurve` self-description (a line as its
//!   own extrusion trajectory, a circular arc as its own revolution
//!   trajectory — honest data for loci the surfaces under-determine,
//!   e.g. a sphere's π-copy meridian or a profile edge inside a
//!   plane).
//!
//! The first candidate that certifies wins; if none does, the typed
//! [`StepImportError::Adoption`] carries every attempt and its typed
//! refusal (structured, for future remedy flows). A wrong guess
//! cannot survive: certification pins endpoints, the mid-parameter
//! witness, transversality/tangency margins, and seam-side residuals
//! at the kernel's own ε.

use geom_brep::{EdgeCurveSpec, EdgeGeometry, MappedCurve};
use geom_core::{Affine3, Point2, Point3};
use geom_curves::Curve3;
use geom_surfaces::Surface;
use topo::{Body, FaceKey, FaceSurface, LoopKey};

use crate::assemble::Assembled;
use crate::entities::SolidSpec;
use crate::error::{AdoptionAttempt, AdoptionCandidate, StepImportError};

/// Runs phases B and C for one assembled solid (module docs).
pub(crate) fn finish(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    asm: &Assembled,
) -> Result<(), StepImportError> {
    let face_keys = designate_faces(body, solid, asm)?;
    rotate_loop_firsts(body, solid, asm)?;
    attach_surfaces(body, solid, &face_keys)?;
    adopt_edges(body, solid, asm)
}

/// The body loop realizing target loop `l`.
fn body_loop(
    body: &Body<f64>,
    asm: &Assembled,
    l: usize,
    solid_id: u64,
) -> Result<LoopKey, StepImportError> {
    let first = asm.target.loops[l][0];
    Ok(body
        .get_half_edge(asm.use_he[first])
        .ok_or(StepImportError::Topology {
            id: solid_id,
            what: "internal: a realized half-edge does not resolve in adoption",
        })?
        .parent_loop)
}

/// The face currently owning a body loop.
fn owning_face(body: &Body<f64>, l: LoopKey, solid_id: u64) -> Result<FaceKey, StepImportError> {
    Ok(body
        .get_loop(l)
        .ok_or(StepImportError::Topology {
            id: solid_id,
            what: "internal: a realized loop does not resolve in adoption",
        })?
        .face)
}

/// Phase B's partition surgery: normalize (every loop an outer), then
/// demote each file face's rings onto its outer's face. Returns the
/// body face per file face, in `CLOSED_SHELL` order.
fn designate_faces(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    asm: &Assembled,
) -> Result<Vec<FaceKey>, StepImportError> {
    let op_err = |source| StepImportError::Assembly {
        id: solid.id,
        source,
    };
    // Normalize: promote every ring-designated realized loop.
    for l in 0..asm.target.loops.len() {
        let lk = body_loop(body, asm, l, solid.id)?;
        let f = owning_face(body, lk, solid.id)?;
        let outer = body
            .get_face(f)
            .ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: a loop's face does not resolve in adoption",
            })?
            .outer;
        if outer != lk {
            body.mfkrh_plug(lk).map_err(op_err)?;
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
            let f_cur = owning_face(body, lk, solid.id)?;
            // Park anchor: the first realized loop living on another
            // face (deterministic scan; one always exists with ≥ 2
            // loops, since every face here is single-loop).
            let mut park = None;
            for other in 0..asm.target.loops.len() {
                let ok = body_loop(body, asm, other, solid.id)?;
                let of = owning_face(body, ok, solid.id)?;
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
            body.mfkrh_plug(lk).map_err(op_err)?;
        }
    }
    // Designate: each file face's rings become rings of its outer's
    // face (`kfmrh` demotes the ring's transient face wholesale),
    // appended in file ring order — the writer's ring emission order.
    let mut face_keys = Vec::with_capacity(asm.target.face_loops.len());
    for (outer_l, ring_ls) in &asm.target.face_loops {
        let outer_lk = body_loop(body, asm, *outer_l, solid.id)?;
        let f0 = owning_face(body, outer_lk, solid.id)?;
        for &rl in ring_ls {
            let ring_lk = body_loop(body, asm, rl, solid.id)?;
            let fr = owning_face(body, ring_lk, solid.id)?;
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
    solid: &SolidSpec,
    asm: &Assembled,
) -> Result<(), StepImportError> {
    let op_err = |source| StepImportError::Assembly {
        id: solid.id,
        source,
    };
    for seq in &asm.target.loops {
        let t = asm.use_he[seq[0]];
        let he = body.get_half_edge(t).ok_or(StepImportError::Topology {
            id: solid.id,
            what: "internal: a realized half-edge does not resolve in rotation",
        })?;
        let current_first = match body
            .get_loop(he.parent_loop)
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
        let start = he.start;
        let p = *body
            .get_vertex(start)
            .and_then(|v| body.get_point(v.point))
            .ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: a realized vertex does not resolve in rotation",
            })?;
        let offset = Point3::new(p.x + 1.0, p.y, p.z);
        let strut = body
            .mev_line(topo::MevSite::Fan { he1: t, he2: t }, offset)
            .map_err(op_err)?;
        body.kev(strut.he_plus).map_err(op_err)?;
    }
    Ok(())
}

/// A surface's exact structural signature: variant tag + field bits,
/// the dedup key that restores writer-side surface-key sharing
/// (bitwise identity — an exact structural comparison, no ε).
fn surface_sig(surface: &Surface<f64>) -> Vec<u64> {
    let p = |p: Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    let v = |v: geom_core::Vec3<f64>| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()];
    match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => [&[0u64][..], &p(origin), &v(normal), &v(u_ref)].concat(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => [
            &[1u64][..],
            &p(origin),
            &v(axis),
            &[radius.to_bits()],
            &v(u_ref),
        ]
        .concat(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => [
            &[2u64][..],
            &p(apex),
            &v(axis),
            &[half_angle.to_bits()],
            &v(u_ref),
        ]
        .concat(),
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => [
            &[3u64][..],
            &p(center),
            &[radius.to_bits()],
            &v(axis),
            &v(u_ref),
        ]
        .concat(),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => [
            &[4u64][..],
            &p(center),
            &v(axis),
            &[major_radius.to_bits(), minor_radius.to_bits()],
            &v(u_ref),
        ]
        .concat(),
        // The full structural payload: degrees, knot values, control
        // bits, weight bits (M7-3). A tag-only arm here was the
        // silent-wrong-body trap: once NURBS surfaces parse, every
        // wall in a body would share ONE surface key — four distinct
        // walls collapsing to one surface, exactly the class of wrong
        // the dedup exists to prevent — so the signature hashes every
        // field the record states, like the analytic arms above.
        // Counts lead each variable-length section so two payloads
        // with different shapes cannot alias by concatenation.
        Surface::Nurbs(ref payload) => {
            let (nu, nv) = payload.control_counts();
            let mut sig = vec![
                5u64,
                payload.knots_u().degree() as u64,
                payload.knots_v().degree() as u64,
                payload.knots_u().knots().len() as u64,
                payload.knots_v().knots().len() as u64,
                nu as u64,
                nv as u64,
            ];
            sig.extend(payload.knots_u().knots().iter().map(|k| k.to_bits()));
            sig.extend(payload.knots_v().knots().iter().map(|k| k.to_bits()));
            sig.extend(payload.control().iter().flat_map(|q| p(*q)));
            sig.extend(payload.weights().iter().map(|w| w.to_bits()));
            sig
        }
    }
}

/// Phase B's attachment: surfaces (deduped to shared keys) and the
/// `same_sense` bit, in `CLOSED_SHELL` face order.
fn attach_surfaces(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    face_keys: &[FaceKey],
) -> Result<(), StepImportError> {
    let op_err = |source| StepImportError::Assembly {
        id: solid.id,
        source,
    };
    let mut seen: std::collections::BTreeMap<Vec<u64>, topo::SurfaceKey> =
        std::collections::BTreeMap::new();
    for (spec, &fk) in solid.faces.iter().zip(face_keys) {
        let sig = surface_sig(&spec.surface);
        let attached = match seen.get(&sig) {
            Some(&key) => body
                .set_face_surface(fk, FaceSurface::Shared(key))
                .map_err(op_err)?,
            None => body
                .set_face_surface(fk, FaceSurface::New(spec.surface.clone()))
                .map_err(op_err)?,
        };
        seen.insert(sig, attached);
        body.set_face_sense(fk, spec.sense).map_err(op_err)?;
    }
    Ok(())
}

/// Phase C: the per-edge adoption ladder (module docs).
fn adopt_edges(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    asm: &Assembled,
) -> Result<(), StepImportError> {
    for (&edge_id, spec) in &solid.edges {
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
        let face_surface = |body: &Body<f64>, he| -> Result<topo::SurfaceKey, StepImportError> {
            let loop_key = body
                .get_half_edge(he)
                .ok_or(resolve("internal: a realized half-edge does not resolve"))?
                .parent_loop;
            let face = body
                .get_loop(loop_key)
                .ok_or(resolve("internal: a realized loop does not resolve"))?
                .face;
            Ok(body
                .get_face(face)
                .ok_or(resolve("internal: a realized face does not resolve"))?
                .surface)
        };
        let fs_plus = face_surface(body, he_plus)?;
        let fs_minus = face_surface(body, he_minus)?;
        let witness = spec.carrier.eval((spec.t0 + spec.t1) / 2.0);
        let p_start = solid.vertices[&spec.start];
        let p_end = solid.vertices[&spec.end];

        // The candidate descriptions, in preference order (module
        // docs: intrinsic before conventional).
        let mut candidates: Vec<(AdoptionCandidate, EdgeGeometry<f64>)> = Vec::new();
        let mut conventional = true;
        let mut nurbs_rim = false;
        if fs_plus != fs_minus {
            // The IsoCurve rung (M7-3): a NURBS-carried edge between
            // two described NURBS walls is the loft/sweep wall–wall
            // seam class — the carrier the writer emitted IS one
            // wall's `u ∈ {0, 1}` boundary column
            // (`geom_brep::boundary_iso_u`, a control-net copy), so
            // the match is BITWISE, sound own-corpus (the printer
            // round-trips bits; an ε_in-tolerant match is an
            // M7-2-style widening, not this rung's). Offered FIRST:
            // it is the description class the native builder stores
            // (the at-rest preference is the native body's state),
            // and the intrinsic rungs below cannot certify a Nurbs
            // resolved surface at all (`geom_brep` check 1 refuses
            // typed) — they stay on the ladder so a non-matching
            // edge still reports every attempt. `u = 0` arms lead:
            // each native seam is minted as its forward wall's
            // `u = 0` boundary, so the first certifying candidate
            // reproduces the native description exactly. `v0`/`v1`
            // are the carrier's own derived interval — its full knot
            // domain, which the bitwise match pins to the wall's v
            // domain ([0, 1] for every exported wall).
            if let Curve3::Nurbs(ref nurbs_carrier) = spec.carrier {
                for end in [false, true] {
                    for wall in [fs_plus, fs_minus] {
                        if let Some(Surface::Nurbs(wp)) = body.get_surface(wall)
                            && !wp.is_placeholder()
                            && let Ok(iso) = geom_brep::boundary_iso_u(wp.as_ref(), end)
                            && bitwise_iso_match(nurbs_carrier, &iso)
                        {
                            candidates.push((
                                AdoptionCandidate::IsoCurve,
                                EdgeGeometry::IsoCurve {
                                    surface: wall,
                                    u: if end { 1.0 } else { 0.0 },
                                    v0: spec.t0,
                                    v1: spec.t1,
                                },
                            ));
                        }
                    }
                }
            }
            candidates.push((
                AdoptionCandidate::Intersection,
                EdgeGeometry::Intersection {
                    s1: fs_plus,
                    s2: fs_minus,
                    witness,
                },
            ));
            candidates.push((
                AdoptionCandidate::TangentIntersection,
                EdgeGeometry::TangentIntersection {
                    s1: fs_plus,
                    s2: fs_minus,
                    witness,
                },
            ));
            // The conventional rung, for distinct surface RECORDS that
            // describe **the same locus** (M7-2): a boolean union's
            // re-tiled flat side arrives as two coplanar PLANE records
            // with different origins, so the importer's bitwise key
            // sharing cannot merge them and the edge between them is
            // neither a transverse intersection nor a tangency with a
            // second-order margin — the kernel's own gate says so in
            // its refusal ("the surfaces under-determine the locus
            // there — a G2 conventional join keeps its MappedCurve
            // description BY THIS PREDICATE, D2's split"), and this
            // rung follows that instruction.
            //
            // The condition is COINCIDENCE, tested on the surfaces'
            // own fields at the ambient tolerance — not "the intrinsic
            // rungs failed". A conventional self-description certifies
            // against nothing but itself, so offering it wherever an
            // intersection refuses would make the ladder vacuous: a
            // face pointed at the WRONG surface would sail through.
            // Under coincidence the two records describe one surface,
            // which is exactly the same-surface case the `else` branch
            // below already treats conventionally.
            //
            // Coincidence of the SURFACES is only half the gate. A
            // conventional self-description certifies against nothing
            // but itself, so once the rung is offered the carrier is
            // never checked against anything again — and a carrier that
            // wanders off the coincident locus adopts cleanly, with the
            // wrong volume, caught only by a later whole-body tier-3
            // pass a caller need not run (and `import_step`'s own error
            // contract promises it will not need to). The rung
            // therefore requires BOTH: the two records describe one
            // locus, and this edge's carrier lies on it.
            // **The Nurbs-adjacency exemption (M7-3 item 4)** — the
            // cap-plane × NURBS-wall rim class, exempt BY KIND
            // (mirroring tier-3 check 4's flip-B exemption,
            // `topo::validate`): coincidence is an implicit-form
            // question and a NURBS wall has no implicit form, so the
            // gate above can never answer for this pair — while the
            // class itself is exactly the conventional one (the
            // wall's `v ∈ {0, 1}` iso IS the placed profile segment;
            // the loft builder's own rims carry `PlacedSegment`).
            // What keeps the rung honest here is not this gate but
            // the pcurve mint the import runs unconditionally: on a
            // non-rational wall every rim's chart image is derived
            // and CERTIFIED against the wall (`nurbs_iso_derive`'s
            // side pick + the iso lane), so a carrier that wanders
            // off the wall boundary fails the import loudly. A
            // rational wall mints nothing — exactly the native
            // rational body's state, whose tier-3 refusal the import
            // preserves (item 5's Arm B).
            nurbs_rim = nurbs_plane_pair(body.get_surface(fs_plus), body.get_surface(fs_minus));
            conventional = nurbs_rim
                || (coincident_surfaces(body.get_surface(fs_plus), body.get_surface(fs_minus))
                    && carrier_on_surface(
                        body.get_surface(fs_plus),
                        &spec.carrier,
                        spec.t0,
                        spec.t1,
                    ));
        } else {
            let periodic = body.get_surface(fs_plus).is_some_and(|s| {
                matches!(
                    s,
                    Surface::Cylinder { .. }
                        | Surface::Cone { .. }
                        | Surface::Sphere { .. }
                        | Surface::Torus { .. }
                )
            });
            if periodic {
                candidates.push((
                    AdoptionCandidate::Seam,
                    EdgeGeometry::Seam { surface: fs_plus },
                ));
            }
        }
        if conventional
            && let Some(mapped) =
                mapped_self_description(&spec.carrier, p_start, p_end, spec.t0, spec.t1, nurbs_rim)
        {
            candidates.push((
                AdoptionCandidate::MappedCurve,
                EdgeGeometry::MappedCurve(mapped),
            ));
        }

        let mut attempts = Vec::new();
        let mut adopted = false;
        for (candidate, description) in candidates {
            let attempt = EdgeCurveSpec {
                description,
                carrier: spec.carrier.clone(),
                param_start: spec.t0,
                param_end: spec.t1,
            };
            match body.set_edge_curve(edge_key, attempt) {
                Ok(_) => {
                    adopted = true;
                    break;
                }
                Err(refusal) => attempts.push(AdoptionAttempt { candidate, refusal }),
            }
        }
        if !adopted {
            return Err(StepImportError::Adoption {
                id: edge_id,
                attempts,
            });
        }
    }
    Ok(())
}

/// The conventional self-description for carriers the surfaces
/// under-determine: a line is its start point's trajectory under the
/// translation to its end, a circular arc its start point's
/// trajectory under the rotation through the derived angle — honest
/// pushforward data (the same shapes native constructions store),
/// exact for the parsed carrier. Ellipse/NURBS carriers have no
/// mapped form (none exists in `geom_brep::MappedCurve`'s vocabulary,
/// and no exported body puts one inside a single surface); the ladder
/// then reports every refusal typed.
///
/// A **Nurbs-adjacent LINE rim** (`nurbs_rim`, M7-3 item 4) takes the
/// `PlacedSegment` shape instead of `ExtrudedPoint`, for two reasons
/// with one root: `PlacedSegment { Line }` is the description the
/// native loft builder stores for exactly this edge class (the
/// imported body lands in the native description state), and it is
/// the shape the NURBS chart's pcurve derivation accepts
/// (`topo`'s `nurbs_iso_derive` — its `ExtrudedPoint` mint from a
/// LINE carrier would refuse there, the measured mint blocker; the
/// alternative, an `ExtrudedPoint` arm in `nurbs_iso_derive` itself,
/// would widen the kernel's own certification vocabulary to spare
/// the importer a description it can synthesize exactly). The
/// segment is the carrier's own interval on its own axis
/// (`a = (t0, 0)`, `b = (t1, 0)` under a rigid frame whose x-axis is
/// the carrier direction), so the described locus is the carrier's
/// bit for bit up to the placement arithmetic. Arc rims stay
/// `RevolvedPoint` in both cases — on a rational wall nothing mints
/// (the native rational body's own state), and a non-rational wall
/// has no arc rims to mint (its profile was a polyline).
fn mapped_self_description(
    carrier: &Curve3<f64>,
    p_start: Point3<f64>,
    p_end: Point3<f64>,
    t0: f64,
    t1: f64,
    nurbs_rim: bool,
) -> Option<MappedCurve<f64>> {
    match carrier {
        Curve3::Line { origin, dir } if nurbs_rim => Some(MappedCurve::PlacedSegment {
            segment: geom_brep::SketchSegment::Line {
                a: Point2::new(t0, 0.0),
                b: Point2::new(t1, 0.0),
            },
            place: line_frame(*origin, *dir),
        }),
        Curve3::Line { .. } => Some(MappedCurve::ExtrudedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p_start - Point3::origin()),
            vec: p_end - p_start,
        }),
        Curve3::Circle { center, axis, .. } => Some(MappedCurve::RevolvedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p_start - Point3::origin()),
            axis_origin: *center,
            axis_dir: *axis,
            angle: t1 - t0,
        }),
        Curve3::Ellipse { .. } | Curve3::Nurbs(_) => None,
    }
}

/// One side a described (non-placeholder) NURBS wall, the other a
/// plane — the cap-rim adjacency the conventional rung's exemption
/// names (its call site's comment). Any other pairing answers
/// `false`: the exemption is exactly as wide as the class it serves.
fn nurbs_plane_pair(s1: Option<&Surface<f64>>, s2: Option<&Surface<f64>>) -> bool {
    let described_nurbs = |s: Option<&Surface<f64>>| {
        matches!(s, Some(Surface::Nurbs(p)) if !p.is_placeholder())
    };
    let plane = |s: Option<&Surface<f64>>| matches!(s, Some(Surface::Plane { .. }));
    (described_nurbs(s1) && plane(s2)) || (plane(s1) && described_nurbs(s2))
}

/// A rigid frame whose x-axis is the (unit) line direction, placed at
/// the line's origin — the `PlacedSegment` placement for a
/// Nurbs-adjacent rim ([`mapped_self_description`]). The y/z columns
/// complete an orthonormal frame (they never move a segment point —
/// every sketch point has `y = 0` — but a placement claims rigidity
/// as conventional data, so honest perpendiculars are minted, the
/// same first-coordinate-axis selection the placement reader uses).
fn line_frame(origin: Point3<f64>, dir: geom_core::Vec3<f64>) -> Affine3<f64> {
    let candidate = if dir.x.abs() < 1.0 {
        geom_core::Vec3::new(1.0, 0.0, 0.0)
    } else {
        geom_core::Vec3::new(0.0, 1.0, 0.0)
    };
    let perpendicular = candidate - dir * dir.dot(candidate);
    let y = perpendicular / perpendicular.norm();
    let z = dir.cross(y);
    Affine3::from_parts(geom_core::Mat3::from_cols(dir, y, z), origin - Point3::origin())
}

/// Bitwise equality of a parsed NURBS carrier against a wall's
/// boundary iso-curve — degree, knot values, control points and
/// weights all to the bit (the IsoCurve rung's match; its call site's
/// soundness comment). Weight lengths follow control lengths on both
/// sides by construction (`NurbsCurve3::new` validates them equal).
fn bitwise_iso_match(carrier: &geom_curves::NurbsCurve3<f64>, iso: &geom_curves::NurbsCurve3<f64>) -> bool {
    let bits3 = |p: &Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    carrier.knots().degree() == iso.knots().degree()
        && carrier.knots().knots().len() == iso.knots().knots().len()
        && carrier
            .knots()
            .knots()
            .iter()
            .zip(iso.knots().knots())
            .all(|(a, b)| a.to_bits() == b.to_bits())
        && carrier.control().len() == iso.control().len()
        && carrier
            .control()
            .iter()
            .zip(iso.control())
            .all(|(a, b)| bits3(a) == bits3(b))
        && carrier
            .weights()
            .iter()
            .zip(iso.weights())
            .all(|(a, b)| a.to_bits() == b.to_bits())
}

/// Whether two distinct surface records describe **the same locus** at
/// the ambient tolerance (module docs' conventional rung).
///
/// Only the plane case is decided here, because only the plane case is
/// measured: a boolean union re-tiles a flat side into several coplanar
/// `PLANE` records with different origins. Any other pair answers
/// `false` — the conventional rung is then not offered, and the edge
/// must earn an intrinsic description or refuse typed, which is the
/// conservative direction.
fn coincident_surfaces(s1: Option<&Surface<f64>>, s2: Option<&Surface<f64>>) -> bool {
    let eps = geom_core::Tolerance::get().eps;
    match (s1, s2) {
        (
            Some(&Surface::Plane {
                origin: o1,
                normal: n1,
                ..
            }),
            Some(&Surface::Plane {
                origin: o2,
                normal: n2,
                ..
            }),
        ) => {
            // Parallel normals (either orientation — a face's material
            // side is its `sense`, not its surface record's normal),
            // and one plane's origin on the other.
            n1.cross(n2).norm() <= eps && (o2 - o1).dot(n1).abs() <= eps
        }
        _ => false,
    }
}

/// Whether `carrier` lies ON `surface` over `[t0, t1]` — the locus
/// check the conventional rung owes (its call site's comment).
///
/// Sampled through the same door the kernel's own certification uses:
/// [`geom_brep::CERT_SAMPLES`] uniform parameters, endpoints included,
/// each residual measured against the ambient tolerance — the same
/// count and the same ε the `set_edge_curve` gates spend on their own
/// residuals, so an edge that passes here has not passed a laxer test
/// than the intrinsic rungs face.
///
/// Only the plane case decides, matching [`coincident_surfaces`]: any
/// other surface answers `false`, which withholds the rung and leaves
/// the edge to earn an intrinsic description or refuse typed.
fn carrier_on_surface(
    surface: Option<&Surface<f64>>,
    carrier: &Curve3<f64>,
    t0: f64,
    t1: f64,
) -> bool {
    let Some(&Surface::Plane { origin, normal, .. }) = surface else {
        return false;
    };
    let eps = geom_core::Tolerance::get().eps;
    let n = normal.norm();
    if !(n.is_finite() && n > 0.0) {
        return false;
    }
    (0..geom_brep::CERT_SAMPLES).all(|i| {
        let f = f64::from(i) / f64::from(geom_brep::CERT_SAMPLES - 1);
        let p = carrier.eval(t0 + (t1 - t0) * f);
        ((p - origin).dot(normal) / n).abs() <= eps
    })
}

#[cfg(test)]
mod tests {
    use geom_core::spline::KnotVector;
    use geom_surfaces::NurbsSurface;

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
                payload.knots_u().clone(),
                payload.knots_v().clone(),
                payload.control().to_vec(),
                weights,
            )
            .unwrap(),
        ));
        assert_ne!(surface_sig(&base), surface_sig(&reweighted));
        let kv3 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let refined = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(
                payload.knots_u().clone(),
                kv3,
                vec![Point3::new(0.0, 0.0, 0.0); 8],
                vec![1.0; 8],
            )
            .unwrap(),
        ));
        assert_ne!(surface_sig(&base), surface_sig(&refined));
    }
}

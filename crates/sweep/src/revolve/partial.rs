//! The partial (θ < 2π) revolve: an extrude-shaped wedge sweep
//! (Mäntylä §12.3's pattern under our CCW convention) with plane wedge
//! caps and the axis-contact special classes handled in-line — on-axis
//! vertices get no strut (rotation fixes them), on-axis segments get
//! no wall (the edge is shared by the two caps). See the module docs
//! for the ratified case split.
//!
//! Phase order (fixed, D9): start lamina (outer chain + closing `mef`
//! carrying the start cap's Newell plane), holes (bridge `mev` +
//! `kemr` + chain + closing `mef` + same-shell `kfmrh` — extrude's
//! shape), per-loop sweep (struts at off-axis vertices, walls for
//! off-axis segments, latitude-join classification), end cap plane,
//! then the upgrade pass (cap–wall meridians, cap–cap axis edges).

use geom_brep::newell_plane;
use geom_core::{Affine3, Band, Decide, Point3, Sign};
use topo::{Body, EdgeKey, FaceKey, FaceSurface, MefSite, MevSite};

use super::axis::{AxisFrame, LoopClasses, WallClass};
use super::chain::build_chain;
use super::surfaces::{strut_spec, wall_surface};
use super::upgrade::{describe_at_rest, upgrade_intersection};
use super::{RevolveError, Revolved, RevolvedKind, SweptSeg, WALL_COSURFACE};
use crate::swept::{cap_points, cosurface, face_surface_key, placed_segment_spec, turn_axis};
use geom_core::Tol;

/// Builds the wedge solid (file docs). `reverse` is the already-decided
/// sign class of θ (`true` ⇔ θ definitely positive — module docs'
/// winding convention); it selects the θ-signed rim-carrier axis
/// structurally (no re-inspection of θ).
pub(super) fn build_partial<T: Decide>(
    frame: &AxisFrame<T>,
    loops: &[Vec<SweptSeg<T>>],
    classes: &[LoopClasses<T>],
    theta: T,
    reverse: bool,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let place = frame.place;
    let rot = Affine3::rotation_about_axis(frame.o3, frame.a3, theta);
    let place_end = rot * place;
    let n_end = rot.linear * frame.n3;
    let axis_c = turn_axis(
        if reverse {
            Sign::Positive
        } else {
            Sign::Negative
        },
        frame.a3,
    );

    // World points: start chain, and end chain (pinned vertices are
    // fixed by the rotation — their end point IS the start point, so
    // no drift enters the shared entities).
    let points: Vec<Vec<Point3<T>>> = loops
        .iter()
        .map(|segs| segs.iter().map(|s| frame.world(s.a)).collect())
        .collect();
    let rpoints: Vec<Vec<Point3<T>>> = loops
        .iter()
        .zip(classes)
        .map(|(segs, cls)| {
            segs.iter()
                .enumerate()
                .map(|(j, s)| {
                    if cls.verts[j].pinned {
                        frame.world(s.a)
                    } else {
                        rot.transform_point(frame.world(s.a))
                    }
                })
                .collect()
        })
        .collect();

    // ---- Phase 1: start lamina (outer loop; extrude's shape). ----
    let outer = &loops[0];
    let qs = &points[0];
    let mut body = Body::<T>::new();
    let seed = body.mvfs(qs[0])?;
    // Start cap plane: the mef face's loop runs the chain reversed;
    // first point kept, rest reversed (extrude's bottom-cap order).
    // Derived from the sketch data alone — it reads no entity and
    // mints none — so the closing surface can be in hand before the
    // chain that will carry it.
    let forward = cap_points(outer, qs, place);
    let mut start_order: Vec<Point3<T>> = Vec::with_capacity(forward.len());
    if let Some(&p0) = forward.first() {
        start_order.push(p0);
    }
    for &p in forward.iter().skip(1).rev() {
        start_order.push(p);
    }
    let start_plane =
        newell_plane(&start_order, band).map_err(|source| RevolveError::CapPlane { source })?;
    let start = build_chain(
        &mut body,
        frame,
        seed.r#loop,
        seed.vertex,
        outer,
        qs,
        FaceSurface::New(start_plane),
        tol,
    )?;
    let end_face = seed.face;
    let start_face = start.face;
    let start_surface = face_surface_key(&body, start_face)?;
    let mut bases = Vec::with_capacity(loops.len());
    bases.push(start.hes);
    let mut verts = Vec::with_capacity(loops.len());
    verts.push(start.verts);

    // ---- Phase 2: holes (rings in the seed face + kfmrh into the
    // start cap; extrude's shape — hole vertices are never on-axis for
    // a validated profile, so the generic sweep handles them). ----
    let anchor = bases[0][0];
    for (li, segs) in loops.iter().enumerate().skip(1) {
        let hq = &points[li];
        let bridge = body.mev_line(
            MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            hq[0],
            tol,
        )?;
        let ring = body.kemr(bridge.he_plus, bridge.he_minus)?.ring;
        let hole = build_chain(
            &mut body,
            frame,
            ring,
            bridge.vertex,
            segs,
            hq,
            FaceSurface::Shared(start_surface),
            tol,
        )?;
        body.kfmrh(start_face, hole.face)?;
        bases.push(hole.hes);
        // The chain's vertices are recorded for EVERY loop, holes
        // included, though a validated profile's hole vertices are
        // never on-axis (the phase note above), so the pole export
        // reads none of these today. The uniformity is the point: the
        // assembly stays one loop keyed on `pinned`, which is the real
        // discriminator, and a hole that ever reached the axis would
        // export its pole rather than silently lose it.
        verts.push(hole.verts);
    }

    // ---- Phase 3: sweep each loop (struts, walls, latitude joins).
    // ----
    let mut walls_all: Vec<Vec<Option<FaceKey>>> = Vec::with_capacity(loops.len());
    let mut rims_all: Vec<Vec<Option<EdgeKey>>> = Vec::with_capacity(loops.len());
    let mut tops_all: Vec<Vec<Option<EdgeKey>>> = Vec::with_capacity(loops.len());
    for (li, (segs, hes)) in loops.iter().zip(&bases).enumerate() {
        let swept = sweep_loop(
            &mut body,
            li,
            segs,
            &classes[li],
            hes,
            &points[li],
            &rpoints[li],
            frame,
            theta,
            axis_c,
            place_end,
            n_end,
            band,
            tol,
        )?;
        walls_all.push(swept.faces);
        rims_all.push(swept.rims);
        tops_all.push(swept.tops);
    }

    // ---- Phase 4: the swept face survives as the end cap. ----
    let raised = cap_points(&loops[0], &rpoints[0], place_end);
    let end_plane =
        newell_plane(&raised, band).map_err(|source| RevolveError::CapPlane { source })?;
    let end_surface = body.set_face_surface(end_face, FaceSurface::New(end_plane))?;

    // ---- Phase 5: rim upgrades (both cap planes exist): loops in
    // canonical order, segments in swept order; per walled segment the
    // start rim then the end rim; on-axis segments classify the two
    // caps against each other (module docs). ----
    finish_partial(
        &mut body,
        loops,
        classes,
        &bases,
        &walls_all,
        &tops_all,
        start_surface,
        end_surface,
        band,
        tol,
    )?;

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "revolve (partial) postcondition: result is not tier-2 valid (kernel bug)",
    );

    // ---- Assembly: canonical-indexed key bundle. ----
    let mut walls_c = Vec::with_capacity(loops.len());
    let mut rims_c = Vec::with_capacity(loops.len());
    let mut start_mer = Vec::with_capacity(loops.len());
    let mut end_mer = Vec::with_capacity(loops.len());
    let mut poles_c = Vec::with_capacity(loops.len());
    for (li, segs) in loops.iter().enumerate() {
        let n = segs.len();
        let mut wc = vec![None; n];
        let mut rc = vec![None; n];
        let mut sm = vec![None; n];
        let mut em = vec![None; n];
        let mut pc = vec![None; n];
        for (j, s) in segs.iter().enumerate() {
            wc[s.canonical_segment] = walls_all[li][j];
            rc[s.canonical_vertex] = rims_all[li][j];
            // A pinned vertex is fixed by the rotation, so its start-
            // chain vertex IS the end chain's: the pole.
            if classes[li].verts[j].pinned {
                pc[s.canonical_vertex] = Some(verts[li][j]);
            }
            let bottom = he_edge(&body, bases[li][j])?;
            sm[s.canonical_segment] = Some(bottom);
            em[s.canonical_segment] = Some(tops_all[li][j].unwrap_or(bottom));
        }
        walls_c.push(wc);
        rims_c.push(rc);
        poles_c.push(pc);
        // Meridian chains are total per segment (`Option` only bridges
        // the fill loop above).
        start_mer.push(sm.into_iter().flatten().collect());
        end_mer.push(em.into_iter().flatten().collect());
    }

    Ok(Revolved {
        body,
        solid: seed.solid,
        shell: seed.shell,
        cavities: Vec::new(),
        walls: walls_c,
        rims: rims_c,
        poles: poles_c,
        kind: RevolvedKind::Partial {
            start_cap: start_face,
            end_cap: end_face,
            start_meridians: start_mer,
            end_meridians: end_mer,
        },
    })
}

/// An half-edge's edge key (total).
pub(super) fn he_edge<T: Decide>(
    body: &Body<T>,
    he: topo::HalfEdgeKey,
) -> Result<EdgeKey, RevolveError> {
    Ok(body
        .get_half_edge(he)
        .ok_or(topo::EulerOpError::StaleKey {
            key: topo::EntityId::HalfEdge(he),
        })?
        .edge)
}

/// The rim-upgrade pass (phase 5): per walled segment the start-chain
/// then end-chain meridian upgrades to `Intersection { cap, wall }`;
/// per on-axis segment the shared edge classifies the two caps against
/// each other — `Intersection { start, end }` when definitely
/// transverse (θ ≠ π), conventional when definitely smooth (θ = π).
#[allow(clippy::too_many_arguments)] // one internal call site.
fn finish_partial<T: Decide>(
    body: &mut Body<T>,
    loops: &[Vec<SweptSeg<T>>],
    classes: &[LoopClasses<T>],
    bases: &[Vec<topo::HalfEdgeKey>],
    walls_all: &[Vec<Option<FaceKey>>],
    tops_all: &[Vec<Option<EdgeKey>>],
    start_surface: topo::SurfaceKey,
    end_surface: topo::SurfaceKey,
    band: Band,
    tol: Tol,
) -> Result<(), RevolveError> {
    for (li, segs) in loops.iter().enumerate() {
        let n = segs.len();
        for j in 0..n {
            let segment_index = segs[j].canonical_segment;
            let sliver = |source| RevolveError::SliverRim {
                loop_index: li,
                segment_index,
                source,
            };
            let bottom = he_edge(body, bases[li][j])?;
            match walls_all[li][j] {
                Some(wall_face) => {
                    let wall = face_surface_key(body, wall_face)?;
                    upgrade_intersection(body, bottom, start_surface, wall, band, sliver, tol)?;
                    if let Some(top) = tops_all[li][j] {
                        upgrade_intersection(body, top, end_surface, wall, band, sliver, tol)?;
                    }
                }
                None => {
                    debug_assert!(matches!(classes[li].walls[j], WallClass::OnAxis));
                    upgrade_intersection(
                        body,
                        bottom,
                        start_surface,
                        end_surface,
                        band,
                        sliver,
                        tol,
                    )?;
                }
            }
        }
    }
    Ok(())
}

/// One loop's sweep products, swept-indexed (`None` = on-axis class).
pub(super) struct LoopSwept {
    pub(super) faces: Vec<Option<FaceKey>>,
    pub(super) rims: Vec<Option<EdgeKey>>,
    pub(super) tops: Vec<Option<EdgeKey>>,
}

/// Sweeps one loop of the seed face: struts at off-axis vertices,
/// walls for off-axis segments (site addressing per the pinned-run
/// derivation — see M2-LOG PR 5), latitude-join classification.
#[allow(clippy::too_many_arguments)] // one internal call site; the
// arguments are the sweep's fixed context.
pub(super) fn sweep_loop<T: Decide>(
    body: &mut Body<T>,
    loop_index: usize,
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    hes: &[topo::HalfEdgeKey],
    qs: &[Point3<T>],
    rq: &[Point3<T>],
    frame: &AxisFrame<T>,
    theta: T,
    axis_c: geom_core::Vec3<T>,
    place_end: Affine3<T>,
    n_end: geom_core::Vec3<T>,
    band: Band,
    tol: Tol,
) -> Result<LoopSwept, RevolveError> {
    let n = segs.len();
    let walled: Vec<bool> = cls.walls.iter().map(|w| w.kind().is_some()).collect();

    // Cosurface run structure, decided up front for the whole loop —
    // including the wrap pair — before any wall is minted (the PR 4
    // SHOULD-1 lesson). Pairs across a pinned (on-axis) segment are
    // structurally false: the run is broken by the axis contact.
    let mut pair = Vec::with_capacity(n);
    for j in 0..n {
        let p = (j + n - 1) % n;
        let linked = walled[p]
            && walled[j]
            && cosurface(&segs[p], &segs[j], WALL_COSURFACE, band).map_err(|source| {
                RevolveError::CosurfaceEscalated {
                    loop_index,
                    vertex_index: segs[j].canonical_vertex,
                    source,
                }
            })?;
        pair.push(linked);
    }

    // Struts: one latitude arc per off-axis vertex, traversal order.
    let mut struts: Vec<Option<topo::MevCreated>> = Vec::with_capacity(n);
    for j in 0..n {
        if cls.verts[j].pinned {
            struts.push(None);
            continue;
        }
        let m = body.mev(
            MevSite::Fan {
                he1: hes[j],
                he2: hes[j],
            },
            rq[j],
            strut_spec(segs[j].a, cls.verts[j].r, qs[j], frame, theta, axis_c),
            tol,
        )?;
        struts.push(Some(m));
    }

    // Walls: per off-axis segment, ascending; the new edge is the end
    // (rotated) chain edge, the new face the wall. Site derivation:
    // he1 = the strut minus at v_j (or hes[j] when v_j is pinned);
    // he2 = the strut minus at v_{j+1} (or hes[j+1] when pinned; at
    // the wrap, the first wall's plus half if segment 0 was walled,
    // else hes[0]).
    let mut faces: Vec<Option<FaceKey>> = Vec::with_capacity(n);
    let mut tops: Vec<Option<EdgeKey>> = Vec::with_capacity(n);
    let mut first_top: Option<topo::HalfEdgeKey> = None;
    for j in 0..n {
        let Some(kind) = cls.walls[j].kind() else {
            faces.push(None);
            tops.push(None);
            continue;
        };
        let he1 = match &struts[j] {
            Some(s) => s.he_minus,
            None => hes[j],
        };
        let next = (j + 1) % n;
        let he2 = if j + 1 < n {
            match &struts[next] {
                Some(s) => s.he_minus,
                None => hes[next],
            }
        } else {
            match first_top {
                Some(top) => top,
                // Segment 0 is on-axis (or this loop's only wall is
                // segment n−1): the wrap representative is hes[0],
                // still in the seed loop.
                None => hes[0],
            }
        };
        // Sharing shape (PR 4 SHOULD-1's precompute): `pair[j]` shares
        // the previous wall's key; a run reaching segment 0 through
        // the wrap shares the first wall's key. The `None` fallbacks
        // are unreachable (pair implies walled) and mint an
        // identical-by-construction surface defensively rather than
        // erroring on a phantom state.
        let shared = if pair[j] && j > 0 {
            faces[(j + n - 1) % n]
        } else if j > 0 && pair[0] && ((j + 1)..n).all(|k| pair[k]) {
            faces[0]
        } else {
            None
        };
        let surface = match shared {
            Some(f) => FaceSurface::Shared(face_surface_key(body, f)?),
            None => FaceSurface::New(wall_surface(kind, &segs[j], frame)),
        };
        let mef = body.mef(
            MefSite::Chords { he1, he2 },
            placed_segment_spec(&segs[j], place_end, n_end, rq[j], rq[next]),
            surface,
            tol,
        )?;
        if j == 0 {
            first_top = Some(mef.he_plus);
        }
        // The honest orientation bit (M5 S11): a wall whose material
        // lies against its revolution surface's chart normal (bore
        // cylinder, inward cone, under-side plane annulus, concave
        // sphere/torus band) is attached `sense: false` — classified
        // from the profile's stored winding structure
        // (`WallClass::Wall::sense`); attached here because `mef`
        // cannot know the material side.
        if cls.walls[j].sense() == Some(false) {
            body.set_face_sense(mef.face, false)?;
        }
        faces.push(Some(mef.face));
        tops.push(Some(mef.edge));
    }

    // Latitude joins: strut j joins the walls of segments j−1 and j
    // (both exist whenever the strut does — a vertex flanked by an
    // on-axis segment is pinned). Identical keys are conventional
    // structurally; distinct keys classify at the carrier midpoint.
    let mut rims: Vec<Option<EdgeKey>> = Vec::with_capacity(n);
    for j in 0..n {
        let Some(strut) = &struts[j] else {
            rims.push(None);
            continue;
        };
        rims.push(Some(strut.edge));
        let f_prev = faces[(j + n - 1) % n];
        let f_next = faces[j];
        let (Some(fp), Some(fnx)) = (f_prev, f_next) else {
            continue; // unreachable by the pinned-adjacency argument
        };
        let k_prev = face_surface_key(body, fp)?;
        let k_next = face_surface_key(body, fnx)?;
        if k_prev == k_next {
            describe_at_rest(body, strut.edge, k_prev, tol)?;
            continue;
        }
        let vertex_index = segs[j].canonical_vertex;
        upgrade_intersection(
            body,
            strut.edge,
            k_prev,
            k_next,
            band,
            |source| RevolveError::SliverJoin {
                loop_index,
                vertex_index,
                source,
            },
            tol,
        )?;
    }

    Ok(LoopSwept { faces, rims, tops })
}

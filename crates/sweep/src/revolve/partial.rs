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
//! shape), per-loop sweep (struts at off-axis vertices, one wall per
//! off-axis segment, latitude-join classification), end cap plane, then
//! the upgrade pass (cap–wall meridians, cap–cap axis edges). Every
//! phase builds the loops with their runs collapsed ([`Collapsed`]), so
//! a station inside a run has no entity.

use geom_core::{Affine3, Band, Decide, Point3, Sign};
use topo::{Body, EdgeKey, FaceKey, FaceSurface, MevSite};

use super::axis::{AxisFrame, LoopClasses, WallClass};
use super::chain::build_chain;
use super::runs::{Collapsed, collapse_runs};
use super::surfaces::{revolved_strut_spec, wall_surface};
use super::turn::{TurnEnds, sweep_turn};
use super::upgrade::upgrade_intersection;
use super::{RevolveError, Revolved, RevolvedKind, SweptSeg};
use crate::swept::{
    CapEnd, Placing, cap_plane, cap_points, face_surface_key, placed_segment_spec, turn_axis,
};
use geom_core::Tol;

/// Builds the wedge solid (file docs). `reverse` is the already-decided
/// sign class of θ (`true` ⇔ θ definitely positive — module docs'
/// winding convention); it selects the θ-signed rim-carrier axis
/// structurally (no re-inspection of θ).
pub(super) fn build_partial<T: Decide + topo::AtRestPolicy>(
    frame: &AxisFrame<T>,
    loops: &[Vec<SweptSeg<T>>],
    classes: &[LoopClasses<T>],
    theta: T,
    reverse: bool,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let place = frame.place;
    let far = Placing::turned(place, frame.o3, frame.a3, theta);
    let place_end = far.map();
    let n_end = Affine3::rotation_about_axis(frame.o3, frame.a3, theta).linear * frame.n3;
    let axis_c = turn_axis(
        if reverse {
            Sign::Positive
        } else {
            Sign::Negative
        },
        frame.a3,
    );

    // Each loop with its runs collapsed: a station inside a run has no
    // entity, so a cap carries a run as one meridian edge.
    let cols = loops
        .iter()
        .zip(classes)
        .enumerate()
        .map(|(li, (segs, cls))| collapse_runs(segs, cls, li, band))
        .collect::<Result<Vec<_>, _>>()?;
    let loops: Vec<&[SweptSeg<T>]> = cols.iter().map(|c| &c.segs[..]).collect();

    // World points: start chain, and end chain (pinned vertices are
    // fixed by the rotation — their end point IS the start point, so
    // no drift enters the shared entities).
    let points: Vec<Vec<Point3<T>>> = loops
        .iter()
        .map(|segs| segs.iter().map(|s| frame.world(s.a)).collect())
        .collect();
    let rpoints: Vec<Vec<Point3<T>>> = cols
        .iter()
        .map(|col| {
            col.segs
                .iter()
                .enumerate()
                .map(|(j, s)| {
                    if col.cls.verts[j].pinned {
                        frame.world(s.a)
                    } else {
                        far.point(s.a)
                    }
                })
                .collect()
        })
        .collect();

    // ---- Phase 1: start lamina (outer loop; extrude's shape). ----
    let outer = loops[0];
    let qs = &points[0];
    // One surgery scope for the whole build (`topo::surgery`): tier 1
    // is this door's postcondition and the tier-2 check below subsumes
    // it. The guard owns the borrow, so a refusal on the way closes
    // the scope by dropping it.
    let mut built = Body::<T>::new();
    let mut body = built.begin_surgery();
    // A one-segment loop (D1's full turn) is swept whole in phases 1–2,
    // far end first (`turn::sweep_turn`), so its seed is the far vertex.
    let ends = |li: usize| TurnEnds {
        near: points[li][0],
        far: rpoints[li][0],
        place_far: far,
        n_far: n_end,
    };
    let seed = body.mvfs(
        if profile::is_full_turn(outer) {
            rpoints[0][0]
        } else {
            qs[0]
        },
        true,
    )?;
    // Start cap plane: derived from the sketch data alone — it reads no
    // entity and mints none — so the closing surface can be in hand
    // before the chain that will carry it.
    let start_plane = cap_plane(
        &cap_points(outer, qs, place),
        place,
        reverse,
        CapEnd::Start,
        band,
    )
    .map_err(|source| RevolveError::CapPlane { source })?;
    let start_cap = FaceSurface::New {
        surface: start_plane,
        sense: true,
    };
    let mut swept_early: Vec<Option<LoopSwept>> = (0..loops.len()).map(|_| None).collect();
    let mut bases = Vec::with_capacity(loops.len());
    let mut verts = Vec::with_capacity(loops.len());
    let (start_face, anchor) = if profile::is_full_turn(outer) {
        let (turn, swept) = sweep_turn(
            &mut body,
            frame,
            &cols[0].cls,
            &outer[0],
            seed.r#loop,
            &ends(0),
            theta,
            axis_c,
            start_cap,
            tol,
        )?;
        bases.push(vec![turn.near_in_wall]);
        verts.push(vec![he_start(&body, turn.near_in_wall)]);
        swept_early[0] = Some(swept);
        (turn.near_face, turn.far_kept)
    } else {
        let start = build_chain(
            &mut body,
            frame,
            seed.r#loop,
            seed.vertex,
            outer,
            qs,
            start_cap,
            tol,
        )?;
        let anchor = start.hes[0];
        bases.push(start.hes);
        verts.push(start.verts);
        (start.face, anchor)
    };
    let end_face = seed.face;
    let start_surface = face_surface_key(&body, start_face);

    // ---- Phase 2: holes (rings in the seed face + kfmrh into the
    // start cap; extrude's shape — hole vertices are never on-axis for
    // a validated profile, so the generic sweep handles them). ----
    for (li, segs) in loops.iter().enumerate().skip(1) {
        let hq = &points[li];
        if profile::is_full_turn(segs) {
            let (turn, swept) = crate::swept::full_turn_hole(
                &mut body,
                anchor,
                rpoints[li][0],
                start_face,
                tol,
                |b, ring, disc| {
                    sweep_turn(
                        b,
                        frame,
                        &cols[li].cls,
                        &segs[0],
                        ring,
                        &ends(li),
                        theta,
                        axis_c,
                        disc,
                        tol,
                    )
                },
            )?;
            bases.push(vec![turn.near_in_wall]);
            verts.push(vec![he_start(&body, turn.near_in_wall)]);
            swept_early[li] = Some(swept);
            continue;
        }
        let (ring, ring_vertex) = crate::swept::plant_hole_ring(&mut body, anchor, hq[0], tol)?;
        let hole = build_chain(
            &mut body,
            frame,
            ring,
            ring_vertex,
            segs,
            hq,
            crate::swept::transient_disc(start_surface),
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
    for (li, (col, hes)) in cols.iter().zip(&bases).enumerate() {
        let swept = match swept_early[li].take() {
            Some(swept) => swept,
            None => sweep_loop(
                &mut body,
                li,
                col,
                hes,
                &points[li],
                &rpoints[li],
                frame,
                theta,
                axis_c,
                far,
                n_end,
                band,
                tol,
            )?,
        };
        walls_all.push(swept.faces);
        rims_all.push(swept.rims);
        tops_all.push(swept.tops);
    }

    // ---- Phase 4: the swept face survives as the end cap. ----
    let end_plane = cap_plane(
        &cap_points(loops[0], &rpoints[0], far),
        place_end,
        reverse,
        CapEnd::End,
        band,
    )
    .map_err(|source| RevolveError::CapPlane { source })?;
    let end_surface = body.set_face_surface(
        end_face,
        FaceSurface::New {
            surface: end_plane,
            sense: true,
        },
    )?;

    // ---- Phase 5: rim upgrades (both cap planes exist): loops in
    // canonical order, segments in swept order; per walled segment the
    // start rim then the end rim; on-axis segments classify the two
    // caps against each other (module docs). ----
    finish_partial(
        &mut body,
        &cols,
        &bases,
        &walls_all,
        &tops_all,
        start_surface,
        end_surface,
        band,
        tol,
    )?;

    body.close_already_checked();
    let body = built;
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
    for (li, col) in cols.iter().enumerate() {
        let n = col.n_canon;
        let mut wc = vec![None; n];
        let mut rc = vec![None; n];
        let mut sm = vec![None; n];
        let mut em = vec![None; n];
        let mut pc = vec![None; n];
        for (j, s) in col.segs.iter().enumerate() {
            rc[s.canonical_vertex] = rims_all[li][j];
            // A pinned vertex is fixed by the rotation, so its start-
            // chain vertex IS the end chain's: the pole.
            if col.cls.verts[j].pinned {
                pc[s.canonical_vertex] = Some(verts[li][j]);
            }
            let bottom = he_edge(&body, bases[li][j]);
            for &m in &col.members[j] {
                wc[m] = walls_all[li][j];
                sm[m] = Some(bottom);
                em[m] = Some(tops_all[li][j].unwrap_or(bottom));
            }
        }
        walls_c.push(super::bands_of(&wc, &col.members));
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
        bands: walls_c,
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

/// The start vertex of `he`, a half-edge the calling driver minted.
///
/// # Panics
///
/// If `he` is not live: every caller passes a half-edge its own driver
/// minted and reads it before any step that kills it.
#[track_caller]
fn he_start<T: Decide>(body: &Body<T>, he: topo::HalfEdgeKey) -> topo::VertexKey {
    body.get_half_edge(he)
        .unwrap_or_else(|| {
            unreachable!(
                "half-edge {he:?} was minted by this driver and is read before any kill of it"
            )
        })
        .start
}

/// The edge of `he`, a chain half-edge the calling driver minted.
///
/// # Panics
///
/// If `he` is not live: every caller passes a half-edge its own driver
/// minted and reads it before any step that kills it.
#[track_caller]
pub(super) fn he_edge<T: Decide>(body: &Body<T>, he: topo::HalfEdgeKey) -> EdgeKey {
    body.get_half_edge(he)
        .unwrap_or_else(|| {
            unreachable!(
                "half-edge {he:?} was minted by this driver and is read before any kill of it"
            )
        })
        .edge
}

/// The rim-upgrade pass (phase 5): per walled segment the start-chain
/// then end-chain meridian upgrades to `Intersection { cap, wall }`;
/// per on-axis segment the shared edge classifies the two caps against
/// each other — `Intersection { start, end }` when definitely
/// transverse (θ ≠ π), conventional when definitely smooth (θ = π).
#[allow(clippy::too_many_arguments)] // one internal call site.
fn finish_partial<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    cols: &[Collapsed<T>],
    bases: &[Vec<topo::HalfEdgeKey>],
    walls_all: &[Vec<Option<FaceKey>>],
    tops_all: &[Vec<Option<EdgeKey>>],
    start_surface: topo::SurfaceKey,
    end_surface: topo::SurfaceKey,
    band: Band,
    tol: Tol,
) -> Result<(), RevolveError> {
    for (li, col) in cols.iter().enumerate() {
        for (j, seg) in col.segs.iter().enumerate() {
            let segment_index = seg.canonical_segment;
            let sliver = |reading, source| RevolveError::SliverRim {
                loop_index: li,
                segment_index,
                reading,
                source,
            };
            let bottom = he_edge(body, bases[li][j]);
            match walls_all[li][j] {
                Some(wall_face) => {
                    let wall = face_surface_key(body, wall_face);
                    upgrade_intersection(body, bottom, start_surface, wall, band, sliver, tol)?;
                    if let Some(top) = tops_all[li][j] {
                        upgrade_intersection(body, top, end_surface, wall, band, sliver, tol)?;
                    }
                }
                None => {
                    debug_assert!(matches!(col.cls.walls[j], WallClass::OnAxis));
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

/// One loop's sweep products, per collapsed segment (`None` = on-axis
/// class).
pub(super) struct LoopSwept {
    pub(super) faces: Vec<Option<FaceKey>>,
    pub(super) rims: Vec<Option<EdgeKey>>,
    pub(super) tops: Vec<Option<EdgeKey>>,
}

/// Sweeps one collapsed loop of the seed face: struts at off-axis
/// vertices, walls for off-axis segments (site addressing per the
/// pinned-run derivation — see M2-LOG PR 5), latitude-join
/// classification.
#[allow(clippy::too_many_arguments)] // one internal call site; the
// arguments are the sweep's fixed context.
pub(super) fn sweep_loop<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    loop_index: usize,
    col: &Collapsed<T>,
    hes: &[topo::HalfEdgeKey],
    qs: &[Point3<T>],
    rq: &[Point3<T>],
    frame: &AxisFrame<T>,
    theta: T,
    axis_c: geom_core::Vec3<T>,
    far: Placing<T>,
    n_end: geom_core::Vec3<T>,
    band: Band,
    tol: Tol,
) -> Result<LoopSwept, RevolveError> {
    let (segs, cls) = (&col.segs[..], &col.cls);
    let n = segs.len();

    // Struts: one latitude arc per off-axis vertex, in traversal order.
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
            revolved_strut_spec(segs[j].a, cls.verts[j].r, qs[j], frame, theta, axis_c, tol),
            tol,
        )?;
        struts.push(Some(m));
    }

    // Walls, one per walled segment (`swept::build_walls`): the
    // far-side edge is the end (rotated) meridian. A segment's ends are
    // its struts' minus halves, or the seed half-edge where the vertex
    // is pinned (on the axis, no strut).
    let at = |j: usize| match &struts[j] {
        Some(s) => s.he_minus,
        None => hes[j],
    };
    let walls = crate::swept::build_walls(
        body,
        n,
        at,
        |body, j, faces| {
            let WallClass::Wall { kind, sense } = cls.walls[j] else {
                return Ok(None);
            };
            // Cocircular arcs of a circle cut into arcs share one key
            // across their walls (`swept::shared_wall`). A wall whose
            // material lies against its revolution surface's chart
            // normal (bore cylinder, inward cone, under-side plane
            // annulus, concave sphere/torus band) states `sense: false`,
            // classified from the profile's stored winding structure
            // (`WallClass::Wall::sense`).
            let surface = match crate::swept::shared_wall(&col.joins, faces, j) {
                Some(f) => FaceSurface::Shared {
                    key: face_surface_key(body, f),
                    sense,
                },
                None => FaceSurface::New {
                    surface: wall_surface(&kind, &segs[j], frame),
                    sense,
                },
            };
            let end = (j + 1) % n;
            let far = placed_segment_spec(&segs[j], far, n_end, rq[j], rq[end], tol);
            Ok::<_, RevolveError>(Some((far, surface)))
        },
        tol,
    )?;
    let (faces, tops) = (walls.faces, walls.tops);

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
        let k_prev = face_surface_key(body, fp);
        let k_next = face_surface_key(body, fnx);
        if k_prev == k_next {
            body.describe_at_rest(strut.edge, k_prev, tol)?;
            continue;
        }
        let vertex_index = segs[j].canonical_vertex;
        upgrade_intersection(
            body,
            strut.edge,
            k_prev,
            k_next,
            band,
            |reading, source| RevolveError::SliverJoin {
                loop_index,
                vertex_index,
                reading,
                source,
            },
            tol,
        )?;
    }

    Ok(LoopSwept { faces, rims, tops })
}

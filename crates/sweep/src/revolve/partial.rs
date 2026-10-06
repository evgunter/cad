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
//! shape), per-loop sweep (struts at off-axis vertices leading a run,
//! one wall per run of off-axis segments — a station inside a run is a
//! vertex of both wedge caps' meridian chains and of nothing else —
//! latitude-join classification), end cap plane, then the upgrade pass
//! (cap–wall meridians, cap–cap axis edges).

use geom_brep::newell_plane;
use geom_core::{Affine3, Band, Decide, Point3, Real, Sign};
use topo::{Body, EdgeKey, FaceKey, FaceSurface, MevSite};

use super::axis::{AxisFrame, LoopClasses, WallClass};
use super::chain::build_chain;
use super::surfaces::{revolved_strut_spec, wall_surface};
use super::upgrade::upgrade_intersection;
use super::{RevolveError, Revolved, RevolvedKind, SweptSeg, WALL_COSURFACE};
use crate::swept::{Join, cap_points, face_surface_key, placed_segment_spec, turn_axis};
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
                        rot.transform_point(frame.world(s.a))
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
    let seed = body.mvfs(qs[0], true)?;
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
        // Newell over the loop the cap runs: outward, as extrude's.
        FaceSurface::New {
            surface: start_plane,
            sense: true,
        },
        tol,
    )?;
    let end_face = seed.face;
    let start_face = start.face;
    let start_surface = face_surface_key(&body, start_face);
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
            // The disc is transient: `kfmrh` kills it at once, and
            // nothing reads its bit.
            FaceSurface::Shared {
                key: start_surface,
                sense: false,
            },
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
        let swept = sweep_loop(
            &mut body,
            li,
            col,
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
    let far_loop = cap_points(loops[0], &rpoints[0], place_end);
    let end_plane =
        newell_plane(&far_loop, band).map_err(|source| RevolveError::CapPlane { source })?;
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
            let sliver = |source| RevolveError::SliverRim {
                loop_index: li,
                segment_index,
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
    place_end: Affine3<T>,
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
            let far = placed_segment_spec(&segs[j], place_end, n_end, rq[j], rq[end], tol);
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

/// A loop's cosurface verdicts for a revolve
/// (`swept::cosurface_pairs`): two consecutive on-axis segments are one
/// carrier, the axis itself; a pair with one on-axis side is
/// structurally false.
fn loop_pairs<T: Decide>(
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    loop_index: usize,
    band: Band,
) -> Result<Vec<bool>, RevolveError> {
    let walled = |j: usize| cls.walls[j].kind().is_some();
    let n = segs.len();
    let mut pair =
        crate::swept::cosurface_pairs(segs, walled, WALL_COSURFACE, band, |j, source| {
            RevolveError::CosurfaceEscalated {
                loop_index,
                vertex_index: segs[j].canonical_vertex,
                source,
            }
        })?;
    for (j, p) in pair.iter_mut().enumerate() {
        if !walled(j) && !walled((j + n - 1) % n) {
            *p = true;
        }
    }
    Ok(pair)
}

/// A revolve's loop with each run of segments on one carrier collapsed
/// to one (crate README, "Walls: one per run"; `swept::collapse_runs`):
/// a station inside a run has no entity in either revolve, so the
/// builders never see it. The run's segment is its first one carried
/// to the run's end (an arc's sweep summed over the run), classified as
/// the first one was — the run is one carrier by the cosurface verdict,
/// and a run of on-axis segments is the axis.
pub(super) struct Collapsed<T: Real> {
    /// The collapsed swept segments, in run order.
    pub(super) segs: Vec<SweptSeg<T>>,
    /// Their classes: each run's leading vertex and first wall.
    pub(super) cls: LoopClasses<T>,
    /// How each collapsed segment's wall meets the previous one's.
    pub(super) joins: Vec<Join>,
    /// Per collapsed segment, the canonical segments its run holds, in
    /// swept order.
    pub(super) members: Vec<Vec<usize>>,
    /// The canonical loop's segment count.
    pub(super) n_canon: usize,
}

/// Collapses one loop's runs ([`Collapsed`]).
///
/// # Errors
///
/// [`RevolveError::PinnedRunStation`] for a station on the axis inside
/// a run of walls: the wall would reach the axis and carry on past it,
/// which the half-plane checks refuse first; surfaced rather than
/// trusted.
pub(super) fn collapse_runs<T: Decide>(
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    loop_index: usize,
    band: Band,
) -> Result<Collapsed<T>, RevolveError> {
    let pair = loop_pairs(segs, cls, loop_index, band)?;
    let col = crate::swept::collapse_runs(segs, &crate::swept::joins(segs, &pair), |seg, next| {
        seg.continued(next)
    });
    for run in &col.members {
        let walled = cls.walls[run[0]].kind().is_some();
        if let Some(&s) = run[1..].iter().find(|&&s| walled && cls.verts[s].pinned) {
            return Err(RevolveError::PinnedRunStation {
                loop_index,
                vertex_index: segs[s].canonical_vertex,
            });
        }
    }
    Ok(Collapsed {
        cls: LoopClasses {
            verts: col.members.iter().map(|run| cls.verts[run[0]]).collect(),
            walls: col.members.iter().map(|run| cls.walls[run[0]]).collect(),
        },
        members: col
            .members
            .iter()
            .map(|run| run.iter().map(|&s| segs[s].canonical_segment).collect())
            .collect(),
        joins: col.joins,
        segs: col.segs,
        n_canon: segs.len(),
    })
}

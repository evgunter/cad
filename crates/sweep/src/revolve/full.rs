//! The full (θ = 2π) revolve: Mäntylä §12.5's closed-figure rotational
//! sweep transcribed to certified operators under our CCW convention,
//! with the two axis-contact classes as first-class case analysis
//! (Problem 12.2 done properly — module docs).
//!
//! **Lamina case** (no axis contact): the profile lamina is swept in
//! one full-period band (extrude's sweep shape — struts are
//! full-period latitude rims, walls the complete revolution surfaces,
//! the copied chain minted at the original coordinates: full period is
//! definitionally the identity). The two lamina faces survive as the
//! two coincident seam discs; same-shell `kfmrh` demotes one into the
//! other (the genus supplier — the washer is genus 1), and the
//! loopglue zip (`mekr` null edge + `kev` per vertex pair, `mef` +
//! `kev` + `kef` per subsequent pair, final `kef`) erases the seam,
//! leaving each meridian edge with both halves in its own wall — the
//! `Seam` state. The book's pseudo-wire opening (12.11's null-edge
//! `lmev` + `lkef`) is subsumed: our sweep operates on the closed
//! lamina directly, so the opening's entities would be minted only for
//! the zip to kill them (deviation recorded in the PR).
//!
//! **Wire case** (axis contact = one or more contiguous runs of
//! on-axis segments): the on-axis runs are omitted — the profile opens
//! into wires whose tips land on the axis as poles/apexes, one for the
//! outer shell and one per gap between two runs for that gap's cavity
//! (`build_full`). The wire sweep
//! struts only interior vertices; tip walls close directly onto the
//! fixed tip vertices; the zip runs with no `kfmrh` (an axis-anchored
//! wire adds no handle — genus 0) and no null-edge `mekr` (the tips
//! were never duplicated). The two π-bands exist so a pole or apex
//! keeps valence 2 through a CURVED wall's two meridians; a plane wall
//! needs neither, so it is made one face before the build returns —
//! its band-2 twin killed into it across the angle-π copy, its angle-0
//! meridian killed with the pole it ends at (a disc) or into the ring
//! that separates its inner circle (an annulus).
//!
//! **Runs** (crate README, "Walls: one per run"): both cases build from
//! the loop with each run of segments on one carrier collapsed to one
//! (`runs::Collapsed`), so a station inside a run has no entity here; the
//! handles map each run's wall and meridians back onto every canonical
//! segment it holds.

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Decide, Margin, Point2, Point3, Real, Sign};
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite, MekrSite, MevSite};
use topo::{Body, EdgeKey, FaceKey, VertexKey};

use super::axis::{AxisFrame, AxisRun, LoopClasses, VertexClass, WallClass, WallKind};
use super::chain::build_chain;
use super::partial::{he_edge, sweep_loop};
use super::runs::{Collapsed, collapse_runs};
use super::surfaces::{revolved_strut_spec, wall_surface};
use super::turn::{TurnEnds, sweep_turn};
use super::upgrade::{upgrade_intersection, upgrade_meridian_wrap};
use super::{BandWall, Cavity, CavityOrigin, RevolveError, Revolved, RevolvedKind, SweptSeg};
use crate::swept::{decide, face_surface_key, placed_segment_spec, turn_axis};
use geom_core::Tol;

/// Builds the full solid of revolution (file docs). `theta` is +2π
/// (the module-doc convention; the sweep traverses reversed chains —
/// a hole loop's stored clockwise chain traversed FORWARD, see
/// `revolve`).
///
/// **Pockets** (crate topo's README, structural conventions): the
/// result is DEFINED as `revolve(filled) − Σ revolve(pocket)` and
/// executed as the degenerate no-crossing arm. `filled` builds as a
/// lamina (no axis contact) or as the wire of the stretch joining the
/// axially extreme run ends. Each pocket then builds as its own closed
/// solid of revolution in hole-as-outer orientation — a hole loop
/// through the lamina path, a gap's pocket as the wire of the stretch
/// joining the gap's two run ends — and its reversed boundary is
/// inserted as a cavity shell through the shared void-insertion door
/// ([`topo::insert_void`]). Containment evidence is CARRIED from the
/// profile's own validation, and revolution about the shared axis
/// cannot bring two points closer than their meridian distance, so
/// strict 2-D separation is strict 3-D containment verbatim:
/// - loops 1.. are canonical hole loops, each decided strictly inside
///   the outer loop (the containment forest's parity decision plus the
///   simplicity pass's definite clearance margins);
/// - a gap's stretch and the outer stretch are non-adjacent segments of
///   one loop (an axis run separates them on each side), which the
///   simplicity pass decided definitely apart, and the gap itself lies
///   strictly between the outer stretch's two poles;
/// - two gaps' stretches are non-adjacent segments of the same loop too,
///   decided apart by the same pass, and their gaps are disjoint
///   intervals of the axis, so two pocket cavities are disjoint;
/// - a hole lies inside the outer loop and a pocket outside it, and
///   the validation decided the hole apart from every segment of the
///   outer loop, the pocket's stretch included, so a hole's cavity and
///   a pocket's are disjoint.
///
/// So the carried sign is `Positive` by the validation's own decisions,
/// and no 3-D containment test (box or probe) runs here at all. No SSI,
/// no crossing pipeline: the door is a structural insertion.
///
/// **A validated profile is not the only source of that evidence.**
/// `tube_along_arc_hollow` reaches this path with hand-built
/// concentric circle loops and no profile at all; what stands in for
/// the validation is the tube door's own wall funnel, which decides
/// the thickness, the bore, AND the realized gap between the two
/// radii the walls will store definitely positive before anything is
/// minted. Any caller of this path owes an equivalent: a decided
/// strict-containment fact about the SKETCH loops, not about the
/// numbers a caller wrote.
pub(super) fn build_full<T: Decide + topo::AtRestPolicy>(
    frame: &AxisFrame<T>,
    loops: &[Vec<SweptSeg<T>>],
    classes: &[LoopClasses<T>],
    theta: T,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let outer = collapse_runs(&loops[0], &classes[0], 0, band)?;
    let runs = super::axis::analyze_contact(&outer.segs, &outer.cls, 0)?;
    let (mut out, pockets) = match runs[..] {
        [] if profile::is_full_turn(&outer.segs) => {
            (build_turn_lamina(frame, &outer, theta, tol)?, Vec::new())
        }
        [] => (
            build_lamina(frame, 0, &outer, theta, band, tol)?,
            Vec::new(),
        ),
        [run] => (
            build_wire(frame, &Wire::off_run(&outer, run), theta, band, tol)?,
            Vec::new(),
        ),
        _ => {
            let (filled, pockets) = gap_stretches(frame, &outer, &runs, band)?;
            let wire = Wire::of(&outer, filled.start, filled.len);
            (build_wire(frame, &wire, theta, band, tol)?, pockets)
        }
    };

    for (li, (segs, cls)) in loops.iter().zip(classes).enumerate().skip(1) {
        // A hole touching the axis contradicts its validated strict
        // containment (interior points of an `r ≥ 0` region are
        // strictly off-axis) — reachable only through a tolerance
        // disagreement between validation and this run; refused typed.
        if cls.verts.iter().any(|v| v.pinned) {
            return Err(RevolveError::HoleTouchesAxis { loop_index: li });
        }
        let col = collapse_runs(segs, cls, li, band)?;
        let hole = if profile::is_full_turn(&col.segs) {
            build_turn_lamina(frame, &col, theta, tol)
        } else {
            build_lamina(frame, li, &col, theta, band, tol)
        }?;
        let n = segs.len();
        let origin = CavityOrigin::Hole { loop_index: li };
        let grafted = insert_pocket(&mut out, hole, origin)?;
        out.bands.push(grafted.bands);
        out.rims.push(grafted.rims);
        out.poles.push(vec![None; n]);
        let RevolvedKind::Full { meridians, .. } = &mut out.kind else {
            unreachable!("build_full's outer arm answers with RevolvedKind::Full")
        };
        meridians.push(grafted.meridians);
    }

    for (gap, stretch) in pockets.into_iter().enumerate() {
        let wire = pocket_wire(frame, &outer, stretch, band)?;
        let pocket = build_wire(frame, &wire, theta, band, tol)?;
        let grafted = insert_pocket(&mut out, pocket, CavityOrigin::Pocket { gap })?;
        // The pocket's walls are swept by the outer loop's own segments,
        // so its handles land in loop 0's maps, where the filled shell
        // left those segments' entries `None`.
        out.bands[0].extend(grafted.bands);
        fill(&mut out.rims[0], &grafted.rims);
        fill(&mut out.poles[0], &grafted.poles);
        let RevolvedKind::Full {
            meridians,
            pi_walls,
            pi_meridians,
            pi_rims,
            ..
        } = &mut out.kind
        else {
            unreachable!("build_full's outer arm answers with RevolvedKind::Full")
        };
        fill(&mut meridians[0], &grafted.meridians);
        fill(pi_walls, &grafted.pi_walls);
        fill(pi_meridians, &grafted.pi_meridians);
        fill(pi_rims, &grafted.pi_rims);
    }
    // Loop 0's walls in swept order of their first segment: a wire
    // starts at the end of an axis run, and the filled shell's and the
    // pockets' walls interleave as the loop holds them.
    let mut position = vec![0; outer.n_canon];
    for (j, run) in outer.members.iter().enumerate() {
        for &m in run {
            position[m] = j;
        }
    }
    out.bands[0].sort_by_key(|band| position[band.segments[0]]);

    // **Unconditional, and it is the door's whole postcondition.** The
    // phase builders above run under a surgery scope, so tier 1 is not
    // re-derived inside them; this is where the body a caller sees is
    // checked, and at tier 2, which subsumes it.
    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&out.body),
        Ok(()),
        "revolve (full) postcondition: result is not tier-2 valid (kernel bug)",
    );
    Ok(out)
}

/// Writes `from`'s entries over `into`'s where `from` holds one.
fn fill<K: Copy>(into: &mut [Option<K>], from: &[Option<K>]) {
    for (slot, &e) in into.iter_mut().zip(from) {
        if e.is_some() {
            *slot = e;
        }
    }
}

/// An off-axis stretch of a collapsed loop: `len` segments from vertex
/// `start`, both tips on the axis.
#[derive(Clone, Copy, Debug)]
struct Stretch {
    start: usize,
    len: usize,
}

/// The stretches between a loop's axis runs (k ≥ 2), sorted by what
/// they close: the one joining the axially extreme run ends (the filled
/// profile's wire), and each gap's, lowest gap first.
///
/// The swept traversal runs every axis run along `+d̂` (it is the
/// clockwise traversal of a region at `r ≥ 0`, so its material lies to
/// the right, at `+r`). A gap's stretch therefore climbs from one run's
/// upper end to the next run's lower end, and the filled stretch is the
/// one that descends, from the top run back to the bottom one. Each
/// jump is decided through the funnel; a simple loop has exactly one
/// descent.
fn gap_stretches<T: Decide>(
    frame: &AxisFrame<T>,
    col: &Collapsed<T>,
    runs: &[AxisRun],
    band: Band,
) -> Result<(Stretch, Vec<Stretch>), RevolveError> {
    let n = col.segs.len();
    let k = runs.len();
    let mut stretches = Vec::with_capacity(k);
    let mut filled = None;
    for (j, run) in runs.iter().enumerate() {
        let start = (run.start + run.len) % n;
        let next = runs[(j + 1) % k].start;
        stretches.push(Stretch {
            start,
            len: (next + n - start) % n,
        });
        let vertex_index = col.segs[start].canonical_vertex;
        let jump = frame.axial(col.segs[next].a) - frame.axial(col.segs[start].a);
        match decide("axis_gap", Margin::of(jump), band).map_err(|source| {
            RevolveError::AxisGapEscalated {
                loop_index: 0,
                vertex_index,
                source,
            }
        })? {
            Sign::Positive => {}
            Sign::Negative => {
                if filled.replace(j).is_some() {
                    unreachable!(
                        "two stretches of one simple loop descend between its axis runs, \
                         though the runs ascend and a simple loop returns to its lowest run \
                         once"
                    )
                }
            }
            // Two run ends at one axial station on the axis are one
            // point the loop meets twice: a pinch.
            Sign::Zero => {
                return Err(RevolveError::NonManifoldAxisContact {
                    loop_index: 0,
                    vertex_index,
                });
            }
        }
    }
    let Some(f) = filled else {
        unreachable!("no stretch of a closed loop descends, though its axis runs ascend")
    };
    let pockets = (1..k).map(|g| stretches[(f + g) % k]).collect();
    Ok((stretches[f], pockets))
}

/// A gap's pocket as a wire in hole-as-outer orientation: the gap's
/// stretch of the outer loop's one collapse, traversed backward. The
/// swept traversal has the profile on its right; backward, the pocket
/// is on its right, which is how a swept traversal holds its material,
/// so each wall is classified with `reverse` set as the outer loop's
/// are. The vertex classes are the outer loop's own (a point's radial
/// class has no direction), and so are the runs and their members, in
/// the loop's swept order.
fn pocket_wire<T: Decide>(
    frame: &AxisFrame<T>,
    outer: &Collapsed<T>,
    stretch: Stretch,
    band: Band,
) -> Result<Wire<T>, RevolveError> {
    let n = outer.segs.len();
    let m = stretch.len;
    // Wire vertex `i` is the stretch's vertex `m − i`; wire segment `i`
    // is the stretch's segment `m − 1 − i`, reversed.
    let at = |i: usize| (stretch.start + i) % n;
    let verts: Vec<VertexClass<T>> = (0..=m).map(|i| outer.cls.verts[at(m - i)]).collect();
    let canon_verts: Vec<usize> = (0..=m)
        .map(|i| outer.segs[at(m - i)].canonical_vertex)
        .collect();
    let segs: Vec<SweptSeg<T>> = (0..m)
        .map(|i| outer.segs[at(m - 1 - i)].reversed(canon_verts[i]))
        .collect();
    let walls = (0..m)
        .map(|i| {
            super::axis::classify_segment(&segs[i], &verts[i], &verts[i + 1], frame, 0, true, band)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Wire {
        points: (0..=m).map(|i| outer.segs[at(m - i)].a).collect(),
        members: (0..m)
            .map(|i| outer.members[at(m - 1 - i)].clone())
            .collect(),
        segs,
        walls,
        verts,
        canon_verts,
        n_canon: outer.n_canon,
    })
}

/// A pocket's handles re-keyed into the result body, over its canonical
/// loop.
struct Grafted {
    bands: Vec<BandWall>,
    rims: Vec<Option<EdgeKey>>,
    poles: Vec<Option<VertexKey>>,
    meridians: Vec<Option<EdgeKey>>,
    pi_walls: Vec<Option<FaceKey>>,
    pi_meridians: Vec<Option<EdgeKey>>,
    pi_rims: Vec<Option<EdgeKey>>,
}

/// Inserts a pocket's solid of revolution (built in hole-as-outer
/// orientation) into `out` as a cavity through the void door, with the
/// profile's carried containment (`build_full`'s docs), records the
/// cavity under `origin`, and re-keys the pocket's handles into the
/// result body.
fn insert_pocket<T: Decide>(
    out: &mut Revolved<T>,
    pocket: Revolved<T>,
    origin: CavityOrigin,
) -> Result<Grafted, RevolveError> {
    let evidence = topo::VoidEvidence {
        shells: vec![(
            pocket.shell,
            // The profile validation's strict-separation decision,
            // carried verbatim (`build_full`'s docs).
            topo::VoidContainment::Carried {
                sign: Sign::Positive,
            },
        )],
    };
    let inserted =
        topo::insert_void(&mut out.body, out.solid, pocket.body, &evidence).map_err(|source| {
            RevolveError::VoidInsertion {
                cavity: origin,
                source,
            }
        })?;
    // The graft's bridge is the ONLY bridge, total over the cavity's
    // live entities, and these handles are the pocket body's own.
    let bridged = |what: &str| -> ! {
        unreachable!(
            "the pocket's {what} handle is missing from the void graft bridge: the bridge is \
             total over the cavity's live entities, and the handle is one the pocket build \
             minted"
        )
    };
    let face = |f: FaceKey| inserted.face(f).unwrap_or_else(|| bridged("face"));
    let edge = |e: EdgeKey| inserted.edge(e).unwrap_or_else(|| bridged("edge"));
    let edges = |es: &[Option<EdgeKey>]| es.iter().map(|e| e.map(edge)).collect::<Vec<_>>();
    let RevolvedKind::Full {
        meridians,
        pi_walls,
        pi_meridians,
        pi_rims,
        ..
    } = &pocket.kind
    else {
        unreachable!("a full revolve's pocket answers with RevolvedKind::Full")
    };
    out.cavities.push(Cavity {
        origin,
        shell: inserted
            .shell(pocket.shell)
            .unwrap_or_else(|| bridged("shell")),
    });
    Ok(Grafted {
        bands: pocket.bands[0]
            .iter()
            .map(|band| BandWall {
                face: face(band.face),
                segments: band.segments.clone(),
            })
            .collect(),
        rims: edges(&pocket.rims[0]),
        poles: pocket.poles[0]
            .iter()
            .map(|v| v.map(|v| inserted.vertex(v).unwrap_or_else(|| bridged("pole"))))
            .collect(),
        meridians: edges(&meridians[0]),
        pi_walls: pi_walls.iter().map(|f| f.map(face)).collect(),
        pi_meridians: edges(pi_meridians),
        pi_rims: edges(pi_rims),
    })
}

/// The lamina case (file docs): closed loop, no axis contact — the
/// outer loop of an off-axis profile, or (with `loop_index > 0`, for
/// error attribution) one hole loop building as its own
/// hole-as-outer solid of revolution before the door reverses it.
fn build_lamina<T: Decide + topo::AtRestPolicy>(
    frame: &AxisFrame<T>,
    loop_index: usize,
    col: &Collapsed<T>,
    theta: T,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let (segs, cls) = (&col.segs[..], &col.cls);
    let place = frame.place;
    let n = segs.len();
    let qs: Vec<Point3<T>> = segs.iter().map(|s| frame.world(s.a)).collect();

    // ---- Phase 1: the profile lamina (extrude's shape). Both faces
    // are transient seam discs (killed by the zip), so the closing mef
    // face keeps an honest Nurbs no-description, like the mvfs seed.
    // One surgery scope for the whole build (`topo::surgery`): tier 1
    // is the door's postcondition, and `build_full` pays it over the
    // finished body — at tier 2, which subsumes it — so the close
    // here adds no second whole-body walk. The guard owns the borrow,
    // so a refusal on the way closes the scope by dropping it.
    let mut built = Body::<T>::new();
    let mut body = built.begin_surgery();
    let seed = body.mvfs(qs[0], true)?;
    let lamina = build_chain(
        &mut body,
        frame,
        seed.r#loop,
        seed.vertex,
        segs,
        &qs,
        // A placeholder has no chart normal to state a side against;
        // the bit is provisional, and the disc is killed by the zip.
        FaceSurface::New {
            surface: Surface::nurbs_placeholder(),
            sense: true,
        },
        tol,
    )?;
    let hes = lamina.hes;
    let start_disc = lamina.face;

    // ---- Phase 2: the one-band sweep (the generic pinned-aware sweep
    // with nothing pinned; rotated copies at the original coordinates
    // and the original placement — full period is the identity). ----
    let axis_c = turn_axis(Sign::Positive, frame.a3);
    let swept = sweep_loop(
        &mut body,
        loop_index,
        col,
        &hes,
        &qs,
        &qs,
        frame,
        theta,
        axis_c,
        place.into(),
        frame.n3,
        band,
        tol,
    )?;

    // ---- Phase 3: seam closure — kfmrh + the loopglue zip (see the
    // file docs). ----
    body.kfmrh(start_disc, seed.face)?;
    // Each copied-chain edge is read before the `kef` that kills it.
    let c_plus = |body: &Body<T>, edge: EdgeKey| -> topo::HalfEdgeKey {
        body.get_edge(edge)
            .unwrap_or_else(|| {
                unreachable!(
                    "copied-chain edge {edge:?} was minted by this sweep and is not yet killed"
                )
            })
            .he_plus
    };
    let e_minus = |body: &Body<T>, he: topo::HalfEdgeKey| -> topo::HalfEdgeKey {
        let edge = he_edge(body, he);
        body.get_edge(edge)
            .unwrap_or_else(|| unreachable!("edge {edge:?} was read off live half-edge {he:?}"))
            .he_minus
    };
    // Copied-chain edges (the walls' mef edges), all present in the
    // lamina case (nothing is pinned). The defensive fallback to the
    // chain edge is unreachable; were it ever taken, the zip's own
    // operator preconditions would refuse loudly (typed `Op`).
    let mut tops: Vec<EdgeKey> = Vec::with_capacity(n);
    for (j, t) in swept.tops.iter().enumerate() {
        tops.push(match t {
            Some(e) => *e,
            None => he_edge(&body, hes[j]),
        });
    }
    // The site's two keys are read out before the call: a `Surgery`
    // guard derefs, and a deref is not a two-phase borrow.
    let (target, ring) = (e_minus(&body, hes[n - 1]), c_plus(&body, tops[0]));
    let n0 = body.mekr(
        MekrSite::Cycles { target, ring },
        EdgeCurveSpec::self_loop_circle_at(qs[0]),
        tol,
    )?;
    // Each kill merges a copied vertex into its coincident original
    // across a certified closing circle; the merged fan keeps its
    // carriers, re-certified at the survivor under the run's band.
    body.kev_describing(n0.he_plus, &[], tol)?;
    for j in 1..n {
        let (he1, he2) = (e_minus(&body, hes[j - 1]), c_plus(&body, tops[j]));
        let nj = body.mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::self_loop_circle_at(qs[j]),
            FaceSurface::Inherit,
            tol,
        )?;
        body.kev_describing(nj.he_plus, &[], tol)?;
        let victim = c_plus(&body, tops[j - 1]);
        body.kef(victim)?;
    }
    let victim = c_plus(&body, tops[n - 1]);
    body.kef(victim)?;

    // ---- Phase 4: each surviving chain edge now has both halves in
    // its wall. A periodic wall's takes `Seam`; a plane wall's is a
    // slit through an annulus, killed into the ring that separates its
    // inner circle (`kemr`), as the wire case's annulus is. ----
    let mut meridians: Vec<Option<EdgeKey>> = vec![None; n];
    for (j, he) in hes.iter().enumerate() {
        let Some(f) = swept.faces[j] else { continue };
        if let WallClass::Wall {
            kind: WallKind::Plane { outward },
            ..
        } = cls.walls[j]
        {
            unslit_plane_wall(&mut body, *he, SlitEnd::Annulus { outward })?;
            continue;
        }
        let wall = face_surface_key(&body, f);
        let edge = he_edge(&body, *he);
        upgrade_meridian_wrap(&mut body, edge, wall, tol)?;
        meridians[j] = Some(edge);
    }

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "revolve (full, lamina) postcondition: result is not tier-2 valid (kernel bug)",
    );

    // ---- Assembly (canonical indexing; a run's segments read its
    // one wall and meridian, its stations no rim). ----
    let nc = col.n_canon;
    let mut walls_c = vec![None; nc];
    let mut rims_c = vec![None; nc];
    let mut mer_c = vec![None; nc];
    body.close_already_checked();
    for (j, s) in segs.iter().enumerate() {
        rims_c[s.canonical_vertex] = swept.rims[j];
        for &m in &col.members[j] {
            walls_c[m] = swept.faces[j];
            mer_c[m] = meridians[j];
        }
    }
    Ok(Revolved {
        body: built,
        solid: seed.solid,
        shell: seed.shell,
        cavities: Vec::new(),
        bands: vec![super::bands_of(&walls_c, &col.members)],
        rims: vec![rims_c],
        // The lamina case is the no-axis-contact case: no profile
        // vertex is on-axis, so there are no poles.
        poles: vec![vec![None; nc]],
        kind: RevolvedKind::Full {
            wire: false,
            meridians: vec![mer_c],
            pi_walls: vec![None; nc],
            pi_meridians: vec![None; nc],
            pi_rims: vec![None; nc],
        },
    })
}

/// The lamina case of a one-segment loop (D1's full turn): ONE torus
/// face closed on itself both ways, its loop the fundamental polygon —
/// the meridian (the loop's one segment, a wrap edge in `u`) and the
/// latitude circle through its one vertex (a wrap edge in `v`), each
/// used once each way at that vertex.
///
/// Built as the lamina case is, with one segment: the turn is swept
/// whole (`turn::sweep_turn`), its far copy at the original coordinates
/// and placement (a full period is the identity), and the same seam
/// closure — `kfmrh` of the seed into the near face, the null-edge
/// `mekr` and its `kev`, and the `kef` of the far copy — leaves the
/// near meridian with both halves in the wall.
fn build_turn_lamina<T: Decide + topo::AtRestPolicy>(
    frame: &AxisFrame<T>,
    col: &Collapsed<T>,
    theta: T,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let seg = &col.segs[0];
    let q = frame.world(seg.a);
    let mut built = Body::<T>::new();
    let mut body = built.begin_surgery();
    let seed = body.mvfs(q, true)?;
    let (turn, swept) = sweep_turn(
        &mut body,
        frame,
        &col.cls,
        seg,
        seed.r#loop,
        &TurnEnds {
            near: q,
            far: q,
            place_far: frame.place.into(),
            n_far: frame.n3,
        },
        theta,
        turn_axis(Sign::Positive, frame.a3),
        // A transient disc, killed by the closure, like the lamina's.
        FaceSurface::New {
            surface: Surface::nurbs_placeholder(),
            sense: true,
        },
        tol,
    )?;
    body.kfmrh(turn.near_face, seed.face)?;
    let near = he_edge(&body, turn.near_in_wall);
    let target = body
        .get_edge(near)
        .unwrap_or_else(|| unreachable!("the near rim {near:?} was just minted"))
        .he_minus;
    let n0 = body.mekr(
        MekrSite::Cycles {
            target,
            ring: turn.far_kept,
        },
        EdgeCurveSpec::self_loop_circle_at(q),
        tol,
    )?;
    body.kev_describing(n0.he_plus, &[], tol)?;
    body.kef(turn.far_kept)?;
    let wall = face_surface_key(&body, turn.wall);
    upgrade_meridian_wrap(&mut body, near, wall, tol)?;

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "revolve (full, one-segment lamina) postcondition: result is not tier-2 valid (kernel bug)",
    );

    let nc = col.n_canon;
    let mut walls_c = vec![None; nc];
    let mut rims_c = vec![None; nc];
    let mut mer_c = vec![None; nc];
    body.close_already_checked();
    rims_c[seg.canonical_vertex] = swept.rims[0];
    for &m in &col.members[0] {
        walls_c[m] = Some(turn.wall);
        mer_c[m] = Some(near);
    }
    Ok(Revolved {
        body: built,
        solid: seed.solid,
        shell: seed.shell,
        cavities: Vec::new(),
        bands: vec![super::bands_of(&walls_c, &col.members)],
        rims: vec![rims_c],
        poles: vec![vec![None; nc]],
        kind: RevolvedKind::Full {
            wire: false,
            meridians: vec![mer_c],
            pi_walls: vec![None; nc],
            pi_meridians: vec![None; nc],
            pi_rims: vec![None; nc],
        },
    })
}

/// A wire: an open chain of off-axis (collapsed) segments whose two
/// tips lie on the axis — the off-axis stretch a full revolve sweeps
/// between two axis-run ends. Wire segment `i` runs from wire vertex
/// `i` to `i + 1`; vertices `0` and `k` are the pinned tips.
struct Wire<T: Real> {
    /// The `k` segments, in swept order.
    segs: Vec<SweptSeg<T>>,
    /// Their wall classes.
    walls: Vec<WallClass<T>>,
    /// The `k + 1` vertex classes.
    verts: Vec<VertexClass<T>>,
    /// The `k + 1` vertex points, sketch coordinates.
    points: Vec<Point2<T>>,
    /// The `k + 1` canonical vertex indices.
    canon_verts: Vec<usize>,
    /// Per segment, the canonical segments its run holds.
    members: Vec<Vec<usize>>,
    /// The canonical loop's segment count.
    n_canon: usize,
}

impl<T: Real> Wire<T> {
    /// The stretch of `col` from vertex `start` along `len` segments.
    fn of(col: &Collapsed<T>, start: usize, len: usize) -> Self {
        let n = col.segs.len();
        let at = |i: usize| (start + i) % n;
        Self {
            segs: (0..len).map(|i| col.segs[at(i)]).collect(),
            walls: (0..len).map(|i| col.cls.walls[at(i)]).collect(),
            verts: (0..=len).map(|i| col.cls.verts[at(i)]).collect(),
            points: (0..=len).map(|i| col.segs[at(i)].a).collect(),
            canon_verts: (0..=len)
                .map(|i| col.segs[at(i)].canonical_vertex)
                .collect(),
            members: (0..len).map(|i| col.members[at(i)].clone()).collect(),
            n_canon: col.n_canon,
        }
    }

    /// The stretch of `col` that remains once its one axis run is
    /// omitted.
    fn off_run(col: &Collapsed<T>, run: AxisRun) -> Self {
        Self::of(col, run.start + run.len, col.segs.len() - run.len)
    }
}

/// The wire case (file docs): the wire sweeps as an open chain
/// anchored on the axis at both tips —
/// in **two π-bands**, so each pole/apex ends with valence 2 (the
/// angle-0 and angle-π meridians, the pole form [`geom::pole_form`]
/// describes; a one-band wire would leave the tips valence-1, which
/// tier 2 rightly bans as strut scaffolding). No seam
/// zip exists in this path: band 2 is carved out of the original wire
/// face by one rim-closing `mef` per interior vertex, and the wire
/// face itself survives as segment 0's band-2 wall. The handles are
/// indexed over the wire's canonical loop, `None` off the wire.
fn build_wire<T: Decide + topo::AtRestPolicy>(
    frame: &AxisFrame<T>,
    wire: &Wire<T>,
    theta: T,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let (segs, walls, verts) = (&wire.segs[..], &wire.walls[..], &wire.verts[..]);
    let place = frame.place;
    let k = segs.len();
    let half = theta * T::from_f64(0.5);
    let far = crate::swept::Placing::turned(place, frame.o3, frame.axis, half)
        .map_err(RevolveError::Turn)?;
    let Some(n_pi) = far.turn_vec(frame.n3) else {
        unreachable!("a turned placing has a turn")
    };
    let pinned = |i: usize| verts[i].pinned;
    let qw: Vec<Point3<T>> = wire.points.iter().map(|&p| frame.world(p)).collect();
    let qpi: Vec<Point3<T>> = (0..=k)
        .map(|i| {
            if pinned(i) {
                qw[i]
            } else {
                far.point(wire.points[i])
            }
        })
        .collect();

    // ---- Phase 1: the open chain (a wire: one face, loop up one side
    // and back the other; no closing mef). ----
    // One surgery scope for the whole build — see `build_lamina`.
    let mut built = Body::<T>::new();
    let mut body = built.begin_surgery();
    let seed = body.mvfs(qw[0], true)?;
    let mut hes = Vec::with_capacity(k);
    let first = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        qw[1],
        placed_segment_spec(&segs[0], place, frame.n3, qw[0], qw[1], tol),
        tol,
    )?;
    hes.push(first.he_plus);
    let mut prev = first;
    for i in 1..k {
        let m = body.mev(
            MevSite::Fan {
                he1: prev.he_minus,
                he2: prev.he_minus,
            },
            qw[i + 1],
            placed_segment_spec(&segs[i], place, frame.n3, qw[i], qw[i + 1], tol),
            tol,
        )?;
        hes.push(m.he_plus);
        prev = m;
    }
    // The two poles, by construction: the wire's tips are the axis
    // run's end vertices, pinned by the rotation, so each is ONE body
    // vertex — the mvfs seed at wire vertex 0 and the last chain
    // `mev`'s vertex at wire vertex k (`prev` is `first` when k = 1).
    let (pole_near, pole_far) = (seed.vertex, prev.vertex);

    // ---- Phase 2: band 1 — sweep the wire by +π (struts are
    // half-period rims at interior vertices; walls carry the FULL
    // revolution surfaces; the mef edges are the angle-π meridian
    // copies). The runs are collapsed (`collapse_runs`), so no two
    // adjacent wire walls continue one carrier: each takes its own
    // surface.
    let axis_c = turn_axis(Sign::Positive, frame.a3);
    let mut struts: Vec<Option<topo::construction::MevCreated>> = vec![None];
    for i in 1..k {
        let m = body.mev(
            MevSite::Fan {
                he1: hes[i],
                he2: hes[i],
            },
            qpi[i],
            revolved_strut_spec(wire.points[i], verts[i].r, qw[i], frame, half, axis_c, tol),
            tol,
        )?;
        struts.push(Some(m));
    }
    let mut faces: Vec<FaceKey> = Vec::with_capacity(k);
    let mut tops: Vec<EdgeKey> = Vec::with_capacity(k);
    for i in 0..k {
        let he1 = match &struts[i] {
            Some(s) => s.he_minus,
            None => hes[0],
        };
        let he2 = if i + 1 < k {
            match &struts[i + 1] {
                Some(s) => s.he_minus,
                // Unreachable: interior wire vertices are off-axis.
                None => hes[i + 1],
            }
        } else {
            // The far tip: the return-side half arriving there.
            let edge = he_edge(&body, hes[k - 1]);
            body.get_edge(edge)
                .unwrap_or_else(|| {
                    unreachable!(
                        "edge {edge:?} was read off live chain half-edge {:?}",
                        hes[k - 1]
                    )
                })
                .he_minus
        };
        // The wall states its classified sense — see
        // `partial::sweep_loop`.
        let surface = match walls[i] {
            WallClass::Wall { kind, sense } => FaceSurface::New {
                surface: wall_surface(&kind, &segs[i], frame),
                sense,
            },
            // Unreachable: wire segments are off-axis by construction.
            WallClass::OnAxis => FaceSurface::Inherit,
        };
        let mef = body.mef(
            MefSite::Chords { he1, he2 },
            placed_segment_spec(&segs[i], far, n_pi, qpi[i], qpi[i + 1], tol),
            surface,
            tol,
        )?;
        faces.push(mef.face);
        tops.push(mef.edge);
    }

    // Band-1 latitude joins at interior struts (same-key runs
    // conventional; witness = carrier(π/2·|θ|-fraction) midpoint).
    for i in 1..k {
        let Some(strut) = &struts[i] else { continue };
        let k_prev = face_surface_key(&body, faces[i - 1]);
        let k_next = face_surface_key(&body, faces[i]);
        if k_prev == k_next {
            body.describe_at_rest(strut.edge, tol)?;
            continue;
        }
        let vertex_index = wire.canon_verts[i];
        upgrade_intersection(
            &mut body,
            strut.edge,
            k_prev,
            k_next,
            band,
            |reading, source| RevolveError::SliverJoin {
                loop_index: 0,
                vertex_index,
                reading,
                source,
            },
            tol,
        )?;
    }

    // ---- Phase 3: band 2 — one rim-closing mef per interior vertex
    // carves the π…2π walls out of the wire face; the wire face itself
    // survives as segment 0's band-2 wall (see the file docs). ----
    let mut band2_faces: Vec<FaceKey> = vec![seed.face];
    let mut rims2: Vec<Option<EdgeKey>> = vec![None];
    for i in 1..k {
        let he1 = body
            .get_edge(tops[i])
            .unwrap_or_else(|| {
                unreachable!(
                    "band-1 rim {:?} was minted by this driver's mef; band 2 kills none",
                    tops[i]
                )
            })
            .he_plus;
        let e_prev = he_edge(&body, hes[i - 1]);
        let he2 = body
            .get_edge(e_prev)
            .unwrap_or_else(|| {
                unreachable!(
                    "edge {e_prev:?} was read off live chain half-edge {:?}",
                    hes[i - 1]
                )
            })
            .he_minus;
        let center = frame.foot3(wire.points[i]);
        let rim = qpi[i] - center;
        // The same rim identity as `revolve::surfaces`', at the same
        // guarantee (its comment carries the argument).
        crate::swept::register_rim_identity(rim, verts[i].r, tol);
        let spec = EdgeCurveSpec {
            // The return half of the whole turn's own range, on the
            // sketch placement: no turned placement is evaluated.
            description: geom_brep::EdgeDescriptionSpec::Scaffold(
                geom_brep::MappedCurve::whole(geom_brep::MappedSource::RevolvedPoint {
                    point: wire.points[i],
                    place,
                    axis_origin: frame.o3,
                    axis_dir: frame.a3,
                    angle: theta,
                })
                .restrict(T::from_f64(0.5), T::one()),
            ),
            carrier: geom::Curve3::Circle {
                center,
                axis: axis_c,
                radius: verts[i].r,
                u_ref: rim.normalize(),
            },
            param_start: T::zero(),
            param_end: half.abs(),
        };
        // The band-2 wall is the same classified wall as its band-1
        // twin: same surface, same material side, same sense.
        let twin = twin_wall(&body, faces[i]);
        let mef = body.mef(MefSite::Chords { he1, he2 }, spec, twin, tol)?;
        band2_faces.push(mef.face);
        rims2.push(Some(mef.edge));
    }
    // The surviving wire face becomes segment 0's band-2 wall — it
    // takes wall 0's sense along with its surface.
    let wall0 = twin_wall(&body, faces[0]);
    body.set_face_surface(seed.face, wall0)?;

    // Band-2 latitude joins (same surface-key pairs as band 1).
    for i in 1..k {
        let Some(rim2) = rims2[i] else { continue };
        let k_prev = face_surface_key(&body, band2_faces[i - 1]);
        let k_next = face_surface_key(&body, band2_faces[i]);
        if k_prev == k_next {
            body.describe_at_rest(rim2, tol)?;
            continue;
        }
        let vertex_index = wire.canon_verts[i];
        upgrade_intersection(
            &mut body,
            rim2,
            k_prev,
            k_next,
            band,
            |reading, source| RevolveError::SliverJoin {
                loop_index: 0,
                vertex_index,
                reading,
                source,
            },
            tol,
        )?;
    }

    // ---- Phase 4: the plane walls made whole. The π split exists so a
    // pole or apex keeps valence 2 through a CURVED wall's two
    // meridians; a plane wall needs neither. Its band-2 twin is killed
    // into it across the angle-π copy (`kef`), and its angle-0
    // meridian, now a slit with both halves in the wall, goes with the
    // pole it ends at (`kev`: a disc) or into the ring that makes the
    // wall an annulus (`kemr`). ----
    let mut plane = vec![false; k];
    let mut pole_killed = [false, false];
    for i in 0..k {
        let WallClass::Wall {
            kind: WallKind::Plane { outward },
            ..
        } = walls[i]
        else {
            continue;
        };
        plane[i] = true;
        let pi_edge = body.get_edge(tops[i]).unwrap_or_else(|| {
            unreachable!(
                "angle-π rim {:?} is minted here; only its own iteration's kef kills it",
                tops[i]
            )
        });
        let (hp, hm) = (pi_edge.he_plus, pi_edge.he_minus);
        let twin_side = if body.face_of_half_edge(hp) == Some(band2_faces[i]) {
            hp
        } else {
            hm
        };
        body.kef(twin_side)?;
        let end = match (pinned(i), pinned(i + 1)) {
            (true, _) => {
                pole_killed[0] = true;
                SlitEnd::PoleAtStart
            }
            (_, true) => {
                pole_killed[1] = true;
                SlitEnd::PoleAtEnd
            }
            _ => SlitEnd::Annulus { outward },
        };
        unslit_plane_wall(&mut body, hes[i], end)?;
    }

    // ---- Phase 5: meridian upgrades — both meridians part a wall's
    // two π-bands, so each takes the wall's chart image at rest
    // (`upgrade_meridian_wrap` reads that off its faces; D3's
    // transience fence — the wall exists by now, so neither copy needs
    // the scaffolding door). ----
    for i in 0..k {
        if plane[i] {
            continue;
        }
        let wall = face_surface_key(&body, faces[i]);
        let edge = he_edge(&body, hes[i]);
        upgrade_meridian_wrap(&mut body, edge, wall, tol)?;
        if body.get_edge(tops[i]).is_some() {
            body.describe_at_rest(tops[i], tol)?;
        }
    }

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "revolve (full, wire) postcondition: result is not tier-2 valid (kernel bug)",
    );

    // ---- Assembly (canonical indexing; omitted run entries None; a
    // wall run's segments read its one wall and meridians, its stations
    // no rim; a plane wall has no π twin and no meridian). ----
    let nc = wire.n_canon;
    let mut walls_c = vec![None; nc];
    let mut rims_c = vec![None; nc];
    let mut mer_c = vec![None; nc];
    let mut pi_walls = vec![None; nc];
    let mut pi_mer = vec![None; nc];
    let mut pi_rims = vec![None; nc];
    for i in 0..k {
        let (mer, pi_wall, pi_m) = if plane[i] {
            (None, None, None)
        } else {
            (
                Some(he_edge(&body, hes[i])),
                Some(band2_faces[i]),
                Some(tops[i]),
            )
        };
        for &m in &wire.members[i] {
            walls_c[m] = Some(faces[i]);
            mer_c[m] = mer;
            pi_walls[m] = pi_wall;
            pi_mer[m] = pi_m;
        }
        if let Some(strut) = &struts[i] {
            rims_c[wire.canon_verts[i]] = Some(strut.edge);
        }
        pi_rims[wire.canon_verts[i]] = rims2[i];
    }
    // Poles: the wire's two tips, where a cone or curved wall ends on
    // them; a plane disc's centre took its pole with its slit. The
    // run's INTERIOR vertices stay `None` — the omitted run took them
    // with it, so no body vertex answers to them.
    let mut poles_c = vec![None; nc];
    if !pole_killed[0] {
        poles_c[wire.canon_verts[0]] = Some(pole_near);
    }
    if !pole_killed[1] {
        poles_c[wire.canon_verts[k]] = Some(pole_far);
    }
    body.close_already_checked();
    Ok(Revolved {
        body: built,
        solid: seed.solid,
        shell: seed.shell,
        cavities: Vec::new(),
        bands: vec![super::bands_of(&walls_c, &wire.members)],
        rims: vec![rims_c],
        poles: vec![poles_c],
        kind: RevolvedKind::Full {
            wire: true,
            meridians: vec![mer_c],
            pi_walls,
            pi_meridians: pi_mer,
            pi_rims,
        },
    })
}

/// The spec that puts a band-2 wall on its band-1 twin's surface with
/// the twin's sense: one classified wall, one material side. The copy
/// is the classification itself: band 1 minted the twin off the wire
/// face's placeholder chart, where its classified bit is written as
/// stated.
#[track_caller]
fn twin_wall<T: Decide>(body: &Body<T>, twin: FaceKey) -> FaceSurface<T> {
    let face = body.get_face(twin).unwrap_or_else(|| {
        unreachable!("band-1 wall {twin:?} was minted by this driver and band 2 kills no wall")
    });
    FaceSurface::Shared {
        key: face.surface,
        sense: face.sense,
    }
}

/// Where a full revolve's plane wall ends its meridian slit.
#[derive(Clone, Copy, Debug)]
enum SlitEnd {
    /// Both ends on circles: the wall is an annulus, inner circle at the
    /// swept segment's start when `outward`.
    Annulus {
        /// The swept chord runs away from the axis.
        outward: bool,
    },
    /// The swept segment starts on the axis: a disc, centre at the start.
    PoleAtStart,
    /// The swept segment ends on the axis: a disc, centre at the end.
    PoleAtEnd,
}

/// Kills a full revolve's plane-wall meridian — the angle-0 chain edge
/// whose two halves both lie in the wall, `chain` the half running the
/// swept segment's start to its end. A disc's slit goes with its pole
/// (`kev` of the half pointing at it); an annulus's becomes the ring
/// that separates its inner circle (`kemr`, whose first argument's side
/// becomes the ring: the half arriving at the inner circle is followed
/// by that circle).
fn unslit_plane_wall<T: Decide>(
    body: &mut Body<T>,
    chain: topo::HalfEdgeKey,
    end: SlitEnd,
) -> Result<(), RevolveError> {
    let edge = he_edge(body, chain);
    let e = body.get_edge(edge).unwrap_or_else(|| {
        unreachable!("edge {edge:?} was read off live chain half-edge {chain:?}")
    });
    let mate = if e.he_plus == chain {
        e.he_minus
    } else {
        e.he_plus
    };
    match end {
        SlitEnd::PoleAtStart => {
            body.kev(mate)?;
        }
        SlitEnd::PoleAtEnd => {
            body.kev(chain)?;
        }
        SlitEnd::Annulus { outward: true } => {
            body.kemr(mate, chain)?;
        }
        SlitEnd::Annulus { outward: false } => {
            body.kemr(chain, mate)?;
        }
    }
    Ok(())
}

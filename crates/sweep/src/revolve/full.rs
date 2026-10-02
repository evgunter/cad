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
//! **Wire case** (axis contact = one contiguous run of on-axis
//! segments): the on-axis run is omitted — the profile opens into a
//! wire whose tips land on the axis as poles/apexes. The wire sweep
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
//! ([`Collapsed`]), so a station inside a run has no entity here; the
//! handles map each run's wall and meridians back onto every canonical
//! segment it holds.

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Decide, Point3, Real, Sign};
use topo::{Body, EdgeKey, FaceKey, FaceSurface, MefSite, MekrSite, MevSite};

use super::axis::{AxisFrame, AxisRun, LoopClasses, WallClass, WallKind};
use super::chain::build_chain;
use super::partial::{he_edge, sweep_loop};
use super::surfaces::{revolved_strut_spec, wall_surface};
use super::upgrade::{upgrade_intersection, upgrade_meridian_seam};
use super::{RevolveError, Revolved, RevolvedKind, SweptSeg};
use crate::swept::{face_surface_key, placed_segment_spec, turn_axis};
use geom_core::Tol;

/// Builds the full solid of revolution (file docs). `theta` is +2π
/// (the module-doc convention; the sweep traverses reversed chains —
/// a hole loop's stored clockwise chain traversed FORWARD, see
/// `revolve`).
///
/// **Holed profiles** (the ratified O4 definition): the result is
/// DEFINED as `revolve(outer) − revolve(hole-as-outer)` and executed
/// as the degenerate no-crossing arm. The outer loop builds exactly
/// as before (lamina or wire); each hole loop then builds as its own
/// closed solid of revolution through the same lamina path — the
/// per-hole seam surgery is the ordinary kfmrh + loopglue zip on the
/// hole's own body — and its reversed boundary is inserted as a
/// cavity shell through the shared void-insertion door
/// ([`topo::insert_void`]). Containment evidence is CARRIED from the
/// profile's own validation: loops 1.. are canonical hole loops, each
/// decided strictly inside the outer loop (the containment forest's
/// parity decision plus the simplicity pass's definite clearance
/// margins), and revolution about the shared axis maps 2-D strict
/// containment to 3-D strict containment verbatim — so the carried
/// sign is `Positive` by the validation's own decisions, and no 3-D
/// containment test (box or probe) runs here at all. No SSI, no
/// crossing pipeline: the door is a structural insertion.
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
pub(super) fn build_full<T: Decide>(
    frame: &AxisFrame<T>,
    loops: &[Vec<SweptSeg<T>>],
    classes: &[LoopClasses<T>],
    theta: T,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let outer = collapse_runs(&loops[0], &classes[0], 0, band)?;
    let run = super::axis::analyze_contact(&outer.segs, &outer.cls, 0)?;
    let mut out = match run {
        None => build_lamina(frame, 0, &outer, theta, band, tol),
        Some(run) => build_wire(frame, &outer, run, theta, band, tol),
    }?;

    for (li, (segs, cls)) in loops.iter().zip(classes).enumerate().skip(1) {
        // A hole touching the axis contradicts its validated strict
        // containment (interior points of an `r ≥ 0` region are
        // strictly off-axis) — reachable only through a tolerance
        // disagreement between validation and this run; refused typed.
        if cls.verts.iter().any(|v| v.pinned) {
            return Err(RevolveError::HoleTouchesAxis { loop_index: li });
        }
        let col = collapse_runs(segs, cls, li, band)?;
        let hole = build_lamina(frame, li, &col, theta, band, tol)?;
        let hole_walls = hole.walls();
        let evidence = topo::VoidEvidence {
            shells: vec![(
                hole.shell,
                // The profile validation's strict-containment decision,
                // carried verbatim (fn docs).
                topo::VoidContainment::Carried {
                    sign: Sign::Positive,
                },
            )],
        };
        let inserted = topo::insert_void(&mut out.body, out.solid, hole.body, &evidence).map_err(
            |source| RevolveError::VoidInsertion {
                loop_index: li,
                source,
            },
        )?;
        // Re-key the hole's handles into the result body (the graft's
        // bridge is the ONLY bridge; a miss is graft corruption).
        let desync = |_| RevolveError::VoidInsertion {
            loop_index: li,
            source: topo::VoidInsertError::Corrupt {
                what: "hole handle missing from the void graft bridge",
            },
        };
        let n = segs.len();
        let RevolvedKind::Full {
            meridians: hole_mer,
            ..
        } = &hole.kind
        else {
            unreachable!("build_lamina answers with RevolvedKind::Full")
        };
        let mut walls_c = vec![None; n];
        let mut rims_c = vec![None; n];
        let mut mer_c = vec![None; n];
        for j in 0..n {
            if let Some(f) = hole_walls[0][j] {
                walls_c[j] = Some(inserted.face(f).ok_or(()).map_err(desync)?);
            }
            if let Some(e) = hole.rims[0][j] {
                rims_c[j] = Some(inserted.edge(e).ok_or(()).map_err(desync)?);
            }
            if let Some(e) = hole_mer[0][j] {
                mer_c[j] = Some(inserted.edge(e).ok_or(()).map_err(desync)?);
            }
        }
        out.cavities
            .push(inserted.shell(hole.shell).ok_or(()).map_err(desync)?);
        out.bands.push(super::bands_of(&walls_c, &col.members));
        out.rims.push(rims_c);
        out.poles.push(vec![None; n]);
        let RevolvedKind::Full { meridians, .. } = &mut out.kind else {
            unreachable!("build_full's outer arm answers with RevolvedKind::Full")
        };
        meridians.push(mer_c);
    }

    // **Unconditional, and it is the door's whole postcondition.** The
    // phase builders above run under a surgery scope, so tier 1 is not
    // re-derived inside them; this is where the body a caller sees is
    // checked, and at tier 2, which subsumes it. It used to run on the
    // holed arm only, beside a tier-1 sweep per phase — two walks
    // there and a weaker check on the single-loop arm.
    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&out.body),
        Ok(()),
        "revolve (full) postcondition: result is not tier-2 valid (kernel bug)",
    );
    Ok(out)
}

/// The lamina case (file docs): closed loop, no axis contact — the
/// outer loop of an off-axis profile, or (with `loop_index > 0`, for
/// error attribution) one hole loop building as its own
/// hole-as-outer solid of revolution before the door reverses it.
fn build_lamina<T: Decide>(
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
        &mut body, loop_index, segs, cls, &hes, &qs, &qs, frame, theta, axis_c, place, frame.n3,
        band, tol,
    )?;

    // ---- Phase 3: seam closure — kfmrh + the loopglue zip (see the
    // file docs). ----
    body.kfmrh(start_disc, seed.face)?;
    let c_plus = |body: &Body<T>, edge: EdgeKey| -> Result<topo::HalfEdgeKey, RevolveError> {
        Ok(body
            .get_edge(edge)
            .ok_or(topo::EulerOpError::StaleKey {
                key: topo::EntityId::Edge(edge),
            })?
            .he_plus)
    };
    let e_minus =
        |body: &Body<T>, he: topo::HalfEdgeKey| -> Result<topo::HalfEdgeKey, RevolveError> {
            let edge = he_edge(body, he)?;
            Ok(body
                .get_edge(edge)
                .ok_or(topo::EulerOpError::StaleKey {
                    key: topo::EntityId::Edge(edge),
                })?
                .he_minus)
        };
    // Copied-chain edges (the walls' mef edges), all present in the
    // lamina case (nothing is pinned). The defensive fallback to the
    // chain edge is unreachable; were it ever taken, the zip's own
    // operator preconditions would refuse loudly (typed `Op`).
    let mut tops: Vec<EdgeKey> = Vec::with_capacity(n);
    for (j, t) in swept.tops.iter().enumerate() {
        tops.push(match t {
            Some(e) => *e,
            None => he_edge(&body, hes[j])?,
        });
    }
    // The site's two keys are read out before the call: a `Surgery`
    // guard derefs, and a deref is not a two-phase borrow.
    let (target, ring) = (e_minus(&body, hes[n - 1])?, c_plus(&body, tops[0])?);
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
        let (he1, he2) = (e_minus(&body, hes[j - 1])?, c_plus(&body, tops[j])?);
        let nj = body.mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::self_loop_circle_at(qs[j]),
            FaceSurface::Inherit,
            tol,
        )?;
        body.kev_describing(nj.he_plus, &[], tol)?;
        let victim = c_plus(&body, tops[j - 1])?;
        body.kef(victim)?;
    }
    let victim = c_plus(&body, tops[n - 1])?;
    body.kef(victim)?;

    // ---- Phase 4: meridian upgrades — each surviving chain edge now
    // has both halves in its wall; periodic walls take `Seam`, plane
    // walls keep the conventional description (module docs). A plane
    // annulus keeps its slit here: it is one face already, and the
    // doubly-traversed meridian is what a one-edge rim's blend reads
    // its support by. ----
    let mut meridians: Vec<Option<EdgeKey>> = vec![None; n];
    for (j, he) in hes.iter().enumerate() {
        if let Some(f) = swept.faces[j] {
            let wall = face_surface_key(&body, f)?;
            let edge = he_edge(&body, *he)?;
            upgrade_meridian_seam(&mut body, edge, wall, tol)?;
            meridians[j] = Some(edge);
        }
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

/// The wire case (file docs): the on-axis run is omitted; the rest of
/// the loop sweeps as an open wire anchored on the axis at both tips —
/// in **two π-bands**, so each pole/apex ends with valence 2 (the
/// angle-0 and angle-π meridians; a one-band wire would leave the tips
/// valence-1, which tier 2 rightly bans as strut scaffolding). No seam
/// zip exists in this path: band 2 is carved out of the original wire
/// face by one rim-closing `mef` per interior vertex, and the wire
/// face itself survives as segment 0's band-2 wall.
fn build_wire<T: Decide>(
    frame: &AxisFrame<T>,
    col: &Collapsed<T>,
    run: AxisRun,
    theta: T,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let (segs, cls) = (&col.segs[..], &col.cls);
    let place = frame.place;
    let n = segs.len();
    let k = n - run.len;
    let half = theta * T::from_f64(0.5);
    let rot_pi = geom_core::Affine3::rotation_about_axis(frame.o3, frame.a3, half);
    let place_pi = rot_pi * place;
    let n_pi = rot_pi.linear * frame.n3;
    // Wire segment i is swept segment (run.start + run.len + i) mod n;
    // wire vertex i is wire segment i's start; wire vertex k is the
    // run's start vertex (both tips pinned).
    let wseg = |i: usize| (run.start + run.len + i) % n;
    let wvert = |i: usize| if i < k { wseg(i) } else { run.start };
    let pinned = |i: usize| cls.verts[wvert(i)].pinned;
    let qw: Vec<Point3<T>> = (0..=k).map(|i| frame.world(segs[wvert(i)].a)).collect();
    let qpi: Vec<Point3<T>> = (0..=k)
        .map(|i| {
            if pinned(i) {
                qw[i]
            } else {
                rot_pi.transform_point(qw[i])
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
        placed_segment_spec(&segs[wseg(0)], place, frame.n3, qw[0], qw[1], tol),
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
            placed_segment_spec(&segs[wseg(i)], place, frame.n3, qw[i], qw[i + 1], tol),
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
    let mut struts: Vec<Option<topo::MevCreated>> = vec![None];
    for i in 1..k {
        let m = body.mev(
            MevSite::Fan {
                he1: hes[i],
                he2: hes[i],
            },
            qpi[i],
            revolved_strut_spec(
                segs[wseg(i)].a,
                cls.verts[wseg(i)].r,
                qw[i],
                frame,
                half,
                axis_c,
                tol,
            ),
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
            let edge = he_edge(&body, hes[k - 1])?;
            body.get_edge(edge)
                .ok_or(topo::EulerOpError::StaleKey {
                    key: topo::EntityId::Edge(edge),
                })?
                .he_minus
        };
        // The wall states its classified sense — see
        // `partial::sweep_loop`.
        let surface = match cls.walls[wseg(i)] {
            WallClass::Wall { kind, sense } => FaceSurface::New {
                surface: wall_surface(&kind, &segs[wseg(i)], frame),
                sense,
            },
            // Unreachable: wire segments are off-axis by construction.
            WallClass::OnAxis => FaceSurface::Inherit,
        };
        let mef = body.mef(
            MefSite::Chords { he1, he2 },
            placed_segment_spec(&segs[wseg(i)], place_pi, n_pi, qpi[i], qpi[i + 1], tol),
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
        let k_prev = face_surface_key(&body, faces[i - 1])?;
        let k_next = face_surface_key(&body, faces[i])?;
        if k_prev == k_next {
            body.describe_at_rest(strut.edge, k_prev, tol)?;
            continue;
        }
        let vertex_index = segs[wseg(i)].canonical_vertex;
        upgrade_intersection(
            &mut body,
            strut.edge,
            k_prev,
            k_next,
            band,
            |source| RevolveError::SliverJoin {
                loop_index: 0,
                vertex_index,
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
            .ok_or(topo::EulerOpError::StaleKey {
                key: topo::EntityId::Edge(tops[i]),
            })?
            .he_plus;
        let e_prev = he_edge(&body, hes[i - 1])?;
        let he2 = body
            .get_edge(e_prev)
            .ok_or(topo::EulerOpError::StaleKey {
                key: topo::EntityId::Edge(e_prev),
            })?
            .he_minus;
        let center = frame.foot3(segs[wseg(i)].a);
        let rim = qpi[i] - center;
        // The same rim identity as `revolve::surfaces`', at the same
        // guarantee (its comment carries the argument).
        crate::swept::register_rim_identity(rim, cls.verts[wseg(i)].r, tol);
        let spec = EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Scaffold(
                geom_brep::MappedCurve::RevolvedPoint {
                    point: segs[wseg(i)].a,
                    place: place_pi,
                    axis_origin: frame.o3,
                    axis_dir: frame.a3,
                    angle: half,
                },
            ),
            carrier: geom::Curve3::Circle {
                center,
                axis: axis_c,
                radius: cls.verts[wseg(i)].r,
                u_ref: rim.normalize(),
            },
            param_start: T::zero(),
            param_end: half.abs(),
        };
        // The band-2 wall is the same classified wall as its band-1
        // twin: same surface, same material side, same sense.
        let twin = twin_wall(&body, faces[i])?;
        let mef = body.mef(MefSite::Chords { he1, he2 }, spec, twin, tol)?;
        band2_faces.push(mef.face);
        rims2.push(Some(mef.edge));
    }
    // The surviving wire face becomes segment 0's band-2 wall — it
    // takes wall 0's sense along with its surface.
    let wall0 = twin_wall(&body, faces[0])?;
    body.set_face_surface(seed.face, wall0)?;

    // Band-2 latitude joins (same surface-key pairs as band 1).
    for i in 1..k {
        let Some(rim2) = rims2[i] else { continue };
        let k_prev = face_surface_key(&body, band2_faces[i - 1])?;
        let k_next = face_surface_key(&body, band2_faces[i])?;
        if k_prev == k_next {
            body.describe_at_rest(rim2, k_prev, tol)?;
            continue;
        }
        let vertex_index = segs[wseg(i)].canonical_vertex;
        upgrade_intersection(
            &mut body,
            rim2,
            k_prev,
            k_next,
            band,
            |source| RevolveError::SliverJoin {
                loop_index: 0,
                vertex_index,
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
        } = cls.walls[wseg(i)]
        else {
            continue;
        };
        plane[i] = true;
        let pi_edge = body.get_edge(tops[i]).ok_or(topo::EulerOpError::StaleKey {
            key: topo::EntityId::Edge(tops[i]),
        })?;
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

    // ---- Phase 5: meridian upgrades — angle-0 chain edges sit on the
    // u = 0 seam of their (periodic) wall surfaces; the angle-π copies
    // are NOT the seam, so they take the wall's chart image WITHOUT
    // D1's seam obligation (module docs; D3's transience fence — the
    // wall exists by now, so neither copy needs the scaffolding
    // door). ----
    for i in 0..k {
        if plane[i] {
            continue;
        }
        let wall = face_surface_key(&body, faces[i])?;
        let edge = he_edge(&body, hes[i])?;
        upgrade_meridian_seam(&mut body, edge, wall, tol)?;
        if body.get_edge(tops[i]).is_some() {
            body.describe_at_rest(tops[i], wall, tol)?;
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
    let nc = col.n_canon;
    let mut walls_c = vec![None; nc];
    let mut rims_c = vec![None; nc];
    let mut mer_c = vec![None; nc];
    let mut pi_walls = vec![None; nc];
    let mut pi_mer = vec![None; nc];
    let mut pi_rims = vec![None; nc];
    for i in 0..k {
        let j = wseg(i);
        let s = &segs[j];
        let (mer, pi_wall, pi_m) = if plane[i] {
            (None, None, None)
        } else {
            (
                Some(he_edge(&body, hes[i])?),
                Some(band2_faces[i]),
                Some(tops[i]),
            )
        };
        for &m in &col.members[j] {
            walls_c[m] = Some(faces[i]);
            mer_c[m] = mer;
            pi_walls[m] = pi_wall;
            pi_mer[m] = pi_m;
        }
        if let Some(strut) = &struts[i] {
            rims_c[s.canonical_vertex] = Some(strut.edge);
        }
        pi_rims[s.canonical_vertex] = rims2[i];
    }
    // Poles: the axis run's two end vertices, where a cone or curved
    // wall ends on them; a plane disc's centre took its pole with its
    // slit. The run's INTERIOR vertices stay `None` — the omitted run
    // took them with it, so no body vertex answers to them.
    let mut poles_c = vec![None; nc];
    if !pole_killed[0] {
        poles_c[segs[wvert(0)].canonical_vertex] = Some(pole_near);
    }
    if !pole_killed[1] {
        poles_c[segs[wvert(k)].canonical_vertex] = Some(pole_far);
    }
    body.close_already_checked();
    Ok(Revolved {
        body: built,
        solid: seed.solid,
        shell: seed.shell,
        cavities: Vec::new(),
        bands: vec![super::bands_of(&walls_c, &col.members)],
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
fn twin_wall<T: Decide>(
    body: &Body<T>,
    twin: FaceKey,
) -> Result<FaceSurface<T>, topo::EulerOpError> {
    let face = body.get_face(twin).ok_or(topo::EulerOpError::StaleKey {
        key: topo::EntityId::Face(twin),
    })?;
    Ok(FaceSurface::Shared {
        key: face.surface,
        sense: face.sense,
    })
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
    let edge = he_edge(body, chain)?;
    let e = body.get_edge(edge).ok_or(topo::EulerOpError::StaleKey {
        key: topo::EntityId::Edge(edge),
    })?;
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

/// A full revolve's loop with each wall run collapsed to one segment
/// (crate README, "Walls: one per run": a station inside a run has no
/// entity in a full revolve, so the builders never see it). The run's
/// segment is its first one carried to the run's end (an arc's sweep
/// summed over the run), classified as the first one was — the run is
/// one carrier by the cosurface verdict.
pub(super) struct Collapsed<T: Real> {
    /// The collapsed swept segments, in run order.
    pub(super) segs: Vec<SweptSeg<T>>,
    /// Their classes: each run's leading vertex and first wall.
    pub(super) cls: LoopClasses<T>,
    /// Per collapsed segment, the canonical segments its run holds, in
    /// swept order.
    pub(super) members: Vec<Vec<usize>>,
    /// The canonical loop's segment count.
    pub(super) n_canon: usize,
}

/// Collapses one loop's wall runs ([`Collapsed`]), from the cosurface
/// verdicts between walled neighbours (the partial revolve's, which
/// keeps each station on its wedge caps instead).
pub(super) fn collapse_runs<T: Decide>(
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    loop_index: usize,
    band: Band,
) -> Result<Collapsed<T>, RevolveError> {
    let n = segs.len();
    let walled = |j: usize| cls.walls[j].kind().is_some();
    let pair = super::partial::loop_pairs(segs, cls, loop_index, band)?;
    let runs = crate::swept::wall_runs(segs, &pair, walled, crate::swept::CurvedRuns::Whole);
    let mut out = Collapsed {
        segs: Vec::with_capacity(runs.len()),
        cls: LoopClasses {
            verts: Vec::with_capacity(runs.len()),
            walls: Vec::with_capacity(runs.len()),
        },
        members: Vec::with_capacity(runs.len()),
        n_canon: n,
    };
    for run in runs {
        let last = (run.first + run.len - 1) % n;
        let kind = run
            .segments(n)
            .skip(1)
            .fold(segs[run.first].kind, |kind, s| kind.continued(segs[s].kind));
        out.segs.push(SweptSeg {
            b: segs[last].b,
            kind,
            ..segs[run.first]
        });
        out.cls.verts.push(cls.verts[run.first]);
        out.cls.walls.push(cls.walls[run.first]);
        out.members
            .push(run.segments(n).map(|s| segs[s].canonical_segment).collect());
    }
    Ok(out)
}

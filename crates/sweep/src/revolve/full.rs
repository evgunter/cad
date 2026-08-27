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
//! were never duplicated).

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Decide, Point3, Sign};
use topo::{Body, EdgeKey, FaceKey, FaceSurface, MefSite, MekrSite, MevSite};

use super::axis::{AxisFrame, AxisRun, LoopClasses};
use super::chain::build_chain;
use super::partial::{he_edge, sweep_loop};
use super::surfaces::{strut_spec, wall_surface};
use super::upgrade::{describe_at_rest, upgrade_intersection, upgrade_meridian_seam};
use super::{RevolveError, Revolved, RevolvedKind, SweptSeg, WALL_COSURFACE};
use crate::swept::{cosurface, face_surface_key, placed_segment_spec, turn_axis};
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
    let segs = &loops[0];
    let cls = &classes[0];
    let run = super::axis::analyze_contact(segs, cls, 0)?;
    let mut out = match run {
        None => build_lamina(frame, 0, segs, cls, theta, band, tol),
        Some(run) => build_wire(frame, segs, cls, run, theta, band, tol),
    }?;

    for (li, (segs, cls)) in loops.iter().zip(classes).enumerate().skip(1) {
        // A hole touching the axis contradicts its validated strict
        // containment (interior points of an `r ≥ 0` region are
        // strictly off-axis) — reachable only through a tolerance
        // disagreement between validation and this run; refused typed.
        if cls.verts.iter().any(|v| v.pinned) {
            return Err(RevolveError::HoleTouchesAxis { loop_index: li });
        }
        let hole = build_lamina(frame, li, segs, cls, theta, band, tol)?;
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
        let inserted = topo::insert_void(&mut out.body, out.solid, hole.body, &evidence, tol)
            .map_err(|source| RevolveError::VoidInsertion {
                loop_index: li,
                source,
            })?;
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
            if let Some(f) = hole.walls[0][j] {
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
        out.walls.push(walls_c);
        out.rims.push(rims_c);
        out.poles.push(vec![None; n]);
        let RevolvedKind::Full { meridians, .. } = &mut out.kind else {
            unreachable!("build_full's outer arm answers with RevolvedKind::Full")
        };
        meridians.push(mer_c);
    }

    #[cfg(debug_assertions)]
    if loops.len() > 1 {
        debug_assert_eq!(
            topo::validate_closed(&out.body),
            Ok(()),
            "revolve (full, holed) postcondition: cavity insertion broke tier 2 (kernel bug)",
        );
    }
    Ok(out)
}

/// The lamina case (file docs): closed loop, no axis contact — the
/// outer loop of an off-axis profile, or (with `loop_index > 0`, for
/// error attribution) one hole loop building as its own
/// hole-as-outer solid of revolution before the door reverses it.
fn build_lamina<T: Decide>(
    frame: &AxisFrame<T>,
    loop_index: usize,
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    theta: T,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let place = frame.place;
    let n = segs.len();
    let qs: Vec<Point3<T>> = segs.iter().map(|s| frame.world(s.a)).collect();

    // ---- Phase 1: the profile lamina (extrude's shape). Both faces
    // are transient seam discs (killed by the zip), so the closing mef
    // face keeps an honest Nurbs no-description, like the mvfs seed.
    let mut body = Body::<T>::new();
    let seed = body.mvfs(qs[0])?;
    let lamina = build_chain(
        &mut body,
        frame,
        seed.r#loop,
        seed.vertex,
        segs,
        &qs,
        FaceSurface::New(Surface::nurbs_placeholder()),
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
    let n0 = body.mekr(
        MekrSite::Cycles {
            target: e_minus(&body, hes[n - 1])?,
            ring: c_plus(&body, tops[0])?,
        },
        EdgeCurveSpec::self_loop_circle_at(qs[0]),
        tol,
    )?;
    body.kev(n0.he_plus)?;
    for j in 1..n {
        let nj = body.mef(
            MefSite::Chords {
                he1: e_minus(&body, hes[j - 1])?,
                he2: c_plus(&body, tops[j])?,
            },
            EdgeCurveSpec::self_loop_circle_at(qs[j]),
            FaceSurface::Inherit,
            tol,
        )?;
        body.kev(nj.he_plus)?;
        body.kef(c_plus(&body, tops[j - 1])?)?;
    }
    body.kef(c_plus(&body, tops[n - 1])?)?;

    // ---- Phase 4: meridian upgrades — each surviving chain edge now
    // has both halves in its wall; periodic walls take `Seam`, plane
    // walls keep the conventional description (module docs). ----
    for (j, he) in hes.iter().enumerate() {
        if let Some(f) = swept.faces[j] {
            let wall = face_surface_key(&body, f)?;
            let edge = he_edge(&body, *he)?;
            upgrade_meridian_seam(&mut body, edge, wall, tol)?;
        }
    }

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "revolve (full, lamina) postcondition: result is not tier-2 valid (kernel bug)",
    );

    // ---- Assembly (canonical indexing). ----
    let mut walls_c = vec![None; n];
    let mut rims_c = vec![None; n];
    let mut mer_c = vec![None; n];
    for (j, s) in segs.iter().enumerate() {
        walls_c[s.canonical_segment] = swept.faces[j];
        rims_c[s.canonical_vertex] = swept.rims[j];
        mer_c[s.canonical_segment] = Some(he_edge(&body, hes[j])?);
    }
    Ok(Revolved {
        body,
        solid: seed.solid,
        shell: seed.shell,
        cavities: Vec::new(),
        walls: vec![walls_c],
        rims: vec![rims_c],
        // The lamina case is the no-axis-contact case: no profile
        // vertex is on-axis, so there are no poles.
        poles: vec![vec![None; n]],
        kind: RevolvedKind::Full {
            meridians: vec![mer_c],
            pi_walls: vec![None; n],
            pi_meridians: vec![None; n],
            pi_rims: vec![None; n],
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
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    run: AxisRun,
    theta: T,
    band: Band,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
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
    let mut body = Body::<T>::new();
    let seed = body.mvfs(qw[0])?;
    let mut hes = Vec::with_capacity(k);
    let first = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        qw[1],
        placed_segment_spec(&segs[wseg(0)], place, frame.n3, qw[0], qw[1]),
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
            placed_segment_spec(&segs[wseg(i)], place, frame.n3, qw[i], qw[i + 1]),
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
    // copies). Cosurface pairs precomputed; no wrap on an open chain.
    let axis_c = turn_axis(Sign::Positive, frame.a3);
    let mut pair = vec![false; k];
    for i in 1..k {
        pair[i] = cosurface(&segs[wseg(i - 1)], &segs[wseg(i)], WALL_COSURFACE, band).map_err(
            |source| RevolveError::CosurfaceEscalated {
                loop_index: 0,
                vertex_index: segs[wseg(i)].canonical_vertex,
                source,
            },
        )?;
    }
    let mut struts: Vec<Option<topo::MevCreated>> = vec![None];
    for i in 1..k {
        let m = body.mev(
            MevSite::Fan {
                he1: hes[i],
                he2: hes[i],
            },
            qpi[i],
            strut_spec(
                segs[wseg(i)].a,
                cls.verts[wseg(i)].r,
                qw[i],
                frame,
                half,
                axis_c,
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
        let kind = cls.walls[wseg(i)].kind();
        let surface = match (pair[i], i, kind) {
            (true, 1.., _) => FaceSurface::Shared(face_surface_key(&body, faces[i - 1])?),
            (_, _, Some(kind)) => FaceSurface::New(wall_surface(kind, &segs[wseg(i)], frame)),
            // Unreachable: wire segments are off-axis by construction.
            (_, _, None) => FaceSurface::Inherit,
        };
        let mef = body.mef(
            MefSite::Chords { he1, he2 },
            placed_segment_spec(&segs[wseg(i)], place_pi, n_pi, qpi[i], qpi[i + 1]),
            surface,
            tol,
        )?;
        // The honest orientation bit (M5 S11) — see
        // `partial::sweep_loop`; the band-2 twins below inherit the
        // same classification.
        if cls.walls[wseg(i)].sense() == Some(false) {
            body.set_face_sense(mef.face, false)?;
        }
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
            describe_at_rest(&mut body, strut.edge, k_prev, tol)?;
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
                u_ref: (qpi[i] - center).normalize(),
            },
            param_start: T::zero(),
            param_end: half.abs(),
        };
        let mef = body.mef(
            MefSite::Chords { he1, he2 },
            spec,
            FaceSurface::Shared(face_surface_key(&body, faces[i])?),
            tol,
        )?;
        // The band-2 wall is the same classified wall as its band-1
        // twin (M5 S11): same surface, same material side, same sense.
        if cls.walls[wseg(i)].sense() == Some(false) {
            body.set_face_sense(mef.face, false)?;
        }
        band2_faces.push(mef.face);
        rims2.push(Some(mef.edge));
    }
    let wall0_key = face_surface_key(&body, faces[0])?;
    body.set_face_surface(seed.face, FaceSurface::Shared(wall0_key))?;
    // The surviving wire face becomes segment 0's band-2 wall — it
    // takes wall 0's sense along with its surface (M5 S11).
    if cls.walls[wseg(0)].sense() == Some(false) {
        body.set_face_sense(seed.face, false)?;
    }

    // Band-2 latitude joins (same surface-key pairs as band 1).
    for i in 1..k {
        let Some(rim2) = rims2[i] else { continue };
        let k_prev = face_surface_key(&body, band2_faces[i - 1])?;
        let k_next = face_surface_key(&body, band2_faces[i])?;
        if k_prev == k_next {
            describe_at_rest(&mut body, rim2, k_prev, tol)?;
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

    // ---- Phase 4: meridian upgrades — angle-0 chain edges sit on the
    // u = 0 seam of their (periodic) wall surfaces; the angle-π copies
    // are NOT the seam, so they take the wall's chart image WITHOUT
    // D1's seam obligation (module docs; D3's transience fence — the
    // wall exists by now, so neither copy needs the scaffolding
    // door). ----
    for i in 0..k {
        let wall = face_surface_key(&body, faces[i])?;
        let edge = he_edge(&body, hes[i])?;
        upgrade_meridian_seam(&mut body, edge, wall, tol)?;
        if body.get_edge(tops[i]).is_some() {
            describe_at_rest(&mut body, tops[i], wall, tol)?;
        }
    }

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "revolve (full, wire) postcondition: result is not tier-2 valid (kernel bug)",
    );

    // ---- Assembly (canonical indexing; omitted run entries None). ----
    let mut walls_c = vec![None; n];
    let mut rims_c = vec![None; n];
    let mut mer_c = vec![None; n];
    let mut pi_walls = vec![None; n];
    let mut pi_mer = vec![None; n];
    let mut pi_rims = vec![None; n];
    for i in 0..k {
        let s = &segs[wseg(i)];
        walls_c[s.canonical_segment] = Some(faces[i]);
        mer_c[s.canonical_segment] = Some(he_edge(&body, hes[i])?);
        pi_walls[s.canonical_segment] = Some(band2_faces[i]);
        pi_mer[s.canonical_segment] = Some(tops[i]);
        if let Some(strut) = &struts[i] {
            rims_c[s.canonical_vertex] = Some(strut.edge);
        }
        pi_rims[s.canonical_vertex] = rims2[i];
    }
    // Poles: the run's two end vertices. The run's INTERIOR vertices
    // stay `None` — the omitted run took them with it, so no body
    // vertex answers to them.
    let mut poles_c = vec![None; n];
    poles_c[segs[wvert(0)].canonical_vertex] = Some(pole_near);
    poles_c[segs[wvert(k)].canonical_vertex] = Some(pole_far);
    Ok(Revolved {
        body,
        solid: seed.solid,
        shell: seed.shell,
        cavities: Vec::new(),
        walls: vec![walls_c],
        rims: vec![rims_c],
        poles: vec![poles_c],
        kind: RevolvedKind::Full {
            meridians: vec![mer_c],
            pi_walls,
            pi_meridians: pi_mer,
            pi_rims,
        },
    })
}

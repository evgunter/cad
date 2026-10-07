//! **A ring on a cone face winds its island and re-homes its rings
//! without a chart** ([`super::path_island_winding`],
//! [`super::path_ring_side`]). No public door reaches a cone face's ring
//! lane yet. The boolean's operand gate refuses a cone operand
//! (`work/germ/boolean-sector-algebra-has-no-cone-arm.md`). The plane
//! split admits a cone face and re-homes through the same
//! [`super::ChordJoiner`], but nothing it is handed carries a ring on one:
//! a body at rest cannot (its volume refuses
//! `MassPropsError::RingOnCurvedFace`), and a plane's section of a cone
//! goes round the axis, so it reaches the face's seam rather than
//! landing as a ring. So these rows build the lane's input on a cone
//! sheet: a face of the cone
//! between two rims and two rulings, a ring of it whose run is an
//! island's boundary less one edge, and that edge as the closing chord.
//! Each reading is held to the islands' membership oracles
//! ([`crate::ring_path::cone_islands`]).

use super::*;
use crate::MevSite;
use crate::ring_path::cone_islands::{
    Frame, Island, IslandEdge, cone, frames, lune, on_nappe, plane, sector,
};
use crate::ring_path::cone_islands::{band, scales};
use core::f64::consts::FRAC_PI_6;

fn tol() -> Tol {
    Tol::witness()
}

/// The certified spec of `e` on the cone `surface`: a section arc run
/// forward from its corner (on the reversed carrier where its parameter
/// falls), or a ruling's straight segment.
fn edge_spec(body: &mut Body<f64>, surface: SurfaceKey, e: &IslandEdge) -> EdgeCurveSpec<f64> {
    let (t0, t1) = e.params;
    let Some((o, n)) = e.plane else {
        return EdgeCurveSpec::line_between(e.carrier.eval(t0), e.carrier.eval(t1));
    };
    let s2 = body.add_surface(plane(o, n));
    let (carrier, t0, t1) = if t1 > t0 {
        (e.carrier.clone(), t0, t1)
    } else {
        (e.carrier.reversed().unwrap(), -t0, -t1)
    };
    EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            s1: surface,
            s2,
            witness: carrier.mid_point(t0, t1),
        },
        carrier,
        param_start: t0,
        param_end: t1,
    }
}

/// The rim arc of the nappe at height `h` from azimuth `t0` to `t1`.
fn rim(f: Frame, h: f64, (t0, t1): (f64, f64)) -> IslandEdge {
    let centre = f.at(0.0, 0.0, h);
    IslandEdge {
        carrier: geom::Curve3::Circle {
            center: centre,
            axis: f.z,
            radius: h * f.s * FRAC_PI_6.tan(),
            u_ref: f.x,
        },
        params: (t0, t1),
        plane: Some((centre, f.z)),
    }
}

/// **A face of `cone` between its rims at heights 0.05 and 3 and its
/// rulings at azimuths ±1.2**, of sense `sense`, and its surface.
fn sheet(f: Frame, cone: geom::Surface<f64>, sense: bool) -> (Body<f64>, FaceKey, SurfaceKey) {
    sheet_to(f, cone, sense, 1.2)
}

/// [`sheet`] between the rulings at azimuths `±t`, its first vertex on
/// the rim at height 0.05 and azimuth `−t`.
fn sheet_to(
    f: Frame,
    cone: geom::Surface<f64>,
    sense: bool,
    t: f64,
) -> (Body<f64>, FaceKey, SurfaceKey) {
    sheet_up_to(f, cone, sense, (t, 3.0))
}

/// [`sheet_to`] with its high rim at height `hi`.
fn sheet_up_to(
    f: Frame,
    cone: geom::Surface<f64>,
    sense: bool,
    (t, hi): (f64, f64),
) -> (Body<f64>, FaceKey, SurfaceKey) {
    let lo = 0.05;
    let mut body = Body::<f64>::new();
    let start = on_nappe(f, lo, -t);
    let seed = body.mvfs(start, sense).unwrap();
    let surface = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: cone,
                sense,
            },
        )
        .unwrap();
    let low = edge_spec(&mut body, surface, &rim(f, lo, (-t, t)));
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            on_nappe(f, lo, t),
            low,
            tol(),
        )
        .unwrap();
    let up = body
        .mev_line(
            MevSite::Fan {
                he1: first.he_minus,
                he2: first.he_minus,
            },
            on_nappe(f, hi, t),
            tol(),
        )
        .unwrap();
    let high = edge_spec(&mut body, surface, &rim(f, hi, (t, -t)));
    let last = body
        .mev(
            MevSite::Fan {
                he1: up.he_minus,
                he2: up.he_minus,
            },
            on_nappe(f, hi, -t),
            high,
            tol(),
        )
        .unwrap();
    let face = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: last.he_minus,
            },
            EdgeCurveSpec::line_between(start, on_nappe(f, hi, -t)),
            FaceSurface::Shared {
                key: surface,
                sense,
            },
            tol(),
        )
        .unwrap()
        .face;
    (body, face, surface)
}

/// An empty ring of `face` at `p`, bridged from the outer loop's first
/// vertex and the bridge killed.
fn empty_ring(body: &mut Body<f64>, face: FaceKey, p: Point3<f64>) -> LoopKey {
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("outer is a cycle");
    };
    let u = body.half_edge_start_point(first).unwrap();
    let bridge = body
        .mev(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            p,
            EdgeCurveSpec::line_between(u, p),
            tol(),
        )
        .unwrap();
    body.kemr(bridge.he_plus, bridge.he_minus).unwrap().ring
}

/// The island's run from corner `r`: every edge but the one closing it
/// back to corner `r`, grown as a strut chain in an empty ring of
/// `face`. The run's first and last forward halves, and the closing
/// edge.
fn run_from(
    body: &mut Body<f64>,
    (face, surface): (FaceKey, SurfaceKey),
    island: &Island,
    r: usize,
) -> ((HalfEdgeKey, HalfEdgeKey), IslandEdge) {
    let n = island.edges.len();
    let ring = empty_ring(body, face, island.corners[r]);
    let mut halves: Vec<crate::MevCreated> = Vec::new();
    for k in 0..n - 1 {
        let e = &island.edges[(r + k) % n];
        let spec = edge_spec(body, surface, e);
        let site = match halves.last() {
            None => MevSite::Lone { r#loop: ring },
            Some(h) => MevSite::Fan {
                he1: h.he_minus,
                he2: h.he_minus,
            },
        };
        halves.push(
            body.mev(site, island.corners[(r + k + 1) % n], spec, tol())
                .unwrap(),
        );
    }
    let (first, last) = (halves[0].he_plus, halves[n - 2].he_plus);
    ((first, last), island.edges[(r + n - 1) % n].clone())
}

/// `island` traversed the other way round.
fn reversed(island: Island) -> Island {
    let n = island.edges.len();
    Island {
        corners: (0..n).map(|k| island.corners[(n - k) % n]).collect(),
        edges: (0..n)
            .map(|k| {
                let e = &island.edges[n - 1 - k];
                IslandEdge {
                    params: (e.params.1, e.params.0),
                    ..e.clone()
                }
            })
            .collect(),
        inside: island.inside,
    }
}

/// The oracle's winding of the run that leaves `island`'s corner `r` on
/// a face of sense `sense`: CCW (Positive) when the island lies on the
/// run's left about the outward normal, read a step `step` to the left
/// of the run's first edge's midpoint.
fn oracle_winding(
    cone: &geom::Surface<f64>,
    sense: bool,
    island: &Island,
    r: usize,
    step: f64,
) -> Sign {
    let e = &island.edges[r];
    let (t0, t1) = e.params;
    let m = e.carrier.mid_point(t0, t1);
    let travel = e.carrier.deriv(0.5 * (t0 + t1)) * (t1 - t0);
    let outward =
        geom_brep::OutwardNormal::from_chart(geom_brep::implicit_gradient(cone, m), sense).vec();
    let left = outward.cross(travel).normalize();
    if (island.inside)(m + left * step) {
        Sign::Positive
    } else {
        Sign::Negative
    }
}

/// The islands every row reads, each with the step its oracle reads at.
fn islands(f: Frame, cone: &geom::Surface<f64>) -> Vec<(&'static str, Island, f64)> {
    vec![
        ("lune", lune(f, cone, 1.0), 1e-4 * f.s),
        ("lune by the apex", lune(f, cone, 0.1), 1e-5 * f.s),
        ("sector", sector(f, cone), 1e-4 * f.s),
    ]
}

/// **The island a cone ring's run walls off winds as its oracle says**:
/// every island, from every corner and in both directions, on both
/// nappes, in three frames at two scales (1 and 1e-3) and at both
/// senses — the island on the far
/// side of the closing chord's section or on its apex side, near the
/// apex or not, closed by an ellipse, a parallel, or a ruling (straight,
/// and as a line segment). Both windings are read.
#[test]
fn a_cone_ring_run_winds_its_island_as_the_oracle_does() {
    let mut seen = [0usize; 2];
    let scales = scales(1e-3, 0.1);
    for f in frames()
        .into_iter()
        .flat_map(|f| scales.iter().map(move |&s| f.scaled(s)))
    {
        for mirror in [false, true] {
            for sense in [true, false] {
                let surface = cone(f, mirror);
                for back in [false, true] {
                    for (name, island, step) in islands(f, &surface) {
                        let island = if back { reversed(island) } else { island };
                        for r in 0..island.edges.len() {
                            let want = oracle_winding(&surface, sense, &island, r, step);
                            let (mut body, face, key) = sheet(f, surface.clone(), sense);
                            let (run, closing) = run_from(&mut body, (face, key), &island, r);
                            let spec = edge_spec(&mut body, key, &closing);
                            let label = format!(
                                "{name}, mirror {mirror}, sense {sense}, back {back}, from {r}"
                            );
                            let mut closings = vec![Some(spec)];
                            if closing.plane.is_none() {
                                closings.push(None);
                            }
                            for closing in closings {
                                let got =
                                    path_island_winding(&body, face, run, closing.as_ref(), band())
                                        .unwrap()
                                        .unwrap();
                                assert_eq!(got, want, "{label}, closed by {closing:?}");
                            }
                            seen[usize::from(want == Sign::Positive)] += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(seen[0] > 0 && seen[1] > 0, "both windings read: {seen:?}");
}

/// A point of the sheet inside `island` and one outside it, off its
/// boundary: the first of a grid of the sheet's points the oracle reads
/// each way.
fn bystanders(f: Frame, island: &Island) -> (Point3<f64>, Point3<f64>) {
    let grid: Vec<_> = [0.08, 0.14, 0.2, 0.25, 0.4, 0.9, 1.4, 1.8, 2.2, 2.6]
        .into_iter()
        .flat_map(|h| (0..9).map(move |k| on_nappe(f, h, -1.0 + 0.25 * f64::from(k))))
        .collect();
    let find = |inside: bool| {
        *grid
            .iter()
            .find(|&&p| (island.inside)(p) == inside)
            .expect("the grid holds a point each way")
    };
    (find(true), find(false))
}

/// **A bystander ring of a cone face moves into the island a run walls
/// off when it lies inside it, and stays when it does not**: the island
/// is walled off by its closing chord (a section arc, or the straight
/// chord along a ruling), and ring re-homing reads each bystander by a
/// path. Every island, from every corner, on both nappes, in three
/// frames at two scales (1 and 1e-3); no reading refuses.
#[test]
fn a_ring_on_a_cone_face_is_re_homed_by_a_path() {
    let scales = scales(1e-3, 0.1);
    for f in frames()
        .into_iter()
        .flat_map(|f| scales.iter().map(move |&s| f.scaled(s)))
    {
        for mirror in [false, true] {
            let surface = cone(f, mirror);
            for (name, island, _) in islands(f, &surface) {
                let (p_in, p_out) = bystanders(f, &island);
                for r in 0..island.edges.len() {
                    let label = format!("{name}, mirror {mirror}, from {r}");
                    let (mut body, face, key) = sheet(f, surface.clone(), true);
                    let ring_in = empty_ring(&mut body, face, p_in);
                    let ring_out = empty_ring(&mut body, face, p_out);
                    let (run, closing) = run_from(&mut body, (face, key), &island, r);
                    let after = body.get_half_edge(run.1).unwrap().next;
                    let site = MefSite::Chords {
                        he1: run.0,
                        he2: after,
                    };
                    let made = match closing.plane {
                        Some(_) => {
                            let back = IslandEdge {
                                params: (closing.params.1, closing.params.0),
                                ..closing
                            };
                            let spec = edge_spec(&mut body, key, &back);
                            body.mef(site, spec, FaceSurface::Shared { key, sense: true }, tol())
                        }
                        None => body.mef_chord(site, tol()),
                    }
                    .unwrap();
                    let remainder = body.get_half_edge(after).unwrap().parent_loop;
                    let mut joiner = ChordJoiner::new(band());
                    joiner
                        .rehome_rings(&mut body, face, made.face, remainder)
                        .unwrap_or_else(|e| panic!("{label}: {e:?}"));
                    let face_of = |ring| body.get_loop(ring).unwrap().face;
                    assert_eq!(
                        face_of(ring_in),
                        made.face,
                        "{label}: the inside ring moves"
                    );
                    assert_eq!(face_of(ring_out), face, "{label}: the outside ring stays");
                }
            }
        }
    }
}

/// A point's height along the frame's axis and its azimuth.
fn height_azimuth(f: Frame, p: Point3<f64>) -> (f64, f64) {
    let w = p - f.o;
    (w.dot(f.z) / f.s, w.dot(f.y).atan2(w.dot(f.x)))
}

/// The offsets a path is moved off a corner by, from the band's zero
/// threshold to ten times its escalation threshold, geometrically.
fn offsets_through_the_band() -> impl Iterator<Item = f64> {
    let b = band();
    let (lo, hi) = (b.zero(), 10.0 * b.escalate());
    let steps = 120;
    (0..=steps).map(move |k| lo * (hi / lo).powf(f64::from(k) / f64::from(steps)))
}

/// **A path whose reading escalates by an island's corner says nothing,
/// and the next path winds the island.** The sector's run from its
/// corner on the low ellipse at 0.9, closed by its ruling at 0.9, on a
/// sheet whose ruling at `−t` passes a hair outside the sector's corner
/// at height 2.2 and azimuth −0.6. The first path, down that ruling from
/// the sheet's first outer-loop point and round the parallel through the
/// chord's midpoint (arriving across the ruling, so its lean is
/// decided), reads the corner's arcs in the escalation band at some
/// offset. The winding still reads as the oracle says.
#[test]
fn a_path_escalating_by_a_corner_hands_the_winding_to_the_next() {
    let f = frames()[0];
    let surface = cone(f, false);
    let island = sector(f, &surface);
    let want = oracle_winding(&surface, true, &island, 2, 1e-4);
    let (h_c, az_c) = height_azimuth(f, island.corners[0]);
    let rho = h_c * FRAC_PI_6.tan();
    let quadric = Quadric::of(&surface).unwrap();
    let mut escalated = 0;
    for delta in offsets_through_the_band() {
        let (mut body, face, key) = sheet_to(f, surface.clone(), true, delta / rho - az_c);
        let (run, closing) = run_from(&mut body, (face, key), &island, 2);
        assert!(
            closing.plane.is_none(),
            "the closing chord is the ruling at 0.9"
        );
        let spec = edge_spec(&mut body, key, &closing);
        let cycle = body.loop_cycle(run.0).unwrap();
        let end = cycle.iter().position(|&he| he == run.1).unwrap();
        let mut arcs = run_loop_arcs(&body, face, (&surface, &quadric), &cycle[..=end]).unwrap();
        arcs.push(LoopArc::of(&spec.carrier, (spec.param_start, spec.param_end)).unwrap());
        let q = spec.carrier.mid_point(spec.param_start, spec.param_end);
        let w = outer_references(&body, face, &[]).unwrap()[0];
        let first = quadric.paths((w, q), band()).unwrap().remove(0);
        if path_parity(&first, &arcs, Some(arcs.len() - 1), band()).is_err() {
            escalated += 1;
            let got = path_island_winding(&body, face, run, Some(&spec), band());
            assert!(
                matches!(got, Ok(Ok(sign)) if sign == want),
                "offset {delta:e}: {got:?}, the oracle {want:?}"
            );
        }
    }
    assert!(
        escalated > 0,
        "no offset puts the first path in the escalation band"
    );
}

/// Walls the island off by its closing edge (`closing`, run back from the
/// run's last corner): a section arc's spec, or the straight chord along
/// a ruling. The new face, and the ring the run leaves.
fn wall_off(
    body: &mut Body<f64>,
    key: SurfaceKey,
    run: (HalfEdgeKey, HalfEdgeKey),
    closing: IslandEdge,
) -> (FaceKey, LoopKey) {
    let after = body.get_half_edge(run.1).unwrap().next;
    let site = MefSite::Chords {
        he1: run.0,
        he2: after,
    };
    let made = if closing.plane.is_some() {
        let back = IslandEdge {
            params: (closing.params.1, closing.params.0),
            ..closing
        };
        let spec = edge_spec(body, key, &back);
        let shared = FaceSurface::Shared { key, sense: true };
        body.mef(site, spec, shared, tol())
    } else {
        body.mef_chord(site, tol())
    }
    .unwrap();
    (made.face, body.get_half_edge(after).unwrap().parent_loop)
}

/// **A path whose reading escalates by an island's corner says nothing,
/// and the next path re-homes the ring.** A bystander outside the lune,
/// on the ruling a hair past its second corner; and one inside the
/// sector, on a sheet whose high rim passes a hair above the sector's
/// top corner. The first path from each — along the bystander's ruling,
/// and round the first outer-loop point's parallel — reads the corner's
/// edges in the escalation band at some offset, and the next path puts
/// it where the oracle does. Both answers are read, so an escalation
/// taken for either goes red.
#[test]
fn a_path_escalating_by_a_corner_hands_re_homing_to_the_next() {
    let f = frames()[0];
    let surface = cone(f, false);
    let quadric = Quadric::of(&surface).unwrap();
    let (lune, sector) = (lune(f, &surface, 1.0), sector(f, &surface));
    let mut escalated = [0; 2];
    let mut read = |island: &Island,
                    p: Point3<f64>,
                    (mut body, face, key): (Body<f64>, FaceKey, SurfaceKey),
                    label: String| {
        let inside = (island.inside)(p);
        let ring = empty_ring(&mut body, face, p);
        let (run, closing) = run_from(&mut body, (face, key), island, 0);
        let (newf, remainder) = wall_off(&mut body, key, run, closing);
        let cycle = outer_cycle(&body, newf).unwrap().unwrap();
        let arcs = run_loop_arcs(&body, newf, (&surface, &quadric), &cycle).unwrap();
        let skip: Vec<_> = cycle
            .iter()
            .map(|&he| body.get_half_edge(he).unwrap().edge)
            .collect();
        let w = outer_references(&body, face, &skip).unwrap()[0];
        let first = quadric.paths((p, w), band()).unwrap().remove(0);
        if path_parity(&first, &arcs, None, band()).is_ok() {
            return;
        }
        escalated[usize::from(inside)] += 1;
        ChordJoiner::new(band())
            .rehome_rings(&mut body, face, newf, remainder)
            .unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert_eq!(
            body.get_loop(ring).unwrap().face,
            if inside { newf } else { face },
            "{label}: inside {inside}"
        );
    };
    // Outside the lune, on the ruling a hair past its second corner.
    let (h_c, az_c) = height_azimuth(f, lune.corners[1]);
    let rho = h_c * FRAC_PI_6.tan();
    for delta in offsets_through_the_band() {
        for h in [1.0, 2.0] {
            let p = on_nappe(f, h, az_c - delta * h / (h_c * rho));
            let sheet = sheet(f, surface.clone(), true);
            read(
                &lune,
                p,
                sheet,
                format!("lune, offset {delta:e}, height {h}"),
            );
        }
    }
    // Inside the sector, on a sheet whose high rim passes a hair above
    // its top corner.
    let top = height_azimuth(f, sector.corners[0]).0;
    for delta in offsets_through_the_band() {
        let p = on_nappe(f, 1.4, 0.15);
        let sheet = sheet_up_to(f, surface.clone(), true, (1.2, top + delta));
        read(&sector, p, sheet, format!("sector, offset {delta:e}"));
    }
    assert!(
        escalated.iter().all(|&n| n > 0),
        "first paths in the escalation band, outside and inside: {escalated:?}"
    );
}

/// **The arrival arc reads the closing chord's second meeting with it.**
/// The sector's run from its corner on the low ellipse at −0.6, closed by
/// that ellipse, whose midpoint sits just off its lowest point: the
/// parallel through the midpoint meets the ellipse again at about the
/// mirror azimuth, inside the closing arc. The first path, down the
/// sheet's ruling at −1.2 from its first outer-loop point and round that
/// parallel, crosses the island's boundary there and nowhere else before
/// it arrives, so it decides by itself, odd; the winding is the
/// oracle's.
#[test]
fn the_arrival_arc_reads_its_second_meeting_with_the_closing_chord() {
    for f in scales(1e-3, 0.1)
        .into_iter()
        .map(|s| frames()[usize::from(s < 1.0)].scaled(s))
    {
        let surface = cone(f, false);
        let island = sector(f, &surface);
        let (mut body, face, key) = sheet(f, surface.clone(), true);
        let (run, closing) = run_from(&mut body, (face, key), &island, 3);
        assert!(
            closing.plane.is_some(),
            "the closing chord is the low ellipse"
        );
        let spec = edge_spec(&mut body, key, &closing);
        let quadric = Quadric::of(&surface).unwrap();
        let cycle = body.loop_cycle(run.0).unwrap();
        let end = cycle.iter().position(|&he| he == run.1).unwrap();
        let mut arcs = run_loop_arcs(&body, face, (&surface, &quadric), &cycle[..=end]).unwrap();
        arcs.push(LoopArc::of(&spec.carrier, (spec.param_start, spec.param_end)).unwrap());
        let q = spec.carrier.mid_point(spec.param_start, spec.param_end);
        let w = outer_references(&body, face, &[]).unwrap()[0];
        assert!(
            !(island.inside)(w),
            "the outer-loop point is off the island"
        );
        let first = quadric.paths((w, q), band()).unwrap().remove(0);
        assert_eq!(
            path_parity(&first, &arcs, Some(arcs.len() - 1), band()),
            Ok(Some(true)),
            "the arrival arc's second root is the one crossing"
        );
        let want = oracle_winding(&surface, true, &island, 3, 1e-4 * f.s);
        let got = path_island_winding(&body, face, run, Some(&spec), band());
        assert!(
            matches!(got, Ok(Ok(sign)) if sign == want),
            "{got:?}, the oracle {want:?}"
        );
    }
}

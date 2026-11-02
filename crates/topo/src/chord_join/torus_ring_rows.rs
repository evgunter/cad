//! **A ring on a torus face winds its island and re-homes its rings
//! without a chart** ([`super::path_island_winding`],
//! [`super::path_ring_side`]), by the paths of
//! [`crate::ring_path::Quadric::Torus`]. The boolean reaches the island
//! winding only past the plane × torus chord, so these rows build the
//! lane's input on a torus sheet: a face of the torus between two
//! meridians and two parallels, a ring of it whose run is an island's
//! boundary less one edge, and that edge as the closing chord. Each
//! reading is held to the islands' membership oracles
//! ([`crate::ring_path::torus_islands`]).

use super::*;
use crate::construction::MevSite;
use crate::ring_path::cone_islands::{Frame, band, frames, plane, scales};
use crate::ring_path::torus_islands::{
    Island, IslandEdge, crest_islands, islands, meridian, on_torus, parallel, spiric_islands, torus,
};

fn tol() -> Tol {
    Tol::witness()
}

/// The sheet's chart rectangle: azimuths `±U`, tube angles `±V`.
const U: f64 = 2.0;
const V: f64 = 3.0;

/// The certified spec of `e` on the torus `surface`: the section arc of
/// its plane, run forward from its corner (on the reversed carrier where
/// its parameter falls). The tube's top or bottom circle, whose plane
/// touches the torus, is no transverse section: it is described in the
/// torus's chart.
fn edge_spec(body: &mut Body<f64>, surface: SurfaceKey, e: &IslandEdge) -> EdgeCurveSpec<f64> {
    let (t0, t1) = e.params;
    let (o, n) = e.plane;
    let (carrier, t0, t1) = if t1 > t0 {
        (e.carrier.clone(), t0, t1)
    } else {
        (e.carrier.reversed().unwrap(), -t0, -t1)
    };
    let Some(&geom::Surface::Torus { major_radius, .. }) = body.get_surface(surface) else {
        panic!("the sheet is a torus");
    };
    let touching = matches!(
        e.carrier,
        geom::Curve3::Circle { radius, .. } if (radius - major_radius).abs() < 1e-9 * major_radius
    );
    let description = if touching {
        geom_brep::EdgeDescriptionSpec::Chart {
            surface,
            image: None,
            wrap: false,
            declared: None,
        }
    } else {
        geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(surface, body.add_surface(plane(o, n))),
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
        }
    };
    EdgeCurveSpec {
        description,
        carrier,
        param_start: t0,
        param_end: t1,
    }
}

/// **The face of `torus` over the chart rectangle `(±U) × (±V)`**, of
/// sense `sense`, and its surface: from its corner `(−U, −V)` along the
/// parallel, up the meridian at `U`, back along the parallel at `V` and
/// down the meridian at `−U`. It spans more than half a turn about the
/// axis and the tube's outer side, top, bottom and most of its inner
/// side.
fn sheet(f: Frame, torus: geom::Surface<f64>, sense: bool) -> (Body<f64>, FaceKey, SurfaceKey) {
    let mut body = Body::<f64>::new();
    let start = on_torus(f, -U, -V);
    let seed = body.mvfs(start, sense).unwrap();
    let surface = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: torus,
                sense,
            },
        )
        .unwrap();
    let low = edge_spec(&mut body, surface, &parallel(f, -V, (-U, U)));
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            on_torus(f, U, -V),
            low,
            tol(),
        )
        .unwrap();
    let side = edge_spec(&mut body, surface, &meridian(f, U, (-V, V)));
    let up = body
        .mev(
            MevSite::Fan {
                he1: first.he_minus,
                he2: first.he_minus,
            },
            on_torus(f, U, V),
            side,
            tol(),
        )
        .unwrap();
    let high = edge_spec(&mut body, surface, &parallel(f, V, (U, -U)));
    let last = body
        .mev(
            MevSite::Fan {
                he1: up.he_minus,
                he2: up.he_minus,
            },
            on_torus(f, -U, V),
            high,
            tol(),
        )
        .unwrap();
    let back = edge_spec(&mut body, surface, &meridian(f, -U, (-V, V)));
    let face = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: last.he_minus,
            },
            back,
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
    let mut halves: Vec<crate::construction::MevCreated> = Vec::new();
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
    torus: &geom::Surface<f64>,
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
        geom_brep::OutwardNormal::from_chart(geom_brep::implicit_gradient(torus, m), sense).vec();
    let left = outward.cross(travel).normalize();
    if (island.inside)(m + left * step) {
        Sign::Positive
    } else {
        Sign::Negative
    }
}

/// The frames every row reads, at scale 1 and, where the band reads it,
/// 1e-3 (the islands' narrowest feature is 0.6 units).
fn scaled_frames() -> Vec<Frame> {
    let scales = scales(1e-3, 0.6);
    frames()
        .into_iter()
        .flat_map(|f| scales.iter().map(move |&s| f.scaled(s)))
        .collect()
}

/// **The island a torus ring's run walls off winds as its oracle says**:
/// every island (on the outer side, the inner side, over the tube's top,
/// between one plane's two parallels, and wider than half a turn), from
/// every corner and in both directions, with the torus's axis either
/// way, in three frames at two scales and at both senses — closed by a
/// meridian or a parallel, outer or inner. Both windings are read. Every
/// decided path from every outer-loop point reads the oracle's winding
/// too, not only the first one the lane takes, so an arc of the run the
/// reading missed would show on the paths that cross it.
#[test]
fn a_torus_ring_run_winds_its_island_as_the_oracle_does() {
    winds_as_the_oracle(islands);
}

/// The winding rows over the islands `islands` names in each frame.
fn winds_as_the_oracle(islands: fn(Frame) -> Vec<(&'static str, Island)>) {
    let mut seen = [0usize; 2];
    let mut paths = 0usize;
    for f in scaled_frames() {
        for mirror in [false, true] {
            for sense in [true, false] {
                let surface = torus(f, mirror);
                for back in [false, true] {
                    for (name, island) in islands(f) {
                        let island = if back { reversed(island) } else { island };
                        for r in 0..island.edges.len() {
                            let want = oracle_winding(&surface, sense, &island, r, 1e-4 * f.s);
                            let (mut body, face, key) = sheet(f, surface.clone(), sense);
                            let (run, closing) = run_from(&mut body, (face, key), &island, r);
                            let spec = edge_spec(&mut body, key, &closing);
                            let got = path_island_winding(&body, face, run, Some(&spec), band());
                            assert!(
                                matches!(got, Ok(Ok(sign)) if sign == want),
                                "{name}, mirror {mirror}, sense {sense}, back {back}, from {r}: \
                                 {got:?}, the oracle {want:?}"
                            );
                            seen[usize::from(want == Sign::Positive)] += 1;
                            let reading = IslandReading::new(&body, face, run, Some(&spec), band())
                                .unwrap()
                                .unwrap();
                            for (k, path) in reading
                                .paths(&body, face, band())
                                .unwrap()
                                .into_iter()
                                .enumerate()
                            {
                                if let Ok(Some(sign)) = path.and_then(|p| reading.read(p, band())) {
                                    assert_eq!(
                                        sign, want,
                                        "{name}, mirror {mirror}, sense {sense}, back {back}, \
                                         from {r}, path {k}: the oracle {want:?}"
                                    );
                                    paths += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(seen[0] > 0 && seen[1] > 0, "both windings read: {seen:?}");
    let runs = seen[0] + seen[1];
    assert!(paths > 4 * runs, "{paths} paths decided over {runs} runs");
}

/// A point of the sheet inside `island` and one outside it, off its
/// boundary: the first of a grid of the sheet's points the oracle reads
/// each way.
fn bystanders(f: Frame, island: &Island) -> (Point3<f64>, Point3<f64>) {
    let grid: Vec<_> = (0..9)
        .flat_map(|j| {
            (0..9).map(move |k| {
                on_torus(
                    f,
                    -1.85 + 0.45 * f64::from(k) + 0.013,
                    -2.85 + 0.7 * f64::from(j) + 0.017,
                )
            })
        })
        .collect();
    let find = |inside: bool| {
        *grid
            .iter()
            .find(|&&p| (island.inside)(p) == inside)
            .expect("the grid holds a point each way")
    };
    (find(true), find(false))
}

/// `island` walled off in `face` from corner `r`: its run, closed by
/// its last edge, and the face's rings re-homed. The island's face.
fn wall_off(
    body: &mut Body<f64>,
    (face, key): (FaceKey, SurfaceKey),
    island: &Island,
    r: usize,
) -> Result<FaceKey, SplitJoinError> {
    let (run, closing) = run_from(body, (face, key), island, r);
    let after = body.get_half_edge(run.1).unwrap().next;
    let site = MefSite::Chords {
        he1: run.0,
        he2: after,
    };
    let back = IslandEdge {
        params: (closing.params.1, closing.params.0),
        ..closing
    };
    let spec = edge_spec(body, key, &back);
    let made = body
        .mef(site, spec, FaceSurface::Shared { key, sense: true }, tol())
        .unwrap();
    let remainder = body.get_half_edge(after).unwrap().parent_loop;
    ChordJoiner::new(band()).rehome_rings(body, face, made.face, remainder)?;
    Ok(made.face)
}

/// **A bystander ring of a torus face moves into the island a run walls
/// off when it lies inside it, and stays when it does not**: the island
/// is walled off by its closing chord, and ring re-homing reads each
/// bystander by a path. Every island, from every corner, with the
/// torus's axis either way, in three frames at two scales; no reading
/// refuses.
#[test]
fn a_ring_on_a_torus_face_is_re_homed_by_a_path() {
    re_homes_as_the_oracle(islands);
}

/// The re-homing rows over the islands `islands` names in each frame.
fn re_homes_as_the_oracle(islands: fn(Frame) -> Vec<(&'static str, Island)>) {
    for f in scaled_frames() {
        for mirror in [false, true] {
            let surface = torus(f, mirror);
            for (name, island) in islands(f) {
                let (p_in, p_out) = bystanders(f, &island);
                for r in 0..island.edges.len() {
                    let label = format!("{name}, mirror {mirror}, from {r}");
                    let (mut body, face, key) = sheet(f, surface.clone(), true);
                    let ring_in = empty_ring(&mut body, face, p_in);
                    let ring_out = empty_ring(&mut body, face, p_out);
                    let made = wall_off(&mut body, (face, key), &island, r)
                        .unwrap_or_else(|e| panic!("{label}: {e:?}"));
                    let face_of = |ring| body.get_loop(ring).unwrap().face;
                    assert_eq!(face_of(ring_in), made, "{label}: the inside ring moves");
                    assert_eq!(face_of(ring_out), face, "{label}: the outside ring stays");
                }
            }
        }
    }
}

/// **A run on a spiric oval winds its island and re-homes its rings as
/// the oracle says**: strips between two planes parallel to the axis,
/// on the tube's outer side on either side of the axis's trace and on
/// its inner side, one whose planes stand either side of the axis, and
/// one bounded by an oval and a meridian
/// ([`crate::ring_path::torus_islands::spiric_islands`]), each oval's
/// own loop told from the one across the trace. From every corner, so
/// with an oval in the run and as the closing chord, in both directions,
/// at both senses, with the torus's axis either way, in three frames at
/// two scales; every decided path agrees, and a bystander ring moves
/// into the island exactly when it lies inside.
#[test]
fn a_run_on_a_spiric_oval_winds_its_island_as_the_oracle_does() {
    winds_as_the_oracle(spiric_islands);
    re_homes_as_the_oracle(spiric_islands);
}

/// **A ring standing on the run's planes, off its arcs, stays where it
/// is**: the strip between the planes `x = 0.2` and `x = 1` on one side
/// of the axis's trace walled off, and a ring at each corner of the same
/// strip across the trace, which lies on an oval's plane on the loop
/// across the trace and on a parallel's plane, on that parallel's circle
/// out of the run's span: the shape of a bar through a torus's hole,
/// whose two pierce islands stand on one face's planes. Each path from
/// the ring starts there, with a lead of no length on both planes, which
/// is read as the point it is and passed over. From every corner of the
/// run, with the torus's axis either way, in three frames at two scales.
#[test]
fn a_ring_on_the_runs_planes_off_its_arcs_stays() {
    use crate::ring_path::torus_islands::oval_strip;
    for f in scaled_frames() {
        for mirror in [false, true] {
            let surface = torus(f, mirror);
            let island = oval_strip(f, (0.2, 1.0), true, (-0.5, 0.8));
            let across = oval_strip(f, (0.2, 1.0), false, (-0.5, 0.8));
            for &p in &across.corners {
                for r in 0..island.edges.len() {
                    let label = format!("mirror {mirror}, ring at {p:?}, from {r}");
                    let (mut body, face, key) = sheet(f, surface.clone(), true);
                    let ring = empty_ring(&mut body, face, p);
                    wall_off(&mut body, (face, key), &island, r)
                        .unwrap_or_else(|e| panic!("{label}: {e:?}"));
                    assert_eq!(
                        body.get_loop(ring).unwrap().face,
                        face,
                        "{label}: the ring stays"
                    );
                }
            }
        }
    }
}

/// **A run on the tube's top or bottom circle winds its island and
/// re-homes its rings as the oracle says**: that parallel, of radius
/// `R`, lies in a plane that touches the torus along it, so it is read
/// across the cylinder `ρ = R`, which cuts the torus in it transversally
/// ([`crate::ring_path::LoopArc::Crest`]). Islands up to the top, down
/// to the bottom, from the top inward, and between the two (a donut's
/// outer face), from every corner, so with the circle in the run and as
/// the closing chord, in both directions, at both senses, with the
/// torus's axis either way, in three frames at two scales; every decided
/// path agrees, and a bystander ring moves into the island exactly when
/// it lies inside.
#[test]
fn a_run_on_the_tubes_top_or_bottom_circle_winds_its_island_as_the_oracle_does() {
    winds_as_the_oracle(crest_islands);
    re_homes_as_the_oracle(crest_islands);
}

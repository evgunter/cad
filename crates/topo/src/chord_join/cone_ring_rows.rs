//! **A ring on a cone face winds its island and re-homes its rings
//! without a chart** ([`super::path_island_winding`],
//! [`super::path_ring_side`]). No boolean reaches a cone face's ring lane
//! yet — the operand gate refuses a cone operand
//! (`work/germ/boolean-sector-algebra-has-no-cone-arm.md`) — so these
//! rows build the lane's input on a cone sheet: a face of the cone
//! between two rims and two rulings, a ring of it whose run is an
//! island's boundary less one edge, and that edge as the closing chord.
//! Each reading is held to the islands' membership oracles
//! ([`crate::ring_path::cone_islands`]).

use super::*;
use crate::MevSite;
use crate::ring_path::cone_islands::band;
use crate::ring_path::cone_islands::{
    Frame, Island, IslandEdge, cone, frames, lune, on_nappe, plane, sector,
};
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
            radius: h * FRAC_PI_6.tan(),
            u_ref: f.x,
        },
        params: (t0, t1),
        plane: Some((centre, f.z)),
    }
}

/// **A face of `cone` between its rims at heights 0.05 and 3 and its
/// rulings at azimuths ±1.2**, of sense `sense`, and its surface.
fn sheet(f: Frame, cone: geom::Surface<f64>, sense: bool) -> (Body<f64>, FaceKey, SurfaceKey) {
    let (lo, hi, t) = (0.05, 3.0, 1.2);
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
    let chart = geom_brep::implicit_gradient(cone, m);
    let outward = if sense { chart } else { -chart };
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
        ("lune", lune(f, cone, 1.0), 1e-4),
        ("lune by the apex", lune(f, cone, 0.1), 1e-5),
        ("sector", sector(f, cone), 1e-4),
    ]
}

/// **The island a cone ring's run walls off winds as its oracle says**:
/// every island, from every corner and in both directions, on both
/// nappes, in three frames and at both senses — the island on the far
/// side of the closing chord's section or on its apex side, near the
/// apex or not, closed by an ellipse, a parallel, or a ruling (straight,
/// and as a line segment). Both windings are read.
#[test]
fn a_cone_ring_run_winds_its_island_as_the_oracle_does() {
    let mut seen = [0usize; 2];
    for f in frames() {
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
/// path. Every island, from every corner, on both nappes and in three
/// frames; no reading refuses.
#[test]
fn a_ring_on_a_cone_face_is_re_homed_by_a_path() {
    for f in frames() {
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

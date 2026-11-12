//! **A ring-lane island on a spline wall winds by paths along the
//! wall's iso curves** ([`super::path_island_winding_closed_by`]), and
//! re-homes its bystanders the same way ([`super::path_ring_side`]): a
//! flat spline sheet holding an island of four straight spline edges,
//! each the certified section of the wall by a plane, its last side the
//! closing chord.

test_utils::gated_to![
    "crates/topo/src/chord_join.rs",
    "crates/topo/src/ring_path.rs",
    "crates/topo/src/ring_path/spline_wall.rs",
    "crates/geom-brep/src/curve_pair.rs",
    "crates/geom-brep/src/curve_pair/",
];

use super::sibling_escalation_rows::{band, tol};
use super::*;
use crate::euler::{FaceSurface, MefSite, MevSite};
use geom_core::spline::KnotVector;

/// The island's corners on the flat sheet `(u, v, 0)`, counterclockwise
/// about `+z`, the sheet's chart normal.
const CORNERS: [(f64, f64); 4] = [(0.3, 0.3), (0.7, 0.3), (0.7, 0.7), (0.3, 0.7)];

fn on((u, v): (f64, f64)) -> Point3<f64> {
    Point3::new(u, v, 0.0)
}

/// The flat spline sheet `(u, v, 0)` over the chart's square
/// `[0.05, 0.95]²`, one bilinear span, each side a straight edge stated
/// by the iso line it lies on. The body and the sheet's face.
fn sheet() -> (Body<f64>, FaceKey) {
    let kv = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let chart = geom::NurbsSurface::new(
        kv(),
        kv(),
        vec![
            on((0.0, 0.0)),
            on((0.0, 1.0)),
            on((1.0, 0.0)),
            on((1.0, 1.0)),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let uv = [(0.05, 0.05), (0.95, 0.05), (0.95, 0.95), (0.05, 0.95)];
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(on(uv[0]), true).unwrap();
    let key = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(chart)),
                sense: true,
            },
        )
        .unwrap();
    let edge = |a: (f64, f64), b: (f64, f64)| {
        let line = EdgeCurveSpec::line_between(on(a), on(b));
        let len = line.param_end;
        EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart_image(
                key,
                geom_brep::Pcurve::IsoLine {
                    p0: geom_core::Point2::new(a.0, a.1),
                    pl: geom_core::Vec2::new((b.0 - a.0) / len, (b.1 - a.1) / len),
                },
            ),
            ..line
        }
    };
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            on(uv[1]),
            edge(uv[0], uv[1]),
            tol(),
        )
        .unwrap();
    let mut tip = first;
    for k in 2..4 {
        tip = body
            .mev(
                MevSite::Fan {
                    he1: tip.he_minus,
                    he2: tip.he_minus,
                },
                on(uv[k]),
                edge(uv[k - 1], uv[k]),
                tol(),
            )
            .unwrap();
    }
    let after = body.get_half_edge(tip.he_plus).unwrap().next;
    let face = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: after,
            },
            edge(uv[0], uv[3]),
            FaceSurface::Shared { key, sense: true },
            tol(),
        )
        .unwrap()
        .face;
    (body, face)
}

/// The plane through `a` and `b` square to the sheet, a surface the
/// body gains.
fn plane_through(
    body: &mut Body<f64>,
    (a, b): (Point3<f64>, Point3<f64>),
) -> crate::geometry::SurfaceKey {
    let along = (b - a).normalize();
    body.add_surface(geom::Surface::Plane {
        origin: a,
        normal: Vec3::unit_z().cross(along),
        u_ref: along,
    })
}

/// The certified spec of the straight spline from `a` to `b` on the
/// sheet, described as the section of the wall `wall` by `plane`.
fn side(
    (wall, plane): (crate::geometry::SurfaceKey, crate::geometry::SurfaceKey),
    (a, b): (Point3<f64>, Point3<f64>),
) -> EdgeCurveSpec<f64> {
    let net = geom::NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![a, b],
        vec![1.0, 1.0],
    )
    .unwrap();
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(net));
    EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(wall, plane),
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(0.0, 1.0)),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.0,
    }
}

/// An empty ring of `face` at the chart point `p`: a strut from the
/// outer loop's first vertex, straight on the flat chart and stated by
/// its image there, then killed.
fn ring_at(
    body: &mut Body<f64>,
    face: FaceKey,
    wall: crate::geometry::SurfaceKey,
    p: (f64, f64),
) -> LoopKey {
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("outer is a cycle");
    };
    let from = body.half_edge_start_point(first).unwrap();
    let line = EdgeCurveSpec::line_between(from, on(p));
    let len = line.param_end;
    let image = geom::NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, len, len], 1).unwrap(),
        vec![
            geom_core::Point2::new(from.x, from.y),
            geom_core::Point2::new(p.0, p.1),
        ],
        vec![1.0; 2],
    )
    .unwrap();
    let strut = body
        .mev(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            on(p),
            EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::chart_image(
                    wall,
                    geom_brep::Pcurve::General(std::sync::Arc::new(image)),
                ),
                ..line
            },
            tol(),
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap().ring
}

/// The flat sheet with a ring through `corners` in order, its last side
/// left to the closing chord, and a bystander ring at each of `points`:
/// the island's winding as the ring lane reads it, then, after the
/// chord's `mef`, whether each bystander moved to the island.
fn island(
    corners: [(f64, f64); 4],
    points: &[(f64, f64)],
) -> (Result<Sign, Indeterminate>, Vec<bool>) {
    let (mut body, face) = sheet();
    let wall = body.get_face(face).unwrap().surface;
    // A null edge holds the sheet open, as a boolean's wall is held
    // while its chords are joined: a complete spline face takes no new
    // half-edge.
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("outer is a cycle");
    };
    body.mev_null(
        MevSite::Fan {
            he1: first,
            he2: first,
        },
        crate::null::NewVertexSide::Above,
    )
    .unwrap();
    let rings: Vec<LoopKey> = points
        .iter()
        .map(|&p| ring_at(&mut body, face, wall, p))
        .collect();
    let ring = ring_at(&mut body, face, wall, corners[0]);
    let mut tip: Option<crate::construction::MevCreated> = None;
    let mut first = None;
    for k in 0..3 {
        let site = match &tip {
            None => MevSite::Lone { r#loop: ring },
            Some(h) => MevSite::Fan {
                he1: h.he_minus,
                he2: h.he_minus,
            },
        };
        let ends = (on(corners[k]), on(corners[k + 1]));
        let plane = plane_through(&mut body, ends);
        let spec = side((wall, plane), ends);
        let made = body.mev(site, on(corners[k + 1]), spec, tol()).unwrap();
        first.get_or_insert(made.he_plus);
        tip = Some(made);
    }
    let (first, last) = (first.unwrap(), tip.unwrap().he_plus);
    // The chord's plane, the chord as it closes the run, back to its
    // start, and as `mef` mints it, from the run's first vertex to its
    // last.
    let ends = (on(corners[3]), on(corners[0]));
    let plane = plane_through(&mut body, ends);
    let closing = side((wall, plane), ends);
    let chord = side((wall, plane), (ends.1, ends.0));
    let arc = SplineArc::on_by_construction(&closing.carrier, (0.0, 1.0)).unwrap();
    let wound = path_island_winding_closed_by(
        &body,
        face,
        (first, last),
        (Some(&closing), Some(&arc)),
        band(),
    )
    .unwrap();
    let after = body.get_half_edge(last).unwrap().next;
    let made = body
        .mef(
            MefSite::Chords {
                he1: first,
                he2: after,
            },
            chord,
            FaceSurface::Shared {
                key: wall,
                sense: true,
            },
            tol(),
        )
        .unwrap();
    let remainder = body.get_half_edge(after).unwrap().parent_loop;
    ChordJoiner::new(band())
        .rehome_rings(&mut body, face, made.face, remainder)
        .unwrap();
    let moved = rings
        .into_iter()
        .map(|r| body.get_loop(r).unwrap().face == made.face)
        .collect();
    (wound, moved)
}

/// **An island on a spline wall winds as its corners turn about the
/// wall's outward normal, and its bystanders are placed by the square
/// they stand in or out of**: the run through the corners
/// counterclockwise about the normal winds positive, the same corners
/// taken the other way round negative, and of four bystanders the two
/// inside the island's square move to its face.
#[test]
fn a_spline_wall_island_winds_as_its_corners_turn_and_places_its_bystanders() {
    let points = [(0.5, 0.5), (0.35, 0.65), (0.1, 0.5), (0.85, 0.2)];
    let (wound, moved) = island(CORNERS, &points);
    assert_eq!(wound, Ok(Sign::Positive), "counterclockwise corners");
    assert_eq!(moved, [true, true, false, false], "each bystander's face");
    let [a, b, c, d] = CORNERS;
    let (wound, _) = island([a, d, c, b], &[]);
    assert_eq!(wound, Ok(Sign::Negative), "clockwise corners");
}

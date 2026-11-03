//! **A solid touching itself at a point on a curved face is read over
//! its faces' tangent cones** ([`super::self_touch_verdict`],
//! [`Link::tangent_of`]): a parallelepiped's corner resting on a
//! half-disc post of its own solid (`crate::tier3_tests::half_disc`, a
//! cylinder wall, its chord's plane, and planar caps bounded by arcs),
//! held at the post's corner, inside its bottom arc, or inside its wall,
//! near the origin and a kilometre off it. The corner's edges point off
//! the post (a rest), into it or across it (a crossing), along the
//! wall's ruling (a line the wall keeps), along an arc's tangent (a
//! tangency the bends part one order down, [`crate::link::FaceSector::tangent`]'s
//! second-order reading: off the post or under it a rest, into it
//! refused), or along the wall's circle's tangent (refused).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use crate::test_support_fixtures::mapped_cube;
use geom_core::{Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The post's radius and height.
const R: f64 = 0.8;
const H: f64 = 1.0;
/// The azimuth of the touch inside the arc or the wall, and its height
/// up the wall.
const THETA: f64 = 0.7;
const UP: f64 = 0.4;

/// Where on the post the corner rests.
#[derive(Clone, Copy, Debug)]
enum Seat {
    /// On the post's corner at azimuth 0 on its bottom, tied to its
    /// vertex there.
    Corner,
    /// Inside its bottom arc at [`THETA`], tied to the arc.
    Arc,
    /// Inside its wall at [`THETA`], [`UP`] above the bottom, tied to
    /// the wall.
    Wall,
}

/// The post with its axis corner at `c`, and a parallelepiped cornered
/// on it at `seat`, spanned by `edges`, each given as its components
/// along the wall's radial `n` at the seat's azimuth, the circle's
/// direction `z × n` and `z`. The parallelepiped's shell is moved into
/// the post's solid. Returns the body, the corner, and the ties that
/// hold it at the seat.
fn seat(c: Point3<f64>, at: Seat, edges: [[f64; 3]; 3]) -> (Body<f64>, VertexKey, Ties) {
    let tol = Tol::witness();
    let mut body = crate::tier3_tests::half_disc(R, H, (c.x, c.y, c.z), false, tol);
    let theta = match at {
        Seat::Corner => 0.0,
        Seat::Arc | Seat::Wall => THETA,
    };
    let n = Vec3::new(theta.cos(), theta.sin(), 0.0);
    let z = Vec3::unit_z();
    let up = match at {
        Seat::Wall => UP,
        Seat::Corner | Seat::Arc => 0.0,
    };
    let p = c + n * R + z * up;
    let [mut a, mut b, e] = edges.map(|[x, y, w]| n * x + z.cross(n) * y + z * w);
    if a.cross(b).dot(e) < 0.0 {
        std::mem::swap(&mut a, &mut b);
    }
    let part = mapped_cube::<f64>(move |u, v, w| p + a * u + b * v + e * w, tol);
    let before: BTreeSet<VertexKey> = body.vertices.keys().collect();
    let keep = census_entity_solid(&body, EntityId::Vertex(*before.first().unwrap())).unwrap();
    let gone = crate::instance::graft_disjoint(&mut body, &part).unwrap();
    for sh in body.solids.remove(gone).unwrap().shells {
        body.shells[sh].solid = keep;
        body.solids[keep].shells.push(sh);
    }
    body.solid_provenance.remove(gone);
    let near =
        |body: &Body<f64>, v: VertexKey| (body.points[body.vertices[v].point] - p).norm() < 1e-9;
    let corner = body
        .vertices
        .keys()
        .find(|&v| !before.contains(&v) && near(&body, v))
        .expect("the parallelepiped's corner on the post");
    let ties = match at {
        Seat::Corner => {
            let post = before.iter().copied().find(|&v| near(&body, v)).unwrap();
            Ties {
                vv: vec![(corner, post), (post, corner)],
                ..Ties::default()
            }
        }
        Seat::Arc => {
            let arc = body
                .edges
                .iter()
                .find(|(_, edge)| {
                    matches!(
                        body.curves.get(edge.curve).and_then(crate::null::CurveGeom::certified)
                            .map(|c| c.carrier().clone()),
                        Some(geom::Curve3::Circle(arc)) if (arc.frame.center.z - c.z).abs() < 1e-12
                    )
                })
                .map(|(k, _)| k)
                .expect("the bottom arc");
            Ties {
                ve: vec![(corner, arc)],
                ..Ties::default()
            }
        }
        Seat::Wall => {
            let wall = body
                .faces()
                .find(|(_, f)| {
                    matches!(
                        body.get_surface(f.surface),
                        Some(geom::Surface::Cylinder { .. })
                    )
                })
                .map(|(k, _)| k)
                .expect("the wall");
            Ties {
                vf: vec![(corner, wall)],
                ..Ties::default()
            }
        }
    };
    (body, corner, ties)
}

/// The self-touch reading of the corner as `ties` hold it: what the
/// planar reading answered, and the solid's verdict.
fn read(body: &Body<f64>, corner: VertexKey, ties: &Ties) -> (TouchUnread, SelfTouch) {
    let geo = snapshot(body);
    let keys = touch_keys(&geo, ties, &[corner]).expect("the corner's keys");
    let at = geo.vmap[&corner];
    let s = census_entity_solid(body, EntityId::Vertex(corner)).unwrap();
    let planar = match Link::touch_at_point(body, &geo, ties, s, &keys, at, band()) {
        Ok(_) => panic!("a link with a curved face or edge read as planar"),
        Err(why) => why,
    };
    (
        planar,
        self_touch_verdict(body, &geo, ties, s, &keys, at, band()),
    )
}

/// The rows at `at`: where the corner's edges point and what the
/// reading must make of it, near the origin and a kilometre off it.
/// Red where the planar reading reads a curved face or edge, or a row
/// reads otherwise.
fn rows_hold(at: Seat) {
    use TouchVerdict::Crossing;
    let off = [[0.3, 0.1, -0.05], [0.3, -0.1, 0.1], [0.3, 0.05, -0.1]];
    let into = [[-0.3, 0.1, 0.15], [-0.3, -0.1, 0.1], [-0.2, 0.05, 0.3]];
    let straddle = [[-0.2, 0.2, 0.1], [0.3, -0.1, 0.1], [0.3, 0.05, -0.1]];
    let ruling = [[0.0, 0.0, 0.3], [0.3, -0.1, 0.1], [0.3, 0.15, -0.1]];
    let ruling_into = [[0.0, 0.0, 0.3], [0.3, -0.1, 0.1], [-0.3, 0.15, 0.1]];
    let circle = [[0.0, 0.3, 0.0], [0.3, -0.1, 0.1], [0.3, 0.05, -0.1]];
    // Along the arc's tangent, the other two edges into the post, or
    // inward under its bottom.
    let circle_into = [[0.0, 0.3, 0.0], [-0.3, -0.1, 0.15], [-0.2, 0.05, 0.3]];
    let circle_under = [[0.0, 0.3, 0.0], [-0.3, -0.1, -0.1], [-0.2, 0.05, -0.3]];
    let corner_into = [[-0.3, 0.1, 0.15], [-0.1, 0.3, 0.1], [-0.2, 0.05, 0.3]];
    let under = [[0.0, 0.0, -0.3], [0.3, -0.1, 0.1], [0.3, 0.05, -0.1]];
    // Along the circle, off the wall by three of the band's zero (in
    // band) and by three of its escalation (decided).
    let lift = |by: f64| [[by, 0.3, 0.0], [0.3, -0.1, 0.1], [0.3, 0.05, -0.1]];
    let (zero, escalate) = (band().zero(), band().escalate());
    let rest = |v: &SelfTouch| matches!(v, SelfTouch::Read(TouchVerdict::Rest(_)));
    let in_band = |v: &SelfTouch| *v == SelfTouch::Read(TouchVerdict::Unread(TouchUnread::InBand));
    let crossing = |v: &SelfTouch| *v == SelfTouch::Read(Crossing);
    let tangency = |v: &SelfTouch| *v == SelfTouch::SecondOrder(TouchUnread::Unreadable);
    type Reads = fn(&SelfTouch) -> bool;
    let rows: Vec<(&str, [[f64; 3]; 3], Reads)> = match at {
        Seat::Corner => vec![
            ("off", off, rest),
            ("into", corner_into, crossing),
            ("straddling", straddle, crossing),
            ("along the arc", circle, rest),
            ("along the arc, into", circle_into, tangency),
        ],
        Seat::Arc => vec![
            ("off", off, rest),
            ("into", into, crossing),
            ("along the arc", circle, rest),
            ("along the arc, into", circle_into, tangency),
            ("along the arc, under the post", circle_under, rest),
            ("down the wall's plane, under the post", under, rest),
            (
                "down the wall's plane in band of it, under the post",
                [[3.0 * zero, 0.0, -0.3], [0.3, -0.1, 0.1], [0.3, 0.05, -0.1]],
                in_band,
            ),
        ],
        Seat::Wall => vec![
            ("off", off, rest),
            ("straddling", straddle, crossing),
            ("along the ruling", ruling, rest),
            ("along the ruling, into", ruling_into, crossing),
            ("along the circle", circle, tangency),
            (
                "along the circle, off it in band",
                lift(3.0 * zero),
                in_band,
            ),
            (
                "along the circle, off it past the band",
                lift(3.0 * escalate),
                rest,
            ),
        ],
    };
    let mut bad = Vec::new();
    for far in [0.0, 1e3] {
        let c = Point3::new(0.3 + far, 0.2 - 2.0 * far, 0.5 * far);
        for &(what, edges, reads) in &rows {
            let (body, corner, ties) = seat(c, at, edges);
            let (planar, verdict) = read(&body, corner, &ties);
            if planar != TouchUnread::Unreadable || !reads(&verdict) {
                bad.push(format!(
                    "{at:?}, {what}, {far} off: planar {planar:?}, {verdict:?}"
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A corner on a curved post's corner reads the post's tangent
/// corner**: the corner of its wall, its chord's plane and its bottom,
/// whose arc leaves the vertex along its tangent. Off the post it rests,
/// into it or straddling it it crosses; an edge along the arc's tangent
/// parts from the arc one order down, the arc bending into the post: off
/// the post it rests, into it it refuses. Red where a curved face's
/// normal or an arc's departure or bend reads the other way.
#[test]
fn a_corner_on_its_own_curved_posts_corner_reads_its_tangent_corner() {
    rows_hold(Seat::Corner);
}

/// **A corner inside a curved post's arc reads the arc's two faces as
/// half-planes**: the wall's and the bottom's, along the arc's tangent
/// there. Off the post it rests, into it it crosses; an edge along the
/// arc's tangent rests off the post or under it, and refuses into it.
/// Red where the arc's direction or bend at the point reads the other
/// way.
#[test]
fn a_corner_inside_its_own_curved_posts_arc_reads_its_two_half_planes() {
    rows_hold(Seat::Arc);
}

/// **A corner inside a curved post's wall reads the wall as its
/// tangent plane**: off the post it rests, straddling the wall it
/// crosses; an edge along the wall's ruling lies in the wall as the
/// wall keeps it, so the corner reads as its other edges place it; an
/// edge along the wall's circle leaves the wall only to second order,
/// which no reading here parts, and refuses. Red where the wall's normal
/// reads the other way, a crossing reads as a rest, or the tangency
/// reads at all.
#[test]
fn a_corner_inside_its_own_curved_posts_wall_reads_its_tangent_plane() {
    rows_hold(Seat::Wall);
}

/// **An edge leaves its vertex along its tangent there**
/// ([`departure`]): each half-edge of an arc over `[0.3, 1.9]` of the
/// unit circle, which leaves its two ends at different turns, leaves
/// its start square to the radius there and toward its other end; each
/// chord leaves along itself. Red where a half-edge reads the other
/// end's tangent, or the walk the other way.
#[test]
fn an_edge_leaves_its_vertex_along_its_tangent_there() {
    let center = Point3::new(0.3, -0.2, 0.1);
    let circle = geom::Curve3::circle(center, Vec3::unit_z(), 1.0, Vec3::unit_x());
    let (body, _) = crate::test_support_fixtures::arc_cap(
        circle,
        (0.3, 1.9),
        Vec3::new(0.0, 0.0, 0.0),
        Tol::witness(),
    );
    let at = |v: VertexKey| body.points[body.vertices[v].point];
    let mut arcs = 0;
    for (he, half) in &body.half_edges {
        let (from, to) = (at(half.start), at(body.half_edge_end(he).unwrap()));
        let (d, line, _) = departure(&body, half.start, he).unwrap();
        let d = d.normalize();
        if line {
            assert!(
                (d - (to - from).normalize()).norm() < 1e-12,
                "a chord: {d:?}"
            );
        } else {
            arcs += 1;
            assert!(
                d.dot(from - center).abs() < 1e-12,
                "square to the radius: {d:?}"
            );
            assert!(d.dot(to - from) > 0.1, "toward the other end: {d:?}");
        }
    }
    assert_eq!(arcs, 2, "both halves of the arc");
}

/// **A corner whose two edges leave its vertex along one tangent is a
/// cusp, left to second order**: a lamina in `z = 0` bounded by the unit
/// circle's arc from `(1, 0)` to `(0, 1)`, the line on to `(1, 1)` and
/// the line back down to `(1, 0)`, which leaves that vertex along the
/// arc's own tangent there; a parallelepiped's corner below the lamina
/// on that vertex, in its solid, tied to it. The arc and the line bend
/// apart only to second order, so which side of the tangent the face
/// lies on is no first-order reading's. Red where the cusp reads as half
/// a turn, or as a whole one.
#[test]
fn a_cusp_corner_is_left_to_second_order() {
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use crate::test_support_fixtures::line;
    let tol = Tol::witness();
    let o = Point3::origin();
    let circle = geom::Curve3::circle(o, Vec3::unit_z(), 1.0, Vec3::unit_x());
    let (a, b, c) = (
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    let plane = body.add_surface(geom::Surface::Plane {
        origin: o,
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    });
    let wall = body.add_surface(geom::Surface::Cylinder {
        origin: o,
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    });
    let arc = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    pair: geom_brep::SurfacePair::new(wall, plane),
                    selector: geom_brep::CurveSelector::Witness(
                        circle.eval(core::f64::consts::FRAC_PI_4),
                    ),
                },
                carrier: circle,
                param_start: 0.0,
                param_end: core::f64::consts::FRAC_PI_2,
            },
            tol,
        )
        .unwrap();
    let top = body
        .mev(
            MevSite::Fan {
                he1: arc.he_minus,
                he2: arc.he_minus,
            },
            c,
            line(b, c),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: top.he_minus,
            he2: arc.he_plus,
        },
        line(c, a),
        FaceSurface::Shared {
            key: plane,
            sense: true,
        },
        tol,
    )
    .unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::Shared {
            key: plane,
            sense: false,
        },
    )
    .unwrap();
    let (e0, e1, e2) = (
        Vec3::new(0.3, -0.1, -0.2),
        Vec3::new(-0.1, -0.3, -0.2),
        Vec3::new(0.1, 0.1, -0.3),
    );
    let (e0, e1) = if e0.cross(e1).dot(e2) < 0.0 {
        (e1, e0)
    } else {
        (e0, e1)
    };
    let part = mapped_cube::<f64>(move |u, v, w| a + e0 * u + e1 * v + e2 * w, tol);
    let before: BTreeSet<VertexKey> = body.vertices.keys().collect();
    let keep = census_entity_solid(&body, EntityId::Vertex(*before.first().unwrap())).unwrap();
    let gone = crate::instance::graft_disjoint(&mut body, &part).unwrap();
    for sh in body.solids.remove(gone).unwrap().shells {
        body.shells[sh].solid = keep;
        body.solids[keep].shells.push(sh);
    }
    body.solid_provenance.remove(gone);
    let at = |body: &Body<f64>, v: VertexKey| body.points[body.vertices[v].point];
    let near = |body: &Body<f64>, v: VertexKey| (at(body, v) - a).norm() < 1e-12;
    let corner = body
        .vertices
        .keys()
        .find(|&v| !before.contains(&v) && near(&body, v))
        .unwrap();
    let cusp = before.iter().copied().find(|&v| near(&body, v)).unwrap();
    let ties = Ties {
        vv: vec![(corner, cusp), (cusp, corner)],
        ..Ties::default()
    };
    let (planar, verdict) = read(&body, corner, &ties);
    assert_eq!(
        planar,
        TouchUnread::Unreadable,
        "a planar face's curved edge held at its corner"
    );
    assert_eq!(verdict, SelfTouch::SecondOrder(TouchUnread::Unreadable));
}

/// The arc of the circle about `center` of unit axis `axis` and radius
/// `radius` that edge `e` runs along, the short way from its `he_plus`
/// start to its end.
fn arc_on(
    body: &Body<f64>,
    e: EdgeKey,
    center: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
) -> geom_brep::EdgeCurveSpec<f64> {
    let he = body.edges[e].he_plus;
    let at = |v: VertexKey| body.points[body.vertices[v].point];
    let (p0, p1) = (
        at(body.half_edges[he].start),
        at(body.half_edge_end(he).unwrap()),
    );
    let u = (p0 - center) / radius;
    let turn = |axis: Vec3<f64>| {
        let w = axis.cross(u);
        (p1 - center).dot(w).atan2((p1 - center).dot(u))
    };
    let axis = if turn(axis) < 0.0 { -axis } else { axis };
    geom_brep::EdgeCurveSpec::arc_of_circle(
        geom::Curve3::circle(center, axis, radius, u),
        0.0,
        turn(axis),
    )
    .unwrap()
}

/// **A thorn of its solid's own**, its tip at `c`: the material outside
/// the sphere about `c + (0, A, B)` through `c`, with `y` and `z` above
/// `c`'s and `x` short of a unit past it. Three faces leave the tip
/// along `+x`, each a horn: the plane `z` (outward `−z`) between the
/// line along `x` and the sphere's circle in it, the plane `y` (outward
/// `−y`) between the line and the circle in that, and the sphere
/// (outward toward its centre) between the two circles. One order
/// down along `+x` they are the triangle of the line at `0` and the
/// circles at `(0, 1/A, 0)` and `(0, 0, 1/B)`. A parallelepiped's
/// corner, spanned by `edges`, is grafted on at the tip into the
/// thorn's solid. Returns the body, the corner, and the tie that holds
/// it at the tip.
fn thorn_and_corner(c: Point3<f64>, edges: [Vec3<f64>; 3]) -> (Body<f64>, VertexKey, Ties) {
    use crate::euler::{FaceSurface, MefSite, MevSite};
    const A: f64 = 2.0;
    const B: f64 = 3.0;
    let tol = Tol::witness();
    let x = Vec3::unit_x();
    let (tip, base) = (c, c + x);
    let side_y = c + Vec3::new(1.0, A - (A * A - 1.0).sqrt(), 0.0);
    let side_z = c + Vec3::new(1.0, 0.0, B - (B * B - 1.0).sqrt());
    let mut body = Body::<f64>::new();
    // A tetrahedron by Euler's operators, every edge a chord, then its
    // curved edges and its faces put on their carriers.
    let seed = body.mvfs(tip, true).unwrap();
    let ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            base,
            tol,
        )
        .unwrap();
    let bc = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_minus,
                he2: ab.he_minus,
            },
            side_y,
            tol,
        )
        .unwrap();
    let tri = body
        .mef_chord(
            MefSite::Chords {
                he1: bc.he_minus,
                he2: ab.he_plus,
            },
            tol,
        )
        .unwrap();
    let s = body
        .mev_line(
            MevSite::Fan {
                he1: ab.he_plus,
                he2: ab.he_plus,
            },
            side_z,
            tol,
        )
        .unwrap();
    let f_abd = body
        .mef_chord(
            MefSite::Chords {
                he1: s.he_minus,
                he2: bc.he_plus,
            },
            tol,
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: tri.he_plus,
            he2: f_abd.he_plus,
        },
        tol,
    )
    .unwrap();
    let centre = c + Vec3::new(0.0, A, B);
    let radius = (A * A + B * B).sqrt();
    let at = |body: &Body<f64>, v: VertexKey| body.points[body.vertices[v].point];
    let near = |q: Point3<f64>, r: Point3<f64>| (q - r).norm() < 1e-9;
    let ends = |body: &Body<f64>, e: EdgeKey| {
        let he = body.edges[e].he_plus;
        [
            at(body, body.half_edges[he].start),
            at(body, body.half_edge_end(he).unwrap()),
        ]
    };
    let edge_of = |body: &Body<f64>, p: Point3<f64>, q: Point3<f64>| {
        body.edges
            .keys()
            .find(|&e| {
                let [a, b] = ends(body, e);
                (near(a, p) && near(b, q)) || (near(a, q) && near(b, p))
            })
            .unwrap()
    };
    for (p, q, centre, axis, radius) in [
        (tip, side_y, c + Vec3::new(0.0, A, 0.0), Vec3::unit_z(), A),
        (tip, side_z, c + Vec3::new(0.0, 0.0, B), Vec3::unit_y(), B),
        (
            side_y,
            side_z,
            c + Vec3::new(1.0, A, B),
            x,
            (A * A + B * B - 1.0).sqrt(),
        ),
    ] {
        let e = edge_of(&body, p, q);
        let spec = arc_on(&body, e, centre, axis, radius);
        body.set_edge_curve(e, spec, tol).unwrap();
    }
    let plane = |origin: Point3<f64>, normal: Vec3<f64>, u_ref: Vec3<f64>| geom::Surface::Plane {
        origin,
        normal,
        u_ref,
    };
    let faces: Vec<FaceKey> = body.faces().map(|(f, _)| f).collect();
    for f in faces {
        let held: Vec<Point3<f64>> = face_cycles(&body, body.get_face(f).unwrap())
            .flat_map(|cycle| cycle.unwrap())
            .map(|he| at(&body, body.half_edges[he].start))
            .collect();
        let has = |q: Point3<f64>| held.iter().any(|&r| near(q, r));
        let (surface, sense) = match (has(tip), has(base), has(side_y), has(side_z)) {
            (true, true, true, false) => (plane(c, -Vec3::unit_z(), x), true),
            (true, true, false, true) => (plane(c, -Vec3::unit_y(), x), true),
            (false, true, true, true) => (plane(base, x, Vec3::unit_y()), true),
            (true, false, true, true) => (
                geom::Surface::Sphere {
                    center: centre,
                    radius,
                    axis: x,
                    u_ref: Vec3::unit_y(),
                },
                false,
            ),
            other => panic!("a face of the tetrahedron: {other:?}"),
        };
        body.set_face_surface(f, FaceSurface::New { surface, sense })
            .unwrap();
    }
    let thorn_tip = body
        .vertices
        .keys()
        .find(|&v| near(at(&body, v), tip))
        .unwrap();
    let [mut a, mut b, e] = edges;
    if a.cross(b).dot(e) < 0.0 {
        std::mem::swap(&mut a, &mut b);
    }
    let part = mapped_cube::<f64>(move |u, v, w| tip + a * u + b * v + e * w, tol);
    let before: BTreeSet<VertexKey> = body.vertices.keys().collect();
    let keep = census_entity_solid(&body, EntityId::Vertex(thorn_tip)).unwrap();
    let gone = crate::instance::graft_disjoint(&mut body, &part).unwrap();
    for sh in body.solids.remove(gone).unwrap().shells {
        body.shells[sh].solid = keep;
        body.solids[keep].shells.push(sh);
    }
    body.solid_provenance.remove(gone);
    let corner = body
        .vertices
        .keys()
        .find(|&v| !before.contains(&v) && near(at(&body, v), tip))
        .unwrap();
    let ties = Ties {
        vv: vec![(corner, thorn_tip), (thorn_tip, corner)],
        ..Ties::default()
    };
    (body, corner, ties)
}

/// **A thorn of the solid's own, its three faces horns at its tip,
/// reads whole one order down**: a parallelepiped's corner grafted on
/// at the tip, away from the thorn's ray, rests; one round the ray, its
/// material holding the thorn, refuses. Near the origin and a kilometre
/// off it. Red where a horn reads as the whole turn, a thorn's side
/// reads the other way, or the cone round it is not read.
#[test]
fn a_thorn_of_its_own_reads_its_horns_one_order_down() {
    let away = [
        Vec3::new(-0.3, 0.1, 0.1),
        Vec3::new(-0.1, -0.3, 0.1),
        Vec3::new(-0.1, 0.1, -0.3),
    ];
    let round = [
        Vec3::new(0.3, 0.3, -0.1),
        Vec3::new(0.3, -0.1, 0.3),
        Vec3::new(0.3, -0.2, -0.2),
    ];
    for far in [0.0, 1e3] {
        let c = Point3::new(0.3 + far, 0.2 - 2.0 * far, 0.5 * far);
        let (body, corner, ties) = thorn_and_corner(c, away);
        let (planar, verdict) = read(&body, corner, &ties);
        assert_eq!(planar, TouchUnread::Unreadable, "{far} off: a sphere face");
        assert!(
            matches!(verdict, SelfTouch::Read(TouchVerdict::Rest(_))),
            "{far} off, the corner away: {verdict:?}"
        );
        let (body, corner, ties) = thorn_and_corner(c, round);
        let (_, verdict) = read(&body, corner, &ties);
        assert_eq!(
            verdict,
            SelfTouch::SecondOrder(TouchUnread::Unreadable),
            "{far} off, the corner round the thorn"
        );
    }
}

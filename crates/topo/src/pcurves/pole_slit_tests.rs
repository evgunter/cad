#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
use geom_core::{Band, Point3, Tol, Vec3};

use crate::{Body, FaceKey, LoopBoundary};

use crate::construction::{FaceSurface, MefSite, MevSite};

/// The unit sphere's point at chart `(u, v)`: axis `+z`, seam `+x`.
fn at(u: f64, v: f64) -> Point3<f64> {
    Point3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin())
}

/// **The snowman's cap, built to the shape the probe saw.** A cap of
/// the unit sphere above the latitude `v = 0.5`, bounded by two rim
/// arcs (`u ∈ [0, π]`, its carrier's parameter a period below, and
/// `[π, 2π]`) and a meridian slit up `u = π`
/// from the rim to the north pole and back: rim arc, slit up, slit
/// down, rim arc. Returns the body, the cap and the cap's loop
/// members.
fn slit_cap() -> (Body<f64>, FaceKey, Vec<crate::HalfEdgeKey>) {
    slit_cap_below(1.0).expect("the slit cap mints")
}

/// [`slit_cap`] with its first rim arc's carrier parameter
/// `periods` whole periods below the azimuth it covers, or the
/// minting pass's refusal.
pub(crate) fn slit_cap_below(
    periods: f64,
) -> Result<(Body<f64>, FaceKey, Vec<crate::HalfEdgeKey>), super::PcurveMintError> {
    slit_cap_with(periods, false)
}

/// [`slit_cap_below`], and with `doubled` the slit as TWO edges: a
/// second edge down the same meridian from the pole to the rim, so
/// the cap's loop runs up one and down the other, and the sliver
/// between them is a face of no width, left unminted.
fn slit_cap_with(
    periods: f64,
    doubled: bool,
) -> Result<(Body<f64>, FaceKey, Vec<crate::HalfEdgeKey>), super::PcurveMintError> {
    let tol = Tol::witness();
    let v0 = 0.5;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(0.0, v0), true).unwrap();
    let sphere = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Sphere {
                    center: Point3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
    let rim_plane = body.add_surface(Surface::Plane {
        origin: Point3::new(0.0, 0.0, v0.sin()),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    });
    let rim = |t0: f64, t1: f64| EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(sphere, rim_plane),
            selector: geom_brep::CurveSelector::Witness(at(0.5 * (t0 + t1), v0)),
        },
        carrier: Curve3::circle(
            Point3::new(0.0, 0.0, v0.sin()),
            Vec3::unit_z(),
            v0.cos(),
            Vec3::unit_x(),
        ),
        param_start: t0,
        param_end: t1,
    };
    let first_arc = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(PI, v0),
            // A period below the azimuth it covers, `[0, π]`: its
            // image's base sits a period below the slit's.
            rim(-2.0 * PI * periods, PI - 2.0 * PI * periods),
            tol,
        )
        .unwrap();
    let second_arc = body
        .mef(
            MefSite::Chords {
                he1: first_arc.he_minus,
                he2: first_arc.he_plus,
            },
            rim(PI, 2.0 * PI),
            FaceSurface::Shared {
                key: sphere,
                sense: true,
            },
            tol,
        )
        .unwrap();
    // The cap runs the rim with `u` increasing, seen from above.
    let cap_loop = body.get_half_edge(first_arc.he_plus).unwrap().parent_loop;
    assert_eq!(
        body.get_half_edge(second_arc.he_plus).unwrap().parent_loop,
        cap_loop,
        "both rim arcs run forward around the cap"
    );
    // The slit: up the meridian `u = π`, a great circle in the plane
    // `y = 0`, from the rim to the north pole.
    let meridian_plane = body.add_surface(Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::unit_y(),
        u_ref: Vec3::new(-1.0, 0.0, 0.0),
    });
    let strut = body
        .mev(
            MevSite::Fan {
                he1: second_arc.he_plus,
                he2: second_arc.he_plus,
            },
            at(0.0, FRAC_PI_2),
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(sphere, meridian_plane),
                    selector: geom_brep::CurveSelector::Witness(at(PI, 0.5 * (v0 + FRAC_PI_2))),
                },
                carrier: Curve3::circle(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::unit_y(),
                    1.0,
                    Vec3::new(-1.0, 0.0, 0.0),
                ),
                param_start: v0,
                param_end: FRAC_PI_2,
            },
            tol,
        )
        .unwrap();
    if doubled {
        // Down the same meridian, from the pole: the great circle
        // in the plane `y = 0` again, run the other way.
        body.mef(
            MefSite::Chords {
                he1: strut.he_minus,
                he2: strut.he_plus,
            },
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(sphere, meridian_plane),
                    selector: geom_brep::CurveSelector::Witness(at(PI, 0.5 * (v0 + FRAC_PI_2))),
                },
                carrier: Curve3::circle(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, -1.0, 0.0),
                    1.0,
                    Vec3::unit_z(),
                ),
                param_start: 0.0,
                param_end: FRAC_PI_2 - v0,
            },
            FaceSurface::Shared {
                key: sphere,
                sense: true,
            },
            tol,
        )
        .unwrap();
    }
    let cap_loop = body.get_half_edge(first_arc.he_plus).unwrap().parent_loop;
    let cap = body.get_loop(cap_loop).unwrap().face;
    if doubled {
        crate::mint_pcurves_of(&mut body, &[cap], tol)?;
    } else {
        crate::mint_pcurves(&mut body, tol)?;
    }
    let LoopBoundary::Cycle { first } = body.get_loop(cap_loop).unwrap().boundary else {
        panic!("the cap's loop is a cycle")
    };
    let members = body.loop_cycle(first).unwrap();
    assert_eq!(members.len(), 4, "rim arc, slit up, slit down, rim arc");
    Ok((body, cap, members))
}

/// The member of a doubled slit's loop that leaves the pole, and the
/// cap's tier-3 findings: those naming the cap or a member of its
/// loop (the sliver beside it stores no row, and says so).
fn doubled_pole(body: &Body<f64>, members: &[crate::HalfEdgeKey]) -> crate::HalfEdgeKey {
    let edge = |he: crate::HalfEdgeKey| body.get_half_edge(he).unwrap().edge;
    let pole = members
        .iter()
        .copied()
        .find(|&he| (super::entry_vertex(body, he).z - 1.0).abs() < 1e-12)
        .expect("a member leaves the pole");
    assert_ne!(
        edge(body.get_half_edge(pole).unwrap().prev),
        edge(pole),
        "the slit is two edges"
    );
    pole
}

fn cap_findings(body: &Body<f64>, cap: FaceKey, members: &[crate::HalfEdgeKey]) -> Vec<String> {
    let band = Band::linear(Tol::witness()).unwrap();
    let named: Vec<String> = members
        .iter()
        .map(|he| format!("{he:?}"))
        .chain([format!("{cap:?}")])
        .collect();
    super::validate_pcurves(body, band)
        .iter()
        .map(|finding| format!("{finding:?}"))
        .filter(|finding| named.iter().any(|name| finding.contains(name)))
        .collect()
}

/// **A slit of two edges turns by its loop's winding, on the face's
/// side.** The two edges leave the pole along one direction, so no
/// value at the joint tells no turn from a whole one. The rim winds
/// the chart once with the pole on the cap's side, so the cap wraps
/// the pole and the turn is the whole one against the rim's: the
/// chain closes on its start, and tier 3 decides the same element.
/// Flipped, the face is the loop's other side, the rim winds with
/// the pole behind it, and the sector has no decided width: the
/// production flip drops the rows, and the mint after it refuses by
/// name where it once wrote the cap's own turn.
#[test]
fn a_two_edge_slit_turns_by_its_loops_winding_on_the_faces_side() {
    let tol = Tol::witness();
    let (body, cap, members) = slit_cap_with(1.0, true).expect("the doubled slit cap mints");
    let pole = doubled_pole(&body, &members);
    let turn = body.joint(pole).unwrap().deck();
    assert!(
        turn.u.abs() == 1 && turn.v == 0 && !turn.twin,
        "the doubled slit's turn at the pole is one whole turn: {turn:?}"
    );
    let rest = crate::joint::Winding::of(
        members
            .iter()
            .filter(|&&he| he != pole)
            .map(|&he| body.joint(he).unwrap()),
    );
    assert_eq!(
        rest.deck,
        crate::Deck {
            u: -turn.u,
            v: 0,
            twin: false
        },
        "the turn is against the rest of the loop's winding"
    );
    assert!(
        crate::joint::Winding::of(members.iter().map(|&he| body.joint(he).unwrap())).is_zero(),
        "the cap's chain closes on its start"
    );
    assert_eq!(cap_findings(&body, cap, &members), Vec::<String>::new());

    let mut flipped = body.flipped_face_sense_for_tests(cap, tol).unwrap();
    assert!(
        members
            .iter()
            .all(|&he| flipped.pcurve(he).is_none() && flipped.joint(he).is_none()),
        "the flipped cap keeps no row of its old side, and mints none"
    );
    let refused = crate::mint_pcurves_of(&mut flipped, &[cap], tol);
    assert!(
        matches!(
            refused,
            Err(super::PcurveMintError::SingularTurn {
                miss: super::SingularMiss::EdgesCoincide,
                ..
            })
        ),
        "the flipped slit's turn is decided by nothing: {refused:?}"
    );
}

/// **A split beside a two-edge slit leaves its turn standing.** A
/// meridian's image is a coordinate line, so its children restrict
/// it and the split decides only their own joint: the pole joint
/// keeps the whole turn, and tier 3, which decides it again with
/// the rest of the loop the split lengthened, reads the cap clean.
#[test]
fn a_split_beside_a_two_edge_slit_keeps_its_turn() {
    let tol = Tol::witness();
    let (mut body, cap, members) = slit_cap_with(1.0, true).expect("the doubled slit cap mints");
    let pole = doubled_pole(&body, &members);
    let turn = body.joint(pole).unwrap();
    for half in [pole, body.get_half_edge(pole).unwrap().prev] {
        let (_, t0, t1) = super::half_edge_carrier(&body, half).unwrap();
        let edge = body.get_half_edge(half).unwrap().edge;
        body.split_edge(edge, 0.5 * (t0 + t1), tol)
            .unwrap_or_else(|e| panic!("the slit's edge of {half:?} splits: {e:?}"));
    }
    let cap_loop = body.get_face(cap).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(cap_loop).unwrap().boundary else {
        panic!("the cap's loop is a cycle")
    };
    let members = body.loop_cycle(first).unwrap();
    assert_eq!(members.len(), 6, "each slit edge is two");
    let pole = doubled_pole(&body, &members);
    assert_eq!(body.joint(pole), Some(turn), "the pole joint's turn stands");
    assert_eq!(cap_findings(&body, cap, &members), Vec::<String>::new());
}

/// **A half-turn patch of the unit cylinder** (axis `+z`, seam `+x`)
/// between heights 0 and 1: the bottom arc `u ∈ [0, π]`, its circle
/// carrier's parameter `periods` whole periods below the azimuth it
/// covers, a ruling up `u = π`, the top arc back to `u = 0` and a
/// ruling down. Minted, or the minting pass's refusal.
pub(crate) fn cylinder_patch_below(periods: f64) -> Result<Body<f64>, super::PcurveMintError> {
    cylinder_patch_turned(periods, 0.0)
}

/// [`cylinder_patch_below`] on a cylinder whose `u_ref` is turned by
/// `turn` about its axis: the same patch in the world, its chart
/// azimuth `turn` less, so its left meridian (the world's `x` axis)
/// sits at chart `u = −turn`.
pub(crate) fn cylinder_patch_turned(
    periods: f64,
    turn: f64,
) -> Result<Body<f64>, super::PcurveMintError> {
    let tol = Tol::witness();
    let p = |u: f64, z: f64| Point3::new(u.cos(), u.sin(), z);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0, 0.0), true).unwrap();
    let cylinder = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Cylinder {
                    origin: Point3::new(0.0, 0.0, 0.0),
                    axis: Vec3::unit_z(),
                    radius: 1.0,
                    u_ref: Vec3::new(turn.cos(), turn.sin(), 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
    let rim = |body: &mut Body<f64>, z: f64, t0: f64, t1: f64| {
        let plane = body.add_surface(Surface::Plane {
            origin: Point3::new(0.0, 0.0, z),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(cylinder, plane),
                selector: geom_brep::CurveSelector::Witness(p(0.5 * (t0 + t1), z)),
            },
            carrier: Curve3::circle(
                Point3::new(0.0, 0.0, z),
                Vec3::unit_z(),
                1.0,
                Vec3::unit_x(),
            ),
            param_start: t0,
            param_end: t1,
        }
    };
    let shift = 2.0 * PI * periods;
    let bottom_spec = rim(&mut body, 0.0, -shift, PI - shift);
    let bottom = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(PI, 0.0),
            bottom_spec,
            tol,
        )
        .unwrap();
    let up = body
        .mev(
            MevSite::Fan {
                he1: bottom.he_minus,
                he2: bottom.he_minus,
            },
            p(PI, 1.0),
            EdgeCurveSpec::line_between(p(PI, 0.0), p(PI, 1.0)),
            tol,
        )
        .unwrap();
    // The top arc, its carrier run `[0, π]` like the bottom's, from
    // `u = π` back to `u = 0`.
    let mut top_spec = rim(&mut body, 1.0, 0.0, PI);
    top_spec.carrier = Curve3::circle(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        1.0,
        Vec3::new(-1.0, 0.0, 0.0),
    );
    let top = body
        .mev(
            MevSite::Fan {
                he1: up.he_minus,
                he2: up.he_minus,
            },
            p(0.0, 1.0),
            top_spec,
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: top.he_minus,
            he2: bottom.he_plus,
        },
        EdgeCurveSpec::line_between(p(0.0, 1.0), p(0.0, 0.0)),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    crate::mint_pcurves(&mut body, tol)?;
    Ok(body)
}

/// A half-turn patch of the cone of half-angle `0.4` about `z`, apex
/// at the origin, between the rims at heights `1` and `2`, over the
/// world azimuths `[0, π]`, on a chart whose `u_ref` is turned by
/// `turn`: its left ruling (the world's `x` side) at chart `u = −turn`.
pub(crate) fn cone_patch_turned(turn: f64) -> Result<Body<f64>, super::PcurveMintError> {
    let alpha: f64 = 0.4;
    let (z0, z1) = (1.0, 2.0);
    let tol = Tol::witness();
    let p = |u: f64, z: f64| Point3::new(z * alpha.tan() * u.cos(), z * alpha.tan() * u.sin(), z);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0, z0), true).unwrap();
    let cone = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Cone {
                    apex: Point3::new(0.0, 0.0, 0.0),
                    axis: Vec3::unit_z(),
                    half_angle: alpha,
                    u_ref: Vec3::new(turn.cos(), turn.sin(), 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
    let rim = |body: &mut Body<f64>, z: f64, t0: f64, t1: f64| {
        let plane = body.add_surface(Surface::Plane {
            origin: Point3::new(0.0, 0.0, z),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(cone, plane),
                selector: geom_brep::CurveSelector::Witness(p(0.5 * (t0 + t1), z)),
            },
            carrier: Curve3::circle(
                Point3::new(0.0, 0.0, z),
                Vec3::unit_z(),
                z * alpha.tan(),
                Vec3::unit_x(),
            ),
            param_start: t0,
            param_end: t1,
        }
    };
    let bottom_spec = rim(&mut body, z0, 0.0, PI);
    let bottom = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(PI, z0),
            bottom_spec,
            tol,
        )
        .unwrap();
    let up = body
        .mev(
            MevSite::Fan {
                he1: bottom.he_minus,
                he2: bottom.he_minus,
            },
            p(PI, z1),
            EdgeCurveSpec::line_between(p(PI, z0), p(PI, z1)),
            tol,
        )
        .unwrap();
    // The top arc, from `u = π` back to `u = 0`.
    let mut top_spec = rim(&mut body, z1, 0.0, PI);
    top_spec.carrier = Curve3::circle(
        Point3::new(0.0, 0.0, z1),
        Vec3::new(0.0, 0.0, -1.0),
        z1 * alpha.tan(),
        Vec3::new(-1.0, 0.0, 0.0),
    );
    let top = body
        .mev(
            MevSite::Fan {
                he1: up.he_minus,
                he2: up.he_minus,
            },
            p(0.0, z1),
            top_spec,
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: top.he_minus,
            he2: bottom.he_plus,
        },
        EdgeCurveSpec::line_between(p(0.0, z1), p(0.0, z0)),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    crate::mint_pcurves(&mut body, tol)?;
    Ok(body)
}

/// The cap's stored rows — each half-edge's image and joint element —
/// as text, by half-edge.
fn rows(body: &Body<f64>, members: &[crate::HalfEdgeKey]) -> Vec<String> {
    members
        .iter()
        .map(|&he| format!("{he:?} {:?} {:?}", body.pcurve(he), body.joint(he)))
        .collect()
}

/// **A patch across the seam of a net closed in `u`.** A triangular
/// prism stated as one degree-1 net, `u` knots at `0.3` and `0.7`,
/// its first and last columns the same locus, so the chart closes in
/// `u` with period `1`. The patch covers the two facets either side
/// of the seam, `u ∈ [0.7, 1] ∪ [0, 0.3]`, `v ∈ [0, 1]`: a bottom
/// split at the seam vertex, iso-line sides at `u = 0.3` and `0.7`,
/// a top split the same way. Every edge is a straight line on a
/// facet's edge or along a knot line, so each row is an iso line.
pub(crate) fn prism_patch_across_the_seam() -> Result<(Body<f64>, FaceKey), super::PcurveMintError>
{
    let tol = Tol::witness();
    let (c0, c1, c2) = (
        (1.0, 0.0),
        (-0.5, 0.75_f64.sqrt()),
        (-0.5, -(0.75_f64.sqrt())),
    );
    let at = |c: (f64, f64), z: f64| Point3::new(c.0, c.1, z);
    let ku = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1).unwrap();
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [c0, c1, c2, c0]
        .into_iter()
        .flat_map(|c| [at(c, 0.0), at(c, 1.0)])
        .collect();
    let net = geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(c2, 0.0), true).unwrap();
    let wall = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Nurbs(std::sync::Arc::new(net)),
                sense: true,
            },
        )
        .unwrap();
    let column = |c: (f64, f64)| [at(c, 0.0), at(c, 1.0)];
    // A cap edge: the net's own row at height `v`, its carrier the
    // row on the net's basis and the chart's `u` as its parameter,
    // over `[u0, u1]`; its image the row `v`, stated, since the seam's
    // two sides `u = 1` and `u = 0` are one point to a projection.
    let row = |v: f64, (u0, u1): (f64, f64)| EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart_image(
            wall,
            geom_brep::Pcurve::IsoLine {
                p0: geom_core::Point2::new(0.0, v),
                pl: geom_core::Vec2::new(1.0, 0.0),
            },
        ),
        carrier: Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1)
                    .unwrap(),
                vec![at(c0, v), at(c1, v), at(c2, v), at(c0, v)],
                vec![1.0; 4],
            )
            .unwrap(),
        )),
        param_start: u0,
        param_end: u1,
    };
    // A side: the straight column at `u`, from height `v0` to `v1`.
    let side = |c: (f64, f64), u: f64, (v0, v1): (f64, f64)| {
        let [p, q] = column(c);
        let (p, q) = if v0 < v1 { (p, q) } else { (q, p) };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(
                wall,
                geom_brep::Pcurve::IsoLine {
                    p0: geom_core::Point2::new(u, v0),
                    pl: geom_core::Vec2::new(0.0, v1 - v0),
                },
            ),
            carrier: Curve3::line(p, q - p),
            param_start: 0.0,
            param_end: 1.0,
        }
    };
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    // The bottom, rising in `u` across the seam vertex.
    let b1 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(c0, 0.0),
            row(0.0, (0.7, 1.0)),
            tol,
        )
        .unwrap();
    let b2 = body
        .mev(fan(b1.he_minus), at(c1, 0.0), row(0.0, (0.0, 0.3)), tol)
        .unwrap();
    // Up the side at `u = 0.7` from the start, and the top the same
    // way as the bottom.
    let up = body
        .mev(fan(b1.he_plus), at(c2, 1.0), side(c2, 0.7, (0.0, 1.0)), tol)
        .unwrap();
    let t1 = body
        .mev(fan(up.he_minus), at(c0, 1.0), row(1.0, (0.7, 1.0)), tol)
        .unwrap();
    let t2 = body
        .mev(fan(t1.he_minus), at(c1, 1.0), row(1.0, (0.0, 0.3)), tol)
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: t2.he_minus,
            he2: b2.he_minus,
        },
        side(c1, 0.3, (1.0, 0.0)),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    crate::mint_pcurves(&mut body, tol)?;
    Ok((body, seed.face))
}

/// **A lifted copy is the row and its element, never certified
/// again.** On the prism patch across the seam of a net closed in
/// `u`, each face's walk decides a joint element of a whole chart
/// period, `Deck.u = ±1`, and lifts iso-line rows by it: the patch's
/// side at `u = 0.7` to `u = −0.3`, off the knot domain an iso line's
/// hull bound holds on. `loop_windings` certifies each row as the
/// row it is, before any lift, so both faces read and close. Certifying
/// the lifted copy instead refuses: the bottom row lifted to
/// `u ∈ [−0.3, 0]` reads its net off the domain, and its map
/// residual is exceeded.
#[test]
fn a_lifted_iso_line_off_the_knot_domain_is_its_row_and_element() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (body, _) = prism_patch_across_the_seam().unwrap();
    for (f, face) in body.faces() {
        let s = body.get_surface(face.surface).unwrap().clone();
        let elements: Vec<_> = {
            let LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary else {
                panic!("the outer loop is a cycle")
            };
            body.loop_walk(first)
                .closed("loop", first)
                .into_iter()
                .map(|he| body.joint(he))
                .collect()
        };
        assert!(
            elements.iter().any(|e| matches!(
                e,
                Some(crate::joint::JointElement(d)) if d.u != 0
            )),
            "{f:?}: the walk crosses the seam: {elements:?}"
        );
        let read = super::loop_windings(&body, f, &s, band)
            .unwrap_or_else(|e| panic!("{f:?} reads: {e:?}"));
        assert!(
            read.iter().all(|w| w.zero && !w.singular),
            "{f:?}: every loop's lift closes: {read:?}"
        );
    }
}

/// **A patch across the seam of a net closed in `v`**: the prism of
/// [`prism_patch_across_the_seam`] with its channels exchanged. One
/// degree-1 net, `u` the height (knots `0, 1`), `v` around the prism
/// with knots at `0.3` and `0.7`, its first and last rows the same
/// locus, so the chart closes in `v` with period `1`. The patch
/// covers the two facets either side of the seam, `u ∈ [0, 1]`,
/// `v ∈ [0.7, 1] ∪ [0, 0.3]`: a bottom split at the seam vertex,
/// iso-line sides at `v = 0.3` and `0.7`, a top split the same way.
pub(crate) fn prism_patch_across_the_v_seam() -> Result<(Body<f64>, FaceKey), super::PcurveMintError>
{
    let tol = Tol::witness();
    let (c0, c1, c2) = (
        (1.0, 0.0),
        (-0.5, 0.75_f64.sqrt()),
        (-0.5, -(0.75_f64.sqrt())),
    );
    let at = |c: (f64, f64), z: f64| Point3::new(c.0, c.1, z);
    let around =
        || geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1).unwrap();
    let ku = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [0.0, 1.0]
        .into_iter()
        .flat_map(|z| [c0, c1, c2, c0].map(|c| at(c, z)))
        .collect();
    let net = geom::NurbsSurface::new(ku, around(), control, vec![1.0; 8]).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(at(c2, 0.0), true).unwrap();
    let wall = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Nurbs(std::sync::Arc::new(net)),
                sense: true,
            },
        )
        .unwrap();
    // A cap edge: the net's own column at height `u`, its carrier the
    // column on the net's basis and the chart's `v` as its parameter,
    // over `[v0, v1]`; its image the column `u`, stated, since the
    // seam's two sides `v = 1` and `v = 0` are one point to a
    // projection.
    let cap = |u: f64, (v0, v1): (f64, f64)| EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart_image(
            wall,
            geom_brep::Pcurve::IsoLine {
                p0: geom_core::Point2::new(u, 0.0),
                pl: geom_core::Vec2::new(0.0, 1.0),
            },
        ),
        carrier: Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                around(),
                vec![at(c0, u), at(c1, u), at(c2, u), at(c0, u)],
                vec![1.0; 4],
            )
            .unwrap(),
        )),
        param_start: v0,
        param_end: v1,
    };
    // A side: the straight row at `v`, from height `u0` to `u1`.
    let side = |c: (f64, f64), v: f64, (u0, u1): (f64, f64)| {
        let (p, q) = (at(c, u0), at(c, u1));
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(
                wall,
                geom_brep::Pcurve::IsoLine {
                    p0: geom_core::Point2::new(u0, v),
                    pl: geom_core::Vec2::new(u1 - u0, 0.0),
                },
            ),
            carrier: Curve3::line(p, q - p),
            param_start: 0.0,
            param_end: 1.0,
        }
    };
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    // The bottom, rising in `v` across the seam vertex.
    let b1 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(c0, 0.0),
            cap(0.0, (0.7, 1.0)),
            tol,
        )
        .unwrap();
    let b2 = body
        .mev(fan(b1.he_minus), at(c1, 0.0), cap(0.0, (0.0, 0.3)), tol)
        .unwrap();
    // Up the side at `v = 0.7` from the start, and the top the same
    // way as the bottom.
    let up = body
        .mev(fan(b1.he_plus), at(c2, 1.0), side(c2, 0.7, (0.0, 1.0)), tol)
        .unwrap();
    let t1 = body
        .mev(fan(up.he_minus), at(c0, 1.0), cap(1.0, (0.7, 1.0)), tol)
        .unwrap();
    let t2 = body
        .mev(fan(t1.he_minus), at(c1, 1.0), cap(1.0, (0.0, 0.3)), tol)
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: t2.he_minus,
            he2: b2.he_minus,
        },
        side(c1, 0.3, (1.0, 0.0)),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    crate::mint_pcurves(&mut body, tol)?;
    Ok((body, seed.face))
}

/// **A net closed in `v` lifts its seam crossing by a `v` period**
/// (C4: every closed channel has its own period, the knot domain's
/// length on a net decided closed in that direction). On the prism
/// patch across the `v` seam, each face's walk decides a joint
/// element of a whole `v` period, `Deck.v = ±1`, and no `u` period;
/// both faces' loops close (`loop_windings`); and the body validates. Without the `v` period the seam joint's gap is
/// a whole knot length of `v`, and the mint refuses it as a
/// discontinuity.
#[test]
fn a_net_closed_in_v_lifts_its_seam_crossing_by_a_v_period() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (body, _) = prism_patch_across_the_v_seam()
        .unwrap_or_else(|e| panic!("the prism closed in v mints: {e:?}"));
    for (f, face) in body.faces() {
        let s = body.get_surface(face.surface).unwrap().clone();
        assert!(
            super::chart_u_period(&s, band).is_none(),
            "{f:?}: the net is open in u"
        );
        assert_eq!(
            super::chart_v_period(&s, band).map(|p| p.period().length()),
            Some(1.0),
            "{f:?}: the net closes in v over its knot domain"
        );
        let elements: Vec<_> = {
            let LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary else {
                panic!("the outer loop is a cycle")
            };
            body.loop_walk(first)
                .closed("loop", first)
                .into_iter()
                .map(|he| body.joint(he))
                .collect()
        };
        assert!(
            elements.iter().any(|e| matches!(
                e,
                Some(crate::joint::JointElement(d)) if d.v.abs() == 1 && d.u == 0
            )),
            "{f:?}: the walk crosses the v seam by one period: {elements:?}"
        );
        let read = super::loop_windings(&body, f, &s, band)
            .unwrap_or_else(|e| panic!("{f:?} reads: {e:?}"));
        assert!(
            read.iter().all(|w| w.zero && !w.singular),
            "{f:?}: every loop's lift closes: {read:?}"
        );
    }
    let refusals = super::validate_pcurves(&body, band);
    assert!(refusals.is_empty(), "the body validates: {refusals:?}");
}

/// **The `v` period is the closed net's alone.** A joint whose two
/// chart ends are the two sides of a net's `v` seam, `v = 1` onto
/// `v = 0` at one point: on the net closed in `v` the walk decides
/// `Deck.v = 1` there; on the same net with its last row moved off the
/// first (open in `v`), `chart_v_period` answers none, no `v` element
/// is decided, and the whole knot length of `v` is the gap, a
/// discontinuity.
#[test]
fn only_a_net_closed_in_v_decides_a_v_element() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (c0, c1, c2) = (
        (1.0, 0.0),
        (-0.5, 0.75_f64.sqrt()),
        (-0.5, -(0.75_f64.sqrt())),
    );
    let at = |c: (f64, f64), z: f64| Point3::new(c.0, c.1, z);
    let net = |last: (f64, f64)| {
        let ku = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1).unwrap();
        let control = [0.0, 1.0]
            .into_iter()
            .flat_map(|z| [c0, c1, c2, last].map(|c| at(c, z)))
            .collect();
        Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
        ))
    };
    // The row at `u = 0.5` entering at `v = 0`, its predecessor's exit
    // at `v = 1`: one point on the closed net.
    let image = geom_brep::Pcurve::IsoLine {
        p0: geom_core::Point2::new(0.5, 0.0),
        pl: geom_core::Vec2::new(0.0, 0.3),
    };
    let joint = |s: &Surface<f64>| {
        super::decide_plain_joint(
            super::DescribedChart::of(s).unwrap(),
            &image,
            0.0,
            geom_core::Point2::new(0.5, 1.0),
            at(c0, 0.5),
            super::ChartPeriods::of(s, band),
            band,
        )
    };
    let read = |out: &Result<crate::joint::JointElement, super::PinMiss>| match out {
        Ok(element) => format!("{element:?}"),
        Err(super::PinMiss::Discontinuity) => "Discontinuity".to_owned(),
        Err(super::PinMiss::Escalated(cause)) => format!("Escalated({cause:?})"),
        Err(super::PinMiss::NetStretchUndecided(cause)) => {
            format!("NetStretchUndecided({cause:?})")
        }
        Err(super::PinMiss::Singular(miss)) => format!("{miss:?}"),
    };
    let closed = net(c0);
    assert!(
        matches!(
            joint(&closed),
            Ok(crate::joint::JointElement(crate::joint::Deck {
                u: 0,
                v: 1,
                twin: false
            }))
        ),
        "closed in v: {}",
        read(&joint(&closed))
    );
    let open = net((0.9, 0.1));
    assert!(super::chart_v_period(&open, band).is_none(), "open in v");
    assert!(
        matches!(joint(&open), Err(super::PinMiss::Discontinuity)),
        "open in v: {}",
        read(&joint(&open))
    );
}

/// **A meridian on the window's cut reads the same either side of
/// it.** A generic cut makes an image on it rare, not impossible: a
/// half-turn cylinder patch and a half-turn cone patch on charts
/// turned so the left meridian sits at `u = −1`, the principal
/// window's start, within an ulp either way and a hundred band
/// widths either way (radius 1, so the band is in radians).
/// Within an ulp the placement's mark is in band and its tie-break
/// names one copy whichever side the rounding fell (the first
/// premise); a hundred band widths off it is decided, and names `−1` on one side and
/// `τ − 1` on the other (the second, both read off the stored rows).
/// Nothing downstream tells the copies apart: each body validates,
/// bounds its faces, reads the same polygon modulo a turn, and
/// overlaps itself with the same answer.
#[test]
fn a_meridian_on_the_windows_cut_reads_the_same_either_side_of_it() {
    let band = Band::linear(Tol::witness()).unwrap();
    let tau = core::f64::consts::TAU;
    // The window's cut, pinned as the literal it is (`−1`), so a row
    // that moved the window would go red here rather than follow it.
    let turn = 1.0;
    assert_eq!(
        geom_brep::ANGLE_WINDOW_START,
        -turn,
        "the cut this row sits on"
    );
    // Far enough off the cut that the placement's mark is decided at
    // every ε row: past the escalation band, which is ten band zeros.
    let far = 100.0 * band.zero();
    let turns = [
        turn.next_down(),
        turn,
        turn.next_up(),
        turn - far,
        turn + far,
    ];
    type Build = fn(f64) -> Body<f64>;
    let builds: [(&str, Build); 2] = [
        ("cylinder", |t| cylinder_patch_turned(0.0, t).unwrap()),
        ("cone", |t| cone_patch_turned(t).unwrap()),
    ];
    for (name, build) in builds {
        let bodies: Vec<Body<f64>> = turns.iter().map(|&t| build(t)).collect();
        let mut copies: Vec<Vec<f64>> = Vec::new();
        for (t, body) in turns.iter().zip(&bodies) {
            let mut here = Vec::new();
            assert_eq!(
                super::validate_pcurves(body, band),
                vec![],
                "{name}, turned {t}: validates"
            );
            for (f, face) in body.faces() {
                let s = body.get_surface(face.surface).unwrap().clone();
                let read = super::loop_windings(body, f, &s, band)
                    .unwrap_or_else(|e| panic!("{name}, turned {t}, {f:?}: reads: {e:?}"));
                assert!(
                    read.iter().all(|w| w.zero),
                    "{name}, turned {t}, {f:?}: every loop closes: {read:?}"
                );
            }
            // The left meridian's stored image: the constant azimuth
            // one copy of which is `−1`.
            for (he, _) in body.half_edges() {
                if let Some(geom_brep::Pcurve::Harmonic {
                    u: geom_brep::Channel::Angle { phase, winding: 0 },
                    ..
                }) = body.pcurve(he).map(|c| c.pcurve())
                {
                    let off = (phase + 1.0).rem_euclid(tau);
                    if off < 2.0 * far || tau - off < 2.0 * far {
                        here.push(*phase);
                    }
                }
            }
            assert!(
                !here.is_empty(),
                "{name}, turned {t}: the meridian is stored"
            );
            copies.push(here);
        }
        // The tie-break takes the lower branch, which names the
        // midpoint at the window's closed end, `τ − 1`: pinned as that
        // copy, so an upper tie-break goes red.
        let near: Vec<f64> = copies[..3].concat();
        assert!(
            near.iter().all(|u| (u - (tau - 1.0)).abs() < 1e-12),
            "{name}: the first premise, one copy, τ − 1, within an ulp of the cut: {near:?}"
        );
        let all = copies.concat();
        assert!(
            all.iter().any(|u| (u + 1.0).abs() < 2.0 * far)
                && all.iter().any(|u| (u - (tau - 1.0)).abs() < 2.0 * far),
            "{name}: the second premise, the meridian named on both sides of the cut: {copies:?}"
        );
        let faces: Vec<FaceKey> = bodies[0].faces().map(|(f, _)| f).collect();
        let outcome = |a: usize, b: usize, f: FaceKey| {
            crate::glued_pair_overlap(
                &bodies[a],
                f,
                &bodies[b],
                f,
                crate::CarrierPremise::Decided,
                band,
            )
        };
        // Each body against itself: the copies the rounding named
        // are the only difference across the cut (bodies turned an
        // ulp apart differ in geometry too, which a near-flush seat
        // reads as such).
        for &f in &faces {
            let first = outcome(0, 0, f);
            for a in 1..turns.len() {
                assert_eq!(
                    outcome(a, a, f),
                    first,
                    "{name}, {f:?}: turned {} against turned {}",
                    turns[a],
                    turns[0]
                );
            }
        }
    }
}

/// **A sphere patch anywhere in latitude reads in the chart's
/// region.** A patch between two meridians and two parallels, below
/// latitude −1 rad, south and north of the equator and across it, its
/// meridians' carriers started on the near half of their great circles
/// and on the far half (the frame negated, the span moved by `π`: PR
/// 5225's delta review, `d3`, where the far start stored the twin), is
/// minted, validates, stores every latitude in `[−π/2, π/2]` (the
/// chart's own latitudes, inside the latitude window `geom_brep`'s
/// `Window::Latitude` places them in), and overlaps itself with
/// positive area. Placed in the angular window instead, a
/// meridian below latitude −1 was stored a turn up, which the chart
/// region's sphere arm refuses (PR 5225's first review, P2).
#[test]
fn a_sphere_patch_anywhere_in_latitude_reads_in_the_charts_region() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (o, z) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_z());
    let circle = |center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>| {
        Curve3::Circle(geom::Circle {
            frame: geom::Frame {
                origin: center,
                axis,
                u_ref,
            },
            radius,
        })
    };
    let p = |u: f64, v: f64| Point3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin());
    let rhat = |u: f64| Vec3::new(u.cos(), u.sin(), 0.0);
    let bands = [(-1.4_f64, -1.1_f64), (-0.9, -0.3), (0.3, 1.3), (-0.2, 0.4)];
    for (far, (va, vb)) in [false, true]
        .into_iter()
        .flat_map(|far| bands.map(|b| (far, b)))
    {
        let (flip, sh) = if far { (-1.0, PI) } else { (1.0, 0.0) };
        let (u0, u1) = (0.0_f64, 1.0_f64);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(u0, va), true).unwrap();
        body.set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Sphere {
                    center: o,
                    radius: 1.0,
                    axis: z,
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
        let bottom = EdgeCurveSpec::arc_of_circle(
            circle(Point3::new(0.0, 0.0, va.sin()), z, va.cos(), rhat(u0)),
            0.0,
            u1 - u0,
        )
        .unwrap();
        let b = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(u1, va),
                bottom,
                tol,
            )
            .unwrap();
        let right = EdgeCurveSpec::arc_of_circle(
            circle(o, rhat(u1).cross(z), 1.0, rhat(u1) * flip),
            va + sh,
            vb + sh,
        )
        .unwrap();
        let r = body
            .mev(
                MevSite::Fan {
                    he1: b.he_minus,
                    he2: b.he_minus,
                },
                p(u1, vb),
                right,
                tol,
            )
            .unwrap();
        let top = EdgeCurveSpec::arc_of_circle(
            circle(Point3::new(0.0, 0.0, vb.sin()), -z, vb.cos(), rhat(u1)),
            0.0,
            u1 - u0,
        )
        .unwrap();
        let tp = body
            .mev(
                MevSite::Fan {
                    he1: r.he_minus,
                    he2: r.he_minus,
                },
                p(u0, vb),
                top,
                tol,
            )
            .unwrap();
        let left = EdgeCurveSpec::arc_of_circle(
            circle(o, z.cross(rhat(u0)), 1.0, rhat(u0) * flip),
            -vb + sh,
            -va + sh,
        )
        .unwrap();
        body.mef(
            MefSite::Chords {
                he1: tp.he_minus,
                he2: b.he_plus,
            },
            left,
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        super::mint_pcurves(&mut body, tol).unwrap();
        let label = format!("far {far} [{va}, {vb}]");
        assert_eq!(super::validate_pcurves(&body, band), vec![], "{label}");
        for (he, c) in body.pcurves() {
            let (a, b) = c.params();
            let mid = c.pcurve().eval(0.5 * (a + b));
            assert!(
                mid.y.abs() <= core::f64::consts::FRAC_PI_2 + 1e-12,
                "{label}, {he:?}: a latitude in the chart's region: {mid:?}"
            );
        }
        for (f, _) in body.faces() {
            let o =
                crate::glued_pair_overlap(&body, f, &body, f, crate::CarrierPremise::Decided, band);
            assert!(
                matches!(o, Ok(crate::ChartOverlap::PositiveArea)),
                "{label}, {f:?}: {o:?}"
            );
        }
    }
}

/// **A one-edge loop's split closes on its own children.** A sheet
/// bounded by one whole-turn circle (a cylinder's rim, a sphere's
/// parallel, a torus's parallel) is split at `t ∈ {1, 3, 5, 5.5, 6}`.
/// Past about `t = 4.28` the second child's midpoint is past the
/// window's cut, so it is placed a turn below the first and the
/// loop's closing joint, from the second child back into the first,
/// changes; the edge is its own neighbour, so the split decides that
/// joint between its children. Each split body validates and its rows
/// and elements are byte for byte a fresh mint's (PR 5225's first
/// review, P1).
#[test]
fn a_one_edge_loops_split_closes_on_its_children() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (o, z, x) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_z(), Vec3::unit_x());
    let circle = |center: Point3<f64>, radius: f64| Curve3::circle(center, z, radius, x);
    let lat: f64 = 0.5;
    let cases = [
        (
            "cylinder rim",
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            },
            circle(Point3::new(0.0, 0.0, 0.5), 1.0),
        ),
        (
            "sphere parallel",
            Surface::Sphere {
                center: o,
                radius: 1.0,
                axis: z,
                u_ref: x,
            },
            circle(Point3::new(0.0, 0.0, lat.sin()), lat.cos()),
        ),
        (
            "torus parallel",
            Surface::Torus {
                center: o,
                axis: z,
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: x,
            },
            circle(Point3::new(0.0, 0.0, 0.5), 2.0),
        ),
    ];
    let rows = |body: &Body<f64>| {
        let mut out: Vec<String> = body
            .pcurves()
            .map(|(he, c)| {
                format!(
                    "{he:?} {:?} {:?} {:?} {:?}",
                    c.params(),
                    c.pcurve(),
                    c.certificate(),
                    body.joint(he)
                )
            })
            .collect();
        out.sort();
        out
    };
    for (name, surface, carrier) in cases {
        for t in [1.0, 3.0, 5.0, 5.5, 6.0] {
            let mut body = Body::<f64>::new();
            let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
            body.set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: surface.clone(),
                    sense: true,
                },
            )
            .unwrap();
            let spec = EdgeCurveSpec::arc_of_circle(carrier.clone(), 0.0, 2.0 * PI).unwrap();
            let made = body
                .mef(
                    MefSite::Lone {
                        r#loop: seed.r#loop,
                    },
                    spec,
                    FaceSurface::Inherit,
                    tol,
                )
                .unwrap();
            super::mint_pcurves(&mut body, tol).unwrap();
            let edge = body.get_half_edge(made.he_plus).unwrap().edge;
            body.split_edge(edge, t, tol)
                .unwrap_or_else(|e| panic!("{name} at {t}: splits: {e:?}"));
            assert_eq!(
                super::validate_pcurves(&body, band),
                vec![],
                "{name} at {t}: validates"
            );
            let mut fresh = body.clone();
            super::mint_pcurves(&mut fresh, tol).unwrap();
            assert_eq!(rows(&body), rows(&fresh), "{name} at {t}: the mint's rows");
        }
    }
}

/// **A split's children are the mint's rows, and the loop still
/// closes.** The cylinder patch, its bottom arc's carrier stated 0,
/// ±1, 5 and 10 periods off, is split on that arc, and the split
/// body's rows, their intervals, certificates and joint elements
/// are byte for byte what a fresh `mint_pcurves` of the same body
/// writes; it validates and bounds both faces. Two turns of the
/// chart: one where both children sit inside the window, and one
/// turned so the arc straddles the window's cut at `−1`, where the
/// first child is placed a whole turn from the second and the joint
/// between them is a decided `±1` (the premise, read off the stored
/// element). Dropping the split's re-decision of the parent's end
/// joints (`CarriedRows::entering`, `::leaving`) leaves the
/// straddling patch's joints the parent's, and it no longer
/// validates (`LoopDiscontinuity` on both halves the re-placed child
/// meets); `sweep`'s `the_site_rows_are_the_passs_on_every_revolved_chart`
/// goes red the same way on the full cone.
#[test]
fn a_split_of_an_edge_stated_periods_off_is_the_mints_and_closes() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let rows = |body: &Body<f64>| {
        let mut out: Vec<String> = body
            .pcurves()
            .map(|(he, c)| {
                format!(
                    "{he:?} {:?} {:?} {:?} {:?}",
                    c.params(),
                    c.pcurve(),
                    c.certificate(),
                    body.joint(he)
                )
            })
            .collect();
        out.sort();
        out
    };
    // Inside the window, and straddling its cut: the arc covers
    // chart `u ∈ [−turn, π − turn]`, split an eighth of a turn in.
    for (turn, straddles) in [(0.0, false), (1.0 + PI / 4.0, true)] {
        for periods in [0.0, 1.0, -1.0, 5.0, 10.0] {
            let label = format!("turned {turn}, {periods} periods off");
            let mut body = cylinder_patch_turned(periods, turn).unwrap();
            let bottom = body
                    .edges()
                    .map(|(e, _)| e)
                    .find(|&e| {
                        let he = body.get_edge(e).unwrap().he_plus;
                        matches!(
                            super::half_edge_carrier(&body, he),
                            Ok((Curve3::Circle(geom::Circle { frame: geom::Frame { origin: center, .. }, .. }), _, _)) if center.z == 0.0
                        )
                    })
                    .expect("the bottom arc");
            let he = body.get_edge(bottom).unwrap().he_plus;
            let (_, t0, t1) = super::half_edge_carrier(&body, he).unwrap();
            let t = t0.min(t1) + PI / 4.0;
            let made = body
                .split_edge(bottom, t, tol)
                .unwrap_or_else(|e| panic!("{label}: splits: {e:?}"));
            assert_eq!(
                super::validate_pcurves(&body, band),
                vec![],
                "{label}: validates"
            );
            for (f, face) in body.faces() {
                let s = body.get_surface(face.surface).unwrap().clone();
                let read = super::loop_windings(&body, f, &s, band)
                    .unwrap_or_else(|e| panic!("{label}, {f:?}: reads: {e:?}"));
                assert!(
                    read.iter().all(|w| w.zero),
                    "{label}, {f:?}: every loop closes: {read:?}"
                );
            }
            let mut fresh = body.clone();
            super::mint_pcurves(&mut fresh, tol).unwrap();
            assert_eq!(
                rows(&body),
                rows(&fresh),
                "{label}: the split's rows are the mint's"
            );
            let new = body.get_edge(made.new_edge).unwrap();
            let apart = [new.he_plus, new.he_minus, he]
                .into_iter()
                .any(|h| matches!(body.joint(h), Some(crate::JointElement(d)) if d.u != 0));
            assert_eq!(
                apart, straddles,
                "{label}: the premise, the children a whole turn apart where the arc \
                     straddles the cut"
            );
        }
    }
}

/// **A body stated periods off reads downstream as its principal
/// twin.** The rows the walk mints far out reach the readers built
/// on them, and each reads what the same body at zero periods reads:
/// - `loop_windings` reads both faces of the cylinder patch 5, 6
///   and 10 periods below as it reads the twin's;
/// - `chart_region_overlap` against the twin finds positive area on
///   either face at 5 periods and on the patch's own face at 10;
/// - the slit cap's loop through the pole reads singular at 3 and 10
///   periods as at zero.
///
/// The other face 6 periods below refuses `TouchingBoundary`, and
/// that refusal is rounding, measured: the carrier's stated end
/// `π − 6τ` rounds as it is written, so its seat's corner reads
/// `π` plus 4 ulp, the two loops' walls are no longer exactly
/// collinear, and the overlap refuses as it does any near-flush
/// seat. No branch decision enters it; at 5 periods the end happens
/// to round back to `π`.
#[test]
fn a_body_stated_periods_off_reads_downstream_as_its_principal_twin() {
    let band = Band::linear(Tol::witness()).unwrap();
    let twin = cylinder_patch_below(0.0).unwrap();
    let faces: Vec<FaceKey> = twin.faces().map(|(f, _)| f).collect();
    assert_eq!(faces.len(), 2);
    let bound = |body: &Body<f64>, f: FaceKey| {
        let s = body.proven_face(f).surface().clone();
        super::loop_windings(body, f, &s, band)
    };
    for periods in [5.0, 6.0, 10.0] {
        let body = cylinder_patch_below(periods).unwrap();
        for &f in &faces {
            assert_eq!(bound(&body, f), bound(&twin, f), "{periods} periods, {f:?}");
        }
        let overlaps: Vec<_> = faces
            .iter()
            .map(|&f| crate::chart_region_overlap(&body, f, &twin, f, band))
            .collect();
        let expected = if periods == 5.0 {
            [
                Ok(crate::ChartOverlap::PositiveArea),
                Ok(crate::ChartOverlap::PositiveArea),
            ]
        } else {
            [
                Ok(crate::ChartOverlap::PositiveArea),
                Err(crate::ChartRegionError::TouchingBoundary),
            ]
        };
        assert_eq!(overlaps, expected, "{periods} periods");
    }
    let cap_bound = |periods: f64| {
        let (body, cap, _) = slit_cap_below(periods).unwrap();
        bound(&body, cap).map(|read| read.iter().map(|w| w.singular).collect::<Vec<_>>())
    };
    let principal = cap_bound(0.0);
    assert!(
        matches!(&principal, Ok(singular) if singular.contains(&true)),
        "{principal:?}"
    );
    for periods in [3.0, 10.0] {
        assert_eq!(cap_bound(periods), principal, "{periods} periods");
    }
}

/// **A carrier stated whole periods off its azimuth mints.** Nothing
/// bounds where a circle carrier's parameter runs against the azimuth
/// it covers, so the loop walk's joint gap carries that offset, and
/// the walk reads its marks from zero however far out it lies:
/// - the slit cap with its first rim arc 3 and 10 periods below,
///   counted in the sphere's half steps, 6 and 20 of them;
/// - a half-turn cylinder patch with its bottom arc 5 and 10 periods
///   below.
///
/// Each mints, re-mints from every anchor where it is the slit cap,
/// and reads clean. Check 4 reads no offset here: each stored image is
/// its own placed re-derivation.
#[test]
fn a_carrier_stated_periods_off_its_azimuth_mints() {
    let band = Band::linear(Tol::witness()).unwrap();
    for periods in [3.0, 10.0] {
        let (mut body, cap, members) = slit_cap_below(periods)
            .unwrap_or_else(|e| panic!("slit cap {periods} periods below: {e:?}"));
        let cap_loop = body.get_half_edge(members[0]).unwrap().parent_loop;
        let minted = rows(&body, &members);
        for &anchor in &members {
            body.get_loop_mut(cap_loop).unwrap().boundary = LoopBoundary::Cycle { first: anchor };
            crate::mint_pcurves_of(&mut body, &[cap], Tol::witness()).unwrap_or_else(|e| {
                panic!("{periods} periods below, anchored at {anchor:?}: {e:?}")
            });
            assert_eq!(
                rows(&body, &members),
                minted,
                "{periods}, anchored at {anchor:?}"
            );
            assert_eq!(super::validate_pcurves(&body, band), vec![], "{periods}");
        }
    }
    for periods in [5.0, 10.0] {
        let body = cylinder_patch_below(periods)
            .unwrap_or_else(|e| panic!("cylinder patch {periods} periods below: {e:?}"));
        assert_eq!(
            super::validate_pcurves(&body, band),
            vec![],
            "cylinder {periods}"
        );
    }
}

/// The slit cap's joint at the pole: the one into a half from its
/// own mate.
fn pole_joint(body: &Body<f64>, members: &[crate::HalfEdgeKey]) -> crate::HalfEdgeKey {
    let edge = |he: crate::HalfEdgeKey| body.get_half_edge(he).unwrap().edge;
    let mut at_tips = members
        .iter()
        .copied()
        .filter(|&he| edge(body.get_half_edge(he).unwrap().prev) == edge(he));
    let pole = at_tips.next().expect("the slit turns at its tip");
    assert!(at_tips.next().is_none(), "the slit has one tip");
    pole
}

/// **The pass re-mints a pole-slit loop from every anchor, and reads
/// what tier 3 reads.** The slit's two halves meet at the pole, where
/// the cap wraps the point: the element is one whole turn, against
/// the rim's own, so the loop's chain closes on its start as the
/// cap's rectangle in the chart, in the pass and at tier 3 alike. So
/// with the loop anchored at each of its four half-edges in turn the
/// pass mints the cap, writes the same images and elements, and
/// tier 3 reads it clean (`work/topo/the-pass-refuses-a-tier-3-clean-
/// loop-anchored-past-a-pole-slit`: the pass refused `LoopNotClosed`
/// where the walk from the anchor carried the pole's free azimuth
/// gap into its closure).
#[test]
fn a_pole_slit_loop_re_mints_from_every_anchor() {
    let (mut body, cap, members) = slit_cap();
    let band = Band::linear(Tol::witness()).unwrap();
    let cap_loop = body.get_half_edge(members[0]).unwrap().parent_loop;
    let turn = body.joint(pole_joint(&body, &members)).unwrap().deck();
    assert!(
        turn.u.abs() == 1 && turn.v == 0 && !turn.twin,
        "the slit's turn at the pole is one whole turn: {turn:?}"
    );
    assert!(
        crate::joint::Winding::of(members.iter().map(|&he| body.joint(he).unwrap())).is_zero(),
        "the cap's chain closes on its start"
    );
    let minted = rows(&body, &members);
    for &anchor in &members {
        body.get_loop_mut(cap_loop).unwrap().boundary = LoopBoundary::Cycle { first: anchor };
        crate::mint_pcurves_of(&mut body, &[cap], Tol::witness())
            .unwrap_or_else(|e| panic!("anchored at {anchor:?}, the pass refuses: {e:?}"));
        assert_eq!(rows(&body, &members), minted, "anchored at {anchor:?}");
        assert_eq!(
            super::validate_pcurves(&body, band),
            vec![],
            "anchored at {anchor:?}"
        );
        assert!(
            body.loop_lift(cap_loop).is_ok(),
            "anchored at {anchor:?}, the loop lifts"
        );
    }
}

/// **A planted pole twin is loud.** At the slit's pole every azimuth
/// names one point and the twin fixes the pole's latitude, so the
/// turn with the twin composed in carries the slit's image onto the
/// same pole point — and draws the slit's other half on the twin
/// representation, off the cap's chart region. Tier 3 reads a stored
/// element only as the one it decides there, so the planted twin is
/// a discontinuity at that joint.
#[test]
fn a_planted_twin_on_the_pole_joint_is_loud() {
    let (mut body, _, members) = slit_cap();
    let band = Band::linear(Tol::witness()).unwrap();
    let pole = pole_joint(&body, &members);
    let decided = body.joint(pole).unwrap().deck();
    let sigma = crate::Deck {
        u: 0,
        v: 0,
        twin: true,
    };
    let planted = crate::JointElement(decided.compose(sigma));
    body.attach_joint(pole, planted);
    let findings = super::validate_pcurves(&body, band);
    assert!(
        findings.contains(&super::PcurveMintError::LoopDiscontinuity { half_edge: pole }),
        "a twin planted on the pole joint is a discontinuity there: {findings:?}"
    );
}

/// **A turn at a pole is the face's, and no other is read as it.**
/// The pole joint stored with no turn (the identity: the two halves'
/// one image, met on itself), or with the turn the other way round
/// (the complement's), is a discontinuity at that joint: tier 3
/// reads the stored element as the one decided on the face's side.
#[test]
fn a_pole_turn_stored_as_another_is_loud() {
    let (body, _, members) = slit_cap();
    let band = Band::linear(Tol::witness()).unwrap();
    let pole = pole_joint(&body, &members);
    let decided = body.joint(pole).unwrap();
    for other in [crate::JointElement::IDENTITY, decided.inverse()] {
        let mut planted = body.clone();
        planted.attach_joint(pole, other);
        let findings = super::validate_pcurves(&planted, band);
        assert!(
            findings.contains(&super::PcurveMintError::LoopDiscontinuity { half_edge: pole }),
            "{other:?} planted at the pole is a discontinuity there: {findings:?}"
        );
    }
}

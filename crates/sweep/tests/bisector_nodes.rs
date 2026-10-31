//! **The nodes, built**: two equal cylinders whose axes meet at an
//! angle `θ`, both long enough that each passes through the other, so
//! both nodes `p ± r·n̂` lie inside a face of each wall. Their section,
//! the two bisector-plane ellipses, crosses itself at both nodes, and
//! every boolean of the pair builds:
//!
//! - `∩`, the Steinmetz solid, `V = 16r³/(3 sin θ)`, each node one vertex
//!   where four faces alternate between the walls (valence 4);
//! - `∪`, `V_A + V_B − V∩`, each node one valence-4 vertex;
//! - `A∖B` and `B∖A`, `V_A − V∩` and `V_B − V∩`: two solids touching at
//!   both nodes, each with its own valence-2 vertex there.
//!
//! Each operand is turned about its own axis first, so neither wall's
//! seam passes a node (U2's poses are not these). Rods and bores build
//! in every pairing: two crossing bores, a slanted bore whose difference
//! is one solid pinched to itself, and a rod against a bore, each in its
//! row below. The last row turns the rod within a hair of the bore's
//! axis, where the walls are glued and no node is read: there the
//! answer must not follow the operand order.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::{cyl, repose, spin};
use crate::common::touch_union::only_curved_within_reach;
use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::{Body, BooleanResult};

/// The seam turn about each operand's own axis.
const PHI: f64 = 0.4;
/// The walls' half-length: past every point of the intersection for the
/// angles below (`(1 + |cos θ|)/sin θ < 4`).
const H: f64 = 4.0;

/// `A` about `z`, and `B` about `(0, −sin θ, cos θ)`, each of radius `r`
/// and half-length [`H`], through the origin; their nodes are `±r·x̂`.
fn pair(theta: f64, r: f64) -> (Body<f64>, Body<f64>) {
    let a = spin(&cyl(r, H), Vec3::new(0.0, 0.0, 1.0), PHI);
    let b = spin(
        &spin(&cyl(r, H), Vec3::new(0.0, 0.0, 1.0), PHI),
        Vec3::new(1.0, 0.0, 0.0),
        theta,
    );
    (a, b)
}

/// One operation's result: its solids, the valence of each vertex
/// standing on each node, by node, its 3′ and the body.
struct Built {
    solids: usize,
    at_nodes: [Vec<usize>; 2],
    /// Tier 3′ on the result's own contact records.
    three_prime: Result<(), Vec<topo::ValidationError>>,
    body: topo::AtRestBody<f64>,
}

fn built(
    r: Result<BooleanResult<f64>, topo::BooleanError>,
    nodes: [Point3<f64>; 2],
    what: &str,
) -> Built {
    let Ok(BooleanResult::Body(out)) = r else {
        panic!("{what}: the node builds, got {r:?}");
    };
    let body = &out.body;
    let tol = Tol::witness();
    let valence = |v| body.half_edges().filter(|(_, h)| h.start == v).count();
    let at = |n: Point3<f64>| {
        body.vertex_points()
            .filter(|(_, p)| (*p - n).norm() < 1e-9)
            .map(|(v, _)| valence(v))
            .collect::<Vec<_>>()
    };
    Built {
        solids: body.solids().count(),
        at_nodes: [at(nodes[0]), at(nodes[1])],
        three_prime: topo::validate_pseudomanifold(body, &out.contacts, tol).map(|_| ()),
        body: body.clone(),
    }
}

/// Every operation of the pair at angle `theta`, both operand orders,
/// against the closed forms and the node's shape.
fn assert_family(
    a: &Body<f64>,
    b: &Body<f64>,
    (theta, r): (f64, f64),
    nodes: [Point3<f64>; 2],
    what: &str,
) {
    let tol = Tol::witness();
    let (a, b) = (finished("A", a.clone(), tol), finished("B", b.clone(), tol));
    let v_wall = 2.0 * PI * r * r * H;
    let cap = 16.0 * r.powi(3) / (3.0 * theta.sin());
    let close = |got: &Built, want: f64, op: &str| assert_volume(&got.body, want, op);
    for (first, second, order) in [(&a, &b, "A, B"), (&b, &a, "B, A")] {
        let cut = |op| format!("{what} ({order}), {op}");
        let i = built(topo::intersect(first, second, tol), nodes, &cut("∩"));
        close(&i, cap, &cut("∩"));
        let u = built(topo::union(first, second, tol), nodes, &cut("∪"));
        close(&u, 2.0 * v_wall - cap, &cut("∪"));
        for (got, op) in [(&i, "∩"), (&u, "∪")] {
            assert!(
                got.three_prime.is_ok(),
                "{}: tier 3′ on its own records: {:?}",
                cut(op),
                got.three_prime
            );
            assert_eq!(got.solids, 1, "{}: one solid", cut(op));
            assert_eq!(
                got.at_nodes,
                [vec![4], vec![4]],
                "{}: each node is one vertex of valence 4",
                cut(op)
            );
        }
        let d = built(topo::subtract(first, second, tol), nodes, &cut("∖"));
        close(&d, v_wall - cap, &cut("∖"));
        assert_eq!(d.solids, 2, "{}: two solids", cut("∖"));
        assert_eq!(
            d.at_nodes,
            [vec![2, 2], vec![2, 2]],
            "{}: each node is a valence-2 vertex of each solid",
            cut("∖")
        );
        // The two solids touch at the nodes along curved faces, which
        // the census's cross-solid backstop cannot read yet
        // (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`):
        // that refusal is admitted, and nothing else.
        assert!(
            d.three_prime
                .as_ref()
                .err()
                .is_some_and(|errs| only_curved_within_reach(&d.body, errs)),
            "{}: tier 3′ answers the census lane's frontier alone: {:?}",
            cut("∖"),
            d.three_prime
        );
    }
}

/// **The family at several angles**, each beside its re-posed twin.
#[test]
fn equal_cylinders_with_meeting_axes_build_every_operation_at_the_closed_forms() {
    for (deg, r) in [
        (90.0_f64, 1.0),
        (60.0, 1.0),
        (120.0, 1.0),
        (35.0, 1.0),
        (60.0, 0.5),
    ] {
        let theta = deg.to_radians();
        let (a, b) = pair(theta, r);
        let nodes = [Point3::new(r, 0.0, 0.0), Point3::new(-r, 0.0, 0.0)];
        assert_family(&a, &b, (theta, r), nodes, &format!("{deg}°, r = {r}"));
    }
}

/// **The same family re-posed**: the nodes move with the operands, and
/// nothing else changes.
#[test]
fn the_family_re_posed_builds_the_same() {
    let theta = 60.0_f64.to_radians();
    let (a, b) = pair(theta, 1.0);
    let (a, b) = (repose(&a), repose(&b));
    let r = geom_core::Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    let place =
        geom_core::Affine3::from_parts(r.linear, r.translation + Vec3::new(0.3, -0.45, 0.6));
    let nodes = [
        place.transform_point(Point3::new(1.0, 0.0, 0.0)),
        place.transform_point(Point3::new(-1.0, 0.0, 0.0)),
    ];
    assert_family(&a, &b, (theta, 1.0), nodes, "60°, re-posed");
}

/// A rod about `z` over `z ∈ [z0, z1]`, its seam turned [`PHI`].
fn rod(z0: f64, z1: f64) -> Body<f64> {
    use sweep::{ExtrudeSide, Extrusion, extrude};
    let tol = Tol::witness();
    let lp = profile::circle(geom_core::Point2::new(0.0, 0.0), 1.0, tol).unwrap();
    let plane = profile::SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, z0));
    let p = profile::Profile::new(plane, vec![lp.into()])
        .validate(tol)
        .unwrap();
    let b = extrude(
        &p,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    spin(&b, Vec3::new(0.0, 0.0, 1.0), PHI)
}

/// **A node on an operand's edge is not this lane's** (U2): where a cap
/// stands at the axes' meeting point, its rim circle passes through
/// both nodes, tangent there to the partner wall. With one operand
/// capped there that touch reads in the band's gap and the circle ×
/// wall roots escalate, at the rim and the wall; with both, the rims
/// meet at the nodes, which lie on both
/// walls' boundaries, and the node pass refuses them. Every operation,
/// both poses.
#[test]
fn a_rim_through_the_nodes_refuses_typed() {
    let tol = Tol::witness();
    let across = |b: &Body<f64>| spin(b, Vec3::new(1.0, 0.0, 0.0), -PI / 2.0);
    for (what, a, b, at_node) in [
        ("B's cap", rod(-2.0, 2.0), across(&rod(0.0, 2.0)), false),
        ("A's cap", rod(0.0, 2.0), across(&rod(-2.0, 2.0)), false),
        ("both caps", rod(0.0, 2.0), across(&rod(0.0, 2.0)), true),
    ] {
        for (pose, a, b) in [
            ("direct", a.clone(), b.clone()),
            ("re-posed", repose(&a), repose(&b)),
        ] {
            let (a, b) = (finished("A", a, tol), finished("B", b, tol));
            for (op, r) in [
                ("∪", topo::union(&a, &b, tol)),
                ("∩", topo::intersect(&a, &b, tol)),
                ("A∖B", topo::subtract(&a, &b, tol)),
                ("B∖A", topo::subtract(&b, &a, tol)),
            ] {
                if at_node {
                    assert!(
                        matches!(
                            r,
                            Err(topo::BooleanError::NodeUnbuilt {
                                why: topo::NodeWhy::OnBoundary(_),
                                ..
                            })
                        ),
                        "{what}, {pose}, {op}: the node on both rims, got {r:?}"
                    );
                } else {
                    assert!(
                        matches!(
                            r,
                            Err(topo::BooleanError::CurvedPierceEscalated {
                                decision: topo::BooleanDecision::ArcCylinderRoots,
                                ..
                            })
                        ),
                        "{what}, {pose}, {op}: the rim's touch of the wall escalates, got {r:?}"
                    );
                }
            }
        }
    }
}

/// A unit rod about `z` over `z ∈ [z0, z1]`, its seam turned `phi`.
fn rod_at(z0: f64, z1: f64, phi: f64) -> Body<f64> {
    spin(&rod(z0, z1), Vec3::new(0.0, 0.0, 1.0), phi - PHI)
}

/// **An edge near a node crosses the partner wall through the bisector
/// planes.** Two unit rods at right angles, `A` about `z` over
/// `[−2, 2]` and `B` about `y`, nodes `±x̂`: in turn A's seam and B's
/// seam turned `δ` off the node, A's cap and B's cap standing `δ` off
/// the axes' meeting point. Each such edge meets the partner wall twice,
/// one crossing on each bisector plane, `2δ` apart. Read through the
/// wall's quadric they are a double root `δ²` deep, and every pose
/// short of `√band` refused at the crossing layer; read through the
/// planes:
///
/// - `δ` in the zero band: the two crossings are one, a node on the
///   edge, which keeps the quadric's door where the roots decide it a
///   tangency, and escalates on their decision, at the edge and the
///   wall, where they read it in the band's gap;
/// - `δ` a few bands: their distance in band escalates `VertexOnVertex`
///   (or, at a fragment's end, `VertexOnCurvedFace`);
/// - `δ` beyond the band and short of `√band`: each pierce reads the
///   partner wall through the plane the sweep found it on, and the arcs
///   from each pierce to the node read their wedge at their faces'
///   reach, so every pose builds at its closed form (at ε 1e-12 a cap's
///   short ruling refuses at its pcurve,
///   `work/pcert/a-short-ruling-on-a-cylinder-chart-mints-its-pcurve-at-the-seams-u.md`);
/// - `δ` past `√band`: every operation builds at its closed form.
///
/// Every operation, both operand orders, both poses, at every ε.
#[test]
fn an_edge_near_a_node_crosses_through_the_bisector_planes() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let (zero, escalate) = (band.zero(), band.escalate());
    let fine = zero < 1e-10;
    let across = |b: &Body<f64>| spin(b, Vec3::new(1.0, 0.0, 0.0), -PI / 2.0);
    let v_rod = 4.0 * PI;
    // The Steinmetz solid's piece beyond `y = d` of B's axis.
    let beyond = |d: f64| {
        let s = (1.0 - d * d).sqrt();
        2.0 * (s.powi(3) / 3.0 - d * d.acos() + s)
    };
    enum Want {
        Door,
        Escalates,
        PastThePierce,
        Builds,
    }
    for (delta, want) in [
        (0.3 * zero, Want::Door),
        (4.0 * zero, Want::Escalates),
        (30.0 * escalate, Want::PastThePierce),
        (5.0 * escalate.sqrt(), Want::Builds),
    ] {
        let poses = [
            (
                "A's seam",
                rod_at(-2.0, 2.0, delta),
                across(&rod(-2.0, 2.0)),
            ),
            (
                "B's seam",
                rod(-2.0, 2.0),
                across(&rod_at(-2.0, 2.0, delta)),
            ),
            ("A's cap", rod(delta, 2.0), across(&rod(-2.0, 2.0))),
            ("B's cap", rod(-2.0, 2.0), across(&rod(delta, 2.0))),
        ];
        for (what, a, b) in poses {
            let capped = what.ends_with("cap");
            let (va, vb) = match what {
                "A's cap" => (PI * (2.0 - delta), v_rod),
                "B's cap" => (v_rod, PI * (2.0 - delta)),
                _ => (v_rod, v_rod),
            };
            let cap = if capped { beyond(delta) } else { 16.0 / 3.0 };
            for (pose, a, b) in [
                ("direct", a.clone(), b.clone()),
                ("re-posed", repose(&a), repose(&b)),
            ] {
                let (a, b) = (finished("A", a, tol), finished("B", b, tol));
                for (order, x, y, vx, vy) in [("A, B", &a, &b, va, vb), ("B, A", &b, &a, vb, va)] {
                    for (op, r, closed) in [
                        ("∪", topo::union(x, y, tol), vx + vy - cap),
                        ("∩", topo::intersect(x, y, tol), cap),
                        ("∖", topo::subtract(x, y, tol), vx - cap),
                    ] {
                        let cut = format!("{what} δ = {delta:e}, {pose} ({order}), {op}");
                        match want {
                            Want::Door => assert!(
                                matches!(
                                    r,
                                    Err(topo::BooleanError::CurvedPierceUnsupported { .. }
                                        | topo::BooleanError::CurvedPierceEscalated {
                                            decision: topo::BooleanDecision::ArcCylinderRoots,
                                            ..
                                        })
                                ),
                                "{cut}: one crossing, the quadric's door or a touch on its roots, \
                                 got {r:?}"
                            ),
                            Want::Escalates => assert!(
                                matches!(
                                    r,
                                    Err(topo::BooleanError::Escalated {
                                        decision: topo::BooleanDecision::VertexOnVertex
                                            | topo::BooleanDecision::Coincidence(
                                                topo::Coincide::VertexOnCurvedFace,
                                                _
                                            ),
                                        ..
                                    })
                                ),
                                "{cut}: the crossings' distance in band, got {r:?}"
                            ),
                            // At ε 1e-12 the ruling where the cap meets the
                            // partner wall between the two pierces mints its
                            // pcurve at the wall's seam
                            // (`work/pcert/a-short-ruling-on-a-cylinder-chart-mints-its-pcurve-at-the-seams-u.md`).
                            Want::PastThePierce if capped && fine && pose == "re-posed" => {
                                assert!(
                                    map_residual(&r),
                                    "{cut}: the short ruling's pcurve, got {r:?}"
                                );
                            }
                            // Otherwise every pose builds: the pierce reads the
                            // partner wall through its branch, and the short arcs
                            // from each pierce to the node read their wedge at
                            // their faces' reach.
                            Want::PastThePierce => {
                                let Ok(BooleanResult::Body(out)) = r else {
                                    panic!("{cut}: builds, got {r:?}");
                                };
                                let m = topo::mass_properties(
                                    &out.body,
                                    tol,
                                    topo::test_support::pinned_resolution(tol),
                                )
                                .unwrap();
                                assert!(
                                    (m.enclosure.volume - closed).abs()
                                        <= m.enclosure.volume_pad + 1e-9,
                                    "{cut}: volume {} ± {}, want {closed}",
                                    m.enclosure.volume,
                                    m.enclosure.volume_pad
                                );
                            }
                            Want::Builds => {
                                let Ok(BooleanResult::Body(out)) = r else {
                                    panic!("{cut}: builds, got {r:?}");
                                };
                                // A cap `δ` off the station leaves its disc two
                                // lunes `δ²/2` wide beside the partner wall, which
                                // the mesh does not triangulate at any chord tried
                                // (`work/tess/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`):
                                // its volume is held to the certified bracket alone.
                                if capped {
                                    let m = topo::mass_properties(
                                        &out.body,
                                        tol,
                                        topo::test_support::pinned_resolution(tol),
                                    )
                                    .unwrap();
                                    assert!(
                                        (m.enclosure.volume - closed).abs()
                                            <= m.enclosure.volume_pad + 1e-9,
                                        "{cut}: volume {} ± {}, want {closed}",
                                        m.enclosure.volume,
                                        m.enclosure.volume_pad
                                    );
                                } else {
                                    assert_volume(&out.body, closed, &cut);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Whether `r` is a pcurve's chart map refusing its residual, at the
/// merge or at rest.
fn map_residual(r: &Result<BooleanResult<f64>, topo::BooleanError>) -> bool {
    let text = format!("{r:?}");
    text.contains("Pcurve") && text.contains("MapResidual")
}

/// **U2p's acceptance poses at the witness band**: two unit rods at right
/// angles, each seam turned and each cap standing `δ ∈ {1e-8, 1e-6,
/// 1e-4}` off a node, and both operands capped, every operation, both
/// operand orders and both poses.
///
/// - Each seam and each cap at 1e-6 and 1e-4, and both caps, build at
///   their closed forms (both caps: `2X − 2X³/3 − 2δ·acos δ`,
///   `X = √(1 − δ²)`): the arcs from each pierce to the node, `2δ` long at
///   a dihedral of `δ`, read their wedge at their faces' reach.
/// - 1e-8 is the witness band's escalation edge (K·ε). The node stands
///   1e-8 off the seam, so its placement escalates (margin 9.99e-9); both
///   caps' sectors stand about a band apart (margins 1e-9 to 4e-9) and
///   escalate, or their rims' placement does; a single cap builds at its
///   closed form, or its rim's placement escalates where
///   its margin of about `δ` rounds inside the edge (9.999999995e-9 in one
///   re-posed order). D4's ε < δ < K·ε.
#[test]
fn the_near_node_poses_at_the_witness_band() {
    let tol = Tol::witness();
    if geom_core::Band::linear(tol).unwrap().zero() != 1e-9 {
        test_utils::vacuity::stood_down(
            "the witness band only",
            "the near-node acceptance poses are stated at ε 1e-9; the band-relative \
             row `an_edge_near_a_node_crosses_through_the_bisector_planes` holds them \
             at every ε",
        );
        return;
    }
    let across = |b: &Body<f64>| spin(b, Vec3::new(1.0, 0.0, 0.0), -PI / 2.0);
    let v = |d: f64| PI * (2.0 - d);
    let beyond = |d: f64| {
        let s = (1.0 - d * d).sqrt();
        2.0 * (s.powi(3) / 3.0 - d * d.acos() + s)
    };
    // Both caps: `x² + y² ≤ 1`, `x² + z² ≤ 1`, `y ≥ d`, `z ≥ d`. At each
    // `x` the section is the square `[d, s]²`, `s = √(1 − x²)`, so the
    // piece is `∫ (s − d)² dx` over `|x| ≤ X = √(1 − d²)`.
    let both = |d: f64| {
        let x = (1.0 - d * d).sqrt();
        2.0 * x - 2.0 * x.powi(3) / 3.0 - 2.0 * d * d.acos()
    };
    for d in [1e-8, 1e-6, 1e-4] {
        let poses = [
            (
                "A's seam",
                rod_at(-2.0, 2.0, d),
                across(&rod(-2.0, 2.0)),
                4.0 * PI,
                4.0 * PI,
                16.0 / 3.0,
            ),
            (
                "B's seam",
                rod(-2.0, 2.0),
                across(&rod_at(-2.0, 2.0, d)),
                4.0 * PI,
                4.0 * PI,
                16.0 / 3.0,
            ),
            (
                "A's cap",
                rod(d, 2.0),
                across(&rod(-2.0, 2.0)),
                v(d),
                4.0 * PI,
                beyond(d),
            ),
            (
                "B's cap",
                rod(-2.0, 2.0),
                across(&rod(d, 2.0)),
                4.0 * PI,
                v(d),
                beyond(d),
            ),
            (
                "both caps",
                rod(d, 2.0),
                across(&rod(d, 2.0)),
                v(d),
                v(d),
                both(d),
            ),
        ];
        let mut single_cap_builds = 0;
        for (what, a, b, va, vb, cap) in poses {
            for (pose, a, b) in [
                ("direct", a.clone(), b.clone()),
                ("re-posed", repose(&a), repose(&b)),
            ] {
                let (a, b) = (finished("A", a, tol), finished("B", b, tol));
                for (order, x, y, vx, vy) in [("A, B", &a, &b, va, vb), ("B, A", &b, &a, vb, va)] {
                    for (op, r, closed) in [
                        ("∪", topo::union(x, y, tol), vx + vy - cap),
                        ("∩", topo::intersect(x, y, tol), cap),
                        ("∖", topo::subtract(x, y, tol), vx - cap),
                    ] {
                        let cut = format!("{what} δ = {d:e}, {pose} ({order}), {op}");
                        let seam = what.ends_with("seam");
                        if d == 1e-8 {
                            // The witness band's escalation edge.
                            let decision = match &r {
                                Err(topo::BooleanError::Escalated { decision, .. }) => {
                                    Some(*decision)
                                }
                                _ => None,
                            };
                            if seam {
                                // The node stands `δ` off the seam: its
                                // placement in the seam's face.
                                assert!(
                                    matches!(
                                        decision,
                                        Some(topo::BooleanDecision::Containment { .. })
                                    ),
                                    "{cut}: the node's placement in band, got {r:?}"
                                );
                                continue;
                            }
                            if what == "both caps" {
                                // A rim's placement `δ` off its boundary, or
                                // the two caps' sectors a band apart.
                                assert!(
                                    matches!(
                                        decision,
                                        Some(
                                            topo::BooleanDecision::Containment { .. }
                                                | topo::BooleanDecision::Coincidence(
                                                    topo::Coincide::Sectors
                                                        | topo::Coincide::SectorSide,
                                                    _
                                                )
                                        )
                                    ),
                                    "{cut}: a rim or the caps' sectors in band, got {r:?}"
                                );
                                continue;
                            }
                            // A single cap: its rim's placement in the
                            // partner's face, `δ` off its boundary, lands
                            // either side of the edge by its rounding.
                            if r.is_err() {
                                assert!(
                                    matches!(
                                        decision,
                                        Some(topo::BooleanDecision::Containment { .. })
                                    ),
                                    "{cut}: the rim's placement in band, got {r:?}"
                                );
                                continue;
                            }
                            single_cap_builds += 1;
                        }
                        let Ok(BooleanResult::Body(out)) = r else {
                            panic!("{cut}: builds, got {r:?}");
                        };
                        let m = topo::mass_properties(
                            &out.body,
                            tol,
                            topo::test_support::pinned_resolution(tol),
                        )
                        .unwrap();
                        assert!(
                            (m.enclosure.volume - closed).abs() <= m.enclosure.volume_pad + 1e-9,
                            "{cut}: volume {} ± {}, want {closed}",
                            m.enclosure.volume,
                            m.enclosure.volume_pad
                        );
                    }
                }
            }
        }
        if d == 1e-8 {
            assert!(
                single_cap_builds > 0,
                "a single cap 1e-8 off builds in at least one pose and order"
            );
        }
    }
}

/// **A single cap just past the band builds as the parting leaves it**:
/// A's or B's cap `δ = √(2·c·K·ε)` off the other rod's axis, `c` 1.1,
/// 1.2 and 1.3, and `δ = 1e-4`. The cap's rim arcs from each pierce to
/// the node lie about `δ²/2 = c·K·ε` off the other rod's wall at their
/// middles and nearer it toward their ends, so some of their points
/// read in band of the wall and their middles off it: the arc does not
/// lie in the wall, and the parting leaves it alone (an in-band point
/// beside a definite one decides nothing, and the wall's side is read
/// at its carrier's foot). Every op, both orders and both poses, at
/// every ε: no cell refuses at the parting (`EdgeInFace`), and each
/// that builds is at its closed form. At ε 1e-9 and 1e-6 every cell
/// builds; at 1e-12 the merge refuses some, as it does without the
/// parting.
#[test]
fn a_single_cap_just_past_the_band_builds_as_the_parting_leaves_it() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    // At ε 1e-9 and coarser every cell builds; at 1e-12 the merge
    // refuses some.
    let at_witness = band.zero() >= 1e-9;
    let across = |b: &Body<f64>| spin(b, Vec3::new(1.0, 0.0, 0.0), -PI / 2.0);
    let v = |d: f64| PI * (2.0 - d);
    let beyond = |d: f64| {
        let s = (1.0 - d * d).sqrt();
        2.0 * (s.powi(3) / 3.0 - d * d.acos() + s)
    };
    let ds = [1.1, 1.2, 1.3]
        .map(|c: f64| (2.0 * c * band.escalate()).sqrt())
        .into_iter()
        .chain([1e-4]);
    for d in ds {
        for (what, a, b, va, vb) in [
            (
                "A's cap",
                rod(d, 2.0),
                across(&rod(-2.0, 2.0)),
                v(d),
                4.0 * PI,
            ),
            (
                "B's cap",
                rod(-2.0, 2.0),
                across(&rod(d, 2.0)),
                4.0 * PI,
                v(d),
            ),
        ] {
            let cap = beyond(d);
            for (pose, a, b) in [
                ("direct", a.clone(), b.clone()),
                ("re-posed", repose(&a), repose(&b)),
            ] {
                let (a, b) = (finished("A", a, tol), finished("B", b, tol));
                for (order, x, y, vx, vy) in [("A, B", &a, &b, va, vb), ("B, A", &b, &a, vb, va)] {
                    for (op, r, closed) in [
                        ("∪", topo::union(x, y, tol), vx + vy - cap),
                        ("∩", topo::intersect(x, y, tol), cap),
                        ("∖", topo::subtract(x, y, tol), vx - cap),
                    ] {
                        let cut = format!("{what} δ = {d:e}, {pose} ({order}), {op}");
                        let out = match r {
                            Ok(BooleanResult::Body(out)) => out,
                            Err(e) if !at_witness => {
                                assert!(
                                    !matches!(e, topo::BooleanError::EdgeInFace { .. }),
                                    "{cut}: the parting refuses, {e:?}"
                                );
                                continue;
                            }
                            r => panic!("{cut}: builds, got {r:?}"),
                        };
                        let m = topo::mass_properties(
                            &out.body,
                            tol,
                            topo::test_support::pinned_resolution(tol),
                        )
                        .unwrap();
                        assert!(
                            (m.enclosure.volume - closed).abs() <= m.enclosure.volume_pad + 1e-9,
                            "{cut}: volume {} ± {}, want {closed}",
                            m.enclosure.volume,
                            m.enclosure.volume_pad
                        );
                    }
                }
            }
        }
    }
}

/// **A pierce whose branch the sweep did not record keeps the tangent
/// plane.** With every recorded branch forgotten
/// ([`topo::forget_branches`]), A's cap `30·escalate` off the station
/// reads its pierces' sides as before the record, on the pierced wall's
/// tangent plane, where the sagitta charge refuses
/// `CurvedSectorSideUnsupported`; with the record it builds.
#[test]
fn a_pierce_with_no_recorded_branch_reads_the_tangent_plane() {
    let tol = Tol::witness();
    let d = 30.0 * geom_core::Band::linear(tol).unwrap().escalate();
    let across = |b: &Body<f64>| spin(b, Vec3::new(1.0, 0.0, 0.0), -PI / 2.0);
    let (a, b) = (
        finished("A", rod(d, 2.0), tol),
        finished("B", across(&rod(-2.0, 2.0)), tol),
    );
    topo::forget_branches(true);
    let forgotten = topo::intersect(&a, &b, tol);
    topo::forget_branches(false);
    assert!(
        matches!(
            forgotten,
            Err(topo::BooleanError::CurvedSectorSideUnsupported { .. })
        ),
        "no record: the tangent plane's sagitta, got {forgotten:?}"
    );
    let recorded = topo::intersect(&a, &b, tol);
    assert!(
        matches!(recorded, Ok(BooleanResult::Body(_))),
        "the record: builds, got {recorded:?}"
    );
}

/// **A rim vertex on a node refuses as a node on the boundary.** A branch
/// rod capped at the axes' meeting point with its seam turned onto the
/// node puts its rim's one vertex there, on both bisector planes: the
/// crossing reads an operand vertex at a node, which no lane builds yet,
/// and names it `NodeUnbuilt(OnBoundary)` with its recourse. The same
/// branch capped `10·√band` off the station (`1e-3` at the witness
/// band), its seam still at the node's angle, builds at its closed form. Both operand roles, both poses,
/// every operation.
#[test]
fn a_rim_vertex_on_a_node_refuses_as_a_node_on_the_boundary() {
    let tol = Tol::witness();
    let across = |b: &Body<f64>| spin(b, Vec3::new(1.0, 0.0, 0.0), -PI / 2.0);
    for (what, a, b) in [
        (
            "A's rim vertex",
            rod_at(0.0, 2.0, 0.0),
            across(&rod(-2.0, 2.0)),
        ),
        (
            "B's rim vertex",
            rod(-2.0, 2.0),
            across(&rod_at(0.0, 2.0, 0.0)),
        ),
    ] {
        for (pose, a, b) in [
            ("direct", a.clone(), b.clone()),
            ("re-posed", repose(&a), repose(&b)),
        ] {
            let (a, b) = (finished("A", a, tol), finished("B", b, tol));
            for (op, r) in [
                ("∪", topo::union(&a, &b, tol)),
                ("∩", topo::intersect(&a, &b, tol)),
                ("A∖B", topo::subtract(&a, &b, tol)),
                ("B∖A", topo::subtract(&b, &a, tol)),
            ] {
                assert!(
                    matches!(
                        r,
                        Err(topo::BooleanError::NodeUnbuilt {
                            why: topo::NodeWhy::OnBoundary(_),
                            ..
                        })
                    ),
                    "{what}, {pose}, {op}: a node at an operand vertex, got {r:?}"
                );
            }
        }
    }
    let dz = 10.0 * geom_core::Band::linear(tol).unwrap().escalate().sqrt();
    let (a, b) = (
        finished("A", rod_at(dz, 2.0, 0.0), tol),
        finished("B", across(&rod(-2.0, 2.0)), tol),
    );
    let s = (1.0 - dz * dz).sqrt();
    let cap = 2.0 * (s.powi(3) / 3.0 - dz * dz.acos() + s);
    let (va, vb) = (PI * (2.0 - dz), 4.0 * PI);
    for (op, r, want) in [
        ("∪", topo::union(&a, &b, tol), va + vb - cap),
        ("∩", topo::intersect(&a, &b, tol), cap),
        ("A∖B", topo::subtract(&a, &b, tol), va - cap),
        ("B∖A", topo::subtract(&b, &a, tol), vb - cap),
    ] {
        let Ok(BooleanResult::Body(out)) = r else {
            panic!("capped {dz:e} off, {op}: builds, got {r:?}");
        };
        // Its cap's lunes are the mesh's
        // (`work/tess/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`).
        let m = topo::mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap();
        assert!(
            (m.enclosure.volume - want).abs() <= m.enclosure.volume_pad + 1e-9,
            "capped {dz:e} off, {op}: volume {} ± {}, want {want}",
            m.enclosure.volume,
            m.enclosure.volume_pad
        );
    }
}

/// **At oblique angles a seam near a node builds**: A's seam turned `δ`
/// off a node, at 35° with `r = 1` and at 60° with `r = 0.5`, crosses the
/// partner wall at two pierces that read it through their bisector
/// plane, and the arcs from each pierce to the node read their wedge at
/// their faces' reach. At `δ = √band`, `3·√band` and `5·√band` every
/// operation builds. Every operation, both operand orders.
#[test]
fn an_oblique_seam_near_a_node_builds() {
    let tol = Tol::witness();
    let root = geom_core::Band::linear(tol).unwrap().escalate().sqrt();
    for (deg, r) in [(35.0_f64, 1.0), (60.0, 0.5)] {
        for k in [1.0, 3.0, 5.0] {
            let delta = k * root;
            let a = spin(&cyl(r, H), Vec3::new(0.0, 0.0, 1.0), delta);
            let b = spin(
                &spin(&cyl(r, H), Vec3::new(0.0, 0.0, 1.0), PHI),
                Vec3::new(1.0, 0.0, 0.0),
                deg.to_radians(),
            );
            let (a, b) = (finished("A", a, tol), finished("B", b, tol));
            for (order, x, y) in [("A, B", &a, &b), ("B, A", &b, &a)] {
                for (op, got) in [
                    ("∪", topo::union(x, y, tol)),
                    ("∩", topo::intersect(x, y, tol)),
                    ("∖", topo::subtract(x, y, tol)),
                ] {
                    let what = format!("{deg}°, r = {r}, δ = {delta:e} ({order}), {op}");
                    assert!(got.is_ok(), "{what}: builds, got {got:?}");
                }
            }
        }
    }
}

/// A brick less a unit rod through it, its seam turned [`PHI`]: a bore,
/// the wall's face bounding the brick's material away from its axis.
fn bored(brick: [(f64, f64); 3], rod: Body<f64>) -> Body<f64> {
    let tol = Tol::witness();
    let brick = finished(
        "the brick",
        sweep::test_support::brick(brick[0], brick[1], brick[2], tol),
        tol,
    );
    topo::subtract(&brick, &finished("the rod", rod, tol), tol)
        .expect("the bore")
        .body()
        .expect("the bore is a body")
        .body
        .clone()
        .into_body()
}

/// A unit rod about `y`, half-length `h`, its seam turned [`PHI`].
fn rod_y(h: f64) -> Body<f64> {
    spin(
        &spin(&cyl(1.0, h), Vec3::new(0.0, 0.0, 1.0), PHI),
        Vec3::new(1.0, 0.0, 0.0),
        PI / 2.0,
    )
}

/// A built body's volume against `want`: within its certified bracket,
/// and within half a percent of its mesh's enclosed volume (the chordal
/// deficit at `2e-3` is a fraction of that), an oracle independent of
/// the quadrature.
fn assert_volume(body: &topo::AtRestBody<f64>, want: f64, what: &str) {
    let tol = Tol::witness();
    let m = topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol)).unwrap();
    assert!(
        (m.enclosure.volume - want).abs() <= m.enclosure.volume_pad + 1e-9,
        "{what}: volume {} ± {}, want {want}",
        m.enclosure.volume,
        m.enclosure.volume_pad
    );
    let mesh = mesh::tessellate(body, 2e-3, tol).expect("the result tessellates");
    // Where two solids of the result touch at a node their faces there
    // are tangent, and a chord beside the node closes a triangle on
    // each: a mesh edge four triangles hold, which the at-rest body has
    // no counterpart of. One solid is held watertight.
    if body.solids().count() == 1
        && let Err(e) = mesh::validate::check_mesh(&mesh)
    {
        panic!("{what}: a watertight mesh, got {e:?}");
    }
    let meshed = mesh::validate::signed_volume(&mesh);
    assert!(
        (meshed - want).abs() <= 5e-3 * want,
        "{what}: the mesh encloses {meshed}, want {want}"
    );
}

/// **Two crossing bores build every operation**: each brick's wall
/// bounds its material away from its axis, so round each node A's
/// sectors outside B and B's inside A are both those about `±â₂`, the
/// ring the node mints. `A` is `[−2, 2]³` bored about `z`, `B` is
/// `[−1.5, 1.5] × [−3, 3] × [−1.5, 1.5]` bored about `y`, both bores of
/// unit radius through the origin; their common box is `3 × 4 × 3`,
/// which holds the bores `3π` and `4π` of it and their Steinmetz `16/3`
/// whole, so `∩ = 36 − 7π + 16/3`. Both operand orders and the re-posed
/// twin, against the closed forms and the mesh.
#[test]
fn two_crossing_bores_build_every_operation_at_the_closed_forms() {
    let tol = Tol::witness();
    let a = bored(
        [(-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)],
        spin(&cyl(1.0, 3.0), Vec3::new(0.0, 0.0, 1.0), PHI),
    );
    let b = bored([(-1.5, 1.5), (-3.0, 3.0), (-1.5, 1.5)], rod_y(4.0));
    let (va, vb) = (64.0 - 4.0 * PI, 54.0 - 6.0 * PI);
    let cap = 36.0 - 7.0 * PI + 16.0 / 3.0;
    for (pose, a, b) in [
        ("direct", a.clone(), b.clone()),
        ("re-posed", repose(&a), repose(&b)),
    ] {
        let (a, b) = (finished("A", a, tol), finished("B", b, tol));
        for (order, x, y, vx, vy) in [("A, B", &a, &b, va, vb), ("B, A", &b, &a, vb, va)] {
            for (op, r, want) in [
                ("∪", topo::union(x, y, tol), vx + vy - cap),
                ("∩", topo::intersect(x, y, tol), cap),
                ("∖", topo::subtract(x, y, tol), vx - cap),
            ] {
                let what = format!("{pose} ({order}), {op}");
                let Ok(BooleanResult::Body(out)) = r else {
                    panic!("{what}: two crossing bores build, got {r:?}");
                };
                assert_volume(&out.body, want, &what);
            }
        }
    }
}

/// **A rod against a bore builds every operation.** The rod's face
/// bounds its material on its axis's side and the bore's away from it,
/// so round each node A's sectors and B's sectors inside the other's
/// material are the same ones, those about the rod's axis: the node's
/// ring spans them on both walls, A's struts standing In. The rod runs
/// along `y` turned about `x` to make `θ` with the bore's axis `z`, the
/// bore the brick `[−2, 2]³` less a unit rod about `z`.
///
/// - At right angles, in closed form: the rod's piece in the brick
///   holds the bicylinder whole, so `∩ = 4π − 16/3`, `∪ = 64 + 16/3`,
///   `rod ∖ bore = 4π + 16/3`, `bore ∖ rod = 64 − 8π + 16/3`.
/// - At every angle, by inclusion–exclusion within the certified
///   brackets, anchored on the operands' own closed forms `8π` and
///   `64 − 4π`, and each volume within half a percent of its mesh's.
/// - Shape: `∪` is one shell touching itself at both nodes (two
///   valence-2 vertices at each, one per cone), passing 3′; `∩` is two
///   solids touching at the nodes, its 3′ the census lane's frontier
///   and only it; `rod ∖ bore` and `bore ∖ rod` have one valence-4
///   vertex at each node and pass 3′, `bore ∖ rod` one solid and
///   `rod ∖ bore` three where the rod is steep (its piece in the hole
///   walled off from its ends by the brick) and one where it is
///   shallow (the hole's mouths join them).
///
/// Both operand orders and the re-posed twin.
#[test]
fn a_rod_against_a_bore_builds_every_operation() {
    let tol = Tol::witness();
    let measure = |b: &topo::AtRestBody<f64>| {
        let m = topo::mass_properties(b, tol, topo::test_support::pinned_resolution(tol)).unwrap();
        (m.enclosure.volume, m.enclosure.volume_pad)
    };
    let bore = bored(
        [(-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)],
        spin(&cyl(1.0, 3.0), Vec3::new(0.0, 0.0, 1.0), PHI),
    );
    let rod_at = |deg: f64| {
        spin(
            &rod_y(4.0),
            Vec3::new(1.0, 0.0, 0.0),
            PI / 2.0 - deg.to_radians(),
        )
    };
    let at_origin = [1.0, -1.0].map(|x| Point3::new(x, 0.0, 0.0));
    let r = geom_core::Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    let place =
        geom_core::Affine3::from_parts(r.linear, r.translation + Vec3::new(0.3, -0.45, 0.6));
    /// A pose: its name, the rod, the bored brick, the nodes, whether it
    /// is square, and how many solids `rod ∖ bore` holds.
    type Pose = (String, Body<f64>, Body<f64>, [Point3<f64>; 2], bool, usize);
    // Steep, the rod's piece in the hole is walled off from its ends by
    // the brick; shallow, the hole's mouths join them.
    let rod_pieces = |deg: f64| if (55.0..=125.0).contains(&deg) { 3 } else { 1 };
    let mut poses: Vec<Pose> = [90.0_f64, 60.0, 35.0, 150.0]
        .into_iter()
        .map(|deg| {
            (
                format!("{deg}°"),
                rod_at(deg),
                bore.clone(),
                at_origin,
                deg == 90.0,
                rod_pieces(deg),
            )
        })
        .collect();
    poses.push((
        "90°, re-posed".to_string(),
        repose(&rod_at(90.0)),
        repose(&bore),
        at_origin.map(|n| place.transform_point(n)),
        true,
        rod_pieces(90.0),
    ));
    let bicylinder = 16.0 / 3.0;
    for (what, rod, bore, nodes, square, pieces) in poses {
        let (rod, bore) = (
            finished("the rod", rod, tol),
            finished("the bore", bore, tol),
        );
        let (vr, vb) = (measure(&rod), measure(&bore));
        let close = |got: (f64, f64), want: f64, pad: f64, what: &str| {
            assert!(
                (got.0 - want).abs() <= got.1 + pad + 1e-9,
                "{what}: volume {} ± {}, want {want} ± {pad}",
                got.0,
                got.1
            );
        };
        // The rod `|y| ≤ 4` of radius 1, the brick less a unit bore
        // through it.
        close(vr, 8.0 * PI, 0.0, &format!("{what}, the rod"));
        close(vb, 64.0 - 4.0 * PI, 0.0, &format!("{what}, the bore"));
        let rb = built(
            topo::subtract(&rod, &bore, tol),
            nodes,
            &format!("{what}, rod ∖ bore"),
        );
        let br = built(
            topo::subtract(&bore, &rod, tol),
            nodes,
            &format!("{what}, bore ∖ rod"),
        );
        let (vrb, vbr) = (measure(&rb.body), measure(&br.body));
        let mut vis = Vec::new();
        for (order, x, y) in [("rod, bore", &rod, &bore), ("bore, rod", &bore, &rod)] {
            let cut = |op: &str| format!("{what} ({order}), {op}");
            let i = built(topo::intersect(x, y, tol), nodes, &cut("∩"));
            let u = built(topo::union(x, y, tol), nodes, &cut("∪"));
            let (vi, vu) = (measure(&i.body), measure(&u.body));
            close(
                (vu.0 + vi.0, vu.1 + vi.1),
                vr.0 + vb.0,
                vr.1 + vb.1,
                &cut("∪ + ∩"),
            );
            if square {
                // At right angles the rod's piece in the brick, `4π`,
                // holds the bicylinder whole.
                close(vi, 4.0 * PI - bicylinder, 0.0, &cut("∩, closed form"));
                close(vu, 64.0 + bicylinder, 0.0, &cut("∪, closed form"));
            }
            for (got, op) in [(&i, "∩"), (&u, "∪")] {
                mesh_agrees(&got.body, tol, &cut(op));
            }
            assert_eq!(
                (u.solids, u.body.shells().count(), &u.at_nodes),
                (1, 1, &[vec![2, 2], vec![2, 2]]),
                "{}: one shell touching itself at both nodes",
                cut("∪")
            );
            assert_eq!(
                (i.solids, &i.at_nodes),
                (2, &[vec![2, 2], vec![2, 2]]),
                "{}: two solids, one vertex per cone",
                cut("∩")
            );
            assert!(
                i.three_prime
                    .as_ref()
                    .is_err_and(|errs| only_curved_within_reach(&i.body, errs)),
                "{}: the census lane's frontier, and only it: {:?}",
                cut("∩"),
                i.three_prime
            );
            assert!(
                u.three_prime.is_ok(),
                "{}: 3′ {:?}",
                cut("∪"),
                u.three_prime
            );
            vis.push(vi);
        }
        for vi in vis {
            close(
                vrb,
                vr.0 - vi.0,
                vr.1 + vi.1,
                &format!("{what}, rod ∖ bore"),
            );
            close(
                vbr,
                vb.0 - vi.0,
                vb.1 + vi.1,
                &format!("{what}, bore ∖ rod"),
            );
        }
        if square {
            close(
                vrb,
                4.0 * PI + bicylinder,
                0.0,
                &format!("{what}, rod ∖ bore, closed form"),
            );
            close(
                vbr,
                64.0 - 8.0 * PI + bicylinder,
                0.0,
                &format!("{what}, bore ∖ rod, closed form"),
            );
        }
        for (got, op, solids) in [(&rb, "rod ∖ bore", pieces), (&br, "bore ∖ rod", 1)] {
            let cut = format!("{what}, {op}");
            mesh_agrees(&got.body, tol, &cut);
            assert_eq!(
                (got.solids, &got.at_nodes),
                (solids, &[vec![4], vec![4]]),
                "{cut}: its solids, valence 4 at each node"
            );
            assert!(got.three_prime.is_ok(), "{cut}: 3′ {:?}", got.three_prime);
        }
    }
}

/// A built body's volume within half a percent of its mesh's enclosed
/// volume: volume only, since the results pinch at the nodes.
fn mesh_agrees(body: &topo::AtRestBody<f64>, tol: Tol, what: &str) {
    let mesh = mesh::tessellate(body, 2e-3, tol).expect("the result tessellates");
    let meshed = mesh::validate::signed_volume(&mesh);
    let v = topo::test_support::pinned_volume("the measured body", body, tol).volume;
    assert!(
        (meshed - v).abs() <= 5e-3 * v,
        "{what}: the mesh encloses {meshed}, the body {v}"
    );
}

/// **Two crossing bores at a slant, one of them turned inside its brick**:
/// `A` is `[−2, 2]³` bored about `z`, `B` the brick
/// `[−1.5, 1.5] × [−3, 3] × [−1.5, 1.5]` bored along `y` turned about `x`
/// to make `θ` with `z`, so B's bore leaves its brick through the sides.
/// Every operation builds, both operand orders, and the re-posed twin:
///
/// - `∪` and `∩` are one solid with one valence-4 vertex at each node,
///   and pass 3′ on their own records;
/// - `A∖B` is **one solid pinched to itself at both nodes** — the pieces
///   in B's bore join the frame round it through the bore's exits — each
///   node two valence-2 vertices of one shell, one per cone, the pinch's
///   two sides touching within the shell (D1: a piece that touches itself
///   stays one solid), and it passes 3′ on its own records;
/// - `B∖A` is four solids touching at the nodes, its 3′ the census
///   lane's frontier alone.
///
/// The volumes close under inclusion–exclusion (`∪ + ∩ = V_A + V_B`,
/// `A∖B = V_A − ∩`, `B∖A = V_B − ∩`) within the certified brackets, and
/// each sits within half a percent of its mesh's enclosed volume. The
/// mesh is held to volume alone here: at a pinch the two cones' faces are
/// tangent at the node, and a coarse mesh closes one chord beside it on
/// both, an edge four triangles hold that the body has no counterpart of.
#[test]
fn slanted_bores_build_every_operation_one_difference_pinched_to_itself() {
    // Pinned red since PR 5664: at ε = 1e-12 the 3° pose's B ∖ A exhausts its
    // probe ray (`work/tang/main-red-slanted-bores-b-minus-a-exhausts-its-probe-ray-at-1e-12`,
    // `work/charttail/main-red-ringed-curved-doors-reds-four-sweep-rows`).
    test_utils::pinned_red::row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "the slanted bores",
        &[
            "3° (B, A), ∖: the node builds, got Err(Pieces(Probe { shell: ShellKey(2v5), source: Loop(RayExhausted { loop: LoopKey(16v5) }) }))",
        ],
        slanted_bores_build_every_operation_one_difference_pinched_to_itself_row,
    );
}

/// The row [`slanted_bores_build_every_operation_one_difference_pinched_to_itself`] runs, under its pin.
fn slanted_bores_build_every_operation_one_difference_pinched_to_itself_row() {
    let tol = Tol::witness();
    let measure = |b: &topo::AtRestBody<f64>| {
        let m = topo::mass_properties(b, tol, topo::test_support::pinned_resolution(tol)).unwrap();
        (m.enclosure.volume, m.enclosure.volume_pad)
    };
    let a0 = bored(
        [(-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)],
        spin(&cyl(1.0, 3.0), Vec3::new(0.0, 0.0, 1.0), PHI),
    );
    let tilted = |deg: f64| {
        let phi = PI / 2.0 - deg.to_radians();
        bored(
            [(-1.5, 1.5), (-3.0, 3.0), (-1.5, 1.5)],
            spin(&rod_y(8.0), Vec3::new(1.0, 0.0, 0.0), phi),
        )
    };
    let mut poses = Vec::new();
    for deg in [60.0_f64, 35.0, 150.0, 12.0, 3.0] {
        poses.push((
            format!("{deg}°"),
            a0.clone(),
            tilted(deg),
            [1.0, -1.0].map(|x| Point3::new(x, 0.0, 0.0)),
        ));
    }
    let r = geom_core::Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    let place =
        geom_core::Affine3::from_parts(r.linear, r.translation + Vec3::new(0.3, -0.45, 0.6));
    poses.push((
        "35°, re-posed".to_string(),
        repose(&a0),
        repose(&tilted(35.0)),
        [1.0, -1.0].map(|x| place.transform_point(Point3::new(x, 0.0, 0.0))),
    ));
    for (what, a, b, nodes) in poses {
        let (a, b) = (finished("A", a, tol), finished("B", b, tol));
        let (va, vb) = (measure(&a), measure(&b));
        for (order, x, y, (vx, vy), self_pinched) in [
            ("A, B", &a, &b, (va, vb), true),
            ("B, A", &b, &a, (vb, va), false),
        ] {
            let cut = |op: &str| format!("{what} ({order}), {op}");
            let i = built(topo::intersect(x, y, tol), nodes, &cut("∩"));
            let u = built(topo::union(x, y, tol), nodes, &cut("∪"));
            let d = built(topo::subtract(x, y, tol), nodes, &cut("∖"));
            let (vi, vu, vd) = (measure(&i.body), measure(&u.body), measure(&d.body));
            let close = |got: f64, want: f64, pad: f64, op: &str| {
                assert!(
                    (got - want).abs() <= pad + 1e-9,
                    "{}: volume {got}, want {want} ± {pad}",
                    cut(op)
                );
            };
            close(vu.0 + vi.0, vx.0 + vy.0, vu.1 + vi.1 + vx.1 + vy.1, "∪ + ∩");
            close(vd.0, vx.0 - vi.0, vd.1 + vx.1 + vi.1, "∖");
            for (got, op) in [(&i, "∩"), (&u, "∪"), (&d, "∖")] {
                let mesh = mesh::tessellate(&got.body, 2e-3, tol).expect("the result tessellates");
                let meshed = mesh::validate::signed_volume(&mesh);
                let v = measure(&got.body).0;
                assert!(
                    (meshed - v).abs() <= 5e-3 * v,
                    "{}: the mesh encloses {meshed}, the body {v}",
                    cut(op)
                );
            }
            for (got, op) in [(&i, "∩"), (&u, "∪")] {
                assert_eq!(got.solids, 1, "{}: one solid", cut(op));
                assert_eq!(
                    got.at_nodes,
                    [vec![4], vec![4]],
                    "{}: valence 4 at each node",
                    cut(op)
                );
                assert!(
                    got.three_prime.is_ok(),
                    "{}: 3′ {:?}",
                    cut(op),
                    got.three_prime
                );
            }
            assert_eq!(
                d.at_nodes,
                [vec![2, 2], vec![2, 2]],
                "{}: each node two valence-2 vertices, one per cone",
                cut("∖")
            );
            if self_pinched {
                assert_eq!(
                    (d.solids, d.body.shells().count()),
                    (1, 1),
                    "{}: one solid, one shell, pinched to itself",
                    cut("∖")
                );
                assert!(
                    d.three_prime.is_ok(),
                    "{}: 3′ {:?}",
                    cut("∖"),
                    d.three_prime
                );
            } else {
                assert_eq!(d.solids, 4, "{}: four solids", cut("∖"));
                assert!(
                    d.three_prime
                        .as_ref()
                        .err()
                        .is_some_and(|errs| only_curved_within_reach(&d.body, errs)),
                    "{}: the census lane's frontier alone: {:?}",
                    cut("∖"),
                    d.three_prime
                );
            }
        }
    }
}

/// **A pair that is no node's keeps its own route**: a pin of radius 0.9
/// in a unit bore, tilted `α` about `x`, its wall nearly parallel to the
/// bore's and never meeting it. The node pass reads every wall pair whose
/// boxes overlap, so a reading that refuses for such a pair (here the
/// parallelism, in band) must pre-empt nothing: at every `α` and every
/// operation the boolean answers what its own route does — the body, or
/// at the witness band and `α = 3e-10` the fallback extent's refusal —
/// and never the node reading's escalation.
#[test]
fn a_tilted_pin_in_a_bore_keeps_its_own_route() {
    let tol = Tol::witness();
    let witness = geom_core::Band::linear(tol).unwrap().zero() == 1e-9;
    let hole = bored(
        [(-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)],
        spin(&cyl(1.0, 3.0), Vec3::new(0.0, 0.0, 1.0), PHI),
    );
    for alpha in [0.0_f64, 1e-11, 3e-10, 3e-9] {
        let pin = spin(
            &spin(&cyl(0.9, 3.0), Vec3::new(0.0, 0.0, 1.0), PHI),
            Vec3::new(1.0, 0.0, 0.0),
            alpha,
        );
        let (a, b) = (
            finished("the bore", hole.clone(), tol),
            finished("the pin", pin, tol),
        );
        for (op, r) in [
            ("∪", topo::union(&a, &b, tol)),
            ("∩", topo::intersect(&a, &b, tol)),
            ("bore ∖ pin", topo::subtract(&a, &b, tol)),
            ("pin ∖ bore", topo::subtract(&b, &a, tol)),
        ] {
            let what = format!("α = {alpha:e}, {op}");
            if witness && alpha == 3e-10 {
                assert!(
                    matches!(r, Err(topo::BooleanError::FallbackExtentUnsupported { .. })),
                    "{what}: the fallback extent's refusal, got {r:?}"
                );
            } else {
                assert!(r.is_ok(), "{what}: builds, got {r:?}");
            }
        }
    }
}

/// **A rod turned within a hair of its bore's axis answers the same in
/// either operand order.** At `θ` this small the rod's wall lies within
/// the band of the bore's along its whole length. Its rulings and the
/// bore's are read as two lines where they can meet, their overlap
/// (`geom_brep::curve_pair`'s line pose): the bore's block is four metres
/// deep, so over it a `1e-8°` tilt moves the rod's ruling `7e-10` from
/// the bore's, within the zero band, whichever of the two the sweep split
/// and however. The orders agree on the outcome, and on the decision and
/// margin of an escalation, at every band; which poses build is the
/// band's.
///
/// At the default band:
///
/// - the rod along `y` turned to `1e-9°` or `1e-8°` builds alike in ∩ and
///   ∪: within the block it lies within `7e-10` of the bore, so it is
///   glued, as at `1e-9°` (until 2026-10-15 the `1e-8°` pose escalated
///   alike, the tilt levered by the rod's whole eight metres, and by the
///   split's pieces in one order: the lever was how the sweep split the
///   edge, not where the lines can meet);
/// - the rod about `z` tilted `1e-8°` builds alike, its seam on the
///   tilt's plane (turned `0` or `π`) or a quarter turn off it;
/// - at `1e-7°` both orders escalate on one margin, but at two
///   decisions, which this row does not hold alike
///   (`work/tangtail/a-glued-rod-answers-differently-by-order-off-the-pinned-angles.md`);
///   at a band tight enough to read that tilt definite (`1e-12`), both
///   orders refuse at a pcurve certificate of the long bisector ellipse,
///   past what its stored form evaluates within the band
///   (`work/germ/a-sliver-crossings-long-bisector-ellipse-is-served-past-representability.md`),
///   one in the join and one in the merge.
#[test]
fn a_rod_glued_in_its_bore_answers_alike_in_either_order() {
    let tol = Tol::witness();
    let default_band = tol.eps() == geom_core::tolerance::DEFAULT_EPS;
    let bore = finished(
        "the bore",
        bored(
            [(-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0)],
            spin(&cyl(1.0, 3.0), Vec3::new(0.0, 0.0, 1.0), PHI),
        ),
        tol,
    );
    /// An outcome as the orders are compared: a volume, an escalation's
    /// decision and margin, or another refusal's kind.
    #[derive(Debug)]
    enum Outcome {
        Built(f64, f64),
        Escalated(String, f64),
        /// The refusal's kind, and whether it is a pcurve certificate's.
        Refused(String, bool),
    }
    let outcome = |r: Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(out)) => {
            let m =
                topo::mass_properties(&out.body, tol, topo::test_support::pinned_resolution(tol))
                    .unwrap();
            Outcome::Built(m.enclosure.volume, m.enclosure.volume_pad)
        }
        Ok(_) => Outcome::Built(0.0, 0.0),
        Err(topo::BooleanError::Escalated { decision, diag }) => Outcome::Escalated(
            format!("{decision:?}"),
            diag.margin
                .diagnostic_f64_for_error_text()
                .value()
                .expect("an f64 margin"),
        ),
        Err(e) => {
            let text = format!("{e:?}");
            Outcome::Refused(
                text.split([' ', '{', '(']).next().unwrap().to_string(),
                text.contains("Certify {") && text.contains("ResidualExceeded"),
            )
        }
    };
    let tilted = |rod: Body<f64>, deg: f64| {
        finished(
            "the rod",
            spin(&rod, Vec3::new(1.0, 0.0, 0.0), deg.to_radians()),
            tol,
        )
    };
    let along_y = |deg: f64| tilted(rod_y(4.0), 90.0 - deg);
    let about_z = |turn: f64, deg: f64| {
        tilted(
            spin(&cyl(1.0, 4.0), Vec3::new(0.0, 0.0, 1.0), PHI + turn),
            deg,
        )
    };
    /// A pose: its name, the rod, whether the orders must agree on the
    /// escalation's decision, and whether it builds at the default band.
    type Pose = (String, topo::AtRestBody<f64>, bool, bool);
    let poses: Vec<Pose> = vec![
        ("along y, 1e-9°".into(), along_y(1e-9), true, true),
        ("along y, 1e-8°".into(), along_y(1e-8), true, true),
        ("along y, 1e-7°".into(), along_y(1e-7), false, false),
        (
            "about z, seam 0, 1e-8°".into(),
            about_z(0.0, 1e-8),
            true,
            true,
        ),
        (
            "about z, seam π, 1e-8°".into(),
            about_z(PI, 1e-8),
            true,
            true,
        ),
        (
            "about z, seam π/2, 1e-8°".into(),
            about_z(PI / 2.0, 1e-8),
            true,
            true,
        ),
    ];
    for (what, rod, one_decision, builds) in poses {
        for (op, f) in [
            ("∩", topo::intersect::<f64> as fn(_, _, _) -> _),
            ("∪", topo::union::<f64>),
        ] {
            let (rb, br) = (outcome(f(&rod, &bore, tol)), outcome(f(&bore, &rod, tol)));
            let alike = match (&rb, &br) {
                (Outcome::Built(v, p), Outcome::Built(w, q)) => (v - w).abs() <= p + q + 1e-9,
                (Outcome::Escalated(d, m), Outcome::Escalated(e, n)) => {
                    (!one_decision || d == e) && (m - n).abs() <= 1e-15
                }
                (Outcome::Refused(a, x), Outcome::Refused(b, y)) => {
                    a == b || (!one_decision && *x && *y)
                }
                _ => false,
            };
            assert!(
                alike,
                "{what}, {op}: (rod, bore) {rb:?}, (bore, rod) {br:?}"
            );
            if default_band {
                assert_eq!(
                    matches!(rb, Outcome::Built(..)),
                    builds,
                    "{what}, {op}: {rb:?}"
                );
            }
        }
    }
}

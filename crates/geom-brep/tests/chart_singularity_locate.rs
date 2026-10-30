//! **Where a chart singularity lies against a face**
//! (`geom_brep::chart_singularity::locate`), on loops stated by hand: a
//! pole at a vertex, inside an edge, inside the face or outside it,
//! whatever the loop's carriers, and the readings that cannot place it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

use geom::{Curve3, Surface};
use geom_brep::chart_singularity::{
    FaceLoop, Lies, Located, Singularity, Unread, locate, traversal_corners,
};
use geom_brep::props::LoopEdge;
use geom_core::{Band, Point3, Vec3};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The circle in the plane `n·p = d` of the unit sphere, `n` a unit
/// vector, run counter-clockwise about `n`.
fn section(n: Vec3<f64>, d: f64) -> Curve3<f64> {
    let side = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u_ref = side.cross(n);
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0) + n * d,
        axis: n,
        radius: (1.0 - d * d).sqrt(),
        u_ref: u_ref / u_ref.norm(),
    }
}

/// One closed edge, the whole of `carrier`'s turn.
fn closed(carrier: Curve3<f64>, forward: bool) -> Vec<LoopEdge<f64>> {
    vec![LoopEdge::hand_built(carrier, 0.0, TAU, forward, 0, 0)]
}

/// Where each singular point lies, in the kernel's order, or the
/// reading's failure.
fn lies(surface: &Surface<f64>, loops: &[Vec<LoopEdge<f64>>], sense: bool) -> Vec<Located<f64>> {
    lies_at(surface, loops, sense, band())
}

/// [`lies`] at `band`, each loop's corners its traversal starts.
fn lies_at(
    surface: &Surface<f64>,
    loops: &[Vec<LoopEdge<f64>>],
    sense: bool,
    band: Band,
) -> Vec<Located<f64>> {
    let corners: Vec<Vec<Point3<f64>>> = loops.iter().map(|l| traversal_corners(l)).collect();
    let views: Vec<FaceLoop<'_, f64>> = loops
        .iter()
        .zip(&corners)
        .map(|(edges, corners)| FaceLoop { edges, corners })
        .collect();
    let images: Vec<Vec<LoopEdge<f64>>> = loops.iter().map(|l| hand_image(l)).collect();
    let images: Vec<&[LoopEdge<f64>]> = images.iter().map(Vec::as_slice).collect();
    locate(surface, &views, &images, sense, band)
}

/// **A hand-built loop's chart image**, the structure a body's caller
/// reads off half-edge identity (`topo::surface_boundary::chart_image`)
/// read here off the hand-built tags: an edge followed by one that walks
/// it straight back (tags swapped, traversal reversed, the same stored
/// interval on the same carrier), by the one reduction
/// ([`geom_brep::chart_image::without_reversals`]).
fn hand_image(edges: &[LoopEdge<f64>]) -> Vec<LoopEdge<f64>> {
    let back = |e: &LoopEdge<f64>, f: &LoopEdge<f64>| {
        e.start == f.end
            && e.end == f.start
            && e.forward != f.forward
            && e.t0 == f.t0
            && e.t1 == f.t1
            && format!("{:?}", e.carrier) == format!("{:?}", f.carrier)
    };
    geom_brep::chart_image::without_reversals(edges, back)
}

fn placed(located: &[Located<f64>]) -> Vec<(Singularity, Lies<f64>)> {
    located
        .iter()
        .map(|l| match &l.lies {
            Ok(lies) => (l.singularity, *lies),
            Err(why) => panic!("{:?} unread: {why:?}", l.singularity),
        })
        .collect()
}

/// **A rim-only cap holds its pole inside, and the side is the loop's
/// traversal and the face's sense together**: the latitude circle run
/// east bounds the cap above it under the chart normal, run west the
/// ball below it, and a flipped sense swaps them.
#[test]
fn a_rim_only_cap_holds_the_pole_on_its_side() {
    use Lies::{Interior as In, Outside as Out};
    use Singularity::{North as N, South as S};
    let rim = section(Vec3::new(0.0, 0.0, 1.0), 0.5);
    for (forward, sense, want) in [
        (true, true, [(N, In), (S, Out)]),
        (false, true, [(N, Out), (S, In)]),
        (true, false, [(N, Out), (S, In)]),
        (false, false, [(N, In), (S, Out)]),
    ] {
        assert_eq!(
            placed(&lies(
                &unit_sphere(),
                &[closed(rim.clone(), forward)],
                sense
            )),
            want,
            "rim forward {forward}, sense {sense}"
        );
    }
}

/// **A tilted cap holding a pole holds it with no meridian in its
/// loop**: the reading is form-neutral. The cap about a direction 0.3 rad
/// off the axis holds the north pole; tilted 1.3 rad, it holds neither.
#[test]
fn a_tilted_cap_holds_a_pole_by_its_region_not_its_meridians() {
    let tilted = |a: f64| Vec3::new(a.sin(), 0.0, a.cos());
    let near = placed(&lies(
        &unit_sphere(),
        &[closed(section(tilted(0.3), 0.5), true)],
        true,
    ));
    assert_eq!(
        near,
        [
            (Singularity::North, Lies::Interior),
            (Singularity::South, Lies::Outside)
        ],
        "tilted 0.3 rad"
    );
    let far = placed(&lies(
        &unit_sphere(),
        &[closed(section(tilted(1.3), 0.5), true)],
        true,
    ));
    assert_eq!(
        far,
        [
            (Singularity::North, Lies::Outside),
            (Singularity::South, Lies::Outside)
        ],
        "tilted 1.3 rad"
    );
}

/// **A hole that does not reach the pole leaves it inside**, and a hole
/// round the pole takes it out: the face's side of every loop.
#[test]
fn a_hole_takes_the_pole_out_only_where_it_rings_it() {
    let cap = closed(section(Vec3::new(0.0, 0.0, 1.0), 0.2), true);
    let tilted = Vec3::new(0.6_f64.sin(), 0.0, 0.6_f64.cos());
    let off_pole = closed(section(tilted, 0.95), false);
    let round_pole = closed(section(Vec3::new(0.0, 0.0, 1.0), 0.95), false);
    let read = |hole: &Vec<LoopEdge<f64>>| {
        placed(&lies(&unit_sphere(), &[cap.clone(), hole.clone()], true))[0].1
    };
    assert_eq!(read(&off_pole), Lies::Interior, "a hole off the pole");
    assert_eq!(read(&round_pole), Lies::Outside, "a hole round the pole");
}

/// The meridian circle through `+x` and the poles, `θ = π/2` the north.
fn meridian_xz() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **A lune's poles are its vertices; a great circle's are inside its
/// edge**, at the parameter of each pole.
#[test]
fn poles_at_a_vertex_and_inside_an_edge() {
    let yz = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(1.0, 0.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let lune = vec![
        LoopEdge::hand_built(meridian_xz(), -FRAC_PI_2, FRAC_PI_2, true, 0, 1),
        LoopEdge::hand_built(yz, -FRAC_PI_2, FRAC_PI_2, false, 1, 0),
    ];
    assert_eq!(
        placed(&lies(&unit_sphere(), &[lune], true)),
        [
            (Singularity::North, Lies::AtVertex { r#loop: 0, edge: 1 }),
            (Singularity::South, Lies::AtVertex { r#loop: 0, edge: 0 }),
        ]
    );
    let great = placed(&lies(&unit_sphere(), &[closed(meridian_xz(), true)], true));
    for ((which, lies), want) in great.into_iter().zip([FRAC_PI_2, 3.0 * FRAC_PI_2]) {
        let Lies::InsideEdge {
            r#loop: 0,
            edge: 0,
            t,
        } = lies
        else {
            panic!("{which:?}: {lies:?}")
        };
        assert!((t - want).abs() < 1e-12, "{which:?} at t = {t}, not {want}");
    }
}

/// **A distance the band cannot call is undecided, carrying it**: a rim
/// round the pole 5e-9 from it, between the band's zero and its
/// escalation.
#[test]
fn a_distance_in_band_is_undecided() {
    let d = 5e-9_f64;
    // Its height off the pole, `d²/2`, is below the unit's rounding.
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: d,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let located = lies(&unit_sphere(), &[closed(rim, true)], true);
    let Err(Unread::Boundary { distance, .. }) = &located[0].lies else {
        panic!("{:?}", located[0].lies)
    };
    assert!(
        (distance - d).abs() < 1e-12,
        "the distance carried: {distance}"
    );
}

/// **A carrier read by samples places a point on it or off it, against
/// its certified speed bound**: an ellipse through the north pole places
/// it inside the edge, and a small one far from both poles reads them
/// off it and goes on to the side.
#[test]
fn a_sampled_carrier_through_the_pole_places_it_on_the_edge() {
    let ellipse = |center: Point3<f64>, semi: f64| Curve3::Ellipse {
        center,
        axis: Vec3::new(-1.0, 0.0, 0.0),
        semi_u: semi,
        semi_v: semi,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let through = lies(
        &unit_sphere(),
        &[closed(ellipse(Point3::new(0.0, 0.0, 0.0), 1.0), true)],
        true,
    );
    assert!(
        matches!(
            through[0].lies,
            Ok(Lies::InsideEdge {
                r#loop: 0,
                edge: 0,
                ..
            })
        ),
        "{:?}",
        through[0].lies
    );
    let clear = lies(
        &unit_sphere(),
        &[closed(ellipse(Point3::new(0.99, 0.0, 0.0), 0.1), true)],
        true,
    );
    assert!(
        clear.iter().all(|l| l.lies.is_ok()),
        "an ellipse clear of the poles: {clear:?}"
    );
}

/// **The cone's apex cap holds its apex, and the rest of the nappe does
/// not**: the rim at slant `√2` traversed each way, one sense. The chart
/// normal is outward and the left of `+u` is `+v`, so a rim run east on
/// the positive nappe leaves the apex on its right: outside the face it
/// bounds under the chart normal, and inside it run west.
#[test]
fn a_cone_rim_holds_its_apex_on_one_side() {
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: FRAC_PI_4,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let read = |forward| placed(&lies(&cone, &[closed(rim.clone(), forward)], true));
    assert_eq!(read(true), [(Singularity::Apex, Lies::Outside)], "run east");
    assert_eq!(
        read(false),
        [(Singularity::Apex, Lies::Interior)],
        "run west"
    );
    // A generator from the apex: the apex is that loop's vertex.
    let generator = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 1.0),
    };
    let sector = vec![
        LoopEdge::hand_built(generator.clone(), 0.0, 1.0, true, 0, 1),
        LoopEdge::hand_built(rim.clone(), 0.0, PI, true, 1, 2),
        LoopEdge::hand_built(
            Curve3::Line {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(-1.0, 0.0, 1.0),
            },
            0.0,
            1.0,
            false,
            2,
            0,
        ),
    ];
    assert_eq!(
        placed(&lies(&cone, &[sector], true)),
        [(Singularity::Apex, Lies::AtVertex { r#loop: 0, edge: 0 })]
    );
}

/// **The reading is generic over the certifying scalar**: the rim-only
/// cap at `Interval` places the pole as `f64` does.
#[test]
fn the_cap_reads_alike_at_interval() {
    use geom_core::{Interval, Real};
    let i = Interval::from_f64;
    let lift = |p: Point3<f64>| Point3::new(i(p.x), i(p.y), i(p.z));
    let vlift = |v: Vec3<f64>| Vec3::new(i(v.x), i(v.y), i(v.z));
    let sphere = Surface::Sphere {
        center: lift(Point3::new(0.0, 0.0, 0.0)),
        radius: i(1.0),
        axis: vlift(Vec3::new(0.0, 0.0, 1.0)),
        u_ref: vlift(Vec3::new(1.0, 0.0, 0.0)),
    };
    let z = 0.5_f64;
    let rim = Curve3::Circle {
        center: lift(Point3::new(0.0, 0.0, z)),
        axis: vlift(Vec3::new(0.0, 0.0, 1.0)),
        radius: i((1.0 - z * z).sqrt()),
        u_ref: vlift(Vec3::new(1.0, 0.0, 0.0)),
    };
    let edges = [LoopEdge::hand_built(rim, i(0.0), i(TAU), true, 0, 0)];
    let corners = traversal_corners(&edges);
    let located = locate(
        &sphere,
        &[FaceLoop {
            edges: &edges,
            corners: &corners,
        }],
        &[&edges],
        true,
        band(),
    );
    let got: Vec<_> = located
        .iter()
        .map(|l| (l.singularity, l.lies.as_ref().map(|_| ()).is_ok(), &l.lies))
        .map(|(s, ok, lies)| {
            assert!(ok, "{s:?}: {lies:?}");
            (s, matches!(lies, Ok(Lies::Interior)))
        })
        .collect();
    assert_eq!(
        got,
        [(Singularity::North, true), (Singularity::South, false)]
    );
}

/// **A small face reads its side, however coarse the band**: a cap
/// 2 µm across, a quarter turn from both poles, at the band
/// `(1e-6, 1e-5)`. The poles are decided off its boundary, so which side
/// of its loop they lie on is the loop's orientation and no tolerance
/// reads its width.
#[test]
fn a_small_face_far_from_the_poles_reads_them_outside_at_a_coarse_band() {
    let coarse = Band::new(1e-6, 1e-5).unwrap();
    let d = (1.0_f64 - 1e-12).sqrt();
    for forward in [true, false] {
        let cap = closed(section(Vec3::new(1.0, 0.0, 0.0), d), forward);
        let located = lies_at(&unit_sphere(), &[cap], forward, coarse);
        for l in &located {
            assert!(
                matches!(l.lies, Ok(Lies::Outside)),
                "forward {forward}: {:?} {:?}",
                l.singularity,
                l.lies
            );
        }
    }
}

/// The unit cone about `+Z` with its apex at the origin, half-angle π/4.
fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: FRAC_PI_4,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The parallel of the cone at height `z`.
fn parallel(z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: z.abs(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **A sliver beside the apex reads it by the sliver's orientation**:
/// a quarter-turn patch between the parallels at heights `z` and
/// `z(1 + w)` near the apex, closed by two generators, counter-clockwise
/// in the chart. Its walk's steps along a parallel or a generator carry
/// no error, so however thin, the apex reads outside the patch and
/// inside its complement.
#[test]
fn a_sliver_beside_the_apex_reads_it_by_its_orientation() {
    let generator = |u: f64| Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(u.cos(), u.sin(), 1.0),
    };
    let (u0, u1) = (0.2, 0.2 + FRAC_PI_2);
    for z in [1e-3_f64, 1e-6] {
        for w in [1e-3_f64, 1e-6, 1e-9] {
            let top = z * (1.0 + w);
            let patch = vec![
                LoopEdge::hand_built(parallel(z), u0, u1, true, 0, 1),
                LoopEdge::hand_built(generator(u1), z, top, true, 1, 2),
                LoopEdge::hand_built(parallel(top), u0, u1, false, 2, 3),
                LoopEdge::hand_built(generator(u0), z, top, false, 3, 0),
            ];
            for (sense, want) in [(true, Lies::Outside), (false, Lies::Interior)] {
                let got = lies(&cone(), core::slice::from_ref(&patch), sense);
                assert!(
                    matches!(got[0].lies, Ok(l) if l == want),
                    "z {z} w {w} sense {sense}: {:?}",
                    got[0].lies
                );
            }
        }
    }
}

/// **A sliver on sampled carriers beside a pole never reads a wrong
/// side**: two great-circle arcs `w` apart passing `1e-3` from the north
/// pole, stated as equal-semi ellipses, which no closed form reads. The
/// walk reads both poles outside the sliver where its error bound clears
/// the sliver's area (the wider one, which it must decide), and leaves
/// the side undecided where it does not: never inside.
#[test]
fn a_sampled_sliver_beside_a_pole_reads_outside_or_undecided() {
    use geom_brep::chart_singularity::SideUnread;
    let delta = 1e-3_f64;
    let ellipse = |dist: f64| Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(dist.cos(), 0.0, dist.sin()),
        semi_u: 1.0,
        semi_v: 1.0,
        u_ref: Vec3::new(-dist.sin(), 0.0, dist.cos()),
    };
    let half = 0.05_f64;
    for w in [1e-2_f64, 1e-4, 1e-6] {
        let (a, b) = (ellipse(delta), ellipse(delta + w));
        let end = |c: &Curve3<f64>, t: f64| c.eval(t);
        let rung = |p: Point3<f64>, q: Point3<f64>| {
            let (u, v) = (
                p - Point3::new(0.0, 0.0, 0.0),
                q - Point3::new(0.0, 0.0, 0.0),
            );
            let n = u.cross(v);
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: n / n.norm(),
                radius: 1.0,
                u_ref: u,
            }
        };
        let ang = |p: Point3<f64>, q: Point3<f64>| {
            let (u, v) = (
                p - Point3::new(0.0, 0.0, 0.0),
                q - Point3::new(0.0, 0.0, 0.0),
            );
            u.cross(v).norm().atan2(u.dot(v))
        };
        let (a0, a1, b0, b1) = (end(&a, -half), end(&a, half), end(&b, -half), end(&b, half));
        let sliver = vec![
            LoopEdge::hand_built(a.clone(), -half, half, true, 0, 1),
            LoopEdge::hand_built(rung(a1, b1), 0.0, ang(a1, b1), true, 1, 2),
            LoopEdge::hand_built(b.clone(), -half, half, false, 2, 3),
            LoopEdge::hand_built(rung(b0, a0), 0.0, ang(b0, a0), true, 3, 0),
        ];
        let mut sides = Vec::new();
        for sense in [true, false] {
            let got = lies(&unit_sphere(), core::slice::from_ref(&sliver), sense);
            sides.push(got.iter().map(|l| l.lies.clone()).collect::<Vec<_>>());
        }
        // One sense is the sliver, which holds neither pole.
        let sliver_reads = sides.iter().find(|ls| {
            ls.iter().all(|l| {
                matches!(
                    l,
                    Ok(Lies::Outside)
                        | Err(Unread::Side {
                            why: SideUnread::Straddle,
                            ..
                        })
                )
            })
        });
        assert!(sliver_reads.is_some(), "w {w}: {sides:?}");
        // The wider sliver's area clears the walk's bound: decided.
        if w >= 1e-2 {
            assert!(
                sides
                    .iter()
                    .any(|ls| ls.iter().all(|l| matches!(l, Ok(Lies::Outside)))),
                "w {w}: the walk decides the sliver: {sides:?}"
            );
        }
        assert!(
            sides
                .iter()
                .flatten()
                .all(|l| !matches!(l, Ok(Lies::InsideEdge { .. } | Lies::AtVertex { .. }))),
            "w {w}: the poles are off the sliver: {sides:?}"
        );
    }
}

/// **The two readings the arm exempts** (tier 3's not-yet-checked
/// list): a loop that winds twice states no single winding
/// ([`Unread::Loop`]), and a sampled carrier passing the pole in band
/// where its speed is far below its bound, which needs more bisections
/// than the refinement's budget to settle on or off
/// ([`Unread::Carrier`]).
#[test]
fn a_loop_winding_twice_and_an_unsettled_carrier_are_exempt() {
    // A parallel traversed twice.
    let twice = vec![LoopEdge::hand_built(
        section(Vec3::new(0.0, 0.0, 1.0), 0.5),
        0.0,
        2.0 * TAU,
        true,
        0,
        0,
    )];
    let read = lies(&unit_sphere(), &[twice], true);
    assert!(
        read.iter()
            .all(|l| matches!(l.lies, Err(Unread::Loop { r#loop: 0 }))),
        "{read:?}"
    );
    // An ellipse whose slow end, the end of its major axis, passes `3ε`
    // from the north pole: its speed bound is the major semi-axis, 10⁷
    // times its speed there, so clearing the band-wide stretch at the
    // pole outruns the budget.
    let delta = 3e-9;
    let (major, minor) = (1e7, 1.0);
    let grazing = Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 1.0 + delta - major),
        axis: Vec3::new(0.0, 1.0, 0.0),
        semi_u: major,
        semi_v: minor,
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    };
    let edge = vec![LoopEdge::hand_built(grazing, -0.5, 0.5, true, 0, 1)];
    let read = lies(&unit_sphere(), &[edge], true);
    assert!(
        matches!(read[0].lies, Err(Unread::Carrier { r#loop: 0, edge: 0 })),
        "{:?}",
        read[0].lies
    );
}

/// **Spurs through both poles leave their sides to the rest of the
/// loop**, as import's pole form leaves a face that held both: a cap
/// about `+x` (the rim run forward, so the face is the cap), with a seam
/// up a meridian from the rim to the north pole, on round the whole
/// meridian to the south pole, and back the same way. Both poles are
/// vertices, and the side of each (what import's strut tip asks,
/// [`holds`](geom_brep::chart_singularity::holds)) is the rim's: outside
/// the cap. Each spur's tip turns by half a turn either way, which no
/// turning reads, and the walk cannot certify a loop through both poles:
/// only the chart image, the spurs taken out, reads it.
#[test]
fn spurs_through_both_poles_leave_their_sides_to_the_rim() {
    use geom_brep::chart_singularity::holds;
    let rim = section(Vec3::new(1.0, 0.0, 0.0), 0.6);
    let Curve3::Circle { center, u_ref, .. } = rim.clone() else {
        unreachable!()
    };
    let p0 = center + u_ref * 0.8;
    let w = p0 - Point3::new(0.0, 0.0, 0.0);
    let axis = w.cross(Vec3::new(0.0, 0.0, 1.0));
    let meridian = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: axis / axis.norm(),
        radius: 1.0,
        u_ref: w / w.norm(),
    };
    let face = vec![
        LoopEdge::hand_built(rim, 0.0, TAU, true, 0, 0),
        LoopEdge::hand_built(meridian.clone(), 0.0, FRAC_PI_2, true, 0, 1),
        LoopEdge::hand_built(meridian.clone(), FRAC_PI_2, 1.5 * PI, true, 1, 2),
        LoopEdge::hand_built(meridian.clone(), FRAC_PI_2, 1.5 * PI, false, 2, 1),
        LoopEdge::hand_built(meridian, 0.0, FRAC_PI_2, false, 1, 0),
    ];
    for pole in [Singularity::North, Singularity::South] {
        let image = hand_image(&face);
        assert_eq!(image.len(), 1, "both spurs out, the rim left");
        let got = holds(&unit_sphere(), &[image.as_slice()], true, pole, band());
        assert!(matches!(got, Ok(false)), "{pole:?}: {got:?}");
    }
    assert_eq!(
        placed(&lies(&unit_sphere(), &[face], true)),
        [
            (Singularity::North, Lies::AtVertex { r#loop: 0, edge: 2 }),
            (Singularity::South, Lies::AtVertex { r#loop: 0, edge: 3 })
        ]
    );
}

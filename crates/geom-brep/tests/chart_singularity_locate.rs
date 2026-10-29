//! **Where a chart singularity lies against a face**
//! (`geom_brep::chart_singularity::locate`), on loops stated by hand: a
//! pole at a vertex, inside an edge, inside the face or outside it,
//! whatever the loop's carriers, and the readings that cannot place it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

use geom::{Curve3, Surface};
use geom_brep::chart_singularity::{Lies, Located, Singularity, Unread, locate};
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
    let views: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    locate(surface, &views, sense, band())
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

/// **A carrier read by samples places a point off it, never on it**: an
/// ellipse through the north pole cannot place it, and a small one far
/// from both poles reads them off it and goes on to the side.
#[test]
fn a_sampled_carrier_through_the_pole_is_unread() {
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
        matches!(through[0].lies, Err(Unread::Carrier { r#loop: 0, edge: 0 })),
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
/// not**: the rim at slant `√2` traversed each way, one sense.
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
    let (east, west) = (read(true), read(false));
    assert_eq!(east.len(), 1, "one apex");
    let mut both = [east[0].1, west[0].1];
    both.sort_by_key(|l| matches!(l, Lies::Outside));
    assert_eq!(
        both,
        [Lies::Interior, Lies::Outside],
        "the two traversals of one rim"
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
    let located = locate(&sphere, &[&edges[..]], true, band());
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

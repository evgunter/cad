//! **The ball-side anti-vacuity row for the sheet reduction.** The
//! `side` bit a support trace carries selects the `R ∓ r` fold, and
//! this row pins, bit for bit, the centre and setbacks the coaxial and
//! the ruled arms answer at every ball-side pair — so the two sides
//! give DIFFERENT answers wherever the geometry distinguishes them,
//! and a fold that dropped the bit, or read it backwards, reds here
//! rather than in a downstream mesh.
//!
//! `side: true` is what `Convexity::ball_side` answers for a convex
//! chain over a `sense: true` support (the ball rests behind the chart
//! normal), `false` for a concave one. The pinned values are the
//! reduction's own answers at f64, printed with the shortest
//! round-trip representation so each literal parses back to the exact
//! bit pattern; they were taken at the commit before the bit replaced
//! a `±1` factor on the radius, so the row is also that fold's
//! bit-identity receipt.
//!
//! One ruled pair is left out: the plane–cylinder pair at
//! `(false, true)` has no crossing — the two offsets miss — and
//! answers poison, which pins nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// The pinned literals are the reduction's own bits; the ruled plane's
// `u_ref` lands on `1/√2` to the ulp, and it is pinned as the bits the
// arm answered, not as the constant.
#![allow(clippy::approx_constant)]

use geom::Surface;
use geom_core::{Point3, Vec3};
use sweep::blend::arms::{Meridian, Ruling};

const RADIUS: f64 = 0.25;

fn assert_bits(actual: f64, expected: f64, what: &str) {
    assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "{what}: {actual:?} vs pinned {expected:?}"
    );
}

/// The coaxial arm: a plane ⊥ the axis, a coaxial cylinder and two
/// spheres centred on the axis, all through the rim `(2, 0, 1)`, paired
/// line × line, line × circle and circle × circle.
#[test]
fn the_coaxial_arm_answers_each_ball_side_pair_as_pinned() {
    let m = Meridian {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        rim: Point3::new(2.0, 0.0, 1.0),
    };
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 2.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sph1 = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 5.0f64.sqrt(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sph2 = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 3.0),
        radius: 8.0f64.sqrt(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // (pairing, side_a, side_b, torus centre, major radius, setbacks)
    #[allow(clippy::type_complexity)]
    let pinned: [(&str, bool, bool, [f64; 3], f64, [f64; 2]); 12] = [
        (
            "plane_cyl",
            true,
            true,
            [0.0, 0.0, 0.75],
            1.75,
            [0.25, 0.25],
        ),
        (
            "plane_cyl",
            true,
            false,
            [0.0, 0.0, 0.75],
            2.25,
            [0.25, 0.25],
        ),
        (
            "plane_cyl",
            false,
            true,
            [0.0, 0.0, 1.25],
            1.75,
            [0.25, 0.25],
        ),
        (
            "plane_cyl",
            false,
            false,
            [0.0, 0.0, 1.25],
            2.25,
            [0.25, 0.25],
        ),
        (
            "plane_sph",
            true,
            true,
            [0.0, 0.0, 0.75],
            1.8390122379283138,
            [0.1609877620716862, 0.17081983960283814],
        ),
        (
            "plane_sph",
            true,
            false,
            [0.0, 0.0, 0.75],
            2.3702392260592378,
            [0.37023922605923776, 0.3511303996033269],
        ),
        (
            "plane_sph",
            false,
            true,
            [0.0, 0.0, 1.25],
            1.543361918426817,
            [0.456638081573183, 0.48452654318000254],
        ),
        (
            "plane_sph",
            false,
            false,
            [0.0, 0.0, 1.25],
            2.148961141749635,
            [0.14896114174963504, 0.14127294340105512],
        ),
        (
            "sph_sph",
            true,
            true,
            [0.0, 0.0, 1.0493632622705331],
            1.6862095821833802,
            [0.20793021701344272, 0.20524246709924088],
        ),
        (
            "sph_sph",
            true,
            false,
            [0.0, 0.0, 0.5779587414795015],
            1.900113076739786,
            [0.37603973095059245, 0.33970062426222997],
        ),
        (
            "sph_sph",
            false,
            true,
            [0.0, 0.0, 1.4220412585204985],
            2.039199021139264,
            [0.32461391544106055, 0.35848930023630765],
        ),
        (
            "sph_sph",
            false,
            false,
            [0.0, 0.0, 0.9506367377294668],
            2.2971338188335855,
            [0.1593326602641457, 0.16103744231053305],
        ),
    ];
    let supports = |name: &str| match name {
        "plane_cyl" => (&plane, &cyl),
        "plane_sph" => (&plane, &sph1),
        "sph_sph" => (&sph1, &sph2),
        other => panic!("unknown pairing {other}"),
    };
    for (name, side_a, side_b, center, major, setbacks) in pinned {
        let (a, b) = supports(name);
        let (ta, _) = m.trace(a, side_a).expect("a coaxial support traces");
        let (tb, _) = m.trace(b, side_b).expect("a coaxial support traces");
        let blend = m.blend(ta, tb, RADIUS);
        let Surface::Torus {
            center: c,
            major_radius,
            ..
        } = blend.surface
        else {
            panic!(
                "{name}: the coaxial arm mints a torus, got {:?}",
                blend.surface
            );
        };
        let what = format!("{name} sides ({side_a}, {side_b})");
        assert_bits(c.x, center[0], &format!("{what} centre.x"));
        assert_bits(c.y, center[1], &format!("{what} centre.y"));
        assert_bits(c.z, center[2], &format!("{what} centre.z"));
        assert_bits(major_radius, major, &format!("{what} major radius"));
        assert_bits(blend.trim_a.1, setbacks[0], &format!("{what} setback a"));
        assert_bits(blend.trim_b.1, setbacks[1], &format!("{what} setback b"));
    }
    // The bit is read: the all-true and all-false pairs answer
    // differently on every pairing.
    for name in ["plane_cyl", "plane_sph", "sph_sph"] {
        let (a, b) = supports(name);
        let both = |side: bool| {
            let (ta, _) = m.trace(a, side).expect("traces");
            let (tb, _) = m.trace(b, side).expect("traces");
            m.blend(ta, tb, RADIUS)
        };
        let (convex, concave) = (both(true), both(false));
        assert!(
            convex.spine_curvature.to_bits() != concave.spine_curvature.to_bits()
                || convex.trim_a.1.to_bits() != concave.trim_a.1.to_bits(),
            "{name}: the two ball sides must not reduce to the same blend"
        );
    }
}

/// The ruled arm: two planes containing the ruling, and a plane with
/// the cylinder about the ruling, all through the rim `(1, 0, 0)`.
#[test]
fn the_ruled_arm_answers_each_ball_side_pair_as_pinned() {
    let r = Ruling {
        tau: Vec3::new(0.0, 0.0, 1.0),
        rim: Point3::new(1.0, 0.0, 0.0),
        lever: 1.0,
    };
    let px = Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let py = Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    };
    let pd = Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 1.0, 0.0).normalize(),
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    };
    let cz = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // (pairing, side_a, side_b, cylinder origin, u_ref, setbacks)
    #[allow(clippy::type_complexity)]
    let pinned: [(&str, bool, bool, [f64; 3], [f64; 3], [f64; 2]); 7] = [
        (
            "px_py",
            true,
            true,
            [0.75, -0.25, 0.0],
            [1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "px_py",
            true,
            false,
            [0.75, 0.25, 0.0],
            [1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "px_py",
            false,
            true,
            [1.25, -0.25, 0.0],
            [-1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "px_py",
            false,
            false,
            [1.25, 0.25, 0.0],
            [-1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "pd_cz",
            true,
            true,
            [0.7436715123302204, -0.09722490292349413, 0.0],
            [0.7071067811865476, 0.7071067811865476, 0.0],
            [0.11250322368518698, 0.12990753295868673],
        ),
        (
            "pd_cz",
            true,
            false,
            [1.1458876927113995, -0.49944108330467313, 0.0],
            [0.7071067811865478, 0.7071067811865474, 0.0],
            [0.45631635361577966, 0.408141754371886],
        ),
        (
            "pd_cz",
            false,
            false,
            [1.2453040074843305, 0.1082493831089434, 0.0],
            [-0.7071067811865478, -0.7071067811865474, 0.0],
            [0.0969122542888112, 0.08668095537701102],
        ),
    ];
    for (name, side_a, side_b, origin, u_ref, setbacks) in pinned {
        let (a, b) = match name {
            "px_py" => (&px, &py),
            "pd_cz" => (&pd, &cz),
            other => panic!("unknown pairing {other}"),
        };
        let (ta, _) = r.trace(a, side_a).expect("a ruled support traces");
        let (tb, _) = r.trace(b, side_b).expect("a ruled support traces");
        let blend = r.blend(ta, tb, RADIUS);
        let Surface::Cylinder {
            origin: o,
            u_ref: u,
            ..
        } = blend.surface
        else {
            panic!(
                "{name}: the ruled arm mints a cylinder, got {:?}",
                blend.surface
            );
        };
        let what = format!("{name} sides ({side_a}, {side_b})");
        assert_bits(o.x, origin[0], &format!("{what} origin.x"));
        assert_bits(o.y, origin[1], &format!("{what} origin.y"));
        assert_bits(o.z, origin[2], &format!("{what} origin.z"));
        assert_bits(u.x, u_ref[0], &format!("{what} u_ref.x"));
        assert_bits(u.y, u_ref[1], &format!("{what} u_ref.y"));
        assert_bits(u.z, u_ref[2], &format!("{what} u_ref.z"));
        assert_bits(blend.trim_a.1, setbacks[0], &format!("{what} setback a"));
        assert_bits(blend.trim_b.1, setbacks[1], &format!("{what} setback b"));
    }
}

//! **The ball-side anti-vacuity row for the sheet reduction.** The
//! `side` a support trace carries selects the `R ∓ r` fold, and
//! this row pins, bit for bit, the centre and setbacks the coaxial and
//! the ruled arms answer at every ball-side pair — so the two sides
//! give DIFFERENT answers wherever the geometry distinguishes them,
//! and a fold that dropped the side, or read it backwards, reds here
//! rather than in a downstream mesh.
//!
//! `Inner` is what `battery::ball_side` answers for a convex chain
//! over a `sense: true` support (the ball rests behind the chart
//! normal), `Outer` for a concave one. The pinned values are the
//! reduction's own answers at f64, printed with the shortest
//! round-trip representation so each literal parses back to the exact
//! bit pattern.
//!
//! One ruled pair is left out: the plane–cylinder pair at
//! `(Outer, Inner)` has no crossing — the two offsets miss — and
//! answers poison, which pins nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// The pinned literals are the reduction's own bits. The ruled plane's
// normal is `(1, 1, 0).normalize()`, one ulp below `1/√2`; the band's
// `u_ref` is that normal squared into the band's frame (`geom::Frame::new`
// over the ruling and the foot direction), which renormalizes it to the
// `f64` nearest `1/√2` and writes its zeros `+0`. It is pinned as the bits
// the arm answered, not as the constant. The fixtures' own frames are
// admitted as stated (`geom::Frame::admitted`), so only the arm's frame
// moves a bit.
#![allow(clippy::approx_constant)]

use crate::common::interval::iv;
use geom::Surface;
use geom_brep::SurfaceSide::{self, Inner, Outer};
use geom_core::Bounds;
use geom_core::{Point3, Vec3};
use sweep::blend::Convexity;
use sweep::blend::arms::{Branch, Meridian, Ruling, plane_plane_blend};

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
    let m = Meridian::new(
        geom::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: geom_core::UnitVec3::exact_z(),
        },
        Point3::new(2.0, 0.0, 1.0),
        geom_core::Band::linear(geom_core::Tol::witness()).expect("the band"),
    )
    .expect("the sheet's frame mints");
    let plane = Surface::Plane {
        frame: geom::Frame::admitted(
            Point3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
        .expect("a rigid fixture frame"),
    };
    let cyl = Surface::Cylinder {
        frame: geom::Frame::admitted(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
        .expect("a rigid fixture frame"),
        radius: 2.0,
    };
    let sph1 = Surface::Sphere {
        frame: geom::Frame::admitted(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
        .expect("a rigid fixture frame"),
        radius: 5.0f64.sqrt(),
    };
    let sph2 = Surface::Sphere {
        frame: geom::Frame::admitted(
            Point3::new(0.0, 0.0, 3.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
        .expect("a rigid fixture frame"),
        radius: 8.0f64.sqrt(),
    };
    // (pairing, side_a, side_b, torus centre, major radius, setbacks)
    #[allow(clippy::type_complexity)]
    let pinned: [(&str, SurfaceSide, SurfaceSide, [f64; 3], f64, [f64; 2]); 12] = [
        (
            "plane_cyl",
            Inner,
            Inner,
            [0.0, 0.0, 0.75],
            1.75,
            [0.25, 0.25],
        ),
        (
            "plane_cyl",
            Inner,
            Outer,
            [0.0, 0.0, 0.75],
            2.25,
            [0.25, 0.25],
        ),
        (
            "plane_cyl",
            Outer,
            Inner,
            [0.0, 0.0, 1.25],
            1.75,
            [0.25, 0.25],
        ),
        (
            "plane_cyl",
            Outer,
            Outer,
            [0.0, 0.0, 1.25],
            2.25,
            [0.25, 0.25],
        ),
        (
            "plane_sph",
            Inner,
            Inner,
            [0.0, 0.0, 0.75],
            1.8390122379283138,
            [0.1609877620716862, 0.17081983960283814],
        ),
        (
            "plane_sph",
            Inner,
            Outer,
            [0.0, 0.0, 0.75],
            2.3702392260592378,
            [0.37023922605923776, 0.3511303996033269],
        ),
        (
            "plane_sph",
            Outer,
            Inner,
            [0.0, 0.0, 1.25],
            1.543361918426817,
            [0.456638081573183, 0.48452654318000254],
        ),
        (
            "plane_sph",
            Outer,
            Outer,
            [0.0, 0.0, 1.25],
            2.148961141749635,
            [0.14896114174963504, 0.14127294340105512],
        ),
        (
            "sph_sph",
            Inner,
            Inner,
            [0.0, 0.0, 1.0493632622705333],
            1.68620958218338,
            [0.20793021701344302, 0.2052424670992407],
        ),
        (
            "sph_sph",
            Inner,
            Outer,
            [0.0, 0.0, 0.5779587414795022],
            1.9001130767397856,
            [0.3760397309505914, 0.3397006242622297],
        ),
        (
            "sph_sph",
            Outer,
            Inner,
            [0.0, 0.0, 1.4220412585204985],
            2.0391990211392637,
            [0.3246139154410608, 0.3584893002363074],
        ),
        (
            "sph_sph",
            Outer,
            Outer,
            [0.0, 0.0, 0.9506367377294669],
            2.2971338188335855,
            [0.15933266026414558, 0.16103744231053338],
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
        let blend = m.blend(ta, tb, RADIUS, Branch::Rim);
        let Surface::Torus {
            frame,
            major_radius,
            ..
        } = blend.surface
        else {
            panic!(
                "{name}: the coaxial arm mints a torus, got {:?}",
                blend.surface
            );
        };
        let c = frame.origin();
        let what = format!("{name} sides ({side_a:?}, {side_b:?})");
        assert_bits(c.x, center[0], &format!("{what} centre.x"));
        assert_bits(c.y, center[1], &format!("{what} centre.y"));
        assert_bits(c.z, center[2], &format!("{what} centre.z"));
        assert_bits(major_radius, major, &format!("{what} major radius"));
        assert_bits(blend.trim_a.1, setbacks[0], &format!("{what} setback a"));
        assert_bits(blend.trim_b.1, setbacks[1], &format!("{what} setback b"));
    }
    // The side is read: the all-inner and all-outer pairs answer
    // differently on every pairing.
    for name in ["plane_cyl", "plane_sph", "sph_sph"] {
        let (a, b) = supports(name);
        let both = |side: SurfaceSide| {
            let (ta, _) = m.trace(a, side).expect("traces");
            let (tb, _) = m.trace(b, side).expect("traces");
            m.blend(ta, tb, RADIUS, Branch::Rim)
        };
        let (convex, concave) = (both(Inner), both(Outer));
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
        tau: geom_core::UnitVec3::exact_z(),
        rim: Point3::new(1.0, 0.0, 0.0),
        lever: 1.0,
    };
    let px = Surface::Plane {
        frame: geom::Frame::admitted(
            Point3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
        .expect("a rigid fixture frame"),
    };
    let py = Surface::Plane {
        frame: geom::Frame::admitted(
            Point3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
        .expect("a rigid fixture frame"),
    };
    let pd = Surface::Plane {
        frame: geom::Frame::admitted(
            Point3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0).normalize(),
            Vec3::new(0.0, 0.0, 1.0),
        )
        .expect("a rigid fixture frame"),
    };
    let cz = Surface::Cylinder {
        frame: geom::Frame::admitted(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
        .expect("a rigid fixture frame"),
        radius: 1.0,
    };
    // (pairing, side_a, side_b, cylinder origin, u_ref, setbacks)
    #[allow(clippy::type_complexity)]
    let pinned: [(&str, SurfaceSide, SurfaceSide, [f64; 3], [f64; 3], [f64; 2]); 7] = [
        (
            "px_py",
            Inner,
            Inner,
            [0.75, -0.25, 0.0],
            [1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "px_py",
            Inner,
            Outer,
            [0.75, 0.25, 0.0],
            [1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "px_py",
            Outer,
            Inner,
            [1.25, -0.25, 0.0],
            [-1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "px_py",
            Outer,
            Outer,
            [1.25, 0.25, 0.0],
            [-1.0, 0.0, 0.0],
            [0.25, 0.25],
        ),
        (
            "pd_cz",
            Inner,
            Inner,
            [0.7436715123302204, -0.09722490292349413, 0.0],
            [0.7071067811865476, 0.7071067811865476, 0.0],
            [0.11250322368518698, 0.12990753295868673],
        ),
        (
            "pd_cz",
            Inner,
            Outer,
            [1.1458876927113995, -0.49944108330467313, 0.0],
            [0.7071067811865476, 0.7071067811865476, 0.0],
            [0.45631635361577966, 0.408141754371886],
        ),
        (
            "pd_cz",
            Outer,
            Outer,
            [1.2453040074843305, 0.1082493831089434, 0.0],
            [-0.7071067811865476, -0.7071067811865476, 0.0],
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
        let blend = r
            .blend(
                ta,
                tb,
                RADIUS,
                Branch::Rim,
                geom_core::Band::linear(geom_core::Tol::witness()).expect("the band"),
            )
            .expect("the cylinder's frame mints");
        let Surface::Cylinder {
            frame: carrier_frame,
            ..
        } = blend.surface
        else {
            panic!(
                "{name}: the ruled arm mints a cylinder, got {:?}",
                blend.surface
            );
        };
        let (o, u) = (carrier_frame.origin(), carrier_frame.u_ref());
        let what = format!("{name} sides ({side_a:?}, {side_b:?})");
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

/// **The plane–plane band's seam reference is its first support's foot
/// direction, `±n_a`, to the bit of its own renormalization**: `n_a` on a
/// convex chain, its exact negation on a concave one, squared into the
/// band's frame about the edge (`geom::Frame::new`, whose residual here
/// is the foot direction itself, the edge being square to it), never the
/// foot point less the centre (a radius-scale offset read off body-scale
/// points). At `Interval` it is no wider than that renormalization of
/// `n_a` itself.
#[test]
fn the_plane_plane_bands_u_ref_is_its_first_supports_foot_direction() {
    let n = Vec3::new(0.0, 1.0, -0.6).normalize();
    let up = Vec3::new(0.0, 0.0, 1.0);
    let p = Point3::new(1.3, 0.7, 1.0);
    let tau = Vec3::new(1.0, 0.0, 0.0);
    for (convexity, sign) in [(Convexity::Convex, 1.0), (Convexity::Concave, -1.0)] {
        let blend = plane_plane_blend(
            p,
            geom::stated_direction(tau),
            n,
            up,
            0.1,
            convexity,
            geom_core::Band::linear(geom_core::Tol::witness()).expect("the band"),
        )
        .expect("the cylinder's frame mints");
        let Surface::Cylinder {
            frame: carrier_frame,
            ..
        } = blend.surface
        else {
            panic!("{convexity}: the plane–plane arm mints a cylinder");
        };
        let u_ref = carrier_frame.u_ref();
        let want = (n * sign).normalize();
        for (got, want, c) in [
            (u_ref.x, want.x, "x"),
            (u_ref.y, want.y, "y"),
            (u_ref.z, want.z, "z"),
        ] {
            assert_bits(got, want, &format!("{convexity} u_ref.{c}"));
        }
        let ivv = |v: Vec3<f64>| v.map(iv);
        let n_i = ivv(n);
        let unit_i = (n_i * iv(sign)).normalize();
        let blend = plane_plane_blend(
            p.map(iv),
            geom_core::UnitVec3::checked(tau).map_scalar(iv),
            n_i,
            ivv(up),
            iv(0.1),
            convexity,
            geom_core::Band::linear(geom_core::Tol::witness()).expect("the band"),
        )
        .expect("the cylinder's frame mints");
        let Surface::Cylinder { frame, .. } = blend.surface else {
            panic!("{convexity}: the plane–plane arm mints a cylinder at Interval");
        };
        let u_ref = frame.u_ref();
        for (got, of, c) in [
            (u_ref.x, unit_i.x, "x"),
            (u_ref.y, unit_i.y, "y"),
            (u_ref.z, unit_i.z, "z"),
        ] {
            assert!(
                got.hi() - got.lo() <= of.hi() - of.lo(),
                "{convexity} u_ref.{c} at Interval: {got:?} is wider than n_a's renormalization {of:?}"
            );
        }
    }
}

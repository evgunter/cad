use super::shift_v_branch;
use geom_brep::{Pcurve, SpiricImage, WallTorus};
use geom_core::{Point2, Vec2};

/// **The second-channel branch shift, at its own door.** A body cannot
/// exercise the wall arm: a spiric rim's parameter span is the
/// revolved PROFILE arc's, so the one spiric-bearing body's rims span
/// 1.78 rad and no joint decides a second-channel period on them.
/// So the map's value is pinned HERE, per variant: the image is
/// translated along the second channel and nothing else moves — a
/// wall's `v0`, a cap's and a harmonic image's `p0.y` — so a planted
/// `k·period → 0` is red.
#[test]
fn the_v_shift_translates_every_image_along_v_alone() {
    let period = core::f64::consts::TAU;
    let wall = Pcurve::Spiric {
        major: 0.09375,
        minor: 0.0703125,
        offset: 0.0078125,
        image: SpiricImage::Wall {
            u0: 0.25,
            v0: 0.5,
            sense: -1.0,
            torus: WallTorus::Mirror,
        },
    };
    let Pcurve::Spiric {
        major,
        minor,
        offset,
        image:
            SpiricImage::Wall {
                u0,
                v0,
                sense,
                torus,
            },
    } = shift_v_branch(&wall, 3, geom_brep::Period::new(period))
    else {
        panic!("the wall arm keeps its variant and its image kind");
    };
    assert_eq!(v0, 0.5 + 3.0 * period, "v0 takes the whole-period shift");
    assert_eq!(u0, 0.25, "the azimuth constant is the other door's");
    assert_eq!(sense, -1.0, "the sign is not a branch");
    assert_eq!(torus, WallTorus::Mirror, "nor is the torus");
    assert_eq!((major, minor, offset), (0.09375, 0.0703125, 0.0078125));

    // A cap lives on a plane chart, where no joint decides a
    // second-channel period; the map is the same translation there.
    let cap = Pcurve::Spiric {
        major: 0.09375,
        minor: 0.0703125,
        offset: 0.0078125,
        image: SpiricImage::Cap {
            p0: Point2::new(0.1, 0.2),
            pm: Vec2::new(1.0, 0.0),
            pa: Vec2::new(0.0, 0.0703125),
        },
    };
    let Pcurve::Spiric {
        image: SpiricImage::Cap { p0, pm, pa },
        ..
    } = shift_v_branch(&cap, 3, geom_brep::Period::new(period))
    else {
        panic!("the cap arm keeps its variant and its image kind");
    };
    assert_eq!(
        (p0.x, p0.y),
        (0.1, 0.2 + 3.0 * period),
        "p0.y takes the shift"
    );
    assert_eq!([pm.x, pm.y, pa.x, pa.y], [1.0, 0.0, 0.0, 0.0703125]);

    // Every coefficient live, on no chart: the row is of the map,
    // which takes a free channel's four coefficients alike on any
    // axis, so the shape is the point and no chart certifies it.
    let harmonic = Pcurve::free_harmonic(
        Point2::new(0.3, 0.4),
        Vec2::new(1.0, 0.25),
        Vec2::new(-0.5, 1.0),
        Vec2::new(0.125, -0.375),
    );
    let Some((p0, pa, pb, pl)) =
        shift_v_branch(&harmonic, 3, geom_brep::Period::new(period)).harmonic_form()
    else {
        panic!("the harmonic arm keeps its variant");
    };
    assert_eq!(p0.y, 0.4 + 3.0 * period);
    assert_eq!(p0.x, 0.3);
    // `Vec2` is deliberately not `PartialEq` (a geometric vector
    // is not a thing this kernel compares with `==`), so the three
    // untouched coefficients are read componentwise.
    assert_eq!(
        [pa.x, pa.y, pb.x, pb.y, pl.x, pl.y],
        [1.0, 0.25, -0.5, 1.0, 0.125, -0.375],
        "the trigonometric and linear coefficients are not a branch"
    );
}

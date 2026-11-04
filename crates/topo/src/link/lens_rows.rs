//! **Two sheets on one wedge, read one order down**
//! ([`second_order::lenses`], [`rest::one_material`]): a rod and a bore
//! of one radius whose axes cross, tangent at `p` along `x = 1` (about
//! `p`), each face's corner there bounded by the two curves the
//! carriers meet in. Their sheets part between those curves: a void
//! where each stands on the other's outward side, a sliver of material
//! where each stands behind the other. Each row near the origin and a
//! kilometre off it, in `f64` and in `Interval`.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Interval, Tol};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

fn v<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

/// Where the rows stand: the origin, and a kilometre off it.
fn apexes<T: Real>() -> [Point3<T>; 2] {
    [
        Point3::origin(),
        Point3::new(
            T::from_f64(1000.0),
            T::from_f64(-2000.0),
            T::from_f64(500.0),
        ),
    ]
}

/// `n` distinct edge keys.
fn edges(n: usize) -> Vec<EdgeKey> {
    let mut keys = slotmap::SlotMap::<EdgeKey, ()>::with_key();
    (0..n).map(|_| keys.insert(())).collect()
}

/// The cylinder of radius `r` and unit axis `axis` through `p`, its axis
/// `r` behind `p` along `x`.
fn cylinder<T: Real>(p: Point3<T>, axis: Vec3<T>, r: f64) -> geom::Surface<T> {
    geom::Surface::Cylinder {
        origin: p - v::<T>(r, 0.0, 0.0),
        axis,
        radius: T::from_f64(r),
        u_ref: v(1.0, 0.0, 0.0),
    }
}

/// A face on `surface` of `Face::sense` `sense` cornered at `p`, of
/// outward unit `m`, from `d0` round to `d1` along the edges `ends`, each
/// bending as a curve on both carriers does.
fn corner<T: Decide>(
    p: Point3<T>,
    (surface, sense): (geom::Surface<T>, bool),
    m: Vec3<T>,
    (d0, d1): (Vec3<T>, Vec3<T>),
    ends: [EdgeKey; 2],
) -> TangentPiece<T> {
    let bend = |edge| Bound {
        edge,
        line: false,
        bend: v(-0.5, 0.0, 0.0),
    };
    let fan = TangentFan::Corner {
        d0,
        d1,
        bounds: ends.map(bend),
        spur: false,
    };
    FaceSector::tangent(surface, sense, m, p, fan, band()).expect("a corner's piece")
}

/// The two faces of one lens about `p`, on the wedge from `d0` round
/// `+x` to `d1`: the first on `first`, outward `+x`, the second on
/// `second`, outward `−x` leaning `lean` (a sine) toward `−z`, the two
/// bounded by the same two edges `k`.
fn leaning<T: Decide>(
    p: Point3<T>,
    (first, second): (geom::Surface<T>, geom::Surface<T>),
    (d0, d1): (Vec3<T>, Vec3<T>),
    k: [EdgeKey; 2],
    lean: f64,
) -> [TangentPiece<T>; 2] {
    let back = v::<T>(-(1.0 - lean * lean).sqrt(), 0.0, -lean);
    [
        corner(p, (first, true), v(1.0, 0.0, 0.0), (d0, d1), [k[0], k[1]]),
        corner(p, (second, false), back, (d1, d0), [k[1], k[0]]),
    ]
}

/// [`leaning`] with no lean.
fn lens<T: Decide>(
    p: Point3<T>,
    sheets: (geom::Surface<T>, geom::Surface<T>),
    wedge: (Vec3<T>, Vec3<T>),
    k: [EdgeKey; 2],
) -> [TangentPiece<T>; 2] {
    leaning(p, sheets, wedge, k, 0.0)
}

/// The rod along `y` of radius 1 and the bore along `z` of radius
/// `wide`, through `p`.
fn rod_and_bore<T: Real>(p: Point3<T>, wide: f64) -> (geom::Surface<T>, geom::Surface<T>) {
    rod_and_bore_of(p, 1.0, wide)
}

/// The rod along `y` of radius `rod` and the bore along `z` of radius
/// `bore`, through `p`.
fn rod_and_bore_of<T: Real>(
    p: Point3<T>,
    rod: f64,
    bore: f64,
) -> (geom::Surface<T>, geom::Surface<T>) {
    (
        cylinder(p, v(0.0, 1.0, 0.0), rod),
        cylinder(p, v(0.0, 0.0, 1.0), bore),
    )
}

/// The quarter about `±z` (`up`) or `±y` of the plane `x = 0` about `p`,
/// its two bounds the diagonals.
fn quarter<T: Real>(about: [f64; 3]) -> (Vec3<T>, Vec3<T>) {
    let [x, y, z] = about;
    // Round `+x` from `about`'s clockwise diagonal to its counterclockwise
    // one: `x̂ × about` is a quarter turn on.
    let side = [0.0, -z, y];
    (
        v::<T>(x + -side[0], y - side[1], z - side[2]),
        v::<T>(x + side[0], y + side[1], z + side[2]),
    )
}

/// A whole plane through `p` of outward unit `m`.
fn whole<T: Decide>(p: Point3<T>, m: Vec3<T>) -> TangentPiece<T> {
    let surface = geom::Surface::Plane {
        origin: p,
        normal: m,
        u_ref: m.cross(v::<T>(1.0, 0.3, 0.7)).normalize(),
    };
    let fan = TangentFan::Inside {
        reach: T::from_f64(2.0),
        frame: [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)],
    };
    FaceSector::tangent(surface, true, m, p, fan, band()).expect("a whole plane")
}

fn read<T: Decide>(p: Point3<T>, pieces: Vec<TangentPiece<T>>) -> TouchVerdict {
    rest::one_material(&Link::of_tangent(p, pieces).expect("a link"), band())
}

/// **A lens is the void or the sliver its sheets bend to.** Where the
/// rod is outward `+x` and the bore `−x` (the rod and the bore's
/// complement joined, `rod ∪ bore`), the two sheets on the quarter
/// about `±z` part with each on the other's outward side: two voids in
/// the material. On the quarters about `±y` (`rod ∩ bore`) each stands
/// behind the other: two slivers of material, as on the quarter about
/// `z` with the bore outward `+x` and the rod `−x`. Either alone
/// at the point is a rest. Read against a whole plane through `p`
/// square to the lens's middle, the lens must be the region's other
/// kind: a void inside the plane's material and a sliver outside it rest,
/// the other two are refused. One tangent plane cut in two along the
/// same two edges, both faces the same way out, is no lens. A
/// plane through the lens's middle crosses it. Red where a lens's kind
/// is read the other way round, the region round it is not read, a pair
/// facing one way is taken for a lens, or a sheet crossing it is passed.
fn a_lens_is_the_void_or_the_sliver_its_sheets_bend_to<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    for p in apexes::<T>() {
        let k = edges(4);
        let pair = |i: usize| [k[i], k[i + 1]];
        let mut joined = Vec::new();
        for (up, i) in [(1.0, 0), (-1.0, 2)] {
            joined.extend(lens(
                p,
                rod_and_bore(p, 1.0),
                quarter([0.0, 0.0, up]),
                pair(i),
            ));
        }
        let got = read(p, joined);
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "rod ∪ bore: two voids at {p:?}: {got:?}"
        );
        let mut met = Vec::new();
        for (up, i) in [(1.0, 0), (-1.0, 2)] {
            met.extend(lens(
                p,
                rod_and_bore(p, 1.0),
                quarter([0.0, up, 0.0]),
                pair(i),
            ));
        }
        let got = read(p, met);
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "rod ∩ bore: two slivers at {p:?}: {got:?}"
        );
        // A void about +z and a sliver about −z: no one material outside
        // them.
        let mut mixed = Vec::new();
        mixed.extend(lens(
            p,
            rod_and_bore(p, 1.0),
            quarter([0.0, 0.0, 1.0]),
            pair(0),
        ));
        let (rod, bore) = rod_and_bore(p, 1.0);
        mixed.extend(lens(p, (bore, rod), quarter([0.0, 0.0, -1.0]), pair(2)));
        assert_eq!(read(p, mixed), unread, "a void and a sliver at {p:?}");
        // The rod's tangent plane cut in two along the lens's edges, the
        // two faces the same way out: one sheet, no lens, read as one flat
        // link, which ties no winding (in band).
        let (d0, d1) = quarter::<T>([0.0, 0.0, 1.0]);
        let rod = || rod_and_bore(p, 1.0).0;
        let halves = vec![
            corner(p, (rod(), true), v(1.0, 0.0, 0.0), (d0, d1), pair(0)),
            corner(p, (rod(), true), v(1.0, 0.0, 0.0), (d1, d0), [k[1], k[0]]),
        ];
        assert_eq!(
            read(p, halves),
            TouchVerdict::Unread(TouchUnread::InBand),
            "a tangent plane in two at {p:?}"
        );
        // The plane y = 0 through the lens's middle crosses both its
        // sheets.
        let mut crossed: Vec<_> =
            lens(p, rod_and_bore(p, 1.0), quarter([0.0, 0.0, 1.0]), pair(0)).into();
        crossed.push(whole(p, v(0.0, 1.0, 0.0)));
        assert_eq!(
            read(p, crossed),
            TouchVerdict::Crossing,
            "a plane through a lens at {p:?}"
        );
        // One lens about +z against the plane z = 0 about p.
        for (what, void, below, rests) in [
            ("a void in the plane's material", true, false, true),
            ("a void outside it", true, true, false),
            ("a sliver outside it", false, true, true),
            ("a sliver in its material", false, false, false),
        ] {
            let (rod, bore) = rod_and_bore(p, 1.0);
            let sheets = if void { (rod, bore) } else { (bore, rod) };
            let mut pieces: Vec<_> = lens(p, sheets, quarter([0.0, 0.0, 1.0]), pair(0)).into();
            pieces.push(whole(p, v(0.0, 0.0, if below { 1.0 } else { -1.0 })));
            let got = read(p, pieces);
            if rests {
                assert!(
                    matches!(got, TouchVerdict::Rest(_)),
                    "{what} at {p:?}: {got:?}"
                );
            } else {
                assert_eq!(got, unread, "{what} at {p:?}");
            }
        }
    }
}

#[test]
fn a_lens_is_the_void_or_the_sliver_its_sheets_bend_to_f64() {
    a_lens_is_the_void_or_the_sliver_its_sheets_bend_to::<f64>();
}

#[test]
fn a_lens_is_the_void_or_the_sliver_its_sheets_bend_to_interval() {
    a_lens_is_the_void_or_the_sliver_its_sheets_bend_to::<Interval>();
}

/// **A lens the bends do not decide is not read**: sheets that do not
/// meet along their bounds one order down (a bore of radius 2, which
/// holds neither curve); a bore turned off the rod's axis by so little
/// that the sheets part across the wedge by less than the band at its
/// arm; two sheets on one wedge along edges of their own; and a void
/// just past the escalation whose second plane leans off the first by
/// nine tenths of the band's zero at the arm, which rests unleaning. Red
/// where a lens is read without its bounds, a parting in band is taken
/// for a side, sheets merely on each other are refused as planes on
/// each other, or the lean is not charged.
fn a_lens_the_bends_do_not_decide_is_not_read<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    let zero = band().zero();
    for p in apexes::<T>() {
        let k = edges(4);
        let pair = |i: usize| [k[i], k[i + 1]];
        let mut closed: Vec<_> =
            lens(p, rod_and_bore(p, 2.0), quarter([0.0, 0.0, 1.0]), pair(0)).into();
        closed.extend(lens(
            p,
            rod_and_bore(p, 2.0),
            quarter([0.0, 0.0, -1.0]),
            pair(2),
        ));
        assert_eq!(read(p, closed), unread, "a bore of radius 2 at {p:?}");
        // Axes `ε` apart part the sheets by about `ε·l²` at the middle:
        // here a tenth of the band's zero at the arm.
        let tilt = 0.1 * zero;
        let near_rod = |up: f64, keys| {
            let (rod, _) = rod_and_bore(p, 1.0);
            let twin = geom::Surface::Cylinder {
                origin: p - v::<T>(1.0, 0.0, 0.0),
                axis: v(0.0, tilt.cos(), tilt.sin()),
                radius: T::from_f64(1.0),
                u_ref: v(1.0, 0.0, 0.0),
            };
            let (d0, d1) = quarter::<T>([0.0, 0.0, up]);
            lens(p, (rod, twin), (d0, d1), keys)
        };
        let mut twins: Vec<_> = near_rod(1.0, pair(0)).into();
        twins.extend(near_rod(-1.0, pair(2)));
        assert_eq!(
            read(p, twins),
            unread,
            "a bore off the rod's axis by {tilt} at {p:?}"
        );
        // Sheets on one wedge along edges of their own are no lens: they
        // meet only to first order, unread, not refused as faces on each
        // other.
        let own = edges(8);
        let mut apart = Vec::new();
        for (up, i) in [(1.0, 0), (-1.0, 4)] {
            let (rod, bore) = rod_and_bore(p, 1.0);
            let (d0, d1) = quarter::<T>([0.0, 0.0, up]);
            apart.push(corner(
                p,
                (rod, true),
                v(1.0, 0.0, 0.0),
                (d0, d1),
                [own[i], own[i + 1]],
            ));
            apart.push(corner(
                p,
                (bore, false),
                v(-1.0, 0.0, 0.0),
                (d1, d0),
                [own[i + 2], own[i + 3]],
            ));
        }
        assert_eq!(
            read(p, apart),
            unread,
            "two sheets along edges of their own at {p:?}"
        );
        // A void just past the escalation at the arm `√2` (`l²/2 = 1`,
        // each radius `1/q`), its second plane leaning off the first by
        // nine tenths of the zero there: the lean is charged.
        let q = band().escalate() + 0.5 * zero;
        let lean = 0.9 * zero / core::f64::consts::SQRT_2;
        let thin = |lean: f64| {
            let mut out: Vec<_> = leaning(
                p,
                rod_and_bore_of(p, 1.0 / q, 1.0 / q),
                quarter([0.0, 0.0, 1.0]),
                pair(0),
                lean,
            )
            .into();
            out.extend(lens(
                p,
                rod_and_bore_of(p, 1.0 / q, 1.0 / q),
                quarter([0.0, 0.0, -1.0]),
                pair(2),
            ));
            out
        };
        let got = read(p, thin(0.0));
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "a void just past the escalation at {p:?}: {got:?}"
        );
        assert_eq!(
            read(p, thin(lean)),
            unread,
            "the void with its second plane leaning at {p:?}"
        );
    }
}

#[test]
fn a_lens_the_bends_do_not_decide_is_not_read_f64() {
    a_lens_the_bends_do_not_decide_is_not_read::<f64>();
}

#[test]
fn a_lens_the_bends_do_not_decide_is_not_read_interval() {
    a_lens_the_bends_do_not_decide_is_not_read::<Interval>();
}

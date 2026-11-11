//! **Two sheets on one wedge, read one order down**
//! ([`second_order::lenses`], [`rest::one_material`]): a rod and a bore
//! of one radius whose axes cross, tangent at `p` along `x = 1` (about
//! `p`), each face's corner there bounded by the two curves the
//! carriers meet in. Their sheets part between those curves: a void
//! where each stands on the other's outward side, a sliver of material
//! where each stands behind the other. Each row near the origin and a
//! kilometre off it, in `f64` and in `Interval`, its directions in the
//! frame [`v`] turns.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Interval, Tol};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

/// `(x, y, z)` in the rows' frame, turned a third of a turn about
/// `(1, 1, 1)` to `(z, x, y)`: the whole plane square to a lens's middle
/// then spans the apex's `y` and `z`. Spanning its `x` and `y` (a
/// kilometre and two off), its own rays read against its own wedges on
/// `Interval` at the finest band in band, before any lens is read (as
/// `second_order_rows::turned` finds).
fn v<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(z), T::from_f64(x), T::from_f64(y))
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
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::Unreadable));
    for (at, p) in apexes::<T>().into_iter().enumerate() {
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
        // sheets. It holds the middle, so it spans the apex's `x`: a
        // kilometre off, on `Interval` at the finest band, its own rays
        // read in band (as [`v`] says), and it refuses there.
        let mut crossed: Vec<_> =
            lens(p, rod_and_bore(p, 1.0), quarter([0.0, 0.0, 1.0]), pair(0)).into();
        crossed.push(whole(p, v(0.0, 1.0, 0.0)));
        let got = read(p, crossed);
        let far = at == 1;
        assert!(
            got == TouchVerdict::Crossing
                || (far && got == TouchVerdict::Unread(TouchUnread::InBand)),
            "a plane through a lens at {p:?}: {got:?}"
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
/// arm; a curve both faces hold along one bound only, and a bound 2°
/// off the curve's tangent (each bound is read); a rod flush in a wider
/// bore along their common ruling, two
/// sheets on one half-plane that share no two edges; and a void
/// just past the escalation whose second plane leans off the first by
/// nine tenths of the band's zero at the arm, which rests unleaning. Red
/// where a lens is read without its bounds, a parting in band is taken
/// for a side, sheets merely on each other are refused as planes on
/// each other, or the lean is not charged.
fn a_lens_the_bends_do_not_decide_is_not_read<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::Unreadable));
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
        // A rod in a bore of radius 2 along their common ruling, `p`
        // inside it: two sheets on one half-plane, tied along one line
        // edge each, which the first order keeps. No lens: they meet only
        // to first order, unread, not refused as faces on each other.
        let own = edges(2);
        let along = |surface, sense, m: Vec3<T>, dir: f64, edge| {
            let fan = TangentFan::Edge {
                along: v(0.0, dir, 0.0),
                reach: T::from_f64(2.0),
                edge: Bound {
                    edge,
                    line: true,
                    bend: Vec3::zero(),
                },
            };
            FaceSector::tangent(surface, sense, m, p, fan, band()).expect("an edge's face")
        };
        let flush = vec![
            along(
                cylinder(p, v(0.0, 1.0, 0.0), 1.0),
                true,
                v(1.0, 0.0, 0.0),
                1.0,
                own[0],
            ),
            along(
                cylinder(p, v(0.0, 1.0, 0.0), 2.0),
                false,
                v(-1.0, 0.0, 0.0),
                -1.0,
                own[1],
            ),
        ];
        assert_eq!(read(p, flush), unread, "a rod flush in a bore at {p:?}");
        // One bound's sheets meet one order down and the other's do not:
        // a cylinder of radius `1 + sin 2ψ` turned `ψ = 60°` off the bore's
        // axis toward `−y` holds the rod's curve along `u₀` alone; and the
        // rod and the bore on a wedge whose second ray is turned 2° off
        // their curve's tangent. Each refuses on that bound.
        let psi = 60f64.to_radians();
        let turned = geom::Surface::Cylinder {
            origin: p - v::<T>(1.0 + (2.0 * psi).sin(), 0.0, 0.0),
            axis: v(0.0, -psi.sin(), psi.cos()),
            radius: T::from_f64(1.0 + (2.0 * psi).sin()),
            u_ref: v(1.0, 0.0, 0.0),
        };
        let one: Vec<_> = lens(
            p,
            (rod_and_bore(p, 1.0).0, turned),
            quarter([0.0, 0.0, 1.0]),
            pair(0),
        )
        .into();
        assert_eq!(
            read(p, one),
            unread,
            "a curve held along one bound only at {p:?}"
        );
        let off = 137f64.to_radians();
        let (d0, _) = quarter::<T>([0.0, 0.0, 1.0]);
        let d1 = v::<T>(0.0, 2.0 * off.cos(), 2.0 * off.sin());
        let skew: Vec<_> = lens(p, rod_and_bore(p, 1.0), (d0, d1), pair(0)).into();
        assert_eq!(
            read(p, skew),
            unread,
            "a bound 2° off its curve's tangent at {p:?}"
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

/// A planar face cornered at `p`, of outward unit `m`, from `d0` round
/// to `d1` along the line edges `ends`.
fn flat<T: Decide>(
    p: Point3<T>,
    m: Vec3<T>,
    (d0, d1): (Vec3<T>, Vec3<T>),
    ends: [EdgeKey; 2],
) -> TangentPiece<T> {
    let surface = geom::Surface::Plane {
        origin: p,
        normal: m,
        u_ref: m.cross(v::<T>(1.0, 0.3, 0.7)).normalize(),
    };
    let line = |edge| Bound {
        edge,
        line: true,
        bend: Vec3::zero(),
    };
    let fan = TangentFan::Corner {
        d0,
        d1,
        bounds: ends.map(line),
        spur: false,
    };
    FaceSector::tangent(surface, true, m, p, fan, band()).expect("a planar corner")
}

/// A box corner at `p` whose edge `a` runs along `(0, 0.3, 1)`, on the
/// lens plane `x = 0` strictly inside the quarter about `+z`, and whose
/// other two edges leave that plane toward `+x`: three planar corners, the corner a
/// hole (`hole`, each outward normal into it) or solid.
fn box_corner<T: Decide>(p: Point3<T>, hole: bool, k: [EdgeKey; 3]) -> Vec<TangentPiece<T>> {
    let unit = |[x, y, z]: [f64; 3]| {
        let n = (x * x + y * y + z * z).sqrt();
        [x / n, y / n, z / n]
    };
    let cross =
        |[a, b, c]: [f64; 3], [x, y, z]: [f64; 3]| [b * z - c * y, c * x - a * z, a * y - b * x];
    let dot = |[a, b, c]: [f64; 3], [x, y, z]: [f64; 3]| a * x + b * y + c * z;
    let a = unit([0.0, 0.3, 1.0]);
    let w = unit(cross(a, [1.0, 0.0, 0.0]));
    let off = |t: f64| unit([1.0, t * w[1], t * w[2]]);
    let (mut b, mut c) = (off(0.6), off(-0.6));
    if dot(cross(a, b), c) < 0.0 {
        std::mem::swap(&mut b, &mut c);
    }
    let vec = |[x, y, z]: [f64; 3]| v::<T>(2.0 * x, 2.0 * y, 2.0 * z);
    // Each face's normal into the cone the three edges span: out of the
    // material round a hole, into it round a solid corner.
    let into = |e: [f64; 3], f: [f64; 3]| {
        let [x, y, z] = unit(cross(e, f));
        if hole {
            v::<T>(x, y, z)
        } else {
            v::<T>(-x, -y, -z)
        }
    };
    let [ka, kb, kc] = k;
    [(a, b, ka, kb), (b, c, kb, kc), (c, a, kc, ka)]
        .into_iter()
        .map(|(e, f, ke, kf)| {
            if hole {
                flat(p, into(e, f), (vec(e), vec(f)), [ke, kf])
            } else {
                flat(p, into(e, f), (vec(f), vec(e)), [kf, ke])
            }
        })
        .collect()
}

/// **A sheet reaching a lens along a ray of it refuses in band**: a void
/// lens with a hole's corner, and a sliver lens with a solid corner,
/// each corner's edge on the lens plane strictly inside the wedge. One
/// order down that edge's faces may enter the lens, which nothing reads;
/// the reading refuses typed, in band, as a sheet meeting the wedge at a
/// bound of it refuses, not skipped. Red where such a sheet is passed or
/// left unread.
fn a_sheet_reaching_a_lens_along_a_ray_of_it_refuses_in_band<T: Decide>() {
    let in_band = TouchVerdict::Unread(TouchUnread::InBand);
    for p in apexes::<T>() {
        let k = edges(5);
        let (rod, bore) = rod_and_bore(p, 1.0);
        let mut void: Vec<_> = lens(p, (rod, bore), quarter([0.0, 0.0, 1.0]), [k[0], k[1]]).into();
        void.extend(box_corner(p, true, [k[2], k[3], k[4]]));
        assert_eq!(
            read(p, void),
            in_band,
            "a void lens with a hole's corner at {p:?}"
        );
        let (rod, bore) = rod_and_bore(p, 1.0);
        let mut sliver: Vec<_> =
            lens(p, (bore, rod), quarter([0.0, 0.0, 1.0]), [k[0], k[1]]).into();
        sliver.extend(box_corner(p, false, [k[2], k[3], k[4]]));
        assert_eq!(
            read(p, sliver),
            in_band,
            "a sliver lens with a solid corner at {p:?}"
        );
    }
}

#[test]
fn a_sheet_reaching_a_lens_along_a_ray_of_it_refuses_in_band_f64() {
    a_sheet_reaching_a_lens_along_a_ray_of_it_refuses_in_band::<f64>();
}

#[test]
fn a_sheet_reaching_a_lens_along_a_ray_of_it_refuses_in_band_interval() {
    a_sheet_reaching_a_lens_along_a_ray_of_it_refuses_in_band::<Interval>();
}

/// **Two sheets bounded by the same two edges on one line are no lens**:
/// a plane face and a cylinder face meeting along two line edges that
/// leave `p` one each way along `x`, the cylinder's tangent plane turned
/// off the plane about `x`, their normals opposed. Their bounds alone lie
/// on both planes; the plane's middle does not, so they are an edge's
/// two faces, which rest. Red where two sheets are taken to lie on each
/// other by their bounds alone.
fn a_dihedral_along_two_edges_on_one_line_is_no_lens<T: Decide>() {
    for p in apexes::<T>() {
        let k = edges(2);
        let x = v::<T>(2.0, 0.0, 0.0);
        let n = v::<T>(0.0, -0.8, -0.6);
        let line = |edge| Bound {
            edge,
            line: true,
            bend: Vec3::zero(),
        };
        let top = flat(p, v(0.0, 0.0, 1.0), (x, -x), [k[0], k[1]]);
        let wall = geom::Surface::Cylinder {
            origin: p - n,
            axis: v(1.0, 0.0, 0.0),
            radius: T::from_f64(1.0),
            u_ref: v(0.0, 0.0, 1.0),
        };
        let fan = TangentFan::Corner {
            d0: -x,
            d1: x,
            bounds: [line(k[1]), line(k[0])],
            spur: false,
        };
        let side = FaceSector::tangent(wall, true, n, p, fan, band()).expect("the wall");
        let got = read(p, vec![top, side]);
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "a dihedral at {p:?}: {got:?}"
        );
    }
}

#[test]
fn a_dihedral_along_two_edges_on_one_line_is_no_lens_f64() {
    a_dihedral_along_two_edges_on_one_line_is_no_lens::<f64>();
}

#[test]
fn a_dihedral_along_two_edges_on_one_line_is_no_lens_interval() {
    a_dihedral_along_two_edges_on_one_line_is_no_lens::<Interval>();
}

/// The three horns of a thorn of material along `+x` from `p` (off the
/// lens plane `x = 0`): one order down the triangle of a line edge at
/// `0`, an arc at `(0, 1, 0)` and an arc at `(0, 0, 1)`, as
/// `second_order_rows` builds it. `along` places the thorn's own `x`,
/// `y` and `z`.
fn thorn<T: Decide>(
    p: Point3<T>,
    keys: [EdgeKey; 3],
    along: fn(f64, f64, f64) -> Vec3<T>,
) -> Vec<TangentPiece<T>> {
    let v = along;
    let d = v(2.0, 0.0, 0.0);
    let bound = |edge, bend: Vec3<T>| Bound {
        edge,
        line: false,
        bend,
    };
    let l = Bound {
        edge: keys[0],
        line: true,
        bend: Vec3::zero(),
    };
    let (a, b) = (
        bound(keys[1], v(0.0, 1.0, 0.0)),
        bound(keys[2], v(0.0, 0.0, 1.0)),
    );
    let s = core::f64::consts::FRAC_1_SQRT_2;
    [
        (v(0.0, 0.0, -1.0), [a, l]),
        (v(0.0, -1.0, 0.0), [l, b]),
        (v(0.0, s, s), [b, a]),
    ]
    .into_iter()
    .map(|(m, ends)| {
        let surface = geom::Surface::Plane {
            origin: p,
            normal: m,
            u_ref: m.cross(v(1.0, 0.3, 0.7)).normalize(),
        };
        let fan = TangentFan::Corner {
            d0: d,
            d1: d,
            bounds: ends,
            spur: false,
        };
        let piece = FaceSector::tangent(surface, true, m, p, fan, band()).expect("a horn");
        assert!(
            matches!(piece, TangentPiece::Horn(_)),
            "a horn of the thorn"
        );
        piece
    })
    .collect()
}

/// **A lens beside a thorn is read against the thorn's region**: a
/// thorn of material along `+x` touching a lens's point, its remainder
/// the thorn's horns alone, whose region round them is no material. A
/// sliver there rests; a void, which needs material round it, refuses;
/// a thorn along the lens's middle or its bound, on its closed wedge,
/// or tipped off its middle by an angle the band leaves undecided,
/// refuses in band. Red where the remainder leaves out its horns, reads
/// their region as material, or a horn on the wedge, on its bound or in
/// band of it is passed.
fn a_lens_beside_a_thorn_is_read_against_its_region<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::Unreadable));
    for p in apexes::<T>() {
        let k = edges(5);
        let with = |sheets| {
            let mut out: Vec<_> = lens(p, sheets, quarter([0.0, 0.0, 1.0]), [k[0], k[1]]).into();
            out.extend(thorn(p, [k[2], k[3], k[4]], v::<T>));
            read(p, out)
        };
        let (rod, bore) = rod_and_bore(p, 1.0);
        let got = with((bore, rod));
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "a sliver beside a thorn at {p:?}: {got:?}"
        );
        let (rod, bore) = rod_and_bore(p, 1.0);
        assert_eq!(with((rod, bore)), unread, "a void beside a thorn at {p:?}");
        // The thorn along the lens's middle, on its wedge: refused in band.
        let (rod, bore) = rod_and_bore(p, 1.0);
        let mut on: Vec<_> = lens(p, (bore, rod), quarter([0.0, 0.0, 1.0]), [k[0], k[1]]).into();
        on.extend(thorn(p, [k[2], k[3], k[4]], |a, b, c| v::<T>(b, c, a)));
        assert_eq!(
            read(p, on),
            TouchVerdict::Unread(TouchUnread::InBand),
            "a thorn along a sliver's middle at {p:?}"
        );
        // The thorn along the lens's bound, and along its middle tipped
        // off the lens plane by an angle the band leaves undecided at the
        // horn's reach: each refused in band.
        for (what, along) in [
            (
                "a thorn along a sliver's bound",
                (|a, b, c| framed::<T>([0.0, 1.0, 1.0], [1.0, 0.0, 0.0], [a, b, c]))
                    as fn(f64, f64, f64) -> Vec3<T>,
            ),
            ("a thorn tipped in band off a sliver's middle", |a, b, c| {
                let t = (band().zero() + band().escalate()) / 4.0;
                framed::<T>([t, 0.0, 1.0], [0.0, 1.0, 0.0], [a, b, c])
            }),
        ] {
            let (rod, bore) = rod_and_bore(p, 1.0);
            let mut pieces: Vec<_> =
                lens(p, (bore, rod), quarter([0.0, 0.0, 1.0]), [k[0], k[1]]).into();
            pieces.extend(thorn(p, [k[2], k[3], k[4]], along));
            assert_eq!(
                read(p, pieces),
                TouchVerdict::Unread(TouchUnread::InBand),
                "{what} at {p:?}"
            );
        }
    }
}

/// `a·d̂ + b·ê + c·(d̂ × ê)` in the rows' frame ([`v`]), `ê` the part of
/// `e` square to `d`, both made unit: a thorn's own `x`, `y` and `z`
/// placed along `d`.
fn framed<T: Real>(d: [f64; 3], e: [f64; 3], [a, b, c]: [f64; 3]) -> Vec3<T> {
    let unit = |[x, y, z]: [f64; 3]| {
        let n = (x * x + y * y + z * z).sqrt();
        [x / n, y / n, z / n]
    };
    let d = unit(d);
    let along = e[0] * d[0] + e[1] * d[1] + e[2] * d[2];
    let e = unit([
        e[0] - along * d[0],
        e[1] - along * d[1],
        e[2] - along * d[2],
    ]);
    let f = [
        d[1] * e[2] - d[2] * e[1],
        d[2] * e[0] - d[0] * e[2],
        d[0] * e[1] - d[1] * e[0],
    ];
    let at = |i: usize| a * d[i] + b * e[i] + c * f[i];
    v(at(0), at(1), at(2))
}

#[test]
fn a_lens_beside_a_thorn_is_read_against_its_region_f64() {
    a_lens_beside_a_thorn_is_read_against_its_region::<f64>();
}

#[test]
fn a_lens_beside_a_thorn_is_read_against_its_region_interval() {
    a_lens_beside_a_thorn_is_read_against_its_region::<Interval>();
}

/// A box corner at `p` whose face `(a, b)` lies in the lens plane
/// `x = 0` and holds the quarter about `+z` strictly inside it, its
/// third edge along `c` off that plane: three planar corners, a hole
/// (`hole`, each outward normal into the corner) or solid.
fn holding_corner<T: Decide>(
    p: Point3<T>,
    hole: bool,
    c: [f64; 3],
    k: [EdgeKey; 3],
) -> Vec<TangentPiece<T>> {
    let unit = |[x, y, z]: [f64; 3]| {
        let n = (x * x + y * y + z * z).sqrt();
        [x / n, y / n, z / n]
    };
    let cross =
        |[a, b, c]: [f64; 3], [x, y, z]: [f64; 3]| [b * z - c * y, c * x - a * z, a * y - b * x];
    let dot = |[a, b, c]: [f64; 3], [x, y, z]: [f64; 3]| a * x + b * y + c * z;
    let a = unit([0.0, 1.0, 0.1]);
    let (mut b, mut c) = (unit([0.0, -1.0, 0.1]), unit(c));
    let (mut kb, mut kc) = (k[1], k[2]);
    if dot(cross(a, b), c) < 0.0 {
        std::mem::swap(&mut b, &mut c);
        std::mem::swap(&mut kb, &mut kc);
    }
    let vec = |[x, y, z]: [f64; 3]| v::<T>(2.0 * x, 2.0 * y, 2.0 * z);
    let into = |e: [f64; 3], f: [f64; 3]| {
        let [x, y, z] = unit(cross(e, f));
        if hole {
            v::<T>(x, y, z)
        } else {
            v::<T>(-x, -y, -z)
        }
    };
    [(a, b, k[0], kb), (b, c, kb, kc), (c, a, kc, k[0])]
        .into_iter()
        .map(|(e, f, ke, kf)| {
            if hole {
                flat(p, into(e, f), (vec(e), vec(f)), [ke, kf])
            } else {
                flat(p, into(e, f), (vec(f), vec(e)), [kf, ke])
            }
        })
        .collect()
}

/// **A sheet in the lens plane holding its wedge never rests**: a box
/// corner whose face lies in the lens plane and holds the whole wedge,
/// hole or solid, its third edge either side of the plane, and a whole
/// plane `x = 0` facing either way, each beside a void and a sliver.
/// Such a sheet lies on both of the lens's sheets to first order and is
/// refused before the lens is read. Red where any reads as a rest.
fn a_sheet_holding_a_lens_never_rests<T: Decide>() {
    for p in apexes::<T>() {
        let k = edges(5);
        let lensed = |void: bool| -> Vec<TangentPiece<T>> {
            let (rod, bore) = rod_and_bore(p, 1.0);
            let sheets = if void { (rod, bore) } else { (bore, rod) };
            lens(p, sheets, quarter([0.0, 0.0, 1.0]), [k[0], k[1]]).into()
        };
        for void in [true, false] {
            for hole in [true, false] {
                for c in [[1.0, 0.0, -0.5], [-1.0, 0.0, -0.5]] {
                    let mut pieces = lensed(void);
                    pieces.extend(holding_corner(p, hole, c, [k[2], k[3], k[4]]));
                    let got = read(p, pieces);
                    assert!(
                        !matches!(got, TouchVerdict::Rest(_)),
                        "void {void}, hole {hole}, third edge {c:?} at {p:?}"
                    );
                }
            }
            for mx in [1.0, -1.0] {
                let mut pieces = lensed(void);
                pieces.push(whole(p, v(mx, 0.0, 0.0)));
                let got = read(p, pieces);
                assert!(
                    !matches!(got, TouchVerdict::Rest(_)),
                    "void {void}, the plane x = 0 facing {mx} at {p:?}"
                );
            }
        }
    }
}

#[test]
fn a_sheet_holding_a_lens_never_rests_f64() {
    a_sheet_holding_a_lens_never_rests::<f64>();
}

#[test]
fn a_sheet_holding_a_lens_never_rests_interval() {
    a_sheet_holding_a_lens_never_rests::<Interval>();
}

/// **Two planar faces on one wedge refuse before a tie of the link is
/// read**: two faces in `z = 0` on the quarter from `x` to `y`, outward
/// `±z`, bounded by the same two line edges, beside a sliver whose
/// wedge holds `y`. First order would read the faces' bound along `y`
/// inside the sliver's sheets and refuse it as unread, where the two
/// faces fold; the faces lie on each other over the quarter, which
/// refuses `Degenerate` first. Red where a slit of one wedge is read by
/// its rays alone.
fn a_slit_of_one_wedge_refuses_before_its_ties<T: Decide>() {
    for p in apexes::<T>() {
        let k = edges(4);
        let (x, y) = (v::<T>(2.0, 0.0, 0.0), v::<T>(0.0, 2.0, 0.0));
        let mut pieces: Vec<_> = lens(
            p,
            rod_and_bore(p, 1.0),
            quarter([0.0, 1.0, 0.0]),
            [k[0], k[1]],
        )
        .into();
        pieces.push(flat(p, v(0.0, 0.0, 1.0), (x, y), [k[2], k[3]]));
        pieces.push(flat(p, v(0.0, 0.0, -1.0), (y, x), [k[3], k[2]]));
        assert_eq!(
            read(p, pieces),
            TouchVerdict::Unread(TouchUnread::Degenerate),
            "a slit of one wedge beside a sliver at {p:?}"
        );
    }
}

#[test]
fn a_slit_of_one_wedge_refuses_before_its_ties_f64() {
    a_slit_of_one_wedge_refuses_before_its_ties::<f64>();
}

#[test]
fn a_slit_of_one_wedge_refuses_before_its_ties_interval() {
    a_slit_of_one_wedge_refuses_before_its_ties::<Interval>();
}

/// A face on `surface` of `Face::sense` `sense` cornered at `p`, of
/// outward unit `m`, from `d0` round to `d1` along `bounds`.
fn corner_along<T: Decide>(
    p: Point3<T>,
    (surface, sense): (geom::Surface<T>, bool),
    m: Vec3<T>,
    (d0, d1): (Vec3<T>, Vec3<T>),
    bounds: [Bound<T>; 2],
) -> TangentPiece<T> {
    let fan = TangentFan::Corner {
        d0,
        d1,
        bounds,
        spur: false,
    };
    FaceSector::tangent(surface, sense, m, p, fan, band()).expect("a corner's piece")
}

/// The plane through `p` of outward unit `m`.
fn plane<T: Real>(p: Point3<T>, m: Vec3<T>) -> geom::Surface<T> {
    geom::Surface::Plane {
        origin: p,
        normal: m,
        u_ref: m.cross(v::<T>(1.0, 0.3, 0.7)).normalize(),
    }
}

/// The edge `edge` along a line through `p`.
fn line<T: Real>(edge: EdgeKey) -> Bound<T> {
    Bound {
        edge,
        line: true,
        bend: Vec3::zero(),
    }
}

/// The edge `edge` along a curve bending by `bend` there.
fn curve<T: Real>(edge: EdgeKey, bend: Vec3<T>) -> Bound<T> {
    Bound {
        edge,
        line: false,
        bend,
    }
}

/// The upright cylinder of radius `r` through `p`, its axis `r` behind
/// `p` along `turn·x` (`turn` ±1).
fn upright<T: Real>(p: Point3<T>, r: f64, turn: f64) -> geom::Surface<T> {
    geom::Surface::Cylinder {
        origin: p - v::<T>(turn * r, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        radius: T::from_f64(r),
        u_ref: v(turn, 0.0, 0.0),
    }
}

/// **A lune about `p`** as two lune cavities kissing at their cusps
/// build it: between the upright cylinders of radii `r1 < r2` through
/// `p` ([`upright`]), over `z > 0`, from their shared cusp line `k[0]`
/// along `+z` to their arcs `k[1]` and `k[2]` along `turn·y`, where the
/// floor `z = 0` caps it with a horn between the arcs. `void`: the
/// cylinders' material outside it (a cavity); otherwise a sliver of
/// material. `turn` −1 is the lune turned a half turn about `z`. The
/// sheets part along `y` by `1/r1 − 1/r2`, along `z` not at all, and the
/// form's polar value on `y` and `z` is zero: one order down a void or a
/// sliver, pinched along the cusp line.
fn lune<T: Decide>(
    p: Point3<T>,
    [r1, r2]: [f64; 2],
    turn: f64,
    void: bool,
    k: [EdgeKey; 3],
) -> Vec<TangentPiece<T>> {
    let y = v::<T>(0.0, 2.0 * turn, 0.0);
    let z = v::<T>(0.0, 0.0, 2.0);
    let x = |a: f64| v::<T>(turn * a, 0.0, 0.0);
    let (a1, a2) = (curve(k[1], x(-1.0 / r1)), curve(k[2], x(-1.0 / r2)));
    if void {
        vec![
            corner_along(
                p,
                (upright(p, r1, turn), true),
                x(1.0),
                (y, z),
                [a1, line(k[0])],
            ),
            corner_along(
                p,
                (upright(p, r2, turn), false),
                x(-1.0),
                (z, y),
                [line(k[0]), a2],
            ),
            corner_along(
                p,
                (plane(p, v(0.0, 0.0, 1.0)), true),
                v(0.0, 0.0, 1.0),
                (y, y),
                [a2, a1],
            ),
        ]
    } else {
        vec![
            corner_along(
                p,
                (upright(p, r1, turn), false),
                x(-1.0),
                (z, y),
                [line(k[0]), a1],
            ),
            corner_along(
                p,
                (upright(p, r2, turn), true),
                x(1.0),
                (y, z),
                [a2, line(k[0])],
            ),
            corner_along(
                p,
                (plane(p, v(0.0, 0.0, -1.0)), true),
                v(0.0, 0.0, -1.0),
                (y, y),
                [a1, a2],
            ),
        ]
    }
}

/// **A lune is the void or the sliver its cap closes**: two sheets on one
/// quarter of the plane `x = 0`, pinched along their shared cusp line and
/// capped by a horn of the floor between their arcs. A void alone, two
/// voids kissing at their cusps (the two lune cavities' link), and a
/// sliver alone rest. Read against a whole plane through `p` that misses
/// the quarter, a void inside the plane's material and a sliver outside it
/// rest, the other two are refused; a plane through the lune's middle
/// crosses it; a cap turned over, its normal out of a void, refuses; and
/// a horn of the arcs' edges along another ray caps no lune. Red where a
/// lune's kind is read the other way round, its cap is left in the
/// remainder's horns or found off its ray, the region round it is not
/// read, or a sheet crossing it is passed.
fn a_lune_is_the_void_or_the_sliver_its_cap_closes<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::Unreadable));
    let rests = |got: &TouchVerdict| matches!(got, TouchVerdict::Rest(_));
    for (at, p) in apexes::<T>().into_iter().enumerate() {
        let k = edges(6);
        let one = [k[0], k[1], k[2]];
        let got = read(p, lune(p, [1.0, 2.0], 1.0, true, one));
        assert!(rests(&got), "a void lune at {p:?}: {got:?}");
        let mut kissing = lune(p, [1.0, 2.0], 1.0, true, one);
        kissing.extend(lune(p, [1.0, 2.0], -1.0, true, [k[3], k[4], k[5]]));
        let got = read(p, kissing);
        assert!(rests(&got), "two void lunes kissing at {p:?}: {got:?}");
        let got = read(p, lune(p, [1.0, 2.0], 1.0, false, one));
        assert!(rests(&got), "a sliver lune at {p:?}: {got:?}");
        // The plane y + z = 0 meets the lune's plane along `±(0, 1, −1)`,
        // off the quarter; the lune's middle lies on its `+` side.
        let n = |s: f64| v::<T>(0.0, s, s) * T::from_f64(core::f64::consts::FRAC_1_SQRT_2);
        for (what, void, front, rests_there) in [
            ("a void in the plane's material", true, false, true),
            ("a void outside it", true, true, false),
            ("a sliver outside it", false, true, true),
            ("a sliver in its material", false, false, false),
        ] {
            let mut pieces = lune(p, [1.0, 2.0], 1.0, void, one);
            pieces.push(whole(p, n(if front { 1.0 } else { -1.0 })));
            let got = read(p, pieces);
            if rests_there {
                assert!(rests(&got), "{what} at {p:?}: {got:?}");
            } else {
                assert_eq!(got, unread, "{what} at {p:?}");
            }
        }
        // The plane y = z holds the lune's middle and crosses both its
        // sheets; a kilometre off on `Interval` it may read in band.
        let mut crossed = lune(p, [1.0, 2.0], 1.0, true, one);
        crossed.push(whole(
            p,
            v::<T>(0.0, 1.0, -1.0) * T::from_f64(core::f64::consts::FRAC_1_SQRT_2),
        ));
        let got = read(p, crossed);
        assert!(
            got == TouchVerdict::Crossing
                || (at == 1 && got == TouchVerdict::Unread(TouchUnread::InBand)),
            "a plane through a lune at {p:?}: {got:?}"
        );
        // The void's cap turned over: the floor's normal out of the void.
        let mut over = lune(p, [1.0, 2.0], 1.0, true, one);
        let y = v::<T>(0.0, 2.0, 0.0);
        let (a1, a2) = (
            curve(k[1], v::<T>(-1.0, 0.0, 0.0)),
            curve(k[2], v::<T>(-0.5, 0.0, 0.0)),
        );
        over[2] = corner_along(
            p,
            (plane(p, v(0.0, 0.0, -1.0)), true),
            v(0.0, 0.0, -1.0),
            (y, y),
            [a1, a2],
        );
        assert_eq!(read(p, over), unread, "a void's cap turned over at {p:?}");
        // A horn of the two arcs' edges along `−y`, none along `+y`: no
        // cap, so no lune, and its sheets stay on each other.
        let mut elsewhere = lune(p, [1.0, 2.0], 1.0, true, one);
        elsewhere[2] = corner_along(
            p,
            (plane(p, v(0.0, 0.0, 1.0)), true),
            v(0.0, 0.0, 1.0),
            (-y, -y),
            [a1, a2],
        );
        assert_eq!(
            read(p, elsewhere),
            unread,
            "a horn of the arcs' edges along another ray at {p:?}"
        );
    }
}

#[test]
fn a_lune_is_the_void_or_the_sliver_its_cap_closes_f64() {
    a_lune_is_the_void_or_the_sliver_its_cap_closes::<f64>();
}

#[test]
fn a_lune_is_the_void_or_the_sliver_its_cap_closes_interval() {
    a_lune_is_the_void_or_the_sliver_its_cap_closes::<Interval>();
}

/// **The rod and the bore of [`rod_and_bore`] on one wedge**, from the
/// curve they share along `(0, s0.0, s0.1)` (a pinch) to a cap along the
/// unit `(0, u1.0, u1.1)`, each face's corner there bounded by its own
/// arc of the floor's plane, which holds `u1` and `x`; the floor's horn
/// faces into the wedge. The sheets part there by `u1.1² − u1.0²`, and
/// the form's polar value on the two rays is `s0.1·u1.1 − s0.0·u1.0` over
/// `√2`.
fn capped<T: Decide>(
    p: Point3<T>,
    s0: (f64, f64),
    u1: (f64, f64),
    k: [EdgeKey; 3],
) -> Vec<TangentPiece<T>> {
    let (rod, bore) = rod_and_bore(p, 1.0);
    let r = |(a, b): (f64, f64)| v::<T>(0.0, 2.0 * a, 2.0 * b);
    let shared = curve(k[0], v::<T>(-0.5, 0.0, 0.0));
    let (on_rod, on_bore) = (
        curve(k[1], v::<T>(-u1.1 * u1.1, 0.0, 0.0)),
        curve(k[2], v::<T>(-u1.0 * u1.0, 0.0, 0.0)),
    );
    // The floor's normal: square to `u1` in the plane `x = 0`, toward
    // the wedge's middle. Its horn turns from the arc bending less toward
    // `−x` to the one bending more about `m × u1`, which is `−x` where
    // the normal is `(0, −u1.1, u1.0)` and `+x` where it is turned over.
    let (mid_y, mid_z) = (s0.0 / 2f64.sqrt() + u1.0, s0.1 / 2f64.sqrt() + u1.1);
    let side = if -u1.1 * mid_y + u1.0 * mid_z > 0.0 {
        1.0
    } else {
        -1.0
    };
    let m = v::<T>(0.0, -side * u1.1, side * u1.0);
    let (rod_bend, bore_bend) = (u1.1 * u1.1, u1.0 * u1.0);
    let rod_last = (rod_bend > bore_bend) == (side > 0.0);
    let ends = if rod_last {
        [on_bore, on_rod]
    } else {
        [on_rod, on_bore]
    };
    vec![
        corner_along(
            p,
            (rod, true),
            v(1.0, 0.0, 0.0),
            (r(u1), r(s0)),
            [on_rod, shared],
        ),
        corner_along(
            p,
            (bore, false),
            v(-1.0, 0.0, 0.0),
            (r(s0), r(u1)),
            [shared, on_bore],
        ),
        corner_along(p, (plane(p, m), true), m, (r(u1), r(u1)), ends),
    ]
}

/// **A lune the bends do not decide is not read**. The rod and the bore
/// capped along `z` from their curve along `(0, −1, 1)` rest, a void;
/// capped along `(0, 0.2, −1)` from their curve along `(0, 1, 1)` they
/// part at both ends the same way but cross inside the wedge (the form's
/// polar value on its rays has the other sign), and are not read; so are
/// they on the quarter from `y` to `z` capped at both ends, parting one
/// way at one cap and the other way at the other. A lune
/// whose second sheet does not hold the cusp line (its cylinder turned
/// 30° off it) is not read. A void just past the escalation at its cap
/// rests, and with its second plane leaning off the first by nine tenths
/// of the band's zero there it is not read: the lean is charged. A curved
/// sheet tangent to the floor along the cap's ray, bending away under it,
/// refuses in band: the cut there does not read the cap. Red where the
/// polar value, the pinch, the caps' agreement or the charge is not read,
/// or another sheet at a cap's ray is passed.
fn a_lune_the_bends_do_not_decide_is_not_read<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::Unreadable));
    let zero = band().zero();
    let unit = |(a, b): (f64, f64)| {
        let n = f64::hypot(a, b);
        (a / n, b / n)
    };
    for p in apexes::<T>() {
        let k = edges(3);
        let one = [k[0], k[1], k[2]];
        let got = read(p, capped(p, (-1.0, 1.0), (0.0, 1.0), one));
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "the rod and the bore capped along z at {p:?}: {got:?}"
        );
        assert_eq!(
            read(p, capped(p, (1.0, 1.0), unit((0.2, -1.0)), one)),
            unread,
            "sheets crossing inside the wedge at {p:?}"
        );
        // The rod and the bore on the quarter from `y` to `z`, capped at
        // both: the bore stands past the rod by `−1` at `y` and `+1` at `z`,
        // so the sheets cross between them.
        let (y, z) = (v::<T>(0.0, 2.0, 0.0), v::<T>(0.0, 0.0, 2.0));
        let k5 = edges(4);
        let (rod_y, rod_z) = (
            curve(k5[0], Vec3::zero()),
            curve(k5[1], v::<T>(-1.0, 0.0, 0.0)),
        );
        let (bore_y, bore_z) = (
            curve(k5[2], v::<T>(-1.0, 0.0, 0.0)),
            curve(k5[3], Vec3::zero()),
        );
        let (rod, bore) = rod_and_bore(p, 1.0);
        let (up, side) = (v::<T>(0.0, 0.0, 1.0), v::<T>(0.0, 1.0, 0.0));
        let crossed = vec![
            corner_along(p, (rod, true), v(1.0, 0.0, 0.0), (y, z), [rod_y, rod_z]),
            corner_along(
                p,
                (bore, false),
                v(-1.0, 0.0, 0.0),
                (z, y),
                [bore_z, bore_y],
            ),
            corner_along(p, (plane(p, up), true), up, (y, y), [rod_y, bore_y]),
            corner_along(p, (plane(p, side), true), side, (z, z), [rod_z, bore_z]),
        ];
        assert_eq!(
            read(p, crossed),
            unread,
            "sheets parting each way at two caps at {p:?}"
        );
        // The second sheet's cylinder turned 30° off the cusp line about
        // `x`: it bends `sin²30°/2` along `z`, its arc `cos²30°/2` along `y`.
        let psi = 30f64.to_radians();
        let turned = geom::Surface::Cylinder {
            origin: p - v::<T>(2.0, 0.0, 0.0),
            axis: v(0.0, psi.sin(), psi.cos()),
            radius: T::from_f64(2.0),
            u_ref: v(1.0, 0.0, 0.0),
        };
        let mut off = lune(p, [1.0, 2.0], 1.0, true, one);
        let arc = curve(k[2], v::<T>(-psi.cos().powi(2) / 2.0, 0.0, 0.0));
        off[1] = corner_along(
            p,
            (turned, false),
            v(-1.0, 0.0, 0.0),
            (z, y),
            [line(k[0]), arc],
        );
        off[2] = corner_along(
            p,
            (plane(p, v(0.0, 0.0, 1.0)), true),
            v(0.0, 0.0, 1.0),
            (y, y),
            [arc, curve(k[1], v::<T>(-1.0, 0.0, 0.0))],
        );
        assert_eq!(
            read(p, off),
            unread,
            "a cusp line one sheet leaves at {p:?}"
        );
        // The arm is 2 (`l²/2 = 2`): the sheets part at the cap by
        // `2·(1/r1 − 1/r2)`, here just past the escalation.
        let gap = (band().escalate() + 0.5 * zero) / 2.0;
        let r1 = 1.0 / (1.0 + gap);
        let lean = 0.9 * zero / 2.0;
        let thin = |lean: f64| {
            let mut out = lune(p, [r1, 1.0], 1.0, true, one);
            let back = v::<T>(-(1.0 - lean * lean).sqrt(), lean, 0.0);
            out[1] = corner_along(
                p,
                (upright(p, 1.0, 1.0), false),
                back,
                (z, y),
                [line(k[0]), curve(k[2], v::<T>(-1.0, 0.0, 0.0))],
            );
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
        // A cylinder along `x` under the floor, tangent to it at `p`,
        // its material above: in the cut along `y` a line below the cap.
        let under = geom::Surface::Cylinder {
            origin: p - v::<T>(0.0, 0.0, 3.0),
            axis: v(1.0, 0.0, 0.0),
            radius: T::from_f64(3.0),
            u_ref: v(0.0, 0.0, 1.0),
        };
        let fan = TangentFan::Inside {
            reach: T::from_f64(2.0),
            frame: [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)],
        };
        let mut crowded = lune(p, [1.0, 2.0], 1.0, true, one);
        crowded.push(
            FaceSector::tangent(under, false, v(0.0, 0.0, -1.0), p, fan, band())
                .expect("a whole sheet"),
        );
        assert_eq!(
            read(p, crowded),
            TouchVerdict::Unread(TouchUnread::InBand),
            "a sheet along a cap's ray at {p:?}"
        );
    }
}

#[test]
fn a_lune_the_bends_do_not_decide_is_not_read_f64() {
    a_lune_the_bends_do_not_decide_is_not_read::<f64>();
}

#[test]
fn a_lune_the_bends_do_not_decide_is_not_read_interval() {
    a_lune_the_bends_do_not_decide_is_not_read::<Interval>();
}

/// **A crescent's walls, read at its floor's rim**: `A`'s wall, the
/// upright cylinder of radius `ra` through `p` ([`upright`]), stands in
/// `B`'s, of radius 2, over `z > 0`, tangent along their common ruling,
/// `B`'s wall split there by its line edges along `±z` and `A`'s by its
/// seam along `+z`; `A`'s floor `z = 0` closes it below, its rim one
/// circle through `p` along `±y`. `B ∖ A` near `p`: the crescent's two
/// slivers between the walls, each running into `B`'s material under the
/// floor at the rim. Each sliver is a lune whose `B` sheet runs on past
/// the rim (a mouth).
fn crescent<T: Decide>(p: Point3<T>, ra: f64, k: [EdgeKey; 4]) -> Vec<TangentPiece<T>> {
    let (y, z) = (v::<T>(0.0, 2.0, 0.0), v::<T>(0.0, 0.0, 2.0));
    let [up, down, seam, rim] = k;
    let rim = curve(rim, v::<T>(-1.0 / ra, 0.0, 0.0));
    let outer = |d0, d1, ends: [EdgeKey; 2]| {
        corner_along(
            p,
            (upright(p, 2.0, 1.0), true),
            v(1.0, 0.0, 0.0),
            (d0, d1),
            ends.map(line),
        )
    };
    let inner = |d0, d1, ends| {
        corner_along(
            p,
            (upright(p, ra, 1.0), false),
            v(-1.0, 0.0, 0.0),
            (d0, d1),
            ends,
        )
    };
    vec![
        outer(-z, z, [down, up]),
        outer(z, -z, [up, down]),
        inner(z, y, [line(seam), rim]),
        inner(-y, z, [rim, line(seam)]),
        corner_along(
            p,
            (plane(p, v(0.0, 0.0, 1.0)), true),
            v(0.0, 0.0, 1.0),
            (y, -y),
            [rim, rim],
        ),
    ]
}

/// **A crescent's slivers open into the material at its rim**: `A` of
/// radius 1 in `B` ([`crescent`]) rests, each sliver read as a lune whose
/// `B` sheet runs on past the rim, the rest of `B`'s wall bounded there
/// by the rim. A plane through a sliver's middle crosses it; `A` of
/// radius 3 stands outside `B` and the floor crosses `B`'s wall in the cut
/// at the rim; and `A` of a radius the band does not part from `B`'s is
/// not read. Red where a mouth is not read, `B`'s wall is not cut short in
/// the remainder, or a sliver's kind is read the other way round.
fn a_crescents_slivers_open_into_the_material_at_its_rim<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Curved(CurvedUnread::Unreadable));
    let zero = band().zero();
    for (at, p) in apexes::<T>().into_iter().enumerate() {
        let k = edges(4);
        let k = [k[0], k[1], k[2], k[3]];
        let got = read(p, crescent(p, 1.0, k));
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "a crescent at {p:?}: {got:?}"
        );
        let mut crossed = crescent(p, 1.0, k);
        crossed.push(whole(
            p,
            v::<T>(0.0, 1.0, -1.0) * T::from_f64(core::f64::consts::FRAC_1_SQRT_2),
        ));
        let got = read(p, crossed);
        assert!(
            got == TouchVerdict::Crossing
                || (at == 1 && got == TouchVerdict::Unread(TouchUnread::InBand)),
            "a plane through a crescent's sliver at {p:?}: {got:?}"
        );
        assert_eq!(read(p, crescent(p, 3.0, k)), unread, "A outside B at {p:?}");
        // The arm is 2: radii parting the sheets by a tenth of the zero.
        let near = 1.0 / (0.5 + 0.05 * zero);
        assert_eq!(
            read(p, crescent(p, near, k)),
            unread,
            "A's wall in band of B's at {p:?}"
        );
    }
}

#[test]
fn a_crescents_slivers_open_into_the_material_at_its_rim_f64() {
    a_crescents_slivers_open_into_the_material_at_its_rim::<f64>();
}

#[test]
fn a_crescents_slivers_open_into_the_material_at_its_rim_interval() {
    a_crescents_slivers_open_into_the_material_at_its_rim::<Interval>();
}

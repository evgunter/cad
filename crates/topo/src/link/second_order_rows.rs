//! **A tangent link one order down** ([`second_order`]): a cusp corner
//! read by which side of its ray each edge bends to; the thorn a ring of
//! horns bounds, against the cone round it; and sheets along one ray
//! held apart, or not, by their edges' bends. Each row near the origin
//! and a kilometre off it, in `f64` and in `Interval`.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom_core::{Interval, Tol};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

fn v<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

/// `n` distinct edge keys.
fn edges(n: usize) -> Vec<EdgeKey> {
    let mut keys = slotmap::SlotMap::<EdgeKey, ()>::with_key();
    (0..n).map(|_| keys.insert(())).collect()
}

/// The plane through `p` of unit normal `n`, its reference direction
/// off a direction no normal here lies along.
fn plane<T: Real>(p: Point3<T>, n: Vec3<T>) -> geom::Surface<T> {
    geom::Surface::Plane {
        origin: p,
        normal: n,
        u_ref: n.cross(v::<T>(1.0, 0.3, 0.7)).normalize(),
    }
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

/// The bound along `edge` with curvature vector `bend`, a line where it
/// bends nowhere.
fn bound<T: Real>(edge: EdgeKey, [x, y, z]: [f64; 3]) -> Bound<T> {
    Bound {
        edge,
        line: [x, y, z] == [0.0; 3],
        bend: v(x, y, z),
    }
}

/// A planar face's corner at `p` of outward unit `m`, from `d0` round to
/// `d1`, along the edges `ends`.
fn corner<T: Decide>(
    p: Point3<T>,
    m: Vec3<T>,
    (d0, d1): (Vec3<T>, Vec3<T>),
    ends: [Bound<T>; 2],
) -> TangentPiece<T> {
    let fan = TangentFan::Corner {
        d0,
        d1,
        bounds: ends,
        spur: false,
    };
    FaceSector::tangent_piece(plane(p, m), true, m, p, fan, band()).expect("a corner's piece")
}

/// A whole tangent plane at `p` of outward unit `m`.
fn whole<T: Decide>(p: Point3<T>, m: Vec3<T>) -> TangentPiece<T> {
    let fan = TangentFan::Inside {
        reach: T::from_f64(2.0),
        frame: [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)],
    };
    FaceSector::tangent_piece(plane(p, m), true, m, p, fan, band()).expect("a whole plane")
}

fn read<T: Decide>(p: Point3<T>, pieces: Vec<TangentPiece<T>>) -> TouchVerdict {
    rest::one_material(&Link::of_tangent(p, pieces).expect("a link"), band())
}

fn is_horn<T: Real>(piece: &TangentPiece<T>) -> bool {
    matches!(piece, TangentPiece::Horn(_))
}

/// **A cusp corner reads as its edges bend**: a planar corner whose two
/// edges leave along `+x`, in `z = 0` outward `+z`, so left of the ray
/// is `+y`. The edge back in standing left of the edge out is a horn,
/// right of it the whole turn but a horn; edges bending alike, or apart
/// only along the normal, are not parted. A bend parting the edges just
/// past the escalation, with the departures parting at first order by
/// nine tenths of the band's zero, is not parted either: the residue is
/// charged. Red where the side is read the other way round, read along
/// the normal, or the residue not charged.
fn a_cusp_corner_reads_as_its_edges_bend<T: Decide>() {
    let m = v::<T>(0.0, 0.0, 1.0);
    let d = v::<T>(2.0, 0.0, 0.0);
    let (zero, escalate) = (band().zero(), band().escalate());
    // The parting `(κ₁ − κ₀)·ŷ·l²/2` at the arm `l = 2`.
    let past = (escalate + 0.5 * zero) / 2.0;
    let rows: [(&str, Vec3<T>, [f64; 2], Option<bool>); 7] = [
        ("in left of out", d, [1.0, 2.0], Some(true)),
        ("in right of out", d, [2.0, 1.0], Some(false)),
        ("alike", d, [1.0, 1.0], None),
        ("just past the escalation", d, [0.0, past], Some(true)),
        (
            "just past it, parted by a residue",
            v(2.0, 0.9 * zero, 0.0),
            [0.0, past],
            None,
        ),
        (
            "just past it, parted by a residue the other way",
            v(2.0, -0.9 * zero, 0.0),
            [0.0, past],
            None,
        ),
        ("along the normal", d, [f64::NAN, f64::NAN], None),
    ];
    for p in apexes::<T>() {
        for (what, d1, [k0, k1], horn) in rows.clone() {
            let bends = if k0.is_nan() {
                [v(0.0, 0.0, 1.0), v(0.0, 0.0, 3.0)]
            } else {
                [v(0.0, k0, 0.0), v(0.0, k1, 0.0)]
            };
            let got = second_order::cusp(p, m, (d, d1), bends, band()).map(|c| match c {
                second_order::Cusp::Horn => true,
                second_order::Cusp::Whole => false,
            });
            assert_eq!(got, horn, "{what} at {p:?}");
        }
    }
}

#[test]
fn a_cusp_corner_reads_as_its_edges_bend_f64() {
    a_cusp_corner_reads_as_its_edges_bend::<f64>();
}

#[test]
fn a_cusp_corner_reads_as_its_edges_bend_interval() {
    a_cusp_corner_reads_as_its_edges_bend::<Interval>();
}

/// The three horns of a thorn along `+x` from `p`, one order down the
/// triangle of the line edge `L` at `0`, the arc `A` at `(0, 1, 0)` and
/// the arc `B` at `(0, 0, 1)`: the face in `z = 0` from `A` to `L`, the
/// face in `y = 0` from `L` to `B`, and a curved face through `B` and `A`
/// of outward normal `(0, 1, 1)/√2` there. `void` turns every normal
/// round and every face's loop with it: a needle of void. `keys` names
/// `L`, `A` and `B`.
fn thorn<T: Decide>(p: Point3<T>, keys: [EdgeKey; 3], void: bool) -> Vec<TangentPiece<T>> {
    let d = v::<T>(2.0, 0.0, 0.0);
    let [l, a, b] = [
        bound(keys[0], [0.0; 3]),
        bound(keys[1], [0.0, 1.0, 0.0]),
        bound(keys[2], [0.0, 0.0, 1.0]),
    ];
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let faces: [(Vec3<T>, [Bound<T>; 2]); 3] = [
        (v(0.0, 0.0, -1.0), [a, l]),
        (v(0.0, -1.0, 0.0), [l, b]),
        (v(0.0, s, s), [b, a]),
    ];
    faces
        .into_iter()
        .map(|(m, [e0, e1])| {
            let (m, ends) = if void { (-m, [e1, e0]) } else { (m, [e0, e1]) };
            let piece = corner(p, m, (d, d), ends);
            assert!(is_horn(&piece), "each face of the thorn a horn");
            piece
        })
        .collect()
}

/// **A ring of horns reads as the thorn it bounds**: a thorn of
/// material alone rests, and beside a plane whose material lies away
/// from it; inside that material it refuses. A needle of void refuses
/// alone, and rests inside a plane's material. A ring with a horn
/// missing, a ring of two horns, a triangle squashed flat, and two
/// thorns along one ray refuse; two along opposite rays rest. Red where
/// a thorn's side is read the other way round, the cone round it is not
/// read, a ring is closed that does not close, or two thorns are taken
/// apart that lie along one ray.
fn a_ring_of_horns_reads_as_its_thorn<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    let rest = |v: &TouchVerdict| matches!(v, TouchVerdict::Rest(_));
    for p in apexes::<T>() {
        let k = edges(6);
        let keys = [k[0], k[1], k[2]];
        // The plane `x = 0`, its material toward `−x` or toward `+x`.
        let away = || whole(p, v::<T>(1.0, 0.0, 0.0));
        let toward = || whole(p, v::<T>(-1.0, 0.0, 0.0));
        let with = |mut pieces: Vec<TangentPiece<T>>, more: TangentPiece<T>| {
            pieces.push(more);
            pieces
        };
        assert!(rest(&read(p, thorn(p, keys, false))), "a thorn at {p:?}");
        assert!(
            rest(&read(p, with(thorn(p, keys, false), away()))),
            "a thorn beside a plane at {p:?}"
        );
        assert_eq!(
            read(p, with(thorn(p, keys, false), toward())),
            unread,
            "a thorn inside a plane's material at {p:?}"
        );
        assert_eq!(read(p, thorn(p, keys, true)), unread, "a needle alone at {p:?}");
        assert!(
            rest(&read(p, with(thorn(p, keys, true), toward()))),
            "a needle inside a plane's material at {p:?}"
        );
        let mut short = thorn(p, keys, false);
        short.pop();
        assert_eq!(read(p, short), unread, "a horn missing at {p:?}");
        // Two horns on the line and the arc `A`, the lamina between them.
        let d = v::<T>(2.0, 0.0, 0.0);
        let (l, a) = (bound(k[0], [0.0, 0.0, 0.0]), bound(k[1], [0.0, 1.0, 0.0]));
        let lens = vec![
            corner(p, v(0.0, 0.0, -1.0), (d, d), [a, l]),
            corner(p, v(0.0, 0.0, 1.0), (d, d), [l, a]),
        ];
        assert_eq!(read(p, lens), unread, "a ring of two horns at {p:?}");
        // `B` on the line through `L` and `A`: the curved face's normal
        // then stands square to `z = 0`, and `L` lies on its line.
        let flat = {
            let mut pieces = thorn(p, keys, false);
            let b = bound(k[2], [0.0, 2.0, 0.0]);
            let a = bound(k[1], [0.0, 1.0, 0.0]);
            pieces[1] = corner(p, v(0.0, 0.0, 1.0), (d, d), [bound(k[0], [0.0, 0.0, 0.0]), b]);
            pieces[2] = corner(p, v(0.0, 0.0, -1.0), (d, d), [b, a]);
            pieces
        };
        assert_eq!(read(p, flat), unread, "a thorn squashed flat at {p:?}");
        // Its mirror across `y = 0` shares no edge and lies along the
        // same ray; along `−x` it is a thorn of its own.
        let mirrored = |dir: f64| -> Vec<TangentPiece<T>> {
            let d = v::<T>(2.0 * dir, 0.0, 0.0);
            let [l, a, b] = [
                bound(k[3], [0.0, 0.0, 0.0]),
                bound(k[4], [0.0, -1.0, 0.0]),
                bound(k[5], [0.0, 0.0, 1.0]),
            ];
            let s = core::f64::consts::FRAC_1_SQRT_2;
            let faces: [(Vec3<T>, [Bound<T>; 2]); 3] = [
                (v(0.0, 0.0, -1.0), [l, a]),
                (v(0.0, 1.0, 0.0), [b, l]),
                (v(0.0, -s, s), [a, b]),
            ];
            faces
                .into_iter()
                .map(|(m, [e0, e1])| {
                    let ends = if dir > 0.0 { [e0, e1] } else { [e1, e0] };
                    let piece = corner(p, m, (d, d), ends);
                    assert!(is_horn(&piece), "each face of the mirror a horn");
                    piece
                })
                .collect()
        };
        let mut pair = thorn(p, keys, false);
        pair.extend(mirrored(1.0));
        assert_eq!(read(p, pair), unread, "two thorns along one ray at {p:?}");
        let mut opposite = thorn(p, keys, false);
        opposite.extend(mirrored(-1.0));
        assert!(
            rest(&read(p, opposite)),
            "two thorns along opposite rays at {p:?}"
        );
    }
}

#[test]
fn a_ring_of_horns_reads_as_its_thorn_f64() {
    a_ring_of_horns_reads_as_its_thorn::<f64>();
}

#[test]
fn a_ring_of_horns_reads_as_its_thorn_interval() {
    a_ring_of_horns_reads_as_its_thorn::<Interval>();
}

/// Two wedges along `±x` from `p`, each a curved edge's two faces, one
/// order down at `first` and `second` across the ray: over `first`, a
/// face in `y = 0` outward `+y` reaching up and one in `z = 0` outward
/// `+z` reaching toward `+y`; over `second`, a face in `y = 0` outward
/// `−y` reaching down and one in `z = 0` outward `−z` reaching toward
/// `−y`. At first order the four half-planes pair up on the ray, the
/// material the quarters `y < 0 < z` and `z < 0 < y`; one order down the
/// wedges stand apart where `first` lies toward `+y` of `second`.
fn two_wedges<T: Decide>(p: Point3<T>, first: f64, second: f64) -> Vec<TangentPiece<T>> {
    let k = edges(2);
    let (e1, e2) = (
        bound(k[0], [0.0, first, 0.0]),
        bound(k[1], [0.0, second, 0.0]),
    );
    let x = v::<T>(2.0, 0.0, 0.0);
    vec![
        corner(p, v(0.0, 1.0, 0.0), (-x, x), [e1, e1]),
        corner(p, v(0.0, 0.0, 1.0), (x, -x), [e1, e1]),
        corner(p, v(0.0, -1.0, 0.0), (-x, x), [e2, e2]),
        corner(p, v(0.0, 0.0, -1.0), (x, -x), [e2, e2]),
    ]
}

/// A wedge above `z = 0` along `±x` from `p`, an arc's two faces, one
/// in `y = 0` outward `+y` and one in `y = −z` outward `(0, −1, −1)/√2`,
/// the arc one order down at `(0, 0, up)`; and the whole plane `z = 0`
/// outward `+z`, its material below.
fn wedge_over_a_plane<T: Decide>(p: Point3<T>, up: f64) -> Vec<TangentPiece<T>> {
    let k = edges(1);
    let e = bound(k[0], [0.0, 0.0, up]);
    let x = v::<T>(2.0, 0.0, 0.0);
    let s = core::f64::consts::FRAC_1_SQRT_2;
    vec![
        corner(p, v(0.0, 1.0, 0.0), (-x, x), [e, e]),
        corner(p, v(0.0, -s, -s), (x, -x), [e, e]),
        whole(p, v(0.0, 0.0, 1.0)),
    ]
}

/// A cusp whose face is the whole turn but a horn, at the mouth of a
/// groove along `+x`: the face in `z = 0` outward `+z`, its edge out at
/// `(0, 1, 0)` one order down and its edge back in at `(0, −1, 0)`; the
/// groove's two walls below it, a quarter turn each from the ray down
/// and away, from the edge out toward `(0, 1, −1)` (outward
/// `(0, −1, −1)/√2`) and from the edge in toward `(0, −1, −1)` (outward
/// `(0, 1, −1)/√2`); and the groove's end in `x = 0` between the walls'
/// line edges, outward `+x`. The material lies below the face, either
/// side of the groove.
fn a_grooves_mouth<T: Decide>(p: Point3<T>) -> Vec<TangentPiece<T>> {
    let k = edges(4);
    let (out, back) = (
        bound(k[0], [0.0, 1.0, 0.0]),
        bound(k[1], [0.0, -1.0, 0.0]),
    );
    let (right_edge, left_edge) = (bound(k[2], [0.0; 3]), bound(k[3], [0.0; 3]));
    let x = v::<T>(2.0, 0.0, 0.0);
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let right = v::<T>(0.0, 2.0 * s, -2.0 * s);
    let left = v::<T>(0.0, -2.0 * s, -2.0 * s);
    let mouth = corner(p, v(0.0, 0.0, 1.0), (x, x), [out, back]);
    assert!(!is_horn(&mouth), "the mouth is the whole turn but a horn");
    vec![
        mouth,
        corner(p, v(0.0, -s, -s), (right, x), [right_edge, out]),
        corner(p, v(0.0, s, -s), (x, left), [back, left_edge]),
        corner(p, v(1.0, 0.0, 0.0), (left, right), [left_edge, right_edge]),
    ]
}

/// **Sheets along one ray part, or not, by their edges' bends**: two
/// wedges meeting on a ray at first order rest where their edges stand
/// apart one order down, and refuse where the wedges overlap or the
/// edges stand at one point; a wedge whose arc lies in a plane at first
/// order rests where the arc bends off the plane away from it, and
/// refuses where it bends through it; a groove's mouth, a cusp whose
/// face is the whole turn but a horn, rests. Red where two sheets are
/// read apart that meet, a half-line's direction or anchor is read the
/// other way, or a whole cusp is read as a horn.
fn sheets_along_one_ray_part_by_their_bends<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    let rest = |v: &TouchVerdict| matches!(v, TouchVerdict::Rest(_));
    for p in apexes::<T>() {
        assert!(
            rest(&read(p, two_wedges(p, -1.0, -3.0))),
            "two wedges apart at {p:?}"
        );
        assert_eq!(
            read(p, two_wedges(p, -3.0, -1.0)),
            unread,
            "two wedges overlapping at {p:?}"
        );
        assert_eq!(
            read(p, two_wedges(p, -1.0, -1.0)),
            unread,
            "two wedges at one point at {p:?}"
        );
        assert!(
            rest(&read(p, wedge_over_a_plane(p, 1.0))),
            "an arc bending off a plane at {p:?}"
        );
        assert_eq!(
            read(p, wedge_over_a_plane(p, -1.0)),
            unread,
            "an arc bending through a plane at {p:?}"
        );
        assert!(rest(&read(p, a_grooves_mouth(p))), "a groove's mouth at {p:?}");
    }
}

#[test]
fn sheets_along_one_ray_part_by_their_bends_f64() {
    sheets_along_one_ray_part_by_their_bends::<f64>();
}

#[test]
fn sheets_along_one_ray_part_by_their_bends_interval() {
    sheets_along_one_ray_part_by_their_bends::<Interval>();
}

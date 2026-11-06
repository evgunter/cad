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
    FaceSector::tangent(plane(p, m), true, m, p, fan, band()).expect("a corner's piece")
}

/// A whole tangent plane at `p` of outward unit `m`.
fn whole<T: Decide>(p: Point3<T>, m: Vec3<T>) -> TangentPiece<T> {
    let fan = TangentFan::Inside {
        reach: T::from_f64(2.0),
        frame: [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)],
    };
    FaceSector::tangent(plane(p, m), true, m, p, fan, band()).expect("a whole plane")
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
    type Row<T> = (&'static str, Vec3<T>, [f64; 2], Option<bool>);
    let rows: [Row<T>; 7] = [
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
        for (what, d1, [k0, k1], horn) in rows {
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
        assert_eq!(
            read(p, thorn(p, keys, true)),
            unread,
            "a needle alone at {p:?}"
        );
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
            pieces[1] = corner(
                p,
                v(0.0, 0.0, 1.0),
                (d, d),
                [bound(k[0], [0.0, 0.0, 0.0]), b],
            );
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

/// `(a, b, c)` turned a third of a turn about `(1, 1, 1)`, to `(c, a, b)`:
/// the frame the whole-plane rows are stated in. Their plane spans the
/// apex's `y` and `z` there. Spanning its `x` and `y` (a kilometre and
/// two off), the plane's own cut rays read against its own wedges, on
/// `Interval` at the finest band, in band (`link_wedge`), at first order,
/// before any reading one order down.
fn turned<T: Real>(a: f64, b: f64, c: f64) -> Vec3<T> {
    v(c, a, b)
}

/// A wedge along `±y` from `p` off the plane `x = 0`, an arc's two
/// faces, one in `z = 0` outward `+z` and one in `z = −x` outward
/// `(−1, 0, −1)/√2`, the arc one order down at `(up, 0, 0)`; and the
/// whole plane `x = 0` outward `+x`, its material behind it
/// ([`turned`]).
fn wedge_over_a_plane<T: Decide>(p: Point3<T>, up: f64) -> Vec<TangentPiece<T>> {
    let k = edges(1);
    let e = Bound {
        edge: k[0],
        line: up == 0.0,
        bend: turned(0.0, 0.0, up),
    };
    let y = turned::<T>(2.0, 0.0, 0.0);
    let s = core::f64::consts::FRAC_1_SQRT_2;
    vec![
        corner(p, turned(0.0, 1.0, 0.0), (-y, y), [e, e]),
        corner(p, turned(0.0, -s, -s), (y, -y), [e, e]),
        whole(p, turned(0.0, 0.0, 1.0)),
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
    let (out, back) = (bound(k[0], [0.0, 1.0, 0.0]), bound(k[1], [0.0, -1.0, 0.0]));
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
        assert!(
            rest(&read(p, a_grooves_mouth(p))),
            "a groove's mouth at {p:?}"
        );
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

/// A horn at `p` of outward unit `m` along `ends`, held as built: what
/// [`FaceSector::tangent`] keeps of a cusp corner it reads as one.
fn horn<T: Real>(p: Point3<T>, m: Vec3<T>, ends: [(Vec3<T>, Bound<T>); 2]) -> TangentPiece<T> {
    TangentPiece::Horn(Horn {
        outward: m,
        apex: p,
        ends,
    })
}

/// A thin thorn along `+x` from `p`: the line `L` at `0`, the arc `A`
/// at `(0, 1, 0)` and the arc `B` at `b_at` one order down, `B` leaving
/// the tip tilted `lift` toward `+z` at the arm `2`; the faces in
/// `z = 0` from `A` to `L` and in `y = 0` from `L` to `B`, and one
/// through `B` and `A`. The face from `L` to `B` has its vertex at
/// `p + tied`, a key the tie holds into `p`.
fn thin_thorn<T: Decide>(
    p: Point3<T>,
    b_at: [f64; 3],
    lift: f64,
    tied: Vec3<T>,
) -> Vec<TangentPiece<T>> {
    let k = edges(3);
    let d = v::<T>(2.0, 0.0, 0.0);
    let (l, a, b) = (
        bound(k[0], [0.0; 3]),
        bound(k[1], [0.0, 1.0, 0.0]),
        bound(k[2], b_at),
    );
    let db = v::<T>(2.0, 0.0, lift);
    let n = Vec3::new(0.0, b_at[2], 1.0).normalize();
    vec![
        horn(p, v(0.0, 0.0, -1.0), [(d, a), (d, l)]),
        horn(p + tied, v(0.0, -1.0, 0.0), [(d, l), (db, b)]),
        horn(p, v(0.0, n.y, n.z), [(db, b), (d, a)]),
    ]
}

/// **A thorn is read with its first order charged**: a thorn along
/// `+x` thin enough that its curved face's edge `B` stands just past the
/// escalation off the face in `z = 0` and its line `L` as far off the
/// curved face, one order down. It rests; with `B` leaving the tip
/// tilted off that face by nine tenths of the band's zero at the arm it
/// refuses, the tilt charged; with `B` leaving it by twice the
/// escalation, so that `B`'s edge leaves the face's plane at first
/// order, it refuses however far `B` bends. A ring of four whose third
/// corner lies on the line of the first two refuses. Red where a
/// vertex's first-order residue is not charged, an edge off the face's
/// plane is read as one of the picture, or a vertex on a side's line is
/// passed.
fn a_thorn_is_read_with_its_first_order_charged<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    let (zero, escalate) = (band().zero(), band().escalate());
    // `B` stands `β` across: at the arm `l = 2` its parting is `2β`.
    let beta = (escalate + 0.5 * zero) / 2.0;
    let thin = |p: Point3<T>, b_at: [f64; 3], lift: f64| thin_thorn(p, b_at, lift, Vec3::zero());
    for p in apexes::<T>() {
        assert!(
            matches!(
                read(p, thin(p, [0.0, 0.0, beta], 0.0)),
                TouchVerdict::Rest(_)
            ),
            "a thin thorn at {p:?}"
        );
        assert_eq!(
            read(p, thin(p, [0.0, 0.0, beta], 0.9 * zero)),
            unread,
            "a thin thorn, its edge tilted at {p:?}"
        );
        assert_eq!(
            read(p, thin(p, [0.0, 0.0, 1.0], 4.0 * escalate)),
            unread,
            "a thorn whose edge leaves the face's plane at {p:?}"
        );
        // `M` on the line through `A` and `L`, the face from `M` to `A`
        // lying on the plane of the one from `A` to `L`.
        let k = edges(4);
        let d = v::<T>(2.0, 0.0, 0.0);
        let (m, a, l, b) = (
            bound(k[0], [0.0, 2.0, 0.0]),
            bound(k[1], [0.0, 1.0, 0.0]),
            bound(k[2], [0.0; 3]),
            bound(k[3], [0.0, 0.0, 1.0]),
        );
        let n = Vec3::new(0.0, 1.0, 2.0).normalize();
        let four = vec![
            horn(p, v(0.0, 0.0, -1.0), [(d, m), (d, a)]),
            horn(p, v(0.0, 0.0, -1.0), [(d, a), (d, l)]),
            horn(p, v(0.0, -1.0, 0.0), [(d, l), (d, b)]),
            horn(p, v(0.0, n.y, n.z), [(d, b), (d, m)]),
        ];
        assert_eq!(read(p, four), unread, "a corner on a side's line at {p:?}");
    }
}

#[test]
fn a_thorn_is_read_with_its_first_order_charged_f64() {
    a_thorn_is_read_with_its_first_order_charged::<f64>();
}

#[test]
fn a_thorn_is_read_with_its_first_order_charged_interval() {
    a_thorn_is_read_with_its_first_order_charged::<Interval>();
}

/// [`two_wedges`], the first wedge's edge also `rise` above the plane
/// `z = 0` one order down, and its face in `z = 0` tilted about `x` to
/// the normal `(0, −tilt, 1)`.
fn two_wedges_tilted<T: Decide>(p: Point3<T>, rise: f64, tilt: f64) -> Vec<TangentPiece<T>> {
    let k = edges(2);
    let (e1, e2) = (
        bound(k[0], [0.0, -1.0, rise]),
        bound(k[1], [0.0, -3.0, 0.0]),
    );
    let x = v::<T>(2.0, 0.0, 0.0);
    let n = Vec3::new(0.0, -tilt, 1.0).normalize();
    vec![
        corner(p, v(0.0, 1.0, 0.0), (-x, x), [e1, e1]),
        corner(p, v(0.0, n.y, n.z), (x, -x), [e1, e1]),
        corner(p, v(0.0, -1.0, 0.0), (-x, x), [e2, e2]),
        corner(p, v(0.0, 0.0, -1.0), (x, -x), [e2, e2]),
    ]
}

/// **A sheet along another's line is read with its lean charged**: two
/// wedges whose faces in `z = 0` run on along one line at first order,
/// the first's edge just past the escalation above the other's line one
/// order down. They rest; with the first's face leaning toward the
/// other's line by nine tenths of the band's zero at the arm, they
/// refuse. Red where a sheet's lean along another's line is not
/// charged.
fn a_sheet_along_anothers_line_is_read_with_its_lean_charged<T: Decide>() {
    let (zero, escalate) = (band().zero(), band().escalate());
    let rise = (escalate + 0.5 * zero) / 2.0;
    for p in apexes::<T>() {
        assert!(
            matches!(
                read(p, two_wedges_tilted(p, rise, 0.0)),
                TouchVerdict::Rest(_)
            ),
            "two wedges, one above the other's line, at {p:?}"
        );
        assert_eq!(
            read(p, two_wedges_tilted(p, rise, 0.45 * zero)),
            TouchVerdict::Unread(TouchUnread::Unreadable),
            "two wedges, one leaning toward the other's line, at {p:?}"
        );
    }
}

#[test]
fn a_sheet_along_anothers_line_is_read_with_its_lean_charged_f64() {
    a_sheet_along_anothers_line_is_read_with_its_lean_charged::<f64>();
}

#[test]
fn a_sheet_along_anothers_line_is_read_with_its_lean_charged_interval() {
    a_sheet_along_anothers_line_is_read_with_its_lean_charged::<Interval>();
}

/// [`wedge_over_a_plane`], its arc's departure along `+y` leaning
/// `lean` toward `−x` at the arm, and the plane `x = 0` a ball's tangent
/// plane where `curved`.
fn wedge_over_a_sheet<T: Decide>(
    p: Point3<T>,
    up: f64,
    lean: f64,
    curved: bool,
) -> Vec<TangentPiece<T>> {
    let k = edges(1);
    let e = Bound {
        edge: k[0],
        line: up == 0.0,
        bend: turned(0.0, 0.0, up),
    };
    let (x, out) = (turned::<T>(2.0, 0.0, 0.0), turned::<T>(2.0, 0.0, -lean));
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let n = turned::<T>(0.0, 0.0, 1.0);
    let sheet = if curved {
        let fan = TangentFan::Inside {
            reach: T::from_f64(2.0),
            frame: [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)],
        };
        let ball = geom::Surface::Sphere {
            center: p - n * T::from_f64(2.0),
            radius: T::from_f64(2.0),
            axis: n,
            u_ref: turned(1.0, 0.0, 0.0),
        };
        FaceSector::tangent(ball, true, n, p, fan, band()).expect("a ball's plane")
    } else {
        whole(p, n)
    };
    vec![
        corner(p, turned(0.0, 1.0, 0.0), (-x, out), [e, e]),
        corner(p, turned(0.0, -s, -s), (out, -x), [e, e]),
        sheet,
    ]
}

/// A fin along `±x` from `p`: two faces in `z = 0` reaching toward `+y`
/// from one curved edge, outward `+z` and `−z`; and the whole plane
/// `y = 0` outward `−y`, the edge one order down at `(0, 1, 0)` off it.
fn a_fin_on_a_plane<T: Decide>(p: Point3<T>) -> Vec<TangentPiece<T>> {
    let k = edges(1);
    let e = bound(k[0], [0.0, 1.0, 0.0]);
    let x = v::<T>(2.0, 0.0, 0.0);
    vec![
        corner(p, v(0.0, 0.0, 1.0), (x, -x), [e, e]),
        corner(p, v(0.0, 0.0, -1.0), (-x, x), [e, e]),
        whole(p, v(0.0, -1.0, 0.0)),
    ]
}

/// **What one order down leaves unread stays unread**: an arc bending
/// off a plane by just past the escalation rests, and refuses where its
/// departure leans toward the plane by seven tenths of the band's zero,
/// the lean charged; an arc over a ball's tangent plane refuses, the
/// ball's line in the cut not read; a fin, an edge's two faces running
/// on along one line one way, refuses as unread, not as faces on each
/// other. Red where an anchor's first-order residue is not charged, a
/// curved sheet is read as its plane, or a fold is passed. The lean's
/// charge is witnessed near the origin: a kilometre off, on `Interval` at
/// the finest band, the leaning arc refuses with or without it.
fn what_one_order_down_leaves_unread_stays_unread<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    let (zero, escalate) = (band().zero(), band().escalate());
    let up = (escalate + 0.5 * zero) / 2.0;
    for p in apexes::<T>() {
        assert!(
            matches!(
                read(p, wedge_over_a_sheet(p, up, 0.0, false)),
                TouchVerdict::Rest(_)
            ),
            "an arc just past the escalation off a plane at {p:?}"
        );
        assert_eq!(
            read(p, wedge_over_a_sheet(p, up, 0.7 * zero, false)),
            unread,
            "the arc leaning toward the plane at {p:?}"
        );
        assert_eq!(
            read(p, wedge_over_a_sheet(p, 1.0, 0.0, true)),
            unread,
            "an arc over a ball's tangent plane at {p:?}"
        );
        assert_eq!(read(p, a_fin_on_a_plane(p)), unread, "a fin at {p:?}");
    }
}

#[test]
fn what_one_order_down_leaves_unread_stays_unread_f64() {
    what_one_order_down_leaves_unread_stays_unread::<f64>();
}

#[test]
fn what_one_order_down_leaves_unread_stays_unread_interval() {
    what_one_order_down_leaves_unread_stays_unread::<Interval>();
}

/// A ring of horns along `+x` through the cut points `pts` (in `(y, z)`),
/// each side's face of outward normal the left normal of its directed
/// side, built through `corner` so each must read as a horn; `None`
/// where one does not.
fn ring<T: Decide>(p: Point3<T>, pts: &[[f64; 2]]) -> Option<Vec<TangentPiece<T>>> {
    let k = edges(pts.len());
    let d = v::<T>(2.0, 0.0, 0.0);
    let b = |i: usize| bound::<T>(k[i], [0.0, pts[i][0], pts[i][1]]);
    let mut out = Vec::new();
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        let (dy, dz) = (pts[j][0] - pts[i][0], pts[j][1] - pts[i][1]);
        let len = (dy * dy + dz * dz).sqrt();
        let m = v::<T>(0.0, -dz / len, dy / len);
        let piece = corner(p, m, (d, d), [b(i), b(j)]);
        if !is_horn(&piece) {
            return None;
        }
        out.push(piece);
    }
    Some(out)
}

/// **A ring of horns is read only where it bounds a convex thorn**:
/// clockwise in `(y, z)`, so its faces' normals point out, a convex
/// quad and a convex pentagon rest; the pentagon the other way round is
/// a needle of void alone and refuses. A dart (simple, not convex), a
/// bowtie (a quad crossing itself) and a pentagram (the pentagon two
/// steps at a time), each either way round, refuse: their faces cross.
/// So does the fixture thorn with any one face's normal turned round.
/// Red where a vertex's side of one face is not held to the side read
/// for the others.
fn a_ring_of_horns_is_read_only_as_a_convex_thorn<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    for p in apexes::<T>() {
        // Clockwise in (y, z): left normals point out, a thorn of material.
        let cw = |pts: &[[f64; 2]]| pts.iter().rev().copied().collect::<Vec<_>>();
        let convex = cw(&[[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 4.0]]);
        let got = read(p, ring(p, &convex).expect("horns"));
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "convex quad {got:?} at {p:?}"
        );
        let pent: Vec<[f64; 2]> = (0..5)
            .map(|i| {
                let a = -(i as f64) * core::f64::consts::TAU / 5.0;
                [2.0 + a.cos(), 2.0 + a.sin()]
            })
            .collect();
        let got = read(p, ring(p, &pent).expect("horns"));
        assert!(
            matches!(got, TouchVerdict::Rest(_)),
            "convex pentagon {got:?} at {p:?}"
        );
        // The same pentagon counterclockwise: a needle of void alone.
        let ccw: Vec<[f64; 2]> = pent.iter().rev().copied().collect();
        assert_eq!(
            read(p, ring(p, &ccw).expect("horns")),
            unread,
            "needle at {p:?}"
        );
        // Non-convex (a dart), both senses.
        let dart = cw(&[[0.0, 0.0], [4.0, 0.0], [1.0, 1.0], [0.0, 4.0]]);
        assert_eq!(
            read(p, ring(p, &dart).expect("horns")),
            unread,
            "dart at {p:?}"
        );
        let dart = cw(&dart);
        assert_eq!(
            read(p, ring(p, &dart).expect("horns")),
            unread,
            "dart ccw at {p:?}"
        );
        // Self-intersecting: a bowtie, both senses.
        let bow = [[0.0, 0.0], [2.0, 2.0], [2.0, 0.0], [0.0, 2.0]];
        assert_eq!(
            read(p, ring(p, &bow).expect("horns")),
            unread,
            "bowtie at {p:?}"
        );
        assert_eq!(
            read(p, ring(p, &cw(&bow)).expect("horns")),
            unread,
            "bowtie r at {p:?}"
        );
        // A pentagram: the pentagon visited two steps at a time.
        let star: Vec<[f64; 2]> = (0..5).map(|i| pent[(2 * i) % 5]).collect();
        assert_eq!(
            read(p, ring(p, &star).expect("horns")),
            unread,
            "star at {p:?}"
        );
        assert_eq!(
            read(p, ring(p, &cw(&star)).expect("horns")),
            unread,
            "star r at {p:?}"
        );
        // Mixed normals: the fixture thorn with one face's normal turned
        // round, its ends kept (a Horn held as built).
        let k = edges(3);
        let d = v::<T>(2.0, 0.0, 0.0);
        let (l, a, b) = (
            bound(k[0], [0.0; 3]),
            bound(k[1], [0.0, 1.0, 0.0]),
            bound(k[2], [0.0, 0.0, 1.0]),
        );
        let s = core::f64::consts::FRAC_1_SQRT_2;
        for flip in 0..3 {
            let ms = [v::<T>(0.0, 0.0, -1.0), v(0.0, -1.0, 0.0), v(0.0, s, s)];
            let ends = [[a, l], [l, b], [b, a]];
            let pieces = (0..3)
                .map(|i| {
                    let m = if i == flip { -ms[i] } else { ms[i] };
                    horn(p, m, [(d, ends[i][0]), (d, ends[i][1])])
                })
                .collect();
            assert_eq!(
                read(p, pieces),
                unread,
                "mixed normals, face {flip} at {p:?}"
            );
        }
    }
}

#[test]
fn a_ring_of_horns_is_read_only_as_a_convex_thorn_f64() {
    a_ring_of_horns_is_read_only_as_a_convex_thorn::<f64>();
}

#[test]
fn a_ring_of_horns_is_read_only_as_a_convex_thorn_interval() {
    a_ring_of_horns_is_read_only_as_a_convex_thorn::<Interval>();
}

/// **A cusp is parted only past the escalation, whichever way the face
/// is turned**: the edge back in standing just past the escalation left
/// of the edge out is a horn, just past it right the whole turn, and
/// just inside it either way, or within the band's zero, not parted;
/// the same with the face turned over (its normal and loop reversed).
/// Red where the escalation is read as the zero, or the turned face is
/// read the other way round.
fn a_cusp_is_parted_only_past_the_escalation<T: Decide>() {
    let m = v::<T>(0.0, 0.0, 1.0);
    let d = v::<T>(2.0, 0.0, 0.0);
    let (zero, escalate) = (band().zero(), band().escalate());
    for p in apexes::<T>() {
        for (k1, want) in [
            ((escalate - 0.5 * zero) / 2.0, None),
            ((escalate + 0.5 * zero) / 2.0, Some(true)),
            (-(escalate + 0.5 * zero) / 2.0, Some(false)),
            (-(escalate - 0.5 * zero) / 2.0, None),
            (0.4 * zero, None),
        ] {
            let got = second_order::cusp(p, m, (d, d), [v(0.0, 0.0, 0.0), v(0.0, k1, 0.0)], band())
                .map(|c| matches!(c, second_order::Cusp::Horn));
            assert_eq!(got, want, "k1 {k1} at {p:?}");
            let got =
                second_order::cusp(p, -m, (d, d), [v(0.0, k1, 0.0), v(0.0, 0.0, 0.0)], band())
                    .map(|c| matches!(c, second_order::Cusp::Horn));
            assert_eq!(got, want, "turned over, k1 {k1} at {p:?}");
        }
    }
}

#[test]
fn a_cusp_is_parted_only_past_the_escalation_f64() {
    a_cusp_is_parted_only_past_the_escalation::<f64>();
}

#[test]
fn a_cusp_is_parted_only_past_the_escalation_interval() {
    a_cusp_is_parted_only_past_the_escalation::<Interval>();
}

/// **A thorn's horns are read over the tie that holds their vertices**:
/// the thin thorn of [`a_thorn_is_read_with_its_first_order_charged`],
/// its bend just past the escalation, rests with every horn's vertex at
/// `p`, alone and beside a plane whose material lies away from it. With
/// one horn's vertex on another key, tied off `p` by three quarters of
/// the band's zero, the cone is read over that tie and the bend no
/// longer clears it: it refuses, alone and beside the plane. Red where a
/// horn's vertex is left out of the cone's tie.
fn a_thorn_is_read_over_the_tie_of_its_horns<T: Decide>() {
    let unread = TouchVerdict::Unread(TouchUnread::Unreadable);
    let (zero, escalate) = (band().zero(), band().escalate());
    let beta = (escalate + 0.5 * zero) / 2.0;
    let rest = |v: &TouchVerdict| matches!(v, TouchVerdict::Rest(_));
    for p in apexes::<T>() {
        let thin = |tied: Vec3<T>, beside: bool| {
            let mut pieces = thin_thorn(p, [0.0, 0.0, beta], 0.0, tied);
            if beside {
                pieces.push(whole(p, v(1.0, 0.0, 0.0)));
            }
            read(p, pieces)
        };
        let off = v::<T>(0.0, 0.0, 0.75 * zero);
        for beside in [false, true] {
            assert!(
                rest(&thin(Vec3::zero(), beside)),
                "every vertex at the point, beside a plane {beside}, at {p:?}"
            );
            assert_eq!(
                thin(off, beside),
                unread,
                "a vertex tied off the point, beside a plane {beside}, at {p:?}"
            );
        }
    }
}

#[test]
fn a_thorn_is_read_over_the_tie_of_its_horns_f64() {
    a_thorn_is_read_over_the_tie_of_its_horns::<f64>();
}

#[test]
fn a_thorn_is_read_over_the_tie_of_its_horns_interval() {
    a_thorn_is_read_over_the_tie_of_its_horns::<Interval>();
}

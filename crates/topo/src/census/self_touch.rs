//! **A solid touching itself at a point is read over its whole link**
//! ([`super::sweep_self_touch_points`], [`super::rest::one_material`]):
//! the link bounds one material, its winding two consecutive values, or
//! the census refuses [`ValidationError::BoundaryCrossesItself`].
//!
//! Bare links first, each a cone or a disc at the origin (the windings
//! are given outside to inside), then the bodies: two faces of one solid
//! crossing on vertex-on-face touches, its control with the crossing
//! lifted into pierces, the pit, and two pits beside a needle.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::tests::{At, synthetic};
use super::*;
use crate::euler::{FaceSurface, MefSite, MevCreated, MevSite};
use crate::test_support_fixtures::{brick, describe_as_intersections, line, plane};
use crate::test_support_meeting::{Pose, posed_pyramid};
use crate::{BooleanResult, subtract, union, validate_pseudomanifold};
use geom_core::{Tol, Vec3};

type P = Point3<f64>;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A cone at the origin over `ring` (corners around its axis), material
/// inside it or outside it.
fn cone(ring: &[[f64; 3]], material_inside: bool) -> Link<f64> {
    let o = P::origin();
    let mut verts = vec![o];
    verts.extend(ring.iter().map(|&[x, y, z]| P::new(x, y, z)));
    let n = ring.len();
    let axis = verts[1..].iter().fold(Vec3::zero(), |s, q| s + (*q - o));
    let faces: Vec<_> = (0..n)
        .map(|k| {
            let (a, b) = (1 + k, 1 + (k + 1) % n);
            let m = (verts[a] - o).cross(verts[b] - o).normalize();
            // The outward normal points away from the material.
            if (m.dot(axis) < 0.0) == material_inside {
                (m, vec![vec![0, a, b]])
            } else {
                (-m, vec![vec![0, b, a]])
            }
        })
        .collect();
    synthetic(o, At::Vertex(0), verts, &faces, &[], band()).expect("a cone")
}

/// A square of half-width `r` at height `z` about the z axis.
fn square(r: f64, z: f64) -> [[f64; 3]; 4] {
    [[r, r, z], [-r, r, z], [-r, -r, z], [r, -r, z]]
}

/// The disc `[−2, 2]²` in `z = 0` through the origin, outward `±z`.
fn floor(up: bool) -> Link<f64> {
    let mut verts = vec![
        P::new(-2.0, -2.0, 0.0),
        P::new(2.0, -2.0, 0.0),
        P::new(2.0, 2.0, 0.0),
        P::new(-2.0, 2.0, 0.0),
    ];
    let m = Vec3::new(0.0, 0.0, if up { 1.0 } else { -1.0 });
    if !up {
        verts.reverse();
    }
    synthetic(
        P::origin(),
        At::Face,
        verts,
        &[(m, vec![vec![0, 1, 2, 3]])],
        &[],
        band(),
    )
    .expect("a floor")
}

/// One link of every sector of `links`, all at the origin.
fn whole(links: Vec<Link<f64>>) -> Link<f64> {
    Link::of(
        P::origin(),
        links.into_iter().flat_map(|l| l.sectors).collect(),
    )
    .unwrap()
}

/// A link's sectors with their sides swapped: its void read as material.
fn flipped(link: Link<f64>) -> Link<f64> {
    let sectors = link
        .sectors
        .into_iter()
        .map(|s| Sector {
            outward: -s.outward,
            wedges: s.wedges.iter().rev().map(|&(a, b)| (b, a)).collect(),
            ..s
        })
        .collect();
    Link::of(P::origin(), sectors).unwrap()
}

/// **Each link reads by its winding.** A cavity's corner on a face from
/// inside (2 in the material, 1 in the pit and below), a convex corner
/// resting on one from outside (1, 0, 1), and a pinch (1, 0, 1) bound
/// one material; a spike sunk into the face's material (2, 1, 0), a
/// cavity's corner on a face from outside its material (2, 1, 0), and a
/// sheet through a corner's faces do not.
#[test]
fn a_links_winding_takes_two_values_or_the_link_crosses_itself() {
    let up = || square(0.5, 1.0);
    let down = || square(0.5, -1.0);
    let corner = || {
        let at = |x: f64, y: f64, z: f64| P::new(x, y, z);
        let verts = vec![
            at(0.0, 0.0, 0.0),
            at(1.0, 0.0, 0.0),
            at(0.0, 1.0, 0.0),
            at(0.0, 0.0, 1.0),
        ];
        // The octant's corner: faces x = 0, y = 0, z = 0, material inside.
        let faces = [
            (Vec3::new(-1.0, 0.0, 0.0), vec![vec![0, 3, 2]]),
            (Vec3::new(0.0, -1.0, 0.0), vec![vec![0, 1, 3]]),
            (Vec3::new(0.0, 0.0, -1.0), vec![vec![0, 2, 1]]),
        ];
        synthetic(verts[0], At::Vertex(0), verts, &faces, &[], band()).unwrap()
    };
    // A disc through the corner across its diagonal, in `x = y`.
    let sheet = || {
        let verts = vec![
            P::new(-1.0, -1.0, -1.0),
            P::new(1.0, 1.0, -1.0),
            P::new(1.0, 1.0, 1.0),
            P::new(-1.0, -1.0, 1.0),
        ];
        let m = Vec3::new(1.0, -1.0, 0.0).normalize();
        synthetic(
            P::origin(),
            At::Face,
            verts,
            &[(m, vec![vec![0, 1, 2, 3]])],
            &[],
            band(),
        )
        .unwrap()
    };
    let rows: [(&str, Link<f64>, bool); 6] = [
        (
            "a cavity's corner",
            whole(vec![floor(false), cone(&up(), false)]),
            true,
        ),
        (
            "a convex corner",
            whole(vec![floor(true), cone(&up(), true)]),
            true,
        ),
        (
            "a pinch",
            whole(vec![cone(&up(), true), cone(&down(), true)]),
            true,
        ),
        (
            "a sunk spike",
            whole(vec![floor(false), cone(&up(), true)]),
            false,
        ),
        (
            "a cavity outside the face's material",
            whole(vec![floor(true), cone(&up(), false)]),
            false,
        ),
        (
            "a sheet through a corner",
            whole(vec![corner(), sheet()]),
            false,
        ),
    ];
    for (what, link, rests) in rows {
        let got = rest::one_material(&link, band());
        if rests {
            assert!(got.is_rest(), "{what}: {got:?}");
        } else {
            assert_eq!(got, TouchVerdict::Crossing, "{what}");
        }
    }
}

/// **Two pits sharing an apex and a needle standing in one of them bound
/// one material**: the winding is 2 outside the pits, 1 in pit `A`, 2 in
/// the needle `C` and 1 in pit `B`. Read cell against cell — the
/// materials on opposite sides, or the voids — the pair `(B, C)` is
/// refused both ways, though `B` and `C` border no common region: `C`
/// lies inside `A`, so outside `B`, and their materials overlap; `B`
/// lies outside `C`, and their voids overlap too.
#[test]
fn two_pits_and_a_needle_bound_one_material_where_a_pairwise_reading_refuses() {
    let pit_a = || cone(&square(1.0, 1.0), false);
    let pit_b = || cone(&square(1.0, -1.0), false);
    let needle = || cone(&square(0.2, 1.0), true);
    let got = rest::one_material(&whole(vec![pit_a(), pit_b(), needle()]), band());
    assert!(got.is_rest(), "the needle in a pit: {got:?}");
    assert_eq!(
        rest::judge(&pit_b(), &needle(), band()),
        TouchVerdict::Crossing,
        "(B, C): their materials overlap"
    );
    assert_eq!(
        rest::judge(&flipped(pit_b()), &flipped(needle()), band()),
        TouchVerdict::Crossing,
        "(B, C): their voids overlap"
    );
}

/// **A doubled sheet refuses as degenerate**: a floor and its copy. A
/// link whose sectors do not pair up at their bounds — a cone and one
/// face of another — is not read.
#[test]
fn a_doubled_sheet_or_an_open_link_is_not_read() {
    let got = rest::one_material(&whole(vec![floor(true), floor(true)]), band());
    assert_eq!(
        got,
        TouchVerdict::Unread(TouchUnread::Degenerate),
        "doubled"
    );
    let mut open = cone(&square(0.5, -1.0), true);
    open.sectors.truncate(1);
    let got = rest::one_material(&whole(vec![cone(&square(0.5, 1.0), true), open]), band());
    assert_eq!(got, TouchVerdict::Unread(TouchUnread::Unanalysed), "open");
}

// ---- The bodies.

/// The point of a vertex.
fn at(body: &Body<f64>, v: VertexKey) -> P {
    body.points[body.vertices[v].point]
}

/// The three corners `tri` ordered counterclockwise about the outward
/// normal of the tetrahedron they make with `apex`.
fn outward(tri: [P; 3], apex: P) -> [P; 3] {
    let [a, b, c] = tri;
    if (b - a).cross(c - a).dot(apex - a) < 0.0 {
        [a, b, c]
    } else {
        [a, c, b]
    }
}

/// Struts from each vertex of `ring` (in the loop order of `face`) to
/// `to`, the faces between them closed: `face` moves to `to`, and the
/// vertices there are returned in the same order.
fn extrude(body: &mut Body<f64>, face: FaceKey, ring: &[VertexKey], to: &[P]) -> Vec<VertexKey> {
    let n = ring.len();
    let mut struts: Vec<MevCreated> = Vec::new();
    for i in 0..n {
        let h = body
            .find_half_edge(face, ring[i], ring[(i + 1) % n])
            .unwrap();
        let from = at(body, ring[i]);
        struts.push(
            body.mev(
                MevSite::Fan { he1: h, he2: h },
                to[i],
                line(from, to[i]),
                tol(),
            )
            .unwrap(),
        );
    }
    // The last face closes against the first one's edge along `to`.
    let mut first = None;
    for i in 0..n {
        let j = (i + 1) % n;
        let he2 = match first {
            Some(he) if i == n - 1 => he,
            _ => struts[j].he_minus,
        };
        let m = body
            .mef(
                MefSite::Chords {
                    he1: struts[i].he_minus,
                    he2,
                },
                line(to[i], to[j]),
                FaceSurface::Inherit,
                tol(),
            )
            .unwrap();
        if i == 0 {
            first = Some(m.he_plus);
        }
    }
    struts.iter().map(|s| s.vertex).collect()
}

/// **A tetrahedron grown from the cap `face`** (its loop through
/// `ring`): the cap becomes the face over `outer` (counterclockwise about
/// its outward normal) with `ring` as its hole, and `apex` closes the
/// tetrahedron's three other faces.
fn grow_tet(body: &mut Body<f64>, face: FaceKey, ring: &[VertexKey], outer: [P; 3], apex: P) {
    let h = body.find_half_edge(face, ring[0], ring[1]).unwrap();
    let fan = |he: HalfEdgeKey| MevSite::Fan { he1: he, he2: he };
    let r = at(body, ring[0]);
    let a = body
        .mev(fan(h), outer[0], line(r, outer[0]), tol())
        .unwrap();
    let b = body
        .mev(fan(a.he_minus), outer[1], line(outer[0], outer[1]), tol())
        .unwrap();
    let c = body
        .mev(fan(b.he_minus), outer[2], line(outer[1], outer[2]), tol())
        .unwrap();
    let chords = |he1, he2| MefSite::Chords { he1, he2 };
    let inherit = || FaceSurface::Inherit;
    let d = body
        .mef(
            chords(c.he_minus, a.he_minus),
            line(outer[2], outer[0]),
            inherit(),
            tol(),
        )
        .unwrap();
    body.kemr(a.he_minus, a.he_plus).unwrap();
    let x = body
        .mev(fan(c.he_minus), apex, line(outer[2], apex), tol())
        .unwrap();
    let m = body
        .mef(
            chords(x.he_minus, b.he_minus),
            line(apex, outer[1]),
            inherit(),
            tol(),
        )
        .unwrap();
    body.mef(
        chords(m.he_plus, d.he_minus),
        line(apex, outer[0]),
        inherit(),
        tol(),
    )
    .unwrap();
}

/// **Two tetrahedra in one shell**, `P` over `(0,0,0), (4,1,1),
/// (4,−1,−1), (−10,5,−1)` and `Q` over `(2,0,0), (−2,2,0), (−2,−2,0),
/// (8,1,2)` lifted by `lift` along `z`, joined by a mitred triangular
/// tube of radius 0.25 from a hole in `P`'s face `(0,0,0), (4,−1,−1),
/// (−10,5,−1)` out 3 m along its normal, across, and in 3 m along the
/// normal of `Q`'s face `(2,0,0), (−2,−2,0), (8,1,2)` to a hole there.
/// At `lift = 0` the faces `(0,0,0), (4,1,1), (4,−1,−1)` of `P` and
/// `(2,0,0), (−2,2,0), (−2,−2,0)` of `Q` cross along `[(0,0,0),
/// (2,0,0)]`, each with a vertex inside the other, and nothing else of
/// the body meets.
fn crossing_tetrahedra(lift: f64) -> Body<f64> {
    let p = |x: f64, y: f64, z: f64| P::new(x, y, z);
    let [v1, v2, a, b] = [
        p(0.0, 0.0, 0.0),
        p(4.0, 1.0, 1.0),
        p(4.0, -1.0, -1.0),
        p(-10.0, 5.0, -1.0),
    ];
    let q = |x: f64, y: f64, z: f64| P::new(x, y, z + lift);
    let [w1, w2, c, d] = [
        q(2.0, 0.0, 0.0),
        q(-2.0, 2.0, 0.0),
        q(-2.0, -2.0, 0.0),
        q(8.0, 1.0, 2.0),
    ];
    let (hole_p, hole_q) = (outward([v1, a, b], v2), outward([w1, c, d], w2));
    let centre = |t: [P; 3]| t[0] + ((t[1] - t[0]) + (t[2] - t[0])) / 3.0;
    let normal = |t: [P; 3]| (t[1] - t[0]).cross(t[2] - t[0]).normalize();
    let (p0, np, q0, nq) = (
        centre(hole_p),
        normal(hole_p),
        centre(hole_q),
        normal(hole_q),
    );
    let axis = [p0, p0 + np * 3.0, q0 + nq * 3.0, q0];
    let dir: Vec<Vec3<f64>> = (0..3)
        .map(|i| (axis[i + 1] - axis[i]).normalize())
        .collect();
    let mitre = [
        None,
        Some((dir[0] + dir[1]).normalize()),
        Some((dir[1] + dir[2]).normalize()),
        Some(dir[2]),
    ];
    let u = np.cross(Vec3::new(1.0, 0.0, 0.0)).normalize();
    let w = np.cross(u);
    let mut ring: Vec<P> = (0..3)
        .map(|k| {
            let t = f64::from(k) * std::f64::consts::TAU / 3.0;
            p0 + (u * t.cos() + w * t.sin()) * 0.25
        })
        .collect();
    let mut rings = vec![ring.clone()];
    for s in 1..4 {
        let m = mitre[s].unwrap();
        ring = ring
            .iter()
            .map(|&x| x + dir[s - 1] * ((axis[s] - x).dot(m) / dir[s - 1].dot(m)))
            .collect();
        rings.push(ring.clone());
    }

    let mut body = Body::<f64>::new();
    let r0 = &rings[0];
    let seed = body.mvfs(r0[0], true).unwrap();
    let e01 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            r0[1],
            line(r0[0], r0[1]),
            tol(),
        )
        .unwrap();
    let e12 = body
        .mev(
            MevSite::Fan {
                he1: e01.he_minus,
                he2: e01.he_minus,
            },
            r0[2],
            line(r0[1], r0[2]),
            tol(),
        )
        .unwrap();
    let back = body
        .mef(
            MefSite::Chords {
                he1: e12.he_minus,
                he2: e01.he_plus,
            },
            line(r0[2], r0[0]),
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    let start = [seed.vertex, e01.vertex, e12.vertex];
    let mut cap = start.to_vec();
    for to in &rings[1..] {
        cap = extrude(&mut body, seed.face, &cap, to);
    }
    grow_tet(&mut body, seed.face, &cap, hole_q, w2);
    grow_tet(
        &mut body,
        back.face,
        &[start[2], start[1], start[0]],
        hole_p,
        v2,
    );
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for f in faces {
        let outer = body.get_face(f).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("a face with no cycle")
        };
        let corners: Vec<P> = body
            .loop_cycle(first)
            .unwrap()
            .iter()
            .map(|&h| at(&body, body.get_half_edge(h).unwrap().start))
            .collect();
        body.set_face_surface(
            f,
            FaceSurface::New {
                surface: plane(&corners, tol()),
                sense: true,
            },
        )
        .unwrap();
    }
    describe_as_intersections(&mut body, tol());
    body
}

/// Every vertex-on-face touch a finding names, recorded.
fn recorded(errors: &[ValidationError]) -> ContactRecords {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UnrecordedContact {
                contact: CensusContact::VertexOnFace { vertex, face },
                ..
            } => Some(crate::Cited::new(
                crate::Contact::of(crate::Cell::Vertex(*vertex), crate::Cell::Face(*face)).unwrap(),
                crate::Cites::decided(0),
            )),
            _ => None,
        })
        .collect()
}

/// **Two faces of one solid crossing on vertex-on-face touches refuse
/// with their touches recorded**: each touch is a point where the solid
/// touches itself, and its link there crosses. Lifted off each other,
/// the faces' edges pierce instead.
#[test]
fn faces_of_one_solid_crossing_on_recorded_touches_refuse() {
    let body = crossing_tetrahedra(0.0);
    let none = ContactRecords::default();
    let unrecorded = validate_pseudomanifold(&body, &none, tol()).unwrap_err();
    let touches: Vec<String> = unrecorded
        .iter()
        .filter_map(|e| match e {
            ValidationError::UnrecordedContact {
                contact: CensusContact::VertexOnFace { vertex, .. },
                ..
            } => Some(witness(at(&body, *vertex))),
            _ => None,
        })
        .collect();
    assert_eq!(touches.len(), 4, "{unrecorded:?}");
    let records = recorded(&unrecorded);
    let errors = validate_pseudomanifold(&body, &records, tol()).unwrap_err();
    let crossed: Vec<String> = errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::BoundaryCrossesItself { witness, .. } => Some(witness.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(crossed.len(), errors.len(), "only crossings: {errors:?}");
    assert_eq!(
        BTreeSet::from_iter(&crossed),
        BTreeSet::from_iter(&touches),
        "one crossing at each touch"
    );

    let lifted = crossing_tetrahedra(0.125);
    let errors = validate_pseudomanifold(&lifted, &none, tol()).unwrap_err();
    assert_eq!(errors.len(), 4, "{errors:?}");
    assert!(
        errors.iter().all(|e| matches!(
            e,
            ValidationError::UnrecordedContact {
                contact: CensusContact::EdgeFacePierce { .. },
                ..
            }
        )),
        "{errors:?}"
    );
}

/// A pyramid with its apex at `apex` over the square of half-width `r`
/// about it at height `h` above it.
fn pyramid(apex: [f64; 3], r: f64, h: f64) -> crate::AtRestBody<f64> {
    let [x, y, z] = apex;
    let mut base = [
        [x + r, y + r, z + h],
        [x - r, y + r, z + h],
        [x - r, y - r, z + h],
        [x + r, y - r, z + h],
    ];
    if h > 0.0 {
        base.reverse();
    }
    posed_pyramid(&base, apex, &Pose::rest(), tol())
}

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> crate::AtRestBody<f64> {
    crate::test_support::finished("a block", brick::<f64>(x, y, z, tol()), tol())
}

fn built(what: &str, r: Result<BooleanResult<f64>, crate::BooleanError>) -> crate::AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => {
            if let Err(e) = validate_pseudomanifold(&r.body, &r.contacts, tol()) {
                panic!("{what}: {e:?}");
            }
            r.body
        }
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

/// **The pit holds tier 3′ with its record**: the slab `[0, 4]² × [0,
/// 1]` less a pyramid whose apex rests on the floor, a cavity's corner
/// on a face from inside.
#[test]
fn a_pit_resting_its_apex_on_the_floor_holds_tier_three_prime() {
    let slab = block((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let r = subtract(&slab, &pyramid([2.0, 2.0, 0.0], 1.0, 2.0), tol());
    let Ok(BooleanResult::Body(ref pit)) = r else {
        panic!("the pit builds");
    };
    assert_eq!(pit.body.solids.len(), 1, "one solid");
    assert_eq!(
        pit.contacts.vertex_face().count(),
        1,
        "the apex on the floor"
    );
    built("the pit", r);
}

/// The vertices of `body` at `q`.
fn vertices_at(body: &Body<f64>, q: [f64; 3]) -> usize {
    body.vertices()
        .filter(|(_, v)| {
            let p = body.points[v.point];
            (p - P::new(q[0], q[1], q[2])).norm() < 1e-9
        })
        .count()
}

/// **Two pits and a needle hold tier 3′**: the slab `[0, 4]² × [0, 2]`
/// less two pyramids from `(2, 2, 1)`, one up through the top and one
/// down through the floor, and a needle from the same apex standing in
/// the upper pit, held by an arch over the slab.
#[test]
fn two_pits_and_a_needle_hold_tier_three_prime() {
    let apex = [2.0, 2.0, 1.0];
    let slab = block((0.0, 4.0), (0.0, 4.0), (0.0, 2.0));
    let one = built("one pit", subtract(&slab, &pyramid(apex, 1.0, 2.0), tol()));
    let two = built("two pits", subtract(&one, &pyramid(apex, 1.0, -2.0), tol()));
    let post = block((0.2, 0.8), (1.6, 2.4), (1.5, 4.0));
    let beam = block((0.2, 2.6), (1.6, 2.4), (3.4, 4.0));
    let arch = built("the arch", union(&post, &beam, tol()));
    let held = built("the needle", union(&arch, &pyramid(apex, 0.2, 2.5), tol()));
    let body = built("two pits and a needle", union(&two, &held, tol()));
    assert_eq!(body.solids.len(), 1, "one solid");
    assert_eq!(
        vertices_at(&body, apex),
        3,
        "one vertex per cone at the apex"
    );
}

// ---- At the census door: one solid of several shells, below tier 3's
// one-outer-shell bar, which is why these rows call the census directly.

/// A cell and where it is read: a vertex's corner shares the cube's key
/// at the origin, sits on a key of its own there, or rests inside the
/// cube's edge along `x` at `(0.5, 0, 0)`.
#[derive(Clone, Copy, Debug)]
enum Seat {
    Shared,
    Tied,
    OnEdge,
}

/// **The unit cube and a parallelepiped `u·a + v·b + w·c` cornered at
/// `seat`, one solid**: the parallelepiped's shell moved into the cube's
/// solid. Returns the body, the parallelepiped's corner, and the records
/// that tie it to the cube (none for a shared key).
fn cube_and_corner(
    seat: Seat,
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
) -> (Body<f64>, ContactRecords) {
    let o = match seat {
        Seat::Shared | Seat::Tied => [0.0, 0.0, 0.0],
        Seat::OnEdge => [0.5, 0.0, 0.0],
    };
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let part = crate::test_support_fixtures::mapped_cube::<f64>(
        move |u, v, w| {
            let k = |i: usize| o[i] + u * a[i] + v * b[i] + w * c[i];
            P::new(k(0), k(1), k(2))
        },
        tol(),
    );
    let before: BTreeSet<VertexKey> = body.vertices.keys().collect();
    let keep = census_entity_solid(&body, EntityId::Vertex(*before.first().unwrap())).unwrap();
    let gone = crate::instance::graft_disjoint(&mut body, &part).unwrap();
    for sh in body.solids.remove(gone).unwrap().shells {
        body.shells[sh].solid = keep;
        body.solids[keep].shells.push(sh);
    }
    body.solid_provenance.remove(gone);
    let here =
        |body: &Body<f64>, v: VertexKey| (at(body, v) - P::new(o[0], o[1], o[2])).norm() < 1e-12;
    let corner = body
        .vertices
        .keys()
        .find(|v| !before.contains(v) && here(&body, *v))
        .unwrap();
    let cited =
        |x, y| crate::Cited::new(crate::Contact::of(x, y).unwrap(), crate::Cites::decided(0));
    let records = match seat {
        Seat::Shared => {
            let origin = before.iter().copied().find(|&v| here(&body, v)).unwrap();
            let (old, key) = (body.vertices[corner].point, body.vertices[origin].point);
            body.vertices[corner].point = key;
            body.points.remove(old);
            ContactRecords::default()
        }
        Seat::Tied => {
            let origin = before.iter().copied().find(|&v| here(&body, v)).unwrap();
            [cited(
                crate::Cell::Vertex(corner),
                crate::Cell::Vertex(origin),
            )]
            .into_iter()
            .collect()
        }
        Seat::OnEdge => {
            let geo = snapshot(&body);
            let edge = line_edges(&body)
                .into_iter()
                .find(|e| {
                    let (p0, p1) = (geo.vmap[&e.v0], geo.vmap[&e.v1]);
                    p0.y.abs() + p0.z.abs() + p1.y.abs() + p1.z.abs() < 1e-12
                        && before.contains(&e.v0)
                })
                .unwrap()
                .key;
            [cited(crate::Cell::Vertex(corner), crate::Cell::Edge(edge))]
                .into_iter()
                .collect()
        }
    };
    (body, records)
}

/// The self-touch findings of the census over `body` and `records`.
fn self_touch(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    census_and_certify(body, records, band(), tol(), None)
        .into_iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::BoundaryCrossesItself { .. }
                    | ValidationError::BoundaryCrossesItselfUndecided { .. }
            )
        })
        .collect()
}

/// **A corner of one solid touching its own corner or edge reads the
/// solid's whole link**, on a shared key (pass 1), on keys a record ties
/// (pass 1 again), and inside an edge (pass 2): a parallelepiped
/// reaching into the cube's material crosses it, one turned away rests.
#[test]
fn a_corner_touching_its_own_solid_reads_its_whole_link() {
    let into = ([0.5, 0.1, 0.1], [0.1, 0.5, 0.1], [0.1, 0.1, 0.5]);
    let away = ([-0.1, 0.7, 0.1], [-0.6, 0.2, 0.1], [-0.2, 0.3, 0.6]);
    let edge_into = ([0.1, 0.3, 0.3], [-0.1, 0.3, 0.3], [0.0, 0.2, 0.5]);
    let edge_away = ([0.1, -0.3, -0.3], [-0.1, -0.3, -0.3], [0.0, -0.2, -0.5]);
    for (seat, (a, b, c), crosses) in [
        (Seat::Shared, into, true),
        (Seat::Shared, away, false),
        (Seat::Tied, into, true),
        (Seat::Tied, away, false),
        (Seat::OnEdge, edge_into, true),
        (Seat::OnEdge, edge_away, false),
    ] {
        let (body, records) = cube_and_corner(seat, a, b, c);
        let got = self_touch(&body, &records);
        if crosses {
            assert!(
                matches!(&got[..], [ValidationError::BoundaryCrossesItself { .. }]),
                "{seat:?}, reaching in: {got:?}"
            );
        } else {
            assert!(got.is_empty(), "{seat:?}, turned away: {got:?}");
        }
    }
}

/// **A solid crossing itself blocks its pairs in arm 2**: the crossing
/// tetrahedra with their touches recorded, beside a block in their reach
/// that touches nothing, read as a crossing of that solid's boundary.
#[test]
fn a_solid_crossing_itself_blocks_its_pairs() {
    let mut body = crossing_tetrahedra(0.0);
    let none = ContactRecords::default();
    let records = recorded(&validate_pseudomanifold(&body, &none, tol()).unwrap_err());
    let cube = brick::<f64>((5.0, 5.5), (3.0, 3.5), (-2.5, -2.0), tol());
    crate::instance::graft_disjoint(&mut body, &cube).unwrap();
    let errors = census_and_certify(&body, &records, band(), tol(), None);
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::CensusUndecidable { what, .. } if *what == Undecided::Crossing.what()
        )),
        "{errors:?}"
    );
}

/// **A line touch passing through the point reads as the line rule
/// reads it**: an edge lying along the floor, its wedge above, read
/// inside the edge (a vertex tied inside it, whose line
/// [`line_touch_reaches`] does not skip). Standing on a floor whose
/// material is below, the wedge rests (1 below, 1 in the wedge, 0
/// elsewhere); sunk into a floor whose material is above, it crosses (2
/// in the wedge), as [`edge_in_own_face`] refuses it along the line.
#[test]
fn a_line_touch_through_the_point_reads_as_its_line() {
    let p = |x: f64, y: f64, z: f64| P::new(x, y, z);
    let r5 = 5f64.sqrt();
    for (up, rests) in [(true, true), (false, false)] {
        let mut verts = vec![
            p(-1.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.0, 0.5, 1.0),
            p(-1.0, 0.5, 1.0),
            p(-1.0, -0.5, 1.0),
            p(1.0, -0.5, 1.0),
        ];
        let mut ground = vec![
            p(-2.0, -2.0, 0.0),
            p(2.0, -2.0, 0.0),
            p(2.0, 2.0, 0.0),
            p(-2.0, 2.0, 0.0),
        ];
        if !up {
            ground.reverse();
        }
        verts.extend(ground);
        let faces = [
            (Vec3::new(0.0, 2.0, -1.0) / r5, vec![vec![0, 3, 2, 1]]),
            (Vec3::new(0.0, -2.0, -1.0) / r5, vec![vec![1, 5, 4, 0]]),
            (
                Vec3::new(0.0, 0.0, if up { 1.0 } else { -1.0 }),
                vec![vec![6, 7, 8, 9]],
            ),
        ];
        let edge = [(vec![0, 1], [0, 1])];
        let link = synthetic(P::origin(), At::Edge, verts, &faces, &edge, band())
            .expect("a wedge on a floor");
        let got = rest::one_material(&link, band());
        if rests {
            assert!(got.is_rest(), "standing: {got:?}");
        } else {
            assert_eq!(got, TouchVerdict::Crossing, "sunk");
        }
    }
}

// ---- rev5704-r1 review probes.

/// A disc in the plane spanned by `x` and `e2` with outward `m`, its
/// four corners at `2·(cos, sin)(rot + k·π/2)`.
fn r1_disc(e2: Vec3<f64>, m: Vec3<f64>, rot: f64) -> Link<f64> {
    let e1 = Vec3::new(1.0, 0.0, 0.0);
    let mut ring: Vec<P> = (0..4)
        .map(|k| {
            let t = rot + f64::from(k) * std::f64::consts::FRAC_PI_2;
            P::origin() + (e1 * t.cos() + e2 * t.sin()) * 2.0
        })
        .collect();
    let m = m.normalize();
    if (ring[1] - ring[0]).cross(ring[2] - ring[0]).dot(m) < 0.0 {
        ring.reverse();
    }
    synthetic(
        P::origin(),
        At::Face,
        ring,
        &[(m, vec![vec![0, 1, 2, 3]])],
        &[],
        band(),
    )
    .unwrap()
}

#[test]
fn r1_three_discs_through_one_line() {
    let r2 = std::f64::consts::FRAC_1_SQRT_2;
    let s = |rot| r1_disc(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0), rot);
    let t = |rot| r1_disc(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), rot);
    let w = |rot, sign: f64| {
        r1_disc(
            Vec3::new(0.0, r2, -r2),
            Vec3::new(0.0, -1.0, -1.0) * sign,
            rot,
        )
    };
    let rows: Vec<(&str, Link<f64>)> = vec![
        ("two on axis", whole(vec![s(0.0), t(0.0)])),
        ("two off axis", whole(vec![s(0.3), t(0.2)])),
        ("three on axis", whole(vec![s(0.0), t(0.0), w(0.0, 1.0)])),
        ("three off axis", whole(vec![s(0.3), t(0.2), w(0.1, 1.0)])),
        ("three on axis, W flipped", whole(vec![s(0.0), t(0.0), w(0.0, -1.0)])),
        ("three, only S on axis", whole(vec![s(0.0), t(0.2), w(0.1, 1.0)])),
        ("three, S and T on axis", whole(vec![s(0.0), t(0.0), w(0.1, 1.0)])),
    ];
    for (what, link) in rows {
        eprintln!("R1PROBE {what}: {:?}", rest::one_material(&link, band()));
    }
}

/// The unit cube with parts grafted into its solid.
fn r1_cube_with(parts: &[([f64; 3], [f64; 3], [f64; 3], [f64; 3])]) -> Body<f64> {
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let first = body.vertices.keys().next().unwrap();
    let keep = census_entity_solid(&body, EntityId::Vertex(first)).unwrap();
    for &(o, a, b, c) in parts {
        let part = crate::test_support_fixtures::mapped_cube::<f64>(
            move |u, v, w| {
                let k = |i: usize| o[i] + u * a[i] + v * b[i] + w * c[i];
                P::new(k(0), k(1), k(2))
            },
            tol(),
        );
        let gone = crate::instance::graft_disjoint(&mut body, &part).unwrap();
        for sh in body.solids.remove(gone).unwrap().shells {
            body.shells[sh].solid = keep;
            body.solids[keep].shells.push(sh);
        }
        body.solid_provenance.remove(gone);
    }
    body
}

/// Every unrecorded contact the census names, recorded.
fn r1_record_all(errors: &[ValidationError]) -> ContactRecords {
    use crate::Cell;
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UnrecordedContact { contact, .. } => {
                let (x, y) = match contact {
                    CensusContact::VertexVertex { a, b } => (Cell::Vertex(*a), Cell::Vertex(*b)),
                    CensusContact::VertexOnFace { vertex, face } => {
                        (Cell::Vertex(*vertex), Cell::Face(*face))
                    }
                    CensusContact::VertexOnEdge { vertex, edge } => {
                        (Cell::Vertex(*vertex), Cell::Edge(*edge))
                    }
                    CensusContact::EdgeFaceOverlap { edge, face } => {
                        (Cell::Edge(*edge), Cell::Face(*face))
                    }
                    CensusContact::EdgeEdgeOverlap { a, b } => (Cell::Edge(*a), Cell::Edge(*b)),
                    _ => return None,
                };
                Some(crate::Cited::new(
                    crate::Contact::of(x, y).unwrap(),
                    crate::Cites::decided(0),
                ))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn r1_a_line_touch_hides_a_sunk_corner_at_its_end() {
    let p = [0.5, 0.5, 1.0];
    // B: a corner at `p` sunk into the cube's material.
    let sunk = (p, [0.02, 0.15, -0.05], [0.15, 0.02, -0.05], [0.02, 0.02, -0.2]);
    // C: a block cornered at `p`, its edge along +x lying in the top
    // face, standing above it.
    let resting = (p, [0.0, -0.2, 0.2], [0.3, 0.0, 0.0], [0.05, 0.05, 0.3]);
    for (what, parts) in [
        ("B alone", vec![sunk]),
        ("C alone", vec![resting]),
        ("B and C", vec![sunk, resting]),
    ] {
        let body = r1_cube_with(&parts);
        let first = census_and_certify(&body, &ContactRecords::default(), band(), tol(), None);
        let records = r1_record_all(&first);
        let got = census_and_certify(&body, &records, band(), tol(), None);
        eprintln!("R1PROBE {what}: {} records; findings {got:?}", records.len());
    }
}

fn r1_report(what: &str, r: Result<BooleanResult<f64>, crate::BooleanError>) {
    match r {
        Ok(BooleanResult::Body(r)) => {
            let solids = r.body.solids.len();
            let shells = r.body.shells.len();
            let records = r.contacts.vertex_face().count();
            eprintln!(
                "R1PROBE {what}: solids {solids} shells {shells} vf records {records}: {:?}",
                validate_pseudomanifold(&r.body, &r.contacts, tol())
            );
        }
        other => eprintln!("R1PROBE {what}: op {:?}", other.map(|_| ())),
    }
}

#[test]
fn r1_boolean_self_touches() {
    let slab = || block((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    r1_report(
        "a pyramid standing on its apex on a slab (union)",
        union(&slab(), &pyramid([2.0, 2.0, 1.0], 1.0, 2.0), tol()),
    );
    let tall = || block((0.0, 4.0), (0.0, 4.0), (0.0, 3.0));
    r1_report(
        "a cavity's apex touching the top from inside",
        subtract(&tall(), &pyramid([2.0, 2.0, 3.0], 1.0, -2.0), tol()),
    );
    r1_report(
        "a cavity's apex touching the floor from inside",
        subtract(&tall(), &pyramid([2.0, 2.0, 0.0], 1.0, 2.0), tol()),
    );
    // The mould of a pyramid standing on a plate: the block less the
    // plate and the pyramid on it, apex up to the block's top.
    let plate = block((1.0, 3.0), (1.0, 3.0), (0.0, 0.5));
    let on = built(
        "pyramid on plate",
        union(&plate, &pyramid([2.0, 2.0, 3.0], 0.8, -2.5), tol()),
    );
    r1_report("the mould of a pyramid on a plate", subtract(&tall(), &on, tol()));
    // Two pyramids apex to apex inside a slab: a void pinched at a point.
    let mid = || block((0.0, 4.0), (0.0, 4.0), (0.0, 4.0));
    let up = built(
        "upper void",
        subtract(&mid(), &pyramid([2.0, 2.0, 2.0], 1.0, 1.0), tol()),
    );
    r1_report(
        "two voids apex to apex",
        subtract(&up, &pyramid([2.0, 2.0, 2.0], 1.0, -1.0), tol()),
    );
}

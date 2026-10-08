//! **Holes meeting at one vertex of a face build one body whatever order
//! their members fold in.**
//!
//! A plate `[0, 3] × [0, 2] × [0, 1]` with k prisms standing in it,
//! whose footprints on its top are k holes meeting at one point
//! `MEET = (1.5, 1, 1)`. Each prism is a right prism along an axis tilted
//! out of its own footprint, so above the top the prisms move apart and
//! touch nowhere; below it they cross inside the plate. When k ≥ 3
//! prisms fold before the plate, their union's vertex at `MEET` pierces
//! the top with k Out runs, and the pierce's ring hangs one strut per
//! run round one ring vertex. The struts' order round it is the order
//! the top's loop passes its corners at `MEET`, so in any but the runs'
//! angular order the loop crosses itself there.
//!
//! The rows, each in every member order:
//! - two, three and four wedges;
//! - three wedges clustered on one side, which leave the top a reflex
//!   sector at `MEET`;
//! - an L-shaped hole with its reflex corner at `MEET`, and two wedges in
//!   the quadrant it leaves.
//!
//! A pyramid with its apex at `MEET` meets the top with runs that nest,
//! and so do the holes of `meeting::arch`: nested under one, one between
//! others, or branching, each builds in every op against the plate,
//! whichever region of the link roots the ring.
//!
//! Each order asserts its counts, closed-form volume, tiers 3 and 3′,
//! that every face's corners at one point are angularly disjoint, one
//! vertex at `MEET`, and the body of the first order, compared by geometry.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{
    Hole, MEET, PLATE, Pose, apex_pyramid, arch, arch_cone, at, branching_cone, comb,
    corners_disjoint, ell_and_wedges, four_wedges, inner_rows, notch, notch_rows, orders,
    posed_box, posed_prism, poses, shape, three_wedges, two_wedges, wedge, wedges_on_one_side,
};
use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanResult, union, validate_geometric, validate_pseudomanifold};

fn t() -> Tol {
    Tol::witness()
}

/// Folds the plate (member 0) and `holes`' prisms by union in every
/// order and asserts each result (module docs).
fn every_order(label: &str, holes: &[Hole], counts: [usize; 3]) {
    let rest = Pose::rest();
    let mut members = vec![posed_box("the plate", PLATE, &rest, t())];
    members.extend(holes.iter().map(|h| posed_prism(h, &rest, t())));
    let volume = 6.0 + holes.iter().map(Hole::above).sum::<f64>();
    let c = at(Point3::new(MEET[0], MEET[1], MEET[2]));
    let mut first = None;
    for order in orders(members.len()) {
        let what = format!("{label}, member order {order:?} (0 = plate)");
        let mut body = members[order[0]].clone();
        let mut contacts = None;
        for (k, &i) in order.iter().enumerate().skip(1) {
            match union(&body, &members[i], t()) {
                Ok(BooleanResult::Body(r)) => {
                    body = r.body;
                    contacts = Some(r.contacts);
                }
                Ok(BooleanResult::Empty) => panic!("{what}: step {k} is empty"),
                Err(e) => panic!("{what}: step {k} refused: {e:?}"),
            }
        }
        let got = [
            body.faces().count(),
            body.edges().count(),
            body.vertices().count(),
        ];
        assert_eq!(got, counts, "{what}: faces, edges, vertices");
        assert_eq!(validate_geometric(&body, t()), Ok(()), "{what}: tier 3");
        assert!(
            validate_pseudomanifold(&body, &contacts.unwrap(), t()).is_ok(),
            "{what}: tier 3′"
        );
        let v = topo::mass_properties(&body, t()).unwrap().volume;
        assert!(
            (v - volume).abs() < 1e-9,
            "{what}: volume {v}, closed form {volume}"
        );
        assert_eq!(corners_disjoint(&body), Ok(()), "{what}: corners");
        let s = shape(&body);
        let at_c = body
            .vertices()
            .filter(|&(k, _)| at(topo::readback::vertex_point(&body, k).unwrap()) == c)
            .count();
        assert_eq!(at_c, 1, "{what}: vertices at MEET");
        match &first {
            None => first = Some(s),
            Some(f) => assert!(*f == s, "{what}: a different body from the first order"),
        }
    }
}

/// **Two, three and four wedges meeting at one vertex of the top build
/// one body in every member order.**
#[test]
fn wedges_meeting_at_a_vertex_build_one_body_in_every_member_order() {
    every_order("two wedges", &two_wedges(), [14, 30, 19]);
    every_order("three wedges", &three_wedges(), [18, 39, 24]);
    every_order("four wedges", &four_wedges(), [22, 48, 29]);
}

/// **Holes meeting at a vertex with a reflex sector there build one body
/// in every member order**: three wedges on one side, leaving the top a
/// reflex sector between them, and an L-shaped hole whose reflex corner
/// is the vertex, with two wedges in the quadrant it leaves.
#[test]
fn holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order() {
    every_order(
        "three wedges on one side",
        &wedges_on_one_side(),
        [18, 39, 24],
    );
    every_order("an L and two wedges", &ell_and_wedges(), [21, 48, 30]);
}

fn body(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        Ok(BooleanResult::Empty) => panic!("{what}: empty"),
        Err(e) => panic!("{what}: refused: {e:?}"),
    }
}

fn built(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> topo::BooleanBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r,
        Ok(BooleanResult::Empty) => panic!("{what}: empty"),
        Err(e) => panic!("{what}: refused: {e:?}"),
    }
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(b, t()).unwrap().volume
}

/// Asserts `r` is tier 3 and 3′ valid with `want`, its corners disjoint, and
/// that a block across the meeting point unions with it.
fn sound(what: &str, r: &topo::BooleanBody<f64>, want: f64, pose: &Pose) {
    let b = &r.body;
    assert_eq!(validate_geometric(b, t()), Ok(()), "{what}: tier 3");
    assert!(
        validate_pseudomanifold(b, &r.contacts, t()).is_ok(),
        "{what}: tier 3′"
    );
    let v = volume(b);
    assert!(
        (v - want).abs() < 1e-9,
        "{what}: volume {v}, expected {want}"
    );
    assert_eq!(corners_disjoint(b), Ok(()), "{what}: corners");
    let block = posed_box(
        "a block across the meeting point",
        [(1.21, 1.77), (0.68, 1.31), (0.86, 1.52)],
        pose,
        t(),
    );
    body(&format!("{what}, then ∪ a block"), union(b, &block, t()));
}

/// **The plate against the union of the holes' prisms, in every op and
/// five poses.** The plate's top less the holes passes the meeting point
/// once per hole: as one ring where the holes lie inside it
/// ([`inner_rows`]), as the outer loops of several faces of one plane
/// where some notch its boundary ([`notch_rows`]).
/// - U − P (the prisms above the top) and both intersections (inside
///   it) build sound;
/// - P − U, in one boolean, builds sound too. Both operands keep the
///   meeting point as one vertex, and `zip::split_cones` splits it per
///   cone of the result before the zips, so the top's boundary passes
///   the point once per hole, its corners there disjoint, as the
///   sequential subtract builds it. Main crossed it into rings through
///   one vertex before `split_cones` (JOIN's
///   `two-representations-of-holes-meeting-at-a-point`);
/// - the plate less each prism in turn builds the same volume sound,
///   the meeting point a vertex per hole.
///
/// Volumes are inclusion-exclusion against U's, and against each hole's
/// closed form above the top where the holes lie inside it.
#[test]
fn the_plate_against_the_holes_union_builds_sound_in_every_op() {
    use topo::{intersect, subtract};
    let inner = inner_rows().len();
    for pose in poses() {
        let p = posed_box("the plate", PLATE, &pose, t());
        for (r, (fixture, holes)) in inner_rows().into_iter().chain(notch_rows()).enumerate() {
            let label = format!("{fixture}, {}", pose.label);
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &pose, t())).collect();
            let u = prisms[1..].iter().fold(prisms[0].clone(), |u, q| {
                body(&format!("{label}: the prisms' union"), union(&u, q, t()))
            });
            let inside = volume(&body(&label, intersect(&p, &u, t())));
            if r < inner {
                let above: f64 = holes.iter().map(Hole::above).sum();
                assert!(
                    (volume(&u) - above - inside).abs() < 1e-9,
                    "{label}: P ∩ U against the holes' closed form"
                );
            }
            sound(
                &format!("{label}: U − P"),
                &built(&label, subtract(&u, &p, t())),
                volume(&u) - inside,
                &pose,
            );
            for (what, r) in [
                ("P ∩ U", intersect(&p, &u, t())),
                ("U ∩ P", intersect(&u, &p, t())),
            ] {
                sound(
                    &format!("{label}: {what}"),
                    &built(&label, r),
                    inside,
                    &pose,
                );
            }
            sound(
                &format!("{label}: P − U"),
                &built(&label, subtract(&p, &u, t())),
                6.0 - inside,
                &pose,
            );
            let first = built(
                &format!("{label}: P less each prism"),
                subtract(&p, &prisms[0], t()),
            );
            let seq = prisms[1..].iter().fold(first, |b, q| {
                built(
                    &format!("{label}: P less each prism"),
                    subtract(&b.body, q, t()),
                )
            });
            sound(
                &format!("{label}: P less each prism"),
                &seq,
                6.0 - inside,
                &pose,
            );
        }
    }
}

/// The P − U grid at `k` holes (review 3's sweep, ported): 30° holes in `k` of eight 45° slots
/// round the meeting point (the first slot always taken, as turning
/// the scene about the top's normal moves no answer), each hole a
/// wedge or a notch past the plate's edge, at rest. Every
/// configuration's P − U builds sound: tiers 3 and 3′, `corners_disjoint`,
/// and the volume against P ∩ U.
fn grid(k: usize) {
    use topo::{intersect, subtract};
    fn slots(from: usize, k: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        for s in from..8 {
            cur.push(s);
            slots(s + 1, k, cur, out);
            cur.pop();
        }
    }
    let mut combos = Vec::new();
    slots(1, k, &mut vec![0], &mut combos);
    let rest = Pose::rest();
    let p = posed_box("the plate", PLATE, &rest, t());
    for c in combos {
        for mask in 0..1u32 << k {
            let label = format!("slots {c:?}, notches {mask:0k$b}");
            let holes: Vec<Hole> = c
                .iter()
                .enumerate()
                .map(|(j, &s)| {
                    let a = 45.0f64.mul_add(s as f64, 5.0);
                    if mask >> j & 1 == 1 {
                        notch(a, a + 30.0, j, 2.0)
                    } else {
                        wedge(a, a + 30.0, j)
                    }
                })
                .collect();
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &rest, t())).collect();
            let u = prisms[1..].iter().fold(prisms[0].clone(), |u, q| {
                body(&format!("{label}: the prisms' union"), union(&u, q, t()))
            });
            let inside = volume(&body(&format!("{label}: P ∩ U"), intersect(&p, &u, t())));
            match subtract(&p, &u, t()) {
                Ok(BooleanResult::Body(r)) => {
                    let b = r.body;
                    assert_eq!(
                        validate_geometric(&b, t()),
                        Ok(()),
                        "{label}: P − U, tier 3"
                    );
                    assert!(
                        validate_pseudomanifold(&b, &r.contacts, t()).is_ok(),
                        "{label}: P − U, tier 3′"
                    );
                    assert_eq!(corners_disjoint(&b), Ok(()), "{label}: P − U, corners");
                    let v = volume(&b);
                    assert!(
                        (v - (6.0 - inside)).abs() < 1e-8,
                        "{label}: P − U, volume {v}"
                    );
                }
                Ok(BooleanResult::Empty) => panic!("{label}: P − U is empty"),
                Err(e) => panic!("{label}: P − U refused: {e:?}"),
            }
        }
    }
}

/// **Three holes in every slot pattern and every mix of wedges and
/// notches build P − U sound** ([`grid`]).
#[test]
fn every_three_hole_grid_configuration_builds_p_minus_u_sound() {
    grid(3);
}

/// **Four holes in every slot pattern and every mix of wedges and
/// notches build P − U sound** ([`grid`]).
#[test]
fn every_four_hole_grid_configuration_builds_p_minus_u_sound() {
    grid(4);
}

/// Every op on the plate `p` and `u`, both orders, labelled.
fn every_op(
    p: &AtRestBody<f64>,
    u: &AtRestBody<f64>,
) -> [(&'static str, Result<BooleanResult<f64>, topo::BooleanError>); 6] {
    use topo::{intersect, subtract};
    [
        ("P − U", subtract(p, u, t())),
        ("U − P", subtract(u, p, t())),
        ("P ∪ U", union(p, u, t())),
        ("U ∪ P", union(u, p, t())),
        ("P ∩ U", intersect(p, u, t())),
        ("U ∩ P", intersect(u, p, t())),
    ]
}

/// Whether `q` is inside `body`, `None` on its boundary.
fn inside_of(body: &AtRestBody<f64>, q: Point3<f64>) -> Option<bool> {
    let band = geom_core::Band::linear(t()).unwrap();
    match topo::point_in_solid(body, q, band, t()).unwrap() {
        topo::SolidContainment::In => Some(true),
        topo::SolidContainment::Out => Some(false),
        topo::SolidContainment::OnBoundary => None,
    }
}

/// Probes about [`MEET`] placed by `pose`: a 9 × 9 × 9 grid of points
/// within 0.4 of it, offset off the fixtures' planes, and 64 points
/// 0.03 from it spread over the sphere.
fn probes(pose: &Pose) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            for k in 0..9 {
                let at = |n: i32, d: f64| 0.1f64.mul_add(f64::from(n) - 4.0, d);
                out.push([at(i, 0.0137), at(j, 0.0071), at(k, 0.0093)]);
            }
        }
    }
    for k in 0..64 {
        let z = 1.0 - (2.0 * f64::from(k) + 1.0) / 64.0;
        let (s, c) = (2.399_963 * f64::from(k)).sin_cos();
        let r = z.mul_add(-z, 1.0).sqrt();
        out.push([0.03 * r * c, 0.03 * r * s, 0.03 * z]);
    }
    out.into_iter()
        .map(|d| pose.at([0, 1, 2].map(|i| MEET[i] + d[i])))
        .collect()
}

/// The plate's volume.
const PLATE_VOLUME: f64 = 6.0;

/// The volume of [`apex_pyramid`] over `base`, and of its part below the
/// top, in closed form: the base lies in the plane half a unit from the
/// apex, so each is a sixth of the area its polygon bounds there, the
/// part below the top over the polygon clipped to `z < 0`.
fn pyramid_volumes(base: &[[f64; 3]]) -> (f64, f64) {
    let area = |poly: &[(f64, f64)]| {
        (0..poly.len())
            .map(|i| {
                let ((y0, z0), (y1, z1)) = (poly[i], poly[(i + 1) % poly.len()]);
                y0 * z1 - y1 * z0
            })
            .sum::<f64>()
            .abs()
            / 2.0
    };
    let poly: Vec<(f64, f64)> = base.iter().map(|q| (q[1], q[2])).collect();
    let mut below = Vec::new();
    for i in 0..poly.len() {
        let ((y0, z0), (y1, z1)) = (poly[i], poly[(i + 1) % poly.len()]);
        if z0 < 0.0 {
            below.push((y0, z0));
        }
        if (z0 < 0.0) != (z1 < 0.0) {
            below.push((y0 + (y1 - y0) * z0 / (z0 - z1), 0.0));
        }
    }
    (area(&poly) / 6.0, area(&below) / 6.0)
}

/// Every op on the plate and `u` at `pose`, both operand orders, builds
/// sound ([`sound`]): tiers 3 and 3′, `corners_disjoint`, a block across
/// the meeting point that unions with it, and its volume, from `closed`
/// (`u`'s and its part inside the plate) where given and otherwise from
/// the intersection's, against the identities between the ops. At
/// every probe ([`probes`]) where no body reads its boundary, the result
/// holds material exactly where the op over the operands' own
/// containment does.
fn every_op_sound(label: &str, u: &AtRestBody<f64>, pose: &Pose, closed: Option<(f64, f64)>) {
    use topo::intersect;
    let p = posed_box("the plate", PLATE, pose, t());
    let vu = volume(u);
    let inside = match closed {
        Some((whole, below)) => {
            assert!(
                (vu - whole).abs() < 1e-9,
                "{label}: U's volume {vu}, closed form {whole}"
            );
            below
        }
        None => volume(&built(label, intersect(&p, u, t())).body),
    };
    let probes: Vec<_> = probes(pose)
        .into_iter()
        .filter_map(|q| Some((q, inside_of(&p, q)?, inside_of(u, q)?)))
        .collect();
    for (what, r) in every_op(&p, u) {
        let label = format!("{label}: {what}");
        let (want, keep): (f64, fn(bool, bool) -> bool) = match what {
            "P − U" => (PLATE_VOLUME - inside, |p, u| p && !u),
            "U − P" => (vu - inside, |p, u| u && !p),
            "P ∪ U" | "U ∪ P" => (PLATE_VOLUME + vu - inside, |p, u| p || u),
            _ => (inside, |p, u| p && u),
        };
        let r = built(&label, r);
        sound(&label, &r, want, pose);
        let mut read = 0;
        for &(q, in_p, in_u) in &probes {
            if let Some(got) = inside_of(&r.body, q) {
                assert_eq!(got, keep(in_p, in_u), "{label}: material at {q:?}");
                read += 1;
            }
        }
        assert!(read > 600, "{label}: {read} probes read");
    }
}

/// The arch's prisms' union at `pose`.
fn arch_union(pose: &Pose) -> AtRestBody<f64> {
    let prisms: Vec<_> = arch().iter().map(|h| posed_prism(h, pose, t())).collect();
    prisms[1..].iter().fold(prisms[0].clone(), |u, q| {
        body("the arch's prisms' union", union(&u, q, t()))
    })
}

/// The cones over [`comb`], [`arch_cone`] and [`branching_cone`],
/// labelled, with their runs above the top.
fn cones() -> [(&'static str, Vec<[f64; 3]>, usize); 3] {
    [
        ("the comb", comb(), 3),
        ("the arch cone", arch_cone(), 3),
        ("the branching cone", branching_cone(), 5),
    ]
}

/// **A cone whose runs above the top nest under one builds in every op,
/// pose and operand order** ([`comb`], [`every_op_sound`] against the
/// closed form). The ring is a star: its struts hang at the ring vertex
/// in an order other than the runs' along the apex's link, each facing
/// its end germ first.
#[test]
fn a_cone_whose_runs_nest_under_one_builds_in_every_op() {
    for pose in poses() {
        let u = apex_pyramid(&comb(), &pose, t());
        let label = format!("the comb, {}", pose.label);
        every_op_sound(&label, &u, &pose, Some(pyramid_volumes(&comb())));
    }
}

/// **Runs one between others build in every op, pose and operand
/// order** ([`every_op_sound`]): the holes of [`arch`], whose second run
/// lies between the other two about the top's normal; the arch cone
/// ([`arch_cone`]), one run inside another inside a third, whose ring is
/// a path; and the branching cone ([`branching_cone`]), one run over two
/// each over one more, whose ring is a tree deeper than a path. The
/// cones hold their closed-form volumes.
#[test]
fn runs_one_between_others_build_in_every_op() {
    for pose in poses() {
        every_op_sound(
            &format!("the arch, {}", pose.label),
            &arch_union(&pose),
            &pose,
            None,
        );
        for (name, base, _) in cones().into_iter().skip(1) {
            let u = apex_pyramid(&base, &pose, t());
            every_op_sound(
                &format!("{name}, {}", pose.label),
                &u,
                &pose,
                Some(pyramid_volumes(&base)),
            );
        }
    }
}

/// **The ring builds from every root**: each region of the link above
/// the top taken as the ring vertex ([`topo::test_support::with_ring_root`]),
/// on the arch and each cone at rest, every op sound ([`every_op_sound`]).
#[test]
fn every_root_of_the_ring_builds_in_every_op() {
    let rest = Pose::rest();
    let mut scenes = vec![("the arch".to_string(), arch_union(&rest), 3, None)];
    for (name, base, k) in cones() {
        let closed = Some(pyramid_volumes(&base));
        scenes.push((name.to_string(), apex_pyramid(&base, &rest, t()), k, closed));
    }
    for (name, u, k, closed) in &scenes {
        for root in 0..=*k {
            topo::test_support::with_ring_root(root, || {
                every_op_sound(
                    &format!("{name}, rooted at region {root}"),
                    u,
                    &rest,
                    *closed,
                );
            });
        }
    }
}

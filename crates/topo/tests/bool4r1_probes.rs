//! Reviewer probes for BOOL-4 (issue 750), lane r1.
//!
//! The arm's clear rests on one argument: "one witness decides a whole
//! instance", because the contained instance's interior lies in ONE
//! component of space minus the container's boundary whenever no
//! crossing finding stands against the pair. These rows attack that
//! argument with MULTI-SHELL instances — the shape the argument's
//! dichotomy disposes of by pointing at the reverse ordering.
//!
//! `probe_g` is the one that mattered: a container with a cavity and a
//! post threaded through it share 300 m^3 of material, every contact
//! between them is a true declared vertex-on-face rest, and at the
//! reviewed head the certifying door answered `Ok(())`. The arm now
//! probes every vertex of BOTH instances against the other's material
//! with no gate on the reverse ordering, so the container's own cavity
//! corners — inside the post — decide it; `probe_g` is inverted to that
//! verdict and `probe_a` widened to both orderings (fix pass).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, ContactRecords, SolidContainment, SolidKey, ValidationError, VfContact, VoidContainment,
    VoidEvidence, insert_void, point_in_solid_of, validate_pseudomanifold,
};

type Range = (f64, f64);

/// A brick hollowed by a strictly-interior brick as a VOID shell — one
/// solid, two shells, the unit's own `cavity()` recipe.
fn hollow_box(o: (Range, Range, Range), v: (Range, Range, Range)) -> Body<f64> {
    let mut dst = common::brick::<f64>(o.0, o.1, o.2, Tol::witness());
    let hole = common::brick::<f64>(v.0, v.1, v.2, Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = VoidEvidence {
        shells: hole
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    };
    insert_void(&mut dst, solid, hole, &evidence, Tol::witness()).unwrap();
    assert_eq!(dst.shells().count(), 2);
    dst
}

fn cube(lo: f64, hi: f64, vlo: f64, vhi: f64) -> Body<f64> {
    hollow_box(
        ((lo, hi), (lo, hi), (lo, hi)),
        ((vlo, vhi), (vlo, vhi), (vlo, vhi)),
    )
}

fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
    out
}

fn two_solids(body: &Body<f64>) -> (SolidKey, SolidKey) {
    let v: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    (v[0], v[1])
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Every vertex of the body whose point satisfies `pick`, in arena
/// order, with the per-solid door's verdict against `against`.
fn verdicts(
    body: &Body<f64>,
    against: SolidKey,
    pick: impl Fn(Point3<f64>) -> bool,
) -> Vec<(Point3<f64>, SolidContainment)> {
    body.vertices()
        .map(|(_, d)| *body.get_point(d.point).unwrap())
        .filter(|p| pick(*p))
        .map(|p| {
            (
                p,
                point_in_solid_of(body, against, p, band(), Tol::witness()).unwrap(),
            )
        })
        .collect()
}

/// `true` when every coordinate of `p` is one of `vals`.
fn coords_in(p: Point3<f64>, vals: &[f64]) -> bool {
    [p.x, p.y, p.z]
        .iter()
        .all(|c| vals.iter().any(|v| (c - v).abs() < 1e-12))
}

/// The arm-2 findings: a `CensusUndecidable` naming two SOLIDS, or an
/// `InstanceInterference`.
fn placement_findings(errors: &[ValidationError]) -> Vec<&ValidationError> {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: topo::EntityId::Solid(_),
                    b: topo::EntityId::Solid(_),
                    ..
                } | ValidationError::InstanceInterference { .. }
            )
        })
        .collect()
}

/// **Probe G — an interference fit that CERTIFIES.**
///
/// Container `m` = `[0,10]^3` less a `[3,7]^3` cavity. Part `u` = the
/// post `[2,8]x[2,8]x[0,10]` less a `[4,6]x[4,6]x[3.5,6.5]` cavity of
/// its own: a block spanning the container end to end, hollowed so it
/// owns vertices inside the container's cavity, and wide enough that
/// the container's whole cavity shell sits strictly inside its
/// material.
///
/// - **The materials genuinely overlap.** For `z` in `[0,3)` the post
///   and the container are both solid over `[2,8]x[2,8]` — about
///   300 m^3 of shared material, asserted below through the door
///   itself. This is an interference fit, the class
///   `InstanceInterference` exists to decide.
/// - **No boundary crosses another.** `u`'s end faces lie IN `m`'s
///   `z = 0` / `z = 10` faces (coplanar, strictly inside); nothing else
///   meets at all. The census pushes only `VertexOnFace` and
///   `EdgeFaceOverlap` TOUCHES — the L-bracket's own pattern — so arm
///   2's precondition passes.
/// - **`u`'s first sixteen vertices are `OnBoundary`** and skipped; the
///   seventeenth is a cavity-shell corner strictly `Out` of `m`'s
///   material, and the arm clears the ordering on it.
/// - **The reverse ordering never probes**: `m`'s hull is not inside
///   `u`'s reach box, so its extent gate clears for free.
///
/// Declaring the eight vertex-on-face rests — each geometrically true,
/// the L-bracket's own declaration shape — leaves the census nothing to
/// say about touches. What decides the pair is the REVERSE ordering:
/// the container's cavity-shell corners `(3, 3, 3)` etc. sit strictly
/// inside the post's material, so the arm reports
/// `InstanceInterference` with the post as `outer` and one of the
/// container's vertices as witness — declared and undeclared alike.
/// This row reds if the reverse ordering is ever gated or deleted.
#[test]
fn probe_g_a_post_through_the_container_is_the_reverse_ordering_s_interference() {
    let m = cube(0.0, 10.0, 3.0, 7.0);
    let u = hollow_box(
        ((2.0, 8.0), (2.0, 8.0), (0.0, 10.0)),
        ((4.0, 6.0), (4.0, 6.0), (3.5, 6.5)),
    );
    let body = assembly(&m, &u);
    let (ms, us) = two_solids(&body);

    // The overlap is real, read through the door under review.
    for q in [
        Point3::new(2.5, 2.5, 1.0),
        Point3::new(5.0, 5.0, 1.0),
        Point3::new(7.5, 7.5, 9.0),
    ] {
        assert_eq!(
            point_in_solid_of(&body, ms, q, band(), Tol::witness()).unwrap(),
            SolidContainment::In,
            "{q:?} in the container"
        );
        assert_eq!(
            point_in_solid_of(&body, us, q, band(), Tol::witness()).unwrap(),
            SolidContainment::In,
            "{q:?} in the post"
        );
    }

    // The post's own vertices: sixteen on the container's boundary,
    // then eight strictly outside its material. One instance, two
    // answers — which the arm's invariant says cannot happen without a
    // crossing.
    let post = verdicts(&body, ms, |p| {
        coords_in(p, &[4.0, 6.0, 3.5, 6.5])
            || (coords_in(p, &[2.0, 8.0, 0.0, 10.0]) && (1.9..8.1).contains(&p.x))
    });
    assert_eq!(post.len(), 16, "{post:?}");
    assert!(
        post[..8]
            .iter()
            .all(|(_, v)| *v == SolidContainment::OnBoundary),
        "{post:?}"
    );
    assert!(
        post[8..].iter().all(|(_, v)| *v == SolidContainment::Out),
        "{post:?}"
    );

    // The container's two end faces, by geometry.
    let end_face = |z: f64| {
        body.faces()
            .map(|(f, _)| f)
            .find(|&f| {
                let face = body.get_face(f).unwrap();
                if body.get_shell(face.shell).unwrap().solid != ms {
                    return false;
                }
                let topo::LoopBoundary::Cycle { first } =
                    body.get_loop(face.outer).unwrap().boundary
                else {
                    return false;
                };
                body.loop_cycle(first).unwrap().into_iter().all(|he| {
                    let v = body.get_half_edge(he).unwrap().start;
                    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    (p.z - z).abs() < 1e-12 && !(1.9..8.1).contains(&p.x)
                })
            })
            .unwrap()
    };
    let (bottom, top) = (end_face(0.0), end_face(10.0));
    let mut records = ContactRecords::default();
    for (v, d) in body.vertices() {
        let p = *body.get_point(d.point).unwrap();
        if !coords_in(p, &[2.0, 8.0, 0.0, 10.0]) || !(1.9..8.1).contains(&p.x) {
            continue;
        }
        records.b_on_a.push(VfContact {
            vertex: v,
            face: if p.z == 0.0 { bottom } else { top },
        });
    }
    assert_eq!(records.b_on_a.len(), 8, "the post's eight resting corners");

    // The witness the reverse ordering finds: a cavity corner of the
    // container, strictly inside the post.
    let is_reverse_interference = |e: &ValidationError| match e {
        ValidationError::InstanceInterference {
            outer,
            inner,
            witness,
        } => {
            let p = *body
                .get_point(body.get_vertex(*witness).unwrap().point)
                .unwrap();
            *outer == us && *inner == ms && coords_in(p, &[3.0, 7.0])
        }
        _ => false,
    };
    // Undeclared: the sixteen touches AND the decided interference.
    let undeclared =
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    let placements = placement_findings(&undeclared);
    assert_eq!(placements.len(), 1, "{undeclared:?}");
    assert!(is_reverse_interference(placements[0]), "{undeclared:?}");
    assert!(
        undeclared
            .iter()
            .filter(|e| !matches!(e, ValidationError::InstanceInterference { .. }))
            .all(|e| matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: topo::CensusContact::VertexOnFace { .. }
                        | topo::CensusContact::EdgeFaceOverlap { .. },
                    ..
                }
            )),
        "only touches beside it: {undeclared:?}"
    );

    // Declared, every record true: the interference fit is DECIDED,
    // and it is the only finding.
    let declared = validate_pseudomanifold(&body, &records, Tol::witness()).unwrap_err();
    assert_eq!(declared.len(), 1, "{declared:?}");
    assert!(is_reverse_interference(&declared[0]), "{declared:?}");
}

/// **Probe A — the reverse ordering is load-bearing.** `m` is
/// `[1,9]^3` less a `[4,6]^3` cavity; `u` is `[0,10]^3` less a
/// `[3,7]^3` cavity. The four boxes are strictly nested, so no face,
/// edge or vertex of one body meets the other and the census finds
/// NOTHING before arm 2; their materials nevertheless overlap.
///
/// `u`'s eight outer vertices — the first in its arena order — are
/// strictly OUT of `m` and its eight cavity corners are strictly IN,
/// so a first-`Out` clear would have missed it in the `m`-as-container
/// ordering; and `m`'s own outer corners are strictly in `u`, so the
/// reverse ordering decides it too. Both orderings report, each with
/// its own witness: this row reds if either ordering is deleted or if
/// the first vertex's `Out` is ever allowed to decide again.
#[test]
fn probe_a_both_orderings_decide_and_neither_clears_on_the_first_vertex() {
    let body = assembly(&cube(1.0, 9.0, 4.0, 6.0), &cube(0.0, 10.0, 3.0, 7.0));
    let (ms, us) = two_solids(&body);
    let u_seen = verdicts(&body, ms, |p| coords_in(p, &[0.0, 10.0, 3.0, 7.0]));
    assert_eq!(u_seen[0].1, SolidContainment::Out, "{u_seen:?}");
    assert_eq!(u_seen[8].1, SolidContainment::In, "{u_seen:?}");
    let errors =
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    let orderings: Vec<(SolidKey, SolidKey)> = errors
        .iter()
        .map(|e| match e {
            ValidationError::InstanceInterference { outer, inner, .. } => (*outer, *inner),
            other => panic!("only decided interferences: {other:?}"),
        })
        .collect();
    assert_eq!(orderings, vec![(ms, us), (us, ms)], "{errors:?}");
}

/// **Probe B — one instance, both answers, no crossing.** `m` is
/// `[0,10]^3` less a `[3,7]^3` cavity; `u` is `[2.5,7.5]^3` less a
/// `[3.5,6.5]^3` cavity, wrapped around the container's cavity shell.
/// Eight of `u`'s vertices are strictly IN `m`'s material and eight are
/// strictly OUT of it, with no boundary event anywhere in the body.
/// Every vertex is probed, so the verdict no longer depends on `u`'s
/// vertex ARENA ORDER: the outer shell comes first here and the void
/// shell first in `probe_g`'s post, and both are the interference.
#[test]
fn probe_b_one_instance_answers_both_ways() {
    let body = assembly(&cube(0.0, 10.0, 3.0, 7.0), &cube(2.5, 7.5, 3.5, 6.5));
    let (ms, _) = two_solids(&body);
    let seen = verdicts(&body, ms, |p| coords_in(p, &[2.5, 7.5, 3.5, 6.5]));
    assert_eq!(seen.len(), 16, "{seen:?}");
    assert!(
        seen[..8].iter().all(|(_, v)| *v == SolidContainment::In),
        "{seen:?}"
    );
    assert!(
        seen[8..].iter().all(|(_, v)| *v == SolidContainment::Out),
        "{seen:?}"
    );
    let errors =
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::InstanceInterference { .. })),
        "no boundary event anywhere: {errors:?}"
    );
}

/// **Probe C — a third solid's finding does not leak into a pair.**
/// The issue's L-bracket and its part, plus a third instance resting
/// on the bracket's outer wall. The precondition asks whether a
/// standing finding NAMES either solid of the pair, so the third
/// instance's own touches must not block the bracket × part pair —
/// which still clears.
#[test]
fn probe_c_a_third_solids_findings_do_not_block_a_pair() {
    let l = common::prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
        Tol::witness(),
    );
    let part = common::brick::<f64>((1.0, 2.0), (1.2, 2.0), (0.2, 0.8), Tol::witness());
    let third = common::brick::<f64>((0.5, 1.5), (-1.0, 0.0), (0.2, 0.8), Tol::witness());
    let mut body = assembly(&l.body, &part);
    topo::graft_disjoint(&mut body, &third, Tol::witness()).unwrap();
    let errors =
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    assert_eq!(errors.len(), 16, "{errors:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// **Probe D — a definitely-separated pair never reaches the material
/// test.** Two bricks a metre apart: one extent margin is definitely
/// negative, so the gate clears both orderings for free and no vertex
/// is probed. The "over-width costs a probe per vertex" sentence the PR
/// adds to `boxes.rs` needs a reach box inflated by a CURVED face to
/// bite, and such a pair is arm 1's first.
#[test]
fn probe_d_a_separated_planar_pair_clears_at_the_gate() {
    let a = common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let b = common::brick::<f64>((1.5, 2.5), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let body = assembly(&a, &b);
    let (sa, _) = two_solids(&body);
    let seen = verdicts(&body, sa, |p| p.x >= 1.5);
    assert!(
        seen.iter().all(|(_, v)| *v == SolidContainment::Out),
        "{seen:?}"
    );
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
}

/// **Probe E — the decided interference's Display is a sentence.**
#[test]
fn probe_e_display_is_a_sentence() {
    let e = ValidationError::InstanceInterference {
        outer: SolidKey::default(),
        inner: SolidKey::default(),
        witness: topo::VertexKey::default(),
    };
    let text = e.to_string();
    assert!(text.contains("interference fit"), "{text}");
    assert!(!text.contains("InstanceInterference"), "{text}");
}

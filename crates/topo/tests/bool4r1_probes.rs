//! Reviewer probes for BOOL-4 (issue 750), lane r1.
//!
//! These rows attack the arm's stated invariant — "one witness decides
//! a whole instance" — with multi-shell instances, the shape the
//! argument's dichotomy handles by pointing at the REVERSE ordering.
//! Nothing here is a fixture the unit owns; they exist to measure.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, ContactRecords, SolidContainment, SolidKey, ValidationError, VoidContainment,
    VoidEvidence, insert_void, point_in_solid_of, validate_pseudomanifold,
};

/// A box `[lo, hi]^3` hollowed by the box `[vlo, vhi]^3` as a VOID
/// shell — one solid, two shells, the `cavity()` recipe.
fn hollow(lo: f64, hi: f64, vlo: f64, vhi: f64) -> Body<f64> {
    let mut dst = common::brick::<f64>((lo, hi), (lo, hi), (lo, hi));
    let hole = common::brick::<f64>((vlo, vhi), (vlo, vhi), (vlo, vhi));
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

fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
    out
}

fn two_solids(body: &Body<f64>) -> (SolidKey, SolidKey) {
    let v: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    (v[0], v[1])
}

/// Every vertex of `solid`, in arena order, with the per-solid door's
/// verdict against `against`.
fn verdicts(body: &Body<f64>, against: SolidKey, solid: SolidKey) -> Vec<(Point3<f64>, String)> {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let owner = |v: topo::VertexKey| -> Option<SolidKey> {
        body.vertex_faces(v)
            .into_iter()
            .next()
            .and_then(|f| body.get_face(f))
            .and_then(|d| body.get_shell(d.shell))
            .map(|s| s.solid)
    };
    body.vertices()
        .filter(|(k, _)| owner(*k) == Some(solid))
        .map(|(_, d)| {
            let p = *body.get_point(d.point).unwrap();
            let verdict = match point_in_solid_of(body, against, p, band, tol) {
                Ok(v) => format!("{v:?}"),
                Err(e) => format!("Err({e:?})"),
            };
            (p, verdict)
        })
        .collect()
}

/// **Probe A — the interlocked hollow pair.** `m` is `[1,9]^3` less a
/// `[4,6]^3` void; `u` is `[0,10]^3` less a `[3,7]^3` void. The four
/// boxes are strictly nested, so no face, edge or vertex of one body
/// meets the other: the census finds NOTHING before arm 2. Their
/// MATERIALS nevertheless overlap (the shell `[1,3)`), so the pair is
/// a genuine interference.
///
/// `u`'s eight outer vertices — the first in its arena order — are
/// strictly OUTSIDE `m`'s material, so a single witness taken off `u`
/// would CLEAR the pair. The verdict therefore rests entirely on the
/// other ordering. This row measures both.
#[test]
fn probe_a_interlocked_hollow_pair() {
    let m = hollow(1.0, 9.0, 4.0, 6.0);
    let u = hollow(0.0, 10.0, 3.0, 7.0);
    let body = assembly(&m, &u);
    let (ms, us) = two_solids(&body);

    let u_seen = verdicts(&body, ms, us);
    let m_seen = verdicts(&body, us, ms);
    let first_u = u_seen.first().cloned().unwrap();
    let first_m = m_seen.first().cloned().unwrap();

    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness());
    panic!(
        "u-vertices vs m: {u_seen:?}\nm-vertices vs u: {m_seen:?}\nfirst u {first_u:?} first m {first_m:?}\nverdict: {errors:?}"
    );
}

/// **Probe B — a hollow part wrapped around the container's void.**
/// `m` is `[0,10]^3` less a `[3,7]^3` void; `u` is `[2.5,7.5]^3` less a
/// `[3.5,6.5]^3` void. Again no boundary meets any other. `u`'s OUTER
/// vertices are strictly inside `m`'s material and `u`'s VOID vertices
/// are strictly outside it — the same instance answers both ways, which
/// is what the arm's "one component of space minus the container's
/// boundary" claims cannot happen without a crossing. Here the reverse
/// ordering's BOX gate clears (`m`'s hull is not inside `u`'s reach),
/// so whatever arm 2 answers, it answers from `u`'s vertex arena order.
#[test]
fn probe_b_hollow_part_around_the_containers_void() {
    let m = hollow(0.0, 10.0, 3.0, 7.0);
    let u = hollow(2.5, 7.5, 3.5, 6.5);
    let body = assembly(&m, &u);
    let (ms, us) = two_solids(&body);
    let u_seen = verdicts(&body, ms, us);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness());
    panic!("u-vertices vs m: {u_seen:?}\nverdict: {errors:?}");
}

/// **Probe C — does a third solid's finding leak into an unrelated
/// pair?** `a` and `b` are the issue's clearing placement (a part in a
/// concavity); `c` is a cube pierced by nothing but carrying its own
/// undeclared touch against `a`. The precondition reads a prefix of
/// `errors` and asks whether a finding NAMES either solid of the pair,
/// so a finding on `a`×`c` must not block `a`×`b`.
#[test]
fn probe_c_a_third_solids_finding_and_the_pair() {
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
    );
    let part = common::brick::<f64>((1.0, 2.0), (1.2, 2.0), (0.2, 0.8));
    // A third instance sharing a face with the bracket's outer wall
    // `y = 0`: an undeclared vertex-on-face touch on `a`, nothing more.
    let third = common::brick::<f64>((0.5, 1.5), (-1.0, 0.0), (0.2, 0.8));
    let mut body = assembly(&l.body, &part);
    topo::graft_disjoint(&mut body, &third, Tol::witness()).unwrap();
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness());
    panic!("verdict: {errors:?}");
}

/// **Probe D — a separated pair with an over-wide reach box.** The PR
/// claims over-width now costs a probe per vertex instead of an answer.
/// A cylinder's reach box is a whole ball, so a box beside it is not
/// definitely separated; the material test must answer `Out` on the
/// FIRST vertex. This row measures that it clears, and names how many
/// vertices were asked (one, if the first answers).
#[test]
fn probe_d_over_wide_reach_box_still_clears() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let a = common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = common::brick::<f64>((1.5, 2.5), (0.0, 1.0), (0.0, 1.0));
    let body = assembly(&a, &b);
    let (sa, sb) = two_solids(&body);
    let seen = verdicts(&body, sa, sb);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), tol);
    assert!(matches!(
        point_in_solid_of(&body, sa, Point3::new(1.5, 0.0, 0.0), band, tol),
        Ok(SolidContainment::Out)
    ));
    panic!("b-vertices vs a: {seen:?}\nverdict: {errors:?}");
}

/// **Probe E — the arm's own refusal vocabulary, displayed.** Pins that
/// `InstanceInterference`'s Display sentence is asserted somewhere other
/// than the variant's own construction, and that the tag surface answers.
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

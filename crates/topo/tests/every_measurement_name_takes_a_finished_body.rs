//! **Every name of the measurement door takes a finished body** (Ev's
//! decision on PR 4822): `mass_properties`, `mass_properties_structural`,
//! `classify_shells`, `classify_shells_structural` and
//! `classify_shells_of` all read `&AtRestBody`. The bodies PR 5590's
//! review read through the three untyped siblings — the hollow box with
//! one shell flipped whole, and the box reverted by `Body::revert`, a
//! public op that turns a body inside out — are refused at
//! `AtRestBody::validate`, so none of the five reads them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::test_support::brick;
use topo::{AtRestBody, Body};

fn hollow(tol: Tol) -> AtRestBody<f64> {
    let outer =
        AtRestBody::validate(brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), tol), tol).unwrap();
    let inner =
        AtRestBody::validate(brick::<f64>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0), tol), tol).unwrap();
    let r = topo::boolean::subtract(&outer, &inner, tol).unwrap();
    r.body().expect("material").body.clone()
}

#[test]
fn the_reviews_flipped_shell_and_reverted_bodies_do_not_finish() {
    let tol = Tol::witness();
    let h = hollow(tol);
    assert_eq!(h.shells().count(), 2, "the hollow box has two shells");
    let mut refused = Vec::new();
    for (shell, s) in h.shells() {
        let flipped = s.faces.iter().fold(Body::clone(&h), |b, &k| {
            b.flipped_face_sense_for_tests(k).unwrap()
        });
        assert!(
            AtRestBody::validate(flipped, tol).is_err(),
            "{shell:?} flipped whole finishes"
        );
        refused.push(shell);
    }
    assert_eq!(refused.len(), 2, "each shell's flip is refused");
    let reverted = Body::clone(&h).revert().unwrap();
    assert!(
        AtRestBody::validate(reverted, tol).is_err(),
        "the reverted hollow box finishes"
    );
    // The finished body itself reads alike through every name.
    let every: Vec<_> = h.shells().map(|(k, _)| k).collect();
    let roles =
        |c: Vec<topo::ShellClassification<f64>>| c.iter().map(|c| c.role).collect::<Vec<_>>();
    let whole = roles(topo::classify_shells(&h, tol).unwrap());
    assert_eq!(
        roles(topo::classify_shells_of(&h, &every, tol).unwrap()),
        whole
    );
    assert_eq!(
        roles(topo::classify_shells_structural(&h, tol).unwrap()),
        whole
    );
    assert_eq!(
        topo::mass_properties_structural(&h, tol)
            .unwrap()
            .volume
            .to_bits(),
        topo::test_support::pinned_volume("the measured body", &h, tol)
            .volume
            .to_bits(),
        "a planar hollow box measures alike through the closed form"
    );
}

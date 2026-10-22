//! Review 5590 probes: the untyped public siblings of the typed door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::Tol;
use topo::test_support::brick;
use topo::{AtRestBody, Body, ShellKey};

fn hollow(tol: Tol) -> AtRestBody<f64> {
    let outer =
        AtRestBody::validate(brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), tol), tol).unwrap();
    let inner =
        AtRestBody::validate(brick::<f64>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0), tol), tol).unwrap();
    let r = topo::boolean::subtract(&outer, &inner, tol).unwrap();
    r.body().expect("material").body.clone()
}

/// P2's class through a public door that still takes `&Body`: the
/// hollow box with its inner shell flipped whole, which tier 3 refuses.
#[test]
fn review_5590_untyped_public_siblings_read_a_refused_body() {
    let tol = Tol::witness();
    let h = hollow(tol);
    assert_eq!(h.shells().count(), 2);
    let good = topo::mass_properties(&h, tol).unwrap().volume;
    eprintln!("hollow at the typed door: {good}");
    for (shell, s) in h.shells() {
        let fl = s.faces.iter().fold(Body::clone(&h), |b, &k| {
            b.flipped_face_sense_for_tests(k).unwrap()
        });
        let refused = AtRestBody::validate(fl.clone(), tol).err().map(|e| {
            format!(
                "{:?}",
                e.iter()
                    .map(|x| format!("{x:?}").chars().take(40).collect::<String>())
                    .collect::<Vec<_>>()
            )
        });
        let every: Vec<ShellKey> = fl.shells().map(|(k, _)| k).collect();
        let of = topo::classify_shells_of(&fl, &every, tol)
            .map(|c| c.iter().map(|c| c.role).collect::<Vec<_>>());
        let st = topo::classify_shells_structural(&fl, tol)
            .map(|c| c.iter().map(|c| c.role).collect::<Vec<_>>());
        let ms = topo::mass_properties_structural(&fl, tol).map(|m| m.volume);
        eprintln!(
            "{shell:?} flipped: validate {refused:?}\n  classify_shells_of {of:?}\n  classify_shells_structural {st:?}\n  mass_properties_structural {ms:?}"
        );
    }
    // Body::revert: a public, production op that makes an inside-out body.
    let rev = Body::clone(&h).revert().unwrap();
    eprintln!(
        "reverted: validate refuses {}; mass_properties_structural {:?}",
        AtRestBody::validate(rev.clone(), tol).is_err(),
        topo::mass_properties_structural(&rev, tol).map(|m| m.volume)
    );
}

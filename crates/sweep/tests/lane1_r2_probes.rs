//! LANE-1 R2 probes: the certified/`_structural` split at the shell
//! classification door, and the symbolic tier on every certified door.
//!
//! Two things the unit's own rows do not pin. `classify_shells` is a
//! certified door with a `_structural` twin exactly as `mass_properties`
//! is, and nothing walks the two over a body that separates them; and
//! the certified names at `Sym<f64>` — the wrapper the deleted trait's
//! `Sym` impl existed to keep certifying — are instantiated only for
//! `validate_pseudomanifold` (`topo/tests/lane0_r2_probes.rs`), not for
//! the measurement doors or the marks pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cert_corpus::corpus;
use geom_core::{Bounds, Sym, Tol};
use topo::{Body, ContactRecords};

/// `body` finished, or `None` for one of the corpus's inside-out
/// `~reverted` twins, the one refusal (`NegativeVolume`) it holds. A
/// finished body reads the typed doors; a twin reads the raw ones.
fn finish<T>(name: &str, body: &Body<T>, tol: Tol) -> Option<topo::AtRestBody<T>>
where
    T: geom_core::Decide + geom_core::CertifiedBounds + topo::AtRestPolicy,
{
    match topo::AtRestBody::validate(Body::clone(body), tol) {
        Ok(finished) => Some(finished),
        Err(errors) => {
            assert!(
                name.ends_with("~reverted")
                    && errors
                        .iter()
                        .all(|e| matches!(e, topo::ValidationError::NegativeVolume { .. })),
                "{name}: tier 3 refuses a corpus body that is not an inside-out twin: {errors:?}"
            );
            None
        }
    }
}

/// **The shell door's split, body by body.** On every closed-form body
/// the two doors classify identically (same shells, same roles, same
/// volume bits); on the oblique-cut cylinder, whose ellipse-trimmed face
/// needs the quadrature, the certified door classifies and the
/// `_structural` twin refuses typed at the props layer — the same line
/// `mass_properties` / `mass_properties_structural` draw, at the door the
/// editor's connectedness resident and the shell verbs measure through.
#[test]
fn classify_shells_structural_is_the_closed_form_and_refuses_where_the_lane_would_enclose() {
    let tol = Tol::witness();
    let mut refused = Vec::new();
    for (name, body) in corpus::<f64>() {
        let (certified, structural) = match finish(&name, &body, tol) {
            Some(finished) => (
                topo::classify_shells(&finished, tol),
                topo::classify_shells_structural(&finished, tol),
            ),
            // Raw door: an inside-out `~reverted` twin, which tier 3 refuses (NegativeVolume); both lanes read it.
            None => (
                topo::test_support::classify_unvalidated(&body, tol),
                // Raw door: the structural lane over the same inside-out twin.
                topo::test_support::classify_unvalidated_structural(&body, tol),
            ),
        };
        match (&certified, &structural) {
            (Ok(c), Ok(s)) => {
                assert_eq!(c.len(), s.len(), "{name}: shell count");
                for (a, b) in c.iter().zip(s) {
                    assert_eq!(a.shell, b.shell, "{name}: shell key");
                    assert_eq!(a.solid, b.solid, "{name}: solid key");
                    assert_eq!(
                        (
                            a.enclosure.volume.to_bits(),
                            a.enclosure.surface_area.to_bits()
                        ),
                        (
                            b.enclosure.volume.to_bits(),
                            b.enclosure.surface_area.to_bits()
                        ),
                        "{name}: the closed form is one computation at both doors"
                    );
                    assert_eq!(
                        (
                            a.enclosure.volume_pad.to_bits(),
                            a.enclosure.area_pad.to_bits()
                        ),
                        (
                            b.enclosure.volume_pad.to_bits(),
                            b.enclosure.area_pad.to_bits()
                        ),
                        "{name}: a closed-form shell carries pads of 0 at both doors"
                    );
                }
            }
            (Ok(_), Err(topo::ShellClassifyError::Props { .. })) => refused.push(name.clone()),
            (Ok(_), Err(other)) => panic!("{name}: the structural door refused with {other:?}"),
            (Err(e), _) => panic!("{name}: the certified door refused a corpus body: {e:?}"),
        }
    }
    let mut expect = vec![
        "cut_cylinder_above".to_string(),
        "cut_cylinder_below".to_string(),
        "cut_cylinder_above~reverted".to_string(),
        "cut_cylinder_below~reverted".to_string(),
    ];
    refused.sort();
    expect.sort();
    assert_eq!(
        refused, expect,
        "exactly the ellipse-trimmed bodies separate the two shell doors"
    );
}

/// **Every certified door forms at `Sym<f64>` and answers the base
/// scalar's bits.** The deleted trait's `Sym` impl carried the argument
/// that wrapping a certifying scalar must not demote it; after the fold
/// that argument is the `impl<T: Decide + CertifiedBounds> QuadLane<T>`
/// block alone, and this row is the four doors written at the tier.
#[test]
fn the_certified_doors_form_at_the_symbolic_tier_and_answer_the_base_bits() {
    let tol = Tol::witness();
    let base = corpus::<f64>();
    let sym = corpus::<Sym<f64>>();
    assert_eq!(base.len(), sym.len(), "the corpus builds at both scalars");
    for ((name, b), (sname, s)) in base.iter().zip(&sym) {
        assert_eq!(name, sname);
        let pm_base = topo::validate_pseudomanifold(b, &ContactRecords::default(), tol);
        let pm_sym = topo::validate_pseudomanifold(s, &ContactRecords::default(), tol);
        assert_eq!(pm_base.is_ok(), pm_sym.is_ok(), "{name}: tier-3' verdict");
        assert_eq!(
            topo::contact_marks(b, tol).is_ok(),
            topo::contact_marks(s, tol).is_ok(),
            "{name}: marks verdict"
        );
        let (fb, fs) = (finish(name, b, tol), finish(name, s, tol));
        assert_eq!(
            fb.is_some(),
            fs.is_some(),
            "{name}: tier 3's verdict at both scalars"
        );
        let (mp_base, mp_sym, cs_base, cs_sym) = match (&fb, &fs) {
            (Some(fb), Some(fs)) => (
                topo::mass_properties(fb, tol, topo::test_support::pinned_resolution(tol)),
                topo::mass_properties(fs, tol, topo::test_support::pinned_resolution(tol)),
                topo::classify_shells(fb, tol).is_ok(),
                topo::classify_shells(fs, tol).is_ok(),
            ),
            // Raw door: an inside-out `~reverted` twin at both scalars, which tier 3 refuses (NegativeVolume).
            _ => (
                topo::test_support::measure_unvalidated(
                    b,
                    tol,
                    topo::test_support::pinned_resolution(tol),
                ),
                // Raw door: the symbolic twin of the same inside-out body.
                topo::test_support::measure_unvalidated(
                    s,
                    tol,
                    topo::test_support::pinned_resolution(tol),
                ),
                // Raw door: the same inside-out body at f64.
                topo::test_support::classify_unvalidated(b, tol).is_ok(),
                // Raw door: the same inside-out body at the symbolic tier.
                topo::test_support::classify_unvalidated(s, tol).is_ok(),
            ),
        };
        match (mp_base, mp_sym) {
            (Ok(x), Ok(y)) => assert_eq!(
                x.enclosure.volume.to_bits(),
                Bounds::lo(y.enclosure.volume).to_bits(),
                "{name}: the tier's volume is the base scalar's"
            ),
            (Err(_), Err(_)) => {}
            (x, y) => panic!(
                "{name}: the two scalars split on the measurement: base ok={} sym ok={}",
                x.is_ok(),
                y.is_ok()
            ),
        }
        assert_eq!(cs_base, cs_sym, "{name}: shell verdict");
    }
    let _: Option<&Body<Sym<f64>>> = sym.first().map(|(_, b)| b);
}

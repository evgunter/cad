//! **The side reader's exact ties on trimmed-sphere bodies**
//! (`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`):
//! recorded `Sym<f64>` builds of trimmed-sphere bodies at symmetric and
//! generic poses, and every decision under `topo::SIDE_READER_FAMILY`
//! whose margin is exactly 0. No corpus document casts a sphere aim, so
//! `revalue_corpus::reader_ties_at_the_nominal` cannot see these.
//! Evidence, not a gate: run with
//! `cargo nextest run -p sweep --run-ignored only -E 'test(sphere_aims_reader_ties)' --no-capture`.

#![allow(clippy::print_stdout, clippy::unwrap_used)]

use geom_core::Real as _;
use geom_core::sym::{Sym, SymBudget, revalue::record, with_session};
use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::AtRestBody;

type S = Sym<f64>;

/// A named body build, returning its shell count.
type Case<'a> = (&'static str, Box<dyn Fn() -> usize + 'a>);

#[test]
#[ignore = "evidence: the side reader's exact ties on trimmed-sphere bodies"]
fn sphere_aims_reader_ties() {
    let tol = Tol::witness();
    let v = |x: f64, y: f64, z: f64| Vec3::new(S::from_f64(x), S::from_f64(y), S::from_f64(z));
    let ball = move |r: f64, c: Vec3<S>| -> AtRestBody<S> {
        let at = ball_poled_y(S::from_f64(r), v(0.0, 0.0, 0.0), tol);
        let m = topo::transform_rigid(&at, &geom_core::RigidMotion::translation(c).unwrap(), tol)
            .unwrap();
        finished("ball", m, tol)
    };
    let shells = |r: Result<topo::BooleanResult<S>, topo::BooleanError>| {
        r.map(|r| {
            r.body().map_or(0, |b| {
                topo::classify_shells_structural(&b.body, tol).map_or(0, |c| c.len())
            })
        })
        .unwrap_or(99)
    };
    let sym_lens = move || {
        topo::intersect(
            &ball(1.0, v(0.0, 0.0, -0.5)),
            &ball(1.0, v(0.0, 0.0, 0.5)),
            tol,
        )
    };
    let cases: Vec<Case<'_>> = vec![
        ("sym lens", Box::new(move || shells(sym_lens()))),
        (
            "sym bite",
            Box::new(move || {
                shells(topo::subtract(
                    &ball(1.0, v(0.0, 0.0, -0.5)),
                    &ball(1.0, v(0.0, 0.0, 0.5)),
                    tol,
                ))
            }),
        ),
        (
            "lens",
            Box::new(move || {
                shells(topo::intersect(
                    &ball(1.0, v(0.0, 0.0, 0.0)),
                    &ball(0.8, v(0.0, 0.0, 0.9)),
                    tol,
                ))
            }),
        ),
        (
            "block less sym lens",
            Box::new(move || {
                let Some(lens) = sym_lens()
                    .ok()
                    .and_then(|r| r.body().map(|b| b.body.clone()))
                else {
                    return 98;
                };
                let block = finished(
                    "block",
                    brick::<S>((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol),
                    tol,
                );
                shells(topo::subtract(&block, &lens, tol))
            }),
        ),
    ];
    for (what, f) in cases {
        let ((shells, decisions), _) = with_session(
            SymBudget {
                max_terms: 4096,
                max_degree: 128,
            },
            || record(&f),
        );
        let family: Vec<_> = decisions
            .iter()
            .enumerate()
            .filter(|(_, d)| d.family == Some(topo::SIDE_READER_FAMILY))
            .collect();
        let ties: Vec<_> = family
            .iter()
            .filter(|(_, d)| d.value == Some(0.0))
            .map(|(k, d)| (k + 1, d.predicate))
            .collect();
        println!(
            "SPHERE-TIES eps={:e} {what}: shells {shells}, {} decisions, {} the reader's, {} exact ties {ties:?}",
            tol.eps(),
            decisions.len(),
            family.len(),
            ties.len()
        );
    }
}

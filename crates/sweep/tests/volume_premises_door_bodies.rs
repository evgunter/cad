//! **The walk's own sums stand for the closed form run again on bodies
//! the doors build, on moved copies of them, and under a kept
//! certificate** (`topo::props`' `walked_closed_form`), and a wall whose
//! radius is not its rims' never reads a volume it does not have. The
//! review rows of `volume_premises_over_a_box.rs`'s change, kept; each
//! prints what it measured.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::cell::Cell;

use geom_core::{
    CertifiedBounds, Interval, ParamSymbol, Point2, Real, RigidMotion, Sym, SymBudget, Tol,
    UnitVec3, Vec3,
};
use sweep::Revolution;
use sweep::test_support::{ball_poled_y, corners, revolved_about_y_at, waisted_at};
use topo::test_support::{
    Stored, closed_form_reads, restate_geometry, seamless_drum, volume_role_reading,
};
use topo::{Body, CertifiedPolicy};

type Boxed = Sym<Interval>;

const R: f64 = 0.8125;

fn near<T: CertifiedBounds>(x: T, key: f64) -> bool {
    (x.lo() - key).abs() <= 1e-12 && (x.hi() - key).abs() <= 1e-12
}

type Restated<T> = (Option<Stored>, f64, T);

fn restated<T: Real + CertifiedBounds>(
    mut body: Body<T>,
    table: &[Restated<T>],
) -> (Body<T>, usize) {
    let hits = Cell::new(0usize);
    restate_geometry(&mut body, |arena, x| {
        for &(only, key, to) in table {
            if only.is_none_or(|a| a == arena) && near(x, key) {
                hits.set(hits.get() + 1);
                return to;
            }
        }
        x
    });
    (body, hits.get())
}

const BUDGET: SymBudget = SymBudget {
    max_terms: 64,
    max_degree: 8,
};

fn param_at<T: Real>(name: &str, value: T, lo: f64, hi: f64) -> Sym<T> {
    if lo == hi {
        return Sym::<T>::from_f64(lo);
    }
    Sym::param_over(ParamSymbol::new(test_utils::symbol_id(name)), value, lo, hi)
}

fn eager<Out>(read: impl FnOnce() -> Out) -> Out {
    geom_core::sym::with_session(BUDGET, read).0
}

fn deferring<Out>(read: impl FnOnce() -> Out) -> Out {
    geom_core::sym::deferred::with_deferred_session(
        BUDGET,
        geom_core::SymRules::shipped(),
        geom_core::SymRetry::none(),
        read,
    )
    .0
}

/// Bodies the revolve door builds at an exact parameter.
fn door_bodies<T: CertifiedPolicy + CertifiedBounds>(tol: Tol) -> Vec<(&'static str, Body<T>)> {
    let c = T::from_f64;
    let at = |x: f64, y: f64| (Point2::new(c(x), c(y)), c(0.0));
    vec![
        (
            "ruled cylinder",
            revolved_about_y_at::<T>(
                vec![at(0.0, 0.0), at(R, 0.0), at(R, 1.0), at(0.0, 1.0)],
                Revolution::Full,
                tol,
            ),
        ),
        (
            "frustum",
            revolved_about_y_at::<T>(
                vec![at(0.0, 0.0), at(1.0, 0.0), at(0.5, 1.0), at(0.0, 1.0)],
                Revolution::Full,
                tol,
            ),
        ),
        ("waisted", waisted_at::<T>(tol)),
        (
            "torus",
            revolved_about_y_at::<T>(
                vec![
                    (Point2::new(c(2.0 - R), c(0.0)), c(1.0)),
                    (Point2::new(c(2.0 + R), c(0.0)), c(1.0)),
                ],
                Revolution::Full,
                tol,
            ),
        ),
        (
            "ball",
            ball_poled_y::<T>(c(R), Vec3::new(c(0.25), c(3.0), c(-1.5)), tol),
        ),
        (
            "tube",
            revolved_about_y_at::<T>(
                corners(&[(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)]),
                Revolution::Full,
                tol,
            ),
        ),
    ]
}

fn tilted<T: CertifiedPolicy + CertifiedBounds>(body: &Body<T>, tol: Tol) -> Body<T> {
    let c = T::from_f64;
    let axis = UnitVec3::new(
        Vec3::new(c(1.0), c(2.0), c(3.0)),
        "review_probe_axis",
        geom_core::Band::linear(tol).unwrap(),
    )
    .unwrap();
    let motion =
        RigidMotion::rotation_then_translation(axis, c(0.3), Vec3::new(c(5.0), c(-3.0), c(2.0)))
            .unwrap();
    topo::transform_rigid(body, &motion).unwrap()
}

/// How the two readings of every curved closed-form face compare:
/// `(faces, bit-equal, lifted refused, differ)`, each difference printed.
fn compare<T: CertifiedPolicy + CertifiedBounds>(
    what: &str,
    body: &Body<T>,
    tol: Tol,
) -> (usize, usize, usize, usize) {
    let reads = match closed_form_reads(body, tol) {
        Ok(reads) => reads,
        Err(e) => {
            println!("  {what}: the walk refuses: {e:?}");
            return (0, 0, 0, 0);
        }
    };
    let (mut same, mut refused, mut differ) = (0, 0, 0);
    for read in &reads {
        let Some(walked) = read.walked else {
            println!("  {what} {:?}: no walked read", read.face);
            continue;
        };
        match &read.lifted {
            Err(e) => {
                refused += 1;
                println!("  {what} {:?}: lifted refuses {e:?}", read.face);
            }
            Ok(lifted) => {
                let eq = walked
                    .iter()
                    .zip(lifted)
                    .all(|(w, l)| w.0.to_bits() == l.0.to_bits() && w.1.to_bits() == l.1.to_bits());
                if eq {
                    same += 1;
                } else {
                    differ += 1;
                    println!(
                        "  {what} {:?}: DIFFER walked {walked:?} lifted {lifted:?}",
                        read.face
                    );
                }
            }
        }
    }
    println!(
        "  {what}: {} faces, {same} bit-equal, {refused} lifted-refused, {differ} differ",
        reads.len()
    );
    (reads.len(), same, refused, differ)
}

/// **Bit-equality on door-built bodies and on moved copies of them.**
#[test]
fn walked_and_lifted_agree_bitwise_on_door_built_bodies_and_moved_copies() {
    let tol = Tol::witness();
    let mut differ = 0;
    let mut faces = 0;
    println!("Interval, as built:");
    for (name, body) in door_bodies::<Interval>(tol) {
        let (n, _, _, d) = compare(name, &body, tol);
        faces += n;
        differ += d;
    }
    println!("Interval, moved (rotation 0.3 about (1,2,3), then a translation):");
    for (name, body) in door_bodies::<Interval>(tol) {
        let (_, _, _, d) = compare(name, &tilted(&body, tol), tol);
        differ += d;
    }
    eager(|| {
        println!("Sym<Interval>, as built:");
        for (name, body) in door_bodies::<Boxed>(tol) {
            let (_, _, _, d) = compare(name, &body, tol);
            differ += d;
        }
        println!("Sym<Interval>, moved:");
        for (name, body) in door_bodies::<Boxed>(tol) {
            let (_, _, _, d) = compare(name, &tilted(&body, tol), tol);
            differ += d;
        }
    });
    assert!(faces > 0, "no curved closed-form face was read");
    assert_eq!(differ, 0, "walked and lifted differ where both read");
}

/// **A moved copy's volume, read off the image's own walk, against
/// the unmoved body's**, at the interval scalar: what the re-derivation
/// reads of a body whose geometry a rotation widened.
#[test]
fn a_moved_copys_volume_meets_the_bodys_at_the_interval_scalar() {
    let tol = Tol::witness();
    for (name, body) in door_bodies::<Interval>(tol) {
        let unmoved = volume_role_reading(&body, tol);
        let moved = volume_role_reading(&tilted(&body, tol), tol);
        println!("{name}: unmoved {unmoved:?}\n    moved {moved:?}");
        if let (Ok((_, (lo, hi))), Ok((_, (mlo, mhi)))) = (&unmoved, &moved) {
            assert!(
                mlo <= hi && lo <= mhi,
                "{name}: the moved copy's volume [{mlo}, {mhi}] misses the body's [{lo}, {hi}]"
            );
        }
    }
}

fn outcome(read: Result<(Option<topo::ShellRole>, (f64, f64)), topo::MassPropsError>) -> String {
    match read {
        Ok((role, (lo, hi))) => format!("READS {role:?} [{lo:.9}, {hi:.9}]"),
        Err(e) => {
            let text = format!("{e:?}");
            let kind = ["Escalated", "OffSurface", "NotIsoRectangle", "Corrupt"]
                .into_iter()
                .find(|k| text.contains(k))
                .unwrap_or("other");
            let name = text
                .split('"')
                .find(|s| s.starts_with("props_"))
                .unwrap_or("?")
                .to_owned();
            format!("refused {kind} {name}")
        }
    }
}

/// The drum whose wall's radius is written `delta` off its rims'.
fn misfit<T: CertifiedPolicy + CertifiedBounds>(rims: T, delta: f64, tol: Tol) -> Body<T> {
    let (body, hits) = restated(
        seamless_drum::<T>(R, 1.0, tol),
        &[
            (Some(Stored::Surface), R, rims + T::from_f64(delta)),
            (None, R, rims),
            (None, -R, -rims),
        ],
    );
    assert!(hits > 0);
    body
}

/// **A wall whose radius is not its rims', by an ulp, inside the
/// band, at its edge, past it and by a thousandth**, at every scalar,
/// eager and deferring, exact and boxed. The true volume of the body the
/// rims bound is `π r² h` at the rims' radius and at the wall's; a
/// reading is checked against the hull of the two.
#[test]
fn a_misfit_wall_by_every_amount_reads_no_volume_it_lacks() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let ulp = R * f64::EPSILON;
    let amounts = [
        ("0", 0.0),
        ("1 ulp", ulp),
        ("eps/4", 0.25 * eps),
        ("eps", eps),
        ("4 eps", 4.0 * eps),
        ("64 eps", 64.0 * eps),
        ("1e-6", 1e-6),
        ("1e-3", 1e-3),
    ];
    let truth = |lo_r: f64, hi_r: f64, delta: f64| {
        let v = |r: f64| core::f64::consts::PI * r * r;
        (v(lo_r.min(lo_r + delta)), v(hi_r.max(hi_r + delta)))
    };
    let mut unsound = Vec::new();
    let mut check =
        |what: String,
         (lo_r, hi_r): (f64, f64),
         delta: f64,
         read: Result<(Option<topo::ShellRole>, (f64, f64)), topo::MassPropsError>| {
            if let Ok((_, (lo, hi))) = &read {
                let (t_lo, t_hi) = truth(lo_r, hi_r, delta);
                // The reading must meet the hull of the volumes the two radii
                // give, at some member.
                if *hi < t_lo * (1.0 - 1e-12) || *lo > t_hi * (1.0 + 1e-12) {
                    unsound.push(format!("{what}: [{lo}, {hi}] misses [{t_lo}, {t_hi}]"));
                }
                // And a misfit past the band must not read at all.
                if delta.abs() > 100.0 * eps {
                    unsound.push(format!("{what}: a misfit of {delta} reads a volume"));
                }
            }
            println!("  {what:<44} {}", outcome(read));
        };
    for (label, delta) in amounts {
        println!("misfit {label}:");
        check(
            "f64".into(),
            (R, R),
            delta,
            volume_role_reading(&misfit::<f64>(R, delta, tol), tol),
        );
        check(
            "Interval exact".into(),
            (R, R),
            delta,
            volume_role_reading(&misfit::<Interval>(Interval::from_f64(R), delta, tol), tol),
        );
        for half in [0.125 * eps, 0.05] {
            check(
                format!("Interval box ±{half:e}"),
                (R - half, R + half),
                delta,
                volume_role_reading(
                    &misfit::<Interval>(Interval::from_bounds(R - half, R + half), delta, tol),
                    tol,
                ),
            );
        }
        for (mode, defer) in [("eager", false), ("deferring", true)] {
            let run = |f: &mut dyn FnMut()| {
                if defer {
                    deferring(&mut *f)
                } else {
                    eager(&mut *f)
                }
            };
            run(&mut || {
                check(
                    format!("Sym<f64> exact {mode}"),
                    (R, R),
                    delta,
                    volume_role_reading(
                        &misfit::<Sym<f64>>(Sym::<f64>::from_f64(R), delta, tol),
                        tol,
                    ),
                );
            });
            run(&mut || {
                check(
                    format!("Sym<f64> param {mode}"),
                    (R, R),
                    delta,
                    volume_role_reading(
                        &misfit::<Sym<f64>>(param_at("r", R, R - 0.05, R + 0.05), delta, tol),
                        tol,
                    ),
                );
            });
            run(&mut || {
                check(
                    format!("Sym<Interval> exact {mode}"),
                    (R, R),
                    delta,
                    volume_role_reading(&misfit::<Boxed>(Boxed::from_f64(R), delta, tol), tol),
                );
            });
            for half in [0.125 * eps, 0.05] {
                run(&mut || {
                    check(
                        format!("Sym<Interval> box ±{half:e} {mode}"),
                        (R - half, R + half),
                        delta,
                        volume_role_reading(
                            &misfit::<Boxed>(
                                param_at(
                                    "r",
                                    Interval::from_bounds(R - half, R + half),
                                    R - half,
                                    R + half,
                                ),
                                delta,
                                tol,
                            ),
                            tol,
                        ),
                    );
                });
            }
        }
    }
    assert!(unsound.is_empty(), "{unsound:#?}");
}

/// **The plain interval scalar over a box**: at every width the walk
/// refuses or reads, the lifted run does the same on every curved face,
/// so the arm is reached there only with premises the value channel
/// decided.
#[test]
fn the_interval_scalar_over_a_box_reads_only_where_the_lifted_run_reads() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for half in [
        0.0,
        0.125 * eps,
        0.5 * eps,
        2.0 * eps,
        32.0 * eps,
        1e-6,
        0.05,
    ] {
        let (body, _) = restated(
            seamless_drum::<Interval>(R, 1.0, tol),
            &[
                (None, R, Interval::from_bounds(R - half, R + half)),
                (None, -R, Interval::from_bounds(-R - half, -R + half)),
            ],
        );
        match closed_form_reads(&body, tol) {
            Err(e) => println!("±{half:e}: the walk refuses: {}", outcome(Err(e))),
            Ok(reads) => {
                for read in &reads {
                    println!(
                        "±{half:e}: walked {:?} lifted ok {}",
                        read.walked.is_some(),
                        read.lifted.is_ok()
                    );
                    assert!(
                        read.lifted.is_ok(),
                        "±{half:e}: the walk read a face the lifted run refuses: {:?}",
                        read.lifted
                    );
                }
            }
        }
    }
}

/// **A finished body's kept certificate under a move**
/// (`KeptSign::moved`): the moved copy's volume is settled through the
/// kept runs, which at an exact-witness scalar are now what a curved
/// closed-form face's term is read from. A claim holding the true
/// volume holds, one that excludes it does not, and the sign is read.
fn kept_under_a_move<T>(scalar: &str, tol: Tol)
where
    T: CertifiedPolicy + CertifiedBounds + topo::AtRestPolicy<Finished = topo::AtRestBody<T>>,
{
    use topo::FinishedBody as _;
    let c = T::from_f64;
    let band = geom_core::Band::linear(tol).unwrap();
    let axis = UnitVec3::new(
        Vec3::new(c(1.0), c(-2.0), c(0.5)),
        "review_probe_turn",
        band,
    )
    .unwrap();
    let motion = RigidMotion::rotation(axis, c(0.7))
        .unwrap()
        .translated(Vec3::new(c(3.0), c(-1.25), c(0.5)))
        .unwrap();
    let truths: Vec<f64> = door_bodies::<f64>(tol)
        .iter()
        .map(|(_, body)| {
            let (_, (lo, hi)) = volume_role_reading(body, tol).unwrap();
            0.5 * (lo + hi)
        })
        .collect();
    let mut settled_bodies = 0;
    for ((name, body), v) in door_bodies::<T>(tol).into_iter().zip(truths) {
        let finished = match topo::finish(topo::Built::of(body), tol) {
            Ok(finished) => finished,
            Err(e) => {
                println!("{scalar} {name}: the finish refuses: {e}");
                continue;
            }
        };
        let moved = match finished.moved(&motion, tol) {
            Ok(moved) => moved,
            Err(e) => {
                println!("{scalar} {name}: the move refuses: {e}");
                continue;
            }
        };
        let claim = |lo: f64, hi: f64| topo::Settle::Contains {
            lo: c(lo),
            hi: c(hi),
            name: "review_probe_contains",
        };
        let holds = moved.measure(claim(v * (1.0 - 1e-4), v * (1.0 + 1e-4)));
        let misses = moved.measure(claim(v * 1.001, v * 1.01));
        let below = moved.measure(claim(v * 0.9, v * 0.999));
        println!(
            "{scalar} {name}: true claim {:?}; above {:?}; below {:?}",
            holds.as_ref().map(|m| m.stop),
            misses.as_ref().map(|m| m.stop),
            below.as_ref().map(|m| m.stop)
        );
        let settled = |m: &Result<topo::MassMeasurement<T>, topo::MassPropsError>, want: bool| {
            matches!(
                m.as_ref().map(|m| m.stop),
                Ok(topo::Stop::Settled(topo::SettleVerdict::Holds(got))) if got == want
            )
        };
        assert!(
            settled(&holds, true),
            "{scalar} {name}: the true claim does not hold"
        );
        assert!(
            settled(&misses, false),
            "{scalar} {name}: a claim above the volume holds"
        );
        assert!(
            settled(&below, false),
            "{scalar} {name}: a claim below the volume holds"
        );
        settled_bodies += 1;
    }
    assert!(
        settled_bodies > 0,
        "{scalar}: no body was finished and moved"
    );
}

#[test]
fn a_kept_certificate_under_a_move_settles_the_images_volume() {
    let tol = Tol::witness();
    kept_under_a_move::<f64>("f64", tol);
    kept_under_a_move::<Interval>("Interval", tol);
    eager(|| kept_under_a_move::<Boxed>("Sym<Interval>", tol));
}

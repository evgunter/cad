//! **R1 review probes for M10-P** — independent derivations, not diff
//! re-readings, at the `profile` crate's own doors.
//!
//! Every fixture here is a static witness (a shape you can write down),
//! so per `memories/test-suite-cost.md` no row draws a seed. Rows
//! marked EVIDENCE-ONLY exist to record a reviewed behavior for the
//! review's ledger; they assert real facts and can go red, but their
//! value is the record, not the gate.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{p2, tol};
use geom_core::Tol;
use profile::{
    ArcSweep, Center, Decision, Open, PathError, ProfileLoop, ReplayErrorKind, ReplayStructure,
    Start, StructureRefusalKind, replay, replay_guided, replay_recording,
};

fn s3() -> f64 {
    3.0_f64.sqrt()
}

/// R1's own lens, derived independently of the unit's fixture: the
/// vesica of the two radius-4 circles about (±2, 0) — the same
/// configuration CLASS at double scale, so every derived quantity
/// (corner, setback, fit) sits at different bits — entry anchor at
/// (0, −2√3), one fused `ArcFilletArc`, with the incoming lobe's
/// centre displaced by `dx`.
fn r1_lens(dx: f64, radius: f64) -> Vec<profile::Step<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-2.0 + dx, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -2.0 * s3()),
        },
        radius,
        Center {
            c: p2(2.0, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        Tol::witness(),
    )
    .expect("the lens constructs")
    .program
}

fn same_bits(a: &ProfileLoop<f64>, b: &ProfileLoop<f64>, what: &str) {
    assert_eq!(a.vertices().len(), b.vertices().len(), "{what}: arity");
    for (i, (u, v)) in a.vertices().iter().zip(b.vertices()).enumerate() {
        assert_eq!(u.pos().x.to_bits(), v.pos().x.to_bits(), "{what} v{i} x");
        assert_eq!(u.pos().y.to_bits(), v.pos().y.to_bits(), "{what} v{i} y");
        assert_eq!(u.bulge().to_bits(), v.bulge().to_bits(), "{what} v{i} b");
    }
    assert_eq!(a.tangent_joints(), b.tangent_joints(), "{what}: joints");
}

/// Adversarial programs of R1's own: fillets at extreme radii, a
/// reflex (>π) arc leg, and a near-tangent junction. Guided replay at
/// `f64` must reproduce plain replay bitwise on ALL of them — claim 2
/// beyond the unit's corpus.
fn adversarial_programs() -> Vec<(&'static str, Vec<profile::Step<f64>>)> {
    use profile::{ArcSide, Sweep};
    let w = Tol::witness();
    let mut out: Vec<(&'static str, Vec<profile::Step<f64>>)> = Vec::new();

    // 1. A seam-filleted square with an EXTREME-SMALL radius: the
    //    corner construction runs at the edge of the exact band.
    let tiny = Open
        .at(p2(1.5, 0.0))
        .angle(0.0, w)
        .unwrap()
        .fillet(1e-6, w)
        .unwrap()
        .at(p2(3.0, 1.5), w)
        .unwrap()
        .angle(std::f64::consts::FRAC_PI_2, w)
        .unwrap()
        .fillet(1e-6, w)
        .unwrap()
        .at(p2(1.5, 3.0), w)
        .unwrap()
        .angle(std::f64::consts::PI, w)
        .unwrap()
        .fillet(1e-6, w)
        .unwrap()
        .at(p2(0.0, 1.5), w)
        .unwrap()
        .angle(-std::f64::consts::FRAC_PI_2, w)
        .unwrap()
        .fillet(1e-6, w)
        .unwrap()
        .to(Start, w)
        .unwrap()
        .program;
    out.push(("tiny-radius (1e-6) seam square", tiny));

    // 2. A REFLEX arc leg (sweep 3.0 rad > π/2 by far, and the closing
    //    walk turns through the reflex side) into a fillet.
    let reflex = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, w)
        .unwrap()
        .arc_to(
            Sweep {
                r: 1.0,
                side: ArcSide::Left,
                angle: 3.0,
            },
            w,
        )
        .unwrap()
        .fillet(0.05, w)
        .unwrap()
        .at(p2(-2.0, 0.5), w)
        .unwrap()
        .toward(0.0, -1.0, w)
        .unwrap()
        .line(2.0, w)
        .unwrap()
        .line_to(Start, w)
        .unwrap()
        .program;
    out.push(("reflex arc into fillet", reflex));

    // 3. The lens at a LARGE radius (0.9 — close to the biggest that
    //    still fits the vesica's pocket).
    out.push(("large-radius lens", r1_lens(0.0, 0.9)));

    // 4. A NEAR-TANGENT junction: the lens displaced so the derived
    //    corner sits a hair off the entry anchor (asymmetry 2^-40),
    //    the different-pockets hazard's own neighborhood.
    out.push(("hairline lens 2^-40", r1_lens(2.0_f64.powi(-40), 0.5)));

    out
}

/// Claim 2, adversarially: guided at `f64` ≡ plain at `f64`, bit for
/// bit, on programs the unit's corpus does not contain.
#[test]
fn r1_guided_replay_at_f64_is_plain_replay_on_adversarial_programs() {
    for (name, program) in adversarial_programs() {
        let plain = replay(&program, tol())
            .unwrap_or_else(|e| panic!("{name}: the adversarial program must replay at f64: {e}"));
        let (recorded, structure) = replay_recording(&program, tol()).expect(name);
        same_bits(&plain, &recorded, &format!("{name}: recording"));
        let guided = replay_guided(&program, &structure, tol()).expect(name);
        same_bits(&plain, &guided, &format!("{name}: guided"));
    }
}

/// Claim 3's SUCCESS side, which the unit's suite leaves unexercised:
/// guided replay at `Interval` must also CERTIFY — consume every
/// recorded decision, confirm it, and enclose the f64 lane — on every
/// corpus program whose unguided Interval replay succeeds, several of
/// which resolve real fillets (line×line and arc-carrier alike). The
/// unit's own Interval rows either run unguided (generic_replay) or
/// abort (the hairline lens); this row is the guided-consume path at
/// `Interval` actually succeeding through fillet resolutions.
#[cfg(feature = "interval")]
#[test]
fn r1_guided_replay_at_interval_certifies_the_certifiable_corpus() {
    use common::coverage_corpus;
    use geom_core::{Bounds, Interval};
    let mut certified = 0usize;
    let mut with_fillets = 0usize;
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let (base, structure) = replay_recording(&closed.program, tol()).expect("replays at f64");
        let lifted: Vec<profile::Step<Interval>> =
            closed.program.iter().map(iv::embed_step).collect();
        // Rows the UNGUIDED interval replay cannot certify (the census
        // row) are not this probe's business.
        if replay(&lifted, tol()).is_err() {
            continue;
        }
        let guided = replay_guided(&lifted, &structure, tol()).unwrap_or_else(|e| {
            panic!("row {i}: unguided Interval certifies but guided refuses: {e}")
        });
        certified += 1;
        if !structure.fillets.is_empty() {
            with_fillets += 1;
        }
        assert_eq!(base.vertices().len(), guided.vertices().len(), "row {i}");
        for (k, (a, b)) in base.vertices().iter().zip(guided.vertices()).enumerate() {
            for (what, exact, enc) in [
                ("x", a.pos().x, b.pos().x),
                ("y", a.pos().y, b.pos().y),
                ("bulge", a.bulge(), b.bulge()),
            ] {
                assert!(
                    enc.lo() <= exact && exact <= enc.hi(),
                    "row {i} v{k}: the guided {what} enclosure excludes the f64 lane"
                );
            }
        }
    }
    assert!(
        certified >= 9,
        "the census says 9 of 10 certify: {certified}"
    );
    assert!(
        with_fillets >= 3,
        "the success path must cross real fillet resolutions: {with_fillets}"
    );
}

/// Claim 3/4 on R1's own lens: the hairline case at `Interval` aborts
/// typed, naming a consumed decision; it never re-picks.
#[cfg(feature = "interval")]
#[test]
fn r1_hairline_lens_aborts_typed_at_interval() {
    use geom_core::Interval;
    let program = r1_lens(f64::EPSILON, 0.5);
    let (_, structure) = replay_recording(&program, tol()).expect("replays at f64");
    let lifted: Vec<profile::Step<Interval>> = program.iter().map(iv::embed_step).collect();
    let err = replay_guided(&lifted, &structure, tol())
        .expect_err("the hairline lens cannot confirm at Interval");
    let ReplayErrorKind::Path(PathError::Structure(refusal)) = err.kind else {
        panic!("expected a named structure refusal, got {:?}", err.kind);
    };
    assert!(
        matches!(refusal.decision, Decision::CornerGate { .. }),
        "got {:?}",
        refusal.decision
    );
    assert!(matches!(
        refusal.kind,
        StructureRefusalKind::Indeterminate(_)
    ));
}

/// **Claim 4 at the replay door, with a genuinely wide parameter**: the
/// lens radius as the box [0.3, 0.7]. EVIDENCE-ONLY in part: the row
/// asserts the abort is typed and reports WHICH refusal class actually
/// fires — the spec's "a new typed refusal naming the decision" is only
/// one of the classes the wall can produce, and the review records
/// which one this construction reaches.
#[cfg(feature = "interval")]
#[test]
fn r1_wide_radius_box_aborts_typed_at_interval() {
    use geom_core::Interval;
    let program = r1_lens(0.25, 0.5);
    let (_, structure) = replay_recording(&program, tol()).expect("replays at f64");
    // Embed with the RADIUS widened to a box spanning [0.3, 0.7] —
    // the profile-parameter shape M10-3's driver will feed.
    let lifted: Vec<profile::Step<Interval>> = program
        .iter()
        .map(|s| match *s {
            profile::Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            } => profile::Step::ArcFilletArc {
                spec: iv::embed_spec(spec),
                radius: {
                    let _ = radius;
                    Interval::from_bounds(0.3, 0.7)
                },
                spec2: iv::embed_spec(spec2),
            },
            ref other => iv::embed_step(other),
        })
        .collect();
    let err = replay_guided(&lifted, &structure, tol())
        .expect_err("a radius box of width 0.4 across the lens cannot certify");
    // Typed, and never a panic — beyond that, record the class.
    match err.kind {
        ReplayErrorKind::Path(PathError::Structure(ref refusal)) => {
            println!("wide-radius refusal: Structure({refusal})");
            assert!(
                matches!(
                    refusal.kind,
                    StructureRefusalKind::Indeterminate(_) | StructureRefusalKind::Flipped { .. }
                ),
                "a structure refusal must carry one of its two arms"
            );
        }
        ReplayErrorKind::Path(ref e) => {
            // The honest record: the wall was the lane's own geometry,
            // typed but NOT naming a consumed decision.
            println!("wide-radius refusal (not Structure): {e}");
        }
        ref other => panic!("expected a path refusal, got {other:?}"),
    }
}

/// EVIDENCE-ONLY: a line×line fillet's `candidate`/`survivors`/
/// `corners` record fields are NOT verified under guidance — only its
/// fit signs are. A record whose candidate index is doctored to
/// nonsense still guides a line×line program to the identical loop.
/// Recorded for the census-honesty judgment (the census says a
/// line×line's "fit signs are its whole content", which this row
/// makes concrete: the other three fields are dead weight there).
#[test]
fn r1_line_line_record_ignores_its_ladder_fields() {
    let w = Tol::witness();
    let program = Open
        .at(p2(1.5, 0.0))
        .angle(0.0, w)
        .unwrap()
        .fillet(0.5, w)
        .unwrap()
        .at(p2(3.0, 1.5), w)
        .unwrap()
        .angle(std::f64::consts::FRAC_PI_2, w)
        .unwrap()
        .fillet(0.5, w)
        .unwrap()
        .at(p2(1.5, 3.0), w)
        .unwrap()
        .angle(std::f64::consts::PI, w)
        .unwrap()
        .fillet(0.5, w)
        .unwrap()
        .at(p2(0.0, 1.5), w)
        .unwrap()
        .angle(-std::f64::consts::FRAC_PI_2, w)
        .unwrap()
        .fillet(0.5, w)
        .unwrap()
        .to(Start, w)
        .unwrap()
        .program;
    let (plain, structure) = replay_recording(&program, tol()).expect("replays");
    let doctored = ReplayStructure {
        fillets: structure
            .fillets
            .iter()
            .map(|d| profile::FilletDecision {
                candidate: 7777,
                survivors: 9999,
                corners: vec![],
                ..d.clone()
            })
            .collect(),
    };
    let guided = replay_guided(&program, &doctored, tol())
        .expect("line×line guidance reads only the fit signs, so nonsense elsewhere passes");
    same_bits(&plain, &guided, "doctored line×line record");
}

/// EVIDENCE-ONLY: a tangent-joint set of the SAME length but different
/// CONTENT refuses — correctly — but the refusal's two reported values
/// are the two (equal) lengths, so its message cannot say what moved.
#[test]
fn r1_tangent_joint_flip_reports_equal_lengths() {
    use common::{coverage_corpus, profile};
    // A corpus loop with a PARTIAL declared-joint set, so a same-length
    // different-content lie exists (the all-joints rounded rect has no
    // such neighbor).
    let closed = coverage_corpus()
        .into_iter()
        .find(|c| {
            let n = c.loop_.tangent_joints().len();
            n >= 1 && n < c.loop_.vertices().len()
        })
        .expect("the corpus has a partially-jointed loop");
    let p = profile(vec![closed.loop_]);
    let (_, canonical) = p.validate_recording(tol()).expect("records");
    let mut lied = canonical.clone();
    let n = lied.loops[0].segments.len();
    let joints = &mut lied.loops[0].tangent_joints;
    assert!(!joints.is_empty(), "the loop declares a joint");
    // Same COUNT, different content: move one declared joint to an
    // index that is not in the set.
    let old = joints[0];
    let lie = (0..n)
        .map(|k| (old + 1 + k) % n)
        .find(|k| !joints.contains(k))
        .expect("some index is undeclared");
    joints[0] = lie;
    joints.sort_unstable();
    assert_eq!(
        joints.len(),
        canonical.loops[0].tangent_joints.len(),
        "the lie must keep the length"
    );
    let err = p
        .validate_guided(tol(), &lied)
        .expect_err("a moved joint set must refuse");
    let profile::ProfileError::Structure(refusal) = err else {
        panic!("expected a structure refusal, got {err:?}");
    };
    match refusal.kind {
        StructureRefusalKind::Flipped { recorded, found } => {
            println!("joint-flip refusal reports: recorded={recorded}, found={found}");
            assert_eq!(
                format!("{recorded}"),
                format!("{found}"),
                "EVIDENCE: the two sides display identically (both are the length), \
                 so the refusal cannot say which joint moved — if this row ever \
                 fails, the refusal got more articulate and the review finding is \
                 stale"
            );
        }
        other => panic!("expected the flipped arm, got {other:?}"),
    }
}

/// The `Interval` embedding helpers, whole-module gated per
/// `scripts/check-interval-cfg-additive.py`'s tests rule (a gated bare
/// `fn` is not an allowed item kind; a gated `mod` is).
#[cfg(feature = "interval")]
mod iv {
    #![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza
    /// The generic per-coordinate embedding (mirrors `generic_replay.rs`).
    pub fn embed_step<T: geom_core::Real>(step: &profile::Step<f64>) -> profile::Step<T> {
        use geom_core::Point2;
        use profile::{Step, Target};
        let pt = |p: Point2<f64>| Point2::new(T::from_f64(p.x), T::from_f64(p.y));
        let tgt = |t: Target<f64>| match t {
            Target::Start => Target::Start,
            Target::Point(p) => Target::Point(pt(p)),
        };
        match *step {
            Step::At(p) => Step::At(pt(p)),
            Step::Angle(v) => Step::Angle(T::from_f64(v)),
            Step::Toward { dx, dy } => Step::Toward {
                dx: T::from_f64(dx),
                dy: T::from_f64(dy),
            },
            Step::Tangent => Step::Tangent,
            Step::Turn(v) => Step::Turn(T::from_f64(v)),
            Step::Line(v) => Step::Line(T::from_f64(v)),
            Step::LineTo(t) => Step::LineTo(tgt(t)),
            Step::ArcTo(s) => Step::ArcTo(embed_spec(s)),
            Step::TangentArcTo(t) => Step::TangentArcTo(tgt(t)),
            Step::ArcContinue(p) => Step::ArcContinue(pt(p)),
            Step::Fillet { radius } => Step::Fillet {
                radius: T::from_f64(radius),
            },
            Step::FilletArc { radius, spec } => Step::FilletArc {
                radius: T::from_f64(radius),
                spec: embed_spec(spec),
            },
            Step::ArcFillet { spec, radius } => Step::ArcFillet {
                spec: embed_spec(spec),
                radius: T::from_f64(radius),
            },
            Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            } => Step::ArcFilletArc {
                spec: embed_spec(spec),
                radius: T::from_f64(radius),
                spec2: embed_spec(spec2),
            },
            Step::FarEndTo(p) => Step::FarEndTo(pt(p)),
            Step::CloseTo => Step::CloseTo,
            Step::Circle { centre, radius } => Step::Circle {
                centre: pt(centre),
                radius: T::from_f64(radius),
            },
            Step::CircleSplit {
                centre,
                radius,
                n,
                phase,
            } => Step::CircleSplit {
                centre: pt(centre),
                radius: T::from_f64(radius),
                n,
                phase: T::from_f64(phase),
            },
        }
    }

    /// The arc-spec half of [`embed_step`].
    pub fn embed_spec<T: geom_core::Real>(s: profile::ArcData<f64>) -> profile::ArcData<T> {
        use profile::{ArcData, Target};
        let pt =
            |p: geom_core::Point2<f64>| geom_core::Point2::new(T::from_f64(p.x), T::from_f64(p.y));
        let tgt = |t: Target<f64>| match t {
            Target::Start => Target::Start,
            Target::Point(p) => Target::Point(pt(p)),
        };
        match s {
            ArcData::Radius { r, side } => ArcData::Radius {
                r: T::from_f64(r),
                side,
            },
            ArcData::Bulge { target, b } => ArcData::Bulge {
                target: tgt(target),
                b: T::from_f64(b),
            },
            ArcData::Via { q, target } => ArcData::Via {
                q: pt(q),
                target: tgt(target),
            },
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            ArcData::Sweep { r, side, angle } => ArcData::Sweep {
                r: T::from_f64(r),
                side,
                angle: T::from_f64(angle),
            },
            ArcData::ArcLen { r, side, len } => ArcData::ArcLen {
                r: T::from_f64(r),
                side,
                len: T::from_f64(len),
            },
        }
    }
}

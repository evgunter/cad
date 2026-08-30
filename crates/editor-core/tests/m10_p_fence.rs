//! **The bit-identity fence.**
//!
//! The profile lift is additive machinery beside an unchanged build
//! path, and the fence is what makes "unchanged" a measurement rather
//! than an intention. With the lift OFF — the default, and the only
//! setting the build path ever uses — evaluating the whole Band 4
//! corpus must produce exactly the bits it produced before the lift
//! existed.
//!
//! The digest below is that measurement, and it is deliberately
//! computed from the observable evaluation and nothing else: every
//! node's outcome in id order, and for every body the bits of every
//! point it carries. It is a golden number in the ordinary sense — if
//! it moves, the question is whether the new behaviour is correct, not
//! how to get the old number back. What makes it useful HERE is that
//! this file uses no API the lift introduced, so the same digest can be
//! taken on a pre-lift tree and compared. It was: the three numbers
//! are the ones a checkout of the PRE-LIFT tree produces from this
//! same file, which is what "the build path did not move" means here.
//! **That differential was taken against the roster of the day**, and
//! the roster has since grown twice — see the re-bless paragraphs
//! below, which say what moved and what evidence stands behind each.
//! Re-taking it requires the pre-lift tree AND the current registry.
//!
//! THE TREE THE COMPARISON WAS MADE AGAINST is `41e32c24` — main at
//! the fix pass, which is a pre-lift tree because the lift lives only
//! on this branch. That is a better comparator than the branch's
//! original merge base and a different fact from it: a reader who
//! resolves "the merge base" gets the commit this branch forked at,
//! which has since been overtaken. Re-running the differential means
//! checking a pre-lift `crates/` out under this file again — the
//! numbers below are evidence about the LIFT, not about any particular
//! base, and they held across both comparators.
//!
//! `interval` and `probe` rows ride the same helper, so the fence
//! covers the three scalars the review names rather than the value lane
//! alone.
//!
//! A whole-corpus scalar is a blunt instrument for "did an existing
//! document move", and there is now a SECOND, finer measurement to
//! read beside it: `lib_g16_corpus_name_digests` pins a digest PER
//! DOCUMENT, so a registry addition that disturbed an existing one
//! shows up there as a row that moved and not merely as a scalar that
//! changed. Every roster addition below was checked against it as well
//! as by the removal procedure — each moved its own row and no other.
//! That table is the reason to trust the re-blessings rather than
//! merely to accept them.
//!
//! What still makes the fence mean what it says: the corpus's other
//! rows — `m4_pr8_corpus`'s per-document coverage and cone probes, the
//! Dual/Interval corpus digests, and the persistence round trip — all
//! hold across the same change, and a measurement sink denotes no
//! body, so it contributes no geometry to any pre-existing document.
//! Re-running the pre-lift differential means taking these numbers on
//! a pre-lift tree WITH the same registry; comparing across a registry
//! change compares two different corpora and answers nothing.
//!
//! THE DIGEST IS EPS-INDEPENDENT BY CONSTRUCTION, and has to be: the
//! hosted matrix samples a tolerance row per run, so a golden number
//! that moved with eps would be a fence that only ever gated one row of
//! three. Nothing rendered from a classification band enters it — see
//! the outcome arms below — and the rows assert the SAME constant at
//! every eps, which is itself a claim this file makes and a reason for
//! a reader to be suspicious if one ever needs a second number.
//!
//! WHAT THE INTERVAL ROW IS NOT. It pins that the lift OFF changes
//! nothing at `Interval` — a fence around the BUILD path, which binds
//! its parameters at their nominals. It is not evidence about a WIDE
//! interval parameter, which is a different door: an evaluation
//! carrying an `EvalOptions::param_box` binds `nominal + [lo, hi]`
//! instead, and `m10_3_driver_interval`'s rows are what drive that.
//! The two claims stay separate because the fence's subject is that
//! the lift changed nothing where nothing should change.
//!
//! RE-BLESSED THREE TIMES, every time for a ROSTER change rather than
//! a build-path one: this digest walks `corpus::documents()`, so a new
//! document moves it by construction. Each re-blessing was MEASURED
//! the same way, and the measurement is the procedure — remove the new
//! document ALONE from `documents()` and check every constant comes
//! back at its previous value, which is what "no EXISTING document's
//! bits moved" means here.
//!
//! - LIB-G16 added `die_chamfer`. Removing it alone returned
//!   `f64`/`probe` `ebba499b112fea43, 3350329b8dcf3c2f` and `interval`
//!   `6c3f436b41ecd1b4, e7db67ef2cffe270`.
//! - LIB-CORPUS-DIE added `die_composed_tour`, the demo tour's die.
//!   Removing it alone returned `f64`/`probe` `0f7cdec3cf38ad1e,
//!   01e05bef0382adda` and `interval` `bfb345df4492bc11,
//!   c835f9e36e694ddd` — exactly the constants this file carried
//!   between the two re-blessings.
//! - M10-2 added `measured_web`, carrying the E3/E10 measurement nodes
//!   so the Dual/Interval digests' Measure and Assertion arms are
//!   REACHED rather than merely present. Removing it alone returned
//!   `f64`/`probe` `803b01aaab703256, 3f310d4d77e892ba` and `interval`
//!   `3ee6a402bcb1f12e, ef742c0a0c9dd7da` — the constants this file
//!   carried on `main` before the M10-2 merge, and the rows came back
//!   GREEN against them rather than being compared by hand.
//!
//! The M10-2 measurement is the strongest of the three, for a reason
//! worth stating: this roster minus `measured_web` IS main's roster,
//! so the expected values were not re-derived for the occasion — they
//! were already committed here by someone else. A measurement sink
//! denotes no body, so it contributes no geometry to any pre-existing
//! document, and that prediction is what the removal confirms.
//!
//! The three numbers below are the same digest over the grown roster.
//!
//! The `probe` row is ROSTERED into the K-telemetry sweep's executed
//! floor. Its claim is not a third copy of the `f64` row's: it says the
//! telemetry scalar has not started changing decisions, which is a
//! claim about `Probe` that nothing else in the tree makes, and a claim
//! carried by a suite nothing runs is not carried at all. The
//! `probe`-gated code here therefore executes on the sweep's schedule
//! rather than on the code tier's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{CancelToken, EvalOptions, NodeResult, ValuePayload, evaluate};
use geom_core::Tol;

/// A 128-bit FNV-1a over the evaluation's observable bits.
struct Digest {
    lo: u64,
    hi: u64,
}

impl Digest {
    fn new() -> Self {
        Self {
            lo: 0xcbf2_9ce4_8422_2325,
            hi: 0x9dc5_bb32_e0f7_1a49,
        }
    }

    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.lo = (self.lo ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
            self.hi = (self.hi ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn text(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.u64(u64::from(*b));
        }
        self.u64(u64::MAX);
    }
}

/// The corpus's evaluation, digested at one scalar.
///
/// `bits` maps the scalar's coordinate to its exact representation.
/// Feeding through the caller keeps this file free of any per-scalar
/// door, which is what lets it compile against a pre-lift tree.
fn corpus_digest<T, F, S>(bits: F, scalar: S) -> (u64, u64)
where
    T: editor_core::EvalScalar,
    F: Fn(&mut Digest, &geom_core::Point3<T>),
    S: Fn(&mut Digest, T),
{
    let mut d = Digest::new();
    // The arc-carrier fillet machinery, which no corpus document
    // reaches — see `fixture_digest`.
    fixture_digest::<T>(&mut d, scalar);
    for doc in corpus::documents() {
        d.text(doc.name);
        let ev = evaluate::<T>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        for (id, result) in ev.nodes.iter() {
            d.u64(id.0);
            match result {
                NodeResult::Poisoned { through } => {
                    d.text("poisoned");
                    d.u64(through.0);
                }
                // The OUTCOME, not the rendered message. A refusal's
                // text embeds the classification band, so digesting it
                // would make this whole fence eps-dependent — one
                // golden number per tolerance row, in a repo whose CI
                // deliberately samples three of them. What belongs in a
                // bit-identity fence is that the same nodes succeeded
                // and the same geometry came out; that a refusal is
                // also the SAME refusal is a claim about the lift
                // setting rather than about two trees, and it is
                // carried at full text by `m10_p_lift`'s pinned-vs-
                // guided comparison, which runs both sides in one
                // process at one eps and so can compare messages
                // honestly.
                NodeResult::Failed(_) => d.text("failed"),
                NodeResult::Ok(v) => {
                    d.text(v.payload.kind_name());
                    if let ValuePayload::Body(b) = &v.payload {
                        for (_, p) in b.points() {
                            bits(&mut d, p);
                        }
                    }
                }
            }
        }
    }
    (d.lo, d.hi)
}

/// **The arc-carrier fillet the corpus does not contain.**
///
/// Every Band-4 profile is authored from straight legs and closed
/// carriers, so a corpus digest — however wide — never once enters
/// `arc_fillet::resolve`, which is where the S8 selection ladder, the
/// derived-corner enumeration and the two angular gates live. That is
/// exactly the machinery this unit rewrote, so a fence that could not
/// see it was pinning the wrong half of the tree.
///
/// This fixture is the rocker eye and the vesica lens, built through
/// the typed surface and replayed at the digest's scalar. It uses no
/// API the lift introduced (`replay` is pre-lift), so it travels to a
/// pre-lift tree with the rest of this file. A row that REFUSES is
/// digested as its refusal: the eye does not certify at `Interval`
/// (see `profile`'s `generic_replay` census for why), and "refuses
/// with this message" is as much a bit of behaviour to hold still as
/// "returns these coordinates".
fn fixture_digest<T: profile::ArcCarrierScalar>(d: &mut Digest, bits: impl Fn(&mut Digest, T)) {
    use geom_core::Point2;
    use profile::{ArcData, ArcSweep, Center, Open, Start, Step, Target};
    let p2 = |x: f64, y: f64| Point2::new(x, y);
    let tip = 0.75_f64.sqrt();
    // (1) the eye: circle x circle carriers crossing AT the entry
    // anchor. (2) the vesica lens: the two-survivor corner the S8
    // ladder actually ranks.
    let programs = [
        Open.arc_fillet_arc(
            Center {
                c: p2(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -tip),
            },
            0.35,
            Center {
                c: p2(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        ),
        Open.arc_fillet_arc(
            Center {
                c: p2(-1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -3.0_f64.sqrt()),
            },
            0.5,
            Center {
                c: p2(1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        ),
    ];
    let embed = |step: &Step<f64>| -> Step<T> {
        let pt = |p: Point2<f64>| Point2::new(T::from_f64(p.x), T::from_f64(p.y));
        let tgt = |t: Target<f64>| match t {
            Target::Start => Target::Start,
            Target::Point(p) => Target::Point(pt(p)),
        };
        let spec = |a: ArcData<f64>| match a {
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            _ => unreachable!("this fixture authors Center-mode arcs only"),
        };
        match *step {
            Step::ArcFilletArc {
                spec: a,
                radius,
                spec2,
            } => Step::ArcFilletArc {
                spec: spec(a),
                radius: T::from_f64(radius),
                spec2: spec(spec2),
            },
            _ => unreachable!("this fixture is one fused step"),
        }
    };
    for (i, built) in programs.into_iter().enumerate() {
        d.u64(i as u64);
        let closed = built.expect("the arc-carrier fixture constructs at f64");
        let steps: Vec<Step<T>> = closed.program.iter().map(embed).collect();
        match profile::replay(&steps, Tol::witness()) {
            Ok(lp) => {
                d.text("ok");
                d.u64(lp.vertices().len() as u64);
                for v in lp.vertices() {
                    bits(d, v.pos().x);
                    bits(d, v.pos().y);
                    bits(d, v.bulge());
                }
            }
            // Same reason as the corpus arm above: the eye's interval
            // refusal names its band, and the band is the eps.
            Err(_) => d.text("refused"),
        }
    }
}

fn f64_bits(d: &mut Digest, p: &geom_core::Point3<f64>) {
    for c in [p.x, p.y, p.z] {
        d.u64(c.to_bits());
    }
}

/// **The fence at `f64`.** Evaluating the corpus with the default
/// options produces exactly these bits.
#[test]
fn the_corpus_evaluation_is_bit_identical_at_f64() {
    let got = corpus_digest::<f64, _, _>(f64_bits, |d, v: f64| d.u64(v.to_bits()));
    println!("m10-p fence f64: {got:016x?}");
    assert_eq!(
        got,
        (0x50b4_edef_3d4c_d1ac, 0xa17d_6b19_5195_bee0),
        "the corpus's f64 evaluation moved — see this file's header before \
         touching the number"
    );
}

/// The same fence at `Interval`, where the lift's second pass would
/// otherwise be tempting to leave on.
#[cfg(feature = "interval")]
#[test]
fn the_corpus_evaluation_is_bit_identical_at_interval() {
    use geom_core::{Bounds, Interval};
    let got = corpus_digest::<Interval, _, _>(
        |d, p| {
            for c in [p.x, p.y, p.z] {
                d.u64(c.lo().to_bits());
                d.u64(c.hi().to_bits());
            }
        },
        |d, v: Interval| {
            d.u64(v.lo().to_bits());
            d.u64(v.hi().to_bits());
        },
    );
    println!("m10-p fence interval: {got:016x?}");
    assert_eq!(
        got,
        (0xb09c_3944_a8d7_c440, 0x588d_3ac4_427a_f52c),
        "the corpus's Interval evaluation moved"
    );
}

/// And at `Probe`, the K-telemetry scalar.
#[cfg(feature = "probe")]
#[test]
fn the_corpus_evaluation_is_bit_identical_at_probe() {
    use geom_core::Probe;
    let got = corpus_digest::<Probe, _, _>(
        |d, p| {
            for c in [p.x, p.y, p.z] {
                d.u64(c.0.to_bits());
            }
        },
        |d, v: Probe| d.u64(v.0.to_bits()),
    );
    println!("m10-p fence probe: {got:016x?}");
    // Probe is a transparent f64, so this is the f64 row's number and
    // must stay so: a Probe digest that drifted from it would mean the
    // telemetry scalar had started changing decisions.
    assert_eq!(
        got,
        (0x50b4_edef_3d4c_d1ac, 0xa17d_6b19_5195_bee0),
        "the corpus's Probe evaluation moved"
    );
}

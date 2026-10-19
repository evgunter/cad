//! **The EXACT channel never contradicts its own form** — the measured
//! half of the partition SYM-11 draws.
//!
//! At `Sym<Interval>` a definite non-zero sign is a certified bracket
//! that excludes zero, so a form that is the zero polynomial under it
//! would mean one of the two channels does not contain its real. That
//! is a soundness defect and a panic is the honest answer; what this
//! file measures is that it never happens on the documents the tier is
//! measured on — at the scale each certifies whole at, and past its
//! ceiling, where the residuals are the widest the drive ever sees.
//!
//! The nominal half is pinned by the gating row
//! `m10_9_no_registrant_lies_on_any_measured_document`, which replays
//! the same five documents and now reads the dispute column too. This
//! file is the ceiling-plus-δ half, a second whole-box replay of each.
//! It runs in the slow set: the bracket replays whole past its ceiling
//! and the pad reaches its extrude, about 20 s in a release build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{CancelToken, EvalOptions, NodeResult, evaluate};
use geom_core::{SymCounts, SymRules, Tol};

use crate::m10_9_pins_interval::measured_studies;

/// **THE RECEIPTS PAST EACH CEILING**, in `measured_studies`' order,
/// as `[symbolic, numeric, frozen]`. Identical at
/// ε = 1e-6, 1e-9 and 1e-12: the atoms a residual carries do not
/// depend on the band, and neither does what the tier proves about
/// them.
///
/// The plate and the annulus refuse nowhere the bisection reaches, and
/// replay whole at [`PAST_NO_CEILING`]. The link stops at the
/// extrude's attachment gate (`carrier_matches_mapped_source`). The
/// bracket's and the pad's over-band span readings (`arc_span`,
/// `line_span`) are settled by the segments' ends, which lie on the
/// other carrier, so the bracket replays whole and the pad stops at
/// its extrude's pcurve certification (`pcurve_envelope`). How each value
/// moved since SYM-11 (2026-09-21), merge by merge, is attributed in
/// `work/sym/ignored-sym-receipt-rows-drifted-red-on-main-unattributed`.
///
/// INTENT-LITERALS PR C (a slot holds a variable) adds one theorem per
/// formula written at a slot, an anonymous definition bound through the
/// non-finite door: plate +1, annulus +2, link +2, bracket +3, the pad
/// unmoved; `numeric` and `frozen` hold. The same deltas
/// move `m10_9_pins_interval`'s and `sym_9_retry_interval`'s rows.
///
/// An intrinsic edge description's surfaces becoming a set
/// (`geom_brep::SurfacePair`) moves only `frozen`, measured on the tree
/// holding every change above: 612 → 641, 804 → 828, 486 → 498,
/// 806 → 830, 302 → 314, and no decision column. The cause is the
/// transversality wedge (`geom_brep::dihedral::wedge_decided`'s
/// `n1.cross(n2)`), which the certificate takes in key order and the
/// extrude's dihedral in builder order: symmetric in value up to sign
/// but not in form, so a document that reads one edge's wedge in both
/// orientations builds two hash-consed forms where it built one.
///
/// Settling an in-band span reading on the segments' ends takes the
/// bracket and the pad past validation, which adds the extrude's
/// stages to their counts: bracket `[626, 464, 830]` → `[1453, 1008,
/// 1869]`, pad `[340, 277, 314]` → `[1319, 1159, 2916]`. The
/// attribution of those values, merge by merge, is in
/// `work/sym/ignored-sym-receipt-rows-drifted-red-on-main-unattributed`.
///
/// Check 4 reading one vector at its norm where it summed its parts
/// moves only `frozen` again. On the plate, the annulus and the link,
/// each site measured by reverting it alone: the turning cylinder's
/// pair against its class circle +72, +52, +36, and the cylinder class
/// selector's radial circle at its exact sup (`radial_amplitude`) +16,
/// +24, +8; together 641 → 729, 828 → 904, 498 → 542. On the bracket
/// and the pad, which the settlement above carries past validation,
/// the two together move the bracket's `frozen` 1869 → 1975 and the
/// pad's 2916 → 2985. The pad's replay past the ceiling refuses at
/// check 4 with no term over the band alone, so the refusal decides
/// every term of the envelope, 9 where there were 11, and its
/// `symbolic` reads 1319 → 1317.
/// The fidelity channels' norm and the retired `Tilt` and
/// `Orientation` slots move no column here.
/// The pcurve envelope's orthonormal chart squaring its frame by cross
/// products (`geom::square_frame`) moves only the frozen counts: bracket
/// 1975 → 1969, pad 2985 → 2976.
/// An edge's wedge levered at its faces' extent (`topo::wedge_reach`)
/// where it was levered at the edge's own chord moves only `frozen`,
/// and no decision column: each certified edge's lever reads its two
/// faces' boundary vertices. The plate, the annulus and the link hold;
/// bracket 1969 → 2044, pad 2976 → 3522, whose fillets' edges bound
/// more faces than a planar wall's.
/// The final non-finite check asking boundedness, where it decided
/// `value · 0` (`expr-non-finite-asks-value-times-zero`), takes its
/// theorems out of `symbolic` and moves nothing else: plate 1048 → 1003,
/// annulus 566 → 539, link 356 → 323, bracket 1453 → 1392, pad
/// 1317 → 1260. A decision left; none moved, so the dispute column's
/// zero stands on the same decisions.
///
/// Placing each row over its edge's span in the chart's principal
/// window (PCERT unit B1, C4) moves decisions, and each is attributed:
/// every placed azimuth reads its two half-period marks numerically
/// (`pcurve_chart_placement`), and a cylinder's meridian azimuth is
/// read only on a meridian, so a rim circle centred on the axis no
/// longer takes the frame read the tier proved zero. `symbolic` falls
/// by those frame reads and `numeric` rises by the marks; `frozen` rises
/// by the marks' contradiction checks whose forms froze (plate +12,
/// annulus +16, link +6, bracket +15, pad +21), the frame reads having
/// been proved and frozen nothing. Plate `[1048, 654, 729]` →
/// `[1032, 726, 741]`, annulus `[566, 399, 904]` → `[550, 471, 920]`,
/// link `[356, 259, 542]` → `[348, 295, 548]`, bracket
/// `[1453, 1008, 1969]` → `[1433, 1096, 1984]`, pad `[1317, 1159, 2976]`
/// → `[1305, 1211, 2997]`. No dispute fired. With the wedge levered at
/// its faces' extent as well, the two moves add: bracket `[1433, 1096,
/// 2059]` and pad `[1305, 1211, 3543]`, the wedge's `frozen` (+75, +546)
/// on top of the placement's. The placement's azimuth levered at the
/// carrier's farthest reach from the axis at the span's ends and midpoint
/// (PR 5225's one lever rule) moves `frozen` alone, the marks' forms built
/// from three points: plate 741 → 729, annulus 920 → 940, link 548 → 542,
/// bracket 2059 → 2110, pad 3543 → 3558. No decision moved and no dispute
/// fired. Main's final non-finite check and the placement add: `symbolic`
/// falls by main's count on each (plate −45, annulus −27, link −33,
/// bracket −61, pad −57) from the placement's; `numeric` and `frozen` are
/// the placement's. The frame decided across the window's cut moves
/// `frozen` alone, the cut-side margin's forms: bracket 2110 → 2112, pad
/// 3558 → 3566. No decision moved and no dispute fired.
///
/// A harmonic row's windings stored as structure (PCERT B2a, `[ev]` PR
/// 5113's first decision) retire the decisions that read them off a
/// slope: plate `[987, 726, 729]` → `[939, 686, 729]`, annulus
/// `[523, 471, 940]` → `[475, 431, 940]`, link `[315, 295, 542]` →
/// `[291, 275, 542]`, bracket `[1372, 1096, 2112]` → `[1312, 1046,
/// 2112]`, pad `[1248, 1211, 3566]` → `[1212, 1181, 3566]`. The decisions
/// are gone, not moved; no dispute fired, and `frozen` holds.
///
/// A chord's director spelled as its components over its length
/// (`profile`'s `path::chord_director`) where it was the `sin_cos` of an
/// `atan2`: the link's `frozen` 542 → 556, its decisions held. The pad's
/// ceiling rose with it (`line_span` no longer bounds it; `arc_span`
/// does, at 3.152e3·ε), so its row is read at a new scale and is a new
/// measurement: `[1397, 1178, 4309]`, where it was `[1212, 1181, 3566]`
/// at 2.7783e3·ε. No dispute fired.
const PAST_THE_CEILING: [(&str, [u64; 3]); 5] = [
    ("two_hole_plate", [939, 690, 729]),
    ("r1_annulus", [475, 431, 940]),
    ("r2_link", [291, 275, 556]),
    ("r2_filleted_bracket", [1312, 1046, 2112]),
    ("r2_rounded_pad", [1397, 1178, 4309]),
];

/// The scale, in multiples of ε, a document with no measured refusal
/// replays at. Not the bisection's top, `1e6·ε`: at ε = 1e-6 that is a
/// scale of one, and the annulus's extrude refuses there geometrically
/// before the tier is asked what it is here to be asked.
const PAST_NO_CEILING: f64 = 1e5;

/// One whole-box replay at `Sym<Interval>`, ON ITS OWN THREAD: the
/// session's counts and the first node that refused, or the panic's
/// own message when the theorem-vs-numeric assertion fired. A panic
/// inside the session leaves that thread's session installed and the
/// next replay would refuse to nest, so the thread is what makes the
/// next document runnable ([`test_utils::own_thread`], which keeps the
/// message — an `Err` that said only "it panicked" would send a reader
/// looking for the document that did it).
fn replay_on_a_thread(
    doc: ProfileDoc,
    box_: ParamBox,
    tol: Tol,
) -> Result<(Option<String>, SymCounts), String> {
    test_utils::own_thread::caught(move || {
        let opts = EvalOptions {
            param_box: Some(Arc::new(box_)),
            ..EvalOptions::default()
        };
        let budget = geom_core::SymBudget {
            max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        };
        geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
            let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                evaluate(&doc, None, &CancelToken::new(), &opts, tol);
            ev.order.iter().find_map(|id| match ev.result(*id) {
                Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
                _ => None,
            })
        })
    })
}

/// **THE STOP CLAUSE'S ROW.** The five measured documents replayed at
/// `Sym<Interval>` past their ceilings — `Study::refuses_at`, the
/// scale the measured bracket says the drive refuses at, or
/// [`PAST_NO_CEILING`] for a document that refuses nowhere the
/// bisection reaches — where every leaf's residuals are the widest the
/// corpus produces. The exact
/// witness contradicts its own form at NONE of them, at any ε.
///
/// **The receipts are ASSERTED, not recorded in prose**, so a re-take
/// reds on drift instead of quietly printing a different table:
/// [`PAST_THE_CEILING`] is the measurement, identical at ε = 1e-6,
/// 1e-9 and 1e-12.
///
/// **What runs it: the slow set** (`.config/nextest.toml`), on every PR
/// whose diff touches `editor-core`, and every night at every ε row.
#[test]
fn sym11_the_exact_channel_never_contradicts_past_the_ceiling() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut fired = 0;
    let mut moved = Vec::new();
    for (study, want) in measured_studies(tol).into_iter().zip(PAST_THE_CEILING) {
        let name = study.name;
        assert_eq!(
            name, want.0,
            "PAST_THE_CEILING is read positionally against `measured_studies`, and the two \
             have gone out of order"
        );
        let past = study.refuses_at.unwrap_or(PAST_NO_CEILING);
        let doc = (study.at)(past * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        match replay_on_a_thread(doc, ParamBox::of(&analyzed), tol) {
            Ok((refusal, counts)) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: {counts:?} -> {refusal:?}");
                let got = [counts.symbolic, counts.numeric, counts.frozen];
                if got != want.1 {
                    moved.push(format!("{name}: {got:?}, stored {:?}", want.1));
                }
                if counts.registrations_contradicted != 0 {
                    moved.push(format!(
                        "{name}: a registered identity was contradicted past the ceiling — \
                         that is an axiom a constructor stated and this box disproves: {counts:?}"
                    ));
                }
                if counts.theorems_disputed != 0 {
                    moved.push(format!(
                        "{name}: this lane's witness is EXACT, so the theorem-vs-numeric \
                         contradiction is routed to the `debug_assert!` and never to this \
                         column — a count here is `Interval::WITNESS` having moved, not a \
                         document behaving badly: {counts:?}"
                    ));
                }
            }
            Err(message) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: the CONTRADICTION FIRED");
                println!("     {message}");
                fired += 1;
            }
        }
    }
    assert_eq!(
        fired, 0,
        "the exact witness contradicted its own form on {fired} of the measured documents — \
         a certified bracket that excludes zero and a form that is the zero polynomial \
         cannot both be right, so one of the two channels does not contain its real. That \
         is a soundness defect in a channel and a DIFFERENT unit: stop, file it with the \
         render, and report"
    );
    assert!(
        moved.is_empty(),
        "at eps={eps:e}, past the ceiling (a receipt is [symbolic, numeric, frozen]). This row is the measurement SYM-11 argued the dispute column's \
         zero from, so a number that moved is a decision that moved and is a finding, not a \
         table to refresh:\n{}",
        moved.join("\n")
    );
}

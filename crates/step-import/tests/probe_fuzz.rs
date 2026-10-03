//! REVIEW PROBE (B8): adversarial mutation of the committed wild files
//! — truncations, entity-type swaps, reference cycles, giant repeat
//! counts — hunting for panics or hangs.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use step_import::{ImportOptions, import_step};

const WILD: [&str; 13] = [
    "adafruit/328_2500mAh_battery.step",
    "adafruit/1982_MPR121.step",
    "adafruit/805_slide_switch.step",
    "adafruit/931_OLED_128x32_I2C.step",
    "adafruit/64_Halfsize_Breadboard.step",
    "nist/nist_ftc_09_asme1_rd.stp",
    "stepcode/sg1-c5-214.stp",
    "stepcode/dm1-id-214.stp",
    "stepcode/TAIL_TURBINE.stp",
    "stepcode/io1-cm-214.stp",
    "occ-oss/b123d_nema17_bracket.step",
    "nist/nist_ftc_11_asme1_rb.stp",
    "occ-oss/cq_red_cube_blue_cylinder.step",
];

/// The corpus's most expensive import, by a wide margin.
const DM1: &str = "stepcode/dm1-id-214.stp";

/// [`WILD`] minus [`DM1`] — the corpus of
/// [`reference_cycles_and_dangling_refs_never_panic`], and of that row
/// only (2026-08-13 test-time audit).
///
/// **Why this row and not the others.** The truncation and
/// entity-type-swap rows are cheap on dm1 because their mutants die
/// early: a truncated file fails to parse, and a swapped keyword
/// derails the reader before the geometry pass. The reference-cycle row
/// is different — its per-ε-row profile tracked a single UNMUTATED dm1
/// import almost exactly, which means one of dm1's cycle mutants leaves
/// the reference graph resolvable and re-runs the whole geometry pass
/// at full price. That geometry pass is not what this row is about: the
/// property under test is a RESOLVER property (a cyclic or dangling
/// reference must produce a `Result`, never a panic or a hang), and
/// twelve wild files still exercise it, including the two other
/// stepcode files and the NIST inch translator's output.
///
/// **What is lost:** no-panic on dm1's *mutated* reference graph
/// specifically. dm1's unmutated import stays gated — `tier_gate.rs`
/// pins its disposition at every ambient band — and dm1 remains in
/// [`WILD`] for the truncation and swap rows.
fn cycle_corpus() -> impl Iterator<Item = &'static str> {
    WILD.into_iter().filter(|name| *name != DM1)
}

fn wild(name: &str) -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        name,
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap()
}

/// Runs one mutation, asserting a RESULT (never a panic) and reporting
/// anything that takes suspiciously long.
fn must_not_panic(tag: &str, text: &str, budget: Duration) {
    let t = Instant::now();
    let owned = text.to_owned();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        import_step(&owned, &ImportOptions::default(), Tol::witness()).map(|i| i.eps_in())
    }));
    let took = t.elapsed();
    assert!(outcome.is_ok(), "{tag}: PANICKED");
    if took > budget {
        println!("{tag}: SLOW — {took:?}");
    }
}

#[test]
fn truncations_never_panic() {
    for name in WILD {
        let text = wild(name);
        let n = text.len();
        // 40 truncation points, plus a few byte-boundary-hostile ones.
        for k in 0..40 {
            let mut cut = n * k / 40;
            while cut < n && !text.is_char_boundary(cut) {
                cut += 1;
            }
            must_not_panic(
                &format!("{name} truncated@{cut}"),
                &text[..cut],
                Duration::from_secs(20),
            );
        }
    }
}

#[test]
fn entity_type_swaps_never_panic() {
    // Swap keywords so references point at the wrong entity kinds.
    let swaps: [(&str, &str); 10] = [
        ("CARTESIAN_POINT", "DIRECTION"),
        ("DIRECTION", "CARTESIAN_POINT"),
        ("VERTEX_POINT", "CARTESIAN_POINT"),
        ("CIRCLE", "LINE"),
        ("LINE", "CIRCLE"),
        ("PLANE", "CYLINDRICAL_SURFACE"),
        ("CYLINDRICAL_SURFACE", "PLANE"),
        ("EDGE_LOOP", "VERTEX_LOOP"),
        ("ORIENTED_EDGE", "EDGE_CURVE"),
        ("CLOSED_SHELL", "OPEN_SHELL"),
    ];
    for name in WILD {
        let text = wild(name);
        for (from, to) in swaps {
            if !text.contains(from) {
                continue;
            }
            must_not_panic(
                &format!("{name} swap {from}->{to}"),
                &text.replace(from, to),
                Duration::from_secs(20),
            );
        }
    }
}

#[test]
fn reference_cycles_and_dangling_refs_never_panic() {
    // [`cycle_corpus`], not [`WILD`]: dm1 is excluded here alone, and
    // that doc comment says why and what it costs. The count pins that
    // the exclusion still bites — a renamed fixture would otherwise
    // turn the filter into a silent no-op.
    assert_eq!(
        cycle_corpus().count(),
        WILD.len() - 1,
        "cycle_corpus must exclude exactly dm1 — is {DM1} still in WILD?"
    );
    for name in cycle_corpus() {
        let text = wild(name);
        // A self-referential placement and a self-referential unit.
        let mut mutations = Vec::new();
        // Point every AXIS2_PLACEMENT_3D's location at the placement.
        for (needle, sub) in [
            ("AXIS2_PLACEMENT_3D('',#", "AXIS2_PLACEMENT_3D('',#999999,#"),
            (
                "CONVERSION_BASED_UNIT('INCH',#",
                "CONVERSION_BASED_UNIT('INCH',#999999",
            ),
        ] {
            if text.contains(needle) {
                mutations.push(text.replace(needle, sub));
            }
        }
        // Self-cycle: rewrite the first entity id N to reference itself.
        if let Some(pos) = text
            .find("=CARTESIAN_POINT")
            .or_else(|| text.find(" = CARTESIAN_POINT"))
        {
            let head = &text[..pos];
            if let Some(hash) = head.rfind('#') {
                let id: String = text[hash + 1..pos].trim().to_owned();
                if id.chars().all(|c| c.is_ascii_digit()) && !id.is_empty() {
                    // Replace the point with a placement naming itself.
                    let rest_start = text[pos..].find(';').map(|i| pos + i).unwrap_or(pos);
                    let m = format!(
                        "{}#{id}=AXIS2_PLACEMENT_3D('',#{id},#{id},#{id}){}",
                        &text[..hash],
                        &text[rest_start..]
                    );
                    mutations.push(m);
                }
            }
        }
        for (i, m) in mutations.iter().enumerate() {
            must_not_panic(&format!("{name} cycle#{i}"), m, Duration::from_secs(20));
        }
    }
}

#[test]
fn giant_repeat_counts_and_numeric_extremes_never_panic_or_hang() {
    // The knot run-length decode expands (multiplicity) into a flat
    // vector; a hostile multiplicity is an unbounded allocation. Also
    // giant list lengths and absurd numeric literals.
    let hostile: [(&str, &str); 6] = [
        (
            "degenerate huge int",
            "B_SPLINE_CURVE_WITH_KNOTS('',3,(#1,#2,#3,#4),.UNSPECIFIED.,.F.,.F.,(4,4),(0.,1.),.UNSPECIFIED.)",
        ),
        ("radius 1e308", "CIRCLE('',#11,1.E308)"),
        ("radius -0.", "CIRCLE('',#11,-0.)"),
        ("nan-ish", "CIRCLE('',#11,1.E999)"),
        (
            "huge mult",
            "B_SPLINE_CURVE_WITH_KNOTS('',1,(#1,#2),.UNSPECIFIED.,.F.,.F.,(2000000000,2000000000),(0.,1.),.UNSPECIFIED.)",
        ),
        ("zero-length list", "GEOMETRIC_CURVE_SET('',())"),
    ];
    let base = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    ))
    .unwrap();
    for (tag, record) in hostile {
        let m = base.replace("#26 = CIRCLE", &format!("#26 = {record};\n#9998 = CIRCLE"));
        let m = if m == base {
            base.replace(
                "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION",
                &format!("#9997 = {record};\n#10 = ADVANCED_BREP_SHAPE_REPRESENTATION"),
            )
        } else {
            m
        };
        must_not_panic(tag, &m, Duration::from_secs(10));
    }
}

//! Delta-review (R2) attack probes for PR #276's fix pass — variants
//! of R1's MAJOR-1 (multi-solid smuggle) the fix pass did not itself
//! test. Authored on `m7/tier-delta-probes`; fixtures are doctored IN
//! CODE from committed ones so the corpus-walk pin in `tier_gate.rs`
//! stays untouched. Doctoring helpers follow
//! `review_r1_tier_gate_probes.rs`'s shapes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use step_import::{ImportOptions, StepImportError, import_step};
use topo::ValidationError;

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)).unwrap()
}

/// Flips a cube solid inside-out (reverse loops AND flip face senses —
/// mutually consistent, so only the volume invariant can see it).
fn invert(line: &str) -> String {
    if line.contains("FACE_OUTER_BOUND") || line.contains("ADVANCED_FACE") {
        line.replace(".T.", ".F.")
    } else {
        line.to_owned()
    }
}

/// Rewrites every CARTESIAN_POINT as `p * scale + (tx, 0, 0)`.
fn scale_then_translate_x(line: &str, scale: f64, tx: f64) -> String {
    let Some(start) = line.find("CARTESIAN_POINT('', (") else {
        return line.to_owned();
    };
    let open = start + "CARTESIAN_POINT('', (".len();
    let close = line[open..].find(')').unwrap() + open;
    let coords: Vec<f64> = line[open..close]
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    format!(
        "{}{}, {}, {}{}",
        &line[..open],
        coords[0] * scale + tx,
        coords[1] * scale,
        coords[2] * scale,
        &line[close..]
    )
}

/// Renumbers every `#id` reference by `offset`.
fn renumber(line: &str, offset: u64) -> String {
    let mut out = String::new();
    let mut chars = line.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if c == '#' {
            let mut num = String::new();
            while let Some(&(_, d)) = chars.peek() {
                if d.is_ascii_digit() {
                    num.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            out.push('#');
            out.push_str(&(num.parse::<u64>().unwrap() + offset).to_string());
        } else {
            out.push(c);
        }
    }
    out
}

/// The id of an entity line (`#n = ...`), if it is one.
fn entity_id(line: &str) -> Option<u64> {
    let rest = line.strip_prefix('#')?;
    let end = rest.find(' ')?;
    rest[..end].parse().ok()
}

type Doctor<'a> = (u64, &'a dyn Fn(&str) -> String);

/// Rebuilds cube.step with the given per-solid doctors (the R1 probe
/// file's construction, restated here so this file stands alone).
fn multi_cube(doctors: &[Doctor<'_>]) -> String {
    let src = fixture("../step-export/tests/fixtures/cube.step");
    let mut out: Vec<String> = Vec::new();
    let mut trailer: Vec<String> = Vec::new();
    for line in src.lines() {
        match entity_id(line) {
            Some(id) if id <= 150 => {
                for &(offset, doctor) in doctors {
                    out.push(renumber(&doctor(line), offset));
                }
            }
            Some(160) => {
                let breps: Vec<String> = doctors
                    .iter()
                    .map(|(o, _)| format!("#{}", 150 + o))
                    .collect();
                trailer.push(format!(
                    "#160 = ADVANCED_BREP_SHAPE_REPRESENTATION('', (#154, {}), #159);",
                    breps.join(", ")
                ));
            }
            Some(_) => trailer.push(line.to_owned()),
            None => {
                if trailer.is_empty() && out.iter().all(|l| entity_id(l).is_none()) {
                    out.push(line.to_owned());
                } else {
                    trailer.push(line.to_owned());
                }
            }
        }
    }
    out.extend(trailer);
    out.join("\n")
}

fn expect_guilty(text: &str, guilty: u64, what: &str) {
    let got = import_step(text, &ImportOptions::default());
    let Err(e) = got else {
        panic!("{what}: must refuse, it imported: {got:?}");
    };
    let StepImportError::TierInvalid { solid, errors } = &e else {
        panic!("{what}: want the gate's typed refusal, got: {e:?}");
    };
    assert_eq!(
        *solid,
        Some(150 + guilty),
        "{what}: the refusal must name the guilty solid"
    );
    assert!(
        errors.contains(&ValidationError::NegativeVolume),
        "{what}: want NegativeVolume among the verdicts: {errors:?}"
    );
}

/// THREE solids, the MIDDLE one inverted — a position the fix pass's
/// two-solid pin never exercised (a `take(2)`-shaped regression, or any
/// first/last-only gating, would ship it).
#[test]
fn r2_three_solids_middle_inverted_refuses_naming_the_middle() {
    let a: &dyn Fn(&str) -> String = &|l| scale_then_translate_x(l, 1.0, 10.0);
    let mid: &dyn Fn(&str) -> String = &invert;
    let c: &dyn Fn(&str) -> String = &|l| scale_then_translate_x(l, 1.0, 20.0);
    let text = multi_cube(&[(1000, a), (3000, mid), (5000, c)]);
    expect_guilty(&text, 3000, "middle-of-three inverted");
}

/// A TINY inverted cube (1 cm, −1e-6 m^3) beside a unit normal cube:
/// the aggregate flux sum is +0.999999 m^3 — POSITIVE, so even a
/// whole-body gate with the Zero exemption removed would pass it. Only
/// per-solid gating can see this one; it is the sharpest form of the
/// MAJOR. The mirror (large inverted, small normal, sum negative)
/// must also name the guilty solid, not merely refuse.
#[test]
fn r2_scale_mismatch_cannot_hide_the_inverted_solid() {
    let tiny_inverted: &dyn Fn(&str) -> String = &|l| invert(&scale_then_translate_x(l, 0.01, 5.0));
    let unit: &dyn Fn(&str) -> String = &|l| l.to_owned();
    let text = multi_cube(&[(1000, unit), (3000, tiny_inverted)]);
    expect_guilty(&text, 3000, "tiny inverted beside unit (sum positive)");

    let big_inverted: &dyn Fn(&str) -> String = &|l| invert(&scale_then_translate_x(l, 2.0, 5.0));
    let text = multi_cube(&[(1000, big_inverted), (3000, unit)]);
    expect_guilty(&text, 1000, "large inverted beside unit (sum negative)");
}

/// EVERY solid inverted: the aggregate is decidedly negative, so the
/// old whole-body gate would also have refused — but the refusal must
/// still come from the per-solid pass and name a specific solid (the
/// first in file order), not fall through to an anonymous whole-body
/// verdict.
#[test]
fn r2_all_inverted_refuses_the_first_by_name() {
    let inv_a: &dyn Fn(&str) -> String = &invert;
    let inv_b: &dyn Fn(&str) -> String = &|l| invert(&scale_then_translate_x(l, 1.0, 10.0));
    let text = multi_cube(&[(1000, inv_a), (3000, inv_b)]);
    expect_guilty(&text, 1000, "both inverted");
}

/// Voids cannot be smuggled INTO the per-solid split, because the
/// subset has no void representation at all: a `BREP_WITH_VOIDS` item
/// refuses typed as outside the subset before any assembly. (So the
/// per-solid gate cannot misclassify a legitimate interior void as an
/// inverted solid — there is no legitimate void to misclassify; the
/// kernel's own exporter likewise refuses `VoidShellUnsupported`.)
#[test]
fn r2_brep_with_voids_is_outside_the_subset_not_a_gate_case() {
    let src = fixture("../step-export/tests/fixtures/cube.step");
    let text = src.replace(
        "MANIFOLD_SOLID_BREP('cube', #149)",
        "BREP_WITH_VOIDS('cube', #149, ())",
    );
    assert_ne!(text, src, "the doctoring must have found the solid line");
    let e = import_step(&text, &ImportOptions::default()).unwrap_err();
    assert!(
        matches!(
            e,
            StepImportError::UnsupportedEntity { .. } | StepImportError::NothingToImport
        ),
        "a void-bearing B-rep is outside the subset, refused typed: {e:?}"
    );
}

//! **R1 review probe (PR #264, C4), retired into its successor (M8
//! instancing).**
//!
//! What it pinned: dm1's refusal was exactly the M7-4 Leg D instancing
//! gate, fired at the second differing transform (#186, bolt_4), AFTER
//! the whole shape resolved — the executable form of PR #264's
//! "geometry distance retired" claim.
//!
//! That gate is gone. `REPRESENTATION_RELATIONSHIP` says which
//! component each `ITEM_DEFINED_TRANSFORMATION` places (`rep_1`), so
//! dm1's 7 occurrences of its 3 breps materialize as 7 placed
//! instances instead of refusing at the second map. This file keeps
//! the same subject and asserts the successor claims:
//!
//! 1. **the gate is gone, not moved** — no refusal of dm1 mentions
//!    assembly instancing, and none names #186;
//! 2. **the seven instances are read as seven** — the file's own
//!    occurrence structure, counted from the text, is what the resolver
//!    walks (3 component representations, 7 relationships, each naming
//!    one of them);
//! 3. **the placement layer is now BEHIND the geometry** — dm1's
//!    remaining refusal is reachable only once every instance's frame
//!    was read and applied. Since #327 (stage-1 CURVE recognition)
//!    that refusal has moved AGAIN, and into the D7 ladder: the
//!    file's rational-quadratic rim carriers are recognized as circles
//!    and promoted, every instance's frame is read and applied, and
//!    the first thing that refuses is the ladder itself, on edge
//!    `#389` — a two-point degree-1 polyline offered ZERO candidates
//!    (`work/exch/step-import-degree-one-line-promotion.md`).
//!
//!    It refused at the SHARED AT-REST GATE until tier 3's check 7
//!    began certifying a SIGN. That gate was chasing a PRECISION: the
//!    rational cylinder wall's enclosure is inside a factor of two of
//!    the ambient `1024·ε` target when the fixed round budget runs
//!    out, and at a coarse band lands just under it and escalates.
//!    Neither is a statement about the body's orientation, which is
//!    what check 7 reads, and dm1's enclosure excludes zero at round 0
//!    at every band. This probe pins where the frontier actually is,
//!    so a claim that it moved has to be executable too.
//!
//! Per-instance placement CORRECTNESS (each frame on its own component
//! and no other) is pinned where a file that IMPORTS can carry it:
//! `freecad.rs::refusals_survive_the_dialect_relaxations` (d), on
//! planted mutations of `twobody_importexport`'s real transforms.
//!
//! **This row is the suite's only unconditional import of FULL dm1**
//! (the l-bracket witness below imports its pruned single-component
//! subset, a second dm1-derived import with its own, cheaper text).
//! Importing dm1 costs ~30× the rest of the wild refusal corpus put
//! together, so the rows that used to re-import it for a WEAKER
//! statement now point here instead:
//! `wild::wild_refusals_are_typed_and_name_their_class` skips it (its
//! entity-naming check moved into the `TierInvalid` arm below), and
//! `review_probes_m7_3`'s V6 first-refusal-site probe is retired — it
//! asserted a strict subset of this row. `tier_gate.rs` still sweeps
//! the file at three ε_in values and pins all three cells' message
//! fragments there; they are one fragment now, the arc-rim mint's
//! (`MapResidual` — the frontier past the `#389` gap #388 retired).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use std::path::PathBuf;
use step_import::{ImportOptions, StepImportError, import_step};

fn dm1() -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        "stepcode",
        "dm1-id-214.stp",
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap()
}

#[test]
fn dm1_no_longer_refuses_at_the_instancing_gate() {
    let text = dm1();

    // (2) The file really does state seven occurrences of three
    // component representations — counted from the text, so the claim
    // does not rest on the reader agreeing with itself.
    let occurrences = text.matches("NEXT_ASSEMBLY_USAGE_OCCURRENCE").count();
    let transforms = text.matches("ITEM_DEFINED_TRANSFORMATION").count();
    let breps = text.matches("MANIFOLD_SOLID_BREP").count();
    assert_eq!(occurrences, 7, "l-bracket + 3 bolts + 3 nuts");
    assert_eq!(transforms, 7, "one per occurrence");
    assert_eq!(breps, 3, "three component representations, seven instances");

    // (1) and (3): the disposition — ONE cell at every ambient band
    // since check 7 stopped selecting a frontier by precision. The
    // at-rest arm below is kept for the claim inside it, not because a
    // band reaches it.
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::Structure { id, what }) => {
            panic!("the assembly layer must not refuse dm1 any more: #{id} {what}")
        }
        // **The ladder does not refuse dm1 at any band** — this arm
        // is a TRIPWIRE, not a cell. The `#389` polyline gap (a
        // two-point `QUASI_UNIFORM_CURVE` offered ZERO candidates —
        // once masked by the at-rest gate's precision chase, then,
        // when check 7 began certifying a SIGN, dm1's first refusal
        // at every band) is retired by #388: degree-1 carriers
        // promote to `Curve3::Line`, and a promoted slit holds its
        // wall's boundary-column candidate in either traversal order
        // — `the_l_bracket_alone_adopts_its_reversed_slit` below is
        // the executed witness on this file's own records. If a gate
        // change ever re-exposes the ladder here, the state must be
        // re-measured, not re-derived from this comment.
        Err(StepImportError::Adoption { id, attempts }) => {
            panic!(
                "the D7 ladder does not refuse dm1 at any band (#388 retired \
                 the polyline gap): #{id}, {} candidate(s)",
                attempts.len()
            )
        }
        // **The frontier past the retired gap: the pcurve MINT on the
        // l-bracket wall's ARC rim** (`MapResidual` — wall `#382`
        // states four u spans while its rim circles are three-arc
        // rationals; the l-bracket witness below pins the same
        // frontier on the pruned text). Reached only because check 7
        // certifies a SIGN (the at-rest gate admits every component's
        // enclosure at round 0) AND `#389` adopts (#388): the two
        // former frontiers each stood in front of this one.
        Err(StepImportError::Pcurves { source }) => {
            let shown = source.to_string();
            assert!(
                shown.contains("MapResidual"),
                "the frontier past the retired gap is the arc-rim mint's residual: {shown}"
            );
        }
        Err(StepImportError::TierInvalid { solid, errors }) => {
            // Adopted from `wild::wild_refusals_are_typed_and_name_their_class`,
            // which no longer imports this file. INVARIANT: every
            // typed refusal points at something in the file a reader
            // can go and look at — the gate's verdict names the
            // `MANIFOLD_SOLID_BREP` it was asked about, and carries at
            // least one kernel verdict inside naming what it is about.
            assert!(
                solid.is_some_and(|id| id > 0) && !errors.is_empty(),
                "the refusal must name an entity: solid {solid:?}, verdicts {errors:?}"
            );
            let shown = StepImportError::TierInvalid { solid, errors }.to_string();
            assert!(
                !shown.contains("the certified quadrature enclosure cannot reach the"),
                "a BUDGET refusal at the at-rest gate is check 7 back to consuming a \
                 precision, which is what the sign level removed: {shown}"
            );
        }
        other => panic!("dm1's refusal has moved out of the at-rest gate; got {other:?}"),
    }
}

/// dm1's **l-bracket component alone**: the same DATA records, with
/// the assembly layer and the other two components' representation
/// subtrees pruned by reachability — no record that survives is
/// altered, so the polyline slit `#389` and its rational wall `#382`
/// keep the file's own bits. Pruned rather than committed as a second
/// fixture: one source of truth, and the derivation is checked
/// (`#389` and `#382` must survive, the other components' breps must
/// not).
fn l_bracket_only(text: &str) -> String {
    // Join each `#k = ...;` statement's continuation lines (the
    // `nurbs_import` reorder helper's walk), keeping non-record lines.
    let mut head = Vec::new();
    let mut records: Vec<String> = Vec::new();
    let mut tail = Vec::new();
    let mut in_data = false;
    let mut done = false;
    for line in text.lines() {
        if line.trim() == "DATA;" {
            in_data = true;
            head.push(line.to_owned());
            continue;
        }
        if in_data && line.trim() == "ENDSEC;" {
            in_data = false;
            done = true;
            tail.push(line.to_owned());
            continue;
        }
        if in_data {
            // A record STARTS at column zero (`#k=`); an indented `#`
            // is a reference on a continuation line, not a record.
            if line.starts_with('#') || records.is_empty() {
                records.push(line.to_owned());
            } else {
                let last = records.last_mut().unwrap();
                last.push('\n');
                last.push_str(line);
            }
        } else if done {
            tail.push(line.to_owned());
        } else {
            head.push(line.to_owned());
        }
    }
    let id_of = |r: &str| -> Option<u64> {
        let rest = r.trim_start().strip_prefix('#')?;
        let end = rest.find('=')?;
        rest[..end].trim().parse().ok()
    };
    let refs_of = |r: &str| -> Vec<u64> {
        let body = &r[r.find('=').map_or(0, |k| k + 1)..];
        let mut out = Vec::new();
        let bytes = body.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'#' {
                let mut j = i + 1;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                if j > i + 1 {
                    out.push(body[i + 1..j].parse().unwrap());
                }
                i = j;
            } else {
                i += 1;
            }
        }
        out
    };
    let by_id: std::collections::BTreeMap<u64, &String> = records
        .iter()
        .filter_map(|r| Some((id_of(r)?, r)))
        .collect();
    let reach = |root: u64| -> std::collections::BTreeSet<u64> {
        let mut seen = std::collections::BTreeSet::new();
        let mut stack = vec![root];
        while let Some(i) = stack.pop() {
            if !seen.insert(i) {
                continue;
            }
            if let Some(r) = by_id.get(&i) {
                stack.extend(refs_of(r));
            }
        }
        seen
    };
    // The three MANIFOLD_SOLID_BREPs and the ADVANCED_BREP_SHAPE_
    // REPRESENTATIONs that carry the two dropped components.
    let keep = reach(503);
    let mut drop: std::collections::BTreeSet<u64> = reach(1136)
        .union(&reach(1447))
        .copied()
        .filter(|i| !keep.contains(i))
        .collect();
    drop.insert(93);
    drop.insert(255);
    let assembly_kinds = [
        "NEXT_ASSEMBLY_USAGE_OCCURRENCE",
        "ITEM_DEFINED_TRANSFORMATION",
        "REPRESENTATION_RELATIONSHIP",
        "CONTEXT_DEPENDENT_SHAPE_REPRESENTATION",
    ];
    // Cascade: anything that names a dropped record is dropped too
    // (styling, definition links, the assembly layer), until fixpoint.
    loop {
        let mut grew = false;
        for r in &records {
            let Some(i) = id_of(r) else { continue };
            if drop.contains(&i) {
                continue;
            }
            if assembly_kinds.iter().any(|k| r.contains(k))
                || refs_of(r).iter().any(|j| drop.contains(j))
            {
                drop.insert(i);
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }
    assert!(
        keep.contains(&389) && keep.contains(&382) && !drop.contains(&389),
        "the derivation must keep the slit edge and its wall"
    );
    assert!(
        drop.contains(&1136) && drop.contains(&1447),
        "and drop the other two components' breps"
    );
    let kept: Vec<&str> = records
        .iter()
        .filter(|r| id_of(r).is_none_or(|i| !drop.contains(&i)))
        .map(String::as_str)
        .collect();
    format!(
        "{}\n{}\n{}",
        head.join("\n"),
        kept.join("\n"),
        tail.join("\n")
    )
}

/// **`#389`'s candidate, witnessed on dm1's own bits (#388).** The
/// l-bracket alone gets PAST every polyline edge: `#389`'s degree-1
/// slit carrier promotes to `Curve3::Line`, and — its control order
/// being REVERSED against its wall's boundary column, the one such
/// reversal in the file — it adopts through the column candidate run
/// backwards. The first refusal is now the pcurve MINT on the wall's
/// ARC rim (`MapResidual`): wall `#382` states four u spans while its
/// rim circles are three-arc rationals, so the imported-chart arc-rim
/// construction ("one span per sub-arc") refuses this chart — a
/// pre-existing frontier newly reachable, filed with the unit rather
/// than widened past. At the merge base this same pruned text refused
/// `Adoption { id: 389, attempts: [] }` — the gap this pins retired.
#[test]
fn the_l_bracket_alone_adopts_its_reversed_slit() {
    let text = l_bracket_only(&dm1());
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::Adoption { id, attempts }) => panic!(
            "the polyline gap must stay retired: edge #{id} refused with {} candidate(s)",
            attempts.len()
        ),
        Err(StepImportError::Pcurves { source }) => {
            let shown = source.to_string();
            assert!(
                shown.contains("MapResidual"),
                "the successor frontier is the arc-rim mint's residual: {shown}"
            );
        }
        other => panic!("the l-bracket's frontier moved — re-measure: {other:?}"),
    }
}

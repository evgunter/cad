//! The demo-scene K-telemetry sweep (M4 PR 8b, D3): rebuilds every
//! tour scene at the recording scalar [`Probe`] — the SAME generic
//! constructors the f64 tour runs, so there is no duplicated scene
//! data to drift — and dumps every recorded `MarginSample` as CSV in
//! the M2 K-report file convention (`shape,predicate,margin,
//! band_zero,band_escalate,outcome`), shapes namespaced
//! `demo/<scene>` (the corpus harness `m4_pr8_k_probe.rs` namespaces
//! `corpus/<doc>`).
//!
//! Per body the sweep mirrors `crate::run_body`'s validation ladder
//! (tiers 1 + 2, then 3′ with declared contacts for boolean results
//! or plain tier 3 otherwise, then exact mass properties) — minus the
//! mesh/STL/STEP export lanes, which decide nothing at kernel level.
//! The lily's wall probes carry the refusal-path samples the M2 report
//! asked for (refusal-path predicates never sample on all-valid
//! corpora). The bowtie row that used to ride beside them left with the
//! finale scene (LIB-RETTAIL); its refusal is pinned in the profile
//! crate's own validation suite now, outside the K sweep.
//!
//! One process per ε (`Tolerance` is a OnceLock):
//!
//! ```sh
//! CAD_TOLERANCE_EPS=1e-6 cargo run -- k-probe /tmp/demos-eps-1e-6.csv
//! ```

use std::io::Write as _;

use pncad::geom_core::Sign;
use pncad::geom_core::k_stats::{self, Probe, SampleOutcome};
use pncad::topo::{Body, ContactRecords};

use crate::{
    az, bodies, bool_bodies, bossplate, crosslap, curvedcut, cutaway, heatsink, letterforms, lily,
    projectbox, rocker,
};
use pncad::geom_core::Tol;

/// One probed body: label, body, declared contacts if it is a boolean
/// result (selects the 3′ gate, exactly as in `run_body`).
type ProbeBody = (String, Body<Probe>, Option<ContactRecords>);

fn plain(name: &str, b: Body<Probe>) -> ProbeBody {
    (name.to_string(), b, None)
}

fn seamed(name: &str, bb: pncad::topo::BooleanBody<Probe>) -> ProbeBody {
    (name.to_string(), bb.body, Some(bb.contacts))
}

/// `run_body`'s validation ladder, export lanes omitted.
fn validate_probe(label: &str, body: &Body<Probe>, contacts: Option<&ContactRecords>, tol: Tol) {
    pncad::topo::validate(body).unwrap_or_else(|e| panic!("{label}: tier 1 at Probe: {e:?}"));
    pncad::topo::validate_closed(body)
        .unwrap_or_else(|e| panic!("{label}: tier 2 at Probe: {e:?}"));
    match contacts {
        Some(c) => pncad::topo::validate_pseudomanifold(body, c, tol)
            .unwrap_or_else(|e| panic!("{label}: tier 3' at Probe: {e:?}")),
        None => pncad::topo::validate_geometric(body, tol)
            .unwrap_or_else(|e| panic!("{label}: tier 3 at Probe: {e:?}")),
    }
    pncad::topo::mass_properties(body, tol)
        .unwrap_or_else(|e| panic!("{label}: mass at Probe: {e:?}"));
}

fn outcome_str(o: SampleOutcome) -> &'static str {
    match o {
        SampleOutcome::Definite(Sign::Negative) => "negative",
        SampleOutcome::Definite(Sign::Zero) => "zero",
        SampleOutcome::Definite(Sign::Positive) => "positive",
        SampleOutcome::Indeterminate => "indeterminate",
        SampleOutcome::Invalid => "invalid",
    }
}

/// Builds one scene group under a fresh recording, validates its
/// bodies, and appends every sample as a `demo/<scene>` CSV row.
fn sweep<F: FnOnce() -> Vec<ProbeBody>>(
    csv: &mut String,
    total: &mut usize,
    unnamed: &mut usize,
    scene: &str,
    build: F,
    tol: Tol,
) {
    k_stats::start_recording();
    let bodies = build();
    for (label, body, contacts) in &bodies {
        validate_probe(label, body, contacts.as_ref(), tol);
    }
    let samples = k_stats::take_samples();
    *total += samples.len();
    for s in &samples {
        if s.predicate == "<unnamed>" {
            *unnamed += 1;
        }
        csv.push_str(&format!(
            "demo/{scene},{},{:e},{:e},{:e},{}\n",
            s.predicate,
            s.margin,
            s.band_zero,
            s.band_escalate,
            outcome_str(s.outcome)
        ));
    }
    eprintln!("k_probe(demo): {scene}: {} samples", samples.len());
}

/// The sweep entry point (`demo-tour k-probe [out.csv]`; stdout when
/// no path is given).
pub fn run(out: Option<String>, tol: Tol) {
    let eps = tol.eps();
    let mut csv = String::from("shape,predicate,margin,band_zero,band_escalate,outcome\n");
    let (mut total, mut unnamed) = (0usize, 0usize);
    let s = &mut csv;
    let t = &mut total;
    let u = &mut unnamed;

    sweep(
        s,
        t,
        u,
        "bracket",
        || vec![plain("bracket", bodies::bracket(tol))],
        tol,
    );
    sweep(
        s,
        t,
        u,
        "plate",
        || vec![plain("plate", bodies::plate(tol))],
        tol,
    );
    sweep(
        s,
        t,
        u,
        "vase",
        || vec![plain("vase", bodies::vase(tol))],
        tol,
    );
    sweep(
        s,
        t,
        u,
        "sheave",
        || vec![plain("sheave", bodies::sheave(tol).0)],
        tol,
    );
    sweep(
        s,
        t,
        u,
        "chute",
        || vec![plain("chute", bodies::chute(tol).0)],
        tol,
    );
    // The fillet gates are reified K-funnel predicates (S2's seven),
    // so the rocker's six filleted corners get their own sweep group.
    sweep(
        s,
        t,
        u,
        "rocker",
        || vec![plain("rocker", rocker::rocker(tol))],
        tol,
    );
    // The PR 11 flip: the tilted cut joined the standard ladder, so
    // its quadrature-lane predicates (props_quad_*) record here too.
    sweep(
        s,
        t,
        u,
        "tiltedcut",
        || {
            let (above, below) = curvedcut::build::<Probe>(tol);
            vec![
                plain("tiltedcut_above", above),
                plain("tiltedcut_below", below),
            ]
        },
        tol,
    );
    sweep(
        s,
        t,
        u,
        "bossplate",
        || {
            // Contact-free transverse curved boolean: plain tier 3 (the 3′
            // census is exact-on-planar; see the stop's routing note).
            let bb = bossplate::build::<Probe>(tol);
            let contact_free = bb.contacts.vv.is_empty()
                && bb.contacts.a_on_b.is_empty()
                && bb.contacts.b_on_a.is_empty();
            vec![if contact_free {
                plain("bossplate", bb.body)
            } else {
                seamed("bossplate", bb)
            }]
        },
        tol,
    );
    sweep(
        s,
        t,
        u,
        "die",
        || vec![seamed("die", bool_bodies::die(tol).0)],
        tol,
    );
    sweep(
        s,
        t,
        u,
        "table",
        || vec![seamed("table", bool_bodies::table(tol).0)],
        tol,
    );
    sweep(
        s,
        t,
        u,
        "voidbox",
        || vec![seamed("voidbox", bool_bodies::voidbox(tol))],
        tol,
    );
    sweep(
        s,
        t,
        u,
        "letterforms",
        || {
            let (two, three) = letterforms::build(tol);
            vec![seamed("silhouette", two), seamed("silhouette3", three)]
        },
        tol,
    );
    sweep(s, t, u, "az", || vec![seamed("az", az::build(tol))], tol);
    sweep(
        s,
        t,
        u,
        "crosslap",
        || {
            let (a, b, glued, b_lifted, _refusal) = crosslap::build(tol);
            vec![
                seamed("crosslap_a", a),
                seamed("crosslap_b", b),
                seamed("crosslap_glued", glued),
                plain("crosslap_exp_b", b_lifted),
            ]
        },
        tol,
    );
    sweep(
        s,
        t,
        u,
        "projectbox_cutaway",
        || {
            // The cutaway splits the SAME box the projectbox stop ships, so
            // the two scenes share one sweep group (rebuilding the 15-op
            // chain per group would double-count its samples).
            let (acc, _vol) = projectbox::build(tol);
            let boxbody = acc.body.clone();
            let ((above, below), _numbers) = cutaway::build(&boxbody, tol);
            vec![
                seamed("projectbox", acc),
                plain("cutaway_above", above),
                plain("cutaway_below", below),
            ]
        },
        tol,
    );
    sweep(
        s,
        t,
        u,
        "heatsink",
        || {
            heatsink::probe_solids(tol)
                .into_iter()
                .enumerate()
                .map(|(i, bb)| seamed(&format!("heatsink_{}", [5, 7, 9][i]), bb))
                .collect()
        },
        tol,
    );
    // The fairy lantern: the only WINDOWED TUBE built from world-
    // coordinate intent in the corpus (torus segments), plus
    // doubly-truncated sphere zones and crescent-section blades swept
    // along arching spines.
    sweep(
        s,
        t,
        u,
        "lily",
        || {
            lily::plant::<Probe>(tol)
                .into_iter()
                .map(|piece| plain(piece.name, piece.body))
                .collect()
        },
        tol,
    );
    // Refusal-path samples again (the M2 K report's standing gap): the
    // lily's seven walls all refuse, and their margins record here.
    sweep(
        s,
        t,
        u,
        "lily_walls",
        || {
            lily::wall_probes::<Probe>(tol);
            Vec::new()
        },
        tol,
    );

    assert_eq!(
        unnamed, 0,
        "<unnamed> must be unreachable from shipped decide paths"
    );
    eprintln!("k_probe(demo): eps={eps:e}, {total} samples");
    match out {
        Some(path) => {
            let mut f = std::fs::File::create(&path).expect("create k-probe output file");
            f.write_all(csv.as_bytes()).expect("write csv");
            eprintln!("k_probe(demo): wrote {path}");
        }
        None => print!("{csv}"),
    }
}

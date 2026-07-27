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
//! The finale's bowtie refusal runs too: the M2 report noted
//! refusal-path predicates never sample on all-valid corpora.
//!
//! One process per ε (`Tolerance` is a OnceLock):
//!
//! ```sh
//! CAD_TOLERANCE_EPS=1e-6 cargo run -- k-probe /tmp/demos-eps-1e-6.csv
//! ```

use std::io::Write as _;

use geom_core::Sign;
use geom_core::k_stats::{self, Probe, SampleOutcome};
use topo::{Body, ContactRecords};

use crate::{az, bodies, bool_bodies, crosslap, cutaway, heatsink, letterforms, projectbox};

/// One probed body: label, body, declared contacts if it is a boolean
/// result (selects the 3′ gate, exactly as in `run_body`).
type ProbeBody = (String, Body<Probe>, Option<ContactRecords>);

fn plain(name: &str, b: Body<Probe>) -> ProbeBody {
    (name.to_string(), b, None)
}

fn seamed(name: &str, bb: topo::BooleanBody<Probe>) -> ProbeBody {
    (name.to_string(), bb.body, Some(bb.contacts))
}

/// `run_body`'s validation ladder, export lanes omitted.
fn validate_probe(label: &str, body: &Body<Probe>, contacts: Option<&ContactRecords>) {
    topo::validate(body).unwrap_or_else(|e| panic!("{label}: tier 1 at Probe: {e:?}"));
    topo::validate_closed(body).unwrap_or_else(|e| panic!("{label}: tier 2 at Probe: {e:?}"));
    match contacts {
        Some(c) => topo::validate_pseudomanifold(body, c)
            .unwrap_or_else(|e| panic!("{label}: tier 3' at Probe: {e:?}")),
        None => topo::validate_geometric(body)
            .unwrap_or_else(|e| panic!("{label}: tier 3 at Probe: {e:?}")),
    }
    topo::mass_properties(body).unwrap_or_else(|e| panic!("{label}: mass at Probe: {e:?}"));
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
) {
    k_stats::start_recording();
    let bodies = build();
    for (label, body, contacts) in &bodies {
        validate_probe(label, body, contacts.as_ref());
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
pub fn run(out: Option<String>) {
    let eps = geom_core::Tolerance::get().eps;
    let mut csv = String::from("shape,predicate,margin,band_zero,band_escalate,outcome\n");
    let (mut total, mut unnamed) = (0usize, 0usize);
    let s = &mut csv;
    let t = &mut total;
    let u = &mut unnamed;

    sweep(s, t, u, "bracket", || {
        vec![plain("bracket", bodies::bracket())]
    });
    sweep(s, t, u, "plate", || vec![plain("plate", bodies::plate())]);
    sweep(s, t, u, "vase", || vec![plain("vase", bodies::vase())]);
    sweep(s, t, u, "sheave", || {
        vec![plain("sheave", bodies::sheave().0)]
    });
    sweep(s, t, u, "chute", || vec![plain("chute", bodies::chute().0)]);
    sweep(s, t, u, "die", || vec![seamed("die", bool_bodies::die().0)]);
    sweep(s, t, u, "table", || {
        vec![seamed("table", bool_bodies::table().0)]
    });
    sweep(s, t, u, "voidbox", || {
        vec![seamed("voidbox", bool_bodies::voidbox())]
    });
    sweep(s, t, u, "letterforms", || {
        let (two, three) = letterforms::build();
        vec![seamed("silhouette", two), seamed("silhouette3", three)]
    });
    sweep(s, t, u, "az", || vec![seamed("az", az::build())]);
    sweep(s, t, u, "crosslap", || {
        let (a, b, b_lifted, _refusal) = crosslap::build();
        vec![
            seamed("crosslap_a", a),
            seamed("crosslap_b", b),
            plain("crosslap_exp_b", b_lifted),
        ]
    });
    sweep(s, t, u, "projectbox_cutaway", || {
        // The cutaway splits the SAME box the projectbox stop ships, so
        // the two scenes share one sweep group (rebuilding the 15-op
        // chain per group would double-count its samples).
        let (acc, _vol) = projectbox::build();
        let boxbody = acc.body.clone();
        let ((above, below), _numbers) = cutaway::build(&boxbody);
        vec![
            seamed("projectbox", acc),
            plain("cutaway_above", above),
            plain("cutaway_below", below),
        ]
    });
    sweep(s, t, u, "heatsink", || {
        heatsink::probe_solids()
            .into_iter()
            .enumerate()
            .map(|(i, bb)| seamed(&format!("heatsink_{}", [5, 7, 9][i]), bb))
            .collect()
    });
    sweep(s, t, u, "finale_bowtie", || {
        // Refusal-path samples: the bowtie profile is REFUSED typed —
        // no body comes back, but its validation margins record.
        bodies::finale_fail_loud::<Probe>();
        Vec::new()
    });

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

//! PERF LANE HARNESS (perf/explore-kernel, never merged) — per-document,
//! per-stage wall clock over the Band 4 corpus, on the RELEASE profile.
//!
//! `cargo run --release --example perf_kernel -p editor-core -- [reps] [filter]`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]
#![allow(missing_docs, unreachable_pub)]

use std::time::Instant;

use editor_core::{CancelToken, EvalOptions, Evaluation, ProfileDoc, apply, evaluate};
use geom_core::Tol;

#[path = "../tests/corpus/mod.rs"]
mod corpus;
#[path = "../tests/fixture/mod.rs"]
mod fixture;

fn ms(nanos: u128) -> f64 {
    nanos as f64 / 1e6
}

fn stats(mut v: Vec<f64>) -> (f64, f64, f64) {
    v.sort_by(f64::total_cmp);
    (v[v.len() / 2], v[0], v[v.len() - 1])
}

fn eval_once(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn main() {
    let mut args = std::env::args().skip(1);
    let reps: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(5);
    let filter = args.next().unwrap_or_default();
    let trace = args.next().is_some_and(|s| s == "trace");

    println!("# perf_kernel — release, reps={reps}");
    println!("doc\tnodes\tfull_med_ms\tfull_min\tfull_max\tincr_med_ms\tincr_min\tincr_max\tcone");

    let mut stage_report: Vec<(String, Vec<(&'static str, u128, u64)>, Vec<(&'static str, u128, u64)>)> =
        Vec::new();

    for d in corpus::documents() {
        if !filter.is_empty() && !d.name.contains(&filter) {
            continue;
        }
        // Warm-up (allocator, lazily-built tables).
        let _ = eval_once(&d.doc, None);

        let mut full = Vec::new();
        for _ in 0..reps {
            let t0 = Instant::now();
            let ev = eval_once(&d.doc, None);
            full.push(t0.elapsed().as_secs_f64() * 1e3);
            std::hint::black_box(&ev);
        }

        // One armed rep for the stage breakdown.
        geom_core::perf_probe::reset();
        geom_core::perf_probe::enable(true);
        geom_core::perf_probe::enable_trace(trace);
        let base = eval_once(&d.doc, None);
        geom_core::perf_probe::enable(false);
        geom_core::perf_probe::enable_trace(false);
        let full_stages = geom_core::perf_probe::take();
        if trace {
            for (i, (stage, nanos)) in geom_core::perf_probe::take_trace().iter().enumerate() {
                println!("  trace[{i}] {stage} {:.3} ms", ms(*nanos));
            }
        }

        let bumped = d.bumped();
        let mut incr = Vec::new();
        for _ in 0..reps {
            let t0 = Instant::now();
            let ev = eval_once(&bumped, Some(&base));
            incr.push(t0.elapsed().as_secs_f64() * 1e3);
            std::hint::black_box(&ev);
        }
        geom_core::perf_probe::reset();
        geom_core::perf_probe::enable(true);
        let inc_ev = eval_once(&bumped, Some(&base));
        geom_core::perf_probe::enable(false);
        let incr_stages = geom_core::perf_probe::take();

        let (fm, fmin, fmax) = stats(full);
        let (im, imin, imax) = stats(incr);
        println!(
            "{}\t{}\t{fm:.2}\t{fmin:.2}\t{fmax:.2}\t{im:.2}\t{imin:.2}\t{imax:.2}\t{}/{}",
            d.name,
            d.len(),
            inc_ev.reused,
            inc_ev.recomputed,
        );
        stage_report.push((d.name.to_string(), full_stages, incr_stages));
    }

    // The assembly/commit lane at the pinned split point: the heat
    // sink driven to N fins, which is what a Python or GUI caller
    // lands when it asks for checks or an assembly.
    if filter.is_empty() || filter == "split" {
        println!("\n# the gather/checks split (heat sink at N fins; 3 reps, median ms)");
        println!("fins\tsolids\tfaces\teval_ms\tgather_ms\tchecks_on_ms\trun_checks_ms\tassemble_ms\ttier3_aggregate_ms");
        for fins in [10_i64, 40, 160] {
            let tol = Tol::witness();
            let base = corpus::documents()
                .into_iter()
                .find(|d| d.name == "heat_sink")
                .expect("the corpus carries the heat sink");
            let doc = apply(
                &base.doc,
                &editor_core::DocEdit::SetDocParam {
                    name: editor_core::ParamName::new("fins"),
                    value: editor_core::DocParam::Count { value: fins },
                },
                tol,
            )
            .expect("the fin count is a document parameter")
            .doc;
            let mut t = Vec::new();
            for _ in 0..3 {
                let t0 = Instant::now();
                let ev = eval_once(&doc, None);
                t.push(t0.elapsed().as_secs_f64() * 1e3);
                std::hint::black_box(&ev);
            }
            let eval_ms = stats(t).0;
            let ev = eval_once(&doc, None);
            let mut t = Vec::new();
            let mut solids = 0;
            let mut faces = 0;
            for _ in 0..3 {
                let t0 = Instant::now();
                let p = editor_core::product_recorded(&doc, &ev, tol);
                t.push(t0.elapsed().as_secs_f64() * 1e3);
                if let Ok(p) = &p {
                    solids = p.body.solids().count();
                    faces = p.body.faces().count();
                }
            }
            let gather_ms = stats(t).0;
            let gathered = editor_core::product_recorded(&doc, &ev, tol).ok();
            let cfg = editor_core::ChecksConfig::default();
            let mut t = Vec::new();
            for _ in 0..3 {
                let t0 = Instant::now();
                let r = gathered.as_ref().map(|g| {
                    editor_core::run_checks_on(&doc, &ev, editor_core::Subject::Product(g), &cfg, tol)
                });
                t.push(t0.elapsed().as_secs_f64() * 1e3);
                std::hint::black_box(&r);
            }
            let checks_on_ms = stats(t).0;
            let mut t = Vec::new();
            for _ in 0..3 {
                let t0 = Instant::now();
                let r = editor_core::run_checks(&doc, &ev, &cfg, tol);
                t.push(t0.elapsed().as_secs_f64() * 1e3);
                std::hint::black_box(&r).as_ref().ok();
            }
            let run_checks_ms = stats(t).0;
            let mut t = Vec::new();
            for _ in 0..3 {
                let t0 = Instant::now();
                let r = editor_core::assemble(&doc, &ev, tol);
                t.push(t0.elapsed().as_secs_f64() * 1e3);
                std::hint::black_box(&r).as_ref().ok();
            }
            let assemble_ms = stats(t).0;
            let mut t = Vec::new();
            for _ in 0..3 {
                let t0 = Instant::now();
                let r = gathered
                    .as_ref()
                    .map(|g| topo::validate::validate_pseudomanifold(&g.body, &g.contacts, tol));
                t.push(t0.elapsed().as_secs_f64() * 1e3);
                std::hint::black_box(&r);
            }
            let tier3_ms = stats(t).0;
            println!(
                "{fins}\t{solids}\t{faces}\t{eval_ms:.1}\t{gather_ms:.1}\t{checks_on_ms:.1}\t{run_checks_ms:.1}\t{assemble_ms:.1}\t{tier3_ms:.1}"
            );
        }
    }

    println!("\n# downstream lanes on the result body (3 reps each, median ms)");
    println!("doc\tfaces\ttess1e-3\ttris\ttess1e-4\ttris\tmassprops\ttier3\tstl_bin\tstep");
    for d in corpus::documents() {
        if !filter.is_empty() && !d.name.contains(&filter) {
            continue;
        }
        let Some(result) = d.result else { continue };
        let ev = eval_once(&d.doc, None);
        let Some(v) = ev.value(result) else { continue };
        let body = match &v.payload {
            editor_core::ValuePayload::Body(b) => (**b).clone(),
            editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body {
                body, ..
            }) => (**body).clone(),
            _ => continue,
        };
        let tol = Tol::witness();
        let mut row = vec![format!("{}", d.name), format!("{}", body.faces().count())];
        for delta in [1e-3_f64, 1e-4_f64] {
            let mut t = Vec::new();
            let mut tris = 0usize;
            for _ in 0..3 {
                let t0 = Instant::now();
                match mesh::tessellate(&body, delta, tol) {
                    Ok(m) => {
                        tris = m.patches.iter().map(|p| p.triangles.len()).sum();
                        t.push(t0.elapsed().as_secs_f64() * 1e3);
                    }
                    Err(_) => {
                        t.push(f64::NAN);
                    }
                }
            }
            row.push(format!("{:.2}", stats(t).0));
            row.push(format!("{tris}"));
        }
        let mut t = Vec::new();
        for _ in 0..3 {
            let t0 = Instant::now();
            let m = topo::props::mass_properties(&body, tol);
            t.push(t0.elapsed().as_secs_f64() * 1e3);
            std::hint::black_box(&m).as_ref().ok();
        }
        row.push(format!("{:.3}", stats(t).0));
        let mut t = Vec::new();
        for _ in 0..3 {
            let t0 = Instant::now();
            let r = topo::validate::validate_geometric(&body, tol);
            t.push(t0.elapsed().as_secs_f64() * 1e3);
            std::hint::black_box(&r).as_ref().ok();
        }
        row.push(format!("{:.3}", stats(t).0));
        // STL binary, from the 1e-3 mesh.
        match mesh::tessellate(&body, 1e-3, tol) {
            Ok(m) => {
                let mut t = Vec::new();
                for _ in 0..3 {
                    let mut sink = Vec::new();
                    let t0 = Instant::now();
                    let r = stl::write_binary(&m, &stl::BinaryOptions::default(), &mut sink);
                    t.push(t0.elapsed().as_secs_f64() * 1e3);
                    std::hint::black_box(&r).as_ref().ok();
                }
                row.push(format!("{:.3}", stats(t).0));
            }
            Err(_) => row.push("n/a".into()),
        }
        let mut t = Vec::new();
        let mut ok = true;
        for _ in 0..3 {
            let t0 = Instant::now();
            let r = step_export::step_string(&body, &step_export::StepOptions::default(), tol);
            t.push(t0.elapsed().as_secs_f64() * 1e3);
            ok = r.is_ok();
        }
        row.push(if ok {
            format!("{:.3}", stats(t).0)
        } else {
            "refused".into()
        });
        println!("{}", row.join("\t"));
    }

    println!("\n# stage breakdown (one armed rep each; nested spans are INCLUSIVE)");
    for (name, full, incr) in stage_report {
        println!("\n## {name}");
        let mut f = full;
        f.sort_by(|a, b| b.1.cmp(&a.1));
        for (stage, nanos, calls) in f.iter().take(14) {
            println!("  full  {stage:<32} {:>9.2} ms  x{calls}", ms(*nanos));
        }
        let mut i = incr;
        i.sort_by(|a, b| b.1.cmp(&a.1));
        for (stage, nanos, calls) in i.iter().take(8) {
            println!("  incr  {stage:<32} {:>9.2} ms  x{calls}", ms(*nanos));
        }
    }
}

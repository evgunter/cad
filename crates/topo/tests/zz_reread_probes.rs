//! Review probes (PR 4234): every op, both orders, every pose; tally.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;
use common::meeting::{
    Hole, MEET, PLATE, Pose, arch, leaned, posed_box, posed_boxes, posed_prism, posed_pyramid,
    poses, three_wedges, two_wedges, wedge,
};
use geom_core::Tol;
use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use topo::{
    AtRestBody, BooleanError, BooleanResult, intersect, subtract, union, validate_geometric,
};

fn t() -> Tol {
    Tol::witness()
}

fn standing(bearing: f64, rise: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    let corner = |d: f64, r: f64| {
        let (s, c) = (bearing + d).to_radians().sin_cos();
        [r.mul_add(c, MEET[0]), r.mul_add(s, MEET[1]), MEET[2] + rise]
    };
    posed_pyramid(
        &[corner(15.0, r), corner(-15.0, r), corner(0.0, 0.6 * r)],
        MEET,
        pose,
    )
}
/// Apex at MEET, base below (a leg into the plate).
fn hanging(bearing: f64, drop: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    let corner = |d: f64, r: f64| {
        let (s, c) = (bearing + d).to_radians().sin_cos();
        [r.mul_add(c, MEET[0]), r.mul_add(s, MEET[1]), MEET[2] - drop]
    };
    posed_pyramid(
        &[corner(-15.0, r), corner(15.0, r), corner(0.0, 0.6 * r)],
        MEET,
        pose,
    )
}

fn vol(b: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(b, t()).unwrap().volume
}

#[derive(Debug, Clone)]
enum Out {
    Body(f64, bool),
    Empty,
    Err(String),
    Panic(String),
}

fn run(f: impl FnOnce() -> Result<BooleanResult<f64>, BooleanError>) -> Out {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(BooleanResult::Body(r))) => {
            let ok = validate_geometric(&r.body, t()).is_ok();
            Out::Body(vol(&r.body), ok)
        }
        Ok(Ok(BooleanResult::Empty)) => Out::Empty,
        Ok(Err(e)) => Out::Err(format!("{:?}", e.kind())),
        Err(p) => Out::Panic(
            p.downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default()
                .chars()
                .take(90)
                .collect(),
        ),
    }
}

fn try_union(x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> Option<AtRestBody<f64>> {
    match catch_unwind(AssertUnwindSafe(|| union(x, y, t()))) {
        Ok(Ok(BooleanResult::Body(r))) => Some(r.body),
        _ => None,
    }
}

/// Six ops; returns a row label per op and a soundness verdict per op.
fn six(label: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, tally: &mut BTreeMap<String, usize>) {
    let (vx, vy) = (vol(x), vol(y));
    let r = [
        ("x-y", run(|| subtract(x, y, t()))),
        ("y-x", run(|| subtract(y, x, t()))),
        ("xUy", run(|| union(x, y, t()))),
        ("yUx", run(|| union(y, x, t()))),
        ("x^y", run(|| intersect(x, y, t()))),
        ("y^x", run(|| intersect(y, x, t()))),
    ];
    let v = |o: &Out| match o {
        Out::Body(v, _) => Some(*v),
        Out::Empty => Some(0.0),
        _ => None,
    };
    let i = v(&r[4].1);
    for (op, o) in &r {
        // Volume oracle against the intersection where it built.
        let want = match (*op, i) {
            ("x-y", Some(i)) => Some(vx - i),
            ("y-x", Some(i)) => Some(vy - i),
            ("xUy" | "yUx", Some(i)) => Some(vx + vy - i),
            ("y^x", Some(i)) => Some(i),
            _ => None,
        };
        let key = match o {
            Out::Body(got, t3) => {
                let vol_ok = want.is_none_or(|w| (got - w).abs() < 1e-7);
                format!(
                    "BUILT tier3={t3} vol_ok={vol_ok}{}",
                    if want.is_none() { "(unchecked)" } else { "" }
                )
            }
            Out::Empty => "EMPTY".into(),
            Out::Err(k) => format!("ERR {k}"),
            Out::Panic(m) => format!("PANIC {m}"),
        };
        eprintln!("ROW {label} {op}: {key}");
        *tally.entry(key).or_default() += 1;
    }
}

#[test]
fn reread_probe_matrix() {
    let mut tally = BTreeMap::new();
    let mut sk = 0;
    for pose in poses() {
        let p = &pose.label;
        let plate = posed_box("plate", PLATE, &pose);
        let cone = standing(240.0, 0.7, 0.5, &pose);
        // F1: fold plate + standing pyramids at one point, plate first.
        let mut acc = plate.clone();
        for (k, b) in [60.0, 180.0, 300.0].into_iter().enumerate() {
            let pyr = standing(b, 0.5, 0.4, &pose);
            if k > 0 {
                six(
                    &format!("[{p}] fold plate+pyr step{k}"),
                    &acc,
                    &pyr,
                    &mut tally,
                );
            }
            match try_union(&acc, &pyr) {
                Some(u) => acc = u,
                None => {
                    eprintln!("ROW [{p}] fold stops at {k}");
                    break;
                }
            }
        }
        // F2: hanging legs (apex at MEET, into the plate) — touch from the other side.
        let legs = standing(60.0, 0.5, 0.4, &pose);
        let leg = hanging(200.0, 0.5, 0.4, &pose);
        if let Some(u) = try_union(&plate, &legs) {
            six(
                &format!("[{p}] plate+arch vs hanging leg"),
                &u,
                &leg,
                &mut tally,
            );
            if let Some(cube) = try_union(&leg, &cone) {
                six(
                    &format!("[{p}] plate+arch vs leg+cone (one or two apexes)"),
                    &u,
                    &cube,
                    &mut tally,
                );
            } else {
                sk += 1;
                eprintln!("ROW [{p}] leg+cone union refused");
            }
        }
        // F3: prism through the top beside a standing pyramid (the strut case).
        if let Some(one) = try_union(&plate, &standing(60.0, 0.5, 0.4, &pose)) {
            for (n, h) in [
                ("wedge", wedge(200.0, 260.0, 0)),
                ("leaned", leaned(200.0, 260.0, 1, 30.0)),
            ] {
                six(
                    &format!("[{p}] {n} prism vs plate+pyr"),
                    &posed_prism(&h, &pose),
                    &one,
                    &mut tally,
                );
            }
            // Arch holes against plate+pyr.
            for (i, h) in arch().iter().enumerate() {
                six(
                    &format!("[{p}] arch[{i}] prism vs plate+pyr"),
                    &posed_prism(h, &pose),
                    &one,
                    &mut tally,
                );
            }
        }
        // F4: two blocks in face contact vs standing pyramid / prism / hanging.
        let blocks = posed_boxes(
            "blocks",
            &[PLATE, [(0.5, 2.5), (0.5, 1.5), (1.0, 1.5)]],
            &pose,
        );
        six(&format!("[{p}] blocks vs cone"), &blocks, &cone, &mut tally);
        six(
            &format!("[{p}] blocks vs wedge prism"),
            &blocks,
            &posed_prism(&wedge(200.0, 260.0, 0), &pose),
            &mut tally,
        );
        // F5: three blocks: pierce + pierce + pair — corner of top block at MEET.
        let corner3 = posed_boxes(
            "corner block",
            &[PLATE, [(1.5, 2.5), (1.0, 1.8), (1.0, 1.5)]],
            &pose,
        );
        six(
            &format!("[{p}] corner-block vs cone"),
            &corner3,
            &cone,
            &mut tally,
        );
        // F6: prism-union (wedges meeting at MEET) folded with the plate: distinct keys pierce one face at one point.
        for (n, hs) in [
            ("two wedges", two_wedges()),
            ("three wedges", three_wedges()),
        ] {
            let hs: Vec<Hole> = hs;
            let mut u = posed_prism(&hs[0], &pose);
            let mut ok = true;
            for h in &hs[1..] {
                match try_union(&u, &posed_prism(h, &pose)) {
                    Some(b) => u = b,
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                six(&format!("[{p}] {n} union vs plate"), &u, &plate, &mut tally);
            } else {
                eprintln!("ROW [{p}] {n} union refused");
            }
        }
        // F7: arch holes union vs plate (issue's arch pose) and vs plate+pyr.
        let hs = arch();
        let mut u = posed_prism(&hs[0], &pose);
        let mut ok = true;
        for h in &hs[1..] {
            match try_union(&u, &posed_prism(h, &pose)) {
                Some(b) => u = b,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            six(
                &format!("[{p}] arch union vs plate"),
                &u,
                &plate,
                &mut tally,
            );
            six(&format!("[{p}] arch union vs cone"), &u, &cone, &mut tally);
        }
        // F8: union of standing pyramids first, then plate (builds per filing); then vs a cone.
        let [a0, rest @ ..] = [60.0, 180.0, 300.0].map(|b| standing(b, 0.5, 0.4, &pose));
        let mut arches = Some(a0);
        for a in &rest {
            arches = arches.and_then(|u| try_union(&u, a));
        }
        if let Some(ar) = arches {
            six(
                &format!("[{p}] pyr-union vs plate"),
                &ar,
                &plate,
                &mut tally,
            );
            six(&format!("[{p}] pyr-union vs cone"), &ar, &cone, &mut tally);
        }
    }
    eprintln!("SKIPS {sk}");
    for (k, n) in &tally {
        eprintln!("TALLY {n:5}  {k}");
    }
    assert!(
        tally
            .keys()
            .all(|k| !k.starts_with("PANIC") && !k.contains("false")),
        "a panic or unsound body: {tally:?}"
    );
}

//! TEMPORARY (INTENT stage 3 B's one-time migration check, removed
//! before merge): with `PNCAD_S3B_DUMP=<dir>` set, every top-level
//! `f64` evaluation appends each instance's solved world pose and the
//! product's geometry digest to `<dir>/<test>.txt`.

use std::io::Write;

use crate::doc::Doc;
use crate::eval::Evaluation;
use crate::node::Node;
use crate::program::ProfileProgram;

fn fnv(h: &mut u64, bytes: &[u8]) {
    for b in bytes {
        *h ^= u64::from(*b);
        *h = h.wrapping_mul(0x1000_0000_01b3);
    }
}

fn body_digest(body: &topo::Body<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (_, p) in body.points() {
        for c in p.to_array() {
            fnv(&mut h, &c.to_bits().to_be_bytes());
        }
    }
    for (_, curve) in body.curves() {
        fnv(&mut h, format!("{curve:?}").as_bytes());
    }
    for (_, surface) in body.surfaces() {
        fnv(&mut h, format!("{surface:?}").as_bytes());
    }
    fnv(
        &mut h,
        format!(
            "V{}E{}F{}",
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        )
        .as_bytes(),
    );
    h
}

fn frame_bits(f: &crate::placement::Frame) -> String {
    let m = f.affine::<f64>();
    let mut s = String::new();
    for c in [m.linear.c0, m.linear.c1, m.linear.c2, m.translation] {
        for x in c.to_array() {
            s.push_str(&format!("{:016x}", x.to_bits()));
        }
    }
    s
}

pub(crate) fn dump<T: geom_core::Decide + 'static>(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<T>,
    opts: &crate::eval::EvalOptions,
    tol: geom_core::Tol,
) {
    let Some(dir) = std::env::var_os("PNCAD_S3B_DUMP") else {
        return;
    };
    let Some(ev) = (ev as &dyn std::any::Any).downcast_ref::<Evaluation<f64>>() else {
        return;
    };
    let test = std::env::var("NEXTEST_TEST_NAME")
        .ok()
        .or_else(|| std::thread::current().name().map(str::to_owned))
        .unwrap_or_else(|| "unnamed".to_owned());
    let bin = std::env::var("NEXTEST_BINARY_ID").unwrap_or_else(|_| "bin".to_owned());
    // On a thread of its own, so the observed test's thread-local
    // witnesses (decision brackets, the gather counter) see nothing.
    let lines = std::thread::scope(|scope| {
        scope
            .spawn(|| lines_of(doc, ev, opts, tol))
            .join()
            .unwrap_or_else(|_| vec!["panicked".to_owned()])
    });
    let name = format!("{bin}__{test}").replace(['/', ':', ' '], "_");
    let path = std::path::Path::new(&dir).join(format!("{name}.txt"));
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "doc {:?}\n{}", doc.id(), lines.join("\n"));
    }
}

fn lines_of(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    opts: &crate::eval::EvalOptions,
    tol: geom_core::Tol,
) -> Vec<String> {
    let mut lines = Vec::new();
    let reach = crate::eval::mate_reach::<f64>(opts, tol);
    let poses = crate::mate::solve_document(doc, &reach, tol);
    let mut ordinal = 0usize;
    for id in doc.ids() {
        if !matches!(doc.node(id), Some(Node::InstantiatePart { .. })) {
            continue;
        }
        let world = match poses.placement(doc, id) {
            Ok(f) => frame_bits(&f),
            Err(crate::mate::PoseRefusal::Unplaced { .. }) => match poses.relative(id) {
                Some(f) => format!("loose={}", frame_bits(&f)),
                None => "loose-none".to_owned(),
            },
            Err(e) => {
                let s = format!("{e:?}");
                format!("refused:{}", s.chars().take(40).collect::<String>())
            }
        };
        lines.push(format!("instance#{ordinal} world={world}"));
        ordinal += 1;
    }
    match crate::product::product_recorded(doc, ev, tol) {
        Ok(p) => {
            let body = p.body.into_body();
            lines.push(format!("product {:016x}", body_digest(&body)));
        }
        Err(e) => {
            let s = format!("{e:?}");
            lines.push(format!(
                "product refused:{}",
                s.split(|c: char| !c.is_alphanumeric()).next().unwrap_or("")
            ));
        }
    }
    lines
}

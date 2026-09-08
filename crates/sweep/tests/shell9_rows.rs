//! **Every stored pcurve row of a body `shell` built, one line per
//! row.** The verb's closing mint re-derives every row of every body
//! it returns; on a body whose transferred rows were content-correct
//! the re-derived row is bit-identical (the same deterministic pass
//! on the same surfaces), and a row that moves names a body that was
//! carrying a stale row tier 3 did not see. The dump corpora
//! (`shell5_r1_dump`, `shell7_dump`, `shell8_dump`) call [`rows`] on
//! every body they dump, and the row below shells `verbs_shell`'s
//! fixtures the same way. Run with `--nocapture`, grep `[rows]`, diff
//! across trees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::Body;

use super::shell7_common::face_of_he;
use super::verbs_shell::{boxy, tube, vessel};

/// The planar faces of the cap at height `y` on a body revolved about
/// the `y` axis.
fn cap_at_y(body: &Body<f64>, y: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.y - y).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.z.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// One line per stored row, in half-edge-slot order: the half-edge,
/// its face, the parameter window and the image — `{:?}` of every
/// number is its shortest round-trip form, so equal text is equal
/// bits.
pub(crate) fn rows(label: &str, body: &Body<f64>) {
    let mut n = 0;
    for (he, cache) in body.pcurves() {
        n += 1;
        println!(
            "[rows] {label}: he {he:?} face {:?} params {:?} pcurve {:?}",
            face_of_he(body, he),
            cache.params(),
            cache.pcurve()
        );
    }
    println!("[rows] {label}: {n} rows");
}

fn shelled(label: &str, body: &Body<f64>, t: f64, open: &[topo::FaceKey]) {
    match topo::shell_open(body, t, open, Tol::witness()) {
        Ok(s) => rows(label, &s.body),
        Err(e) => println!("[rows] {label}: Err {e}"),
    }
}

/// `verbs_shell`'s fixtures, sealed and opened at their top.
#[test]
fn shell9_rows_verbs_shell_corpus() {
    let b = boxy(2.0, 3.0, 4.0);
    shelled("box sealed", &b, 0.25, &[]);
    let v = vessel(1.0, 2.0);
    shelled("vessel sealed", &v, 0.2, &[]);
    shelled("vessel opened top", &v, 0.2, &cap_at_y(&v, 2.0));
    let u = tube(0.6, 1.0, 2.0);
    shelled("tube sealed", &u, 0.1, &[]);
    shelled("tube opened top", &u, 0.1, &cap_at_y(&u, 2.0));
    let hollow = topo::shell(&v, 0.2, Tol::witness()).expect("hollows").body;
    shelled("hollow vessel shelled again", &hollow, 0.05, &[]);
}

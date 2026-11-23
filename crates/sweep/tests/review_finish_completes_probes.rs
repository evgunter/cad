//! Review probes for PR 7497 (the finish completes rows). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::construction::Construct;
use topo::{Body, FaceKey, HalfEdgeKey};

use super::common::cone_nappe::frustum;
use super::common::latitude_seam::two_arc_sphere;
use super::common::shell_operands::vessel;
use super::common::torus_walls::torus_barrel;

fn tol() -> Tol {
    Tol::witness()
}

fn kind(body: &Body<f64>, f: FaceKey) -> String {
    let s = format!("{:?}", body.face_surface(f).unwrap());
    s.split([' ', '{', '(']).next().unwrap().to_string()
}

/// Faces storing rows, with their row-carrying half-edges in key order.
fn row_faces(body: &Body<f64>) -> Vec<(FaceKey, Vec<HalfEdgeKey>)> {
    let mut out: Vec<(FaceKey, Vec<HalfEdgeKey>)> = Vec::new();
    for (he, _) in body.pcurves() {
        let f = body.face_of_half_edge(he).unwrap();
        match out.iter_mut().find(|(g, _)| *g == f) {
            Some((_, v)) => v.push(he),
            None => out.push((f, vec![he])),
        }
    }
    out
}

fn face_rows(body: &Body<f64>, f: FaceKey) -> Vec<String> {
    body.pcurves()
        .filter(|(he, _)| body.face_of_half_edge(*he).unwrap() == f)
        .map(|(he, c)| format!("{he:?} {c:?} joint {:?}", body.joint(he)))
        .collect()
}

fn verdict(label: &str, body: Body<f64>) -> Result<Body<f64>, String> {
    match topo::finish(body, tol()) {
        Ok(f) => {
            println!("[probe] {label}: FINISHED");
            Ok((*f).clone())
        }
        Err(e) => {
            let s = format!("{:?}", e.as_slice());
            println!("[probe] {label}: REFUSED {}", &s[..s.len().min(300)]);
            Err(s)
        }
    }
}

fn curved_face(body: &Body<f64>) -> (FaceKey, Vec<HalfEdgeKey>) {
    row_faces(body)
        .into_iter()
        .find(|(f, hes)| kind(body, *f) != "Plane" && hes.len() >= 2)
        .expect("a curved face with two rows")
}

/// Claim 1a: two rows of one face swapped (each a row of the wrong
/// half-edge), nothing missing.
#[test]
fn p1a_swapped_rows_are_refused() {
    let mut b = vessel(1.0, 2.0);
    let (_, hes) = curved_face(&b);
    let (a, c) = (hes[0], hes[1]);
    let ra = b.pcurve(a).unwrap().clone();
    let rc = b.pcurve(c).unwrap().clone();
    b.attach_pcurve(a, rc);
    b.attach_pcurve(c, ra);
    assert!(verdict("p1a swapped", b).is_err(), "laundered");
}

/// Claim 1b: a wrong-half-edge row kept beside a gap completion fills.
#[test]
fn p1b_wrong_row_beside_a_gap_is_refused() {
    let mut b = vessel(1.0, 2.0);
    let (_, hes) = curved_face(&b);
    let (a, c) = (hes[0], hes[1]);
    let rc = b.pcurve(c).unwrap().clone();
    b.attach_pcurve(a, rc);
    b.detach_pcurve(c);
    assert!(verdict("p1b wrong+gap", b).is_err(), "laundered");
}

/// Claim 1c: every row of the r=1 vessel replaced by the r=1.1
/// vessel's (same build, same keys): rows whose support changed.
#[test]
fn p1c_rows_of_another_support_are_refused() {
    let mut b = sweep::test_support::lofted_cylinder(2.0, tol());
    let other = sweep::test_support::lofted_cylinder(2.2, tol());
    let (f, hes) = curved_face(&b);
    println!("[probe] p1c face kind {}", kind(&b, f));
    assert_eq!(row_faces(&other).iter().find(|(g, _)| *g == f).unwrap().1, hes);
    for &he in &hes {
        println!("[probe] p1c he {he:?} same row text: {}", format!("{:?}", b.pcurve(he)) == format!("{:?}", other.pcurve(he)));
        println!("[probe] p1c   r=1.0 {:?}\n[probe] p1c   r=1.1 {:?}", b.pcurve(he), other.pcurve(he));
        b.attach_pcurve(he, other.pcurve(he).unwrap().clone());
    }
    let mut gap = b.clone();
    assert!(verdict("p1c support moved, complete", b).is_err(), "laundered");
    gap.detach_pcurve(hes[0]);
    assert!(verdict("p1c support moved, one gap", gap).is_err(), "laundered");
}

/// Claim 1d: a face whose images are all off but whose joint elements
/// stay, one of them made stale (a period off). The contract keeps
/// every stored element; the derive-whole arm (`!rows.stores`) writes
/// the mint's elements over them.
#[test]
fn p1d_stale_element_on_an_imageless_face() {
    let base = vessel(1.0, 2.0);
    let (_, hes) = curved_face(&base);
    let mut clean = base.clone();
    for &he in &hes {
        clean.detach_pcurve(he);
    }
    let mut b = clean.clone();
    let he = hes.iter().copied().find(|&h| b.joint(h).is_some()).expect("a joint");
    let el = b.joint(he).unwrap().deck();
    b.attach_joint(he, topo::JointElement(topo::Deck { u: el.u + 1, ..el }));
    let _ = verdict("p1d imageless, elements kept", clean);
    let r = verdict("p1d imageless, stale element", b.clone());
    if let Ok(done) = &r {
        println!(
            "[probe] p1d stale element {:?} -> finished element {:?}",
            b.joint(he),
            done.joint(he)
        );
    }
    // Control: the same stale element with one image left (half-minted arm).
    let mut half = base.clone();
    for &h in &hes[1..] {
        half.detach_pcurve(h);
    }
    half.attach_joint(he, topo::JointElement(topo::Deck { u: el.u + 1, ..el }));
    let _ = verdict("p1d one image kept, stale element", half);
    assert!(r.is_err(), "stale element laundered by the derive-whole arm");
}

/// Claim 2: per face, half the rows off (every other one, joints kept)
/// against all of them off (the mint's derive_face), compared bit for
/// bit, certificates included.
#[test]
fn p2_fill_matches_whole_derivation() {
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("vessel", vessel(1.0, 2.0)),
        ("frustum", frustum(0.5, 1.0)),
        ("torus_barrel", torus_barrel()),
        ("two_arc_sphere", two_arc_sphere()),
        ("lofted_cylinder", sweep::test_support::lofted_cylinder(2.0, tol())),
    ];
    let mut differ = Vec::new();
    for (name, body) in bodies {
        for (f, hes) in row_faces(&body) {
            let k = kind(&body, f);
            let orig = face_rows(&body, f);
            let mut whole = body.clone();
            for &he in &hes {
                whole.detach_pcurve(he);
                whole.detach_joint(he);
            }
            let mut half = body.clone();
            for (i, &he) in hes.iter().enumerate() {
                if i % 2 == 1 {
                    half.detach_pcurve(he);
                    half.detach_joint(he);
                }
            }
            let w = topo::finish(whole, tol()).map(|x| face_rows(&x, f));
            let h = topo::finish(half, tol()).map(|x| face_rows(&x, f));
            match (&w, &h) {
                (Ok(w), Ok(h)) => {
                    let same_w = *w == orig;
                    let same_h = *h == orig;
                    println!(
                        "[p2] {name} {k} {f:?} rows {} whole==orig {same_w} half==orig {same_h} half==whole {}",
                        hes.len(),
                        w == h
                    );
                    if w != h {
                        for (a, b) in w.iter().zip(h.iter()) {
                            if a != b {
                                println!("[p2]   whole {a}\n[p2]   half  {b}");
                            }
                        }
                        differ.push(format!("{name} {k}"));
                    }
                }
                _ => println!(
                    "[p2] {name} {k} {f:?} whole ok {} half ok {} : {:?} / {:?}",
                    w.is_ok(),
                    h.is_ok(),
                    w.as_ref().err().map(|e| format!("{:?}", e.as_slice())),
                    h.as_ref().err().map(|e| format!("{:?}", e.as_slice()))
                ),
            }
        }
    }
    assert!(differ.is_empty(), "{differ:?}");
}

/// Claim 5: wall time of the finish on complete bodies, against the
/// pure tier-3 check (`validate_geometric`) the verdict runs. Prints;
/// asserts nothing. Run with --release-like care: debug numbers are
/// relative only.
#[test]
fn p5_finish_cost() {
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("vessel", vessel(1.0, 2.0)),
        ("frustum", frustum(0.5, 1.0)),
        ("torus_barrel", torus_barrel()),
        ("two_arc_sphere", two_arc_sphere()),
        ("lofted_cylinder", sweep::test_support::lofted_cylinder(2.0, tol())),
    ];
    for (name, body) in bodies {
        let n = 5;
        let t = std::time::Instant::now();
        for _ in 0..n {
            topo::validate_geometric(&body, tol()).unwrap();
        }
        let check = t.elapsed() / n;
        let t = std::time::Instant::now();
        for _ in 0..n {
            topo::finish(body.clone(), tol()).unwrap();
        }
        let fin = t.elapsed() / n;
        let t = std::time::Instant::now();
        for _ in 0..n {
            topo::test_support::completed(body.clone(), tol()).unwrap();
        }
        let comp = t.elapsed() / n;
        println!("[p5] {name}: tier3 check {check:?} finish {fin:?} (tiers1-2 + completion alone {comp:?})");
    }
}

/// Claim 1e: the spline face (fitted/iso rows): two rows swapped, and a
/// wrong row beside a gap.
#[test]
fn p1e_spline_wrong_rows_are_refused() {
    let base = sweep::test_support::lofted_cylinder(2.0, tol());
    let (f, hes) = curved_face(&base);
    assert_eq!(kind(&base, f), "Nurbs");
    let (a, c) = (hes[0], hes[1]);
    let ra = base.pcurve(a).unwrap().clone();
    let rc = base.pcurve(c).unwrap().clone();
    let mut swapped = base.clone();
    swapped.attach_pcurve(a, rc.clone());
    swapped.attach_pcurve(c, ra);
    let mut gap = base.clone();
    gap.attach_pcurve(a, rc);
    gap.detach_pcurve(c);
    let s = verdict("p1e spline swapped", swapped);
    let g = verdict("p1e spline wrong+gap", gap);
    assert!(s.is_err() && g.is_err(), "laundered");
}

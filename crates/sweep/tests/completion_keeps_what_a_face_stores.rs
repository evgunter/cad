//! **Completion keeps what a face stores** (`topo::pcurves`'
//! completion, run by the finish): a half-minted face fills as the
//! whole derivation would, and every stored image and element stands
//! as found — so a row on the wrong half-edge, an element kept without
//! its images, or an image over a support that moved it is refused at
//! the finish, never replaced. A clear-first completion (deriving every
//! face whole) turns these rows red.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec3};
use topo::construction::Construct;
use topo::{Body, FaceKey, HalfEdgeKey, LoopBoundary};

fn tol() -> Tol {
    Tol::witness()
}

fn face_half_edges(body: &Body<f64>, face: FaceKey) -> Vec<Vec<HalfEdgeKey>> {
    body.face(face)
        .unwrap()
        .loops()
        .filter_map(|(_, l, _)| match l.boundary {
            LoopBoundary::Cycle { first } => Some(body.loop_cycle(first).unwrap()),
            _ => None,
        })
        .collect()
}

fn rows(body: &Body<f64>) -> Vec<(HalfEdgeKey, String, String)> {
    let mut out: Vec<_> = body
        .half_edges()
        .map(|(he, _)| {
            (
                he,
                format!("{:?}", body.pcurve(he)),
                format!("{:?}", body.joint(he)),
            )
        })
        .collect();
    out.sort_by_key(|r| r.0);
    out
}

fn strip(body: &mut Body<f64>, hes: impl IntoIterator<Item = HalfEdgeKey>) {
    for he in hes {
        body.detach_pcurve(he);
        body.detach_joint(he);
    }
}

fn fixtures() -> Vec<(&'static str, Body<f64>)> {
    let t = tol();
    let cyl = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 0.0),
        ],
        sweep::Revolution::Full,
        t,
    );
    let cone = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 1.5), 0.0),
        ],
        sweep::Revolution::Full,
        t,
    );
    let torus = sweep::test_support::revolved_about_y(
        vec![(Point2::new(2.0, -0.5), 1.0), (Point2::new(2.0, 0.5), 1.0)],
        sweep::Revolution::Full,
        t,
    );
    let part_torus = sweep::test_support::revolved_about_y(
        vec![(Point2::new(2.0, -0.5), 1.0), (Point2::new(2.0, 0.5), 1.0)],
        sweep::Revolution::Partial(2.0),
        t,
    );
    let sphere = sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), t);
    let dome = sweep::test_support::dome(1.0, t);
    let cube = sweep::test_support::cube::<f64>(2.0, t);
    let spline = crate::common::approx::twisted_loft(0.3);
    let mut out = vec![
        ("plane cube", cube),
        ("cylinder", cyl),
        ("cone", cone),
        ("torus", torus),
        ("partial torus", part_torus),
        ("sphere", sphere),
        ("dome", dome),
        ("spline loft", spline),
    ];
    for (name, body) in &mut out {
        // The reference: every row derived whole, as the clearing mint did.
        let mut bare = body.clone();
        let hes: Vec<_> = bare.half_edges().map(|(h, _)| h).collect();
        strip(&mut bare, hes);
        *body = topo::test_support::completed(bare, tol())
            .unwrap_or_else(|e| panic!("{name}: the whole derivation completes: {e:?}"));
    }
    out
}

/// A half-minted face filled by completion equals the whole
/// derivation (the old clearing mint's rows), images, certificates and
/// elements, wherever both succeed.
#[test]
fn a_half_minted_face_fills_as_the_whole_derivation() {
    let mut report = Vec::new();
    for (name, reference) in fixtures() {
        let want = rows(&reference);
        for (k, phase) in [(2usize, 0usize), (2, 1), (3, 0), (3, 2)] {
            let mut body = reference.clone();
            let mut stripped = 0;
            let faces: Vec<_> = body.faces().map(|(f, _)| f).collect();
            for face in faces {
                for cycle in face_half_edges(&body, face) {
                    if cycle.len() < 2 {
                        continue;
                    }
                    let hes: Vec<_> = cycle
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i % k == phase)
                        .map(|(_, &h)| h)
                        .collect();
                    stripped += hes.len();
                    strip(&mut body, hes);
                }
            }
            match topo::test_support::completed(body, tol()) {
                Ok(done) => {
                    let got = rows(&done);
                    let diffs: Vec<_> = want
                        .iter()
                        .zip(&got)
                        .filter(|(a, b)| a != b)
                        .map(|(a, b)| {
                            format!(
                                "{:?}\n  whole: {} {}\n  fill:  {} {}",
                                a.0, a.1, a.2, b.1, b.2
                            )
                        })
                        .collect();
                    report.push(format!(
                        "{name} k={k} phase={phase}: stripped {stripped}, {} rows differ",
                        diffs.len()
                    ));
                    for d in diffs.iter().take(3) {
                        report.push(d.clone());
                    }
                    if let Err(e) = topo::test_support::validate_completed(&done, tol()) {
                        report.push(format!("  ... and the filled body does not finish: {e:?}"));
                    }
                }
                Err(e) => report.push(format!(
                    "{name} k={k} phase={phase}: stripped {stripped}, fill REFUSED: {e:?}"
                )),
            }
        }
    }
    let text = report.join("\n");
    println!("{text}");
    assert!(
        !text.contains("differ")
            || text
                .lines()
                .filter(|l| l.contains("differ"))
                .all(|l| l.contains(" 0 rows differ")),
        "fill differs from whole derivation:\n{text}"
    );
    assert!(!text.contains("REFUSED"), "{text}");
    assert!(!text.contains("does not finish"), "{text}");
}

/// A stored row moved onto the wrong half-edge is refused
/// by the finish, and `complete_in_place` (via `completed`) keeps it
/// bit for bit.
#[test]
fn a_row_on_the_wrong_half_edge_is_refused_not_laundered() {
    for (name, reference) in fixtures() {
        let faces: Vec<_> = reference.faces().map(|(f, _)| f).collect();
        let mut tried = 0;
        for face in faces {
            for cycle in face_half_edges(&reference, face) {
                if cycle.len() < 2 {
                    continue;
                }
                let (a, b) = (cycle[0], cycle[1]);
                let (Some(ra), Some(rb)) =
                    (reference.pcurve(a).cloned(), reference.pcurve(b).cloned())
                else {
                    continue;
                };
                // Swap the two images; strip a THIRD half-edge where there is one so
                // the face is half-minted and completion must walk around the stale row.
                for half in [false, true] {
                    let mut body = reference.clone();
                    body.attach_pcurve(a, rb.clone());
                    body.attach_pcurve(b, ra.clone());
                    if half {
                        if cycle.len() < 3 {
                            continue;
                        }
                        strip(&mut body, [cycle[2]]);
                    }
                    tried += 1;
                    let before = format!("{:?}", body.pcurve(a));
                    if let Ok(done) = topo::test_support::completed(body.clone(), tol()) {
                        assert_eq!(
                            before,
                            format!("{:?}", done.pcurve(a)),
                            "{name}: completion replaced a stored row"
                        );
                    }
                    let r = topo::test_support::validate_completed(&body, tol());
                    assert!(
                        r.is_err(),
                        "{name} face {face:?} half={half}: a swapped row FINISHED (laundered)"
                    );
                }
            }
        }
        println!("{name}: {tried} swaps refused");
    }
}

/// A stale joint element kept on a face that stores no
/// image (`stores == false`) — does completion's whole derivation
/// overwrite it?
#[test]
fn an_element_kept_without_its_images_is_not_replaced() {
    let mut laundered = Vec::new();
    for (name, reference) in fixtures() {
        let faces: Vec<_> = reference.faces().map(|(f, _)| f).collect();
        for face in faces {
            let cycles = face_half_edges(&reference, face);
            let all: Vec<_> = cycles.iter().flatten().copied().collect();
            let Some((he, element)) = all.iter().find_map(|&h| reference.joint(h).map(|e| (h, e)))
            else {
                continue;
            };
            let mut body = reference.clone();
            for &h in &all {
                body.detach_pcurve(h);
            }
            let wrong = topo::JointElement(topo::Deck {
                u: element.deck().u + 1,
                ..element.deck()
            });
            body.attach_joint(he, wrong);
            let r = topo::test_support::validate_completed(&body, tol());
            let after = topo::test_support::completed(body, tol())
                .ok()
                .map(|d| d.joint(he));
            if r.is_ok() {
                laundered.push(format!(
                    "{name} face {face:?}: stored {wrong:?} became {after:?} and the body FINISHED"
                ));
            }
        }
    }
    println!("{}", laundered.join("\n"));
    assert!(laundered.is_empty(), "{}", laundered.join("\n"));
}

/// Rows whose support changed — the rows of a same-keyed
/// body with a different carrier (radius / twist) transplanted — are
/// read by their support at the gate: a row whose recorded support is
/// not the edge's current one is certified anew against the current
/// carrier, never taken on its certificate's word. So a row whose image
/// is not the current edge's (the cone's, whose height moves its
/// images) is refused, whole and with a gap completion fills around;
/// a row whose image certifies against the current carrier — a
/// cylinder's or torus's, whose chart image does not read the radius,
/// and the twisted loft's — is current, and the body finishes.
#[test]
fn rows_over_a_moved_support_finish_where_their_images_are_current_and_refuse_where_not() {
    let t = tol();
    let rev = |v: Vec<(Point2<f64>, f64)>| {
        let b = sweep::test_support::revolved_about_y(v, sweep::Revolution::Full, t);
        topo::test_support::completed(b, t).unwrap()
    };
    let cyl = |r: f64| {
        rev(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 0.0),
        ])
    };
    let torus = |big: f64| {
        rev(vec![
            (Point2::new(big, -0.5), 1.0),
            (Point2::new(big, 0.5), 1.0),
        ])
    };
    let cone = |h: f64| {
        rev(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, h), 0.0),
        ])
    };
    // (name, body, other, whether the transplanted rows are stale)
    let pairs = vec![
        ("cylinder r", cyl(1.0), cyl(1.2), false),
        ("torus R", torus(2.0), torus(2.1), false),
        ("cone h", cone(1.5), cone(1.7), true),
        (
            "spline twist",
            crate::common::approx::twisted_loft(0.3),
            crate::common::approx::twisted_loft(0.35),
            false,
        ),
    ];
    let mut laundered = Vec::new();
    let mut wrongly_refused = Vec::new();
    for (name, body, other, stale) in pairs {
        for gap in [false, true] {
            let mut moved = body.clone();
            let mut planted = 0;
            for (he, row) in other.pcurves() {
                if moved.get_half_edge(he).is_some()
                    && moved
                        .pcurve(he)
                        .is_some_and(|r| format!("{r:?}") != format!("{row:?}"))
                {
                    if planted == 0 && !gap {
                        println!("ORIG {:?}\nPLNT {row:?}", moved.pcurve(he).unwrap());
                    }
                    moved.attach_pcurve(he, row.clone());
                    planted += 1;
                }
            }
            if gap {
                // strip one non-planted-irrelevant half-edge per curved face
                let faces: Vec<_> = moved.faces().map(|(f, _)| f).collect();
                for face in faces {
                    if let Some(c) = face_half_edges(&moved, face)
                        .into_iter()
                        .find(|c| c.len() >= 3)
                    {
                        strip(&mut moved, [c[0]]);
                    }
                }
            }
            let r = topo::test_support::validate_completed(&moved, t);
            println!(
                "{name} gap={gap}: planted {planted}, finish -> {}",
                match &r {
                    Ok(()) => "OK (LAUNDERED?)".to_string(),
                    Err(findings) => format!("{} findings", findings.len()),
                }
            );
            assert!(planted > 0, "{name}: nothing transplanted");
            if stale && r.is_ok() {
                laundered.push(format!("{name} gap={gap}"));
            }
            if !stale && r.is_err() {
                wrongly_refused.push(format!("{name} gap={gap}: {r:?}"));
            }
        }
    }
    assert!(laundered.is_empty(), "{laundered:?}");
    assert!(wrongly_refused.is_empty(), "{wrongly_refused:?}");
}

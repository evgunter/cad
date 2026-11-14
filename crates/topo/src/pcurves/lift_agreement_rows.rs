//! **The decided lift is the read lift.** [`Lift::decided`] runs the
//! mint's walk and stores nothing; [`Lift::read`] reads what the mint
//! stored. On one body the two are one chain, a joint through the
//! sphere's twin sheet and a joint on a pole included.
//!
//! The fixture is the unit sphere about `z`, cut into a quarter and its
//! complement by the `x z` great circle's upper half, from
//! `A = (1, 0, 0)` over the north pole to `B = (−1, 0, 0)`, and the
//! equator's half from `B` back to `A`. With no vertex at the pole the
//! half's image runs over it and ends on the chart's twin sheet, so the
//! joint at its far end closes through the twin; with a vertex there the
//! same region's loop has a pole joint instead.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point3, Tol, Vec3};

use crate::body::Body;
use crate::construction::{FaceSurface, MefSite, MevSite};
use crate::pcurves::{Lift, loop_rows, mint_pcurves};

/// The `x z` great circle by latitude: `eval(t) = (cos t, 0, sin t)`.
fn great_xz() -> Curve3<f64> {
    Curve3::circle(
        Point3::origin(),
        Vec3::new(0.0, -1.0, 0.0),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    )
}

/// The equator from `B`: `eval(t) = (−cos t, −sin t, 0)`.
fn equator_from_b() -> Curve3<f64> {
    Curve3::circle(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
        Vec3::new(-1.0, 0.0, 0.0),
    )
}

fn quarter(vertex_at_pole: bool) -> Body<f64> {
    let tol = Tol::witness();
    let g = great_xz();
    let arc = |from: f64, to: f64| EdgeCurveSpec::arc_of_circle(g.clone(), from, to).unwrap();
    let mut body = Body::<f64>::new();
    let start = body.mvfs(g.eval(0.0), true).unwrap();
    body.set_face_surface(
        start.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .unwrap();
    let lone = MevSite::Lone {
        r#loop: start.r#loop,
    };
    let (first_plus, last_minus) = if vertex_at_pole {
        let up = body
            .mev(lone, g.eval(FRAC_PI_2), arc(0.0, FRAC_PI_2), tol)
            .unwrap();
        let down = body
            .mev(
                MevSite::Fan {
                    he1: up.he_minus,
                    he2: up.he_minus,
                },
                g.eval(PI),
                arc(FRAC_PI_2, PI),
                tol,
            )
            .unwrap();
        (up.he_plus, down.he_minus)
    } else {
        let over = body.mev(lone, g.eval(PI), arc(0.0, PI), tol).unwrap();
        (over.he_plus, over.he_minus)
    };
    body.mef(
        MefSite::Chords {
            he1: last_minus,
            he2: first_plus,
        },
        EdgeCurveSpec::arc_of_circle(equator_from_b(), 0.0, PI).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

#[test]
fn the_decided_lift_is_the_read_lift_through_a_twin_and_a_pole() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut twins = 0;
    let mut turns = 0;
    for vertex_at_pole in [false, true] {
        let mut body = quarter(vertex_at_pole);
        mint_pcurves(&mut body, Tol::witness()).unwrap();
        for face in body.face_views() {
            let outer = face.record().outer;
            let cycle = loop_rows(&body, outer);
            let decided = Lift::decided(&body, face.key(), face.surface(), &cycle, true, band)
                .unwrap_or_else(|refusal| {
                    panic!("vertex at the pole {vertex_at_pole}: the decided lift: {refusal:?}")
                });
            let read = Lift::read(&body, outer).unwrap();
            let chain = |lift: &Lift<f64>| -> Vec<_> {
                lift.rows()
                    .iter()
                    .map(|row| (row.half_edge, row.lift))
                    .collect()
            };
            assert_eq!(
                chain(&decided),
                chain(&read),
                "vertex at the pole {vertex_at_pole}: the walk decides the chain the body stores"
            );
            assert_eq!(decided.winding(), read.winding());
            assert_eq!(decided.window(), read.window());
            twins += read.rows().iter().filter(|row| row.lift.twin).count();
            turns += cycle
                .iter()
                .filter(|&&he| {
                    let record = body.get_half_edge(he).unwrap();
                    let at_pole = body
                        .vertex_points()
                        .any(|(v, p)| v == record.start && (p.z - 1.0).abs() < 1e-12);
                    at_pole && body.joint(he).is_some_and(|e| !e.deck().twin)
                })
                .count();
        }
    }
    assert!(
        twins > 0,
        "the over-pole half closes through the twin sheet"
    );
    assert_eq!(turns, 2, "each face's loop turns once at the pole vertex");
}

/// **A split at a pole decides its children's turn.** The half great
/// circle over the pole is one edge whose image ends on the chart's
/// twin sheet. Split at the pole, each child is placed again over its
/// own span, on its own principal sheet, so the twin leaves the loop
/// and the children's joint is a turn at the pole: the face's, read
/// off the two carriers' tangents there. The split decides it, and the
/// body it leaves stores the lift the walk decides, with nothing for
/// tier 3 to find. (A child that stays on the twin sheet is the general
/// lane's: `sweep`'s `tilted_sphere_pair` rows.)
#[test]
fn a_split_at_the_pole_decides_its_childrens_turn() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = quarter(false);
    mint_pcurves(&mut body, tol).unwrap();
    let over_pole = body
        .edges()
        .map(|(key, _)| key)
        .find(|&key| {
            let (carrier, ..) =
                crate::pcurves::half_edge_carrier(&body, body.get_edge(key).unwrap().he_plus)
                    .unwrap();
            (carrier.eval(FRAC_PI_2).z - 1.0).abs() < 1e-12
        })
        .expect("the half great circle over the pole");
    body.split_edge(over_pole, FRAC_PI_2, tol)
        .unwrap_or_else(|e| panic!("the split at the pole: {e:?}"));
    assert_eq!(crate::pcurves::validate_pcurves(&body, band), vec![]);
    let mut turns = 0;
    for face in body.face_views() {
        let outer = face.record().outer;
        let cycle = loop_rows(&body, outer);
        assert_eq!(cycle.len(), 3, "two children and the equator's half");
        let decided = Lift::decided(&body, face.key(), face.surface(), &cycle, true, band)
            .unwrap_or_else(|refusal| panic!("the decided lift: {refusal:?}"));
        let read = Lift::read(&body, outer).unwrap();
        let chain = |lift: &Lift<f64>| -> Vec<_> {
            lift.rows()
                .iter()
                .map(|row| (row.half_edge, row.lift))
                .collect()
        };
        assert_eq!(chain(&decided), chain(&read));
        assert_eq!(decided.winding(), read.winding());
        assert!(
            cycle
                .iter()
                .all(|&he| body.joint(he).is_some_and(|e| !e.deck().twin)),
            "each child is on its own principal sheet"
        );
        turns += cycle
            .iter()
            .filter(|&&he| {
                let start = body.get_half_edge(he).unwrap().start;
                body.vertex_points()
                    .any(|(v, p)| v == start && (p.z - 1.0).abs() < 1e-12)
            })
            .count();
    }
    assert_eq!(turns, 2, "each face's loop turns once at the split vertex");
}

/// **No file outside the mint pins a chart branch.** A loop's position
/// in its face's chart is its lift, and the one decision of a joint's
/// branch is `decide_joint` (DESIGN `[one-way.places]`). A second pin is
/// spelled with the movers and readers the decision is built from, so
/// every use of one outside `pcurves.rs` and `pcurves/` is listed here
/// with what it is,
/// and a new one fails until it is read and listed, or goes through
/// `Lift`.
///
/// The pinned exception is `mesh::walk`'s `unwrap_tie`, the curved
/// lane's own placement of a pole junction
/// (`work/tesswalk/mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers.md`).
///
/// **No function but the decision makes an element.** A `JointElement`
/// is constructed from a deck in one file of the mint's shipped code,
/// `pcurves/decision.rs`, and there in `decide_joint`; every other
/// element the mint holds is one a decision returned, the identity, or a
/// sum of stored ones (`joint.rs`'s own algebra). Outside it the
/// constructions are listed as the movers are. The mint's rows, the
/// modules `pcurves.rs` declares under `#[cfg(test)]`, state elements to
/// compare against and are not counted.
///
/// **What this guard cannot see**: a walk that places a loop's edges on
/// a branch with arithmetic of its own, spelling none of the movers and
/// making no element. It would NOT have caught the walk this unit
/// retired, `mesh::trimmed`'s `choose_pole_azimuths`, which added and
/// subtracted `TAU` on chart coordinates. Two such walks are known to
/// remain, both below `topo` and reading no body:
/// `geom_brep::props::quad::rotational`'s `loop_lifts`, and
/// `geom_brep::chart_singularity::walk_loop`
/// (`work/quad/props-and-tier-3-each-derive-a-loops-chart-position-again.md`).
/// What stops the next one is review against DESIGN `[one-way.places]`,
/// which names `Lift` as a loop's one chart position.
#[test]
fn no_file_outside_the_mint_pins_a_chart_branch() {
    const SPELLINGS: [&str; 6] = [
        ".shift_branch(",
        "shift_v_branch(",
        "sphere_twin(",
        "lever_periods(",
        "branches_astride_as(",
        "unwrap_tie(",
    ];
    // (file, uses, what they are).
    const LISTED: [(&str, usize, &str); 5] = [
        (
            "geom-brep/src/pcurve_cache.rs",
            1,
            "a certification row moving an image a period, to show the certificate follows",
        ),
        (
            "geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs",
            3,
            "the envelope lemma's fuzz, moving images it then certifies",
        ),
        (
            "mesh/src/walk.rs",
            2,
            "PINNED EXCEPTION: unwrap_tie, the curved lane's pole junction (tesswalk)",
        ),
        (
            "topo/src/chart_region.rs",
            1,
            "the fold between two faces' azimuth windows: no joint of a loop",
        ),
        (
            "topo/src/joint.rs",
            3,
            "Deck::apply, the lift's own mover of an image by a decided element",
        ),
    ];
    // (file, constructions of an element from a deck, what they are).
    const MAKERS: [(&str, usize, &str); 3] = [
        (
            "topo/src/joint.rs",
            5,
            "the type, and its own algebra's rows",
        ),
        (
            "topo/src/pcurves/decision.rs",
            1,
            "decide_joint, the one decision",
        ),
        (
            "topo/src/test_support_fixtures.rs",
            1,
            "a fixture that states a wrong element for tier 3 to refuse",
        ),
    ];
    const MAKER: &str = "JointElement(";
    const OWN: &str = "topo/src/pcurves/lift_agreement_rows.rs";
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    // The mint's rows: the modules `pcurves.rs` declares under
    // `#[cfg(test)]`.
    let root = std::fs::read_to_string(crates.join("topo/src/pcurves.rs")).unwrap();
    let mut rows: Vec<String> = Vec::new();
    let mut gated = false;
    for line in root.lines() {
        let module = line
            .strip_prefix("pub(crate) mod ")
            .or_else(|| line.strip_prefix("mod "))
            .and_then(|rest| rest.strip_suffix(';'));
        match module {
            Some(name) if gated => rows.push(format!("topo/src/pcurves/{name}.rs")),
            _ => {}
        }
        gated = line == "#[cfg(test)]" || (gated && line.starts_with("#["));
    }
    assert!(
        rows.iter().any(|row| row == OWN),
        "the rows `pcurves.rs` declares are read without this file among them"
    );
    let mut found: Vec<(String, usize)> = Vec::new();
    let mut makers: Vec<(String, usize)> = Vec::new();
    for member in std::fs::read_dir(&crates).unwrap() {
        let src = member.unwrap().path().join("src");
        if !src.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        walk(&src, &mut files);
        for file in files {
            let name = file
                .strip_prefix(&crates)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if rows.contains(&name) {
                continue;
            }
            let text = std::fs::read_to_string(&file).unwrap();
            let code = || {
                text.lines()
                    .filter(|line| !line.trim_start().starts_with("//"))
            };
            let made = code()
                .map(|line| line.matches(MAKER).count())
                .sum::<usize>();
            if made > 0 {
                makers.push((name.clone(), made));
            }
            if name == "topo/src/pcurves/decision.rs" {
                let lines: Vec<&str> = code().collect();
                for (at, _) in lines
                    .iter()
                    .enumerate()
                    .filter(|(_, line)| line.contains(MAKER))
                {
                    let within = lines[..at]
                        .iter()
                        .rev()
                        .find(|line| line.starts_with("fn ") || line.starts_with("pub"))
                        .copied()
                        .unwrap_or_default();
                    assert!(
                        within.starts_with("pub(super) fn decide_joint<"),
                        "an element is made from a deck in `{within}`, not in `decide_joint`"
                    );
                }
            }
            if name == "topo/src/pcurves.rs" || name.starts_with("topo/src/pcurves/") {
                continue;
            }
            let uses: usize = code()
                .map(|line| {
                    SPELLINGS
                        .iter()
                        .map(|s| line.matches(s).count())
                        .sum::<usize>()
                })
                .sum();
            if uses > 0 {
                found.push((name, uses));
            }
        }
    }
    found.sort();
    let listed: Vec<(String, usize)> = LISTED
        .iter()
        .map(|&(file, uses, _)| (file.to_owned(), uses))
        .collect();
    assert_eq!(
        found, listed,
        "the uses of a chart-branch mover or reader outside `pcurves.rs` and `pcurves/` are not \
         the listed ones. A new one is a second pin of a joint's branch: read the loop's `Lift` instead, \
         or list the use with what it is. A listed one gone: strike its row."
    );
    makers.sort();
    let listed: Vec<(String, usize)> = MAKERS
        .iter()
        .map(|&(file, made, _)| (file.to_owned(), made))
        .collect();
    assert_eq!(
        makers, listed,
        "the constructions of a `JointElement` from a deck are not the listed ones. An \
         element is what `decide_joint` returns, the identity, or a sum of stored elements: \
         a new construction is a second decision of a joint's branch."
    );
}

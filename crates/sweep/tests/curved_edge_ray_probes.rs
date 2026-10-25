//! **Probes of the curved edge-meeting ray**
//! (`work/flush/a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim.md`):
//! what the side reader's aimed rays read on bodies with curved
//! boundaries, aimed at each face's own aim and at and beside every edge.
//! Evidence for review, not pins: run with
//! `cargo nextest run -p sweep --run-ignored only curved_edge_ray_probes --no-capture`
//! (and `CAD_TOLERANCE_EPS=1e-6` or `1e-12` for the other rows).

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{
    ball_poled_z, bored_block_of_arcs, boss_of_arcs, brick, disc_of_arcs, hemisphere_on_flat_base,
    prism, revolved_about_y,
};
use topo::Body;

/// The fixtures: each a body with a curved boundary, built through the
/// public doors.
fn fixtures(tol: Tol) -> Vec<(&'static str, Body<f64>)> {
    // A quarter turn's bulge.
    let quarter = (PI / 8.0).tan();
    let torus = || vec![(Point2::new(2.5, 0.0), 1.0), (Point2::new(1.5, 0.0), 1.0)];
    let mut out = vec![
        ("cylinder 2 arcs", disc_of_arcs(2, 0.5, 1.0, tol)),
        ("cylinder 3 arcs", disc_of_arcs(3, 0.5, 1.0, tol)),
        ("boss on a box", boss_of_arcs(2, 1.0, 0.3, 0.5, 1.0, tol)),
        ("bored block", bored_block_of_arcs(2, 1.0, 1.0, 0.3, tol)),
        (
            "filleted edge",
            prism(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.8), quarter),
                    (Point2::new(0.8, 1.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                1.0,
                tol,
            ),
        ),
        (
            "cone frustum",
            revolved_about_y(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(0.6, 1.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                Revolution::Full,
                tol,
            ),
        ),
        ("ball", ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol)),
        ("hemisphere", hemisphere_on_flat_base(1.0, tol)),
        ("torus", revolved_about_y(torus(), Revolution::Full, tol)),
        (
            "half torus",
            revolved_about_y(torus(), Revolution::Partial(PI), tol),
        ),
    ];
    // Thin: a disc `m·ε` thick (a rod as narrow is no valid profile:
    // its arcs' sweep is in band).
    for m in [12.0, 21.0, 30.0] {
        let w = m * tol.eps();
        out.push((
            leak(format!("disc {m}ε thick")),
            disc_of_arcs(2, 0.5, w, tol),
        ));
    }
    let frustum = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(0.3, 0.5), 0.0),
            (Point2::new(0.0, 0.5), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let block = brick::<f64>((-1.0, 1.0), (-1.0, 0.0), (-1.0, 1.0), tol);
    let at_rest = |b: &Body<f64>| topo::AtRestBody::validate(b.clone(), tol).expect("at rest");
    match topo::union(&at_rest(&block), &at_rest(&frustum), tol) {
        Ok(r) => match r.body() {
            Some(b) => out.push(("cone frustum on a box", b.body.clone().into_body())),
            None => println!("CER frustum on a box: empty"),
        },
        Err(e) => println!("CER frustum on a box: refuses {e:?}"),
    }
    out
}

/// A fixture's name, for the run's life.
fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// What one reading is, as a bucket.
fn bucket(r: &str) -> &'static str {
    for (needle, name) in [
        ("Side(Out)", "Out"),
        ("Side(In)", "In"),
        ("Pair(", "pair"),
        ("SetAside(Crossing", "set aside: crossing in band"),
        ("SetAside(Graze", "set aside: graze"),
        ("SetAside(Corner", "set aside: corner"),
        ("SetAside(Wedge", "set aside: wedge"),
        ("SetAside(Limit", "set aside: limit"),
        ("no aim", "no aim"),
    ] {
        if r.contains(needle) {
            return name;
        }
    }
    "other"
}

/// Tallies of `lines`' buckets, as text.
fn tally<'a>(lines: impl IntoIterator<Item = &'a str>) -> String {
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for l in lines {
        *counts.entry(bucket(l)).or_default() += 1;
    }
    counts
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Every fixture, both ways out: each face's own aim, the reader's
/// first-decider read, and rays at and beside every edge.
#[test]
#[ignore = "probe: run by hand"]
fn probe_curved_bodies_aimed_rays() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let offsets = [0.0, 0.3 * k_eps, k_eps, 3.0 * k_eps, 30.0 * k_eps];
    let only = std::env::var("CER_ONLY").ok();
    for (name, body) in fixtures(tol) {
        if only.as_deref().is_some_and(|o| o != name) {
            continue;
        }
        for inverted in [false, true] {
            let body = if inverted {
                body.revert().expect("reverts")
            } else {
                body.clone()
            };
            let what = format!("eps={:e} {name} inverted={inverted}", tol.eps());
            if let Err(e) = topo::AtRestBody::validate(body.clone(), tol) {
                println!(
                    "CER {what}: not at rest: {}",
                    format!("{e:?}").chars().take(300).collect::<String>()
                );
            }
            let reader = topo::test_support::aimed_rays(&body, band);
            let read = reader
                .iter()
                .map(|r| {
                    if r.starts_with("Ok(Side(") {
                        r.chars().take(16).collect::<String>()
                    } else {
                        format!("volume ({})", r.chars().take(300).collect::<String>())
                    }
                })
                .collect::<Vec<_>>();
            println!("CER {what}: reader {read:?}");
            let each = topo::test_support::each_aim(&body, band);
            println!(
                "CER {what}: aims [{}]",
                tally(each.iter().map(String::as_str))
            );
            for l in &each {
                if !l.contains("Side(") && !l.contains("no aim") {
                    println!(
                        "CER {what}:   aim {}",
                        l.chars().take(400).collect::<String>()
                    );
                }
            }
            let rays = topo::test_support::edge_rays(&body, &offsets, band);
            let planar_line = |l: &&String| l.starts_with("Line") && l.contains("[Plane | Plane]");
            println!(
                "CER {what}: edge rays, planar line edges [{}]",
                tally(rays.iter().filter(planar_line).map(String::as_str))
            );
            println!(
                "CER {what}: edge rays, other edges [{}]",
                tally(rays.iter().filter(|l| !planar_line(l)).map(String::as_str))
            );
            for l in rays.iter().filter(|l| !planar_line(l)) {
                println!(
                    "CER {what}:   ray {}",
                    l.chars().take(300).collect::<String>()
                );
            }
        }
    }
}

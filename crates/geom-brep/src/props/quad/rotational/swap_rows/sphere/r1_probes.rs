//! R1's probes of PR 6509 (review only, not for merge).

use super::*;

/// Runs `loops` under both names; returns lines of what is off.
fn probe(
    label: &str,
    ball: Ball,
    outward: bool,
    loops: &[Vec<SphereEdge>],
    area: f64,
    about: Point3<f64>,
) -> Vec<String> {
    let (flux, area) = sphere_truth(ball, outward, loops, area, about);
    let loops = &loops;
    let mut off = Vec::new();
    for (which, name) in [("own", Name::Own), ("twin", Name::Twin)] {
        let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            sphere_run(ball, outward, loops, about, name)
        }));
        match run {
            Err(_) => println!("{label} ({which}): FIXTURE PANICKED (mint refused)"),
            Ok(Err(e)) => println!("{label} ({which}): REFUSED {e:?}"),
            Ok(Ok(got)) => {
                let ok = got.flux.lo() <= flux
                    && flux <= got.flux.hi()
                    && got.area.lo() <= area
                    && area <= got.area.hi();
                let line = format!(
                    "{label} ({which}): {} area [{:.12e}, {:.12e}] want {:.12e}; flux [{:.12e}, {:.12e}] want {:.12e}",
                    if ok { "HOLDS" } else { "WRONG" },
                    got.area.lo(),
                    got.area.hi(),
                    area,
                    got.flux.lo(),
                    got.flux.hi(),
                    flux
                );
                println!("{line}");
                if !ok {
                    off.push(line);
                }
            }
        }
    }
    off
}

fn ball() -> Ball {
    Ball::tilted(Point3::new(0.4, -0.2, 0.3), 0.7)
}

fn about() -> Point3<f64> {
    Point3::new(0.1, 0.5, -0.2)
}

/// P1: a cap sector and a lune whose angle at the pole exceeds π, so the
/// closure takes a whole turn off the pole junction (m = ±1).
#[test]
fn r1_pole_angles_past_a_half_turn() {
    let b = ball();
    let level = 0.4_f64;
    let mut off = Vec::new();
    for a in [2.0, PI, 3.5, 4.5, 6.0, 6.2] {
        off.extend(probe(
            &format!("sector a={a}"),
            b,
            true,
            &[vec![
                rim(b, level, (0.0, a), true, true),
                meridian(b, a, (level, FRAC_PI_2), true),
                meridian(b, 0.0, (level, FRAC_PI_2), false),
            ]],
            a * b.r * b.r * (1.0 - level.sin()),
            about(),
        ));
        // the complement sector, inward sense on the same loop reversed
        off.extend(probe(
            &format!("sector a={a}, inward"),
            b,
            false,
            &[vec![
                meridian(b, 0.0, (level, FRAC_PI_2), true),
                meridian(b, a, (level, FRAC_PI_2), false),
                rim(b, level, (0.0, a), false, true),
            ]],
            a * b.r * b.r * (1.0 - level.sin()),
            about(),
        ));
        off.extend(probe(
            &format!("lune a={a}"),
            b,
            true,
            &[vec![
                meridian(b, a, (-FRAC_PI_2, FRAC_PI_2), true),
                meridian(b, 0.0, (-FRAC_PI_2, FRAC_PI_2), false),
            ]],
            2.0 * a * b.r * b.r,
            about(),
        ));
    }
    assert!(off.is_empty(), "{off:#?}");
}

/// P2: hemispheres exactly, and faces near the full sphere.
#[test]
fn r1_hemispheres_and_near_full() {
    let b = ball();
    let mut off = Vec::new();
    for swap in [false, true] {
        off.extend(probe(
            &format!("north hemisphere (swap {swap})"),
            b,
            true,
            &north_cap(b, 0.0, true, swap),
            0.5 * b.sphere_area(),
            about(),
        ));
        off.extend(probe(
            &format!("south hemisphere inward (swap {swap})"),
            b,
            false,
            &north_cap(b, 0.0, true, swap),
            0.5 * b.sphere_area(),
            about(),
        ));
    }
    off.extend(probe(
        "tilted hemisphere",
        b,
        true,
        &[vec![tilted(b, (1.0, 0.2), FRAC_PI_2, true)]],
        0.5 * b.sphere_area(),
        about(),
    ));
    for theta in [1e-2, 1e-3, 1e-4] {
        off.extend(probe(
            &format!("sphere less a disc theta={theta}"),
            b,
            true,
            &[vec![tilted(b, (1.0, 0.2), theta, false)]],
            b.sphere_area() - b.cap(theta),
            about(),
        ));
        off.extend(probe(
            &format!("sphere less a polar cap theta={theta}"),
            b,
            true,
            &north_cap(b, FRAC_PI_2 - theta, false, true),
            b.sphere_area() - b.cap(theta),
            about(),
        ));
    }
    // A disc of angular radius near pi (its complement tiny).
    off.extend(probe(
        "a disc of radius pi - 1e-3",
        b,
        true,
        &[vec![tilted(b, (1.0, 0.2), PI - 1e-3, true)]],
        b.cap(PI - 1e-3),
        about(),
    ));
    assert!(off.is_empty(), "{off:#?}");
}

/// P3: a chord passing ever closer to a pole, and through it (the pole
/// then not a vertex): refuse or hold, never a wrong enclosure.
#[test]
fn r1_chords_at_the_pole() {
    let b = ball();
    let theta = 0.4;
    let mut off = Vec::new();
    for miss in [1e-3, 1e-4, 1e-6, 1e-9, 0.0, -1e-6, -1e-3] {
        let at = (0.7, FRAC_PI_2 - theta - miss);
        let holds_pole = miss < 0.0;
        let disc = b.cap(theta);
        off.extend(probe(
            &format!("disc missing the pole by {miss}"),
            b,
            true,
            &[vec![tilted(b, at, theta, true)]],
            disc,
            about(),
        ));
        off.extend(probe(
            &format!("sphere less that disc ({miss}, holds N: {})", !holds_pole),
            b,
            true,
            &[vec![tilted(b, at, theta, false)]],
            b.sphere_area() - disc,
            about(),
        ));
    }
    assert!(off.is_empty(), "{off:#?}");
}

/// P4: a flipped sense bit is certified as the complement: what the
/// lane reads when `outward` disagrees with the loop's face.
#[test]
fn r1_a_flipped_bit_reads_the_complement() {
    let b = ball();
    let level = 0.4_f64;
    let cap = b.cap(FRAC_PI_2 - level);
    let got = sphere_run(b, false, &north_cap(b, level, true, true), about(), Name::Own);
    println!("cap loop, bit flipped: {got:?}; cap {cap}, rest {}", b.sphere_area() - cap);
}

/// P5: a loop with an open junction (a 0.3 rad azimuth gap at latitude
/// 0.4, ~0.19 m) refuses under both names. Under the twin `cos v < 0`,
/// so the azimuth lever's `abs` is what keeps the gap positive.
#[test]
fn r1_an_open_junction_refuses_under_both_names() {
    let b = ball();
    let level = 0.4_f64;
    let loops = [vec![
        rim(b, level, (0.0, 2.0), true, false),
        meridian(b, 2.3, (level, FRAC_PI_2), true),
        meridian(b, 0.0, (level, FRAC_PI_2), false),
    ]];
    let mut off = Vec::new();
    for (which, name) in [("own", Name::Own), ("twin", Name::Twin)] {
        let got = sphere_run(b, true, &loops, about(), name);
        println!("open junction ({which}): {got:?}");
        if got.is_ok() {
            off.push(format!("{which}: certified an open loop: {got:?}"));
        }
    }
    assert!(off.is_empty(), "{off:#?}");
}

//! **Acceptance 5 on the conic quarter torus** (`docs/QUAD-MEASURE-SPEC.md`;
//! PR 5741's review, m3): a closed body whose walls are the review's
//! costly face class — PR 5589's first reviewer's 9×9 conic torus, a
//! rational surface of revolution at `R = 24 m, r = 6 m` — measured
//! through the door under `Resolution(1.024e-6 m)`, answers a bracket
//! that holds Pappus's volume, never a refusal. Its walls' rounds cost
//! 81 spans' cells under the one per-face work wall, so they stop at
//! the wall short of the resolution.
//!
//! **The body.** No builder closes a body around a conic patch, so the
//! file is made: the kernel revolves a circle of radius `r` centred at
//! `R` a quarter turn about `y` and writes it, and the two toroidal
//! half-bands it writes are swapped for the same torus as rational
//! B-spline surfaces of 9 conic spans each way (u the minor half
//! circle, 20° a span; v the major quarter turn, 10° a span; Piegl–
//! Tiller's surface of revolution, the review's `conic_quarter_torus`
//! over a half minor circle). The two equator circles the halves meet
//! on are swapped for their own boundary columns, the same bits, which
//! the importer adopts as iso-curves (two tangent walls have no
//! transverse intersection to adopt the circle by). The caps stay the
//! writer's planes.
//!
//! **The oracle** is independent of the quadrature: Pappus, the disc's
//! area times the path of its centre, `π r² · R · π/2 = 432 π²` m³.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use std::fmt::Write as _;

use geom_core::{Point2, Tol, Vec2};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use quantity::Length;
use step_import::{ImportOptions, StepImport, import_step};

/// The torus: major radius `R`, minor `r` (m), and the conic spans a
/// direction.
const R: f64 = 24.0;
const MINOR: f64 = 6.0;
const SPANS: usize = 9;

/// An exact unit value where the angle is a multiple of `π/2`: the
/// boundary columns must be the equators' bits, and `sin π` is not `0`.
fn snapped(x: f64) -> f64 {
    for exact in [0.0, 1.0, -1.0] {
        if (x - exact).abs() < 1e-12 {
            return exact;
        }
    }
    x
}

/// The unit circle's arc `[a0, a1]` as `SPANS` rational quadratic
/// spans joined at double knots: control points `(cos, sin)` and
/// weights, the middle point of a span off the circle by `1/cos(a/2)`
/// with that weight.
fn unit_arc(a0: f64, a1: f64) -> Vec<((f64, f64), f64)> {
    #[allow(clippy::cast_precision_loss)]
    let a = (a1 - a0) / SPANS as f64;
    let at = |t: f64, k: f64| (snapped(t.cos() / k), snapped(t.sin() / k));
    let mut arc = vec![(at(a0, 1.0), 1.0)];
    for i in 0..SPANS {
        #[allow(clippy::cast_precision_loss)]
        let (b0, b1) = (a0 + a * i as f64, a0 + a * (i + 1) as f64);
        let c = (0.5 * a).cos();
        arc.push((at(0.5 * (b0 + b1), c), c));
        arc.push((at(b1, 1.0), 1.0));
    }
    arc
}

/// The STEP knot multiplicities and values both spline directions take.
fn knots() -> (String, String) {
    let mut mult = vec!["3".to_owned()];
    mult.extend((1..SPANS).map(|_| "2".to_owned()));
    mult.push("3".to_owned());
    #[allow(clippy::cast_precision_loss)]
    let values: Vec<String> = (0..=SPANS)
        .map(|k| format!("{:?}", k as f64 / SPANS as f64))
        .collect();
    (
        format!("({})", mult.join(", ")),
        format!("({})", values.join(", ")),
    )
}

/// Appends `#id = CARTESIAN_POINT` records to `points` from `next` on,
/// returning their references.
fn points(xyz: &[(f64, f64, f64)], next: &mut usize, points: &mut String) -> Vec<String> {
    xyz.iter()
        .map(|&(x, y, z)| {
            writeln!(
                points,
                "#{next} = CARTESIAN_POINT('', ({x:?}, {y:?}, {z:?}));"
            )
            .unwrap();
            *next += 1;
            format!("#{}", *next - 1)
        })
        .collect()
}

/// A point on the torus revolved about `y` from `+x` toward `−z`, at
/// the minor arc's control point `(cx, sx)` and the major arc's
/// `(ca, sa)`.
fn on_torus((cx, sx): (f64, f64), (ca, sa): (f64, f64)) -> (f64, f64, f64) {
    let d = R + MINOR * cx;
    (d * ca, MINOR * sx, -d * sa)
}

/// The half band `φ ∈ [φ0, φ1]` as the conic surface, u the minor arc.
fn conic_band(phi: (f64, f64), next: &mut usize, extra: &mut String) -> String {
    let (minor, major) = (unit_arc(phi.0, phi.1), unit_arc(0.0, FRAC_PI_2));
    let (mut rows, mut weights) = (Vec::new(), Vec::new());
    for &(m, wm) in &minor {
        let xyz: Vec<_> = major.iter().map(|&(a, _)| on_torus(m, a)).collect();
        rows.push(format!("({})", points(&xyz, next, extra).join(", ")));
        let w: Vec<String> = major
            .iter()
            .map(|&(_, wa)| format!("{:?}", wm * wa))
            .collect();
        weights.push(format!("({})", w.join(", ")));
    }
    let (mult, values) = knots();
    format!(
        "( BOUNDED_SURFACE() B_SPLINE_SURFACE(2, 2, ({}), .UNSPECIFIED., .F., .F., .F.) \
         B_SPLINE_SURFACE_WITH_KNOTS({mult}, {mult}, {values}, {values}, .UNSPECIFIED.) \
         GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_SURFACE(({})) \
         REPRESENTATION_ITEM('') SURFACE() )",
        rows.join(", "),
        weights.join(", ")
    )
}

/// The equator of radius `rho` as the bands' own boundary column.
fn equator(rho: f64, next: &mut usize, extra: &mut String) -> String {
    let major = unit_arc(0.0, FRAC_PI_2);
    let xyz: Vec<_> = major
        .iter()
        .map(|&((ca, sa), _)| (rho * ca, 0.0, -rho * sa))
        .collect();
    let refs = points(&xyz, next, extra);
    let w: Vec<String> = major.iter().map(|&(_, wa)| format!("{wa:?}")).collect();
    let (mult, values) = knots();
    format!(
        "( BOUNDED_CURVE() B_SPLINE_CURVE(2, ({}), .UNSPECIFIED., .F., .F.) \
         B_SPLINE_CURVE_WITH_KNOTS({mult}, {values}, .UNSPECIFIED.) CURVE() \
         GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE(({})) REPRESENTATION_ITEM('') )",
        refs.join(", "),
        w.join(", ")
    )
}

/// The record `#id = <head>...;` of `text` whose head is `head`, as its
/// id and the whole record.
fn records<'t>(text: &'t str, head: &str) -> Vec<(&'t str, &'t str)> {
    text.lines()
        .filter_map(|l| {
            let (id, rest) = l.strip_prefix('#')?.split_once(" = ")?;
            rest.starts_with(head).then_some((id, l))
        })
        .collect()
}

/// The quarter-turn torus segment's file with its walls made conic.
fn conic_quarter_torus_step(tol: Tol) -> String {
    let lp = bulge_loop(vec![
        (Point2::new(R - MINOR, 0.0), 1.0),
        (Point2::new(R + MINOR, 0.0), 1.0),
    ]);
    let vp = profile::Profile::new(SketchPlane::<f64>::xy(), vec![lp])
        .validate(tol)
        .expect("the disc profile validates");
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = sweep::revolve(&vp, axis, sweep::Revolution::Partial(FRAC_PI_2), tol)
        .expect("the quarter turn revolves")
        .body;
    let mut text = step_export::step_string(
        &crate::common::at_rest(&body),
        &step_export::StepOptions::default(),
        tol,
    )
    .expect("the torus segment writes");
    let (mut next, mut extra) = (1_000_000usize, String::new());
    // The half bands: the one above the `xz` plane is `φ ∈ [0, π]`. The
    // conic chart's u × v is the torus's inward normal, so each face
    // takes the opposite sense to its surface.
    let tori = records(&text, "TOROIDAL_SURFACE");
    assert_eq!(tori.len(), 2, "the writer splits the wall at its equators");
    let mut swaps: Vec<(String, String)> = Vec::new();
    for (id, record) in tori {
        let face = records(&text, "ADVANCED_FACE")
            .into_iter()
            .find(|(_, l)| l.contains(&format!(", #{id}, .T.);")))
            .expect("the band's face")
            .1;
        let phi = if face_is_upper(&text, face) {
            (0.0, PI)
        } else {
            (PI, 2.0 * PI)
        };
        let band = conic_band(phi, &mut next, &mut extra);
        swaps.push((record.to_owned(), format!("#{id} = {band};")));
        swaps.push((face.to_owned(), face.replace(".T.);", ".F.);")));
    }
    for rho in [R - MINOR, R + MINOR] {
        let (id, record) = records(&text, "CIRCLE")
            .into_iter()
            .find(|(_, l)| l.ends_with(&format!(", {rho:?});")))
            .expect("the equator circle");
        let column = equator(rho, &mut next, &mut extra);
        swaps.push((record.to_owned(), format!("#{id} = {column};")));
    }
    for (old, new) in swaps {
        text = text.replacen(&old, &new, 1);
    }
    text.replacen("ENDSEC;\nEND-ISO", &format!("{extra}ENDSEC;\nEND-ISO"), 1)
}

/// Whether the band face `face` lies above the `xz` plane, read off
/// the rim semicircle its loop carries: a circle's points sweep from
/// its reference direction toward `axis × ref`, so the semicircle — and
/// the band it bounds — is above the plane where that vector is `+y`.
fn face_is_upper(text: &str, face: &str) -> bool {
    let record = |r: &str| -> &str {
        let head = format!("{r} = ");
        text.lines()
            .find(|l| l.starts_with(&head))
            .unwrap_or_else(|| panic!("record {r}"))
    };
    // A record's `#` references, in order.
    let refs = |l: &str| -> Vec<String> {
        l.split('#')
            .skip(2)
            .map(|s| {
                format!(
                    "#{}",
                    s.split(|c: char| !c.is_ascii_digit()).next().unwrap()
                )
            })
            .collect()
    };
    let dir = |r: &str| -> [f64; 3] {
        let l = record(r);
        let inner = &l[l.rfind('(').unwrap() + 1..l.find(')').unwrap()];
        let v: Vec<f64> = inner
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        [v[0], v[1], v[2]]
    };
    let lp = &refs(record(&refs(face)[0]))[0];
    for oe in refs(record(lp)) {
        let ec = &refs(record(&oe))[0];
        let curve = record(&refs(record(ec))[2]);
        let rim = curve.contains("CIRCLE") && curve.ends_with(&format!(", {MINOR:?});"));
        if !rim {
            continue;
        }
        let place = refs(record(&refs(curve)[0]));
        let (axis, refd) = (dir(&place[1]), dir(&place[2]));
        return axis[2] * refd[0] - axis[0] * refd[2] > 0.0;
    }
    panic!("the band carries a rim semicircle")
}

/// **The conic quarter torus answers a bracket at the wall that holds
/// Pappus** under a resolution of 1.024e-6 m, at every ε.
#[test]
fn the_conic_quarter_torus_answers_a_bracket_holding_pappus() {
    let tol = Tol::witness();
    let text = conic_quarter_torus_step(tol);
    let Ok(StepImport::Solid { body, .. }) = import_step(&text, &ImportOptions::default(), tol)
    else {
        panic!("the conic torus segment imports as a solid");
    };
    let walls = body
        .body()
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.body().get_surface(f.surface),
                Some(geom::Surface::Nurbs(_))
            )
        })
        .count();
    assert_eq!(walls, 2, "both half bands are the conic surface");
    let measured = body
        .measure(topo::Settle::Resolution(Length::from_meters(1.024e-6)))
        .unwrap_or_else(|e| panic!("the conic torus measures, never refuses: {e:?}"));
    let e = measured.enclosure;
    let pappus = PI * MINOR * MINOR * R * FRAC_PI_2;
    assert!(
        (e.volume - pappus).abs() <= e.volume_pad,
        "the bracket {} ± {:e} holds Pappus's {pappus}",
        e.volume,
        e.volume_pad
    );
    assert_eq!(
        measured.stop,
        topo::Stop::Wall,
        "the costly walls stop at their wall, short of the resolution: {measured:?}"
    );
    eprintln!(
        "ACCEPTANCE-5 conic quarter torus @ ε = {:e}, resolution 1.024e-6 m: stop {:?}, V = {} ± {:e} (Pappus {pappus})",
        tol.eps(),
        measured.stop,
        e.volume,
        e.volume_pad
    );
}

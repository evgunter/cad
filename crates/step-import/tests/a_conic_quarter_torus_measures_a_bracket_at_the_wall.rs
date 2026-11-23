//! **Acceptance 5 on a foreign conic patch** (`docs/QUAD-MEASURE-SPEC.md`;
//! PR 5741's review, m3), and the promotion that keeps the analytic
//! case off it.
//!
//! **The bodies.** No builder closes a body around a conic patch, so the
//! file is made: the kernel revolves a circle of radius `r` centred at
//! `R` a quarter turn about `y` and writes it, and the two toroidal
//! half-bands it writes are swapped for rational B-spline surfaces of 9
//! conic spans each way (u the minor half arc, 20° a span; v the major
//! quarter turn, 10° a span; Piegl–Tiller's surface of revolution, the
//! review's `conic_quarter_torus` over a half minor arc). The two equator
//! circles the halves meet on are swapped for their own boundary columns,
//! the same bits, which the importer adopts as iso-curves (two tangent
//! walls have no transverse intersection to adopt the circle by). The
//! caps stay the writer's planes.
//!
//! - **The conic quarter torus** (`R = 24 m, r = 6 m`, PR 5589's first
//!   reviewer's 9×9 conic torus): each wall's net is exactly a torus, so
//!   stage-1 recognition promotes it on the whole patch before the
//!   C⁰-knot split (`[ev]` PR 6014's ruling), and the body imports with
//!   two torus walls and no spline wall at all.
//! - **The conic quarter elliptic tube**: the same file with the minor
//!   profile an ellipse of semi-axes `r` and `ASPECT·r` (its height
//!   scaled), the rim semicircles swapped for the bands' own boundary
//!   rows. A rational conic of revolution that is no analytic kind, so
//!   it stays a spline: the import splits each wall at its C⁰ knots into
//!   one-span walls (`docs/DESIGN.md` [topology]), and measured through
//!   the door under `Resolution(1.024e-6 m)` it answers a bracket, never
//!   a refusal, with every one-span wall meeting the resolution.
//!
//! **The oracle** is independent of the quadrature: Pappus, the
//! profile's area times the path of its centre, `π r·(ASPECT·r) · R · π/2`
//! m³ (`432 π²` for the torus).

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

/// The elliptic tube's height over its width.
const ASPECT: f64 = 0.75;

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

/// A point on the tube of height `aspect` over its width revolved about
/// `y` from `+x` toward `−z`, at the minor arc's control point
/// `(cx, sx)` and the major arc's `(ca, sa)`.
fn on_tube((cx, sx): (f64, f64), (ca, sa): (f64, f64), aspect: f64) -> (f64, f64, f64) {
    let d = R + MINOR * cx;
    (d * ca, MINOR * aspect * sx, -d * sa)
}

/// The half band `φ ∈ [φ0, φ1]` as the conic surface, u the minor arc.
fn conic_band(phi: (f64, f64), aspect: f64, next: &mut usize, extra: &mut String) -> String {
    let (minor, major) = (unit_arc(phi.0, phi.1), unit_arc(0.0, FRAC_PI_2));
    let (mut rows, mut weights) = (Vec::new(), Vec::new());
    for &(m, wm) in &minor {
        let xyz: Vec<_> = major.iter().map(|&(a, _)| on_tube(m, a, aspect)).collect();
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

/// The half band `φ ∈ [φ0, φ1]`'s boundary row at the major arc's end
/// `(ca, sa)` — the rim it shares with a cap — as a curve, the same bits
/// as the band's row, running with the band's `u`.
fn rim(
    phi: (f64, f64),
    (ca, sa): (f64, f64),
    aspect: f64,
    next: &mut usize,
    extra: &mut String,
) -> String {
    let minor = unit_arc(phi.0, phi.1);
    let xyz: Vec<_> = minor
        .iter()
        .map(|&(m, _)| on_tube(m, (ca, sa), aspect))
        .collect();
    let refs = points(&xyz, next, extra);
    let w: Vec<String> = minor.iter().map(|&(_, wm)| format!("{wm:?}")).collect();
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

/// The record `#r = …;` of `text`.
fn record<'t>(text: &'t str, r: &str) -> &'t str {
    let head = format!("{r} = ");
    text.lines()
        .find(|l| l.starts_with(&head))
        .unwrap_or_else(|| panic!("record {r}"))
}

/// A record's `#` references, in order.
fn refs(l: &str) -> Vec<String> {
    l.split('#')
        .skip(2)
        .map(|s| {
            format!(
                "#{}",
                s.split(|c: char| !c.is_ascii_digit()).next().unwrap()
            )
        })
        .collect()
}

/// The coordinate triple a `CARTESIAN_POINT` or `DIRECTION` record `r`
/// states.
fn triple(text: &str, r: &str) -> [f64; 3] {
    let l = record(text, r);
    let inner = &l[l.rfind('(').unwrap() + 1..l.find(')').unwrap()];
    let v: Vec<f64> = inner
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    [v[0], v[1], v[2]]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// The quarter-turn segment's file with its walls made conic, the tube's
/// height `aspect` times its width (module docs).
fn conic_quarter_step(tol: Tol, aspect: f64) -> String {
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
    let [(torus, _)] = tori[..] else {
        panic!("the writer writes the wall's one surface once: {tori:?}")
    };
    let cites = format!(", #{torus}, .T.);");
    let walls: Vec<&str> = records(&text, "ADVANCED_FACE")
        .into_iter()
        .filter(|(_, l)| l.contains(&cites))
        .map(|(_, l)| l)
        .collect();
    assert_eq!(walls.len(), 2, "the writer splits the wall at its equators");
    // Each half band is a surface entity of its own, cited by its face.
    let mut swaps: Vec<(String, String)> = Vec::new();
    for face in walls {
        let phi = if face_is_upper(&text, face) {
            (0.0, PI)
        } else {
            (PI, 2.0 * PI)
        };
        let band = conic_band(phi, aspect, &mut next, &mut extra);
        let id = next;
        next += 1;
        extra.push_str(&format!("#{id} = {band};\n"));
        swaps.push((
            face.to_owned(),
            face.replace(&cites, &format!(", #{id}, .F.);")),
        ));
    }
    for rho in [R - MINOR, R + MINOR] {
        let (id, record) = records(&text, "CIRCLE")
            .into_iter()
            .find(|(_, l)| l.ends_with(&format!(", {rho:?});")))
            .expect("the equator circle");
        let column = equator(rho, &mut next, &mut extra);
        swaps.push((record.to_owned(), format!("#{id} = {column};")));
    }
    // The tube's rims, where it is no circle: each rim semicircle is
    // swapped for its band's boundary row at its cap, and its edge
    // takes the sense that keeps its run from one vertex to the other.
    if aspect.to_bits() != 1.0_f64.to_bits() {
        for (id, circle) in records(&text, "CIRCLE")
            .into_iter()
            .filter(|(_, l)| l.ends_with(&format!(", {MINOR:?});")))
        {
            let edge = records(&text, "EDGE_CURVE")
                .into_iter()
                .find(|(_, l)| l.contains(&format!(", #{id}, .T.);")))
                .expect("the rim's edge")
                .1;
            let place = refs(record(&text, &refs(circle)[0]));
            let (centre, axis, refd) = (
                triple(&text, &place[0]),
                triple(&text, &place[1]),
                triple(&text, &place[2]),
            );
            let start = triple(&text, &refs(record(&text, &refs(edge)[0]))[0]);
            // The semicircle's middle, a quarter turn on from its start.
            let s: [f64; 3] = core::array::from_fn(|k| (start[k] - centre[k]) / MINOR);
            let side = cross(axis, refd);
            let (c0, s0) = (dot(s, refd), dot(s, side));
            let upper = -s0 * refd[1] + c0 * side[1] > 0.0;
            let phi = if upper { (0.0, PI) } else { (PI, 2.0 * PI) };
            // The cap at the turn's start is the `xy` plane, its rims
            // centred on `+x`; the other's on `−z`.
            let cap = if centre[0].abs() > centre[2].abs() {
                (1.0, 0.0)
            } else {
                (0.0, 1.0)
            };
            let curve = rim(phi, cap, aspect, &mut next, &mut extra);
            swaps.push((circle.to_owned(), format!("#{id} = {curve};")));
            let first = on_tube(unit_arc(phi.0, phi.1)[0].0, cap, aspect);
            let along = (start[0] - first.0).abs()
                + (start[1] - first.1).abs()
                + (start[2] - first.2).abs()
                < 1e-9;
            if !along {
                swaps.push((edge.to_owned(), edge.replace(".T.);", ".F.);")));
            }
        }
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
    let lp = &refs(record(text, &refs(face)[0]))[0];
    for oe in refs(record(text, lp)) {
        let ec = &refs(record(text, &oe))[0];
        let curve = record(text, &refs(record(text, ec))[2]);
        let rim = curve.contains("CIRCLE") && curve.ends_with(&format!(", {MINOR:?});"));
        if !rim {
            continue;
        }
        let place = refs(record(text, &refs(curve)[0]));
        let (axis, refd) = (triple(text, &place[1]), triple(text, &place[2]));
        return axis[2] * refd[0] - axis[0] * refd[2] > 0.0;
    }
    panic!("the band carries a rim semicircle")
}

/// The walls the file's spline surfaces split into at their C⁰ knots:
/// per surface, one more than its crease runs (interior multiplicity at
/// least the degree) along each direction, multiplied.
fn one_span_walls(text: &str) -> usize {
    text.lines()
        .filter_map(|l| {
            let (_, head) = l.split_once("B_SPLINE_SURFACE(")?;
            let mut degrees = head.split(',').map(|d| d.trim().parse::<usize>().unwrap());
            let (p, q) = (degrees.next()?, degrees.next()?);
            let (_, knots) = l.split_once("B_SPLINE_SURFACE_WITH_KNOTS(")?;
            let mut lists = knots.split(')').map(|m| {
                m.trim_start_matches([',', ' ', '('])
                    .split(',')
                    .map(|k| k.trim().parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            });
            let (mu, mv) = (lists.next()?, lists.next()?);
            let tiles =
                |m: &[usize], d: usize| 1 + m[1..m.len() - 1].iter().filter(|&&k| k >= d).count();
            Some(tiles(&mu, p) * tiles(&mv, q))
        })
        .sum()
}

/// The file `text` imported as a solid at `tol`.
fn solid(text: &str, tol: Tol) -> (topo::AtRestBody<f64>, StepImport) {
    match import_step(text, &ImportOptions::default(), tol) {
        Ok(imported @ StepImport::Solid { .. }) => {
            let StepImport::Solid { ref body, .. } = imported else {
                unreachable!("matched as a solid")
            };
            (body.clone(), imported)
        }
        Ok(_) => panic!("the conic segment imports as a solid, not a wireframe"),
        Err(e) => panic!("the conic segment imports as a solid: {}", e.render()),
    }
}

/// The body's faces whose surface `kind` accepts.
fn faces_where(body: &topo::AtRestBody<f64>, kind: impl Fn(&geom::Surface<f64>) -> bool) -> usize {
    body.faces()
        .filter(|(fk, _)| body.face_surface(*fk).is_ok_and(&kind))
        .count()
}

/// `body` measured under a resolution of 1.024e-6 m: the bracket holds
/// Pappus's `pappus` to within `moved` — the most the import may move the
/// walls by, as a volume — and the oracle's own `f64` rounding, and every
/// face meets the resolution.
fn measures_pappus(body: &topo::AtRestBody<f64>, pappus: f64, moved: f64, what: &str, tol: Tol) {
    let measured = body
        .measure(topo::Settle::Resolution(Length::from_meters(1.024e-6)))
        .unwrap_or_else(|e| panic!("{what} measures, never refuses: {e:?}"));
    let e = measured.enclosure;
    let oracle = 8.0 * f64::EPSILON * pappus;
    assert!(
        (e.volume - pappus).abs() <= e.volume_pad + moved + oracle,
        "{what}: the bracket {} ± {:e} holds Pappus's {pappus} to within the walls' \
         move {moved:e} m³ and the oracle's rounding {oracle:e} m³",
        e.volume,
        e.volume_pad
    );
    assert_eq!(
        measured.stop,
        topo::Stop::Met,
        "{what}: every wall meets the resolution: {measured:?}"
    );
    eprintln!(
        "ACCEPTANCE-5 {what} @ ε = {:e}, resolution 1.024e-6 m: stop {:?}, V = {} ± {:e} (Pappus {pappus})",
        tol.eps(),
        measured.stop,
        e.volume,
        e.volume_pad
    );
}

/// **The conic quarter torus imports with two torus walls, promoted on
/// the whole patch, and its bracket holds Pappus**, at the run's ε (each
/// eps row CI runs, `CAD_TOLERANCE_EPS`): each
/// wall's 81 conic spans become one analytic face, which the C⁰-knot
/// split never cuts, and each promotion is reported with the certified
/// residual it moved the geometry by, within ε_in.
#[test]
fn the_conic_quarter_torus_imports_as_two_torus_walls_holding_pappus() {
    let tol = Tol::witness();
    let text = conic_quarter_step(tol, 1.0);
    assert_eq!(
        one_span_walls(&text),
        162,
        "the file states 81 spans per wall"
    );
    let (body, imported) = solid(&text, tol);
    let normalizations = imported.normalizations();
    assert_eq!(
        faces_where(&body, |s| matches!(s, geom::Surface::Torus { .. })),
        2,
        "each half band imports as one torus wall"
    );
    assert_eq!(
        faces_where(&body, |s| matches!(s, geom::Surface::Nurbs(_))),
        0,
        "no spline wall is left to split"
    );
    let promoted: Vec<f64> = normalizations
        .iter()
        .filter_map(|n| match n.kind {
            step_import::NormalizationKind::SurfacePromotion {
                to: step_import::PromotedKind::Torus,
                residual,
            } => Some(residual),
            _ => None,
        })
        .collect();
    assert_eq!(
        promoted.len(),
        2,
        "both walls report their promotion: {normalizations:?}"
    );
    for &residual in &promoted {
        assert!(
            residual <= imported.eps_in(),
            "the promotion moved the geometry by at most ε_in = {:e}: {residual:e}",
            imported.eps_in()
        );
    }
    // D7 lets the import move the walls by at most ε_in, so the volume
    // moves by at most ε_in times the walls' area, `(2πr)(2πR)/4` on the
    // quarter torus — a cap fixed by the file's band, not by the residual
    // the promotion reports.
    let moved = imported.eps_in() * PI * PI * MINOR * R;
    measures_pappus(
        &body,
        PI * MINOR * MINOR * R * FRAC_PI_2,
        moved,
        "conic quarter torus",
        tol,
    );
}

/// **The conic quarter elliptic tube answers a bracket at the wall that
/// holds Pappus** under a resolution of 1.024e-6 m, at the run's ε (each
/// eps row CI runs, `CAD_TOLERANCE_EPS`): no kind
/// promotes it, so the import splits each wall at its C⁰ knots, and every
/// one-span wall meets the resolution.
#[test]
fn the_conic_quarter_elliptic_tube_answers_a_bracket_holding_pappus() {
    let tol = Tol::witness();
    let text = conic_quarter_step(tol, ASPECT);
    let (body, _) = solid(&text, tol);
    // The import splits each wall at its C⁰ knots (`docs/DESIGN.md`
    // [topology]); the measure then reads one-span walls.
    assert_eq!(
        faces_where(&body, |s| matches!(s, geom::Surface::Nurbs(_))),
        one_span_walls(&text),
        "each half band's conic spans import as one wall per span"
    );
    measures_pappus(
        &body,
        PI * MINOR * (ASPECT * MINOR) * R * FRAC_PI_2,
        0.0,
        "conic quarter elliptic tube",
        tol,
    );
}

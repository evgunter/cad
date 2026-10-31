//! **The section certificate's spline × spline rows**: the classifier
//! through the march door (C3's whole reading of two walls, the rows and
//! corners they share handed in) and the per-pair rule over what it
//! lists.
//!
//! Every row names the mutant that turns it red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::search_kit::band;
use super::section_cert_rows::{IN, OUT, at, p};
use super::*;
use geom::Surface;
use geom_brep::ssi::{ChartCorner, ChartEnd, PairCorner};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Vec2};

/// The pair's section through the march door, `known` handed in, over a
/// reach the spline arm does not read.
fn read(f: &Surface<f64>, g: &Surface<f64>, known: Known<f64>) -> Section<f64> {
    classify_known(
        f,
        g,
        Reach {
            centre: p(0.0, 0.0, 0.0),
            radius: 1.0,
        },
        Some(geom_brep::SsiLane::marched()),
        band(),
        known,
    )
}

/// A biquadratic over `[0, 1]²` standing over the rectangle `xs × ys`,
/// `x` along `u` and `y` along `v`, whose height is `z(i, j)` at control
/// point `(i, j)`.
fn sheet(xs: (f64, f64), ys: (f64, f64), z: &dyn Fn(usize, usize) -> f64) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let at = |(lo, hi): (f64, f64), i: usize| {
        #[allow(clippy::cast_precision_loss)]
        let t = 0.5 * i as f64;
        lo + (hi - lo) * t
    };
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            p(at(xs, i), at(ys, j), z(i, j))
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap(),
    ))
}

/// The unit square.
const UNIT: (f64, f64) = (0.0, 1.0);

/// The dome `z = −(x − ½)² − (y − ½)²` over the unit square.
fn dome() -> Surface<f64> {
    let g = [-0.25, 0.25, -0.25];
    sheet(UNIT, UNIT, &|i, j| g[i] + g[j])
}

/// `(t − ½)²` over `[a, b]` as a quadratic's Bernstein coefficients.
fn square_about_half((a, b): (f64, f64)) -> [f64; 3] {
    let f = |t: f64| (t - 0.5).powi(2);
    [f(a), f(a) + (b - a) * (a - 0.5), f(b)]
}

/// The bowl `z = (x − ½)² + (y − ½)² − 0.1` over its own rectangle, so
/// its chart is not the dome's: it meets the dome in the circle of
/// radius `√0.05` about `(½, ½, −0.05)`, inside both rectangles.
fn bowl() -> Surface<f64> {
    let (xs, ys) = ((-0.2, 1.1), (-0.1, 1.3));
    let (gx, gy) = (square_about_half(xs), square_about_half(ys));
    sheet(xs, ys, &|i, j| gx[i] + gy[j] - 0.1)
}

/// The one component of a section of one.
fn lone(s: &Section<f64>) -> Component<f64> {
    match s {
        Section::Components { parts, .. } if parts.len() == 1 => parts[0],
        other => panic!("not one component: {other:?}"),
    }
}

/// **A loop inside two spline walls is never cleared, and its witness is
/// placed on the band widened by its pad.** The dome and the bowl meet
/// in a circle inside both rectangles: one closed component, reaching no
/// chart boundary, its witness within its pad of the circle and that pad
/// a few bands. With no event its witness decides it: `In` both faces is
/// R-loop, and it clears only `Out` of a face (W3); with an event it is
/// the lone evented component (W4). Against a face whose edge stands
/// one and a half pads past the band, the placement, padded by twice the
/// pad, reads the witness on the edge, so it does not clear.
///
/// Red against the boundary clause read true for a closed loop (every
/// verdict clears as `Reaches`), against the witness's pad dropped at
/// its placement (the component's or the rule's hand-off), and against a
/// pad that drops the root enclosure's reach. The witness also lies
/// within twice its pad of each wall, a check off the closed form.
#[test]
fn a_loop_inside_two_spline_walls_is_never_cleared() {
    let s = read(&dome(), &bowl(), Known::default());
    let c = lone(&s);
    assert!(c.reaches.is_none(), "a loop reaches no side: {c:?}");
    let w = c.witness.expect("a loop carries a witness");
    let r = ((w.x - 0.5).powi(2) + (w.y - 0.5).powi(2)).sqrt();
    let off = ((r - 0.05_f64.sqrt()).powi(2) + (w.z + 0.05).powi(2)).sqrt();
    assert!(
        off <= c.pad && c.pad > 0.0 && c.pad <= 1e3 * band().zero(),
        "the witness stands {off:e} off the circle, within its pad {:e}, a few bands of {:e}",
        c.pad,
        band().zero()
    );
    let (dx, dy) = (w.x - 0.5, w.y - 0.5);
    let r2 = dx * dx + dy * dy;
    let (on_dome, on_bowl) = ((w.z + r2).abs(), (w.z - r2 + 0.1).abs());
    assert!(
        on_dome <= 2.0 * c.pad && on_bowl <= 2.0 * c.pad,
        "the witness stands {on_dome:e} and {on_bowl:e} off the walls, past twice its pad"
    );
    let rule = |evented, place| certify(&s, evented, band(), |_| Sheet::Not, place);
    assert_eq!(rule(false, at(IN, IN)), Err(Refusal::Loop), "in both");
    assert_eq!(
        rule(false, at(None, None)),
        Err(Refusal::Undecided),
        "placed nowhere"
    );
    assert_eq!(
        rule(false, at(OUT, IN)),
        Ok(vec![Cleared::Out(Side::F)]),
        "out of F"
    );
    assert_eq!(
        rule(true, at(IN, IN)),
        Ok(vec![Cleared::LoneEvented]),
        "evented"
    );
    let d = band().zero() + 1.5 * c.pad;
    let edge = |_, wide: Band| [if d > wide.zero() { OUT } else { None }, IN];
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, edge),
        Err(Refusal::Undecided),
        "a witness inside its pad of an edge does not clear"
    );
}

/// **An open branch between two walls clears by W1′.** The ramps
/// `z = ±0.4·(x − ½)`, the second standing past the first's `v` sides,
/// meet in the line `x = ½` across the first wall's rectangle: one
/// component, an open branch between two crossings of the first chart's
/// `v` sides, which bound a face there, so it clears whatever its
/// witness places. Red against an open branch read as a loop (its
/// witness, placed nowhere, is R-undec).
#[test]
fn an_open_branch_between_two_walls_clears_by_w1_prime() {
    let up = sheet(UNIT, UNIT, &|i, _| 0.4 * (0.5 * i as f64 - 0.5));
    let down = sheet(UNIT, (-0.25, 1.25), &|i, _| -0.4 * (0.5 * i as f64 - 0.5));
    let s = read(&up, &down, Known::default());
    let c = lone(&s);
    assert_eq!(c.reaches, Some(Side::F), "the branch reaches F's sides: {c:?}");
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
        Ok(vec![Cleared::Reaches(Side::F)]),
        "{s:?}"
    );
}

/// The ramp `z = h·y` over the unit square, bent in `x` off its `v = 0`
/// row, which is the segment `x ∈ [0, 1]` of the `x` axis.
fn ramp(h: f64) -> Surface<f64> {
    let bend = [0.0, 0.1, 0.0];
    sheet(UNIT, UNIT, &|i, j| {
        #[allow(clippy::cast_precision_loss)]
        let y = 0.5 * j as f64;
        (h + bend[i]) * y
    })
}

/// The two ramps' shared row, the `v = 0` side of each, over the carrier
/// parameter `x`.
fn shared_rim() -> Known<f64> {
    let image = geom_brep::Pcurve::IsoLine {
        p0: Point2::new(0.0, 0.0),
        pl: Vec2::new(1.0, 0.0),
    };
    Known {
        rows: vec![geom_brep::KnownRow {
            images: [image.clone(), image],
            span: (0.0, 1.0),
        }],
        corners: Vec::new(),
    }
}

/// **A shared rim handed in clears as a `Side` region; one not handed in
/// refuses as the boundary pass's.** Two ramps rising either way from
/// their shared `v = 0` row meet along it alone. Handed in, the row's
/// cover reads the section there as the row: one `Side` region on both
/// charts' `v = 0` sides, cleared by W1′. Not handed in, the pass meets
/// the section running along a side of both rectangles and refuses it,
/// named (R-tan). Red against a `Side` region read as a loop (no
/// witness: R-undec), and against the cover's width dropped (the
/// accounting meets cells beside the row that no cover holds).
#[test]
fn a_shared_rim_handed_in_clears_and_one_not_refuses() {
    let (f, g) = (ramp(0.5), ramp(-0.5));
    let s = read(&f, &g, shared_rim());
    let c = lone(&s);
    assert_eq!(c.reaches, Some(Side::F), "the rim reaches both: {c:?}");
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
        Ok(vec![Cleared::Reaches(Side::F)]),
        "handed in: {s:?}"
    );
    let bare = read(&f, &g, Known::default());
    assert_eq!(
        certify(&bare, false, band(), |_| Sheet::Not, at(IN, IN)),
        Err(Refusal::Tangent("ssi_pair_boundary")),
        "not handed in: {bare:?}"
    );
}

/// The sheet `z = 0.2·x·y` over the unit square: its corner `(0, 0)` at
/// the origin.
fn flat() -> Surface<f64> {
    sheet(UNIT, UNIT, &|i, j| {
        #[allow(clippy::cast_precision_loss)]
        let (x, y) = (0.5 * i as f64, 0.5 * j as f64);
        0.2 * x * y
    })
}

/// **A shared corner vertex is a `Corner` region where the section leaves
/// there; where a branch starts there it is no region.** The plane
/// `z = x − y` over `x ∈ [−1, 0]` meets the flat sheet, which stands over
/// `x ≥ 0`, at their shared corner alone: handed in, the corner's cell
/// reads the section leaving the domain along some coordinate whichever
/// way it runs, one `Corner` region, cleared by W1′; not handed in, the
/// pass meets the section at a corner of both rectangles and refuses it.
/// The plane `z = x − y/2` over `y ∈ [0, 1.5]` shares the same corner and
/// runs into both rectangles from it: the corner is a crossing, the
/// branch's end, which limb 3 does not yet certify on four domain faces
/// at once
/// (`work/ssiedge/a-pair-branch-ending-at-a-corner-of-both-rectangles-is-not-certified.md`),
/// so the pair refuses rather than clearing. Red against a `Corner`
/// region read as a loop (R-undec), and against the corner's signs read
/// as leaving where a branch starts (a region that clears).
#[test]
fn a_shared_corner_is_a_region_or_a_crossing() {
    let low = ChartCorner {
        u: ChartEnd::Low,
        v: ChartEnd::Low,
    };
    let leaves = sheet((-1.0, 0.0), UNIT, &|i, j| {
        #[allow(clippy::cast_precision_loss)]
        let (x, y) = (0.5 * i as f64 - 1.0, 0.5 * j as f64);
        x - y
    });
    let corner = |b: ChartCorner| Known {
        rows: Vec::new(),
        corners: vec![PairCorner([low, b])],
    };
    let at_corner = ChartCorner {
        u: ChartEnd::High,
        v: ChartEnd::Low,
    };
    let s = read(&flat(), &leaves, corner(at_corner));
    assert_eq!(lone(&s).reaches, Some(Side::F), "{s:?}");
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
        Ok(vec![Cleared::Reaches(Side::F)]),
        "the corner handed in: {s:?}"
    );
    let bare = read(&flat(), &leaves, Known::default());
    assert_eq!(
        certify(&bare, false, band(), |_| Sheet::Not, at(IN, IN)),
        Err(Refusal::Tangent("ssi_pair_boundary")),
        "not handed in: {bare:?}"
    );
    let enters = sheet(UNIT, (0.0, 1.5), &|i, j| {
        #[allow(clippy::cast_precision_loss)]
        let (x, y) = (0.5 * i as f64, 0.75 * j as f64);
        x - 0.5 * y
    });
    let s = read(&flat(), &enters, corner(low));
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
        Err(Refusal::Reach),
        "a branch from the corner: {s:?}"
    );
}

/// A tube about the `z` axis, `z ∈ [0, 1]`: its `u` row a quadratic
/// closed on itself at the corner `(1, −1)` where `closed`, or stopping
/// short of it, linear in `v` along the axis.
fn tube(closed: bool) -> Surface<f64> {
    let ku =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let last = if closed { (1.0, -1.0) } else { (1.0, -0.5) };
    let ring = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), last];
    let control = ring
        .iter()
        .flat_map(|&(x, y)| [p(x, y, 0.0), p(x, y, 1.0)])
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap(),
    ))
}

/// **A branch across a closed channel's seam is not cleared by reaching
/// it.** A spline sheet at `z = ½`, square to the tube's axis and
/// standing past it, meets it in one branch from one `u` side of the
/// tube's chart to the other. Where the `u` row closes on itself those
/// sides are a seam a face may lie across with no edge on it, so W1′
/// does not clear the branch and, placed nowhere, it refuses undecided;
/// where the row stops short they are the wall's boundary, and it
/// clears. Red against W1′ read of the kind (the seam's branch clears).
#[test]
fn a_branch_across_a_closed_channels_seam_is_not_cleared_by_reaching_it() {
    let level = sheet((-2.0, 2.0), (-2.0, 2.0), &|_, _| 0.5);
    for (closed, want) in [
        (true, Err(Refusal::Undecided)),
        (false, Ok(vec![Cleared::Reaches(Side::G)])),
    ] {
        let s = read(&level, &tube(closed), Known::default());
        assert_eq!(
            certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
            want,
            "closed {closed}: {s:?}"
        );
    }
}

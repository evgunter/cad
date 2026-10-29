//! **The parallel-tori arm's rows**: two tori whose axes are parallel
//! and apart. Each table row's count is checked against the section
//! traced on the first torus's chart, and every witness against the
//! traced component it lands on: one part per traced component, no two
//! parts on one.
//!
//! Every row names the mutant that turns it red.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use super::search_kit::{band, join, label, matched, near_cells};
use super::section_cert_rows::{IN, OUT, at, classify, on_both, p, tangent, torus};
use super::*;
use core::f64::consts::TAU;
use geom::Surface;
use geom_core::{Point3, Vec3};

/// A torus about `axis` through `center`.
fn torus_about(center: Point3<f64>, axis: Vec3<f64>, major: f64, minor: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Torus {
        center,
        axis,
        major_radius: major,
        minor_radius: minor,
        u_ref: axis.orthonormal_basis().0,
    }
}

fn parts(s: &Section<f64>) -> (&[Component<f64>], bool) {
    match s {
        Section::Components { parts, single } => (parts, *single),
        other => panic!("not classified: {other:?}"),
    }
}

/// The first torus's chart, `N × N` cells over `(u, v)`, both periodic.
const N: usize = 300;

/// The torus `(R, r)` about `z` at the origin, at chart cell `(i, j)`.
fn chart(big_r: f64, r: f64, i: usize, j: usize) -> Point3<f64> {
    let (u, w) = (TAU * i as f64 / N as f64, TAU * j as f64 / N as f64);
    let rho = big_r + r * w.cos();
    p(rho * u.cos(), rho * u.sin(), r * w.sin())
}

/// The partner's residual traced over the chart of the torus `(R, r)`
/// about `z`: each cell (`j·N + i`, `i` along `u`) whose corners change
/// sign, labelled by its component (cell adjacency, both directions
/// periodic).
fn trace(big_r: f64, r: f64, partner: &Surface<f64>) -> (Vec<Option<usize>>, usize) {
    let g: Vec<f64> = (0..N * N)
        .map(|k| geom_brep::implicit_residual(partner, chart(big_r, r, k % N, k / N)))
        .collect();
    let at = |i: usize, j: usize| g[(j % N) * N + i % N];
    let active: Vec<bool> = (0..N * N)
        .map(|k| {
            let (i, j) = (k % N, k / N);
            let s = [at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1)];
            s.iter().any(|&x| x > 0.0) && s.iter().any(|&x| x <= 0.0)
        })
        .collect();
    let mut parent: Vec<usize> = (0..N * N).collect();
    for k in (0..N * N).filter(|&k| active[k]) {
        let (i, j) = (k % N, k / N);
        for (di, dj) in [(1, 0), (0, 1), (1, 1), (N - 1, 1)] {
            let n = ((j + dj) % N) * N + (i + di) % N;
            if active[n] {
                join(&mut parent, k, n);
            }
        }
    }
    label(&mut parent, &active)
}

/// The traced components within two cells of `w`'s chart cell.
fn near(label: &[Option<usize>], big_r: f64, w: Point3<f64>) -> Vec<usize> {
    let rho = w.x.hypot(w.y);
    let cell = |t: f64| (t.rem_euclid(TAU) / TAU * N as f64) as isize;
    let (i, j) = (cell(w.y.atan2(w.x)), cell(w.z.atan2(rho - big_r)));
    near_cells(label, (N, N), (i, j), 2, true)
}

/// `(R, r)`, `(R₂, r₂)`, `(e, h)`, the traced count written down, and
/// what the row is.
type Row = ((f64, f64), (f64, f64), (f64, f64), usize, &'static str);

/// **Every class the pose takes, against the trace.** The first torus
/// about `z` at the origin, the partner about `z` through `(e, 0, h)`.
/// Mutants: listing one mirror copy of a loop clear of `Π` (count),
/// witnessing every point of `Π` instead of every other (count), the
/// parity started on a point's strand read off the wrong sheet (two
/// parts on one traced component), a lost meridian-circle pair (count),
/// a level end refused even where its node clears the triangle (not
/// classified).
#[test]
fn parallel_tori_every_class_against_the_trace() {
    let rows: [Row; 12] = [
        (
            (2.0, 0.5),
            (2.0, 0.5),
            (1.0, 0.3),
            2,
            "equal tori, one raised",
        ),
        (
            (2.0, 0.5),
            (1.0, 0.3),
            (1.0, 0.0),
            2,
            "cores internally tangent, the thinner tube nested there",
        ),
        (
            (2.0, 0.5),
            (1.0, 0.3),
            (1.4, 0.0),
            3,
            "the thinner tube entering the fatter across Π, leaving it either side",
        ),
        (
            (2.0, 0.5),
            (0.5, 0.2),
            (2.0, 0.0),
            2,
            "a small torus centred on the core circle",
        ),
        (
            (2.0, 0.5),
            (2.0, 0.3),
            (4.5, 0.1),
            1,
            "outer equators overlapping across Π",
        ),
        ((2.0, 0.5), (1.0, 0.3), (5.0, 0.0), 0, "apart"),
        ((2.0, 0.5), (2.0, 0.5), (1.0, 1.2), 0, "slabs apart"),
        (
            (2.0, 0.3),
            (1.5, 0.6),
            (0.8, 0.05),
            3,
            "the partner's tube spanning the first's heights",
        ),
        (
            (2.0, 0.5),
            (2.0, 0.5),
            (4.9, 0.0),
            1,
            "equal tubes level, their node beyond the triangle",
        ),
        (
            (2.0, 0.3),
            (1.0, 0.3),
            (0.5, 0.0),
            1,
            "equal tubes level, their node within the triangle's near edge",
        ),
        (
            (2.0, 0.5),
            (2.0, 0.25),
            (4.6, 0.25),
            1,
            "tops level, the partner owning the bottom, the node beyond",
        ),
        (
            (2.0, 0.5),
            (2.0, 0.5),
            (4.5, 1.0),
            0,
            "slabs level, the node beyond",
        ),
    ];
    for ((big_r, r), (big_r2, r2), (e, h), count, what) in rows {
        let f = torus(p(0.0, 0.0, 0.0), big_r, r);
        let g = torus(p(e, 0.0, h), big_r2, r2);
        let s = classify(&f, &g);
        let (ps, single) = parts(&s);
        let (label, traced) = trace(big_r, r, &g);
        assert_eq!(traced, count, "{what}: the trace counts the written count");
        assert_eq!(ps.len(), count, "{what}: one part per component: {s:?}");
        assert_eq!(single, count == 1, "{what}: single only on one component");
        let candidates: Vec<Vec<usize>> = ps
            .iter()
            .map(|c| {
                let w = c.witness.expect("every part carries a witness");
                on_both(w, &f, &g);
                near(&label, big_r, w)
            })
            .collect();
        assert!(
            matched(&candidates),
            "{what}: each part's witness on a traced component of its own: {candidates:?}"
        );
        assert_eq!(
            parts(&classify(&g, &f)).0.len(),
            count,
            "{what}: the swapped order counts the same"
        );
    }
}

/// **Level ends whose node lies inside the triangle refuse R-tan**: the
/// matching curve `W` has a node at `(R, R₂)` there (both tubes' sheets
/// turning at once), and inside `|ρ − ρ₂| < e < ρ + ρ₂` it is a pair of
/// points of both tori where both normals lie along the axes, a
/// tangency. Two tubes of one radius at one height (their tops level),
/// and two stacked so one's top meets the other's bottom (their slabs
/// level). The pose's critical gap decides each tangency before the arm
/// counts, so the refusal is named by it (`tt_critical_gap`). Mutant: the
/// gap read past (the arm's level rows name them).
#[test]
fn level_ends_with_the_node_inside_refuse_as_a_tangency() {
    for (h, row) in [(0.0, "tt_critical_gap"), (1.0, "tt_critical_gap")] {
        let s = classify(
            &torus(p(0.0, 0.0, 0.0), 2.0, 0.5),
            &torus(p(1.0, 0.0, h), 2.0, 0.5),
        );
        assert_eq!(tangent(&s), row, "h = {h}");
    }
}

/// Inside arcs of the matching curve of two level tubes of one radius
/// `r` (`h = 0`), read off its closed form: `W` is the two loops
/// `ρ − R = ±(ρ₂ − R₂) = r cos θ`, `y = r sin θ`, and each arc of a loop
/// inside `|ρ − ρ₂| < e < ρ + ρ₂` closes with its mirror into one
/// component, a loop wholly inside into two. An oracle the chart trace
/// cannot be near tangent tops, where its cells straddle both tubes.
fn level_tubes_count(big_r: f64, big_r2: f64, r: f64, e: f64) -> usize {
    const M: usize = 200_000;
    [1.0, -1.0]
        .into_iter()
        .map(|branch: f64| {
            let inside: Vec<bool> = (0..M)
                .map(|k| {
                    let w = r * (TAU * k as f64 / M as f64).cos();
                    let (rho, rho2) = (big_r + w, big_r2 + branch * w);
                    (rho - rho2).abs() < e && e < rho + rho2
                })
                .collect();
            if inside.iter().all(|&x| x) {
                2
            } else {
                (0..M)
                    .filter(|&k| inside[k] && !inside[(k + M - 1) % M])
                    .count()
            }
        })
        .sum()
}

/// **Level tubes of one radius, the node near either edge of the
/// triangle, against the matching curve's closed form.** Each row's
/// node `(R, R₂)` clears the triangle by about one neck
/// `√(2r·escalate)` or three, beyond `R + R₂` or within `|R − R₂|`.
/// Mutants: a neck doubled (the one-neck rows refuse), the parity
/// started on the wrong sheet (count).
#[test]
fn level_tubes_near_the_edge_against_the_closed_form() {
    let esc = band().escalate();
    for (big_r2, r, sense) in [(2.0_f64, 0.5_f64, 1.0_f64), (1.0, 0.3, -1.0)] {
        let neck = (2.0 * r * esc).sqrt();
        for necks in [1.05, 3.0] {
            let e = if sense > 0.0 {
                2.0 + big_r2 + necks * neck + 2.0 * esc
            } else {
                (2.0 - big_r2) - necks * neck - 2.0 * esc
            };
            let what = format!("R₂ {big_r2}, the node {necks} necks off the edge");
            let f = torus(p(0.0, 0.0, 0.0), 2.0, r);
            let g = torus(p(e, 0.0, 0.0), big_r2, r);
            let s = classify(&f, &g);
            let (ps, _) = parts(&s);
            let count = level_tubes_count(2.0, big_r2, r, e);
            assert_eq!(count, 1, "{what}: the closed form counts one");
            assert_eq!(ps.len(), count, "{what}: one part per component: {s:?}");
            for c in ps {
                on_both(c.witness.expect("a witness"), &f, &g);
            }
        }
    }
}

/// A pinched slabs row whose node clears the triangle by more than
/// the neck but less than both sheets' deviations together: the tori
/// meet, so the arm may not read no section. Mutant: the slabs neck
/// halved.
#[test]
fn level_slabs_within_the_sheets_sum_of_the_node_do_not_read_none() {
    let esc = band().escalate();
    for (big_r2, r, sense) in [(2.0_f64, 0.5_f64, 1.0_f64), (1.0, 0.3, -1.0)] {
        let neck = (2.0 * r * esc).sqrt();
        let h = 2.0 * r - 0.9 * esc;
        let e = if sense > 0.0 {
            2.0 + big_r2 + 1.05 * neck + 2.0 * esc
        } else {
            (2.0 - big_r2) - 1.1 * neck - 2.0 * esc
        };
        let s = classify(
            &torus(p(0.0, 0.0, 0.0), 2.0, r),
            &torus(p(e, 0.0, h), big_r2, r),
        );
        assert!(
            !matches!(&s, Section::Components { parts, .. } if parts.is_empty()),
            "R₂ {big_r2}: the tori meet, yet the arm reads none: {s:?}"
        );
    }
}

/// **A level end off true level by `0.9·escalate`, its node `0.8√escalate`
/// past the edge, refuses on its level row**: the true end's sheets stand
/// `≈ 0.95√escalate` from the node, inside the triangle, and the neck the
/// row proves (`√(2r·1.9·escalate) ≈ 1.38√escalate`) does not clear.
/// Where only the bottom stands near level, the pose's critical gap reads
/// a critical value in the band first, and the refusal is its
/// (`tt_critical_gap`). Three `√escalate` past, the node clears and the
/// pair classifies, its witnesses on both tori. Mutant: the level ends'
/// neck halved (the first row refuses on the order row instead, the
/// second classifies).
#[test]
fn a_level_end_off_level_refuses_within_its_neck_and_reads_past_it() {
    let esc = band().escalate();
    let s = esc.sqrt();
    for (what, r2, h, row) in [
        (
            "both ends",
            0.5,
            0.9 * esc,
            "section_torus_parallel_tori_top",
        ),
        (
            "the top",
            0.25,
            0.25 + 0.9 * esc,
            "section_torus_parallel_tori_top",
        ),
        ("the bottom", 0.25, -0.25 + 0.9 * esc, "tt_critical_gap"),
    ] {
        let f = torus(p(0.0, 0.0, 0.0), 2.0, 0.5);
        let near = torus(p(4.0 + 0.8 * s, 0.0, h), 2.0, r2);
        assert_eq!(
            tangent(&classify(&f, &near)),
            row,
            "{what}, within the neck"
        );
        let far = torus(p(4.0 + 3.0 * s, 0.0, h), 2.0, r2);
        let out = classify(&f, &far);
        let (ps, _) = parts(&out);
        assert_eq!(ps.len(), 1, "{what}, past the neck: {out:?}");
        on_both(ps[0].witness.expect("a witness"), &f, &far);
    }
}

/// The lily's stem torus, and its arch torus at the given offset `e`
/// from it along the line to the fork (`3.9` on the lily: the cores
/// internally tangent there) and height `h`, axes both `−y`.
fn lily(e: f64, h: f64) -> [Surface<f64>; 2] {
    let a = 22f64.to_radians();
    let toward = Vec3::new(a.cos(), 0.0, a.sin());
    let stem_c = p(-5.0, 0.0, 0.0);
    let down = Vec3::new(0.0, -1.0, 0.0);
    [
        torus_about(stem_c, down, 5.0, 0.060),
        torus_about(stem_c + toward * e + down * h, down, 1.1, 0.052),
    ]
}

/// **The lily's wall 1 pair: two loops, mirror images across the plane
/// of the axes, which is the weld plane.** Near the fork the arch's tube
/// (`0.052`) stands nested in the stem's (`0.060`); either side of it the
/// cores part and the arch's tube leaves through the stem's inner wall,
/// one loop each side, neither on the weld plane. Mutant: a loop of `W`
/// read wholly outside the triangle (no parts), or listed once.
#[test]
fn the_lilys_tube_walls_meet_in_two_mirror_loops_off_the_weld_plane() {
    let [stem, arch] = lily(3.9, 0.0);
    let fork = {
        let a = 22f64.to_radians();
        p(-5.0 + 5.0 * a.cos(), 0.0, 5.0 * a.sin())
    };
    let weld_normal = Vec3::new(-(22f64.to_radians().sin()), 0.0, 22f64.to_radians().cos());
    for (f, g, order) in [(&stem, &arch, "stem, arch"), (&arch, &stem, "arch, stem")] {
        let s = classify(f, g);
        let (ps, single) = parts(&s);
        assert_eq!(ps.len(), 2, "{order}: two loops: {s:?}");
        assert!(!single, "{order}: two components are never single");
        let side: Vec<f64> = ps
            .iter()
            .map(|c| {
                let w = c.witness.expect("witnessed");
                on_both(w, f, g);
                (w - fork).dot(weld_normal)
            })
            .collect();
        assert!(
            side[0] * side[1] < 0.0 && (side[0] + side[1]).abs() < 1e-9,
            "{order}: the witnesses are mirror images either side of the weld plane: {side:?}"
        );
        assert!(
            side.iter().all(|d| d.abs() > 0.1),
            "{order}: neither stands on the weld plane: {side:?}"
        );
    }
}

/// **Both sides of the lily's boundary.** The arch's offset pushed past
/// the tubes' difference (`0.008`) at the fork makes the meridian
/// circles cross on the weld plane: one loop through it, witnessed there.
/// Held within the band of that difference, the circles are tangent:
/// R-tan, never two loops. Raised clear of the stem's slab, the arch
/// meets nothing; within the band of touching it, R-tan. Mutants: the
/// nest row decided on the near side of the band (a count on the
/// tangency), the slab row dropped (parts for tori apart).
#[test]
fn the_lilys_pair_either_side_of_its_tangencies() {
    let [stem, arch] = lily(3.9 + 0.02, 0.0);
    let s = classify(&stem, &arch);
    let (ps, single) = parts(&s);
    let a = 22f64.to_radians();
    let weld_normal = Vec3::new(-a.sin(), 0.0, a.cos());
    let fork = p(-5.0 + 5.0 * a.cos(), 0.0, 5.0 * a.sin());
    let mut side: Vec<f64> = ps
        .iter()
        .map(|c| {
            let w = c.witness.unwrap();
            on_both(w, &stem, &arch);
            (w - fork).dot(weld_normal)
        })
        .collect();
    side.sort_by(f64::total_cmp);
    // The arch's tube now pokes out of the stem's outer wall across the
    // weld plane, and still leaves through its inner wall either side,
    // where the cores cross.
    assert_eq!(ps.len(), 3, "past the difference: three loops: {s:?}");
    assert!(!single, "three loops are not single");
    assert!(
        side[0] < -0.1 && side[1].abs() < 1e-9 && side[2] > 0.1,
        "one loop witnessed on the weld plane, a mirror pair off it: {side:?}"
    );
    let [stem, arch] = lily(3.9 + 0.008, 0.0);
    assert_eq!(
        tangent(&classify(&stem, &arch)),
        "tt_critical_gap",
        "the meridian circles tangent at the fork refuse R-tan"
    );
    let [stem, arch] = lily(3.9, 0.2);
    assert_eq!(parts(&classify(&stem, &arch)).0.len(), 0, "slabs apart");
    let [stem, arch] = lily(3.9, 0.112);
    assert_eq!(
        tangent(&classify(&stem, &arch)),
        "tt_critical_gap",
        "slabs touching refuse R-tan"
    );
}

/// **The faces' windows decide the lily's pair, not the carriers.**
/// With the stem's window ending at the weld plane and the arch's
/// starting there, each loop's witness stands outside one window and
/// the pair clears; with windows reaching across the weld plane over a
/// loop and no event, the loop is interior to both and the pair refuses
/// R-loop, never clear. Mutant: a classification that lists no loops for
/// this pose (it would clear whatever the windows reach).
#[test]
fn the_lilys_pair_clears_only_on_windows_that_miss_its_loops() {
    let [stem, arch] = lily(3.9, 0.0);
    let s = classify(&stem, &arch);
    let a = 22f64.to_radians();
    let weld_normal = Vec3::new(-a.sin(), 0.0, a.cos());
    let fork = p(-5.0 + 5.0 * a.cos(), 0.0, 5.0 * a.sin());
    // The stem's window ends at the weld plane, the arch's starts there.
    let windows = |w: Point3<f64>, _| {
        let after = (w - fork).dot(weld_normal) > 0.0;
        [if after { OUT } else { IN }, if after { IN } else { OUT }]
    };
    let mut cleared = certify(&s, false, band(), |_| Sheet::Not, windows)
        .expect("windows parted by the weld plane clear both loops");
    cleared.sort_by_key(|c| format!("{c:?}"));
    assert_eq!(
        cleared,
        [Cleared::Out(Side::F), Cleared::Out(Side::G)],
        "one loop outside the stem's window, the other outside the arch's"
    );
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, at(IN, IN)),
        Err(Refusal::Loop),
        "windows reaching over a loop with no event refuse R-loop"
    );
}

/// **A tilt within the band routes parallel, and the gap reads the
/// operands as they stand.** The partner's near tube circle stands `n ε`
/// past internally tangent to the torus's outer one
/// (`d − |r − r₂| = n ε`). At `30 ε`, exactly parallel or tilted by
/// `ε / 100` (which the lever over the partner's ball reads parallel), the
/// pair has one loop; at `3 ε` the critical gap is in the band and both
/// refuse R-tan on it. Both offsets are in band units, so the rows read
/// alike at every ε. Mutants: the tilt levered by a reach the value does
/// not hold (the tilted pose refuses on reach), the gap read bare of the
/// band (the `3 ε` pose answers).
#[test]
fn a_tilt_within_the_band_routes_parallel_and_the_gap_reads_the_operands() {
    let eps = geom_core::Tol::witness().eps();
    let f = torus(p(0.0, 0.0, 0.0), 2.0, 0.5);
    let partner = |n: f64, t: f64| Surface::Torus {
        center: p(3.2 + n * eps, 0.0, 0.0),
        axis: Vec3::new(t.sin(), 0.0, t.cos()),
        major_radius: 1.0,
        minor_radius: 0.3,
        u_ref: Vec3::new(t.cos(), 0.0, -t.sin()),
    };
    for t in [0.0, eps / 100.0] {
        assert_eq!(
            parts(&classify(&f, &partner(30.0, t))).0.len(),
            1,
            "tilted by {t}: one loop"
        );
        assert!(
            tangent(&classify(&f, &partner(3.0, t))).starts_with("tt_critical_gap"),
            "tilted by {t}: the gap in the band"
        );
    }
}

/// **With an event on the pair, the lily's two loops refuse R-undec
/// unless each witness is placed out.** A multi-part section never takes
/// W4, so an event clears nothing: windows over both loops refuse
/// undecided (not R-loop, which needs the pair's silence), and windows
/// parted by the weld plane still clear each loop by its witness.
/// Mutant: `single` set on a two-part section (W4 would clear both).
#[test]
fn an_evented_lily_pair_clears_only_by_its_witnesses() {
    let [stem, arch] = lily(3.9, 0.0);
    let s = classify(&stem, &arch);
    assert_eq!(
        certify(&s, true, band(), |_| Sheet::Not, at(IN, IN)),
        Err(Refusal::Undecided),
        "evented, both witnesses inside both faces: R-undec"
    );
    let a = 22f64.to_radians();
    let weld_normal = Vec3::new(-a.sin(), 0.0, a.cos());
    let fork = p(-5.0 + 5.0 * a.cos(), 0.0, 5.0 * a.sin());
    let windows = |w: Point3<f64>, _| {
        let after = (w - fork).dot(weld_normal) > 0.0;
        [if after { OUT } else { IN }, if after { IN } else { OUT }]
    };
    let mut cleared =
        certify(&s, true, band(), |_| Sheet::Not, windows).expect("each witness out of one window");
    cleared.sort_by_key(|c| format!("{c:?}"));
    assert_eq!(cleared, [Cleared::Out(Side::F), Cleared::Out(Side::G)]);
}

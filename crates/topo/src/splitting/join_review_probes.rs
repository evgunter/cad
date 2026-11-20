//! Reviewer probes for `split_section_area` (PR 7146's review): a lens
//! of a far-centred circle, read through `certify_section_area`, its
//! margin printed for an external oracle. Prints, asserts nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]
use super::*;
use geom_core::Vec3;

fn sweep_for(band: Band, tol: Tol) -> Sweep<f64> {
    let plane = crate::test_support_fixtures::split_plane(Point3::origin(), Vec3::unit_z(), tol);
    Sweep {
        ends: Vec::new(),
        partner: SecondaryMap::new(),
        joiner: ChordJoiner::new(band),
        completed: Vec::new(),
        above_set: SecondaryMap::new(),
        plane,
        band,
        section: SectionCtx {
            origin: plane.origin,
            normal: plane.normal,
            plane_key: None,
            branch: None,
        },
    }
}

fn lens(carrier: geom::Curve3<f64>, span: (f64, f64), tol: Tol) -> Option<Body<f64>> {
    use geom_brep::EdgeCurveSpec;
    let (p0, p1) = (carrier.eval(span.0), carrier.eval(span.1));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).ok()?;
    let arc = body
        .mev(
            crate::euler::MevSite::Lone { r#loop: seed.r#loop },
            p1,
            EdgeCurveSpec::arc_of_circle(carrier, span.0, span.1)?,
            tol,
        )
        .ok()?;
    body.mef(
        crate::euler::MefSite::Chords { he1: arc.he_minus, he2: arc.he_plus },
        EdgeCurveSpec::line_between(p1, p0),
        crate::euler::FaceSurface::Inherit,
        tol,
    )
    .ok()?;
    Some(body)
}

/// A tiny deterministic generator (xorshift), so both trees read one pose list.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[test]
#[ignore = "reviewer probe: prints margins for an external oracle"]
fn review_lens_margins() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let real = Band::linear(tol).unwrap();
    let wide = Band::new(1.0e-300, 1.0e300).unwrap();
    let (s_real, s_wide) = (sweep_for(real, tol), sweep_for(wide, tol));
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let fars = [1.0e2, 3.0e2, 1.0e3, 3.0e3, 1.0e4, 1.0e5, 1.0e6];
    let n = std::env::var("REVIEW_LENS_N").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    for far in fars {
        for i in 0..n {
            let width = 12.0 * eps * rng.next();
            let psi = 6.283_185_307_179_586 * rng.next();
            // arcs adjacent to the seam and far from it, either side
            let t0 = [0.0, 1.7, -2.5, 3.0, 0.3][i % 5] + 0.01 * rng.next();
            let frac = rng.next();
            let turn = (12.0 * width / far).sqrt();
            let (ta, tb) = if i % 5 == 0 { (t0 - frac * turn, t0 + (1.0 - frac) * turn) } else { (t0, t0 + turn) };
            let q = Point3::new(10.0 * rng.next() - 5.0, 10.0 * rng.next() - 5.0, 0.0);
            let u = Vec3::new(psi.cos(), psi.sin(), 0.0);
            let v = Vec3::new(-psi.sin(), psi.cos(), 0.0);
            let tm = 0.5 * (ta + tb);
            let centre = q - (u * tm.cos() + v * tm.sin()) * far;
            let carrier = geom::Curve3::circle(centre, Vec3::unit_z(), far, u);
            let Some(body) = lens(carrier, (ta, tb), tol) else {
                println!("LENS eps={eps:e} far={far:e} i={i} BUILDFAIL");
                continue;
            };
            for (k, (lp, data)) in body.loops().enumerate() {
                let verdict = s_real.certify_section_area(&body, data.face, lp);
                let value = s_wide.certify_section_area(&body, data.face, lp);
                let v = match verdict {
                    Ok(()) => "POS".to_string(),
                    Err(JoinError::DegenerateSection { .. }) => "ZERO".to_string(),
                    Err(JoinError::Escalated { .. }) => "ESC".to_string(),
                    Err(e) => format!("OTHER:{e:?}"),
                };
                println!(
                    "LENS eps={eps:e} far={far:e} i={i} k={k} ta={ta:e} tb={tb:e} verdict={v} wide={value:?}"
                );
            }
        }
    }
}

/// Large arcs (past a half turn, near a whole turn) at radius 1, both
/// loops: the margin's sign and size against a closed form.
#[test]
#[ignore = "reviewer probe"]
fn review_big_arcs() {
    let tol = Tol::witness();
    let wide = Band::new(1.0e-300, 1.0e300).unwrap();
    let s = sweep_for(wide, tol);
    for (r, ta, tb) in [(1.0, 0.0, 3.5), (1.0, 1.0, 7.0), (1.0, -3.0, 3.0), (2.5, 0.4, 6.2), (1.0, 0.0, 1.0e-3)] {
        let carrier = geom::Curve3::circle(Point3::new(0.3, -0.2, 0.0), Vec3::unit_z(), r, Vec3::new(0.6, 0.8, 0.0));
        let Some(body) = lens(carrier, (ta, tb), tol) else {
            println!("BIG r={r} ta={ta} tb={tb} BUILDFAIL");
            continue;
        };
        for (k, (lp, data)) in body.loops().enumerate() {
            let got = s.certify_section_area(&body, data.face, lp);
            println!("BIG r={r} ta={ta:e} tb={tb:e} k={k} wide={got:?}");
        }
    }
}

/// The re-valuation pin over `chord_bulge` at `Sym<f64>`: the DAG the
/// recording scalar keeps for `x − sin x` re-valued at the build's own
/// point should be the build's own bits (`revalue_pin`'s first assert).
#[test]
fn review_revalue_pin_over_chord_bulge() {
    let found = crate::revalue_pin::revalued([0.5, 0.5 + 1.0e-3], |_, p| {
        let span = p * <crate::revalue_pin::S as geom_core::Real>::from_f64(1.0e-3);
        vec![geom_brep::props::chord_bulge(span)]
    });
    assert_eq!(found.laundered, 0, "{found:?}");
}

/// A triangle with one arc side: arc `p0 → p1`, then `p1 → p2 → p0`.
fn arc_triangle(carrier: geom::Curve3<f64>, span: (f64, f64), p2: Point3<f64>, tol: Tol) -> Option<Body<f64>> {
    use geom_brep::EdgeCurveSpec;
    let (p0, p1) = (carrier.eval(span.0), carrier.eval(span.1));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).ok()?;
    let arc = body
        .mev(crate::euler::MevSite::Lone { r#loop: seed.r#loop }, p1,
             EdgeCurveSpec::arc_of_circle(carrier, span.0, span.1)?, tol)
        .ok()?;
    let side = body
        .mev(crate::euler::MevSite::Fan { he1: arc.he_minus, he2: arc.he_minus }, p2, EdgeCurveSpec::line_between(p1, p2), tol)
        .ok()?;
    body.mef(
        crate::euler::MefSite::Chords { he1: side.he_minus, he2: arc.he_plus },
        EdgeCurveSpec::line_between(p2, p0),
        crate::euler::FaceSurface::Inherit,
        tol,
    )
    .ok()?;
    Some(body)
}

#[test]
#[ignore = "reviewer probe"]
fn review_arc_triangles() {
    let tol = Tol::witness();
    let wide = Band::new(1.0e-300, 1.0e300).unwrap();
    let s = sweep_for(wide, tol);
    // (radius, t0, t1, apex): apex on either side of the chord
    for (r, ta, tb, apex) in [
        (1.0, 0.0, 1.0, (0.0, 0.0)), (1.0, 0.0, 1.0, (3.0, 3.0)), (1.0, 0.0, 4.0, (0.0, 0.0)),
        (1.0, 0.0, 4.0, (-3.0, 3.0)), (1.0e4, 0.0, 1.0e-4, (1.0e4, 0.5)), (1.0e4, 0.0, 1.0e-4, (1.0e4 - 1.0, 0.5)),
    ] {
        let carrier = geom::Curve3::circle(Point3::origin(), Vec3::unit_z(), r, Vec3::unit_x());
        let p2 = Point3::new(apex.0, apex.1, 0.0);
        let Some(body) = arc_triangle(carrier, (ta, tb), p2, tol) else {
            println!("TRI r={r} ta={ta} tb={tb} apex={apex:?} BUILDFAIL");
            continue;
        };
        for (k, (lp, data)) in body.loops().enumerate() {
            let got = s.certify_section_area(&body, data.face, lp);
            println!("TRI r={r:e} ta={ta:e} tb={tb:e} apex={apex:?} k={k} wide={got:?}");
        }
    }
}

/// **A far-centred arc's sliver reads its own width to a part in a
/// million** (review of PR 7146). Lenses of circles of radius `far`,
/// at seams turned every way, arcs adjacent to the seam and far from
/// it, widths across the band; each loop's margin read through a band
/// too wide to decide, so the reading itself is compared. Oracle: the
/// two-term brackets of `Δ − sin Δ` and of the chord, which hold the
/// true `2A/P` to a part in `Δ²`. Red on the merge base's formula
/// (errors to 1e7 relative) and on the new formula with `chord_bulge`
/// read as `Δ − sin Δ` (errors to 60%).
#[test]
fn a_far_centred_arcs_sliver_reads_its_own_width_to_a_part_in_a_million() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let wide = Band::new(1.0e-300, 1.0e300).unwrap();
    let s = sweep_for(wide, tol);
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    // Past these an arc's own certification refuses it (`carrier_matches_mapped_source`).
    let fars: &[f64] = if eps < 1.0e-9 { &[1.0e2, 3.0e2, 1.0e3] } else { &[1.0e2, 1.0e3, 1.0e4, 1.0e5, 1.0e6] };
    for &far in fars {
        for i in 0..40 {
            let width = eps * (0.1 + 11.9 * rng.next());
            let psi = 6.283_185_307_179_586 * rng.next();
            let t0 = [0.0, 1.7, -2.5, 3.0, 0.3][i % 5] + 0.01 * rng.next();
            let frac = rng.next();
            let turn = (12.0 * width / far).sqrt();
            let (ta, tb) = if i % 5 == 0 { (t0 - frac * turn, t0 + (1.0 - frac) * turn) } else { (t0, t0 + turn) };
            let q = Point3::new(10.0 * rng.next() - 5.0, 10.0 * rng.next() - 5.0, 0.0);
            let u = Vec3::new(psi.cos(), psi.sin(), 0.0);
            let v = Vec3::new(-psi.sin(), psi.cos(), 0.0);
            let tm = 0.5 * (ta + tb);
            let centre = q - (u * tm.cos() + v * tm.sin()) * far;
            let carrier = geom::Curve3::circle(centre, Vec3::unit_z(), far, u);
            let body = lens(carrier, (ta, tb), tol).unwrap_or_else(|| panic!("far {far:e}, pose {i}: the lens builds"));
            let dt = tb - ta;
            let (d2, lead) = (dt * dt, far * far * dt * dt * dt / 6.0);
            let (area_lo, area_hi) = (lead * (1.0 - d2 / 20.0), lead);
            let (per_lo, per_hi) = (far * dt * (2.0 - d2 / 24.0), 2.0 * far * dt);
            let (lo, hi) = (area_lo / per_hi, area_hi / per_lo);
            for (lp, data) in body.loops() {
                let got = s.certify_section_area(&body, data.face, lp);
                let Err(JoinError::Escalated { diag, .. }) = got else {
                    panic!("far {far:e}, pose {i}: a band of 1e-300 to 1e300 decides nothing, read {got:?}");
                };
                let text = format!("{:?}", diag.margin);
                let read: f64 = text
                    .split(['(', ','])
                    .nth(2)
                    .and_then(|x| x.trim().parse().ok())
                    .unwrap_or_else(|| panic!("a point margin's value in {text}"));
                assert!(
                    lo * (1.0 - 1.0e-6) <= read && read <= hi * (1.0 + 1.0e-6),
                    "far {far:e}, pose {i}, span [{ta:e}, {tb:e}]: the lens's width is in \
                     [{lo:e}, {hi:e}] and the site read {read:e}"
                );
            }
        }
    }
}

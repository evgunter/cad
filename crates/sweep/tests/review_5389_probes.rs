//! Review probes for PR 5389 (deferred covered circles).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::revolved_about_y;
use topo::BooleanResult;

fn tol() -> Tol {
    Tol::witness()
}
fn band() -> Band {
    Band::linear(tol()).unwrap()
}
fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).unwrap()
}
fn place(b: &topo::Body<f64>, a: Affine3<f64>) -> topo::Body<f64> {
    topo::transform_rigid(b, &geom_core::test_support::rigid(&a, tol()), tol()).unwrap()
}
fn about_y(
    (inner, outer): (f64, f64),
    (lo, hi): (f64, f64),
    turn: Option<f64>,
    spin: f64,
    x: f64,
) -> topo::Body<f64> {
    let profile = vec![
        (Point2::new(inner, lo), 0.0),
        (Point2::new(outer, lo), 0.0),
        (Point2::new(outer, hi), 0.0),
        (Point2::new(inner, hi), 0.0),
    ];
    let rev = turn.map_or(sweep::Revolution::Full, sweep::Revolution::Partial);
    let body = revolved_about_y(profile, rev, tol());
    place(
        &body,
        Affine3::translation(Vec3::new(x, 0.0, 0.0))
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), spin),
    )
}
/// Spherical cap of a radius-`r` sphere about the origin, `{y >= c}`.
fn cap_about_y(r: f64, c: f64) -> topo::Body<f64> {
    let s = (r * r - c * c).sqrt();
    let delta = FRAC_PI_2 - (c / r).asin();
    let bulge = (delta / 4.0).tan();
    revolved_about_y(
        vec![
            (Point2::new(0.0, c), 0.0),
            (Point2::new(s, c), bulge),
            (Point2::new(0.0, r), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}
fn at(body: &topo::Body<f64>, v: topo::VertexKey) -> String {
    let (_, p) = body.vertex_points().find(|(k, _)| *k == v).unwrap();
    format!("({:.6}, {:.6}, {:.6})", p.x, p.y, p.z)
}
fn records(r: &topo::BooleanBody<f64>) -> Vec<String> {
    let body: &topo::Body<f64> = &r.body;
    let cell = |c: topo::Cell| match c {
        topo::Cell::Vertex(v) => format!("vertex {}", at(body, v)),
        topo::Cell::Edge(e) => {
            let mut ends: Vec<String> = body
                .half_edges()
                .filter(|(_, h)| h.edge == e)
                .map(|(_, h)| at(body, h.start))
                .collect();
            ends.sort();
            format!("edge {ends:?}")
        }
        topo::Cell::Face(f) => {
            let face = body.get_face(f).unwrap();
            format!("face {:?}", body.get_surface(face.surface).unwrap().kind())
        }
    };
    let mut rows: Vec<String> = r
        .contacts
        .cell_pairs()
        .map(|(x, y)| {
            let mut pair = [cell(x), cell(y)];
            pair.sort();
            pair.join(" / ")
        })
        .collect();
    rows.sort();
    rows
}

/// Every op in both orders, reported (not asserted).
fn report(pose: &str, x: &topo::Body<f64>, y: &topo::Body<f64>, vx: f64, vy: f64) -> Vec<String> {
    let (x, y) = (at_rest(x.clone()), at_rest(y.clone()));
    let mut out = Vec::new();
    for (order, p, q, vp) in [("x,y", &x, &y, vx), ("y,x", &y, &x, vy)] {
        let u = match topo::union(p, q, tol()) {
            Ok(BooleanResult::Body(u)) => {
                let v = topo::mass_properties(&u.body, tol()).unwrap().volume;
                let f = topo::validate_pseudomanifold(&u.body, &u.contacts, tol())
                    .err()
                    .unwrap_or_default();
                format!(
                    "Ok solids={} vol={v:.9} (want {:.9}) kind={:?} recs={:?} 3'={:?}",
                    u.body.solids().count(),
                    vx + vy,
                    u.kind,
                    records(&u),
                    f.iter().map(|e| format!("{e:?}").chars().take(600).collect::<String>()).collect::<Vec<_>>()
                )
            }
            Ok(BooleanResult::Empty) => "Empty".into(),
            Err(e) => format!("Err {}", format!("{e:?}").chars().take(400).collect::<String>()),
        };
        let d = match topo::subtract(p, q, tol()) {
            Ok(BooleanResult::Body(d)) => format!(
                "Ok {:?} vol={:.9} (want {vp:.9})",
                d.kind,
                topo::mass_properties(&d.body, tol()).unwrap().volume
            ),
            Ok(BooleanResult::Empty) => "Empty".into(),
            Err(e) => format!("Err {}", format!("{e:?}").chars().take(400).collect::<String>()),
        };
        let m = match topo::intersect(p, q, tol()) {
            Ok(BooleanResult::Body(m)) => format!("Ok {:?}", m.kind),
            Ok(BooleanResult::Empty) => "Empty".into(),
            Err(e) => format!("Err {}", format!("{e:?}").chars().take(400).collect::<String>()),
        };
        let line = format!("[{pose}] {order}: U {u}\n    D {d}\n    I {m}");
        eprintln!("{line}");
        out.push(line);
    }
    out
}

/// A spherical cap (sphere r=1) tilted `alpha` in the xy plane, inside a
/// y-bore of radius `1 + gap`; the cap's rim circle meets the bore's
/// tangency circle twice. The bore's seam is spun to `seam_at` touch
/// (Some(0|1)) or away (None).
fn cap_in_bore(alpha: f64, c: f64, seam_at: Option<usize>, cap_spin: f64) -> (topo::Body<f64>, topo::Body<f64>, f64, f64) {
    let gap = 0.5 * band().zero();
    let r = 1.0;
    let cap0 = cap_about_y(r, c);
    let cap = place(
        &cap0,
        Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), -(FRAC_PI_2 - alpha))
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), cap_spin),
    );
    // touches: y = 0, x = c / cos(alpha), z = ±sqrt(1 - x^2)
    let xt = c / alpha.cos();
    let zt = (1.0 - xt * xt).sqrt();
    let theta = match seam_at {
        Some(0) => (-zt).atan2(xt),
        Some(_) => (zt).atan2(xt),
        None => PI,
    };
    let bore = about_y((r + gap, 1.5), (-1.5, 1.5), None, theta, 0.0);
    let h = r - c;
    let vcap = PI * h * h * (3.0 * r - h) / 3.0;
    let vbore = PI * (1.5 * 1.5 - (r + gap) * (r + gap)) * 3.0;
    (bore, cap, vbore, vcap)
}

#[test]
fn probe_cap_in_bore() {
    for (name, alpha, seam, spin) in [
        ("seam at touch 0, a=0.3", 0.3, Some(0), PI),
        ("seam at touch 1, a=0.3", 0.3, Some(1), PI),
        ("seam away, a=0.3", 0.3, None, PI),
        ("seam at touch 0, a=0.6", 0.6, Some(0), PI),
        ("seam at touch 0, a=0.3, cap spin 0", 0.3, Some(0), 0.0),
    ] {
        let (b, c, vb, vc) = cap_in_bore(alpha, 0.5, seam, spin);
        report(name, &b, &c, vb, vc);
    }
}

/// A puck (disc radius `rho`, thickness 0.3, top face `y = 0`, axis y,
/// spun `puck_spin` about y) with a rod (radius `r`) lying on it along
/// x, over `x in [lo, hi]`, its axis at `y = r + gap`, spun `rod_spin`
/// about its own axis (0 puts its seam on the contact ruling). The
/// puck's rim circle meets the rod's carrier at `(±rho, 0, 0)`.
fn rod_on_puck(
    rho: f64,
    r: f64,
    (lo, hi): (f64, f64),
    puck_spin: f64,
    rod_spin: f64,
    gap_k: f64,
) -> (topo::Body<f64>, topo::Body<f64>, f64, f64) {
    let gap = gap_k * band().zero();
    let puck = about_y((0.0, rho), (-0.3, 0.0), None, puck_spin, 0.0);
    let rod0 = about_y((0.0, r), (lo, hi), None, rod_spin, 0.0);
    let rod = place(
        &rod0,
        Affine3::translation(Vec3::new(0.0, r + gap, 0.0))
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), -FRAC_PI_2),
    );
    (puck, rod, PI * rho * rho * 0.3, PI * r * r * (hi - lo))
}

#[test]
fn probe_rod_on_puck() {
    for (name, span, ps, rs) in [
        ("seam on ruling, rod past both touches", (-1.5, 1.5), 1.0, 0.0),
        ("seam off ruling, rod past both touches", (-1.5, 1.5), 1.0, 0.7),
        ("seam on ruling, rod ends at T2", (-1.5, 1.0), 1.0, 0.0),
        ("seam off ruling, rod ends at T2", (-1.5, 1.0), 1.0, 0.7),
        ("seam on ruling, rod ends inside disc", (-1.5, 0.5), 1.0, 0.0),
        ("seam off ruling, rod inside disc", (-0.5, 0.5), 1.0, 0.7),
        ("seam on ruling, puck vertex at T2", (-1.5, 1.5), 0.0, 0.0),
    ] {
        let (p, r, vp, vr) = rod_on_puck(1.0, 0.25, span, ps, rs, 0.5);
        report(name, &p, &r, vp, vr);
    }
}

/// As `rod_on_puck`, with the rod's axis moved to `z = z0`, so the
/// contact ruling is the chord `z = z0` and both touches
/// `(±sqrt(1 - z0^2), 0, z0)` lie in the puck rim's `z > 0` half
/// (puck spin 0 puts the rim's vertices at `(±1, 0, 0)`).
fn rod_on_puck_off_centre(
    z0: f64,
    (lo, hi): (f64, f64),
    puck_spin: f64,
    rod_spin: f64,
) -> (topo::Body<f64>, topo::Body<f64>, f64, f64) {
    let (puck, rod, vp, vr) = rod_on_puck(1.0, 0.25, (lo, hi), puck_spin, rod_spin, 0.5);
    let rod = place(&rod, Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    (puck, rod, vp, vr)
}

#[test]
fn probe_rod_on_puck_off_centre() {
    let z0 = 0.5;
    let t = (1.0f64 - z0 * z0).sqrt();
    for (name, span, ps, rs) in [
        ("off-centre, seam on ruling, past both", (-1.5, 1.5), 0.0, 0.0),
        ("off-centre, seam off ruling, past both", (-1.5, 1.5), 0.0, 0.7),
        ("off-centre, seam off, rod ends at T2", (-1.5, t), 0.0, 0.7),
        ("off-centre, seam off, rod ends at T2, puck spun 0.2", (-1.5, t), 0.2, 0.7),
        ("off-centre, seam off, rod ends at T2, puck spun -0.2", (-1.5, t), -0.2, 0.7),
        ("off-centre, seam on, rod ends at T2", (-1.5, t), 0.0, 0.0),
        ("off-centre, seam off, rod from T1 to T2", (-t, t), 0.0, 0.7),
        ("off-centre, seam off, rod ends at T1", (-t, 1.5), 0.0, 0.7),
    ] {
        let (p, r, vp, vr) = rod_on_puck_off_centre(z0, span, ps, rs);
        report(name, &p, &r, vp, vr);
    }
}

/// One op's outcome as a comparable string (records by position).
fn outcome(r: Result<BooleanResult<f64>, topo::BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Body(b)) => format!(
            "Ok {:?} solids={} vol={:.9} recs={:?}",
            b.kind,
            b.body.solids().count(),
            topo::mass_properties(&b.body, tol()).unwrap().volume,
            records(&b)
        ),
        Ok(BooleanResult::Empty) => "Empty".into(),
        Err(e) => format!("Err {:?}", e.kind()),
    }
}

/// ∪ and ∩ compared across orders (∖ is not symmetric); returns
/// (agree, summary).
fn symmetric(x: &topo::Body<f64>, y: &topo::Body<f64>) -> (bool, String) {
    let (x, y) = (at_rest(x.clone()), at_rest(y.clone()));
    let u = [outcome(topo::union(&x, &y, tol())), outcome(topo::union(&y, &x, tol()))];
    let i = [outcome(topo::intersect(&x, &y, tol())), outcome(topo::intersect(&y, &x, tol()))];
    let short = |s: &str| s.chars().take(70).collect::<String>();
    (
        u[0] == u[1] && i[0] == i[1],
        format!("U[{} | {}] I[{} | {}]", short(&u[0]), short(&u[1]), short(&i[0]), short(&i[1])),
    )
}

#[test]
fn probe_two_tubes_near_variants() {
    let z = band().zero();
    let mut bad = Vec::new();
    // across the band
    for k in [-5.0, -1.5, -0.9, -0.5, 0.0, 0.5, 0.9, 1.5, 5.0, 50.0] {
        let x = about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0);
        let y = about_y((0.3, 1.0), (0.2, 0.8), None, 0.0, 2.0 + k * z);
        let (ok, s) = symmetric(&x, &y);
        eprintln!("[gap {k}eps] agree={ok} {s}");
        if !ok { bad.push(format!("gap {k}")); }
    }
    // first tube's seam angle (0 = on the tangent line), second spun too
    for (s1, s2) in [(0.1, 0.0), (1e-7, 0.0), (0.0, PI), (0.0, 0.3), (PI, PI), (0.3, 0.3), (2.0, 0.0)] {
        let x = about_y((0.3, 1.0), (0.0, 1.0), None, s1, 0.0);
        let y = about_y((0.3, 1.0), (0.2, 0.8), None, s2, 2.0 + 0.5 * z);
        let (ok, s) = symmetric(&x, &y);
        eprintln!("[seams {s1},{s2}] agree={ok} {s}");
        if !ok { bad.push(format!("seams {s1},{s2}")); }
    }
    // both turned about z, and the pair with equal heights
    for t in [0.2, 1.0, 2.5] {
        let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), t);
        let x = place(&about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0), rot);
        let y = place(&about_y((0.3, 1.0), (0.2, 0.8), None, 0.0, 2.0 + 0.5 * z), rot);
        let (ok, s) = symmetric(&x, &y);
        eprintln!("[turned {t}] agree={ok} {s}");
        if !ok { bad.push(format!("turned {t}")); }
    }
    {
        let x = about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0);
        let y = about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 2.0 + 0.5 * z);
        let (ok, s) = symmetric(&x, &y);
        eprintln!("[equal heights] agree={ok} {s}");
        if !ok { bad.push("equal heights".into()); }
    }
    // rod in the bore at other turns: rod r=0.2 about (0.1 - 0.5eps), spin
    for spin in [-0.25 * FRAC_PI_2, -0.5 * FRAC_PI_2, -0.75 * FRAC_PI_2, PI, 0.0] {
        let x = about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0);
        let y = about_y((0.0, 0.2), (0.2, 0.8), Some(FRAC_PI_2), spin, 0.1 - 0.5 * z);
        let (ok, s) = symmetric(&x, &y);
        eprintln!("[quarter rod spin {spin}] agree={ok} {s}");
        if !ok { bad.push(format!("rod spin {spin}")); }
        let y = about_y((0.0, 0.2), (0.2, 0.8), None, spin, 0.1 - 0.5 * z);
        let (ok, s) = symmetric(&x, &y);
        eprintln!("[full rod spin {spin}] agree={ok} {s}");
        if !ok { bad.push(format!("full rod spin {spin}")); }
    }
    eprintln!("DISAGREE: {bad:?}");
}

#[test]
fn probe_rod_on_puck_offsets() {
    for z0 in [0.0, 1e-3, 0.1, 0.3] {
        let t = (1.0f64 - z0 * z0).sqrt();
        for (name, span, ps, rs) in [
            ("seam on, past both", (-1.5, 1.5), 1.0, 0.0),
            ("seam on, past both, puck 0", (-1.5, 1.5), 0.0, 0.0),
            ("seam off, ends at T2", (-1.5, t), 1.0, 0.7),
            ("seam off, ends at T2, puck 0", (-1.5, t), 0.0, 0.7),
        ] {
            let (p, r, vp, vr) = rod_on_puck_off_centre(z0, span, ps, rs);
            report(&format!("z0={z0} {name}"), &p, &r, vp, vr);
        }
    }
}

/// A three-quarter puck (revolve 1.5π, spun `puck_spin`) under the rod.
#[test]
fn probe_rod_on_three_quarter_puck() {
    for ps in [-0.25 * PI, 0.25 * PI, 0.75 * PI, -0.75 * PI] {
        for (name, span, rs) in [
            ("seam on, past both", (-1.5, 1.5), 0.0),
            ("seam off, past both", (-1.5, 1.5), 0.7),
            ("seam off, ends at T2", (-1.5, 1.0), 0.7),
            ("seam off, starts at T1", (-1.0, 1.5), 0.7),
            ("seam on, ends at T2", (-1.5, 1.0), 0.0),
        ] {
            let gap = 0.5 * band().zero();
            let puck = about_y((0.0, 1.0), (-0.3, 0.0), Some(1.5 * PI), ps, 0.0);
            let r = 0.25;
            let rod0 = about_y((0.0, r), span, None, rs, 0.0);
            let rod = place(
                &rod0,
                Affine3::translation(Vec3::new(0.0, r + gap, 0.0))
                    * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), -FRAC_PI_2),
            );
            report(
                &format!("3/4 puck spin {ps:.3} {name}"),
                &puck,
                &rod,
                PI * 0.3 * 0.75,
                PI * r * r * (span.1 - span.0),
            );
        }
    }
}

fn rod_z_review(r: f64, z0: f64, len: f64) -> topo::Body<f64> {
    let t = tol();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, t).unwrap();
    let plane = profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let p = profile::Profile::new(plane, vec![lp.into()]).validate(t).unwrap();
    sweep::extrude(
        &p,
        sweep::Extrusion::Distance { depth: len, side: sweep::ExtrudeSide::Along },
        t,
    )
    .unwrap()
    .body
}

/// The rod-in-bore row in BOTH orders, every turn, with records and
/// membership samples.
#[test]
fn probe_rod_in_bore_both_orders() {
    let t = tol();
    let tube = revolved_about_y(
        vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 2.0), 0.0),
            (Point2::new(1.0, 2.0), 0.0),
        ],
        sweep::Revolution::Full,
        t,
    );
    let tube = place(&tube, Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0));
    let tube = at_rest(tube);
    for k in 0..30 {
        let th = f64::from(k) * core::f64::consts::TAU / 30.0;
        let (cx, cy) = (0.5 * th.cos(), 0.5 * th.sin());
        let rod = at_rest(place(&rod_z_review(0.5, -0.5, 1.0), Affine3::translation(Vec3::new(cx, cy, 0.0))));
        let mut row = Vec::new();
        for (o, p, q) in [("t,r", &tube, &rod), ("r,t", &rod, &tube)] {
            let s = match topo::union(p, q, t) {
                Ok(BooleanResult::Body(b)) => {
                    // membership: tube is 1<=rho<=2, z in [-2,0] (turned
                    // about x: y -> z, so axis along z, z in [-2, 0]?)
                    let mut wrong = 0;
                    for i in 0..9 {
                        for j in 0..9 {
                            for kk in 0..5 {
                                let q = Point3::new(-2.2 + 0.5 * f64::from(i) + 0.013, -2.2 + 0.5 * f64::from(j) + 0.017, -0.45 + 0.2 * f64::from(kk) + 0.011);
                                let rho = (q.x * q.x + q.y * q.y).sqrt();
                                let dr = ((q.x - cx).powi(2) + (q.y - cy).powi(2)).sqrt();
                                let in_rod = dr < 0.5 - 1e-6 && q.z.abs() < 0.5;
                                let near = (dr - 0.5).abs() < 1e-6 || (rho - 1.0).abs() < 1e-6 || (rho - 2.0).abs() < 1e-6;
                                if near { continue; }
                                let in_tube = (1.0..2.0).contains(&rho) && (-2.0..0.0).contains(&q.z) ;
                                let in_tube2 = (1.0..2.0).contains(&rho) && (0.0..2.0).contains(&q.z);
                                let got = topo::point_in_solid(&b.body, q, band(), t);
                                let got_in = matches!(got, Ok(topo::SolidContainment::In));
                                // tube's z-range unknown sign; accept either orientation consistently below
                                let _ = (in_tube, in_tube2);
                                if in_rod && !got_in { wrong += 1; }
                            }
                        }
                    }
                    format!(
                        "Ok solids={} vol={:.9} rod-pts-wrong={wrong} geo={:?} recs={}",
                        b.body.solids().count(),
                        topo::mass_properties(&b.body, t).unwrap().volume,
                        topo::validate_geometric(&b.body, t).is_ok(),
                        records(&b).len()
                    )
                }
                Ok(BooleanResult::Empty) => "Empty".into(),
                Err(e) => format!("Err {:?}", e.kind()),
            };
            row.push(format!("{o}: {s}"));
        }
        eprintln!("[turn {k}] {}", row.join(" || "));
    }
}

#[test]
fn probe_unrecorded_contact_detail() {
    let (p, r, _, _) = rod_on_puck(1.0, 0.25, (-1.5, 1.0), 1.0, 0.0, 0.5);
    let (p, r) = (at_rest(p), at_rest(r));
    for (o, x, y) in [("puck,rod", &p, &r), ("rod,puck", &r, &p)] {
        let Ok(BooleanResult::Body(u)) = topo::union(x, y, tol()) else { panic!() };
        let f = topo::validate_pseudomanifold(&u.body, &u.contacts, tol()).err().unwrap_or_default();
        for e in f {
            if let topo::ValidationError::UnrecordedContact { contact: topo::CensusContact::EdgeFaceOverlap { edge, face }, .. } = e {
                let ends: Vec<String> = u.body.half_edges().filter(|(_, h)| h.edge == edge).map(|(_, h)| at(&u.body, h.start)).collect();
                let fk = u.body.get_surface(u.body.get_face(face).unwrap().surface).unwrap().kind();
                eprintln!("{o}: unrecorded edge {ends:?} in face {fk:?}");
            }
        }
    }
}

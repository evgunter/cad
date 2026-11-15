#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]
use geom_core::{Band, Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, revolved_about_y};

fn mode(m: &str) {
    std::fs::write(format!("/home/user/cs-logs/MODE.{}", std::process::id()), m).unwrap();
}
fn kind(r: &str) -> &'static str {
    if r.contains("Side(In)") { "IN" } else if r.contains("Side(Out)") { "out" }
    else if r.contains("SetAside(Crossing") { "aside:crossing" }
    else if r.contains("SetAside(Graze") { "aside:graze" }
    else if r.contains("SetAside(Corner") || r.contains("SetAside(Wedge") { "aside:corner" }
    else if r.contains("SetAside(Limit") { "aside:limit" }
    else if r.contains("Pair(") { "pair" } else { "other" }
}

/// A torus about `y` (major 2, minor 0.5) with a shallow flat filed on its outside at `x = 2.5 - depth`.
#[test]
fn flat_on_a_torus() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let eps = tol.eps();
    let tube = vec![(Point2::new(2.5, 0.0), 1.0), (Point2::new(1.5, 0.0), 1.0)];
    let torus = revolved_about_y(tube, Revolution::Full, tol);
    for depth in [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6] {
        let d = 1.5 - depth;
        let cutter = brick::<f64>((-3.0, d), (-3.0, 3.0), (-3.0, 3.0), tol);
        let rest = |x: &topo::Body<f64>| topo::finish(x.clone(), tol).map_err(|e| format!("{e:?}"));
        let built = (|| -> Result<Option<topo::Body<f64>>, String> {
            let (a, b) = (rest(&torus)?, rest(&cutter)?);
            let r = topo::intersect(&a, &b, tol).map_err(|e| format!("{e:?}"))?;
            Ok(r.body().map(|b| b.body.clone().into_body()))
        })();
        let body = match built {
            Ok(Some(b)) => b,
            other => { println!("depth {depth:e}: no body: {}", format!("{:?}", other.err()).chars().take(300).collect::<String>()); continue; }
        };
        let frames = topo::test_support::edge_frames(&body, &[0.0005, 0.002, 0.01, 0.03, 0.1, 0.3, 0.5, 0.7, 0.9, 0.97, 0.99, 0.998, 0.9995], band);
        let spiric: Vec<_> = frames.iter().filter(|f| f.label.contains("Spiric")).collect();
        // the dihedral's sine at each sampled point: the two frames there
        let sine = |f: &topo::test_support::EdgeFrame| spiric.iter().filter(|g| (g.point - f.point).norm() < 1e-12 && g.face != f.face).map(|g| g.normal.cross(f.normal).norm()).fold(f64::INFINITY, f64::min);
        let least = spiric.iter().map(|f| sine(f)).fold(f64::INFINITY, f64::min);
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        let (mut rays, mut differ) = (0, 0);
        for f in &spiric {
            let sn = sine(f);
            if sn > 0.2 { continue; }
            for k in [0.0, 3.0, 12.0, 15.0, 20.0, 30.0, 50.0, 100.0, 300.0, 1000.0, 3000.0, 1e4] {
                for sign in [1.0, -1.0] {
                    let aim = f.point + f.into * (sign * k * eps);
                    for n in [f.normal, (f.normal + f.into * 0.7).normalize(), (f.normal - f.into * 0.7).normalize()] {
                        let reads: Vec<&'static str> = ["met", "unread", "clear"].iter().map(|m| { mode(m); kind(&topo::test_support::ray_at(&body, f.face, aim, n, band)) }).collect();
                        rays += 1;
                        if reads[0] != reads[1] || reads[0] != reads[2] { differ += 1; *tally.entry(format!("sin {sn:.1e} o={}ε {reads:?}", sign * k)).or_default() += 1; }
                    }
                }
            }
        }
        println!("depth {depth:e}: {} spiric frames, least dihedral sine {least:.2e}; {rays} rays about those under 0.2, {differ} differ by mode", spiric.len());
        let tally_all: std::collections::BTreeMap<String, usize> = tally.keys().map(|k| k.split("ε ").nth(1).unwrap().to_owned()).fold(Default::default(), |mut m, k| { *m.entry(k).or_default() += 1; m });
        println!("   patterns (keys): {tally_all:?}");
        // the reader's own answer, and containment about the shallow points, by mode
        let sides: Vec<String> = ["met", "unread", "clear"].iter().map(|m| { mode(m); format!("{:?}", topo::test_support::aimed_rays(&body, band)) }).collect();
        println!("   reader: {}", if sides[0] == sides[1] && sides[1] == sides[2] { format!("same in all modes: {}", sides[0]) } else { format!("DIFFER {sides:?}") });
        let mut rng = test_utils::fuzz::start("cs_contain");
        let (mut asked, mut moved) = (0, std::collections::BTreeMap::<String, usize>::new());
        for f in spiric.iter().filter(|f| sine(f) < 0.2) {
            for _ in 0..40 {
                let rho = rng.log_uniform(1e-8, 1e-1);
                let v = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * rho;
                let q = f.point + v;
                let reads: Vec<String> = ["met", "unread", "clear"].iter().map(|m| { mode(m); match topo::point_in_solid(&body, q, band, tol) { Ok(c) => format!("{c:?}"), Err(e) => format!("Err {}", format!("{e:?}").chars().take(24).collect::<String>()) } }).collect();
                asked += 1;
                if reads[0] != reads[1] || reads[0] != reads[2] { *moved.entry(format!("{reads:?}")).or_default() += 1; }
            }
        }
        println!("   containment: {asked} points asked, moved: {moved:?}");
        for (k, v) in tally.iter().take(0) { println!("   {k} x{v}"); }
    }
    mode("met");
}

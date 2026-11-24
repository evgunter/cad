import os
os.chdir('/home/user/cad')
def sub(p, old, new, count=1):
    s=open(p).read(); assert s.count(old)==count,(p,old[:60],s.count(old)); open(p,'w').write(s.replace(old,new))
# helper in geom-brep
sub('crates/geom-brep/src/lib.rs', "pub use fitted_lane::{", '''/// Review probe (not for merge).
#[doc(hidden)]
#[allow(clippy::too_many_arguments, missing_docs)]
pub fn review_probe(site: &str, scalar: &str, old: String, below: String, above: String, ov: String, nv: String) {
    if let Ok(dir) = std::env::var("REVIEW_PROBE_DIR") {
        use std::io::Write;
        let test = std::thread::current().name().unwrap_or("?").to_string();
        let path = format!("{dir}/{}.log", std::process::id());
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "{site}\\t{scalar}\\t{test}\\t{old}\\t{below}\\t{above}\\t{ov}\\t{nv}");
        }
    }
}
#[doc(hidden)]
#[allow(missing_docs)]
pub fn review_v<E>(r: &Result<geom_core::Sign, E>) -> String {
    match r { Ok(s) => format!("{s:?}"), Err(_) => "esc".into() }
}
pub use fitted_lane::{''')
# cone lead
sub('crates/geom-brep/src/curve_face/pose.rs', '''    let lead = || {
        decide(''', '''    let lead = || {
        {
            let old = crate::edge_extent_inf(&geom::Curve3::Line(line), lo, hi);
            let ov = geom_core::k_stats::detached(|| decide("curve_face_pose_cone_lead", Margin::levered(a2 / dd, old), band)).0;
            let nv = geom_core::k_stats::detached(|| decide("curve_face_pose_cone_lead", Margin::levered(a2 / dd, run), band)).0;
            crate::review_probe("cone_lead", T::NAME, format!("{:?}", old.length()), format!("{:?}", run.length()), format!("{:?}", run.length()), crate::review_v(&ov), crate::review_v(&nv));
        }
        decide(''')
# corner graze
sub('crates/topo/src/offset_derive.rs', '''    let graze_at = |sine: T, at: Point3<T>| {
        crate::validate::decide_levered(''', '''    let graze_at = |sine: T, at: Point3<T>| {
        {
            let b = geom_brep::span_reach_bounds(edge, span, at);
            let old = geom_brep::edge_extent_inf(edge, span.0, span.1);
            let ov = geom_core::k_stats::detached(|| crate::validate::decide("offset_corner_graze", Margin::levered(sine, old), band)).0;
            let nv = geom_core::k_stats::detached(|| crate::validate::decide_levered(("offset_corner_graze", "offset_corner_graze_straddle"), sine, b, band)).0;
            geom_brep::review_probe("corner_graze", T::NAME, format!("{:?}", old.length()), format!("{:?}", b.below().length()), format!("{:?}", b.above().length()), geom_brep::review_v(&ov), geom_brep::review_v(&nv));
        }
        crate::validate::decide_levered(''')
# iso lean arm
sub('crates/topo/src/ring_path/spline_wall.rs', '''        geom_brep::within_carrier_bend(reach, &carrier, stop)
    }''', '''        let new = geom_brep::within_carrier_bend(reach, &carrier, stop);
        let old = geom_brep::within_carrier_bend(self.travel().0, &carrier, stop);
        geom_brep::review_probe("iso_arm", T::NAME, format!("{:?}", old.length()), format!("{:?}", new.length()), format!("{:?}", new.length()), "-".into(), "-".into());
        new
    }''')
# trace
T='crates/topo/src/boolean/trace.rs'
sub(T, '''/// One crossing at `point`, `along` the piece,''', '''#[allow(clippy::too_many_arguments)]
fn probe_sense<T: Decide>(site: &str, surface: &Surface<T>, sense: bool, point: Point3<T>, velocity: Vec3<T>, old: geom_core::Lever<T>, new: geom_core::Lever<T>, band: Band) {
    let outward = crate::face_normal::surface_outward_normal(surface, point, sense).vec();
    let x = outward.dot(velocity) / outward.norm();
    let ov = geom_core::k_stats::detached(|| decide(TRACE_CROSSING_SENSE, Margin::levered(x, old), band)).0;
    let nv = geom_core::k_stats::detached(|| decide(TRACE_CROSSING_SENSE, Margin::levered(x, new), band)).0;
    geom_brep::review_probe(site, T::NAME, format!("{:?}", old.length()), format!("{:?}", new.length()), format!("{:?}", new.length()), geom_brep::review_v(&ov), geom_brep::review_v(&nv));
}

/// One crossing at `point`, `along` the piece,''')
sub(T, '''            .map(|(t, point)| crossing(surface, sense, t, point, dir, arm(point), band))''', '''            .map(|(t, point)| {
                let old = geom_brep::ExtentBall::of_covered(
                    geom::cover::Cover::segment(ray.start_point(), ray.start_point() + dir * run).as_ref(),
                )
                .lever_from(ray.start_point());
                probe_sense("trace_ray", surface, sense, point, dir, old, arm(point), band);
                crossing(surface, sense, t, point, dir, arm(point), band)
            })''')
sub(T, '''                        .below();
                crossing(surface, sense, theta, point, velocity, reach, band)''', '''                        .below();
                let old = geom_brep::ExtentBall::of_covered(
                    geom::cover::Cover::arc(&frame, (radius, radius), geom::cover::ParamWindow::new(T::zero(), run)).as_ref(),
                )
                .lever();
                probe_sense("trace_arc", surface, sense, point, velocity, old, reach, band);
                crossing(surface, sense, theta, point, velocity, reach, band)''')
sub(T, '''                    .below();
            crossing(surface, sense, theta, point, velocity, reach, band)''', '''                    .below();
            let old = geom_brep::ExtentBall::of_covered(
                geom::cover::Cover::circle(geom::cover::Core { axis: geom::Line { origin: center, dir: frame.axis_unit() }, radius }).as_ref(),
            )
            .lever();
            probe_sense("trace_circle", surface, sense, point, velocity, old, reach, band);
            crossing(surface, sense, theta, point, velocity, reach, band)''')
print("applied")

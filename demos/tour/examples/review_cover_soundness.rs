//! Review probe (PR 7120): every edge of every tour body, its cover's
//! world box over its span at `Interval` (the box `boundary_of` reads),
//! against 512 f64 samples of the edge. Prints the worst escape per body
//! and the total; an escape larger than a rounding is a cover that is not
//! an enclosure.
use pncad::geom::cover::{BoxAxes, ParamWindow};
use pncad::geom_core::{Bounds, Interval, Real, Tol};

fn main() {
    let tol = Tol::witness();
    let work = std::env::temp_dir().join("review-cover-store");
    std::fs::create_dir_all(&work).unwrap();
    let (mut edges, mut worst_all) = (0usize, 0.0f64);
    demo_tour::walk_tour(
        &mut |stop| {
            for sb in &stop.bodies {
                let body = &sb.body;
                let mut worst = 0.0f64;
                for (ek, _) in body.edges() {
                    let Ok(curve) = body.stored_curve(ek) else { continue };
                    let (t0, t1) = curve.params();
                    let c = curve.carrier().map_scalar(Interval::from_f64);
                    let Ok(cv) = c.cover_over(ParamWindow::new(Interval::from_f64(t0), Interval::from_f64(t1))) else { continue };
                    let (lo, hi) = cv.box_in(&BoxAxes::World);
                    edges += 1;
                    for k in 0..=512 {
                        let t = t0 + (t1 - t0) * f64::from(k) / 512.0;
                        let p = curve.carrier().eval(t);
                        for (x, l, h) in [(p.x, lo.x.lo(), hi.x.hi()), (p.y, lo.y.lo(), hi.y.hi()), (p.z, lo.z.lo(), hi.z.hi())] {
                            worst = worst.max(l - x).max(x - h);
                        }
                    }
                }
                worst_all = worst_all.max(worst);
                println!("SOUND {} | worst escape {:e}", sb.name, worst);
            }
        },
        &work,
        tol,
    );
    println!("SOUND TOTAL edges {edges} worst escape {worst_all:e}");
}

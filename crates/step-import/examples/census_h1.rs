//! EXCH-H1 Phase-1 census harness (throwaway, uncommitted).
//!
//! Enumerates dm1's degree-1 / .POLYLINE_FORM. carriers and computes,
//! for each, the zero-radius cylinder composite's certified sup
//! (√sup metres) and the control-point chord-projection excursions.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::print_stdout,
    clippy::panic
)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface};

const INCH: f64 = 0.0254;

fn main() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/wild/stepcode/dm1-id-214.stp"
    );
    let text = std::fs::read_to_string(path).unwrap();
    // Flatten records: join lines, split on ';'.
    let flat: String = text
        .chars()
        .filter(|c| *c != '\n' && *c != '\r' && *c != ' ')
        .collect();
    let mut points: std::collections::BTreeMap<u64, Point3<f64>> = Default::default();
    let mut carriers: Vec<(u64, Vec<u64>, Vec<f64>)> = Vec::new(); // (id, ctrl refs, knots)
    for rec in flat.split(';') {
        let Some(eq) = rec.find('=') else { continue };
        let id: u64 = match rec[1..eq].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let body = &rec[eq + 1..];
        if let Some(rest) = body.strip_prefix("CARTESIAN_POINT(") {
            // CARTESIAN_POINT('name',(x,y,z))
            let coords_start = rest.find("',(").unwrap() + 3;
            let coords_end = rest[coords_start..].find(')').unwrap() + coords_start;
            let nums: Vec<f64> = rest[coords_start..coords_end]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(nums.len(), 3, "#{id}");
            points.insert(
                id,
                Point3::new(nums[0] * INCH, nums[1] * INCH, nums[2] * INCH),
            );
        } else if let Some(rest) = body.strip_prefix("QUASI_UNIFORM_CURVE(") {
            // QUASI_UNIFORM_CURVE('name',1,(#a,#b),.POLYLINE_FORM.,...)
            let d_start = rest.find("',").unwrap() + 2;
            let (deg, rest2) = rest[d_start..].split_once(",(").unwrap();
            if deg != "1" {
                continue;
            }
            let refs_end = rest2.find(')').unwrap();
            let refs: Vec<u64> = rest2[..refs_end]
                .split(',')
                .map(|s| s.trim_start_matches('#').parse().unwrap())
                .collect();
            // quasi-uniform degree-1 on n points: clamped uniform knots
            let n = refs.len();
            let mut knots = vec![0.0, 0.0];
            for k in 1..n - 1 {
                knots.push(k as f64 / (n - 1) as f64);
            }
            knots.extend([1.0, 1.0]);
            carriers.push((id, refs, knots));
        } else if let Some(rest) = body.strip_prefix("B_SPLINE_CURVE_WITH_KNOTS(") {
            let d_start = rest.find("',").unwrap() + 2;
            let (deg, rest2) = rest[d_start..].split_once(",(").unwrap();
            if deg != "1" {
                continue;
            }
            let refs_end = rest2.find(')').unwrap();
            let refs: Vec<u64> = rest2[..refs_end]
                .split(',')
                .map(|s| s.trim_start_matches('#').parse().unwrap())
                .collect();
            // ...,(mults),(values),.UNSPECIFIED.)
            let tail = &rest2[refs_end..];
            let m_start = tail.find(",(").unwrap() + 2;
            let m_end = tail[m_start..].find(')').unwrap() + m_start;
            let mults: Vec<usize> = tail[m_start..m_end]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let v_start = tail[m_end..].find(",(").unwrap() + m_end + 2;
            let v_end = tail[v_start..].find(')').unwrap() + v_start;
            let values: Vec<f64> = tail[v_start..v_end]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let mut knots = Vec::new();
            for (m, v) in mults.iter().zip(values) {
                knots.extend(std::iter::repeat_n(v, *m));
            }
            carriers.push((id, refs, knots));
        }
    }
    println!(
        "degree-1 .POLYLINE_FORM. carriers found: {}",
        carriers.len()
    );
    println!(
        "{:>6} {:>4} {:>13} {:>13} {:>13} {:>13}",
        "id", "n", "sup(m^2)", "sqrt_sup(m)", "chord_len(m)", "excursion(m)"
    );
    let mut worst_res = 0.0f64;
    let mut worst_exc = 0.0f64;
    for (id, refs, knots) in &carriers {
        let control: Vec<Point3<f64>> = refs.iter().map(|r| points[r]).collect();
        let n = control.len();
        let weights = vec![1.0; n];
        let kv = KnotVector::clamped(knots.clone(), 1).unwrap();
        let curve = NurbsCurve3::new(kv, control.clone(), weights).unwrap();
        let first = control[0];
        let last = control[n - 1];
        let chord = last - first;
        let len = chord.norm();
        let surface = ImplicitSurface::Cylinder {
            point: [first.x, first.y, first.z],
            axis: [chord.x, chord.y, chord.z],
            radius: 0.0,
        };
        let coords = curve.ring_coords();
        let data = CurveRingData::new(curve.knots(), curve.weights(), &coords).unwrap();
        let sup = match compose::implicit_composite(&data, &surface) {
            Ok(form) => form.sup_bound(),
            Err(e) => {
                println!("#{id}: COMPOSITE REFUSED: {e:?}");
                continue;
            }
        };
        let root = sup.abs().sqrt();
        // Control-point chord projections: excursion outside [0, len].
        let dir = chord / len;
        let mut excursion = 0.0f64;
        for p in &control {
            let t = (*p - first).dot(dir);
            if t < 0.0 {
                excursion = excursion.max(-t);
            }
            if t > len {
                excursion = excursion.max(t - len);
            }
        }
        worst_res = worst_res.max(root);
        worst_exc = worst_exc.max(excursion);
        println!("{id:>6} {n:>4} {sup:>13.3e} {root:>13.3e} {len:>13.6e} {excursion:>13.3e}");
    }
    println!(
        "worst sqrt_sup: {worst_res:e} m; worst excursion: {worst_exc:e} m; eps_in(file) = 1e-5 m"
    );
}

//! Review probe for PR 4289 (scratch branch, not for merge): reads cones
//! and directions from `RV_CASES`, writes each reading to `RV_OUT`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use super::*;
use geom_core::Tol;
use std::fmt::Write as _;

fn key(n: u64) -> slotmap::KeyData {
    slotmap::KeyData::from_ffi((1 << 32) | n)
}

fn v(x: &[f64]) -> Vec3<f64> {
    Vec3::new(x[0], x[1], x[2])
}

fn code(c: SideCode) -> &'static str {
    match c {
        SideCode::In => "In",
        SideCode::Out => "Out",
        SideCode::On => "On",
    }
}

fn err(e: &BooleanError) -> String {
    match e {
        BooleanError::Escalated { diag, .. } => {
            format!("Err:{}", diag.predicate.unwrap_or("?"))
        }
        other => format!(
            "Err:{}",
            format!("{other:?}")
                .split_whitespace()
                .next()
                .unwrap_or("?")
        ),
    }
}

#[test]
fn rv_probe_cases() {
    let Ok(path) = std::env::var("RV_CASES") else {
        return;
    };
    let out_path = std::env::var("RV_OUT").unwrap();
    let eps: f64 = std::env::var("RV_EPS")
        .ok()
        .map_or(1e-9, |s| s.parse().unwrap());
    let band = Band::linear_at(Tol::witness(), eps).unwrap();
    let text = std::fs::read_to_string(path).unwrap();
    let o = Point3::new(0.0, 0.0, 0.0);
    let mut out = String::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let w: Vec<&str> = line.split_whitespace().collect();
        if w.first() != Some(&"case") {
            continue;
        }
        let id = w[1];
        let (ns, np): (usize, usize) = (w[2].parse().unwrap(), w[3].parse().unwrap());
        let mut sectors = Vec::new();
        for k in 0..ns {
            let f: Vec<f64> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect();
            let arm = f[17];
            let reach = |edge: f64, far: &[f64]| {
                if edge > 0.5 {
                    Reach::Chord {
                        base: o,
                        far: o + v(far),
                    }
                } else {
                    Reach::Bisector(arm)
                }
            };
            sectors.push(BoolSector {
                he: HalfEdgeKey::from(key(k as u64 + 1)),
                start: v(&f[0..3]),
                end: v(&f[3..6]),
                normal: OutwardNormal::from_chart(v(&f[6..9]), true),
                start_reach: reach(f[9], &f[11..14]),
                end_reach: reach(f[10], &f[14..17]),
                face: FaceKey::from(key(1000 + f[18] as u64)),
                arm,
            });
        }
        let mut probes = Vec::new();
        for k in 0..np {
            let f: Vec<f64> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect();
            probes.push(BoolSector {
                he: HalfEdgeKey::from(key(5000 + k as u64)),
                start: Vec3::new(0.0, 0.0, 1.0),
                end: v(&f[0..3]),
                start_reach: Reach::Bisector(1.0),
                end_reach: Reach::Chord {
                    base: o,
                    far: o + v(&f[3..6]),
                },
                face: FaceKey::from(key(99_999)),
                normal: OutwardNormal::from_chart(Vec3::new(1.0, 0.0, 0.0), true),
                arm: 1.0,
            });
        }
        // the corner's met / pointed, through the full dispatch
        let mp = match wedge_classes(&[], &sectors, band) {
            Ok(Some(r)) => format!("met={} pointed={}", r.met, r.pointed),
            Ok(None) => "none".into(),
            Err(e) => err(&e),
        };
        let cr = match cone_read(&[], &sectors, band) {
            Ok(Some(r)) => format!("met={} pointed={}", r.met, r.pointed),
            Ok(None) => "none".into(),
            Err(e) => err(&e),
        };
        writeln!(out, "case {id} {mp} | {cr}").unwrap();
        let only: Option<(String, usize)> = std::env::var("RV_ONLY").ok().map(|o| {
            let (a, b) = o.split_once(':').unwrap();
            (a.to_string(), b.parse().unwrap())
        });
        for (k, p) in probes.iter().enumerate() {
            if let Some((c, pk)) = &only {
                if c != id || *pk != k {
                    continue;
                }
                eprintln!("sectors: {sectors:#?}");
            }
            let wc = match wedge_classes(std::slice::from_ref(p), &sectors, band) {
                Ok(Some(r)) => code(r.rows[0].1).to_string(),
                Ok(None) => "None".into(),
                Err(e) => err(&e),
            };
            let cs = match cone_side(p.end, p.end_reach, &sectors, band) {
                Ok(Some(c)) => code(c).to_string(),
                Ok(None) => "None".into(),
                Err(e) => err(&e),
            };
            writeln!(out, "p {k} {wc} {cs}").unwrap();
            if only.is_some() {}
        }
    }
    std::fs::write(out_path, out).unwrap();
}

/// Review 2, exemption (a): a flat corner (every face on z = 0) with one
/// face dented 5e-7 m at its far vertex, beside a 1 mm edge. That face's
/// reference `p` reads Zero against the flat faces' plane (levered at the
/// 1 mm arm), and `in_sector` decides `p` outside face 1's sector, so
/// face 1 is passed over. The great arc from the probe to `p` crosses
/// face 1 inside its sector, so the parity is one short. The probe lies
/// 2e-7 rad (200 zero bands at ε = 1e-9) off face 2. The exact `Fraction`
/// oracle reads it `Out`; head reads `In` through `cone_side` and through
/// `wedge_classes`. Red on head 2f67fe0d.
#[test]
fn rv2_a_reference_on_a_plane_beside_a_crossing_inside_the_sector() {
    let band = Band::linear_at(Tol::witness(), 1e-9).unwrap();
    let o = Point3::new(0.0, 0.0, 0.0);
    let rows: [[f64; 19]; 6] = [
        [
            0.9987502603949663,
            0.04997916927067833,
            0.0,
            0.9553364891254865,
            -0.2955202066613026,
            4.999999999999375e-07,
            7.287764486087944e-08,
            -1.456338067317127e-06,
            -0.9999999999989369,
            1.0,
            1.0,
            0.0009987502603949663,
            4.997916927067833e-05,
            0.0,
            0.9553364891254865,
            -0.2955202066613026,
            4.999999999999375e-07,
            0.001,
            0.0,
        ],
        [
            0.8775825618903728,
            0.479425538604203,
            0.0,
            0.9987502603949663,
            0.04997916927067833,
            0.0,
            0.0,
            0.0,
            -1.0,
            1.0,
            1.0,
            0.8775825618903728,
            0.479425538604203,
            0.0,
            0.0009987502603949663,
            4.997916927067833e-05,
            0.0,
            0.001,
            1.0,
        ],
        [
            -0.4161468365471424,
            0.9092974268256817,
            0.0,
            0.8775825618903728,
            0.479425538604203,
            0.0,
            0.0,
            0.0,
            -1.0,
            1.0,
            1.0,
            -0.4161468365471424,
            0.9092974268256817,
            0.0,
            0.8775825618903728,
            0.479425538604203,
            0.0,
            1.0,
            2.0,
        ],
        [
            -0.9364566872907963,
            -0.35078322768961984,
            0.0,
            -0.4161468365471424,
            0.9092974268256817,
            0.0,
            0.0,
            0.0,
            -1.0,
            1.0,
            1.0,
            -0.9364566872907963,
            -0.35078322768961984,
            0.0,
            -0.4161468365471424,
            0.9092974268256817,
            0.0,
            1.0,
            3.0,
        ],
        [
            0.2836621854632263,
            -0.9589242746631386,
            0.0,
            -0.9364566872907963,
            -0.35078322768961984,
            0.0,
            0.0,
            0.0,
            -1.0,
            1.0,
            1.0,
            0.2836621854632263,
            -0.9589242746631386,
            0.0,
            -0.9364566872907963,
            -0.35078322768961984,
            0.0,
            1.0,
            4.0,
        ],
        [
            0.9553364891254865,
            -0.2955202066613026,
            4.999999999999375e-07,
            0.2836621854632263,
            -0.9589242746631386,
            0.0,
            5.760914256723924e-07,
            1.704152842415664e-07,
            -0.9999999999998195,
            1.0,
            1.0,
            0.9553364891254865,
            -0.2955202066613026,
            4.999999999999375e-07,
            0.2836621854632263,
            -0.9589242746631386,
            0.0,
            1.0,
            5.0,
        ],
    ];
    let sectors: Vec<BoolSector<f64>> = rows
        .iter()
        .enumerate()
        .map(|(k, f)| BoolSector {
            he: HalfEdgeKey::from(key(k as u64 + 1)),
            start: v(&f[0..3]),
            end: v(&f[3..6]),
            normal: OutwardNormal::from_chart(v(&f[6..9]), true),
            start_reach: Reach::Chord {
                base: o,
                far: o + v(&f[11..14]),
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + v(&f[14..17]),
            },
            face: FaceKey::from(key(1000 + f[18] as u64)),
            arm: f[17],
        })
        .collect();
    // 0.6 rad round from the dented face, 2e-7 rad below the flat faces.
    let far = Vec3::new(0.8253356149096618, 0.564642473395024, -1.99999999999996e-07);
    let probe = BoolSector {
        he: HalfEdgeKey::from(key(5000)),
        start: Vec3::new(0.0, 0.0, 1.0),
        end: far,
        start_reach: Reach::Bisector(1.0),
        end_reach: Reach::Chord {
            base: o,
            far: o + far,
        },
        face: FaceKey::from(key(99_999)),
        normal: OutwardNormal::from_chart(Vec3::new(1.0, 0.0, 0.0), true),
        arm: 1.0,
    };
    let cs = cone_side(probe.end, probe.end_reach, &sectors, band);
    let wc =
        wedge_classes(std::slice::from_ref(&probe), &sectors, band).map(|r| r.map(|r| r.rows[0].1));
    // Exactly Out; a refusal or None would also be sound.
    assert!(
        !matches!(cs, Ok(Some(SideCode::In))),
        "cone_side reads {cs:?}; exactly Out"
    );
    assert!(
        !matches!(wc, Ok(Some(SideCode::In))),
        "wedge_classes reads {wc:?}; exactly Out"
    );
}

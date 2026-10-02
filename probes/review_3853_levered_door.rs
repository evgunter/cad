//! Review probe for PR #3853: `UnitVec3::levered` against `UnitVec3::new`
//! across lengths and arms. Prints a table; asserts only bit-identity of
//! the normalized vector wherever both doors certify.
use geom_core::{Band, UnitVec3, Vec3};

fn show(r: &Result<UnitVec3<f64>, geom_core::UnitVec3Error>) -> String {
    match r {
        Ok(_) => "Ok".into(),
        Err(geom_core::UnitVec3Error::Escalated(_)) => "Escalated".into(),
        Err(e) => format!("{e:?}"),
    }
}

#[test]
fn levered_vs_new_table() {
    for eps in [1e-12, 1e-9, 1e-6] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let lens = [
            0.0, 5e-324, 1e-310, 1e-300, 1e-14, 1e-11, 1e-8, 1e-5, 1.0, 1e10, 1e300, f64::MAX,
            f64::INFINITY, f64::NAN,
        ];
        let arms = [
            -1.0, 0.0, 1e-13, 1e-10, 1e-7, 1e-3, 1.0, 1e3, 1e6, 1e300, f64::INFINITY, f64::NAN,
        ];
        let mut cert_degenerate = Vec::new();
        let mut refuse_unit = Vec::new();
        for &l in &lens {
            let v = Vec3::new(l * 0.6, 0.0, l * 0.8);
            let n = UnitVec3::new(v, "review_probe", band);
            for &a in &arms {
                let lv = UnitVec3::levered(v, "review_probe", band, a);
                if let (Ok(x), Ok(y)) = (&n, &lv) {
                    let (x, y) = (x.get(), y.get());
                    assert_eq!(
                        [x.x.to_bits(), x.y.to_bits(), x.z.to_bits()],
                        [y.x.to_bits(), y.y.to_bits(), y.z.to_bits()]
                    );
                }
                if lv.is_ok() && n.is_err() {
                    cert_degenerate.push(format!("|v|={l:e} arm={a:e}: new={} levered=Ok", show(&n)));
                }
                if l == 1.0 && !matches!(lv, Ok(_)) {
                    refuse_unit.push(format!("|v|=1 arm={a:e}: levered={}", show(&lv)));
                }
            }
        }
        println!("== eps {eps:e}: levered certifies where new refuses ({}):", cert_degenerate.len());
        for s in &cert_degenerate {
            println!("   {s}");
        }
        println!("== eps {eps:e}: a UNIT vector levered refuses:");
        for s in &refuse_unit {
            println!("   {s}");
        }
    }
}

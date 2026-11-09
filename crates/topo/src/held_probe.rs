//! PROBE ONLY (not for merge): records, at each carrier-held placement,
//! how the on-carrier residual would decide and what the site answers.
use geom_core::{Band, Decide, Margin, Point3, Sign};

use crate::{Body, FaceKey};

pub(crate) fn record<T: Decide>(
    site: &str,
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
    answer: &str,
) {
    let Ok(path) = std::env::var("CAD_HELD_PROBE") else {
        return;
    };
    let Some(view) = body.face(face) else {
        return;
    };
    let surface = view.surface();
    let kind = format!("{:?}", surface.kind());
    let r = geom_brep::implicit_residual(surface, q);
    let verdict = geom_core::k_stats::unlogged("PROBE", || {
        match geom_core::k_stats::decide("held_probe", Margin::of(r), band) {
            Ok(Sign::Zero) => "zero",
            Ok(Sign::Positive | Sign::Negative) => "off",
            Err(_) => "inband",
        }
    });
    let scalar = std::any::type_name::<T>().rsplit("::").next().unwrap_or("?");
    let line = format!("{site}\t{kind}\t{scalar}\t{verdict}\t{answer}\n");
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

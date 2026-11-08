//! One-off side-by-side probe of the split's and the Boolean's sector
//! walks (`run-loops-of-split-and-pierce-are-twins` stage 2). Runs only
//! when `STAR_WALK_PROBE` names a directory, and appends one line per
//! call there. Removed once its counts are taken.

use std::io::Write as _;

use geom_core::{Band, Decide, Vec3};

use super::sectors::{self, BoolSector, Partners};
use super::{Operand, SideCode};
use crate::body::Body;
use crate::entity::VertexKey;
use crate::sector_face::VertexFace;
use crate::sector_shape::SlitTips;
use crate::splitting::PlaneSide;
use geom_brep::sector_shape::CornerBounds;

fn sink() -> Option<std::fs::File> {
    let dir = std::env::var_os("STAR_WALK_PROBE")?;
    let path = std::path::Path::new(&dir).join(format!(
        "{}-{}.log",
        std::process::id(),
        std::thread::current()
            .name()
            .unwrap_or("anon")
            .replace("::", "__")
    ));
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .ok()
}

pub(crate) fn enabled() -> bool {
    std::env::var_os("STAR_WALK_PROBE").is_some()
}

/// One walk's entry: half-edge, real or not, unit direction.
struct Ent<T: geom_core::Real> {
    he: crate::entity::HalfEdgeKey,
    edge: bool,
    dir: Vec3<T>,
}

/// The split's walk geometry at `vertex`, read against `normal` (its
/// sides discarded): the entries `classify_neighborhood_rays` pushes.
fn split_view<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    normal: Vec3<T>,
    tips: &SlitTips,
    band: Band,
) -> Result<(Vec<Ent<T>>, Vec<bool>), String> {
    use crate::splitting::neighborhood::chord;
    let orbit = body.vertex_orbit_linked(vertex);
    let mut out = Vec::new();
    let mut apex = Vec::new();
    for (i, &he) in orbit.iter().enumerate() {
        let (_, dir_a, _) = chord(body, vertex, he).map_err(|e| format!("chord {e:?}"))?;
        out.push(Ent {
            he,
            edge: true,
            dir: dir_a.normalize(),
        });
        let next = orbit[(i + 1) % orbit.len()];
        let (_, dir_b, _) = chord(body, vertex, next).map_err(|e| format!("chord {e:?}"))?;
        let r = crate::sector_face::resolve(body, vertex, he, band)
            .map_err(|e| format!("face {e:?}"))?;
        let is_apex = matches!(r.at_vertex, VertexFace::Apex(_));
        apex.push(is_apex);
        let bounds: CornerBounds = tips.bounds(vertex, r.face, he == next);
        let wide = crate::splitting::wide_entries(
            (vertex, r.face),
            (dir_a, dir_b),
            r.at_vertex,
            bounds,
            normal,
            band,
        )
        .map_err(|e| format!("wide {e:?}"))?;
        for (d, _, _) in wide {
            out.push(Ent {
                he,
                edge: false,
                dir: d.normalize(),
            });
        }
    }
    Ok((out, apex))
}

fn bool_view<T: Decide>(sectors: &[BoolSector<T>]) -> Vec<Ent<T>> {
    sectors
        .iter()
        .map(|s| Ent {
            he: s.he,
            edge: s.end_edge(),
            dir: s.end.normalize(),
        })
        .collect()
}

/// Compares the two views; a short verdict word and detail.
fn compare<T: Decide>(a: &[Ent<T>], b: &[Ent<T>]) -> (&'static str, String) {
    if a.len() != b.len() {
        let ea = a.iter().filter(|e| e.edge).count();
        let eb = b.iter().filter(|e| e.edge).count();
        return (
            "count",
            format!(
                "split {} ({ea} edges) bool {} ({eb} edges)",
                a.len(),
                b.len()
            ),
        );
    }
    for (k, (x, y)) in a.iter().zip(b).enumerate() {
        if x.he != y.he || x.edge != y.edge {
            return ("kind", format!("at {k}"));
        }
        let d = (x.dir - y.dir).norm();
        let far = crate::validate::decide(
            "walk_probe_dir",
            geom_core::Margin::of(d - T::from_f64(1e-4)),
            Band::linear(geom_core::Tol::witness()).expect("band"),
        );
        if !matches!(far, Ok(geom_core::Sign::Negative)) {
            return ("dir", format!("at {k} {d:?}"));
        }
    }
    ("same", String::new())
}

/// The probe at a split vertex: the split's walk (pre-rule classes in
/// `classes`, one per entry) against the Boolean's walk read against
/// the split plane as a pierced plane.
pub(crate) fn at_split<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    normal: Vec3<T>,
    classes: &[PlaneSide],
    tips: &SlitTips,
    band: Band,
) {
    let Some(mut f) = sink() else { return };
    let line = (|| -> Result<String, String> {
        let sectors = sectors::build_sectors(
            body,
            Operand::A,
            vertex,
            tips,
            &Partners::Pierced(normal),
            band,
        )
        .map_err(|e| format!("bool-refused {e:?}"))?;
        let (sv, apex) = split_view(body, vertex, normal, tips, band)?;
        let bv = bool_view(&sectors);
        let (geo, detail) = compare(&sv, &bv);
        // Sides, where the geometry agrees: the Boolean's bound reading
        // against the split plane (a plane datum, no charge).
        let mut side = "n/a".to_string();
        if geo == "same" {
            let n = geom_brep::OutwardNormal::from_chart(normal, true);
            let mut agree = 0;
            let mut diff = Vec::new();
            for (k, s) in sectors.iter().enumerate() {
                let read = sectors::pierce_side(
                    s.end,
                    s.end_reach,
                    n,
                    sectors::FaceBend::Plane,
                    sectors::NO_CURVATURE(),
                    crate::link::metric::Reading::plain(band),
                );
                let code = match read {
                    Ok((r, _)) => Some(r.code),
                    Err(_) => None,
                };
                let want = match classes[k] {
                    PlaneSide::Above => Some(SideCode::Out),
                    PlaneSide::Below => Some(SideCode::In),
                    PlaneSide::On => Some(SideCode::On),
                };
                if code == want {
                    agree += 1;
                } else {
                    diff.push(format!(
                        "{k}:{}:{:?}/{:?}",
                        if s.end_edge() { "e" } else { "b" },
                        classes[k],
                        code
                    ));
                }
            }
            side = if diff.is_empty() {
                format!("sides-same {agree}")
            } else {
                format!("sides-diff {agree} {}", diff.join(","))
            };
        }
        Ok(format!(
            "split {geo} n={} apex={} {detail} {side}",
            sv.len(),
            apex.iter().any(|a| *a)
        ))
    })()
    .unwrap_or_else(|e| format!("split error {e}"));
    let _ = writeln!(f, "{line}");
}

/// The probe at a pierce or a vertex–vertex read: the Boolean's walk
/// against the split's walk geometry read against `normal`.
pub(crate) fn at_bool<T: Decide>(
    site: &'static str,
    body: &Body<T>,
    vertex: VertexKey,
    normal: Vec3<T>,
    sectors: &[BoolSector<T>],
    tips: &SlitTips,
    band: Band,
) {
    let Some(mut f) = sink() else { return };
    let line = match split_view(body, vertex, normal, tips, band) {
        Ok((sv, apex)) => {
            let (geo, detail) = compare(&sv, &bool_view(sectors));
            format!(
                "{site} {geo} n={} apex={} {detail}",
                sv.len(),
                apex.iter().any(|a| *a)
            )
        }
        Err(e) => format!("{site} error {e}"),
    };
    let _ = writeln!(f, "{line}");
}

pub(crate) fn note(line: String) {
    if let Some(mut f) = sink() {
        let _ = writeln!(f, "{line}");
    }
}

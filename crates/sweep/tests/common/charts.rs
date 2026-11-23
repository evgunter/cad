//! A body's CHARTS — its faces grouped by the surface they wear — and
//! the [`topo::ChartMove`] sets the simultaneous offset doors take.
//!
//! **Routing rule** (`common/mod.rs`'s, applied here): what a suite
//! drives an offset door WITH. The doors take a move per group of
//! faces sharing one surface key, so every suite that calls one builds
//! this partition first, and builds it here. Not `sweep::test_support`, whose consumers are
//! other crates: every consumer is in this one. Not [`super::orient`]
//! or [`super::cap_rims`], which read a body a suite has built rather
//! than the input a suite hands a door.
//!
//! **The partition's order**: groups in the order their first face
//! appears in the face arena, and each group's faces in arena order —
//! deterministic, so a printed move set reads the same run to run.
//!
//! **Deliberately NOT absorbed**:
//! `shell10_r2_probes::chart_of`, the ONE chart a given face wears — a
//! filter over the arena for one surface key, not the partition.
//!
//! **Not members, so not copies**: a move set whose DISTANCE rule is
//! the row's own builds on [`charts`] and keeps the rule where it is
//! read (`sf2a_r1_head`'s centroid-signed distances); a per-FACE move
//! set (`sf2a_r2_probes`, `sf2a_r2_interval_probe`, each face its own
//! plane) is not a grouping at all; and a move set a row builds wrong
//! on purpose (a chart named twice, a move swallowing a second face)
//! is what that row is about.

#![allow(dead_code)]

use geom_core::Real;
use topo::{Body, ChartMove, FaceKey, SolidKey, SurfaceKey};

/// `faces` gathered by the surface each wears: groups in
/// first-appearance order, each group in the order its faces arrived.
fn group<T: Real>(body: &Body<T>, faces: impl IntoIterator<Item = FaceKey>) -> Vec<Vec<FaceKey>> {
    let mut out: Vec<(SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for face in faces {
        let key = body.get_face(face).expect("a live face").surface;
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, group)) => group.push(face),
            None => out.push((key, vec![face])),
        }
    }
    out.into_iter().map(|(_, group)| group).collect()
}

/// Every chart of `body`, over every face in arena order.
pub fn charts<T: Real>(body: &Body<T>) -> Vec<Vec<FaceKey>> {
    group(body, body.faces().map(|(k, _)| k))
}

/// Every chart of `solid`, over its faces in arena order
/// ([`Body::faces_of_solid`]).
pub fn charts_of<T: Real>(body: &Body<T>, solid: SolidKey) -> Vec<Vec<FaceKey>> {
    group(
        body,
        body.faces_of_solid(solid).expect("the solid resolves"),
    )
}

/// One move per chart, each by the same signed `distance` along the
/// chart's stored normal ([`ChartMove::distance`]) — out of the
/// material on a positively-sensed face and into it on a reversed one,
/// so one sign is not one direction across a body with both senses.
/// [`moves_inward`] is the sense-aware twin.
pub fn moves_by<T: Real>(charts: Vec<Vec<FaceKey>>, distance: T) -> Vec<ChartMove<T>> {
    charts
        .into_iter()
        .map(|faces| ChartMove { faces, distance })
        .collect()
}

/// One move per chart, each by `t` INTO the material — `shell`'s own
/// inward rule over [`ChartMove::distance`]'s stored normal, which
/// points out of the solid on a positively-sensed face and into it on
/// a reversed one, so a positive sense moves by `−t`.
pub fn moves_inward<T: Real>(body: &Body<T>, charts: Vec<Vec<FaceKey>>, t: T) -> Vec<ChartMove<T>> {
    charts
        .into_iter()
        .map(|faces| {
            let sense = body.get_face(faces[0]).expect("a live face").sense;
            ChartMove {
                faces,
                distance: if sense { -t } else { t },
            }
        })
        .collect()
}

/// Every chart of `body` moved inward by `t` — the move set a suite
/// hollowing a one-solid body hands the simultaneous door, spelled at
/// the door itself rather than through `shell`.
pub fn hollow_moves<T: Real>(body: &Body<T>, t: T) -> Vec<ChartMove<T>> {
    moves_inward(body, charts(body), t)
}

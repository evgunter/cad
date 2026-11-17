//! THROWAWAY: the stage-1 measurement switch of unit 4c (a ruled face
//! with a split side). It does not ship: stage 2 deletes this module and
//! every reader of it.
//!
//! `CURE` picks how the chart lane meshes a face that
//! `walk::splits_a_straight_side` routes there: 0 as on main; 1 no rows,
//! the plain constrained Delaunay step in the lane's own metric, and
//! refinement where the certificate refuses; 4 as 1 with the cylinder's
//! triangulation in its isometric metric; 2 no rows plus one held edge
//! per split side; 3 the walk of the face's two rows.

use std::sync::Mutex;
use std::sync::atomic::{AtomicU8, Ordering};

/// The cure in force (module docs).
pub static CURE: AtomicU8 = AtomicU8::new(0);

/// The pass budget cure 1 refines in.
pub const PASS_BUDGET: usize = 400;

/// One routed face's outcome under a cure.
#[derive(Clone, Debug)]
pub struct FaceStat {
    pub face: topo::FaceKey,
    pub cure: u8,
    /// Attempts the lane took (1 = no rebuild).
    pub passes: usize,
    /// The worst certificate reading of the exit attempt.
    pub worst: f64,
    pub triangles: usize,
    /// Interior points the patch holds (refinement's).
    pub interior: usize,
    pub refused: Option<String>,
}

pub static STATS: Mutex<Vec<FaceStat>> = Mutex::new(Vec::new());

/// The cure in force.
pub fn cure() -> u8 {
    CURE.load(Ordering::Relaxed)
}

/// Sets the cure.
pub fn set(c: u8) {
    CURE.store(c, Ordering::Relaxed);
}

/// Takes the stats recorded since the last take.
pub fn take() -> Vec<FaceStat> {
    std::mem::take(&mut *STATS.lock().unwrap_or_else(std::sync::PoisonError::into_inner))
}

pub(crate) fn note(s: FaceStat) {
    STATS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(s);
}

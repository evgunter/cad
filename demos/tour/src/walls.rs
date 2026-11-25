//! The shared **wall probe**: a frontier, run live.
//!
//! A scene's "wall" is a shape the model WANTED and the kernel would
//! not state. The rule (lily, `curvedcut::walls`) is that a
//! wall is never a comment: it is ATTEMPTED for real, every run, and
//! pinned by its own typed refusal — so a findings list cannot quietly
//! rot behind a frontier that moved.
//!
//! `lily::wall` is a one-line wrapper over [`wall`] that supplies its
//! scene name.
//!
//! A curved demo join (a boolean a person authors on everyday curved
//! parts) is probed by [`join_wall`] instead, which also names the
//! tracker rows the join stops at today ([`CURVED_JOIN_STOPS`]). Those
//! rows are the umbrella row's `blocked_on`, checked here, so a probe
//! re-pinned onto a new stop cannot leave the umbrella naming the old
//! one, and the tracker's band rule reaches the new stop.

/// Runs one wall probe. Three outcomes, and only one of them is
/// silent:
///
/// - the pinned refusal → narrate it, the findings-list entry holds;
/// - a DIFFERENT refusal → panic: the frontier moved underneath the
///   probe, and the findings list would quietly describe the wrong
///   thing;
/// - success → panic: the wall is gone, so the probe and its
///   findings entry must be retired (`retire` says what to do with
///   the scene once it is).
pub fn wall<T, E: core::fmt::Debug>(
    scene: &str,
    n: u32,
    what: &str,
    outcome: Result<T, E>,
    pinned: impl FnOnce(&E) -> bool,
    retire: &str,
) {
    probe(scene, n, what, outcome, pinned, retire, "");
}

/// Runs one **curved demo join's** wall probe: [`wall`], for a wall
/// with an entry in [`CURVED_JOIN_STOPS`]. The probe names the tracker
/// rows its pinned refusal waits on, and a wall that moves or retires
/// says that its entry, and the umbrella row's `blocked_on`, move with it.
///
/// # Panics
///
/// As [`wall`] does, and when `(scene, n)` has no entry in
/// [`CURVED_JOIN_STOPS`].
pub fn join_wall<T, E: core::fmt::Debug>(
    scene: &str,
    n: u32,
    what: &str,
    outcome: Result<T, E>,
    pinned: impl FnOnce(&E) -> bool,
    retire: &str,
) {
    let stop = CURVED_JOIN_STOPS
        .iter()
        .find(|s| s.scene == scene && s.wall == n)
        .unwrap_or_else(|| {
            panic!(
                "{scene} wall {n} ({what}) is a curved demo join with no entry in \
                 walls::CURVED_JOIN_STOPS: add it, and add its rows to {UMBRELLA}'s blocked_on"
            )
        });
    let note = format!(
        " It stops at {:?} (walls::CURVED_JOIN_STOPS): re-derive that entry, and \
         {UMBRELLA}'s table row and blocked_on, with the probe.",
        stop.rows
    );
    probe(scene, n, what, outcome, pinned, retire, &note);
}

fn probe<T, E: core::fmt::Debug>(
    scene: &str,
    n: u32,
    what: &str,
    outcome: Result<T, E>,
    pinned: impl FnOnce(&E) -> bool,
    retire: &str,
    note: &str,
) {
    match outcome {
        Err(e) if pinned(&e) => println!("   wall {n} — {what}: REFUSED TYPED, {e:?}"),
        Err(e) => panic!(
            "wall {n} ({what}) still refuses, but NOT with the refusal it pins \
             ({e:?}) — the wall MOVED. Re-derive this probe AND its findings-list \
             entry before trusting either.{note}"
        ),
        Ok(_) => panic!(
            "wall {n} ({what}) NO LONGER REFUSES — the {scene} can now say this. \
             Retire the probe and {retire}.{note}"
        ),
    }
}

/// The tracker row that keeps every curved demo join's stop in one
/// place, and whose `blocked_on` is the set of those stops.
pub const UMBRELLA: &str = "work/splice/curved-demo-joins-build.md";

/// One curved demo join's current stop: the scene and wall number its
/// probe narrates, and the ids of the tracker rows its pinned refusal
/// waits on, at every ε row the probe is measured at.
#[derive(Debug)]
pub struct JoinStop {
    /// The scene name the probe narrates.
    pub scene: &'static str,
    /// The wall's number in that scene.
    pub wall: u32,
    /// The `work/` item ids the pinned refusal waits on.
    pub rows: &'static [&'static str],
}

/// **The curved demo joins' stops**, one entry per join whose probe is
/// [`join_wall`]. The union of their rows is [`UMBRELLA`]'s
/// `blocked_on`, exactly (`the_umbrella_waits_on_every_join_stop`), so
/// the tracker's band rule (a parked row's blocker carries its band)
/// reaches each stop the moment a probe is re-pinned onto it.
pub const CURVED_JOIN_STOPS: &[JoinStop] = &[
    JoinStop {
        scene: "bottle",
        wall: 3,
        rows: &["a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band"],
    },
    JoinStop {
        scene: "bottle",
        wall: 4,
        rows: &["a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band"],
    },
    // The handle's union builds; the joined pot does not mesh, on the
    // section edge between the vessel's sphere and the handle's torus.
    JoinStop {
        scene: "teapot",
        wall: 2,
        rows: &[
            "cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm",
            "a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound",
        ],
    },
    // At ε 1e-6 and 1e-9 the union stops at containment; at 1e-12, in
    // the join (`teapot::wall3_pinned`).
    JoinStop {
        scene: "teapot",
        wall: 3,
        rows: &[
            "point-containments-ray-test-has-no-spline-arm",
            "a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung",
        ],
    },
    // At the tour's scalar the union gets past its torus × spline
    // sections and stops at the pierce pass. The K sweep's scalar, which
    // has no march lane, reads the section one refusal short, and that
    // is the probe's pin, not the scene's stop.
    JoinStop {
        scene: "lily",
        wall: 8,
        rows: &["lily-wall-8s-leaf-wall-lifts-no-closed-trim-at-the-pierce-pass"],
    },
];

#[cfg(test)]
mod tests {
    use super::{CURVED_JOIN_STOPS, UMBRELLA};
    use std::collections::BTreeSet;

    /// The umbrella's `blocked_on`, read off its front matter.
    fn umbrella_blocked_on() -> BTreeSet<String> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(UMBRELLA);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} reads: {e}", path.display()));
        let line = text
            .lines()
            .skip(1)
            .take_while(|l| *l != "---")
            .find_map(|l| l.strip_prefix("blocked_on:"))
            .unwrap_or_else(|| panic!("{UMBRELLA} has a blocked_on in its front matter"));
        line.trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|id| id.trim().to_owned())
            .filter(|id| !id.is_empty())
            .collect()
    }

    /// A probe re-pinned onto a new stop has to move the umbrella with
    /// it: the rows the join walls wait on and the umbrella's
    /// `blocked_on` are one set.
    #[test]
    fn the_umbrella_waits_on_every_join_stop() {
        let stops: BTreeSet<String> = CURVED_JOIN_STOPS
            .iter()
            .flat_map(|s| s.rows.iter().map(|r| (*r).to_owned()))
            .collect();
        assert_eq!(
            umbrella_blocked_on(),
            stops,
            "{UMBRELLA}'s blocked_on must be exactly the rows walls::CURVED_JOIN_STOPS names"
        );
    }

    #[test]
    fn each_join_wall_has_one_entry_and_a_stop() {
        let mut seen = BTreeSet::new();
        for s in CURVED_JOIN_STOPS {
            assert!(
                seen.insert((s.scene, s.wall)),
                "{} wall {} listed twice",
                s.scene,
                s.wall
            );
            assert!(
                !s.rows.is_empty(),
                "{} wall {} names no stop",
                s.scene,
                s.wall
            );
        }
    }
}

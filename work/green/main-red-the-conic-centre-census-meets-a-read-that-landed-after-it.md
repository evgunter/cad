---
id: main-red-the-conic-centre-census-meets-a-read-that-landed-after-it
kind: issue
title: main red: the lint job's conic-centre census fires on curve_face/end_root.rs, a circle-centre read that landed 26 minutes after the gate
status: open
opened: 2026-11-23
priority: P1
cost: E
---


Found by the ONEROUND orchestrator on 2026-11-23, from a lane's report that plain `cargo clippy -- -D warnings` errors on a clean main, read against the tree.

**What is red.** The lint job's step `clippy (--all-features)` ends in `scripts/conic_centre_census.py --check`, which PR 7510 (`oneround/conic-readers`, merged 21:47Z as `6cd3fb8129`) added: it holds every production read of a conic's centre doors at a pin per file (`scripts/gates/conic-centre-reads.counts`), and a read in a file with no pin is an error. PR 7463 (`heldrecords/elbow-ends`, merged 22:13Z as `09be7534f2`) added `crates/geom-brep/src/curve_face/end_root.rs`, whose circle arm reads `circle.frame().origin()`. The file has no pin. So on any run that selects geom-brep the census fails, and plain `cargo clippy --workspace … -- -D warnings` (without the workflow's `--force-warn clippy::disallowed_methods`) errors at the same line, since the read carries no `#[expect]`.

Not confirmed on a hosted run when filed: the lint job that read the new step green (job 138054735514, PR 7543, 22:25Z to 22:34Z) ran on a merge ref taken before PR 7463 landed. Confirmed by reading: the script's own selftest case "a read in a file with no pin" is red, the file is absent from the pins, and the read is there on main.

**Nobody's slip.** PR 7463's last hosted run predates the gate, and the gate's lane could not see a file that did not exist. It is the ordinary cost of a new ratchet landing beside a lane in flight.

**The fix, asked of the lane that built the gate** (`oneround/conic-census-main-red`): the census run on main's tip and every file it names; each new read converted through the conic's own readings where that is plainly right (in `end_root` the unit radial and tangent at a parameter are the conic's own directions, so no `p − c` is needed), or pinned at main's count with an `#[expect]` and a line on CONIC's row for a circle's centre read as a datum; and plain clippy made to agree with CI's block, so a lane's habitual spelling cannot pass or fail apart from CI's.

P1 (a red main on every PR whose lint job selects geom-brep).

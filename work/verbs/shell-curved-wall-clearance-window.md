---
id: shell-curved-wall-clearance-window
kind: issue
title: shell - the curved wall-clearance window (planar gate landed; curved residue waits on M10's clearance certificate)
status: parked
blocked_on: [M10-5]
opened: 2026-08-27
github: 1055
refs: [1048, 571]
---

## From GitHub issue 1055

Opened 2026-08-27; 0 comments.

**The shell verb's curved wall-clearance window.** Raised by the OFF-D PR-2 review (#1048, ordinal 82, MAJ-1) and adjudicated with Ev; this issue is the documented residue of that ruling.

**What is closed.** `topo::shell` now runs a closed-form planar gate: every pair of non-adjacent PLANAR faces whose outward normals are antiparallel and whose footprints overlap in projection must have at least `2t` of material between them, or the verb refuses `ShellError::WallClearance`. It is conservative in the #571 direction — projected-box overlap can report an overlap the true footprints do not have, and an ambiguous box comparison counts as overlapping — so it may over-refuse and cannot under-refuse.

**What is open.** The same collision class on CURVED walls. Two facing cylinder, cone, torus or fitted walls closer than `2t` still shell to a self-intersecting cavity, silently: the per-face reach margins are all positive (each face's own collapse threshold is fine), the resulting body VALIDATES at tier 3 (every per-face loop stays simple and consistently wound while the walls cross), and the volume is wrong. The measured instance of the class was a planar dumbbell — two blobs joined by a 0.4 neck, shelled at t = 0.3, returning Ok with volume 11.76 against a true erosion volume of 11.312 — and the planar gate now refuses that one; the curved analogue is unchanged.

**Why not a box-based curved gate.** Ruled out explicitly: a shelled tube's concentric walls overlap bounding boxes by construction, so a box test on curved pairs would refuse the verb's own acceptance fixtures. Over-refusing legitimate shells is not an acceptable trade here.

**What would close it.** The general clearance margin over a parameter box — M10's certificate machinery. A shell-specific special case is not wanted; when M10's clearance lands, the gate site in `crates/topo/src/shell.rs` (`wall_clearance`) cites this issue and is where the curved arm goes.

Until then the window is stated plainly in the verb's module docs and in #1048's body: **a curved thin neck can shell silently wrong.**

## Home

`crates/topo/src/shell.rs` is in VERBS' `paths:` territory; parked on M10-5, the E7 clearance unit that names this issue as its first consumer.

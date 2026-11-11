---
id: a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster
kind: issue
title: A source change that moves or adds a decide site skips its crate's recourse_roster census on the PR gate, so it reds main
status: open
opened: 2026-11-11
priority: P4
cost: E
refs: [6809]
---

Filed by PCERT's orchestrator (two main reds in one day).

## What

Each crate's `recourse_roster::every_decided_name_is_read_and_every_site_is_listed` (geom-core, geom-brep, editor-core; topo's `every_decided_name_has_its_door_and_every_site_is_read`) is in the slow set (`.config/nextest.toml`, the `ci` filter). The per-PR gate runs a slow test only when the PR adds or edits it, or names it in an added line. The census reads the crate's `src/`, though, so a PR that moves, adds or renames a decide site, and never touches the roster file, passes its gate and reds main at the next nightly.

On 2026-11-10/11 this happened three times:
- geom-brep, after PR 6755's forwarder rule: PCERT's PR 6531 (`spline_incidence_as` forwarding to a new helper) and sectarms' PR 6518 (a pose test's `decide(x, …)`). Fixed by PR 6809.
- geom-core: onehome's 6717 (the rigidity loop moved from `check_rigid` into `admitted_for_a_suite`). Fixed by PCERT's geom-core roster PR.

## Evidence: nightly 42965378060 (main `b2916558d9`), the same class beyond the rosters

Six more tree-wide source censuses in the slow set went red on main since nightly 42761779270 (`50cfcf499d`), each moved by a PR that never touched the census file. The shape is the same as the rosters': the census reads a crate's or the tree's `src/`, and the gate selects it only when the diff names it. `work/ciw/tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site.md` covers geom-core's; these reach editor-core and pncad-py too.

- geom-core `bounds_census`: PCERT's PR 6532 (`edge_nurbs.rs`'s `knot_lines`, a new sole `T: Bounds` door). Fixed by tint's PR 6818.
- geom-core `certified_endpoint_census`, three sites:
  - spiricarm's PR 6521 (`pcurve_cache.rs`'s `limb2_offset_sup` spindle grading, 2 reads);
  - spiricarm's PR 6509 (`props/quad/projected_edge.rs`, 1 asking read);
  - after the nightly, spiricarm's PR 6805 (`props/quad/rotational.rs`'s `piece_reach`).
  Dispositioned by PCERT's `pcert/nightly-slowset-reds`.
- geom-core `sym_rule_f_rows`: spiricarm's PR 6806 (a copysign mint site in `crates/mesh/src/walk.rs`, 0 → 1 between `eb319b9d^1` and `eb319b9d`).
- editor-core `node_standing`: onecopy's PR 6735 (`eval/mod.rs` 33 → 35 and `eval/parts.rs` 1 → 2 node-result reads, between `1ed46561^1` and `1ed46561`).
- pncad-py `prose_census` (`no_raise_site_composes_a_debug_rendering_into_its_message`): reads' PR 6555 (`crates/pncad-py/src/py/select.rs`'s `message`, a `Debug` rendering, at `a5e6a785`).

The fix above generalises: select every slow-set census whose walk reads a crate's `src/` when the diff touches that `src/`, not only the rosters.

## Fix

Have `scripts/ci-filter.py` select a crate's roster row whenever the diff touches that crate's `src/`. Each roster takes 2–8 s, so this costs one test per touched crate. The alternative is to move the rosters into the fast set if they can be made to fit the bar.

## Evidence: a slow-set behaviour row that guards another program's change (PCERT, 2026-11-11)

The same gap reaches rows that are not censuses.
`a_bending_carrier_on_a_spline_wall_takes_the_general_lane::a_stated_wrong_general_row_keeps_its_term_{f64,interval}`
guard PCERT's `a-minted-refusals-provenance-is-guarded-only-by-its-doc`
and sat in the slow set. PR 6815 changed
`geom_brep::certify`'s stated-image route, never named them, and
turned them red on main; only the nightly saw it (filed and fixed as
PCERT's `main-red-the-stated-wrong-general-rows-refuse-chart-image-unavailable-in-the-slow-set`).
This row's fix would not have run them either: 6815 touched
geom-brep's `src/`, and the rows are topo's, a downstream crate's. A
behaviour row in the slow set that guards an upstream crate's change
needs selecting by the paths it guards, not by its own crate. (The fix
moved the f64 row to the fast set, 0.352 s hosted; the Interval row,
0.467 s, stays slow.)


The same PR showed a second hole in "the PR that edits a slow test runs
it". `scripts/slow_set.py`'s `is_new` reads a row as edited only where an
added `.rs` line spells its leaf name. The fix rewrote the rows' shared
generic body (`a_stated_wrong_general_row_keeps_its_term::<T>`) and left
the two `#[test]` wrappers alone, so its first gated run (run
43019583177) selected 0 slow tests. Naming the rows in the generic's doc
made the gate select them.

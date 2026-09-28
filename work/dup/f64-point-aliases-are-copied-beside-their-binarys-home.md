---
id: f64-point-aliases-are-copied-beside-their-binarys-home
kind: issue
title: An f64 point constructor that lifts nothing (fn p2(x, y) -> Point2<f64> { Point2::new(x, y) } and its p3/v3/pt kin) is spelled at 179 fn definitions, most in test binaries that already hold one
status: open
opened: 2026-09-26
priority: P4
cost: D
---



## Finding

Measured by the S-DUP lane that folded the lifting class
(`coordinates-lifted-into-a-point-are-spelled-per-component`), at
`360eb7320`. That row's class LIFTS `f64` coordinates into a scalar
`T`; this one lifts nothing: a function or closure whose whole body is
`Point2::new(x, y)` (or `Point3`/`Vec3`/`Vec2` with its own arguments
passed straight through). It is the one-line rename of a `const fn`
constructor, spelled per suite.

**179 `fn` definitions and 14 closures.** Instrument: every `fn` taking
two or three `f64` and returning a `Point`/`Vec` at `f64`, over
`git grep` with no path argument, with the body on the next line
compared against `<Type>::new(<its own arguments>)`; 185 matched the
signature, 179 have that body (the other six compute something). The
closures: the same body behind `|x: f64, y: f64| ...`.

By test binary, with the binary's existing home where it has one (the
counts include the homes themselves):

- `sweep` (`tests/all.rs`): 84 `Point2` + 4 `Point3` + 4 `Vec3`. The
  binary holds **three** `f64` `p2`s already — `revolve_common::p2`,
  `mate2_common::p2`, and a private one in `common/cone_nappe.rs` —
  and `common/mod.rs`'s routing list calls `revolve_common` "the place
  `p2` and `eps` presently live despite belonging to no verb". So the
  fold here has a routing step first: which home.
- `profile` (`tests/all.rs`): 34 `Point2`, of which 33 copy
  `common::p2` (the binary's one home). None of the 33 copying suites
  glob-imports `common`, so each fold is a delete plus an import.
- `topo`: 11 in `src` test modules (eleven files, four of them
  `review_m1_*`), and 9 `Point3` + 7 `Vec3` in `tests/`.
- `mesh`: 14 in `tests/`, beside `tests/common/mod.rs::p2` and
  `common/witness_bodies.rs`'s private `p3`/`v3`;
  `r1_probe_bool_route.rs` and `r1_probe_hash.rs` copy the home's `p2`.
- `stl`: `review_m3_pr55_e2e.rs` copies `tests/common/mod.rs::p2`.
- `editor-core` 3, `viewer` 2, `profile/src` 2, `mesh/src` 1,
  `geom-brep` 2.

## Why filed rather than folded

Size alone. The fold is mechanical per site, the way
`common::interval` was, but it is 185 matching signatures (179 of them
pass-through) in 169 files across nine crates — well past one PR
beside the class it was measured from. Method item 6 still applies
site by site when it is taken: `p2(1.0, 2.0)` against
`Point2::new(1.0, 2.0)` is a readability call, and a suite that
prefers the constructor may inline rather than import.

## Why this row is on this slate

Its member files sit on twelve programs' ground (`work.py territory
--files -` over the files carrying a matching signature, 2026-09-26):
`tint` and `tcost` on 154 of the 169, `paths` on 36, `tess` on 11, then
`shell`, `vdoc`, `exch`, `chrome`, `reach`, `curved`, `chart` and
`atrest` on one to three each. No one of those owns the class, and one
thing spelled many times across all of them is this program's charter
(method item 14); any of them may claim a crate's share by `git mv`
of a split row.

## Blind spots

The instrument reads the body on the line after the signature, so a
body that rustfmt splits across lines is counted as "computes
something" rather than as a pass-through. It sees only parameters
typed `f64` in the signature; a generic `T` helper used only at `f64`
is outside it. Closures are counted only when their parameters are
annotated `f64`. A binding of the constructor itself —
`let p2 = Point2::<f64>::new;` (`sweep/tests/bitdump.rs`,
`must_carry_rule.rs`), `let pt = Point3::new;` — is not a definition
and is not counted; those name the constructor and hold no body.

## `sweep`: closed (2026-09-28)

**Inlined.** Ev (2026-09-28) leans towards deleting these wrappers and
spelling the constructor at the call site, shared homes included, and
keeping a helper only where its shortness is the readability.
Nothing in `crates/sweep` qualifies. The longest run of helper calls in
any one block is 13 points (`revolve_determinism.rs`), and most are
four-to-six-point polygons. So **no helper is kept**: every member is
deleted and each call site spells `Point2::new` / `Point3::new` /
`Vec3::new`. That includes the binary's four homes:
`revolve_common::p2`, `mate2_common::p2`, `shell7_common::p2` (a fourth
home, which this row did not name) and `common/cone_nappe.rs`'s private
one.

**Census, re-taken at the merge base `c1b202b2e`**, over every tracked
`.rs` under `crates/sweep` (src included; src holds no member). The
instrument is a regex over whole file text that tolerates multi-line
bodies. It matches a `fn`, a `let` closure or a `let` binding of the
constructor, and accepts it when the body is exactly
`[path::]Type[::<f64>]::new(<its own parameters, in order>)`.

- **92 `fn`s**: 84 `Point2` (one path-qualified, `geom_core::Point2`
  in `bool1_r2_probes`), 4 `Point3`, 4 `Vec3`. That is this row's 84 +
  4 + 4.
- **2 closures**: `rim_of_r1_probes`'s `p2`, and `m5_pr12_refusals`'s
  `n`.
- **10 bindings** of the constructor in 8 files.
- A second instrument read the line after every `fn … -> {Point,Vec}{2,3}<f64> {`,
  normalised the digits, and found the same 92 bodies, with the rest
  computing something.

**The hit list**, every one deleted with its call sites rewritten:

`band_declared_cusp_contacts.rs` (fn `p2`);
`band_subdivided_side_walls.rs` (fn `p2`); `bitdump.rs` (binding
`p2`); `blend4_concave_fillet.rs` (fn `p`, fn `v`);
`blend_recourse_followability.rs` (fn `p2`); `bool1_fix_pass.rs` (fn
`p2`); `bool1_r1_probes.rs` (fn `p2`); `bool1_r2_probes.rs` (fn
`p2`); `census_containment_cause.rs` (fn `p2`); `cert_m2r1_passes.rs`
(fn `p2`); `common/cone_nappe.rs` (fn `p2`); `curved_mergedoor.rs`
(fn `p2`); `extrude_acceptance.rs` (fn `p2`); `fillet_h6_cap_rim.rs`
(fn `p2`); `fillet_h7_transverse_cap.rs` (fn `p2`);
`m5_pr12_battery.rs` (fn `p2`); `m5_pr12_blends.rs` (fn `p`, fn `v`);
`m5_pr12_die.rs` (fn `p2`); `m5_pr12_refusals.rs` (fn `p2`, closure
`n`); `m5_pr5_tilted_cut.rs` (fn `p2`); `m5_pr6_pcurves.rs` (fn
`p2`); `m5_pr9_boss_union.rs` (fn `p2`); `m5_pr9_sector2.rs` (fn
`p2`); `m5_s12_curved_ops.rs` (fn `p2`); `m5_s13_pips.rs` (fn `p2`);
`m6_surgery.rs` (fn `p2`); `m9_2b_r2_probes.rs` (fn `p2`);
`m9_3_wall_door.rs` (fn `p2`); `m9_3_zip.rs` (fn `p2`);
`mate2_common/mod.rs` (fn `p2`); `must_carry_rule.rs` (binding `p2`);
`offd2_r1_probes.rs` (fn `p2`); `offd_r1_probes.rs` (fn `p2`);
`p1b_r1_probes.rs` (fn `p2`); `pcurve_p1b_r2_probes.rs` (fn `p2`);
`pis_arc_capped_poses.rs` (binding `p`); `r1_mate3_probes.rs` (fn
`p2`); `r1_probes_m9_3.rs` (fn `p2`); `r2_mate3_probes.rs` (fn `p2`);
`r2_rim_corpus_probes.rs` (fn `p2`); `review_blend1_r1_probes.rs` (fn
`p2`); `review_blend4_r2_probes.rs` (fn `p`, fn `v`);
`review_blend_e2_r1_probes.rs` (fn `p2`);
`review_blend_k_rk_probes.rs` (binding `p2`);
`review_d2_adv_probes.rs` (fn `p2`); `review_fillet_e2_probes.rs` (fn
`p2`); `review_fillet_h6_r1_probes.rs` (binding `p2` x3);
`review_m2_pr4.rs` (fn `p2`); `review_m3_pr1_sweep.rs` (fn `p2`);
`review_m5_pr9_boss_probe.rs` (fn `p2`);
`review_m6_surgery_probes.rs` (fn `p2`);
`review_must_carry_rule_r1_probes.rs` (binding `p2` x2);
`review_must_carry_rule_r2_probes.rs` (binding `p2`);
`review_pr12_probes.rs` (fn `p2`); `review_s12_adv.rs` (fn `p2`);
`review_verbs_rim_lever_probes.rs` (fn `p2`); `revolve_common/mod.rs`
(fn `p2`); `rim_of_r1_probes.rs` (closure `p2`);
`s49_census_jurisdiction.rs` (fn `p2`); `sf2a_r1.rs` (fn `p2`);
`sf2a_r2_probes.rs` (fn `p2`); `sf2b_axial.rs` (fn `p2`);
`sf2b_head.rs` (fn `p2`); `sf2b_r1_probes.rs` (fn `p2`);
`sf2b_r2_probes.rs` (fn `p2`); `shell10_r2_dump.rs` (fn `p2`);
`shell5_r1_dump.rs` (fn `p2`); `shell5_r1_probes.rs` (fn `p2`);
`shell5_r2_probes.rs` (fn `p2`); `shell6_r1_probes.rs` (fn `p2`);
`shell6_r2_probes.rs` (fn `p2`); `shell7_common.rs` (fn `p2`);
`shell7_dump.rs` (fn `p2`); `shell7_r1_diff.rs` (fn `p2`);
`shell9_r1_probes.rs` (fn `p2`); `shellfix1_bitdump.rs` (fn `p2`);
`shellfix1_r1_probes.rs` (fn `p2`); `spiric_rim.rs` (fn `p2`);
`torax_axial.rs` (fn `p2`); `verbs_arms1_r1_probes.rs` (fn `p2`);
`verbs_arms2_arms.rs` (fn `p3`, fn `v3`); `verbs_cylcyl_probe.rs` (fn
`p2`); `verbs_cylcylb_r1_blinded_probes.rs` (fn `p2`);
`verbs_ga_r2_probes.rs` (fn `p2`); `verbs_gate_r1_probes.rs` (fn
`p2`); `verbs_germarms.rs` (fn `p2`); `verbs_germarms_r1_probes.rs`
(fn `p2`); `verbs_offd.rs` (fn `p2`); `verbs_pierce.rs` (fn `p2`);
`verbs_pierce_r1_probes.rs` (fn `p2`); `verbs_pierce_r2_probes.rs`
(fn `p2`); `verbs_rim_closed_lever.rs` (fn `p2`);
`verbs_rim_r1_probes.rs` (fn `p2`); `verbs_shell.rs` (fn `p2`);
`verbs_shell_r2_probes.rs` (fn `p2`); `verbs_shell_r2b.rs` (fn `p2`).

1773 call sites were rewritten, in the files above and in the suites
that imported a home's `p2`. Nine imported it by name (`latitude_seam`,
`m5_s10_face_sense`, `m5_s11_concave_sense`, `mass_props`,
`mate7a_torus_rest`, `review_m2_pr7`, `review_s11_adv`,
`shell9_r2_dump`, `shell9_r2_probes`). The rest took it through a
`revolve_common::*`, `mate2_common::*` or `shell7_common::*` glob. Two
of the deleted `p2`s had no caller at all (`m5_pr12_die`,
`m6_surgery`).

**Not members, left standing**, because each one computes, converts or
takes a different type:

- `common/interval.rs`'s `p2`/`v2`/`p3`/`v3`, which lift into
  `Interval`, and `k_report`'s `p2`, which lifts into `Probe`;
- the Probe and `iv` closures named `p2`/`p`/`v` (`must_carry_rule`'s
  `a_filleted_block_…` row, `spiric_rim`'s interval rows,
  `review_must_carry_rule_r1_probes`);
- the bulge-vertex `v(x, y, bulge) -> (Point2, f64)` helpers, which
  return a tuple;
- `verbs_shell`'s `v(w, d, h) -> f64`.

**Hazards the rewrite had to step around.** A file-wide rename is not
scope-aware. In two places a different function of the same name
shadowed the f64 helper inside one scope:

- `m5_pr5_tilted_cut`'s two interval rows import `common::interval::p2`
  inside their bodies, and they were excluded from the rewrite;
- `must_carry_rule`'s `probe`-gated row binds its own lifting `p2`
  closure. The rename reached its eight calls and they were restored
  by hand. That row compiles only under `--features probe`, so a
  default-feature build could not have caught it: `cargo clippy -p
  sweep --all-targets --features probe` did.

**Blind spots.** The instrument cannot see a helper assembled by a
macro (`macro_rules!` over `crates/sweep`: none), a `const`/`static`
fn pointer (none), or a helper generic in `T` and used only at `f64`
(none with a pass-through body). It also cannot see a closure that
is passed inline rather than bound by `let`. That last shape is a
`.map(|&(x, y)| …)` adapter, not a helper, and is out of the class.

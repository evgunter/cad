---
id: sweep-suites-import-fixtures-from-other-suites
kind: issue
title: Twenty-two sweep suites import a fixture from another suite rather than from tests/common
status: closed
opened: 2026-09-28
priority: P4
cost: M
closed: 2026-09-28
---


## Finding

Measured by the `dup/b8-a` lane at merge base `c1b202b2e`, while it
closed `one-suite-owns-another-suites-certified-fixture-group`: that
row's shape (a suite reading its fixture out of another SUITE) is a
class in `crates/sweep/tests`, not one site.

Ev ruled (2026-09-27, `two-rules-disagree-on-when-a-fixture-leaves-a-suite`)
that `tests/common`'s narrowest-home rule governs. By that rule an item
two suites share lives in `tests/common`, and a suite that exports one
to another suite owns the other's fixture.

**Owners: seven suites. Consumers: twenty-two.**

- `verbs_shell` (`vessel`, `hollow_box`, `outer_and_void`,
  `two_void_box`, `tube`, `prism`, `cut`, `roles_by_solid`, `v`) ←
  `shell5_r1_probes`, `shell5_r2_probes`, `shell8_dump`,
  `shell8_multi_solid`, `shell8_r1_probes`, `shell8_r2_probes`,
  `shell9_r1_probes`, `shell9_r2_dump`, `shell9_r2_probes`,
  `shell9_rows`, `shell10_r1_probes`, `shell10_r2_cost`,
  `shell10_r2_dump`, `shell10_r2_probes`, `shell10_scoped_walks`;
- `shell9_rows` (`rows`, `print_rows`) ← `shell9_probe`,
  `shell10_r2_dump`, `shell10_scoped_walks`, `shell5_r1_dump`,
  `shell7_dump`, `shell8_dump`;
- `cert_m2r1_passes::corpus` ← `lane1_r2_probes`;
- `bitdump::dump` ← `review_arms2_r1_probes`;
- `review_contact_edge_must_carry_r1_probes` ← `contact_edge_must_carry`
  (inside one row);
- `torax_axial` (`torus_barrel`, `torus_belly`) and `spiric_rim`
  (`vessel_cavity`) ← `pis_arc_capped_poses`, by inline path.

**Instruments.** The first is a `git grep` of
`use (crate|super)::<module>::` over `crates/sweep/tests`, excluding
`common::` and the `*_common` homes. Its blind spot is a path spelled
inline without a `use`. A second pass over
`(crate|super)::<module>::<item>` outside comments and `use` lines
found the three inline-only consumers (`pis_arc_capped_poses`,
`shell5_r1_dump`, `shell7_dump`). Neither pass can see a glob
re-export through a third module (none found under `crates/sweep/tests`).

## Why filed rather than folded

Size. `verbs_shell`'s group alone is nine items read by fifteen
suites, and `shell9_rows` is a row table as well as a printer. Each
group needs the treatment `common::sphere_recut` got: decide what stays
together, write the new module's not-absorbed list, and plant it. That
is a unit, not a drive-by beside the one it was found from. Method
item 6 applies at each site. `v` in `verbs_shell` is a closed-form
volume, which `common/oracles.rs`'s rule may route differently from
the bodies.

## Why this row is on this slate

The ground is S-TCOST's and S-TINT's (`crates/sweep/tests/*`). The
question is where a shared fixture lives, which is this program's
subject, as the row it was found from was.

## Closed (2026-09-28, `dup/b9-b`)

**Census re-taken at merge base `2adacbc0e`** (after #3340), with both
of this row's instruments and a third aimed at their shared gap.

- *Instrument 1* — `git grep -nE '(crate|super)::<module>::'` for every
  suite file's module name over `crates/sweep/tests`, excluding the
  suite itself, `all.rs`, and the `shell7_common` / `shell8_common`
  family homes. It matches `use` lines and inline paths alike, so it
  subsumes the row's second pass.
- *Instrument 2* — the gap both name-anchored passes share: a module
  imported WHOLE (`use crate::<suite>;`, `use crate::<suite> as x;`,
  `use crate::{<suite>, …}`) and then read as `x::item`. It found one
  consumer the row did not: **`cert_m2r1_head` reads
  `cert_m2r1_passes::{corpus, f64_only_corpus}` through
  `use crate::cert_m2r1_passes as passes;`**.
- No `pub use` re-export exists under `crates/sweep/tests`
  (`git grep -n 'pub use'`), so the glob-through-a-third-module blind
  spot is empty there.

The re-take moved the population: **eight owners, 25 consumer files**
(the row said seven and 22). The members it did not name:
`pis_arc_capped_poses::poses` ← `pis_cut_cavity` (an eighth owner),
`spiric_rim::vessel_cavity` ← `contfp_reads_arcs_on_their_carriers` as
well as `pis_arc_capped_poses`, and `cert_m2r1_head` above.

**Where each group went** (every importer re-routed, every exporting
item deleted from its suite):

| group | home |
| --- | --- |
| `verbs_shell`: `vessel`, `tube`, `hollow_box`, `two_void_box`, `outer_and_void`, `roles_by_solid` | new `common::shell_operands` |
| `verbs_shell::v` | `common::oracles::box_volume` (imported as `v`) |
| `verbs_shell::prism` | deleted; its callers use `sweep::test_support::prism` over `corners` |
| `verbs_shell::cut` | deleted; its callers use `common::cavity::cut` |
| `shell9_rows::{rows, print_rows}` | new `common::pcurve_rows` (`shell9_rows` keeps its row) |
| `cert_m2r1_passes::{corpus, f64_only_corpus}` | new `common::cert_corpus` |
| `bitdump::dump` (and its `dump_dir` / `save`, which `review_arms2_r1_probes` re-spelled inline) | new `common::bitdump` |
| `review_contact_edge_must_carry_r1_probes::skewed_cavity_edges` | `common::cavity` |
| `review_contact_edge_must_carry_r1_probes::chart_contact_edges` | new `common::contact_edges` |
| `torax_axial::{torus_barrel, torus_belly}`, `spiric_rim::{vessel_quarter, vessel_cavity}` | new `common::torus_walls` |
| `pis_arc_capped_poses::poses` | new `common::poses`, beside `torax_pose` |

**Copies of the moved items, folded** (a copy of something `common`
holds must say why at the copy or go): `intrinsic_edges` /
`tangent_intersections` (5 named, 3 inline counts across the
must-carry suites); `shell5_r1_dump`'s, `offd2_r1_probes`',
`shellfix1_bitdump`'s and `cert_corpus`'s vessel and tube;
`shell7_common::drum`, `shell10_r2_dump::drum` and `sf2b_axial`'s inline
drum (each is `vessel`); `offd2_r1_probes`' inline hollow box;
`shell7_dump`'s `torus_barrel` / `torus_belly`; the tan(θ/4)
arc-about-centre `bulge` (six byte-identical copies, now
`common::bulge`); the torax re-pose (four inline spellings, now
`common::poses::torax_pose`). The survivors that are different things
under a near name carry a ``NOT `common::`` marker and a line in the
module's list.

**Not members**, read at each site: `shell7_common` and
`shell8_common` (family homes, excluded by this row's own instrument;
filed as `work/dup/sweep-family-homes-sit-outside-the-tests-common-routing-list.md`).
No exporting suite gave a reason for keeping its item that still held:
`cert_m2r1_passes`' "compiles at the merge base f3c035579" and
`shell7_dump` / `shell5_r1_dump`'s "kept free of every symbol the unit
adds" described differentials of units merged long since, and both
files already read later symbols.

**Filed, not folded** (a wider class, never imported across a suite):
`work/fixture/sweep-revolve-about-y-helper-and-its-fixtures-spelled-per-suite.md`
and `work/helper/sweep-planar-cap-finders-spelled-per-suite.md`.

**Drive-bys in files this unit was already in**: `verbs_cylcyl_probe`'s
`crossing_pair_without_edge_events` was `pub(crate)` with no importer
and is private; and `contact_edge_must_carry`'s slim-wedge row asserted
`chart_contact_edges(..) >= 4` for "the four corner arcs" on a body
that carries six chart-described edges (the vent's two among them) —
measured by planting the count down by two (row stayed green) and by
three (row red) — so it now pins `== 6`, as its review twin already
did.

After the move both instruments return nothing outside the family
homes.

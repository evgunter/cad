---
id: debug-only-assert-euler-postcondition-is-on-no-row
kind: issue
title: Fifteen gated statements in topo name no pinned spelling, so the debug-only gate says nothing about them
status: closed
opened: 2026-09-06
refs: [debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call, debug-only-bit-witness-callers-are-on-no-row]
branch: gates/debug-only-topo-class
pr: 2066
closed: 2026-09-06
---


## Finding

Found by the `gates/statement-attribute-items` lane while measuring the
`ArenaDelta` rows it added to
`scripts/gates/bit-identity-debug-only.sh`, and widened to its class by
that PR's style review.

**The class.** A row pins the uses of its SPELLINGS, so a
`#[cfg(debug_assertions)]` statement whose text names no spelling on any
row is invisible to the gate however debug-only the mechanism it belongs
to is. The `topo` arena-delta rows pin `ArenaDelta`; fifteen gated
statements in the same files, in the same mechanism, name it nowhere.

Every one of them is a live hazard on the same terms the rows exist for:
this workspace's `[profile.release]` keeps debug assertions on, so
dropping any of these attributes compiles and passes every test here, and
the first build that refuses it is a consumer's after publish.

**One site, the postcondition call.** `assert_euler_postcondition`
(`crates/topo/src/euler.rs:2238`, itself a `#[cfg(debug_assertions)]`
`pub(crate) fn`) is the second spelling of the arena-delta mechanism and
is on no row. Sixteen of its seventeen uses cost nothing, because the
statement naming it also names `ArenaDelta` and the `ArenaDelta` row
places it. One does not:

- `crates/topo/src/boolean/voids.rs:314` —
  `dst.assert_euler_postcondition(before, transplant, "insert_void");`
  passes a binding, so the call names no `ArenaDelta` and the
  `boolean/voids.rs` row (which pins the struct literal at `:291`) says
  nothing about it.

**Fourteen sites, the counts taken before the mutation.** Each operator
opens with a gated `let before = self.arena_counts();` whose text names
neither spelling:

- `crates/topo/src/euler.rs:1054`, `:1219`, `:1364`
- `crates/topo/src/euler_ring.rs:406`, `:612`, `:744`, `:927`
- `crates/topo/src/euler_kill.rs:417`, `:558`, `:765`, `:1034`
- `crates/topo/src/null.rs:209`
- `crates/topo/src/split.rs:152`
- `crates/topo/src/movefac.rs:67`

`arena_counts` is not incidentally debug-only either: it lives in
`test_support_impl`, which `crates/topo/src/lib.rs:228` declares under
`#[cfg(any(debug_assertions, test, feature = "test-support"))]`. Drop one
of these fourteen attributes and a consumer's release build — no
`debug_assertions`, no `test`, no `test-support` — stops compiling, which
is exactly what the rows are for.

Per-file `assert_euler_postcondition` use counts as measured on this tree
(the code-only view, at identifier boundaries): `euler.rs` 4,
`euler_ring.rs` 4, `euler_kill.rs` 4, `movefac.rs` 2, `null.rs` 1,
`split.rs` 1, `boolean/voids.rs` 1.

## The candidate fix

**For the class, not for the one site.** Adding
`assert_euler_postcondition` alone is a half-fix: it closes
`voids.rs:314` and leaves the fourteen `arena_counts` statements exactly
as blind as they are now. Both spellings go on each of the seven `topo`
rows, and the pins are re-taken over the pair.

## What it costs

The self-test is **quadratic in the subject count**: each subject
contributes a fixed set of cases, and every case runs the gate as a
subprocess over EVERY subject's planted file, so wall clock goes as
(cases per subject × subjects) × subjects. The seven `ArenaDelta` rows
alone took it from 15 s to 72 s. This row proposes seven more spellings
on top of that — spellings add cases linearly rather than quadratically,
so the marginal cost is small beside the row cost already paid, but it is
paid on a base that is already the largest in the directory. One reading,
on one box; nothing re-takes it.

## What the sweep could not match

Eight sites name `assert_euler_postcondition` in prose or in a string
literal rather than calling it, and every one is inside a `cfg(test)`
module, so no row would reach them and none is a use:

- `crates/topo/src/review_d18_probes.rs:258`, `:305`
- `crates/topo/src/review_m1_pr2/release_corruption.rs:277`
- `crates/topo/src/review_m1_pr5_internal.rs:336`, `:369`, `:383`, `:396`
- `crates/topo/src/source_walk.rs:347`

A `topo` file that STARTS calling either spelling is caught by nothing —
that is the gate's KNOWN GAP 7 (a subject is a file), not this row.

## Landed

`scripts/gates/bit-identity-debug-only.sh`: the candidate fix as stated,
the same class closed in the one other rowed file that carried it, and
— after the style review — the reader switched to
`gate_rust_code --skip-cfg-test`, which is what lets the fourth
arena-delta spelling be pinned at all.

**The reading changed, so two pins moved.** The gate read `#[cfg(test)]`
modules as live code. It asks one question — what a CONSUMER'S RELEASE
BUILD compiles — and a test module is compiled by `cargo test` and by
nothing a consumer runs, so reading one can only report a violation that
is not one. `lib.sh` has carried the predicate since PR 2058
(`GATE_CFG_TEST_RE`, `lib.sh:236`; `gate_rust_code --skip-cfg-test`,
`lib.sh:405`) and this gate simply read without it. Under the skip:

- `crates/mesh/src/curved.rs` **13 → 5** and
  `crates/mesh/src/tessellate.rs` **8 → 2**: their census helpers are
  read by test rows beside their live callers, and those reads were
  being counted as production uses. A pin that moves because the
  reading got righter is re-taken, never defended.
- `ArenaCounts` becomes servable. `euler_kill.rs` names it three times
  (`:1121`, `:1331`, `:1982`) inside a `#[cfg(test)] mod tests`; with
  those gone it has no ungated use anywhere, so `euler.rs`'s row can
  pin it — which is what the pre-review Landed section wrongly called a
  spelling this reader could not serve. It was a spelling this reader
  could not serve *while it read test modules*, which is a choice and
  not a limit.

No fixture depended on test-module code being read — nothing under
`scripts/gates/` plants a `#[cfg(test)]` module — so the switch was
mechanical, and the whole self-test passes over it.

**The pins.** Six `topo` rows read
`ArenaDelta|assert_euler_postcondition|arena_counts`; `euler.rs` reads
that plus `ArenaCounts`. Per file, per spelling, measured with the gate's
own matcher over its code-only view (one spelling at a time, pinned 0,
reading the count back out of the pin diagnosis) — never `grep -c`:

| file | `ArenaDelta` | `assert_euler_postcondition` | `arena_counts` | `ArenaCounts` | row pin |
| --- | --- | --- | --- | --- | --- |
| `euler.rs` | 10 | 4 | 4 | 3 | 21 |
| `euler_ring.rs` | 10 | 4 | 4 | — | 17 |
| `euler_kill.rs` | 9 | 4 | 4 | — | 17 |
| `null.rs` | 3 | 1 | 1 | — | 5 |
| `split.rs` | 3 | 1 | 1 | — | 5 |
| `boolean/voids.rs` | 1 | 1 | 2 | — | 4 |
| `movefac.rs` | 4 | 2 | 1 | — | 6 |

`source.rs`, whose row grew the three witnesses: `bit_identity::` 1,
`eq_bits` 1, `plane_bits_witness` 1, `vec3_bits_witness` 1,
`bits_witness` 3; pin **6**.

**The pin is not the column sum, in three rows.** The reader emits one
`USE` per delimiter-cut PIECE, so a piece naming two spellings is one
use and not two: `euler_ring.rs` (18 → 17) and `movefac.rs` (7 → 6) each
carry a one-line `self.assert_euler_postcondition(before, ArenaDelta`
head, and `source.rs` (7 → 6) carries
`geom_core::bit_identity::eq_bits(a, b)` at `:214`, one piece naming
both channel spellings. The other five rows sum. The per-spelling
`assert_euler_postcondition` column reproduces the Finding's measurement
exactly.

**The finding, and the fix, on the dropped attributes.** In a scratch
copy of the thirteen subjects (`--root`), one attribute deleted at a
time and nothing else changed. Before this change the gate printed OK
over every one of them, unchanged; after it, each reds by name with the
"outside any `cfg(debug_assertions)` item" diagnosis:

- `crates/topo/src/boolean/voids.rs:313` →
  `voids.rs:313: dst.assert_euler_postcondition(before, transplant, "")`
- `crates/topo/src/euler.rs:1053` →
  `euler.rs:1053: let before = self.arena_counts()`
- `crates/topo/src/source.rs:172` → `source.rs:172:` the
  `plane_bits_witness` head, plus the `bits_witness(&pairs)` call below
  it
- `crates/topo/src/euler.rs:246` →
  `euler.rs:246: use crate::test_support_impl::ArenaCounts` — the site
  the `--skip-cfg-test` switch bought

Live tree: 68 uses scanned before this branch, **94** after, green.

**The eight prose/literal sites, re-derived.** `grep -rn
assert_euler_postcondition crates/topo/src` outside the seven rowed
files returns exactly the eight the Finding lists (`review_d18_probes.rs
:258 :305`, `review_m1_pr2/release_corruption.rs:277`,
`review_m1_pr5_internal.rs :336 :369 :383 :396`, `source_walk.rs:347`),
and `gate_rust_code` over each of those four files returns NO
`assert_euler_postcondition` at all — every one is a doc comment or a
string body, stripped by the code-only view. All four modules are
declared `#[cfg(test)]` in `crates/topo/src/lib.rs` (`:164`, `:191`,
`:199`, `:205`), so the reader now skips them twice over. None is a
code-view use; the list stands.

**The class sweep over `crates/topo/src`.** Every
`#[cfg(debug_assertions)]` in the crate, and what names no spelling on
any row after this change:

- `source.rs:172`, `:195`, `:211` — `plane_bits_witness`,
  `vec3_bits_witness`, `bits_witness`: the BIT CHANNEL mechanism, in a
  file that already has a row, so **closed here**. Pin 1 → 6.
- `euler.rs:246` — `use crate::test_support_impl::ArenaCounts;`: the
  arena-delta mechanism, in a rowed file, **closed here** under the
  `--skip-cfg-test` switch above. Pin 18 → 21.
- `boolean/plane_eq.rs:173` and `merge_faces.rs:1006` — the BIT CHANNEL
  again, both statement-position attributes over an `if let` calling
  `plane_bits_witness` (and `vec3_bits_witness`). Files with no row, so
  out of this unit by the fence:
  `debug-only-bit-witness-callers-are-on-no-row`, which now carries the
  `cargo check --release` reproduction of the consumer-build break.
- `revert.rs:258`, `attach.rs:92`, `attach.rs:331` — the TIER-1 VALIDITY
  postcondition, `#[cfg(debug_assertions)]` over a `debug_assert_eq!(
  crate::validate::validate(…), Ok(()))`. Not this mechanism, and not a
  hazard: `validate` is ordinary API and `debug_assert_eq!` compiles
  itself out, so the attribute is redundant rather than load-bearing. No
  row.
- `review_m1_pr2/release_corruption.rs:289` — `#[test]
  #[cfg(debug_assertions)]` selecting a test by profile, inside a
  `#[cfg(test)]` module the reader now skips. Not a mechanism. No row.
- `review_d18.rs` (eleven sites), `release_corruption.rs:82`, `:239` —
  `cfg(not(debug_assertions))`, release-only items, which this gate
  reads as not-a-gate by KNOWN GAP 1. No row.
- `lib.rs:228` — `cfg(any(debug_assertions, test, feature =
  "test-support"))` mounting `test_support_impl`. Not a debug-only gate,
  KNOWN GAP 1. No row.

`lib.rs:255`/`:262`, `fixtures.rs:117`/`:126` and
`splitting/reassembly.rs:281` name `arena_counts`/`ArenaCounts` in files
with no row; `fixtures` and `reassembly` are `#[cfg(test)]` modules and
`test_support` is behind the `any(…)` door, so none is a row this reader
would serve. KNOWN GAP 7 for the shape.

**Fixtures.** Every case the harness already runs per subject applies to
the new spellings with no edit: `gate_plant_clean` writes one gated item
per SPELLING and the basic must-fire `plant` case runs once per
spelling, so each of the six new spellings plants its own leak and is
caught on its own. The enclosure/pin/desync cases are properties of the
reader and run once per row. Three changed:

- `plant_near_miss_identifier` was one line carrying a prefix AND a
  suffix (`pre<sym>_post`), which fires only when BOTH anchors are gone.
  It now plants each end alone (`<sym>_before`, the shape a field takes,
  and `saved_<sym>`) plus the CamelCase and SCREAMING_SNAKE forms.
- That case now runs **per spelling, not per row**, because the reader
  takes the anchor at each end from that end of the SPELLING: on the
  first spelling only, `source.rs` never planted `bits_witness` and the
  `topo` rows never planted `arena_counts_before`. A derived form that
  is itself another spelling of the row is dropped rather than planted
  (`arena_counts`'s camel form IS `euler.rs`'s fourth spelling), as is
  one that equals its own symbol (`GATHERS`).
- `gate_selftest_pin` said "did not red for <path>, so that row is
  pinned by nothing" in the case where the gate DID red with a different
  count. Those are two failures wanting two sentences; the second now
  says the row is compared and the NUMBER is stale.

`spelling_upper` joins `spelling_camel` as a named helper, so both
derived forms have the same home.

**The cost, re-taken.** `--selftest` wall clock on this lane's box:
**67.8 s before the branch, 87.2 s after the fix pass** (13 subjects
throughout; 17 → 35 spellings, and the near misses moved from once per
row to once per spelling). Still linear in spellings on the
quadratic-in-subjects base, and well under the 2-minute line at which
the brief asked for the per-case subprocess loop to be reconsidered, so
the loop is untouched. One reading, on one box; nothing re-takes it.

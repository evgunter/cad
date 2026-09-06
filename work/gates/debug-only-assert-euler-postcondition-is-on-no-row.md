---
id: debug-only-assert-euler-postcondition-is-on-no-row
kind: issue
title: Fifteen gated statements in topo name no pinned spelling, so the debug-only gate says nothing about them
status: review
opened: 2026-09-06
refs: [debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call, debug-only-bit-witness-callers-are-on-no-row]
branch: gates/debug-only-topo-class
pr: 2066
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
plus the same class closed in the one other rowed file that carried it.

**The pins.** The seven `topo` rows now read
`ArenaDelta|assert_euler_postcondition|arena_counts`. Per file, per
spelling, measured with the gate's own matcher over its code-only view
(one row's spelling at a time, pinned 0, reading the count back out of
the pin diagnosis) — never `grep -c`:

| file | `ArenaDelta` | `assert_euler_postcondition` | `arena_counts` | row pin |
| --- | --- | --- | --- | --- |
| `euler.rs` | 10 | 4 | 4 | 18 |
| `euler_ring.rs` | 10 | 4 | 4 | 17 |
| `euler_kill.rs` | 9 | 4 | 4 | 17 |
| `null.rs` | 3 | 1 | 1 | 5 |
| `split.rs` | 3 | 1 | 1 | 5 |
| `boolean/voids.rs` | 1 | 1 | 2 | 4 |
| `movefac.rs` | 4 | 2 | 1 | 6 |

**The pin is not the column sum, in two rows.** The reader emits one
`USE` per delimiter-cut PIECE, so a piece naming two spellings — the
`self.assert_euler_postcondition(before, ArenaDelta` head of a one-line
call — is one use and not two. `euler_ring.rs` (18 → 17) and
`movefac.rs` (7 → 6) each carry one such line; the other five rows sum.
The per-spelling `assert_euler_postcondition` column reproduces the
Finding's measurement exactly.

**The finding, and the fix, on two dropped attributes.** In a scratch
copy of the thirteen subjects (`--root`), with `crates/topo/src/boolean/
voids.rs:313` and `crates/topo/src/euler.rs:1053` deleted: before this
change the gate printed OK over both, unchanged, at 68 uses scanned.
After it, both red by name —
`voids.rs:313: dst.assert_euler_postcondition(before, transplant, "")`
and `euler.rs:1053: let before = self.arena_counts()`, each with the
"outside any cfg(debug_assertions) item" diagnosis. Live tree: 68 → 105
uses scanned, green.

**The eight prose/literal sites, re-derived.** `grep -rn
assert_euler_postcondition crates/topo/src` outside the seven rowed
files returns exactly the eight the Finding lists (`review_d18_probes.rs
:258 :305`, `review_m1_pr2/release_corruption.rs:277`,
`review_m1_pr5_internal.rs :336 :369 :383 :396`, `source_walk.rs:347`),
and `gate_rust_code` over each of those four files returns NO
`assert_euler_postcondition` at all — every one is a doc comment or a
string body, stripped by the code-only view. All four modules are
declared `#[cfg(test)]` in `crates/topo/src/lib.rs` (`:164`, `:191`,
`:198`, `:205`). None is a code-view use; the list stands.

**The class sweep over `crates/topo/src`.** Every
`#[cfg(debug_assertions)]` in the crate, and what names no spelling on
any row after this change:

- `source.rs:172`, `:195`, `:211` — `plane_bits_witness`,
  `vec3_bits_witness`, `bits_witness`: the BIT CHANNEL mechanism, in a
  file that already has a row, so **closed here**. The row's spellings
  were `bit_identity::|eq_bits`, which those three gated `fn` heads name
  nowhere; dropping the attribute at `:172` passed the gate before this
  change and reds after it (proved on the scratch root). Pin 1 → 6.
- `euler.rs:246` — `use crate::test_support_impl::ArenaCounts;`: the
  arena-delta mechanism, in a rowed file, and **left open**, because
  `ArenaCounts` is a spelling THIS READER CANNOT SERVE. `euler_kill.rs`
  names it three times (`:1121`, `:1331`, `:1982`) inside a
  `#[cfg(test)] mod tests`, which is code to the code-only view and is
  gated by nothing the reader recognises, so pinning `ArenaCounts` would
  red that row on legitimate test code — the S63 no-cry-wolf rule. Same
  for `lib.rs:255`/`:262`, `fixtures.rs:117`/`:126` and
  `splitting/reassembly.rs:281`, whose homes are `cfg(any(…))` or
  `cfg(test)` and so are not debug-only gates by KNOWN GAP 1.
- `boolean/plane_eq.rs:173` and `merge_faces.rs:1006` — the BIT CHANNEL
  again, both statement-position attributes over an `if let` calling
  `plane_bits_witness` (and `vec3_bits_witness`). Files with no row, so
  out of this unit by the fence:
  `debug-only-bit-witness-callers-are-on-no-row`.
- `revert.rs:258`, `attach.rs:92`, `attach.rs:331` — the TIER-1 VALIDITY
  postcondition, `#[cfg(debug_assertions)]` over a `debug_assert_eq!(
  crate::validate::validate(…), Ok(()))`. Not this mechanism, and not a
  hazard: `validate` is ordinary API and `debug_assert_eq!` compiles
  itself out, so the attribute is redundant rather than load-bearing. No
  row.
- `review_m1_pr2/release_corruption.rs:289` — `#[test]
  #[cfg(debug_assertions)]` selecting a test by profile, inside a
  `#[cfg(test)]` module. Not a mechanism. No row.
- `review_d18.rs` (eleven sites), `release_corruption.rs:82`, `:239` —
  `cfg(not(debug_assertions))`, release-only items, which this gate
  reads as not-a-gate by KNOWN GAP 1. No row.
- `lib.rs:228` — `cfg(any(debug_assertions, test, feature =
  "test-support"))` mounting `test_support_impl`. Not a debug-only gate,
  KNOWN GAP 1. No row.

**Fixtures.** Every case the harness already runs per subject applies to
the new spellings with no edit: `gate_plant_clean` writes one gated item
per SPELLING and the basic must-fire `plant` case runs once per
spelling, so each of the five new spellings plants its own leak and is
caught on its own; the seventeen enclosure/pin/desync cases are
properties of the reader and run once per row. Two cases changed:

- `plant_near_miss_identifier` was one line carrying a prefix AND a
  suffix (`pre<sym>_post`), so it fired only when BOTH anchors were
  gone. It now plants each end alone (`<sym>_before`, the shape a field
  takes, and `saved_<sym>`) plus the CamelCase and SCREAMING_SNAKE forms
  of the symbol. Proved load-bearing by mutation: dropping the tail
  anchor reds on `GATHERS_before`, dropping the lead anchor reds on
  `saved_GATHERS`, and `IGNORECASE = 1` reds on `IDENTIFIED_IDS` — the
  old single line passed all three.
- `spelling_camel` is the one new helper, and a symbol that already IS
  its own camel or upper form (`GATHERS`) drops that line rather than
  planting a real use.

The CamelCase line cannot collide under a case fold (the underscore
survives it), so what holds the live `arena_counts`/`ArenaCounts` pair
apart is `euler.rs`'s PIN: three `ArenaCounts` beside four counted
`arena_counts` calls in one file, so a reader that conflated them reds
on the tree. Stated in the fixture's own comment.

**The cost, re-taken.** `--selftest` wall clock on this lane's box:
**67.8 s before, 81.9 s after** (13 subjects throughout; 17 spellings →
34). +21%, for +17 spellings — linear in spellings as the Finding
predicted, and nowhere near the 2-minute line at which the brief asked
for the per-case loop to be looked at, so the loop is untouched. One
reading, on one box; nothing re-takes it.

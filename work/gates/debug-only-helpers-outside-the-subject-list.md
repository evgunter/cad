---
id: debug-only-helpers-outside-the-subject-list
kind: issue
title: Five debug-only helpers under cfg(debug_assertions) in mesh and topo are on no gate's subject list
status: review
opened: 2026-09-06
branch: gates/debug-only-subjects-2
---


## Finding

Filed by the GATES orchestrator from the `gates/debug-only-subjects`
lane's sweep (PR 2030), which generalised
`scripts/gates/bit-identity-debug-only.sh` to a (subject, symbol)
list and then swept `crates/*/src` for other `#[cfg(debug_assertions)]`
item heads. Five are gated by nothing:

- `crates/mesh/src/curved.rs:868` and `:889`
- `crates/mesh/src/tessellate.rs:298`
- `crates/mesh/src/walk.rs:310`
- `crates/topo/src/euler.rs:981`

Each is a debug-only mechanism whose source shape (every site of the
symbol under the attribute, or inside a `debug_assert!`) nothing in CI
holds, the state `debug-only-counters-have-no-gate` found for
`product.rs`'s gather counter. The candidate fix is a row per helper in
the gate's `SUBJECTS` list with a symbol pattern unique to its subject
(the gate's KNOWN GAP 3: a symbol that is also an ordinary word
over-counts), which is this program's edit; the helpers themselves are
S-MESH's and TOPO's files and are read only. Whether every one WANTS
gating — a `debug_assert!`-shaped helper with one caller is a different
case from a counter read by tests — is decided per row when it is
taken.

**What the sweep could not match** (the lane's own statement):
`#[cfg(all(debug_assertions, …))]` heads, feature-gated mechanisms, an
item head more than three lines below its attribute, and anything under
`tools/`, `demos/` or `crates/*/tests`.

## Decision (the `gates/debug-only-subjects-2` lane)

Four of the five want a pin and have one; the fifth wants one and
cannot have one from this reader.

- `curved.rs:868` `identified_ids` — **yes.** A re-derivation five test
  rows read directly.
- `curved.rs:889` `overused_identified_edge` — **yes.** The emitted-form
  re-derivation, read by four test rows beside its one live caller.
- `tessellate.rs:298` `unpaired_chord_segment` — **yes.** A census over
  the assembled mesh, read by six test rows.
- `walk.rs:310` `overused_identified_edge_in` — **yes.** The one home of
  the fan census, `pub(crate)` and called from two lanes, so a third
  caller added ungated is an O(triangles) scan in the shipped kernel.
  A subject is a FILE, so this symbol takes three rows: `walk.rs` for
  the definition, and the two callers' files, `curved.rs` (folded into
  its row's symbol list) and `trimmed.rs`.
- `euler.rs:981` `ArenaDelta` — **no row.** It wants one on the merits —
  a witness type the debug-only postcondition assert reads, named in
  eight `topo` files — but twelve of its sites put the attribute in
  STATEMENT position over a multi-line call whose arguments contain a
  brace, and the gate's reader reports every one as a lost bracket
  depth. Recorded as the gate's KNOWN GAP 6.

Subjects 2 → 6; uses scanned 5 → 28. No ungated live use was found for
any of the five, `ArenaDelta` included: the three `topo` files whose
sites the reader could place (`boolean/voids.rs`, `movefac.rs`,
`fixtures.rs`) are clean and the other five could not be decided.

**Residue**: KNOWN GAP 6 is a reader limitation with a live population,
and it wants a row of its own on this program's slate — a statement-
position `#[cfg(debug_assertions)]` cannot be served until the reader
can say where such an item ends.

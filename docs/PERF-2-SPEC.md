# PERF-2 — `StableName` keying: the naming table stops paying chain depth

**Status: ratified at dispatch (PERF orchestrator, 2026-09-10).** Binds
the implementer of unit `PERF-2`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is
`work/perf/stablename-key-is-quadratic-on-a-boolean-chain.md`.

## 0. The finding this executes

On `die` (a 21-long subtract chain, 84 nodes) the whole rebuild is
36 ms in release with debug assertions off, and **21.4 ms of it is
`wire.boolean.name_emitter`** — 55–60 % of boolean time, ~40 % of the
rebuild — growing 0.061 → 2.451 ms per step along the chain: quadratic
in chain depth, measured by the PERF kernel lane (harness on branch
`perf/explore-kernel`, stage spans in `geom-core`; fetch it for the
instrument, do not merge it).

The cause: `StableName` is a recursively boxed value —
`RoleSeg::FromA(Box<StableName>)` / `FromB` / `FromMember`
(`crates/editor-core/src/names/role.rs:396` onward, ~20 boxed fields)
— and it is the KEY of `NameTable::forward: BTreeMap<StableName, Entry>`
(`names/table.rs:69`). Every insert (`table.rs:96-107`, from
`emit_topo.rs:518-524`, `:1009`, `:1078`, `:1727`) is O(depth)
comparisons plus a deep clone (`name.clone()` into `reverse`, and
again into the error closure), and every survivor of a boolean step
gains one level, so step k costs O(k · names · log) with allocation
per level.

## 1. What this unit delivers

Insert and lookup on the naming table whose cost does not grow with
descent depth, with **every observable unchanged**:

- every emitted name (`NameTable` contents per node, forward and
  reverse) byte-identical across the whole corpus, the tour and the
  gallery documents;
- every persisted byte identical — names are serialized structurally
  (`persist/pairs.rs`, `persist/check.rs`) and documents on disk must
  round-trip unchanged (the persist suite is the pin, plus a row that
  loads every committed `.pncad` fixture and re-saves it byte-for-byte
  if one does not already exist);
- every selector resolution, tie, `DuplicateName` refusal and
  `Display` text unchanged;
- the ORDER in which anything iterates names unchanged, or — if a
  deliberately different deterministic order is chosen — every output
  that depended on it re-baselined with a sentence saying what moved
  (D9: no hash order may influence anything; a content hash with a
  fixed seed is deterministic, but it is still an order nobody asked
  for, so prefer keeping the structural order).

## 2. The design space, and the constraint on it

`RoleSeg`'s nesting is the readable statement of descent and it is
the naming design's vocabulary (`docs/DOCM-IDENTITY-DESIGN.md`, D5,
the `Node::Union` `FromMember` argument in `role.rs`). This unit does
not redefine what a name IS. Within that:

- **Sharing** — `Rc<StableName>` (or an arena handle) in place of
  `Box`, so a survivor's name is one pointer to the operand's name
  rather than a deep copy; clone becomes O(1). Comparison is still a
  walk unless the handle carries something comparable.
- **A comparable handle** — intern each distinct name once per
  evaluation (a per-evaluation interner the emitter threads through,
  or per node with the operand tables' interned ids referenced by
  content), so `FromA(id)` compares in O(1) by id and the structural
  compare is needed only when two ids differ at the top. The interner
  is new state; say where it lives, what its lifetime is, and how the
  memo (`eval`'s `prior`, which transfers a node's `NameTable` by
  content key) and persistence see through it — a persisted name must
  still be the structural value.
- **A cached content digest beside the value** — a fixed-seed 128-bit
  digest computed once at construction, used to short-circuit
  inequality; ordering stays structural (the digest decides nothing
  about order, only whether a full compare is needed). Simple, local,
  and keeps `BTreeMap`'s order; the compare still walks on equal
  prefixes, which is the common case along a chain, so measure before
  choosing it.

Choose by measurement (§3), not by preference, and reject a shape that
makes `role.rs` harder to read than it is. A shape that touches every
match arm over `RoleSeg` in the tree (a flattening) is out: DOCM is
editing that enum and the diff would not be reviewable as a
performance change.

## 3. The pin and the measurement

- **Pin**: the differential in §1 — name tables and persisted bytes
  across the corpus, the tour and the gallery documents, on the merge
  base and on this branch. Write the row first, commit its hashes,
  then change the code. Keep the row.
- **Measurement**: the kernel lane's stage spans on `die` and
  `die_composed_tour` (release, debug assertions off, 4 vCPU under the
  build slot): `name_emitter`'s share of boolean time from 55–60 % to
  **under 10 %**, and the per-step trace no longer growing with k.
  Report the before/after table with spread. `benches/`
  `kernel/boolean/two_bricks` must not move outside its noise floor
  (it is a two-operand case and should not care).

## 4. Out of fence

The naming rules themselves (which names exist, ties, `FromMember`'s
semantics), `emit_topo.rs`'s traversal order, persistence format, and
the `Display` form. DOCM territory: `crates/editor-core/src/names/*`
is DOCM's; DOCM has no unit dispatched there today, and this unit is
announced in `work/perf/log.md`. Touch `role.rs` only where the
representation changes.

## 5. Report

≤150 lines: the shape chosen and the two rejected with their measured
or argued cost, where the new state (if any) lives and its lifetime,
the differential row's coverage, and the measurements in §3.

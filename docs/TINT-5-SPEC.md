# TINT-5 — the F6 enum weld gets a home, and three sibling ban lists adopt it

**Unit of S-TINT.** Row:
`work/tint/sibling-display-contract-suites-hand-mirror-their-enums-too`.

Branch: `tint/5-f6-weld-home`, cut from **`main`**, base **`main`**.

TINT-1 landed the weld that stops an `assert_f6` ban list drifting from
its enum. It landed it in ONE test binary. This unit gives it a home and
takes it to the three sites outside that binary, one of which is already
wrong.

## The measurements, taken before this spec was written — do not re-derive

All executed on `tint/orchestrator` at the merged base, 2026-09-15.

1. **`mesh::TessellateError` has 15 variants; the ban list holds 14.**
   `crates/mesh/tests/errors.rs:188-203` lists every identifier in enum
   declaration order **except `Band`**, which is the last variant
   (`crates/mesh/src/types.rs`). The author appended the case and not
   the list entry.
2. **`Band` IS constructed as a case**, at `crates/mesh/tests/errors.rs:177`,
   inside the same `cases` slice the row walks. So a rendering of `Band`
   that regressed to its `Debug` dump would pass today: the one
   identifier that would catch it is the one the list does not hold.
3. **All 15 variants have a case in that slice.** Measured per variant.
   So the roster is completable by adding one identifier — this unit is
   small, and the set difference reds on arrival naming `Band` rather
   than naming six absences.
4. **`TessellateError` is a closed enum** (`#[derive(Clone, Debug, PartialEq)]`,
   doc says "closed enum, D4 ¶3"), **not `#[non_exhaustive]`**, so the
   compiler can be the census with no exception of the kind
   `SelectRefusal` forced
   (`work/wire/select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`).
5. **`topo::ContactRefusal` (4) and `topo::readback::ReadbackError` (3)**
   both derive `Debug`, are not `#[non_exhaustive]`, and their lists at
   `crates/topo/tests/display_contract.rs:57` and `:118` are complete
   **as of today**. They are the forecast, not the instance. That does
   not make them not-this-unit: an unwelded complete list is a list
   waiting for its next variant, which is precisely what mesh's was.
6. **`test-utils` is already a `[dev-dependencies]` of both `mesh` and
   `topo`** (`crates/mesh/Cargo.toml:83`, `crates/topo/Cargo.toml:80`),
   so nothing has to be added to reach the shared helper.
7. **All three enums derive `Debug` rather than hand-implementing it**,
   which is what `test_utils::f6::variant_identifier` requires.

## The finding that decides the shape, and the reason this spec exists

**The weld is not in `test-utils`. It is `pub(crate)` inside one test
binary.** `assert_f6_every_variant` lives at
`crates/editor-core/tests/display_contract.rs:80`, and it calls
`crate::switch_program_vocabulary::set_difference`, which lives at
`crates/editor-core/tests/switch_program_vocabulary.rs:855` — also
binary-local.

So a lane told to "apply TINT-1's shape to mesh and topo" would copy the
weld into two more crates. **That is the defect this program exists to
close, minted by the fix for an instance of it** — observation 1 in
`work/tint/process-observations.md`, which has happened on two units out
of two and was caught by the reviewer both times. This spec decides the
shape so that it cannot be the lane's accident:

**`assert_f6_every_variant` and `set_difference` get promoted into
`test-utils` FIRST, and all four sites — editor-core's existing four
pairs, mesh, and topo's two — then call the one copy.**

`set_difference`'s own doc argues this better than this spec can. It
reads *"**The one set comparison in this file**… Writing that twice is a
second copy of the comparator kept in step by hand — in a file whose
subject is a second list kept in step by hand — so it is written once"*.
This unit is that sentence applied one level out. After the promotion it
should say "in this tree", which is stronger and will be true.

**Measured, so the promotion is not a hypothesis**: `set_difference` is
pure — `&[&str]`, `&str` in, `Option<String>` out, nothing but core
formatting — and has exactly three call sites, all inside editor-core's
`all` binary (`switch_program_vocabulary.rs:889`, `:958`,
`display_contract.rs:99`). Promotion pulls nothing else.

## The measurement THIS lane must take first, and report back if it fails

**`test_utils::f6::assert_f6` takes four arguments; both local copies
take three.** The fourth is `fields` — the field-punctuation a rendering
must not leak. editor-core pins `&["node:", "name:"]` in its thin
wrapper (`display_contract.rs:43-48`). `mesh`'s inlined predicate and
`topo`'s local `assert_f6` (`display_contract.rs:30`) have no equivalent
and check nothing of the kind.

So adopting the shared helper forces a per-site decision that does not
exist today, and **`&[]` at every site is the failure mode**: it is a
legal argument, it compiles, every row passes, and the unit ships
"adopted the shared predicate" while three sites check strictly less
than the one they copied from. That would be this program's shape
again — a guard that reads as one and guards nothing.

**Take this first**: for each of the three sites, derive the candidate
`fields` from the enum's own payload field names, then check each
candidate against the false-positive hazard `test_utils::f6`'s module
doc names in full — *a door whose own prose OPENS with a field name*,
the `MeshPickError::PositionOutOfRange` case. Report per site what you
found. **`&[]` is an acceptable answer where it is the measured
answer**; it is not acceptable as a default, and a site that lands `&[]`
says in the code why its enum has no field punctuation worth banning.

If the four-argument door turns out to be wrong for these sites — if
`fields` cannot be given an honest value anywhere outside editor-core —
**stop and report** rather than widening `f6`'s API to suit the lane.

## What this unit must NOT do

**Do not copy the weld.** If you find yourself writing
`fn *_is_exhaustive` plus a set comparison in `mesh/tests/errors.rs`
and again in `topo/tests/display_contract.rs`, the promotion above did
not happen and the unit has minted its subject.

**Do not fix mesh's `Band` by adding the identifier and stopping.** The
row says it: *"Adding `TessellateError::Band` to mesh's list is the
smaller half; the weld is what stops the next one."* The list entry
without the weld is the state that produced this defect.

**Do not hand-type identifiers beside match arms.** TINT-1's landed
correction is that rustc checks a match's patterns and never its
strings, so the covered identifiers are read off each value's own
`Debug` via `variant_identifier`. The `match` is an exhaustiveness
TOKEN returning `()`, naming no identifiers.

**Do not touch `crates/*/src/**`.** Out of fence, including if a
variant's `Display` turns out to be wrong. File it.

## The hole that comes with the shape, and must be stated at every site

`assert_f6_every_variant`'s doc already states it and the promoted copy
must keep it: an author who adds a variant, adds its arm to the
exhaustiveness token — which the compiler forces — and then adds
**neither a case nor a roster entry** is not caught, because nothing
renders the variant and nothing contradicts a roster that never grew.
The compile error is what stands between that and an accident. Closing
it would need the variant list itself to be derivable, which safe Rust
does not offer over a type the test crate does not own.

Carry that sentence to the promoted home rather than restating it three
times, and make each adopting site point at it.

## Prove it bites, per site

The mesh site proves itself: **land the weld before adding `Band` to the
roster and show the set difference red, naming `Band`**, then add it and
show green. That is the best evidence this program has had — a guard
reddening on a live defect on arrival rather than on a planted one.

For the two topo enums, whose lists are complete, plant: delete one
identifier from the roster and show red naming it; add a bogus one and
show red naming it; delete a case and show red. And for all four sites,
show that removing a variant's arm from the exhaustiveness token is a
**compile** error, not a test failure.

## Fences

- In: `crates/*/tests/**`, `crates/test-utils/**`.
- Out: every `crates/*/src/**`, `scripts/**` (S-TCOST's, Track K's, and
  `scripts/check-*.py` is CIW's), `.github/workflows/**` (CIW's),
  `memories/**` and `docs/prompts/**` (Ev's).
- `crates/test-utils/src/lib.rs` is also being edited by TINT-4 on
  `tint/4-roster-weld` (it adds `pub mod roster;`). Expect a one-line
  collision there on your base merge and resolve it by keeping both.

## Verification

Hosted CI is the record: **twelve `test (…)` jobs and five
`k-lint (gate, …)`**, 0 failures. Fewer than twelve means something
narrowed the matrix — say so rather than reporting green.

## Review

One style review by path, no A/B row. Its first question is the one the
last three units' reviews had to ask, and the one this spec has tried to
make impossible to answer badly: **does this fix mint a fresh instance
of what it closes?**

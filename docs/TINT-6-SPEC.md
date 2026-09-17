# TINT-6 — a ladder suite that pins the rungs its header claims

**Unit of S-TINT.** Row:
`work/tint/interrogate-ladder-header-claims-every-rung-and-pins-five`.

Branch: `tint/6-interrogate-ladder`, cut from **`main`**, base **`main`**.

`crates/editor-core/tests/lib_u5_interrogate.rs` opens by saying what it
pins is *"every rung of [`InterrogateError`]"*, and argues for itself on
that scope: *"An untested ladder is one where two rungs silently
collapse into each other."* The enum has ten rungs. The suite reaches
five.

## The measurements, taken before this spec was written — do not re-derive

Executed on `tint/orchestrator` at the merged base, 2026-09-15.

1. **`InterrogateError` has ten variants, derives `Debug`, and is NOT
   `#[non_exhaustive]`** (`crates/editor-core/src/names/interrogate.rs`).
   So `test_utils::f6_variants!` — which TINT-5 landed — applies to it
   cleanly, and rustc can be the census with no exception of the kind
   `SelectRefusal` forced.
2. **The five unpinned rungs are still at zero mentions** in the suite:
   `NodeFailed`, `NodePoisoned`, `NoBodies`, `NoSuchBody`, `Readback`.
   Not one appears in any form.
3. **`NoBodies` is blocked and stays blocked.**
   `work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index`
   is open: the rung is produced but destroyed one frame up by a
   `map_err(|_| ..)`. A row for it today would pin a defect.
4. **The five pinned rungs are pinned by asserting a DOOR'S RETURNED
   ERROR** equals a constructed variant (`:94`, `:119`, `:139`, `:159`,
   `:237`, `:256`) — not by listing names in a header.

## What this spec REFUSES, and why it is not available

**A roster of covered rungs accumulated across the suite's rows.** That
is the shape this seat reached for first, and it has no mechanism inside
the fence: it needs one row to read what another recorded, and
**nextest runs every row in its own process**. Measured, by TINT-2,
under the pinned `cargo-nextest 0.9.140` — two rows, pids 12156 and
12157, a shared `static AtomicUsize` reading 0 in both.

It is written down here because it is the obvious idea and the lane
should not spend its own effort refusing it a third time. TINT-2
refused it for the stand-down tally; this is the same wall one row over.

## The shape

**One row that drives every reachable rung through its own door, in one
process, and welds the covered set to the enum.** Self-contained, so
process isolation is irrelevant. `test_utils::f6_variants!` writes the
exhaustiveness `match` and the identifier roster from one list of
idents, so a rung added to the enum tomorrow makes this file fail to
COMPILE, and the roster cannot drift from the patterns because they are
the same tokens.

The header then says what that row pins, and the row is the thing that
can go red.

## The measurement the LANE takes first, and reports back if it fails

**Are `NodeFailed`, `NodePoisoned`, `NoSuchBody` and `Readback`
reachable from a door a single test can call?**

`memories/refusal-text-is-not-cause.md` governs this and the row already
cites it: *"the arm looks unreachable"* is a claim about a call graph.
**Run the door and read the payload.** Do not reason from the variant's
name, its doc comment, or the absence of callers in a grep.

The answer decides the unit's size, and **both answers are legitimate**:

- reachable → the row drives it and the header keeps that rung;
- not reachable from any door this suite can call → **the unit shrinks
  to narrowing the header to what is actually pinned**, with each
  excluded rung named and the reason measured. That is a complete unit,
  not a failure, and this spec says so in advance so the lane is not
  tempted to manufacture a reachable path.

`NoBodies` is excluded by name whatever the four come back as, citing
SHELL's row, so the exclusion is a stated fact rather than a silent gap.
If SHELL's repair lands first, say so and pin it.

## What this unit will NOT enforce, and must say so before anyone asks

**The header's narrowed sentence is still prose.** Nothing checks that
it matches what the row pins. TINT-4 shipped seven sentences into a new
unchecked prose column and **four were wrong or misplaced on arrival**,
in the unit whose whole subject was a header that overclaimed. State
this at the site and in the PR body rather than discovering it in
review.

If the lane finds a way to weld the sentence, that is a bigger unit than
this one — file it, do not take it.

## Fences

- In: `crates/*/tests/**`, `crates/test-utils/**`.
- Out: every `crates/*/src/**` — including `names/interrogate.rs`. If a
  rung turns out to be genuinely unreachable because of a kernel defect,
  that is a finding to FILE, exactly as `NoBodies` already is.
- Out: `scripts/**`, `.github/workflows/**`, `memories/**`,
  `docs/prompts/**`.

## Verification

Hosted CI is the record: **twelve `test (…)` jobs and five
`k-lint (gate, …)`**, 0 failures. Fewer than twelve means something
narrowed the matrix — say so rather than reporting green.

**Prove the weld bites**: show a rung removed from the roster going red
naming it, a rung added to the enum failing to compile, and — if a rung
is driven — the door actually producing it rather than the test
constructing it.

## Review

One style review by path, no A/B row. Its first question is the standing
one, which this program has now answered wrong five times out of five
units: **does this fix mint a fresh instance of what it closes?**

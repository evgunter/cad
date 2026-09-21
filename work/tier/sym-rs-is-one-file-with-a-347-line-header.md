---
id: sym-rs-is-one-file-with-a-347-line-header
kind: issue
title: geom-core's sym.rs is 3898 lines behind a 347-line header: the tier is one file and its argument is one preamble
status: open
opened: 2026-09-06
parent: SYM-2
priority: P1
cost: D
---

**Found by M10-9's fix pass**, filed as a CLASS item: this is a
readability finding, not a correctness one, and it is filed because the
file crossed a line during this unit and nobody decided to let it.

`crates/geom-core/src/sym.rs` is **3898 lines**, of which the first
**347** are one unbroken `//!` header — the tier's whole argument
(what a normal form is, why atoms are opaque, why the budget freezes,
what each rule is for and why it is or is not on) as a preamble a
reader must hold entire before the first `mod`. M10-8 left it at 3087
lines; M10-9 added the door and its registry and took it to 3898,
touching the header in four places.

Inside it, at top level: the ids and their FNV, the node arena, the
`Int`/`Rat` coefficient tower with its own gcd and isqrt, `Poly` and
`Mono`, the session with its budget/rules/counts, the normal-form
walk, the memos, the registry, `Discharge`, the `Decide` impl, and a
test module. Several of those already have a natural boundary and one
of them (`algebra`, `report`, `signed`) has already been taken.

**Why it is worth a row rather than a shrug.** Two independent reviews
of M10-9 both found the same class of problem — a contract sentence in
the header that the code no longer honoured — and in both cases the
sentence was hundreds of lines from the code it described. A header
that long stops being read as a contract and starts being read as
history. The fix pass corrected three such sentences; the next unit
will write more.

## What is owed

A decision on the split, taken by whichever unit next needs to touch
the tier's structure:

1. **The coefficient tower comes out** — `Int`, `Rat`, `gcd_u128`,
   `isqrt_u128` are ~450 lines with no dependency on the session and a
   self-contained argument (`COEFF_BITS`, the `i128` inline). A
   `sym/rational.rs` costs nothing and takes the biggest independent
   block.
2. **The polynomial comes out** — `Poly`, `Mono`, `mono_mul`,
   `Form` and `within` as `sym/form.rs`.
3. **The header splits with them** — each moved block takes the part
   of the argument that is about it, and the remaining `//!` is about
   the tier as a whole. This is the part that matters: the goal is
   contracts next to the code they bind, not a smaller number.

Not M10-9's to take (a 3900-line move in a fix pass is a diff nobody
can review against a door), and not urgent.

## Re-homed at M10's exit sweep (2026-09-13)

Here because it is this program's file and nothing else's.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## After SYM-2

SYM-2 took the two moves the list above names and the header with them:
`sym.rs` 4,478 -> 3,540 lines, its header 491 -> 457; `sym/rational.rs`
532 (header 18), `sym/form.rs` 417 (header 22). Two further cuts are
identified and neither is taken.

**`sym/form.rs` is three concerns under one header** (SYM-4's R1, Q8):
the monomial and its product, the polynomial, and the quotient form
with its budget — a candidate cut, recorded here as accumulation and
not taken by SYM-4, whose fence was the representation.

**The test module -> `sym/tests.rs`, 1,040 lines.** A pure move, written
and reverted in the fix pass (`c9a38f5e4`, `c9811f408`): it is BLOCKED by
`scripts/gates/register-equal-allowlist.sh`, whose whole-file skip
exempts `sym.rs` as a file that DEFINES `Real::register_equal` and is
thereby also hiding the tier's thirteen test calls of the door. Out of
that file they stand outside every home and the gate reads them as a new
constructor site. The condition: `guard` takes
`register-equal-allowlist-exempts-a-whole-file-and-hides-test-calls`
first — a test call has to be distinguishable from a registrant — and
then this cut is a one-commit relocation.

**The ids and the nodes -> `sym/dag.rs`, ~240 lines**, proposed by the
SYM-2 lane and DEFERRED by the orchestrator, the taker meeting the two
conditions below. `SymId`,
`Hash128`, `ParamSymbol`, `SymOp` with its `tag`/`arity` tables and
`SymNode` with its `id` — the whole of what a node IS and how it is
keyed, between the `// ids` and `// the session` banners. It has the
same shape as the two SYM-2 took: no dependency on the session in any
signature, one self-contained argument (D9 — content hashes, the
explicit per-variant tag, the third FNV and why it is not shared), and
a header section already written as its own (`# Node ids are CONTENT
HASHES (D9)`). Two callers cross the line: `intern` (session state) and
`indet_atom`/`indet_param`/`indet_opaque` (the form's keys), both of
which stay where they are and reach `Hash128` and `SymOp::tag` through
`pub(super)`, exactly as SYM-2's moves do.

**The two conditions the deferral names.** (1) `SymId` and `ParamSymbol`
are PUBLIC types, so the move needs `pub use dag::{SymId, ParamSymbol};`
in `sym.rs` to keep `geom_core::sym::SymId` naming the same path — every
other item SYM-2 moved was crate-private, and this is the first that is
not. (2) `SymOp::Opaque`'s doc and `OPAQUE_SEQ`'s D9 argument are one
argument about one mechanism, and `OPAQUE_SEQ` is session state that
would stay behind: the move would put them in different files, which is
the drift class this item exists to reduce. Take the cut with a decision
about where that argument lives, not before.

**Not the session and the walk, yet.** `Session`, `combine`, `form_in`,
the three memos, `Discharge`, the registry and the `Decide` impl are
the remaining ~1,200 lines and they are one argument, not two: the
walk's memo discipline (plain first, early alongside, door last) is
what the counts mean. Splitting it wants a decision about where the
three-memo argument lives, which is a design question rather than a
move.

## Re-measured at SYM-7's merge (2026-09-15)

`crates/geom-core/src/sym.rs` is **4,299 lines** behind a **690-line**
`//!` header. The title and the body above carry the numbers this row
was filed at (3,898 / 347); both have grown by roughly a tenth and a
double since, and the file is not splitting on its own.

SYM-7 added `sym/memo.rs` (the drive-scoped plain memo, ~300 lines) as a
sibling module rather than inside `sym.rs`, which is the split's own
shape — so the growth here is the `# Cost` section's new numbers, the
D9 section's correction, and the memo's door and publication plumbing,
not a new mechanism that could have gone in a file of its own.

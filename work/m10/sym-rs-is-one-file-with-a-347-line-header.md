---
id: sym-rs-is-one-file-with-a-347-line-header
kind: issue
title: geom-core's sym.rs is 3898 lines behind a 347-line header: the tier is one file and its argument is one preamble
status: open
opened: 2026-09-06
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

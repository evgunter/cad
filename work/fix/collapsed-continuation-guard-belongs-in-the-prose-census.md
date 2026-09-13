---
id: collapsed-continuation-guard-belongs-in-the-prose-census
kind: issue
title: the collapsed-continuation class needs a mechanical guard, and PR 2364 measured the threshold that makes one possible: run >= 9, not 4
status: open
opened: 2026-09-11
refs: [2364, 1809]
---



The residue `collapsed-string-continuations-ship-space-runs-to-users`
(PR 2364) deferred by name: it fixed the instances and said *"whether
the guard belongs here or in `scripts/gates/` is the real question."*
Filed at the moment of disclosure per `work/README.md`, with the
measurement that makes it answerable.

## The fence question is settled, and the closed row guessed it wrong

That row routed the guard to CIW ("CIW's as much as ours"). **It is
not CIW's.** `work/ciw/program.md`'s own `keep_out` reads
*"`scripts/gates/*` is code-quality Track K's"*, so the grep option
was never CIW's ground either.

**And the grep option is the wrong one anyway.** The needle lives
*inside* string literals, and `gate_rust_code` builds the code-**only**
view — the exact obstacle PR 1809 hit and routed around by writing its
census in Rust. That census exists:
`crates/pncad-py/src/prose_census.rs`, *"the mechanical half of the
prose gate"*, which already walks every `Display` impl in `crates/`,
already resolves field types, and already carries an `UNDECIDED`
roster convention for what it cannot decide. **That is the home.** A
second claim there costs a walk it already performs; a
`scripts/gates/` grep would need a path allowlist for the one class
that is not mechanically separable (below) and could carry nothing
else.

## The threshold, measured by PR 2364 rather than guessed

The closed row proposed a run of **four or more** spaces. That is far
too low. The lane classified every hit by hand over the whole tree and
measured both directions:

| pattern | sites | genuine | false | FP rate |
|---|---|---|---|---|
| the closed row's regex, `crates/` only | 27 | 18 | 9 | **33%** |
| widened, run ≥3, all cargo roots | 83 | 27 | 56 | **67%** |
| widened, run ≥4 | 43 | 27 | 16 | **37%** |
| widened, run ≥9 | 29 | 27 | 2 | **7%** |
| widened, run ≥9, minus runs followed by `=` `:` `->` `\|` | 27 | 27 | 0 | **0%** |

**Run length separates the two classes almost perfectly.** Every one
of the 27 real sites carries a run of **≥10** — a collapsed
continuation swallows a whole source indent, and the shallowest in
this tree is ten columns. Deliberate column alignment clusters at 3–8.
The closed row's threshold of 4 sits inside the alignment cluster,
which is why its own hit list was a third false.

## What the guard should say

*A run of ≥9 spaces between two non-space characters, inside a non-raw
string literal, not following a `\n` or `\t` escape, not opening the
literal, not on a comment line.* At PR 2364's merge base that fires on
exactly the 27 real sites and 2 others.

Three exclusions, all lexical and all cheap — **the `\n` rule alone
kills 6 of the closed row's 9 false positives** (block indentation in
a deliberately multi-line printed message).

**One caveat to carry, not to bury:** the final `= : -> |` exclusion
that takes the tree to zero is fitted to 43 hits and the lane
explicitly declined to claim it generalises. Treat it as a
convenience, not a rule, and prefer an `UNDECIDED` roster row over a
silent exclusion if it over-fires.

## The class that is not mechanically separable

**Source-text fixtures** — Rust and Python snippets embedded as
needles in tests, where the indentation is exactly what is being
matched (`pncad/tests/all.rs:1861,4769,4788`,
`viewer/tests/landing_gathers.rs:364`, `pncad-py/src/tests.rs:5497`).
On this tree they fall out under the run floor and the `\n` rule —
**which is luck, not design**, and the lane said so. They will not
keep falling out. This is the second argument for the census over a
grep: a census can name each in a roster row with its reason, the way
`prose_census.rs` already does for its 28 undecided renderings; a grep
can only carry a path list that says nothing about why.

## Blind spots to carry forward

PR 2364's sweep did not search: raw strings (`r"…"`, `r#"…"#`),
literals assembled by `concat!`/`format!` from individually-clean
pieces, collapses whose swallowed indent was 1–2 spaces (below any
workable floor), and messages built at runtime.

## Fence

`crates/pncad-py/src/prose_census.rs` is **LIB's** glob. FIX built it
(PR 1809, this program's unit) and owns the two rows that feed it, so
the claim is by announcement — name the fence in the PR body.

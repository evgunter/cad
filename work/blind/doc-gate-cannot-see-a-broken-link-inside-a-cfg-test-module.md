---
id: doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module
kind: issue
title: doc-gate.sh cannot see a broken intra-doc link inside a #[cfg(test)] module
status: open
opened: 2026-09-16
priority: P3
cost: E
---


Found by CENSUS-ARRIVAL-RESIDUE's fix pass (2026-09-16) and executed
by it, then filed here because `scripts/doc-gate.sh` is CIW's
(`work/ciw/program.md:11`). CENSUS disclosed it at the site and filed
it rather than fixing it: `work/README.md` says *"disclosing a residue
is not scheduling it."*

## What was executed

A broken intra-doc link inside `crates/pncad-py/src/tests.rs` leaves
`scripts/doc-gate.sh` **green**. That module is `#[cfg(test)]`
(`crates/pncad-py/src/tests.rs:1768`), and rustdoc does not build a
`cfg(test)` module at all, so no link inside one is ever resolved.

The finding is recorded at `crates/pncad-py/src/tests.rs:6845`, in the
doc of the walk whose own blind-spot list carried a dangling test name
for two commits without the gate noticing.

## Why it is a row and not a note

**The gate's population is not the population its name implies.** It is
a gate — a green from it is read as "every doc link in this crate
resolves" — and for every `#[cfg(test)] mod` in the tree it says
nothing while reporting agreement over what it can still read. That is
the shape CENSUS exists to find, one instrument over: a check that
scans the wrong population, where the drift is invisible and certified.

Scope is not one file. Every crate in this workspace with a
`#[cfg(test)]` module is unchecked by this gate today, and a name
written inside one is uncheckable rather than merely unchecked — so a
lane that writes `[\`foo\`]` there gets no signal at any point.

## What is not decided here

Whether the repair is `--cfg test` in the gate's rustdoc invocation,
a second rustdoc pass over the test cfg, or a ruling that doc links in
test modules are out of scope and the gate's own doc says so. All
three are CIW's call; what this row asserts is only that the gate's
claim today is wider than what it checks, and that nothing says so.

---
id: the-exact-referee-is-linted-and-run-by-nobody
kind: issue
title: crates/mesh/tests/nurbs_exact_referee.py is linted by CI and run by nobody, so nothing pins its literals against the rows that assert them
status: open
opened: 2026-09-22
---


Filed by TESS-2 from a blinded review of its head. On TINT's slate
because `crates/mesh/tests/` is this program's ground.

## What

`crates/mesh/tests/nurbs_exact_referee.py` computes, in exact rational
arithmetic, the literals that
`crates/mesh/src/nurbs_cert.rs`'s
`the_described_bilinear_rationals_true_second_partials_are_under_the_certified_sups`
asserts against the certified sups. Twenty-one `f64` literals, generated
by `python3 crates/mesh/tests/nurbs_exact_referee.py --rust` and pasted
into the Rust row.

CI lints that file (`scripts/check-python-lint.py` reconciles ruff's walk
against `git ls-files '*.py'`, so every tracked Python file is linted)
and **runs it never**. Nothing anywhere pins that the Rust table still
matches what the script prints. So:

* an edit to the script's mathematics — a wrong differencing index, a
  changed span tie-break — is caught by nothing, and the next lane to
  regenerate the table would silently weaken or break the Rust row;
* an edit to the Rust table by hand (the doc says not to, which is a
  rule and not a mechanism) is caught by nothing either;
* and the script's own `assert`s inside `sqrt_rounded_down` — which are
  what make its answer a proof rather than a seed — execute only when
  someone runs it.

It was moved OUT of `scripts/` precisely because
`scripts/check-ci-mirror-parity.py` refuses an executable check under
`scripts/` that neither CI half names. That refusal was right about the
path and it does not reach this one: outside `scripts/` there is no
roster property at all, which is the hole this row records.

## What a fix looks like

Smallest: a `#[test]` in `crates/mesh/tests/` that shells out to
`python3 crates/mesh/tests/nurbs_exact_referee.py --rust` and compares
the output against the table — which needs the table readable from the
test, so the table moves to a data file both sides read, or the Rust row
prints its own table in the same format and the test diffs the two.
Python availability is the wrinkle: the hosted test job has it, and a
`ci-local.sh` box might not, so the row wants the same
degraded-box posture `check-python-lint.py` already spells out (hard
failure where `GITHUB_ACTIONS` is set, a skip that says so elsewhere).

Cheaper and weaker: run the script from the python-suite job, which
already exists and already has Python, and have IT compare. That buys
the mathematics being executed and the script's own assertions firing,
without pinning the Rust table.

The general shape — "an executable check outside `scripts/` that no half
names" — is the thing with no roster. This row is one instance; whether
TINT wants the instance or the property is TINT's call.

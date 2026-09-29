---
id: rustc-suggests-importing-real-in-a-certification-file
kind: issue
title: rustc's own help steers a certification author to use geom_core::Real, the one import the certification gate forbids
status: open
opened: 2026-09-24
priority: P4
cost: E
---

## Finding

Both RING-5 reviews (#3174, R1 F9 and R2's end-to-end check) wrote a new
certification file — the `Certification` trait imported, `Real` not —
and reached for `.sqrt()` and `.is_poison()` on an `Interval`. rustc
refuses both with E0599, and its help for each is:

> trait `Real` which provides `sqrt` is implemented but not in scope;
> perhaps you want to import it: `use geom_core::Real;`

That is the one import `scripts/gates/certification-doors.sh`'s REAL
rule forbids in a listed file. Following it compiles whenever the file
calls none of `zero`/`one`/`powi` (those go E0034, two traits in scope),
and the gate says so only once the file is on `CERT_IMPORTERS`: an
unlisted file sees UNLISTED alone, so the REAL and POISON diagnoses
arrive after the author has already built on the wrong import.

## What would close it

Not a code change to the doors. Candidates, cheapest first:

- the UNLISTED message (`cert_rule_message`) names the trap up front:
  "listing the file also forbids `use geom_core::Real` — rustc will
  suggest it for `sqrt`/`is_poison`, which certification does not call";
- a `#[diagnostic::on_unimplemented]`-style note is not available for a
  method-not-found on a concrete type, so a doc paragraph on
  `Certification` (`crates/geom-core/src/interval/certification.rs`,
  "What the separation does not cover") saying what to write instead:
  `mag`/`width`/`sqr` for a certification bound, a lane `T` for
  evaluation, `Interval::from_certified` for the crossing.

Filed by RING-5's fix pass on the SCALAR slate, which owns the trait.

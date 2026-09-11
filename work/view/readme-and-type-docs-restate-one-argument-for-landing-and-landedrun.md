---
id: readme-and-type-docs-restate-one-argument-for-landing-and-landedrun
kind: issue
title: the README/type-doc double statement one-argument-for-outstanding found is also true of LandedRun, and nothing declares either a copy
status: open
opened: 2026-09-06
---


## What

The residue of
`work/view/one-argument-for-outstanding-restated-in-five-places.md`,
whose last line asked the same question of the session's other minted
values. Split out at that item's close (#2055) rather than left in its
prose, since a closed item is a record and not a slate.

Checked, and it holds for one of the two:

**`LandedRun`.** `crates/viewer/README.md:379-386` and
`crates/viewer/src/session.rs:333-346` make the same argument in the
same order — that the things a landing produces are statements about
one (document, evaluation) pair, taken from that pair's single gather
in `land`, so that one field read beside another run's would describe a
run that never happened. Neither site declares the other, so the
disclosed-copy vocabulary (`verbatim`, `mirror of`, `re-derived`) does
not find the pair; only reading both does. The README then continues
for two further paragraphs on `body`'s cost, which the type doc points
at rather than repeats — so the drift risk is concentrated in the first
paragraph, where the two are closest.

**`AtRestBadge`.** Not an instance. The README names it and does not
argue it; `crates/viewer/src/session.rs:462-475` is its only home.

## Why it matters here and not everywhere

`crates/viewer/README.md` is the design doc beside the code and is
meant to carry argument, so overlap with a type's doc is not by itself
a defect — the defect is an argument written twice with nothing saying
which is the home, because the gate that reads this README checks its
module tables and not its paragraphs. #2055 settled that for
`Outstanding` by declaring the README the one home and cutting the type
docs to invariant-plus-pointer. The same treatment is what this row
wants for `LandedRun`; it was not taken there because it is not that
unit's subject and the section is load-bearing for `Derived`.

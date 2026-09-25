---
id: rule-gs-magnitude-door-never-asks-rule-c
kind: issue
title: rule G's magnitude door never asks rule C: manifest::magnitude never declines, so the certified fold and the atom door after it are unreachable
status: open
opened: 2026-09-25
priority: P3
refs: [rule-g-is-the-link-and-pads-leaf-cost, DECIDE-7]
---


## What was found (DECIDE-7, by reading)

`crates/geom-core/src/sym/root.rs`'s `magnitude_of_root` asks
`manifest::magnitude` first, and on `Some` returns it. Only on `None`
does it go on to rule C's certified fold (`signed::fold`, under
`signed_root`) and then `magnitude_atom`. But `manifest::magnitude`
never answers `None` for a form with a non-zero constant denominator,
and every `Form` has one:
- a constant folds to its magnitude;
- a manifestly non-negative form is its own magnitude;
- anything else mints the `Abs` atom over the argument as written.

So the two arms after it do not run. The doc on `magnitude_of_root`
says "Rule C's certified fold is asked only after the value-free step,
which is `combine`'s documented order". As the code stands, rule C is
never asked there.

The atom it mints instead is keyed on the sign-normalised root `R`.
`R` is the square root of a primitive integer polynomial, so its
content is one. The key is therefore `magnitude_key`'s, and the atom
is the one `magnitude_atom` would have minted. Only the certified
fold is lost.

DECIDE-7's profile agrees without proving it. The magnitude door ran
204 times on the pad's dev leaf, and 24 of those folded to `R`
(`RootProfile::parts`). The instrument does not count the arms after
it.

## What to decide

Either:
- let the manifest step decline instead of minting, so that rule C
  and the door are reached; this can change what the pad and the link
  decide, and is a ruling on rule G's order; or
- delete the two unreachable arms and correct the doc.

## Home

`crates/geom-core/src/sym/root.rs` (`magnitude_of_root`),
`crates/geom-core/src/sym/manifest.rs` (`magnitude`).

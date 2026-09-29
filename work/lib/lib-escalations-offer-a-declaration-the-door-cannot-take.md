---
id: lib-escalations-offer-a-declaration-the-door-cannot-take
kind: issue
title: lib: Python's import_step and Body.validate_geometric show the census's declare menu and give the caller no declaration
status: open
opened: 2026-09-29
priority: P2
cost: M
---


(CHROME, from the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.
`work.py territory` gives `crates/pncad-py/src/py/value.rs` to BIND and
LIB. It is filed here because `crates/pncad-py` is LIB's.)

## What

`ValidationError::CensusEscalated` (`crates/topo/src/validate.rs`
:1500, `Display` near :3066) ends in `too_close`'s recourse: "Recourse:
declare the coincidence, move the geometry, or lower the tolerance".
Its doc says that is right "where a coincidence between two things has
an object to declare". At the product and assembly gates it has one,
because mates declare contacts. At Rust STEP import it has one too,
through `ImportOptions::declared_contacts`
(`crates/pncad/src/prelude.rs` :568–592).

Two Python doors show the same sentence and give the caller no
declaration to make:

- `import_step` (`crates/pncad-py/src/py/value.rs` near :2131)
  withholds `declared_contacts`. `prelude.rs` says why: `ImportContact`
  has no Python value class yet. A tier refusal renders each verdict
  through `StepImportError::TierInvalid`
  (`crates/step-import/src/error.rs` :485–492, EXCH's ground), so a
  Python caller reads "declare the coincidence" and has no way to do it.
- `Body.validate_geometric` (`value.rs` :465) runs
  `topo::validate_geometric` and takes no declaration at all.

## Repair shape

There are two ways through, one for each door:

- For `import_step`, give `ImportContact` a Python value class and
  expose `declared_contacts`. The declare lever then becomes real there.
- For `Body.validate_geometric`, either accept contacts, or have the
  binding (or `too_close`'s caller) route
  `geom_core::NO_DECLARATION_RECOURSE` when no declaration channel
  exists.

`validate.rs` cannot tell its caller apart, so the routing belongs to
the door.

---
id: import-declaration-channel-has-no-python-spelling
kind: issue
title: The STEP import declaration channel has no Python spelling: import_step withholds declared_contacts because ImportContact has no value class
status: open
opened: 2026-09-15
refs: [1495]
priority: P3
cost: D
---

## Disclosed by PORT's `python-cannot-set-options-structs` (PR for
`port/pyopts-four-doors`)

That unit bound `ImportOptions` at the Python import door and stopped
one field short. `ImportOptions` has two:

- `eps_in` is now `import_step`'s `eps_in=` keyword — the tooth the row
  was filed for;
- `declared_contacts: Vec<ImportContact>` is **withheld**, recorded in
  `crates/pncad-py/src/surface_census.rs`'s options roster as a
  `Spelling::NotBound` with its reason, and decayed against the door's
  own keywords so the reason cannot outlive the fact.

The reason is that binding the field is not a keyword: `ImportContact`
is the import-side declaration channel's element type (M9-2, D7 step
4 — position-anchored contact declarations certified by the same
tier-3′ gate a native declared-contact body runs), and a `list[...]`
keyword whose elements a Python caller cannot construct is a door onto
nothing. What it needs is a value class of its own — a constructor for
the `VertexRest { at: [f64; 3] }` anchor, its stub entry, and the
binding-census rows that follow.

This is the second half of a shape the Rust prelude already closed once
and recorded (`crates/pncad/src/prelude.rs`, the `ImportContact`
paragraph): the Rust channel was *callable and not fillable* until
`ImportContact` was curated, and the Python channel is in exactly that
state now.

`crates/pncad-py/tests/test_binding_census.py`'s `ImportContact` entry
in the different-shape roster names this row's trigger in its own
words — *"binding it is what makes this entry stop being honest"* — and
must move to `BOUND_AS` in the same unit that binds it.

## Home

`work/lib/` — `crates/pncad-py/*` is LIB's territory
(`work/lib/program.md`'s `paths`). The element type itself is
`crates/step-import/`'s and so EXCH's, but nothing kernel-side needs to
change: `ImportContact` is already public, prelude-curated and
constructible from three `f64`.

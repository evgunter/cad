---
id: a-fixture-may-compose-a-dead-door-and-nothing-says-when
kind: issue
title: A unit row may set up its fixture with a composition production does not perform, and the rule that decides when is stated at one site instead of once for the crate
status: open
opened: 2026-09-20
priority: P4
cost: E
---


Filed by VNEWS's frame.rs-cluster adjudication (2026-09-20), which
closed `work/vnews/a-fold-row-composes-a-producer-with-a-dead-door` as
a negative result on its own question and found this half of it lands
across the fence.

## The rule, and where it is stated today

`crates/viewer/src/frame.rs`'s test module holds five rows that compose
a `*_status` producer with `frame::apply`, and two of the six
compositions are dead — no production caller performs them. One of the
two rows discloses that and one does not:

- `a_refused_fold_is_news_about_the_camera_and_apply_overwrites_with_it`
  (`crates/viewer/src/frame.rs:2351`) carries the argument in full at
  `:2337-2349`: *"No production caller composes them any more … What
  survives here is `apply`'s contract"*, plus the pointer to
  `pane::viewport`'s row on the live path, and an inline note at
  `:2367-2368` saying the producer is *"the nearest producer to hand
  rather than a live composition"*.
- `a_clean_fold_retires_the_camera_refusal_it_did_write`
  (`crates/viewer/src/frame.rs:2182`) uses the same dead composition at
  `:2188` to place its fixture and says nothing.

The discriminator the #2026 style review used is the row's **NAME**: a
row is wrong when its name claims the composition is the behaviour, and
right when the name is about a property the live path really has. That
rule is sound and is stated nowhere general — only inside the one doc
comment that applies it.

## Why it is VDOC's

`work/vnews/program.md`'s `keep_out` puts `crates/viewer/README.md` in
this program's territory and says a prose, census or citation defect
there *"is filed on vdoc and never fixed across that fence"*, and
`work/vnews/plan.md`'s dispatch rules reaffirm that the named carve-out
beats the general fence ruling. A crate-wide clause about what a unit
row's fixture may compose belongs in `crates/viewer/README.md`.

## The shape of an answer

One clause on that page, with the name test as its falsifiable half and
the two `frame.rs` rows as its worked instances. It should also say
what the row on the live path is for, since #2026's fix pass added
`pane/viewport.rs`'s
`landing_a_clean_fold_retires_the_camera_refusal_it_landed_before` for
exactly that reason.

**The population may shrink before this lands.**
`work/vnews/ranked-and-unranked-verdicts-are-one-type` would make
`frame::apply` take a ranked verdict, at which point both dead
compositions stop compiling and the clause has one live instance
instead of two. The rule is still worth stating — the shape recurs
whenever a door is added beside an existing one — but re-derive the
instances against the tree before writing it.

---
id: move-the-geometry-is-spelled-as-a-literal-at-a-hundred-sites
kind: issue
title: "Recourse: move the geometry …" is spelled as a literal on 103 lines, beside its home (MOVE_RECOURSE, move_recourse!("so …"))
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
---


Left by `work/onecopy/carry-refusal-copies-and-hand-spelled-move-recourses.md`. That row gave the move-recourse class its spelling (`MOVE_RECOURSE` the whole ending, `COINCIDENCE_RECOURSE` the fragment, `geom_core::move_recourse!("so …")` a move named more closely), and put the gate on the named pair. The gate reads constants, not words. The words themselves are still written by hand under `crates/*/src` (counted on 2026-11-10, at `onecopy/carry-copies`):

| spelling | lines |
|---|---|
| `"… Recourse: move the geometry"`, the whole ending | 13 |
| `"Recourse: move the geometry so …"` | 81 |
| `"Recourse: move the geometry …"`, other continuations | 9 |
| `"Recourse: move <another subject> …"` (`the parts` 42, `the point` 8, `the split` 6, …) | 111 |

Many of the 13 are test pins (`refusal_routes.rs`, `order.rs`, `tier3_tests.rs`, `certify.rs`), which may rightly spell the text they pin. The production ones are copies of `MOVE_RECOURSE`.

**What is owed.**
1. Route the production copies of the whole ending, and the `so …` literals that are `&'static str`, through the home.
2. Decide what a `format!` site with interpolation after "so" writes. `concat!` cannot take it; `{MOVE_RECOURSE} so {…}` is the likely spelling.
3. Then decide whether the gate should read the words `Recourse: move the geometry` as a literal.

The "another subject" lines are a different lever ("move the parts" is an operation's own) and are not the move arm's.

`grep -rn 'Recourse: move' crates/*/src` lists them all.

## Resolution (`onecopy/move-recourse-home`)

**Where the 103 lines were.** All but five were in test code. The other
two were the home's own doc (`geom-core` `predicate.rs`) and one row a
classifier misplaced (`boolean/ops.rs`'s merge-door test, inside
`mod tests`). The five production copies now go through the home:

| site | before | now |
|---|---|---|
| `topo` `contact.rs` `CONTACT_RECOURSE_MARKED` | a literal | `move_recourse!("so the faces clearly touch or clearly part")` |
| `topo` `contact.rs` `CONTACT_RECOURSE`, the unlabelled fragment | `"move the geometry so …"` | `concat!(coincidence_move_arm!(), " so …")` |
| `topo` `merge_faces.rs` `CROSS_OR_SMOOTH` | a literal | `move_recourse!("so …")` |
| `geom-brep` `ssi.rs` `WALL_COLLAPSED_RECOURSE` | a literal | `move_recourse!("so …")` |
| `topo` `validate.rs` `LaminaWedge`, a `write!` | the ending in the format string | `{}` ← `move_recourse!("so …")` |
| `profile` `path.rs` `SeamArrivalOffDirection`, a `write!` with interpolation before the ending | the ending in the format string | `{recourse}` ← `move_recourse!("so …")` |

**The `format!` spelling (owed item 2).** No production site
interpolates after "so". The one spelling is therefore the named move
as a format argument: `{recourse}` / `{}` ← `move_recourse!("so …")`.
That keeps the `so …` check `move_recourse!` makes at compile time.
`{MOVE_RECOURSE} so {…}` is left for a site that needs a value after
"so", and none does.

**Test pins.** The bare whole-ending pins now read the constant:
`refusal_routes` ×2, `tier3_tests`, `census/curved.rs`, `order.rs`,
`span_rows.rs`, `certify.rs`'s prefix check, and `sweep` `swept.rs`'s
`LEVER` table. The rest are 89 lines, 76 of them `… so …`, that pin a
full sentence or a named move. They keep their text, as the row allows.

**The guard (owed item 3): yes.**
- `scripts/gates/recourse-ending-by-hand.sh` reads the words
  `Recourse: move the geometry` as a literal, across `\` continuations,
  in production code under `crates/*/src`. Test code is out (the gate's
  `--skip-cfg-test` view), so a pin may spell the text it pins. "Move
  the parts" and the other subjects pass.
- Its selftest fires on a planted copy across a continuation. It passes
  a test pin and another subject.
- The view's test skip had a hole, which the selftest also pins.
  `scripts/gates/lib.sh` counted the braces of a kept literal as code,
  so a test that searches its own source for `"\n}\n"` closed its
  `#[cfg(test)]` module early, and the rest read as production.
  `ops.rs`'s merge-door test did exactly that. A kept literal's braces
  are now taken back out of the count. Every gate and every `--selftest`
  is green after the change.

**Byte-identical.** Every `validation_error_samples` rendering (Build,
AtRest and `Display`, 2,172 lines, `LaminaWedge` among them) is
identical before and after. The four constants and
`SeamArrivalOffDirection`'s rendering were each asserted equal to their
old literal.

**The lever form (6753 review).** About 25 production `&'static str`
levers spelled the arm unlabelled, `"move the geometry so …"`, for a
slot a recourse label is put before (`LeverOnly`, `SizedDecision`, a
refusal's own lever constant): geom-brep `certify.rs` ×5, `dihedral.rs`,
`edge_crossing.rs` ×2, `recourse.rs` ×4; profile `gate.rs`; topo
`refusal_routes.rs` ×5, `euler.rs`, `props.rs`, `split.rs`,
`splitting/containment.rs` ×2, `validate.rs` ×2. They now go through
`geom_core::move_lever!("so …")`, which `move_recourse!("so …")` is
defined over (the label and it), with the same compile-time `so `
check. `contact.rs`'s two spellings share one condition literal
(`contact_condition!`), through the two macros. The gate refuses the
words opening a literal as a lever too. Four levers open the arm with
another continuation ("nearer the origin", "to a scale", ", or"); the
`so ` check does not take them, and the gate's header names them as a
blind spot. The built libraries' strings that begin
`move the geometry` are identical before and after (54 distinct
`… so …` values).

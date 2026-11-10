---
id: a-lever-opening-the-move-arm-without-so-passes-the-gate
kind: issue
title: Four production levers open "move the geometry" with a continuation other than "so", and the gate passes a fifth
status: open
opened: 2026-11-10
priority: P3
cost: C
---


Left by PR 6753 (`work/onecopy/move-the-geometry-is-spelled-as-a-literal-at-a-hundred-sites.md`), at the orchestrator's direction.

`scripts/gates/recourse-ending-by-hand.sh` refuses `"move the geometry so` opening a production literal, since `geom_core::move_lever!("so …")` is its home. A lever that opens the arm with any other continuation passes: `move_lever!`'s compile-time check takes only `so `. Four production levers do that today:

| site | lever opens |
|---|---|
| `crates/geom-brep/src/certify.rs:1607` | "move the geometry to a scale where the spline face's control points …" |
| `crates/geom-brep/src/ssi.rs:2632` | "move the geometry, or the searched region, until each traced branch's tube …" |
| `crates/geom-brep/src/ssi/exhaust.rs:470` | "move the geometry nearer the origin, within the model's size range, …" |
| `crates/profile/src/fillet_recourse.rs:193` | "move the geometry nearer the origin, keep the radius no larger than …" |

A fifth would pass as silently.

**What is owed.**
1. Make the gate fire on any production literal opening `"move the geometry` at a word boundary, not only before `so`.
2. Hold the four as a counted, recorded exception list in the gate, each with its reason, in the shape `scripts/gates/viewer-module-kinds.sh` keeps its hand-kept lists. The list's count is checked against the hits, so a fifth goes red and a site that goes away takes its line.
3. Decide whether the two "nearer the origin" levers (`fillet_recourse.rs:193`, `ssi/exhaust.rs:470`) want one home beside `geom_core::RANGE_RECOURSE` ("scale the geometry into the session's range"). Both name the model's range, not a coincidence's move, so they may be the range arm's and not the move arm's.

## Resolution (`onecopy/move-arm-exceptions`)

1. **The gate fires on any literal opening the arm.** In
   `scripts/gates/recourse-ending-by-hand.sh`, the unlabelled shape is
   now `"move the geometry` at a word boundary, whatever follows
   (across `\` continuations), in production code outside geom-core,
   the arm's home. Before, it fired only on `"move the geometry so`.
2. **The recorded exceptions.** `MOVE_ARM_EXCEPTIONS` holds them as
   `FILE|NEEDLE|COUNT|REASON`, as `viewer-module-kinds.sh` holds its
   own. A hit no entry names reds. So does an entry whose count is not
   the tree's, either way, which is also what retires it. The list is
   overridable only under `--root`, so the self-test can plant its own
   and no environment exempts the repo. There are five entries: the four
   levers above, and `test-utils` `refusal.rs`'s `BARE_RECOURSES`.
   That one is a dependency-free leaf's restatement of
   `COINCIDENCE_RECOURSE`, which editor-core's
   `the_bare_recourses_are_geom_cores_constants` holds equal, and which
   the old `so` shape never reached. The self-test fires on a fifth
   lever, on an entry above its count, and on an entry for a file with
   none. It passes a recorded entry at its count, and the arm in its home.
3. **The two "nearer the origin" levers do not go beside
   `RANGE_RECOURSE`.** That fragment ("scale the geometry into the
   session's range") is the floating-point format's range: a length
   that overflows or underflows. The two levers are a different failure:
   - `ssi/exhaust.rs` `R3_SCALE`: coordinates too far from the origin to
     resolve ε;
   - `fillet_recourse.rs`: a corner too far from the origin for its
     radius.

   Both are resolution at a distance, not range. Their shared words stop
   at "move the geometry nearer the origin", and their continuations say
   different things, so a common home would hold four words for two
   sites. They are recorded exceptions with that reason. The lever whose
   cause *is* the format's range is `certify.rs`'s "to a scale where the
   spline face's control points stay well inside the range of finite
   numbers". Whether it should end on `RANGE_RECOURSE` is a wording
   change to a rendered sentence, left for a row of its own if wanted.

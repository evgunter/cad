# VGEOM — log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/vgeom/plan.md`. A/B band 5300–5399
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-17) — opened in VIEW's re-scope

Opened by VIEW's orchestrator as one of four successors on VIEW's
ground, per `work/README.md`: *residue is re-homed before the sweep …
to a new program opened for it when the residue coheres into a track of
its own (a dozen items on one territory are a successor's opening
slate, and the closing program opens it)* (Ev, 2026-09-06).

**VIEW is not closed by that act and is not closed by this one.** Its
`Order`'s six units are done, deferred or handed off; its ninety-four
live rows were review accretion on one crate, and four of them cohere
into tracks. VIEW stays open with eight rows, its exit walk is a
separate ratified step, and `docs/DOC-LEDGER.md` records the sweep when
it happens.

21 rows arrived from `work/view/`, each by `git mv` with its body,
its id and its history unchanged — no row's prose was edited on the way
past, and the item schema carries no `program:` field, so a re-home is
the move and nothing else:

- `a-count-slot-launders-a-typed-nan-into-zero`
- `a-fields-text-commits-within-the-renders-own-tolerance`
- `a-flat-rung-pair-is-read-as-flat-below-it`
- `a-nan-edge-distance-wins-its-boundary-rather-than-losing`
- `corner-count-substitutes-u32-max-for-a-length-it-could-not-cast`
- `ctrl-wheel-reaches-no-zoom`
- `cursor-projection-is-f32-in-a-module-whose-matrices-are-f64`
- `finite-bounds-yield-an-infinite-scene-radius`
- `flatten-emits-every-vertex-before-it-judges-any-of-them`
- `id-readback-failure-reads-as-nothing-under-the-cursor`
- `pickindex-merges-parts-on-a-rounded-t-it-never-converts`
- `pickindex-tie-break-rests-on-a-comment`
- `renders-that-multiply-a-finite-guarded-length-spell-the-product-inf`
- `sketch-headings-guard-zero-length-but-not-an-infinite-one`
- `the-budgets-predicted-count-is-not-always-an-over-count`
- `the-fields-door-has-no-width-bound-at-all`
- `the-one-free-transform-is-the-only-total-door-in-camera`
- `the-point3-to-gpu-corner-cast-is-at-three-sites`
- `the-shader-encodes-a-mark-strength-nothing-bounds`
- `viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep`
- `world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure`

The charter that makes these one program, and the sentence that is true
of them and false of the other three tracks' rows, is `plan.md`
§Charter. The band 5300–5399 is claimed in `docs/MODEL-AB-LOG.md` in the
commit that opens this program, per that entry's own rule.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`
if the posture ever changes; under the inherited posture it records no
row.

## 2026-09-17 — `view/shader-mark-strength`

**Dispatched as a VIEW lane, landing on VGEOM's slate.** The cut (#2806)
opened this program while the branch was in flight and carried
`the-shader-encodes-a-mark-strength-nothing-bounds` here by `git mv`;
the branch merged `main` in, followed the rename, and this entry
follows the row rather than staying with the lane's prefix. The
residue it filed moved the same way. `work/view/log.md` resolved to
main's side of the conflict for that reason and lost nothing — both
halves are kept, one of them in this file.

`the-shader-encodes-a-mark-strength-nothing-bounds`, filed by the
review of #2798, said the paint path a NaN reaches is the shader and
that `mark_lane` hands `Mark::strength` to the uniform raw. Both true.
The brief's ruling was to guard on the Rust side, at or before
`mark_lane`, and to say what "guard" means. It means the TYPE.

**Where the guard went, and why not the two places it could have
gone.** In the WGSL it would be a second spelling of a bound the
palette already states, in the one language whose spelling nothing in
this repo compares — and the row beside it compares constants, so it
is blind to a divergence made of a guard by construction. At
`mark_lane` it would be a refusal with nowhere to go: `prepare` writes
a uniform and cannot decline to paint, so the only thing it could do
with a poisoned weight is substitute one, and a substitution at the
last door is the emergent kind wearing a name. `theme::MixFraction`
removes the question: `Mark::strength` and `Theme::ambient` are
values of a type that has no member outside `[0, 1]`, so `mark_lane`
and `block` read `.get()`, the shader gets a weight because no other
kind exists, and there is nothing left to refuse.

**Two doors because the registry is `const`.** `MixFraction::new`
answers a caller and refuses what it cannot weight;
`MixFraction::literal` is private, takes only literals in `theme.rs`'s
own registry, and `assert!`s — which in a `const` context is a BUILD
error, so a palette stating a weight outside the range does not
compile. A `Result` cannot be unwrapped in a `const`, and that is the
whole reason there are two.

**`ambient` was not in the item and is the same defect.** A `pub f32`
on the same `pub` struct, into `base_color`'s `w` lane raw, consumed
by `ambient + (1 - ambient) * lambert`. The register's rule that a
signature change makes the compiler the sweep is what made taking both
cheaper than taking one and filing the other.

**Reachability, and it is a negative result.** No producer of a
non-number strength exists in this tree. Every `Theme` reaching
`ViewportCallback` comes from `Theme::ALL`; `strength` is read
everywhere and computed nowhere; and no public door admits a foreign
`Theme` — `gpu` is private, `ViewerApp::theme` is private, and
`ViewerApp::new`/`run` take no palette. So the crate's API lets a
consumer BUILD a poisoned `Mark` and gives it nowhere to put one. The
defect was latent. Saying so is the point: the item implied a live
paint-path hole, and a lane that reported it as one would have been
repeating the mistake the previous unit's review named.

**The base-tree red is a probe, not a route.** On `origin/main`, a
`Mark { strength: f32::NAN }` — constructible through the public API —
puts a `NaN` in the uniform's `w` lane, and a row asserting the lane
is a weight reds there. On the fixed tree that row cannot be WRITTEN,
because the `Mark` is unconstructible; the base red and the fix are
therefore not two states of one row, and the certification is seven
mutations on the fixed tree, each redding a named row.

**The float-door pair, taken from the register.** `assert_ne!` against
a float is not a distinguishability test, so the door has two rows:
`nothing_outside_the_unit_interval_is_a_mix_fraction` and
`every_weight_in_range_is_a_mix_fraction`. Neither says anything
alone — a door refusing everything passes the first, a door admitting
everything passes the second — and the mutation that shows the pair
earning its keep is `is_finite()`, the register's own *a guard that
admits everything finite is not a bound*, which reds the first and
leaves the second green.

**The parity row was one-sided and nobody had measured it.** Its own
name says it compares the constants *the palette does*, and its body
only ever read `SHADER`. Measured on the base tree: rounding
`channel_to_srgb8`'s exponent to `1.0 / 2.2` leaves
`the_shaders_srgb_curve_states_the_same_constants_the_palette_does`
**green, exit 0**. It reads `theme.rs` as code now and that mutation
reds. Separately, its sentence — *"the constants are what a divergence
would be made of"* — is deleted: #2798 made one out of a guard, and
the row now names that divergence and says where the reason it is
harmless is enforced instead of restating it.

**The sweep's residue is a file.** `gpu` is the crate's only `wgpu`
module, so its two `Uniforms` constructions and six buffer fills are
every write this crate makes to a device; the float lanes that remain
are doored by their producers or not at all, and the `f64 → f32`
narrowing is doored nowhere —
`the-viewport-and-position-lanes-narrow-to-f32-with-no-door`.

PR #2808. Signed (VIEW implementer lane `view/shader-mark-strength`,
landing on VGEOM's slate).

### What the review of #2808 moved (fix pass, same branch)

**The repaired parity row minted a fresh instance of the defect it
closed, one layer down.** The widening read `theme.rs` as a FILE, and
`channel_to_linear` — the decoder, twenty lines above the encoder —
spells `12.92`, `1.055` and `0.055` for the inverse curve. Three of
five constants were answered by a function the row is not about, under
a message naming `channel_to_srgb8`. Executed: with the encoder at
`1.06 * c.powf(1.0 / 2.4) - 0.06` and `12.0 * c`, the file-scoped row
is **green, exit 0**. `test_utils::source::item_body` was already in
the tree and scopes the read to the encoder; the same mutation reds it
now. Generalises past this row: **a row that widens a read owes the
question of what ELSE answers it**, and in a language with one file per
module the second answer is usually a sibling function rather than a
second file. The unit's own finding, re-minted by the unit's own fix.

**A claim asserted at length in doc comments is not asserted.**
`MixFraction`'s build-error half — `literal`'s `assert!` in a `const`
context — is argued in three doc comments and the PR body, and
deleting that `assert!` with the registry untouched reds **no row**
(18/18 green, exit 0). What exists is a backstop for the COMBINED edit
(assert removed *and* a bad literal written: six rows red). It cannot
be guarded the usual way — a `compile_fail` doctest cannot reach a
private item — so the honest move is to say so at the claim site, which
`## Closed` and the row's own doc now do. Q6 at the claim site, not in
a PR body.

**A test's doc promised a reading the test does not take.**
`mix_fractions_are_in_range`'s new doc said it holds *"the numbers
these three palettes state are the numbers their prose says they
are"*. It reads no prose. Its real role is the backstop above.

**A witness has to be a value the lane can hold.** The residue row
named `1.0e308` as the viewport dimension that narrows to `inf`. The
arithmetic is right in isolation and the witness is unreachable:
`f64::from(rect.width()) * pixels_per_point` is two `f32`-derived
numbers, so the lane tops out near `1.158e77`. The threshold is
`f32::MAX`, and the witness is `f64::from(f32::MAX) * 2.0 =
6.805646932770577e38` — finite, inside the lane, `aspect == 1.0`,
`as f32` is `inf`. Corrected in place with the correction recorded,
because a reader who re-derives an unreachable witness distrusts the
whole finding.

**Two shader claims rested on a caller rather than on a type, and both
now say so.** `to_display`'s `clamp` is discussed as if unconditional;
`ENCODE_SRGB`'s early return sits ABOVE it, so on an `*Srgb` surface
the pass runs no clamp at all — which strengthens the argument and was
stated wrongly. And `fs_main`'s `normalize(in.normal)` reads a
**perspective-interpolated** normal (`@location(0)` carries no
`@interpolate(flat)`, unlike `id` and `flag`), so "never a zero to
`normalize`" is an argument about the vertex WRITER; it holds only
because `scene`'s build loop pushes one face normal at all three
corners. A per-vertex normal would end that without redding anything.

**`MixFraction::new` is `const` now**, which is more than the review
asked for and is said here rather than landed quietly. The review's
point was that *"nothing constructible and legal stopped being either"*
is false — an outside consumer's `const Theme` could no longer name a
weight (E0015), because `new` was not `const` and `literal` is private.
Correcting the sentence would have left the crate with a `const` door
it kept for itself; making `new` `const` closes it, and an external
`const Mark` through `MixFraction::new` was executed as a probe and
compiles. The range test is spelled as two comparisons because a
`const fn` cannot call `RangeInclusive::contains`, with the `allow`
carrying that reason.

Signed (VIEW implementer lane `view/shader-mark-strength`, fix pass
after review).
## 2026-09-19 — Ev's grid and committed-profile rows landed (PR 2859)

Orchestrated from a session that took Ev's high-priority GUI rows
across the paused viewer programs. Not an A/B-protocol unit, by Ev's
instruction. One unit took both rows, since they share the per-lane
overlay work. The first lane (Fable) died at the account limit before
writing anything; Opus implemented the unit.

The review had a correctness lane and a style lane. It found no MAJOR:
the uniform layout, vertex-word packing, blend state and depth
behaviour all held. It found two MINORs:
- the just-added profile was drawn twice, once as its preview;
- the undrawn-profile badge had no test. The fix pass showed the path
  is reachable, through a denormal bulge.

The fix pass also took most of the style findings:
- the shader's lane-colour switch is generated from `EdgeLane`, not a
  hand list;
- lane codes are the discriminant;
- a profile-against-grid-and-preview separation test. It moved two
  palettes' profile colours.
- an absolute bound on the grid's width;
- a row for a rotated, offset plane;
- four stale docs.

Declined: merging `CommittedProfile`'s loop type with `PreviewLoop`.
`path_authoring.rs` asserts on the vertex index, and PR 2862 is
rewriting `sketch.rs`.

The review ran on reading alone: the machine-wide build slot was held
for an hour. CI is the test record.

## 2026-09-21 — `vgeom/sketch-infinity`: two guards in `sketch.rs`

Both rows were *a guard that stops a zero and admits an infinity*, and
both doors already had their recourse in the signature, so nothing was
substituted for a value the arithmetic could not compute.

`heading` guarded `length > 0.0` over `dx.hypot(dy)` and an infinity
is greater than zero, so it answered `Some([0.0, 0.0])` — a zero
vector through a door whose `None` exists to say there is no heading.
The guard is now `length.is_finite() && length > 0.0`.

**The first draft of this entry said that conjunct is also what makes
the division a unit vector. It is not**, and the review caught it. It
bounds each quotient to `[-1, 1]`, which is what the code comment
claims; it does not deliver unit length, because at the bottom of the
subnormal range the division has no precision to divide with —
`dx = dy = 5e-324` answers `[1.0, 1.0]`, of length `1.4142`. Four
separations were driven through `preview` and the driver refuses the
degenerate junction two steps earlier at `Tol::witness()` and at
`1e-12`; below `1e-12` was not tried, so that is a negative result
about the search. The residue is
`headings-unit-vector-is-not-unit-at-the-bottom-of-the-range`, filed
rather than left in a row that gets deleted.

Also corrected from the first draft: `dx.hypot(dy)` over two
`1.4e308` differences answers **`inf`**, not `1.98e308`. That figure
is the true result, which is what `f64` has no room for.

`flatten` emitted `[from.x, from.y]` above every guard, and the arc
guards it sits above are all under a `bulge == 0.0` `continue` — so a
polygon reached none of them. It now asks one predicate, `drawable`,
of every coordinate it emits.

**The second row's title says vertices and its body says *"the
population is every coordinate `flatten` emits"*, and the body is
right.** #2798's guard asks about the arc's FRAME — radius, sweep,
centre, start — and a finite frame does not make a finite point: a
major arc of radius `5.05e307` about a centre at `1.29e308` carries
its own far side past the top of the range, and nine of its 256
points come out `inf` with every guarded value a number. That is a
third arm, found by executing the arithmetic over a scan rather than
by reading the guard, and it is guarded here too.

**Every producer is authored finite literals.** The heading case is a
path with corners at `±7e307`: each vertex finite, each `d` finite at
`1.4e308`, and `hypot` the only thing in the chain that overflows.
The vertex case is `At(1e308,0)`, `Toward(1,0)`, `Line(1e308)`, whose
sum replay hands back as `inf`. This is the register's *hunt the
producer, not the input*, twice.

**A measured negative.** All three producers fail validation, so none
reaches `committed`, whose `undrawn` count feeds `frame.rs`'s badge —
that sentence stays true of every profile it counts today, and it is
narrower than the mechanism behind it, filed as
`work/vnews/the-profiles-badge-names-the-arc-case-only.md`.

Certification: the rows are red on `origin/main`'s `sketch.rs` from a
committed tree with every pre-existing row green, and on the fixed
tree each guard deleted alone reds one named row and nothing else.

**The review's two follow-ups landed here rather than after, because
both sit in rows this PR closes.** `flatten` was still spelling
`drawable`'s own question by hand over `centre` eleven lines below the
call that names it — a second copy of the predicate the same diff
introduced to unify, in the same function, with the trap named in the
PR body that did not prevent it. And `drawable`'s SECOND conjunct was
asserted nowhere: deleting `point[1].is_finite()` left the whole
viewer suite green at 637 rows, because every fixture carried its
non-finite coordinate in `x`. Two rows were added — one vertex whose
ordinate is the bad one, and one arc whose frame check is the only
thing that asks, which is the `arc_points`-answers-ONE arm where the
interior loop never runs and nothing else would notice a centre at
`[inf, 5e-7]`.

**The eps matrix caught a fixture this lane had not thought about,
twice over.** The one-segment arc row was first written at a micron
so that `arc_points` would answer the one-segment floor, and hosted
CI's `eps = 1e-6` rows refused its junction at replay two steps before
the flattener saw it — turn margin `3.75e-7 m`, which at that
tolerance is tangency. The arc is millimetre-scale now and buys the
floor from a COARSE display tolerance instead, which is a δ the caller
chooses. **A fixture whose scale is near a gated eps row's is a
fixture about that row**, and the lane's local verification had been
single-eps, which is how it reached CI.

Running the viewer suite at all three rows afterwards turned up a
SECOND red that is not this lane's and that hosted CI cannot see:
`pane::profile::tests::drawing_a_locked_split_circle_above_the_cap_leaves_it_alone`
fails on `main` at `eps = 1e-6` — a `chord_side` margin of `1.127e-6`
inside that row's escalation band — and the viewer's `app`-feature
rows run in exactly one CI step, at the default eps only. Filed as
`work/chrome/a-split-circle-fixture-sits-inside-the-1e-6-escalation-band.md`
and `work/ciw/the-viewer-app-feature-rows-gate-one-eps-of-three.md`.

**And the sweep was tree-only, which the plan's rule says is half of
one.** A tracker pass would have reached
`the-viewport-and-position-lanes-narrow-to-f32-with-no-door`, whose
recorded negative result — *no in-tree producer of a non-finite value
in any of these lanes* — this unit's three fixtures falsify:
`push_segment` narrows exactly what `flatten` emits, and `7e307_f64
as f32` is `inf`. That row now carries the producer, and the reading
it forces back onto this unit is that at these magnitudes the picture
is already nowhere one door along — so *"the production consumer
reaches it"* is true of the door and an overstatement about the
drawing.

## 2026-09-21 — `vgeom/refusal-floor`: Order item 1, four doors in one class

Four rows, one argument, four files:
`a-count-slot-launders-a-typed-nan-into-zero` (P0, `props.rs`),
`world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure`
(`input.rs`), `finite-bounds-yield-an-infinite-scene-radius`
(`camera.rs`) and
`corner-count-substitutes-u32-max-for-a-length-it-could-not-cast`
(`gpu.rs`). All four closed. The unit's shape is CHROME's
`chrome/datums-substitution-sweep` (#2644) rather than a second one:
the door answers an `Option` or a `Result`, the callers take the
refusal through the nothing-to-do they already have, and the doc
comment says why a floor or a substitute would be the wrong repair.

**What the class turned out to be, stated more narrowly than the plan
states it.** The plan's Order says *every row here is a non-finite
value reaching a place that assumed it could not*. Three of the four
are not that. `world_per_px` is the plan's shape exactly — an ordering
used as a domain test on a value with no order. The other three are a
**guard sited one arithmetic upstream of the overflow it is for**:
`camera.rs` bounds `lo` and `hi` and the loss is in `half[i]²`;
`props.rs` splits on the dimension before any literal exists, so the
finiteness refusal it points at is never on the path; `gpu.rs`'s cast
is downstream of a length nothing bounds. In all three the guard is
correct about what it looks at, which is why each survived a sweep.
The useful test is not *does a non-finite value get in* but **does the
door's own answer get asked the question the door's prose asks of its
inputs**.

**What the sweep found that the four rows did not predict.**

- **The same cast has a finite arm.** `SlotValue::of`'s `value as i64`
  saturates for `1e30` as surely as for `inf` — `i64::MAX`, executed —
  and that half is NOT fixed here, because unlike the non-finite half
  it has no refusal to raise: `DimensionError::NonFiniteLiteral`
  exists and says the right sentence, and nothing anywhere says *this
  count does not fit*. Minting one reaches `crates/editor-core`
  (EDIT's and MSOLVE's) or `session::Refusal` (VNEWS's). Filed on this
  slate as
  `a-count-slots-cast-still-saturates-for-a-finite-value-too-large`
  with the fork written out.
- **Fixing the P0 created a news gap.** The drag path maps the new
  refusal through `Refusal::Dimension` and it reaches the status line;
  the TYPED path has no operation to carry one and is silent. Better
  than committing zero, and not what `field_edit`'s doc promises.
  Filed on VNEWS as `a-refused-typed-value-reaches-no-word`, beside
  `undo-and-redo-are-disabled-in-silence-over-a-refusal-that-has-words`,
  which may want the same answer.
- **`camera.rs`'s centre overflows too**, at the same site —
  `lo = 1e308`, `hi = 1.7e308` gives `centre = [inf, 0, 0]` — and it
  needs no guard: scanned over every pair of magnitudes from `1e150`
  to `1e308` in both signs and the first 64 floats above each, 126
  non-finite centres and **none with a finite radius**. So the radius
  guard covers it, and the argument is in the doc comment rather than
  in an assertion nothing can break.
- **`world_per_px`'s quotient overflows independently of its height.**
  The row is about the height; the same door divides an ordinary
  visible height by the smallest subnormal to infinity. Guarded here,
  for #2644's stated reason.

**Two reachability claims are NOT upgraded by this unit, and the PR
says so in the same words the rows do.** `world-per-px` carries a
*"Reachability: no producer found"* over this crate's own writers of
`ViewportSize` — not a proof about egui's layout — and nothing here
searched further. `corner-count` says outright it is a door hardening
and not a reachable defect: `u32::MAX + 1` corners is 51.5 GB of
positions and the allocation is the bound, not the code.

**Each row's table re-derived rather than quoted**, per the register:
`camera.rs`'s four executed rows reproduce under a verbatim replica of
`sphere`'s arithmetic, and the threshold is sharper than the row's *a
few hundred orders of magnitude* — one axis overflows above `1e154`,
three above about `7.7e153`.

**Mutation, five ways, one row each.** Deleting the camera radius
guard, restoring `height_px <= 0.0`, dropping the quotient's
`is_finite`, restoring `unwrap_or(u32::MAX)` inside `draw_range`, and
deleting the `Count` arm's `is_finite` conjunct each red exactly one
of the four new rows and nothing else in the 798-row app-feature suite
(besides `gpu::tests::every_pass_builds_on_a_real_device`, the
standing WGPU-adapter red on a box with no Vulkan).

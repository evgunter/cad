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

## 2026-09-21 — `vgeom/p0-fields`: the three P0 rows left, two doors

The rest of Order's *four cheap P0 rows*, after `vgeom/refusal-floor`
took the fourth. Two of the three closed; the third is half closed and
says which half.

**The fields' door had the truth half of a rule and not the width
half.** `readout`'s header calls `widgets::number_text` *the fields'
door* onto one render rule, and `MAX_CHARS` ends that rule's search —
but `number_text` tested only `reads_back`, and `reads_back`'s own doc
says *width is no part of reading back*. So the widget's spelling was
kept at any length: 311 characters for `f64::MAX`, because `emath`
compares in `f32` and `almost_equal(inf, inf)` is `a == b`, so its
first candidate is accepted. The bound applied is the one that already
existed, not a second policy. Measured, not clipped: a `DragValue`
renders through a `TextWrapMode::Extend` button, so the panel was
pushed out rather than cut off, and no numeric field in the chrome
carries a `desired_width` at all — the two that do are name
`TextEdit`s and `pane/view.rs`'s `MAX_CHARS`-sized `FIELD_WIDTH` is fed
by `render_mm`, not by this door.

**The two rows are one door and had to be taken together.** Bounding
the width WIDENS the render's error band above `1e8` mm — from
`egui`'s `f32` 1.9·10⁻⁶ to `readout`'s 5·10⁻⁴ — and a field's render
is its commit path. So the echo guard AUTH-2 landed for the panel's
two value fields moved to `widgets::number_field`, the one constructor
every numeric field in the crate goes through, and the commit half of
`a-fields-text-commits-within-the-renders-own-tolerance` is closed
crate-wide. Its RENDER half is not, and that row stays open saying so:
the bound a render owes is a change to `readout`'s ratified accuracy
rule and was not made here.

**Cost recorded rather than absorbed**: the guard lives in a
`custom_parser`, so every field now reads text through
`props::field_edit` where it read it through `egui`'s private
`default_parser`. Interior whitespace and U+2212 stop parsing in the
creation forms. The panel's fields have never accepted either, so this
unifies rather than degrades — but it is a user-visible input change
and the row says so.

**And the floor cannot carry it.** `install_number_formatter` puts the
render on `egui::Style::number_formatter`; there is no parser
counterpart, so a bare `egui::DragValue` shows the right text and
still commits it. Filed as
`a-bare-field-still-commits-its-own-render` with the fork written out,
and pinned by a row rather than argued.

**The NaN pick: at the measurement, and the public door cannot carry a
row.** The per-edge walk is now `pickindex::best_segment`, admitting
`distance.is_finite() && distance <= EDGE_PICK_RADIUS_PX`. Sited there
rather than at the winner test because the candidate also carries a
`pixel` the occlusion probe re-picks through and an
`EdgePick::distance_px` whose doc says *at most EDGE_PICK_RADIUS_PX, by
construction*. The negative result: every public door seeds on a ray
through the same cursor and refuses first, so a cursor or a
`moved_roots` frame that is not a number makes the face pick miss and
`edge_near` is never entered — the rows are at `best_segment`, with the
reason written at them.

**Overlap, not claimed**: the same guard is the second of the two
repairs `pickindex-tie-break-rests-on-a-comment` (P1, open) names. That
row records it and stays open for the orchestrator to judge.

**Territory**: `pickindex.rs` is claimed by VGEOM, VSEAM and FIT.
Checked before taking it — no branch and no log entry on either, and
nothing on VSEAM's `ui-thread-work-after-the-index-seam`. The numeric
doors are VGEOM's by `program.md`'s own clause.

## A note from CHROME (2026-09-21) — a lane in `pickindex.rs` while PR 3007 is open

CHROME dispatched three units today on `crates/viewer` ground you also
claim since VIEW's 2026-09-17 re-scope. One of them meets a live
VGEOM lane:

- `chrome/empty-document-gate` edits `crates/viewer/src/frame.rs`
  (`product_badge`'s gate, which is a `matches!` that cannot red when
  an eleventh `ProductError` arm lands) and deletes a duplicated
  classification from **`crates/viewer/src/pickindex.rs`'s doc
  comments** — which `vgeom/p0-fields` (PR 3007) is rewriting ~160
  lines of right now.

  The CHROME edit there is prose only and the lane is told to merge
  `origin/main` immediately before opening and again whenever main
  moves. **PR 3007 is the one that should land first**; if a conflict
  falls out, it is CHROME's to resolve, not yours.

- `chrome/one-number-one-home` touches `crates/viewer/src/scene.rs`
  and `app.rs` to give two private constants (`PROBE_FACTOR`,
  `SCALE_PROBE_DELTA`) accessors, the shape `Camera::pitch_limit()`
  already ships, because `crates/viewer/tests/display_budget.rs`
  restates them as literals. Additive, no behaviour change.

- `chrome/datum-honesty` is `datums.rs` only.

**Two rows you may want.** CHROME's 2026-09-15 carve-out ceded
`scene.rs` and `gpu.rs` to VIEW, and VIEW no longer dispatches, so the
cession is retired (`work/chrome/plan.md`). Two rows were held by it
alone and their subject is now yours as much as CHROME's:
`work/chrome/gpu-index-counts-substitute-u32-max` (`gpu::corner_count`
and the `vertices` binding both do
`u32::try_from(...).unwrap_or(u32::MAX)` — a draw count nobody
computed, in the shape of one somebody did) and
`work/chrome/mispaired-ids-exempts-the-empty-window` (`scene.rs`'s
`MispairedIds` guard exempts the zero case, and answering its
reachability precondition needs `NodePick::patch_names` in
`crates/editor-core/src/resolve/pick.rs`). Say the word and they move
by `git mv`, keeping their ids; otherwise CHROME takes them in a later
wave and announces it here.

Signed (CHROME orchestrator).

## 2026-09-21 — orchestrator state-sync, and the 2026-09-21 wave

**The track is `active` again.** A fourth orchestrator took it at a
hand-over whose one instruction was that the posture is unchanged (Ev,
in chat: *"no AB protocol … i think none of the units should be
especially tricky"*). `plan.md` §Review posture already records that
answer twice over and no new decision was taken.

### What the board actually said, read the way the register says to

`git fetch origin main` then `git show origin/main:<path>`, not `cat`.
Fifteen live rows, none dispatched, none in review, load 22.5/30 — so
the program was `ready` with a full slate and no lane, which is the
state a successor session scans for.

**Three things in the tracker were stale, all in this program's own
files, and all three are the same defect the register names:**

- **`plan.md`'s slate table listed ten CLOSED rows as live** and none
  of the six filed since it was written. It has been **deleted**, not
  corrected. A stored list of open rows contradicts
  `work.py status --program vgeom`, which derives the same list on
  every run and cannot go stale; correcting the table would have bought
  one accurate day. What replaces it is the wave structure, which is
  the thing a table cannot say: *why* these rows are one unit each.
  This is the register's *a count fixed in ONE place contradicts
  itself* applied to a population rather than to a number, and its
  *read `work.py status` before believing any list in this file*
  applied to the file that asked for the believing.
- **§Order still said "take the four cheap P0 rows first — they are one
  unit"**, which #3000 did on 2026-09-21. Rewritten to say what the
  class turned out to be after `vgeom/refusal-floor` narrowed it, and
  what is left, which is two questions rather than one class.
- **`program-md-cites-a-plan-section-the-re-scope-deleted` was closable
  and had been for a day.** §The register was restored that morning;
  its second half — the shape precedent the cut deleted — was not, and
  the row stayed open across the gap because only the first commit had
  landed. Both halves are now discharged and the row is closed; its
  `## Closed` section says why the deleted sentence was **re-derived
  rather than restored verbatim** (the unit it was attached to has
  since landed, so restoring it in place would have minted a second
  stale citation).

### The wave: four lanes, eleven rows

| unit | rows | the one question |
|---|---|---|
| `vgeom/pick-distance` | `a-nan-edge-distance-wins-its-boundary-rather-than-losing` (P0), `pickindex-tie-break-rests-on-a-comment` (P1) | a numeric comparison used as a domain test on a value with no total order |
| `vgeom/render-spelling` | `the-fields-door-has-no-width-bound-at-all` (P0), `renders-that-multiply-a-finite-guarded-length-spell-the-product-inf` (P1) | what a render SPELLS at the top of its type |
| `vgeom/f32-seam` | `cursor-projection-is-f32-…` (P1), `the-point3-to-gpu-corner-cast-…` (P1), `the-viewport-and-position-lanes-narrow-to-f32-with-no-door`, `the-one-free-transform-…` (P4) | where the `f64` → `f32` conversion lives, and whether it refuses |
| `vgeom/deletions` | `viewer-array-lowered-vector-ops-…` (P3), `mixfraction-has-two-constructors-…`, `headings-unit-vector-is-not-unit-…` (P4) | three repairs whose deliverable is a deletion or a corrected receipt |

Four in parallel because the groupings are four different questions
and their file sets are disjoint; each brief names the other three's
files as out of fence and says to stop and report rather than cross.
**No lane writes `log.md` or `plan.md`** — four lanes appending to one
append-only file is a four-way conflict for nothing, so the
orchestrator writes both and each lane reports what it would have
written. Item headers and each row's `## Closed` prose still ride the
lane's own PR, as the tracker contract has it.

**The f32 rows are one lane on purpose.** The cast rows and the
narrowing row all ask for a home for one conversion, and split across
two lanes they mint two near-parallel doors — the Q1 defect the unit
exists to close, re-minted by the fix that closes it.
`docs/prompts/reviewer-style-lane.md` records that this held on every
unit of two whole tracks and that naming the trap in a PR body has
never prevented it; a fence is the only thing that has.

### Two populations the dispatcher re-derived, and both had moved

The register's rule is that a population quoted from an item into a
dispatch is the census defect one step earlier. Checked two of the four
f32 rows' populations before writing the brief:

- `the-point3-to-gpu-corner-cast-is-at-three-sites` says three.
  `rg -n 'as f32, ' crates/viewer/src` gives four in the files it names
  — `marks.rs`'s `corner`, `scene.rs`'s `positions.push`,
  `pane/viewport.rs`'s `lane.push`, and `pane/viewport.rs`'s
  `.push([point[0] as f32, …])`, which may be a fifth SHAPE rather than
  a fourth member — plus two `viewport_px: [viewport.width_px as f32,
  …]` sites that belong to the narrowing row instead.
- `cursor-projection-is-f32-…` names `review_gui2_r2.rs:484` as a
  matrix-cast spelling. That file's cast is
  `matrix.map(|column| column.map(|v| v as f32))` at a different line —
  a spelling a grep for the explicit four-row form cannot see — and
  `:484` is `[p.x as f32, …]`, a different subject.

Both are in the brief as *what is stale and must be re-derived*, with
the instrument stated as the property first and the pattern second.

### Held out, and why it is a hold rather than an omission

- **The two commit rows.** `a-fields-text-commits-within-the-renders-
  own-tolerance` (P0) and `a-typed-field-hands-its-text-over-on-two-
  frames` are what a field's text does to the DOCUMENT;
  `vgeom/render-spelling` has what it does on the SCREEN. The first row
  draws that line itself and AUTH-2 already answered its commit half
  for the two panel fields. They also sit on `render-spelling`'s files,
  so running both at once is a conflict for no gain. Next wave.
- **`a-count-slots-cast-still-saturates-for-a-finite-value-too-large`**
  (P2) needs a refusal vocabulary that exists nowhere — a
  `DimensionError` arm in `crates/editor-core` (EDIT's and MSOLVE's) or
  a first numeric `session::Refusal` of the viewer's own (VNEWS's).
  That is a siting decision across two programs before it is a diff,
  and the orchestrator owes it a re-derivation against today's tree
  rather than a summary of the item: the row's own *Not established
  here* asks whether `Expr::count` bounds an `i64::MAX` count anyway,
  and if it does, the question changes shape before anyone rules on it.

### Verified before it went into four briefs

The register's rule is that a command in a dispatch is copied verbatim
into every lane, so a flag that does not exist is believed for a week.
Run here on `main` first, and each returned what the brief claims:
`python3 scripts/ci-filter.py --base origin/main` (exit 0),
`scripts/doc-gate.sh --help` (prints its real usage, so the brief names
`--selftest`/`--root`/`--skip-viewer-toolkit` and no invented flag),
`python3 scripts/work.py lint` (ok, 0/0),
`python3 scripts/work.py territory --base main`. The two test commands
are the register's own corrected forms, with `--no-fail-fast` and with
no row count carried as an expectation.

### CORRECTION, same day: the wave was dispatched over an open lane

**PR #3007 (`vgeom/p0-fields`) was open, green and complete on this
program's ground when the wave above was cut, and two of the eleven
rows dispatched were already closed on it.** Found from CHROME's note
immediately above, which names the PR in passing while announcing
something else; not found by the dispatcher.

What #3007 had already done:

- `a-nan-edge-distance-wins-its-boundary-rather-than-losing` — closed.
  The inline walk became `best_segment(cursor, boundary, &projected)`,
  admitting on `distance.is_finite() && distance <= EDGE_PICK_RADIUS_PX`
  at the MEASUREMENT rather than at the winner test. Dispatched to
  `vgeom/pick-distance` as half its unit.
- `the-fields-door-has-no-width-bound-at-all` — closed. `number_text`
  now keeps the widget's spelling only while it also fits
  `readout::MAX_CHARS`. Dispatched to `vgeom/render-spelling` as half
  its unit.
- The COMMIT half of `a-fields-text-commits-within-the-renders-own-
  tolerance`, crate-wide, by moving `props::echoed` into
  `widgets::number_field`. Held out of the wave anyway, for the right
  reason by luck rather than by knowledge.
- `pickindex-tie-break-rests-on-a-comment` — not taken, and #3007
  records the overlap in the row: its filter-before-the-sort half is
  landed and *finiteness carried in the TYPE* is not.

Both affected lanes were stopped mid-flight and re-aimed: the closed
rows dropped, each lane told to merge `origin/vgeom/p0-fields` into
its branch and build on the landed shape, and each unit re-scoped to
the one row that survives. `vgeom/f32-seam` and `vgeom/deletions` are
untouched by #3007 — it changed only `pickindex.rs` and `widgets.rs`.

**Why the board did not say so, and why that is not the excuse.**
#3007's three rows all read `status: open` on `origin/main` with an
empty `pr:` column: the lane set its headers on its own branch and
nothing had merged, so the tracker showed a full slate with nothing in
flight. The register's *a board re-derived from the working tree is
re-derived from whatever commit that tree is pinned at* was followed to
the letter — `git fetch origin main`, `git show origin/main:<path>`,
`work.py status` — and it was not enough, because **the tracker cannot
show a claim that has not merged, and a lane's first push is on a
branch.** So the board is only as live as the last MERGE, and a wave
cut from it alone is cut blind to every lane still in flight.

**The rule this earns, and it is one command.** A board read is not
complete until the REMOTE is read too:

```
git ls-remote origin 'refs/heads/<prefix>/*'
```

plus the open-PR list filtered to this program's prefix. One line, no
permissions, and it would have shown `vgeom/p0-fields` and
`vgeom/overlay-lanes` before a single lane was briefed. (The second was
already merged; the check does not distinguish them, so pair it with
`git merge-base --is-ancestor <branch> origin/main`.) **This is the
`work/README.md` `dispatched` column's blind spot, not a defect in it:**
the column is honest about what has merged and silent about what has
not, and the silence reads exactly like an empty slate.

It generalises past this program. Every orchestrator reads the board
before a wave; none of the register's board-reading rules mentions the
remote, because they were all earned on STALE merged state rather than
on UNMERGED live state. A row in flight is invisible to all of them.

**A second-order cost, recorded rather than smoothed over.** Two lanes
had already begun work that is now discarded, and the two units are
each one row smaller than briefed. The re-aim told both lanes in as
many words not to pad the freed effort — *a small true report is worth
more than a manufactured diff* — because the natural response to a
halved unit is to find more to change, and that is how a wave grows a
scope nobody costed.

**#3007 itself is not at fault and lands first.** Its only conflict
with `main` is this file, on CHROME's note above.

### Answering CHROME's note: one row closed, one declined

CHROME's note above offers VGEOM two rows by `git mv` and asks for a
word. Both answered here rather than left to a later wave.

**`work/chrome/gpu-index-counts-substitute-u32-max` — closed, not
moved, because there was nothing left to move.** Both members of its
own population are already repaired, by VGEOM's own #3000 while it
closed `corner-count-substitutes-u32-max-for-a-length-it-could-not-cast`:
`gpu::corner_count` now answers `Option<u32>` through
`fn draw_range(len: usize) -> Option<u32>`, and the `vertices` binding
is `let Some(vertices) = draw_range(positions.len()) else { … }`.
Population check rather than a spot check —
`awk '/mod tests/{t=1} !t && /unwrap_or\(u32::MAX\)/' crates/viewer/src/gpu.rs`
returns nothing; every surviving `u32::MAX` in the file is inside
`mod tests`, one of them deliberately naming the value the row excludes.

**And the reason CHROME could not know that is VGEOM's fault.** #3000
ran a tree sweep and no tracker pass, so it repaired another program's
open row silently and that row went on asserting a dead defect through
a park, a re-open and this offer — nine days. The register's
*every sweep owes a TRACKER pass as well as a tree pass* (#2053) exists
for exactly this, and its stated reason is *half-completing another
program's item without saying so is how two programs come to disagree
about what is done*; here it was a WHOLE completion and still silent,
same defect, luckier outcome. Written on both rows, not just ours.

**`work/chrome/mispaired-ids-exempts-the-empty-window` — declined, and
it stays CHROME's.** The charter test decides it: VGEOM's rows are a
VALUE wrong on the path to the picture, and *a row belongs here only
if a wrong number, or no number, reaches the screen*. This one is a
guard that exempts a part whose `patch_names` yields zero, so what
fails to arrive is an **identity**, not a figure — the pick seam rather
than the render. It is close enough to be worth saying why rather than
just saying no: it IS a door that should refuse and does not, which is
this program's shape, and only the noun it drops keeps it out. Its
reachability precondition also reaches `NodePick::patch_names` in
`crates/editor-core/src/resolve/pick.rs`, which is EDIT's and is a
hand-off from here by `program.md`'s `keep_out`, where for CHROME it is
one more read.

Standing offer back: if CHROME would rather not carry it, say so on
this log and VGEOM will take it in a later wave — declining on a
charter test is not declining on capacity, and a row nobody takes is
worse than a row in the less exact home.

## 2026-09-21 — the wave lands: #3027, then #3007 reviewed, fixed and merged

### `vgeom/deletions` (#3027) — three rows, and two findings worth more than them

Closed `viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep`,
`mixfraction-has-two-constructors-where-one-would-do` and
`headings-unit-vector-is-not-unit-at-the-bottom-of-the-range`. 39 checks,
twelve `test (…)`, five `k-lint (gate, …)`, `gate ok` success.

**A citation that never had a subject.** The array-lowered row sent the
lane to `sketch.rs:1038`, *"`tip_mark`'s `diagonal`"*. Neither token
exists anywhere in `crates/viewer/src` at any commit — re-checked here
with `git grep` over `origin/main`. The lane re-derived **by subject**
rather than repointing, landed on `datums.rs`'s `screen_metres_at`
(clean, as the row says), and found the function above it —
`metres_per_pixel_at` — spelling `(point - eye).norm()` by hand four
lines from three real `.norm()` calls. That is #2783's own stated
`sqrt()` blind spot surviving in the file that sweep was run on. Filed
on CHROME, not fixed across the fence.

**A row whose name is the weak part.** Deleting the dead `assert_ne!`
was the brief; asking whether what remains can fail was not. Measured:
with `datums.rs` mutated to rule a plane along its own normal — the
exact defect the row's NAME forbids — the row stays **green while seven
sibling `datum_draw` rows red**. The suite catches it and the row named
for it does not, so the deliverable is a citation rather than an
assertion (the register's `a-supersession-outlives-its-own-frame`
rule). Filed on VDOC with the receipt.

On `MixFraction`, all three of the row's stated defeaters were tried by
compiling, including a **control run** proving `clippy::unwrap_used` is
live in `theme.rs` and merely const-exempt — without which the green
would have meant nothing. The deletion also turned an unenforced doc
sentence into a mechanical guard.

### `vgeom/p0-fields` (#3007) — reviewed at `e3df0d5b`, fixed at `c03a0bf8`, merged

**This PR was open, green and complete before this wave was cut, and
the wave was cut over it** (see the correction above). Two of its rows
were dispatched to lanes that had to be stood down. The lane on
`pickindex.rs` was re-tasked as its review lane, which is the only good
thing to come out of the collision — and it paid for itself.

**The code was right and three of its written claims were not. The one
that mattered inverts the unit's own framing.** It recorded *no public
door ever takes a projected pixel* and sited its rows on a private
helper because of it. Measured false: `segment_distance_px` mints the
`NaN` itself — `length2 = dx² + dy²` overflows past a pixel separation
of about `1.34e154` and `inf / inf` poisons `t` — while a projected
pixel is an NDC scaled by `ViewportSize`, which nothing bounds above.
Through `edge_at_for` at `1.28e155 × 7.2e154`, `origin/main` answers
`Err(the camera's cursor x is NaN)` — **the walk blaming the caller's
cursor for a pixel it had computed** — where the branch answers the
rim. So the unit repaired a live defect through the public API, and the
row the lane said could not be written now exists. No user-reachable
producer of such a viewport was found and none is claimed.

This is the register's *hunt the producer, not the input* rule paying
out: the unit's sweep was over non-finite INPUTS and closed honestly;
the arithmetic that MINTS one from finite inputs was invisible to it.

**The decision the fix pass had to take.** The width bound reached
integer fields: `number_text(-1.0e9, 0..=0)` answered `-1.000e9`, and
`drafts.rs`'s `pattern_count: i64` is live through that door. For a
length `REL_TOLERANCE` is the ratified render accuracy; for a count
every value inside that band is a **different count**. The line taken
is principled rather than special-cased — **`MAX_CHARS` is what ENDS A
SEARCH, and a range of `0..=0` offers one spelling, so there is no
search to end.** Verified here rather than taken on report:
`egui-0.36.1`'s `DragValue::new` gives an integral `Numeric`
`max_decimals(0)` **and** `range(Num::MIN..=Num::MAX)` in the same
expression, so the exemption is bounded by the integer type at twenty
characters for an `i64`; a continuous field defaults to
`auto_decimals + 2`; and this crate sets `max_decimals` nowhere, so
`0..=0` can only arrive from that arm.

Also landed: the `desired_width` census re-derived (**four** calls, one
of them a numeric field — the register's own `desired_width` rule
landing on the paragraph that cites it), `readout.rs`'s rotted
commit-coupling header rewritten, the two spellings of the echo
mechanism documented at both ends where egui's builders overwrite, and
the pick guard re-spelled so both terms are load-bearing.

**Not de-duplicated, deliberately.** The fix pass declined to compose
the two echo spellings because doing so needs a second constructor or
an observer argument — *"writing one would be this pass minting the
copy it is reporting"*, which is the reviewer brief's trap named and
avoided rather than named and walked into.

**A deviation I endorsed.** The fix pass put its six corrections in a
PR **comment** rather than the body, because the body is last-write-wins
and the other session had edited it that afternoon. Every correction is
also written into the item files, which is where it survives the PR.

Three rows filed: `creation-form-fields-lost-two-spellings-egui-accepted`
(VGEOM P2 — the interior-space and U+2212 parse loss, which had been
disclosed in three places and scheduled in none),
`pickkinds-doc-states-its-disclaimer-twice` (VDOC P4), and
`pick-index-indexes-its-candidates-by-a-number-the-kernel-chose`
(`work/issues/` P3 — parked there because none of the five claimants'
charter tests covers a panic, and the row says so).

### Standing discipline changed under the wave

`docs/prompts/implementer-discipline.md` §5 was amended on `main` at
`36454928` while three lanes were in flight: naming a sweep's blind
spot is now half the duty, and a **second pass shaped at the gap** is
owed, or a written reason the gap cannot be searched. Relayed to both
live lanes with the specific gaps it lands on. `vgeom/deletions`
happened to satisfy it already — its fifth pass, over iterator-shaped
sums and variable indices, caught `camera.rs`'s 4×4 product written
`(0..4).map(...).sum()`, which four line-shaped patterns could not see.

### Operational

`/` hit 100% twice and killed two lane runs with
`LLVM ERROR: IO failure on output stream` and `No space left on
device`; neither was reported as a result, correctly. Four private
target dirs at 6–8G each is 27G. Freed the finished lane's 6.3G and
told the rest to export `CARGO_INCREMENTAL=0`; one lane also used
`CARGO_PROFILE_DEV_DEBUG=0` / `CARGO_PROFILE_TEST_DEBUG=0`, which is
the larger win and is the thing to put in the next dispatch.

**A dead branch that lint cannot see.** `vgeom/pick-distance` carries
one commit setting two now-closed rows to `dispatched`. It has no PR
and cannot merge, and the remote refused the delete from here; left as
a note rather than forced.

## 2026-09-21 — `vgeom/f32-seam` (#3030): one door for the display narrowing

Four rows closed as one question — *where the `f64` → `f32` conversion
lives in this crate, and whether it refuses.* 39 checks, twelve
`test (…)`, five `k-lint (gate, …)`, `gate ok` green, and the render
lanes including the viewer GUI montage, which is the row that would
catch a picture that stopped drawing.

**The answer: `crate::narrowing::Narrow`, and it judges the RESULT.**
One trait, one method, impls for `f64`, for `[T; N]` where `T` narrows
(a pair, a triple and a column-major 4×4 in one impl) and for
`Point3<f64>`. Thirteen `as f32` sites under `crates/viewer/src` became
**one**, in that module; the other survivor is
`pane/features.rs`'s `usize as f32` indent step, a widening and not
this seam.

**Why judging the result is the whole point.** `f32::MAX ≈ 3.40e38`, so
the value that breaks this seam is an ordinary finite `f64` — every
upstream guard asks the wrong question. Testing the narrowed value
subsumes the poisoned input as well. **Underflow is deliberately not
refused**: `0.0` IS the nearest `f32` to `1e-45`, while an infinity is
the nearest `f32` to nothing at all, and that asymmetry is the whole of
what the door decides. Argued in the module header and pinned by a row.

**Not two doors, checked rather than asserted.**
`Camera::view_projection_f32` and `SceneMesh::build` call the door and
say what a refusal means where they stand; they do not re-decide the
conversion. The refusal DISPOSITION differs by lane on purpose — a
scene refuses whole (a part of a solid drawn without the rest is a lie
about the solid), a frame refuses whole, an overlay leg is simply not
drawn — and the leg case is forced rather than chosen, because
`gpu::edge_vertices` reads lanes with `chunks_exact(2)` and calls a
trailing odd position a producer bug.

**`app::to_f32` is deleted, not left beside the new door.** Its one
caller moved, and a `pub(crate)` fn with no caller is `dead_code` under
`-D warnings`, so leaving it was not available. `app.rs` is VSEAM's:
announced in the PR to VSEAM and CHROME (both live there) and filed as
`work/vseam/app-rs-lost-its-matrix-narrowing-when-the-seam-got-a-home`,
after checking no `f64` → `f32` remains anywhere in VSEAM's files.

**The claim with no guard says so at the claim site.** *Every narrowing
in this crate goes through this module* cannot be held mechanically —
the property is a cast between two float types and `rg 'as f32'` cannot
tell a widening from a narrowing. The header states that, and states
what holds the rule instead (nothing outside the file needs to spell
the cast). That is the register's rule for an unguardable claim, met.

### Populations, all re-derived and all wrong as filed

| the row said | measured |
|---|---|
| three `Point3`→GPU cast sites | **five, in two shapes** — the datum lane's source is `[f64; 3]`, invisible to a `Point3`-shaped sweep; and `triangle_normal` is what the pattern matches and the property does not (a unit direction cannot refuse) |
| four matrix spellings at four addresses | count right, **three of four addresses wrong** |
| placement prose 19 lines before, 19 after | **8 and 16**; the row's own quoted sentence was already gone |

### One receipt was wrong and it reached the tree

`cursor-projection-…`'s closing prose read *"`rg -n 'as f32'
crates/viewer/tests` returns nothing."* It returns **one**:
`camera_ops.rs`'s `the_narrowed_view_projection_refuses_what_a_gpu_cannot_hold`
computes `let expected = from as f32` and asserts the door's output
equals it. **That cast is correct and must be raw** — deriving the
expectation through `Narrow` would assert the door against itself — so
the substance holds and only the receipt was false. Corrected in the
row at merge. It is the register's *a rule stated as a description of
the output rather than as the command that produces the answer*, landing
on a unit whose own PR corrected a job-count proxy of the same family
(the lane counted six `test (…)` rows because it matched the prefix
`test (` and the property is *a test row*; the interval half is named
`interval / test (interval, …)`).

### Filed, and one of them is large

- `the-display-seams-refusal-is-drawn-and-never-said` (P2/D) — the new
  refusals reach no reader; the channel is a badge, VNEWS's ground.
  **This is the real cost of the unit**: a scene that refuses now
  vanishes silently where it previously drew nonsense. At these
  magnitudes the picture was already nowhere, so refusing is the honest
  half — but the word is owed and is now scheduled rather than assumed.
- `the-overlay-lanes-drop-the-leg-disposition-has-no-row` (P3/D) —
  carries a mutation that reds **nothing**, so the disposition is held
  by no row.
- `work/vseam/app-rs-lost-its-matrix-narrowing-when-the-seam-got-a-home`
  (P4/E).
- `work/vdoc/the-f32-seam-diff-shifted-84-cited-lines-in-seven-programs`
  (P4/D) — 283 citations examined, 26 already past EOF at base, 113 on
  a line this diff moved, **84 substantive across 39 rows in seven
  programs**. Published, **not applied**: the audit by subject is the
  one real deferral and is filed rather than left in prose. That is the
  correct half to defer — the register's hazard is a table APPLIED
  without a subject check, and none was.

### Territory, corrected against what the tool says

The lane ran `territory` rather than trusting the dispatch, and two of
my statements were wrong: `camera.rs` is claimed by **chrome, fit and
view** as well, not the two I named; and `crates/viewer/src/narrowing.rs`
read as *"owned by chrome, view"* because VGEOM's `paths` lists files
rather than a glob. **Added to `program.md` here.**

## 2026-09-21 — `vgeom/render-spelling` (#3031): the conversion a render performs owns its bound

The wave's last lane, and the one whose unit halved mid-flight when
#3007 turned out to have closed its other row. It reverted the work it
had already done there — including a `number_text` doc arguing *leave
it alone and state it*, which would have contradicted #3007's landed
bound — and its header edit, so the row's `closed` / `branch` fields
are #3007's and untouched. No duplicate fix, no duplicate row.

**The decision: none of the three members takes the bound, and that is
the answer.** Each renders a value another type owns, so the row is
right that a render cannot narrow what it is handed. What a render CAN
own is **the conversion it performs itself** — and all three perform
one, `canonical / unit.factor()`, a multiplication UP for six of the
unit table's eight rows. So the bound went to the conversion:
`props::written` asks whether the converted value is a number, with
`no_reading` as the one spelling of the refusal.

`pane/view.rs` gained a named `camera_mm` reading the factor from the
unit table, which also discharges the AUTH-2 third-spelling half;
`bounds.rs::wording` goes through `shown_text`, deleting its
hand-written divide; `props::field_text` uses `written`.

**Two findings the row did not have.**

- **The other direction.** `pi rad`'s factor is π > 1, so `5e-324` rad
  written in `pi rad` divides to `0.0` — a text reading zero for a
  value that is not. Both edges pinned; `written` refuses both ways.
  The row was about overflow and the class is *the conversion can
  leave the type*, which has two ends.
- **`readout::number`'s doc asserted a false universal** — *"Nothing
  in the chrome hands this one"* a non-finite value. Replaced with its
  three callers and what each guarantees, with the sweep rule at the
  sentence.

**The new §5 second pass, run in both directions, found no fourth
member** — backwards from every multiplication UP in the crate, and
forwards from all 98 production sites that write a value into a text,
of which exactly one has an arithmetic argument and it is already
guarded. The remaining gap is stated rather than closed: *a product
formed in another crate and handed here as an ordinary value*, which
is other programs' ground.

**The CHROME collision fired exactly where it was predicted to.**
`chrome/one-number-one-home` merged while this lane was open and
conflicted on the `bounds.rs` line the addendum named. Resolved to
this lane's side because it is the SUPERSET — `props::shown_text` IS
CHROME's `shown_in` plus the render plus the symbol, plus the refusal
— so CHROME's finding stays discharged and nothing was reverted.
CHROME had also appended evidence to this row file; both halves kept,
theirs before the `## Closed`.

**Stopped at the fence rather than crossing it.** The camera member's
other half — `Camera::new` admits every finite radius, so
`max_distance = radius * 100` arrives already `inf` above
`f64::MAX/100` — belongs at `Camera::new`, in a file another lane held.
Filed as `camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite`
(P1/E) rather than taken.

### Adjudication: one filed row deleted, and why

The lane filed `work/props/props-log-carries-a-committed-conflict-block-on-main`
after a tree-wide marker grep found a live conflict block in
`work/props/log.md` on `main`. The finding was **true when made** and
is now moot twice over: CHROME found it the same afternoon, filed
`work/props/committed-conflict-block-in-the-props-log`, and repaired
it at `97217090`; the markers are gone and CHROME's row is closed.

So the lane's row was an open duplicate, of a closed row, about a
repaired defect. **Deleted at merge** — one file, one item. And its
central argument was wrong in a way worth naming: it read the gate as
*filed for track J and never built*, therefore still owed. The gate
was **decided against** — `work/ciw/committed-conflict-markers-reach-main`
carries Ev's call of 2026-09-04 (*close it — the failure is rare and
not worth the special effort*) two lines below the orphaning sentence
the lane quoted. Reading `:30` and not `:32` is *check that Ev ever
agreed* run in the opposite direction, and it costs the same.

What survived is the datum, and it is recorded on the CIW row where
the ruling lives: **four instances in about three weeks**, the newest
costing two lanes a detour on one afternoon, both finding it by
accident while checking their own merges. `lint` does not read
`log.md`, so `main` was green over it. **The ruling stands and nothing
here reopens it** — a standing gate is Ev's to decide.

## Wave closed — 2026-09-21

Four units dispatched, one stood down and re-tasked as a reviewer,
four PRs merged (#3027, #3007, #3030, #3031) plus two orchestrator
state-syncs. **Twelve rows closed; the slate went 15 open / 10 closed
to 9 open / 22 closed, load 22.5 → 18 of 30.** Eleven rows filed
across six programs (VGEOM, VSEAM, VDOC, CHROME, WIRE, LINALG, and one
to `work/issues/`), every one a file rather than a sentence in a
merged PR body.

**The wave's own lesson is the correction near the top of this
entry**: it was cut over an open lane, because the board cannot show a
claim that has not merged. The remote check is one command and it is
now written down.

## 2026-09-22 — hand-over, and the whole rest of the slate dispatched

A new orchestrator took VGEOM at the resting state the 2026-09-21 wave
left: 9 open rows, load 18 of 30, nothing in flight. Ev's instruction
at the hand-over was to take **the whole remaining slate**, parallelised
as far as the machine's disk allows, and *"no AB protocol"* — which is
this program's standing posture (plan.md §Review posture) reaffirmed a
second time and not a new decision.

### The resting state audited, and one stale branch

`vgeom/pick-distance` is on the remote and is not an ancestor of main.
It is **not unmerged work**: its one commit sets `status: dispatched`
and `branch:` on the two `pickindex.rs` rows, and both rows are
`closed` on main — that lane was stood down and its work absorbed into
`vgeom/p0-fields` (#3007, `c316be7e`). The branch is superseded and
nothing is owed to it. Every other `vgeom/*` branch is an ancestor of
main; `work.py lint` is green; no open vgeom PR and no open `needs_ev`.

Recorded because a claim-only branch left behind by a stood-down lane
reads exactly like lost work to the next reader, and cost this sitting
a check. A lane that stands down should push its claim commit's
reversal or say in the log that the branch is dead.

### A ratification checked, and it was not there

`a-fields-text-commits-within-the-renders-own-tolerance`'s disposition
said its remaining half *"changes `readout`'s own **ratified** accuracy
rule, so it is a design question and not a lane's to answer."* Run
against the tree per CLAUDE.md's *check that Ev ever agreed*:
`REL_TOLERANCE` appears in `crates/viewer/src/readout.rs` and nowhere
else — not in `docs/DESIGN.md`, not in `crates/viewer/GUI-DESIGN.md`
(the ratified GUI page, whose only tolerance clause is the chordal one),
not in `crates/viewer/README.md`. It is the module's own doc argument,
landed inside an ordinary unit PR. **The row's block was weaker than
the row claimed**, and the word is corrected by the unit that takes it.

### Two questions put to Ev at the hand-over, both answered in chat

**The creation-form parser** (`creation-form-fields-lost-two-spellings-egui-accepted`)
— ruled option 1, leave it; the ruling and its reading are on the row,
which is **closed**. #3007's disclosed cost is now a decision rather
than an outstanding debt.

**The render grid** — Ev asked why the bound is `5e-4` rather than
something that scales with ε, recalling a rounding *"like eps/10 for
display"*. The recollection is exact and the rule is in the tree:
`crates/profile/src/path.rs`'s `num` reads
`(DEFAULT_EPS * 0.1).min(x.abs() * 1e-9)` — a cap one decade below ε
met with a relative arm, finer winning — behind 38 refusal-sentence
call sites, landed by FIX #2399. Its doc argues the point directly: ε
is a LENGTH, so a purely relative rule crosses it at one metre and is
coarser above, and the cap is what guarantees that a difference the
kernel can decide is a difference the sentence spells.

`readout`'s `5e-4` is derived from no such thing. Its own doc derives
it from a **field width** — `MAX_CHARS = 10`, ten characters buy four
significant figures, four figures' worst case is half a unit in the
fourth. ε is `1e-9` m = `1e-6` mm, so the viewer's grid is about 500×
coarser than the kernel's at millimetre scale, and `number(1000.001)`
returns `"1000"`. **Nobody chose that against the model; it fell out of
a box.** The unit takes `num`'s grid, and `readout`'s own sentence is
the warrant — *"a box narrower than this clips … that is the box's
number to meet, not this one's to lower"* — applied to the tolerance
rather than only to the width.

### The wave — four lanes, seven rows, dispatched together

Sized on the machine rather than on taste: a `viewer --features app`
target dir measures **3.6 G** and the session has ~28 G, so four
concurrent lanes with their own worktrees and their own
`CARGO_TARGET_DIR`s is the ceiling, not a preference.

| unit | rows | the one question |
|---|---|---|
| `vgeom/render-grid` | `a-fields-text-commits-within-the-renders-own-tolerance` (P0) | `readout.rs`: what accuracy a render owes, answered from ε rather than from a box |
| `vgeom/field-product` | `a-field-bound-to-a-written-value-shows-a-product-that-overflowed` (P1), `a-bare-field-still-commits-its-own-render` (P2), `a-typed-field-hands-its-text-over-on-two-frames` | `widgets.rs`, `props.rs`, `pane/properties.rs`: three residues of the `number_field` door |
| `vgeom/camera-band` | `camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite` (P1) | `camera.rs`: a door that guards its input and not the product it derives |
| `vgeom/seam-refusals` | `the-display-seams-refusal-is-drawn-and-never-said` (P2), `the-overlay-lanes-drop-the-leg-disposition-has-no-row` (P3) | `marks.rs`, `pane/viewport.rs`, `narrowing.rs`: the one f32-seam disposition asserted by nothing and said to nobody |

**The one seam inside the wave is declared rather than fenced.**
`render-grid` changes what `readout::number` spells and `field-product`
holds `widgets.rs`, whose `number_text` calls it. Both briefs carry it:
`readout.rs` is out of `field-product`'s fence, `widgets.rs` is out of
`render-grid`'s, `field-product` may assert no literal spelling that
the grid moves, and `render-grid` merges first.

**Why `field-product` carries three rows and not two.** The unbanded
`a-typed-field-hands-its-text-over-on-two-frames` is the same file and
the same door — its fix extends `number_field`'s parse closure, which
is where the echo guard already lives. Split off, it would be a second
lane editing one function. Banded P2/D by the lane.

### Parked, and why it is not a lane

`a-count-slots-cast-still-saturates-for-a-finite-value-too-large` is
**parked on `work/wire/need-count-spells-every-failure-as-a-pattern-count`**.
The 2026-09-21 re-derivation had already settled the fork's answer — a
door in `editor-core`, not a viewer-side refusal — and what this
sitting adds is that VGEOM may not write it and has no half that lands
alone: refusing instead of saturating needs a word, and there is none
the viewer may raise. Announced on WIRE's log rather than left to be
found. That is the one row of the nine this program cannot take, and
it is parked on a trigger that can fire rather than deferred.

**Slate after this sitting: 9 open → 7 dispatched, 1 closed, 1 parked.**

### The disk ceiling, measured wrong and corrected mid-wave

The wave was sized at four lanes on a measurement of **3.6 G** — a
plain `cargo build -p viewer --features app` into a fresh target dir.
That number is right and was the wrong number to size on. With test
artifacts and `debug/incremental`, the four target dirs reached
**0.8 / 4.7 / 5.6 / 6.0 G after cleanup and 27 G of 28 G before it**,
and the session hit 100% disk with 348 M free while all four lanes
were live.

**Roughly 11 G of the 27 G was `debug/incremental` alone.** Deleting
the four incremental directories recovered all of it and cost nothing
but a rebuild. `CARGO_INCREMENTAL=0` is now set in every lane for the
rest of the wave: incremental buys a tight edit-compile loop, and with
hosted CI as the verification of record and four lanes sharing one
disk, that is the wrong trade.

`memories/agent-lane-operations.md` says *"each lane grows a multi-GB
`target/`"* and prescribes the remedy for the aftermath — *"after a
disk-full crash, purge torn binaries (ELF-magic scan) and treat
pressure-window test results as suspect."* Both were followed. The
ELF-magic scan over all four target dirs found **zero** torn
binaries, so no lane's artifacts were corrupted; the lanes were told
to re-run anything that failed during the window rather than read an
ENOSPC as a finding about their code.

**The rule this wave learned, stated so the next sizing does not
repeat it**: size a parallel wave on a target dir that has RUN THE
TESTS, not one that has built the crate. The two differ by about 2.5×
here, and the difference is what decides how many lanes fit. Offered
to Ev as a refinement to `agent-lane-operations.md`'s disk paragraph,
which today says only *multi-GB*; it is not written there yet, because
`memories/` is Ev's call.

## The camera's zoom band has a top — 2026-09-22 (#3062)

`camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite`
closed. `Camera::new` asked `is_finite` of its `scene_radius` and
nothing of the `×100` band it derives, so every radius above
`f64::MAX / MAX_DISTANCE_FACTOR` gave `max_distance() == inf` and a
`distance` clamped into `..=inf`. The guard is at the door, beside the
`finite` call it already made, in `DisplayTolerance::new`'s shape.

**Three decisions worth keeping.**

**The arm is its own.** `CameraError::SceneRadiusOverflowsZoomBand`
rather than `NotFinite { what: "scene radius band" }`: the input IS
finite and strictly positive and every other guard in the constructor
takes it, so a message calling it a non-number would be false. That is
the distinction `SceneError` already draws between
`InvalidDisplayTolerance` and `DisplayToleranceOverflowsMillimetres`,
and the new arm follows its naming, its doc shape and its scientific
`Display` — a plain `{}` of a value three decades under `f64::MAX` is
three hundred digits.

**`MIN_DISTANCE_FACTOR` was asked and owes no guard.** It multiplies
DOWN, so it cannot overflow; the live question was whether it flushes
a small radius to zero and puts the band's floor on a non-distance. It
does not: `f64::MIN_POSITIVE * 0.05` is about `2.25e14` smallest
subnormals, and the factor would have to fall below about `2.22e-16`
to spend that. Because that is a property of two constants rather than
of any input, the answer is a measuring row rather than a sentence —
`the_bands_floor_is_a_length_at_the_smallest_radius_the_door_admits`
reds if either constant moves into the flushing range.

**The fix did not mint a fresh copy of what it closed.** The guard
would have been a fifth spelling of `radius * MAX_DISTANCE_FACTOR`, so
`band_floor`/`band_ceiling` took the band's two ends and
`min_distance`, `max_distance`, `clamp_distance` and `fitted` now read
them. `camera.rs` also gained its first `#[cfg(test)] mod tests`; the
suite rows stay in `tests/camera_ops.rs`, which is other programs'
ground.

**The sweep's own lesson.** The first pattern — a `is_finite()` guard
with a scaled derivation in the same function — is blind to exactly
the shape that produced this row, a product formed in a different
function from the guard. Three further passes shaped at that gap
(field × named constant; `<<`/`powi`/`powf` ladders; division by the
guarded input) found two more members, both filed: `datum_view`'s
infinite aspect and scale, and `BoundsProbe::new`'s `seed · 2^11`
ladder. What is still blind is a guard and its product in two
different crates, which no pass here crossed.

## `vgeom/field-product` — the numeric field door's three residues (2026-09-22, #3067)

Three residues of `crate::widgets::number_field`, closed together
because they are one door:
`a-field-bound-to-a-written-value-shows-a-product-that-overflowed`
(P1/D), `a-bare-field-still-commits-its-own-render` (P2/D) and
`a-typed-field-hands-its-text-over-on-two-frames`, which arrived
unbanded and is banded P2/D here.

**What a field SHOWS.** The refusal moved above the widget, because an
`egui::DragValue` cannot spell it: the formatter is handed the `f64`
the widget holds, with no unit in scope. `props::shown_value` is the
new conversion door — `shown_in`'s total twin, `Ok(number)` or
`Err(unit)` — and `widgets::named_field` and `widgets::value_field_ops`
now draw `props::no_reading` where the field would be rather than a
field reading `inf`. **The marker stands alone**: a disabled field
still has to be handed the `inf`, and a canonical number beside a
picker naming a different notation is a number to misattribute. No
third refusal vocabulary.

**A gate under the door.** `scripts/gates/viewer-numeric-field-door.sh`
refuses a bare `egui::DragValue` under `crates/viewer/src` outside
`widgets.rs`, which holds both the constructor and the rows that prove
what it adds. The bare-field row's census — *every numeric field in
the crate goes through the door* — was a measurement taken once and is
a standing fact now.

**One keyboard edit, one operation.** `value_field_ops` remembers,
under the widget's own id, the text it last turned into an operation,
and clears it on `gained_focus`. The id comes off the `Response`
rather than being re-derived, which is what the row's argument against
`had_focus_last_frame` asked for. `DocSession::writes_nothing` was not
touched: the two rules are two, and
`a_re_typed_text_after_focusing_again_is_a_second_act` is the case
where the document's rule is the only answer there is.

**The sweep and its blind spot.** Thirteen `number_field(` call sites,
eleven in production, two of them conversions — the row's count, held
at this merge base. The rule reads the CALL SITE, so it is blind to a
product formed one frame up, and the second pass found one:
`pane::properties`' free-move probe converts to millimetres before
handing the triple to `vec3_row_ops`. Not fixed, and the reason is
written at the site: the translation there is never a document value,
only one the probe itself authored.

**Filed**:
`field-texts-literal-arm-is-unreachable-from-both-call-sites` —
`props::field_text`'s literal arm is reachable from neither caller,
and its prose is the only statement in the crate of what a literal
field shows.

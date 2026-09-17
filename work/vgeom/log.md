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

**Dispatched as a VIEW lane, landing on VGEOM's slate.** The cut
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

Signed (VIEW implementer lane `view/shader-mark-strength`).

## For Ev

**1. Recommendation now: the other reply's mechanism, one place projection beside `told_reading`, with two refinements (likely). Mine moved.**

The two replies differ on how the read reaches the reader and on whether the #990 ruling has to change. A *place* is a number the code chose to evaluate at, held so that nothing can compare, store or print it, or turn it back into a `T`.

Two things moved me:
- **The provenance.** The #990 passage was restated in an `[ev]` commit on 11-19 as "the one projection is `Real::told_reading`". That letter forbids a second projection wherever it lives. My round-2 reading, that #990 is only about refusal payloads, does not survive the word "one". Either design amends #990.
- **The tree.** All three chains into the side reader already hold `AtRestPolicy` (`missed_side` and `at_infinity_side` in `solid_contain`; `shell_role`, above `shell_role_reading`, in `props`; check 7's door in `validate`). So my route through `AtRestPolicy` threads nothing. But for the same reason it limits nothing either. Any code bounded `AtRestPolicy`, which is most of `topo`, could call it. It is the same projection as the other reply's, homed in `topo` instead of `geom-core`, and the bracket read belongs with the scalar. `geom-core` is where DL5 and `told_reading` live.

The two refinements:
- **One reader, at a dual too.** I withdraw "the ray keeps riding at a dual". A dual's place is its base value's (DL5.1 already says a bracket read at a dual returns the base bracket), and a place has no tangent. The reader returns a selection and nothing else, so no zero-tangent `T` escapes, which is what DL5.3 asks. There is then no second reader.
- **The admission test for a call site is place-invariance**, not "the thing placed is a choice". A call site is admitted where every decision taken at the place gives the answer every other place would give. For the side reader that is its own theorem: any ray from outside a region reads the same side. Where a selection bounds no region, the existing first-ray rule applies (`[ev]` PR 6592).

**2. The other mechanism's strongest point, and my answer.**

Its point: a right passed as a value leaves two readers, because a caller that cannot mint the right keeps the riding ray. Answered by the tree: no caller lacks the right. But the conclusion stands, for my own reason above: an `AtRestPolicy` door is no narrower than the projection, so there is nothing to gain by keeping it apart.

The question put to that design was what stops `decide(evaluate_at(place(x)))` from deciding on a midpoint. Nothing in the type does, in either design. A place-taking evaluator's result is a `T` and is decided like any `T`. The guard is the call-site allowlist, and each entry has to state the place-invariance argument, so a reviewer can check it against the decision the entry makes. A midpoint decision is visible there because the argument fails: another place would decide differently.

Why this is not the bracket reader the scope rule keeps out:
- It yields no `lo`/`hi` and no `f64`. The only thing that accepts a place is an evaluator.
- Its name is gated. The #701 evasion was a bracket reader under a name the gate does not read.

**3. Remaining difference:** none of substance. If the orchestrator still sees one, it is whether the allowlist's stated reason is "a choice under `[scalars]`" or "place-invariant". I hold that place-invariance is the stronger test and covers the first.

**4. The sentences to put to you.**
- **`real.rs`, #990.** "the one projection is `Real::told_reading`, into a told quantity" becomes: "there are two projections, each into a type that offers nothing a comparison could read: `Real::told_reading`, into a told quantity, and `Decide::place`, into a place only an evaluator takes (the crate README, DL5.2)."
- **`real.rs`, the #701 sentence**, appended: "`Decide::place` reads a bracket under its own name; its call sites are a gated allowlist, each stating why every decision taken at the place is the one any other place would give."
- **README DL5.2, the place row**, appended: "A place may be drawn from a value held at `T` by `Decide::place`: one number of its bracket (the value at a point scalar, the midpoint of an enclosure, the base value's at a dual). It offers nothing but the evaluators that take a place, and is admitted at a call site where what is decided there is the same at every place it could have been."

**Confidence.**
- #990's letter forbids a second projection, so it must be amended: sure.
- The `AtRestPolicy` route is no narrower than the projection: sure, from the bounds in the tree.
- The dual sharing the place, with only a selection leaving: likely.
- Place-invariance as the admission test: likely.

## For the orchestrator

- **Your round-3 provenance answers my round-2 open item.** I could not find #990's 11-19 restatement in the shallow checkout; you supply it as `a8000ffba37`. Row 292's "no obvious way to convert to f64 and back in a way that's valid" was said of what was held back. The place evaluators' private step (`at_place`, which goes through `from_computed`) is the f64-to-`T` direction Ev did accept. A place drawn from a bracket adds the `T`-to-f64 direction, so cite row 292 honestly in the PR: this is the half Ev held back, now asked for with an opaque carrier.
- **The pin `the_step_to_a_place_is_this_files_own`** (`geom/src/places.rs`) requires `at_place` to be declared once in that file. So the fixed ray's constructor from a place lives in `geom::places` beside the other evaluators, or the pin is restated. Prefer living there.
- **At a dual:** the ray's private step would produce a zero-tangent `Dual`. Both `at_place`'s bound (`Value = T`) and the place-row clause "a place is never a `T`" refuse a step that hands a `Dual` back. Either the ray at a dual reads the body's values through `without_tangent` and evaluates at `T::Value`, which matches DL5.2's "result comes back on the value channel", or that clause needs a sentence. I lean to the first. Not checked how large it is.
- **Direction places:** midpoints of a unit vector's entries are not unit. The ray squares them at `T` as `aimed_ray` already does (`UnitVec3::squared`).
- **Unchanged from round 2:** the tilted brick's blocker is tier 3′'s census, not check 7 (M14 table A); the turn read along the ray ships only once the lined brick's bottom cap regression is explained.

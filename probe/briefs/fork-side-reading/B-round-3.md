## For Ev

**1. Recommendation now (likely): I withdraw the scalar projection and take the other reply's mechanism.** The place has one constructor from values held at `T`, in `geom`'s existing place module, bounded on the bracket read alone; the side reader reaches it as a door its scalar supplies through the at-rest policy. #990 is not amended. One point of difference is left, on what happens at a dual (item 3).

Three things moved me, all checked in the tree.
- *The machinery already exists and I had not read it.* `geom::places` holds the place evaluators, with the one step from a number to a scalar private to the file and pinned (`the_step_to_a_place_is_this_files_own`), and one evaluator there is already bounded on the bracket read alone. The at-rest policy already carries nine injected doors (the shell door, the quadrature lane and others), each a type with no values at a dual. A tenth door is the established shape; a new projection on every scalar is a second shape for the same thing.
- *My "more plumbing, two readers" objection was mostly wrong.* The reader's chains are already bounded on the at-rest policy at their heads (`shell_role`, `point_in_solid`, the missed-ray side); only the reader's own few private functions are bare `Decide`. Nothing is threaded.
- *Your recorded words on the DL5 unit*: "only the bounds reading needs the extra check". The other mechanism puts the read where the gate and a reviewer see `Bounds` spelled, at one constructor. Mine hid it behind a new name on every scalar.

**2. The other mechanism's strongest point, and the questions put to me.**
- *Why is a projection on every deciding scalar not the bracket reader the scope rule keeps out?* By reach, it is. Any generic body could have called it. I offered an allowlist of call sites, which is review, not type.
- *What stops `decide(evaluate_at(place(x)))` from deciding on a midpoint?* Under my proposal, nothing but that allowlist. Under the other, the bound: code that does not hold the door cannot make a place. I concede the point.
- *#990.* On rereading with the provenance: the passage opens "A door outside the seams has ... no `f64` out of a `T: Decide` for a refusal payload", and "No projection to a number exists" sits in its bullet on payloads. A place made inside a ledgered seam, that no caller holds as a number, is neither. I asked for an amendment the mechanism I now recommend does not need.

**3. The one remaining difference: a dual.** Two final states, each one sentence.
- *(a) The other reply:* at a dual the door has no values and the ray keeps riding the body, so the reader builds its ray two ways. Text changed: DL5.2's place row and one ledger entry.
- *(b) Mine:* the ray is built one way only, from a place; a body that carries tangents is read on its values, as `geom::places` already says of any geometry that carries one ("it is first read without its tangent, through its `value_view()`, and evaluated there"). Text changed: the same two, nothing more.

Why I lean (b): everything the side reader returns is a side or refusal text, nothing with a tangent, and a dual decides on its value alone, so reading the values loses nothing; and the riding construction, which both of us named as the defect, then exists nowhere. What (b) needs that I could not confirm exists: a value view of a whole body (surfaces and curves have one; topology holds no scalar). If that is a large thing to build, (a) is acceptable: at `Dual<f64>` a riding ray is already one line, and only a dual over an enclosure keeps today's widths. Confidence in (b) over (a): unsure; it is the smaller question.

**4. The sentences.**
- *`geom-core` README, DL5.2, the place row, one sentence added* (the other reply's, tightened): "A place may also be read off a value held at `T`: by the place's own constructor, bounded on the bracket read alone, and only where what is decided at the place is the same at every place it could have been."
- *`real.rs`, the `Bounds` ledger, one entry*: "The side reader's aim. `geom`'s constructor of a line's place reads the middle of an aim's and a direction's enclosures. Necessary: the ray must be one line for every member of a box, and only values held at `T` say where the face is. It decides nothing: a place has no accessor, every branch is a named predicate on what is evaluated there, and any line from outside a region reads the same side."
- *#990: no change.* If you read "No projection to a number exists" as universal, the other reply's scoping ("... of a derived quantity to a number exists for a payload") says what the bullet already means.
- Under (b), no further sentence: the place module's own text covers a dual.

**Confidence.** The door-through-the-policy mechanism over a scalar projection: sure, now. #990 does not reach a place made inside a seam: likely (your reading to confirm). No ratified text beyond the two items above: likely.

## For the orchestrator

- The at-rest policy's own doc says `Bounds` "deliberately does not ride along: ... every door that reads a bracket spells `Bounds` where the reader can see it". The other mechanism satisfies that only if the bracket bound is spelled on the door type's constructor (as `ShellDoor::certified` spells its right), not on a policy method. Worth putting in the unit's brief.
- The policy's doc counts "nine injected doors"; a tenth changes that count. It is a descriptive re-wording that lands with the code, not a second decision.
- `side_at_infinity` and `shell_role_reading` are bare `T: Decide` today; they become bounded on the policy. I checked the three heads named above, not the census, shell and stands callers.
- A ray constructor from a place must live in `geom/src/places.rs` or the pin on the private step moves (the other reply says the same).
- At `Sym` the private step makes the ray's numbers the tier's own unknowns, so no identity is proved through the ray. That is right for a choice, and it means a symbolic zero can no longer rescue a ray reading; M14's hand switch used "point values at the lane scalar", which may or may not have been the same thing. If its reach numbers were taken with literal constants, re-take one row with the real step before pinning widths.
- Not checked: whether `Body` has or can cheaply have a value view (decides (a) against (b)); whether any path runs the side reader at a dual over an enclosure; the text of PR 7155 beyond what round 3 quotes.
- Unchanged from round 2: the turn read along the ray stays out of this unit; the tilted brick is blocked by the embedding census, which wants its own row on the deletion.

## For Ev

**Recommendation (likely).** The fixed ray reads one number per coordinate off the aim's enclosure, through one door. What that door returns is a **place**: a type that offers no number to read, compare or store. It goes only to evaluators that take a place, and here the one evaluator is the ray itself. Every branch is still a decision on `T` values evaluated against that line, so no bracket decides anything.

The read is allowed under the `Bounds` scope rule. A bracket may "prune a candidate set, drive a subdivision", and choosing which line to cast is that kind of steering, so it owes the rule's ledger entry. The #990 ruling does not speak to it, because #990 is about refusal payloads, and a place leaves no door as a number. What both earlier reports got wrong is "nothing needs ratifying". The read needs an entry, and DL5.2 needs one sentence.

Terms.
- *Place*: an `f64` the code chose to evaluate at (DL5.2's row), held so that no caller can see it as a number.
- *Bracket read*: `Bounds::lo`/`hi`, turning a `T` into an `f64`.
- *Riding ray*: today's ray, built from the body's own moving values.
- *Fixed ray*: one line, the same for every member of the box.

**The final state.**
1. `geom::places` gains a 3-D place, a point and a direction. Its one constructor from values held at `T` reads the middle of their enclosures. It is bounded `T: CertifiedBounds` alone, the sole-bound form the scope rule allows for code that only reads brackets. It returns nothing where the value is poison, and then the face offers no aim. What the place means at each scalar:
   - `f64` and `Probe`: the value itself, bit for bit. Today's rays are unchanged there.
   - `Interval`: the enclosure's midpoint.
   - `Sym<Interval>`: its value channel's midpoint.
   - A dual: not admitted by the bound.
2. `geom::Ray` gains a constructor from a place. Its step into `T` is private, like the existing place evaluators' step (`from_computed`):
   - at `Interval` the point `[x, x]`;
   - at `Sym`, an opaque unknown, so no identity is ever proved through the ray, and none should be;
   - at `f64`, the number.

   The ray stays scratch: nothing stores it.
3. The side reader reaches the constructor as a value its scalar supplies through `AtRestPolicy`, the way the shell door is supplied today: a door at the certifying scalars, and a type with no values at a dual. So no `Decide + Bounds` bound spreads up the side reader's three call chains, and duals keep their shell roles inside Booleans. At a dual the ray still rides the body. A dual decides on its value channel alone, and at `Dual<f64>` that channel is a point, so nothing is lost there.
4. What a reader of a place can do: evaluate at it, nothing else. What a reader cannot do: compare it, store it in a body, or put it in a payload. One question is not settled by the type and is the entry's argument: whether **the answer read at a place is the same at every place**. For the side reader it is: any line from outside a region reads the same side. Where a selection bounds no region, the first ray's side is the one read; that is the existing rule from `[ev]` PR 6592, and the riding ray already makes the answer depend on the aim, at `f64` too.
5. The gates:
   - `bounds-allowlist.sh` sees no compound bound, because the read is sole-bound.
   - The ledger still gets an entry for the side reader's use. Necessity: the ray must be one line for every member, and only values held at `T` say where the face is. Decides nothing: the place has no accessor, and every branch is a `Decide`.
   - `from-computed-sites.sh` sees one more private step of a place-taking evaluator, the kind DL5.2 already lists.

**Ratified text.**
- **DL5.2's place row** (`crates/geom-core/README.md`) gains one sentence: *"A place chosen from a value held at `T` is read off its enclosure by the place's own constructor, and offers nothing but the evaluators that take it; a use is admitted where what is decided there is the same at every place it could have been."*
- **#990 stays as written.** "No projection to a number exists" sits inside its second bullet, on refusal payloads ("a `fn margin_of<T: Bounds>(…) -> f64` for everyone's payloads"). A place is no payload and no `f64` a caller holds. If you read that sentence as universal, it should be scoped: *"No projection of a derived quantity to a number exists for a payload"*. I could not find its 11-19 `[ev]` restatement in this checkout, so I cannot say whose words it is.

**Premise check.** The brief's facts check out against the tree, with two corrections:
- Tier 3's public doors already bound `CertifiedPolicy + CertifiedBounds` (`validate_geometric`). Only the reader and its other two chains are bare `Decide` or `AtRestPolicy`.
- DL5.2 licenses handing a place to an evaluator, but it never said how a place is read off a `T`. That gap is real, so citing DL5.2 alone was an overstatement (sure).

**Alternatives, and why each loses.**
- *Keep the riding ray and repair readings one at a time.* Measured: the walls stay at ±0.05.
- *Read the side at one member and carry it by the embedding certificate* (my first answer). The gate holds no member, and plain `Interval` builds none of the measured bodies. The new measurement also shows tier 3′'s embedding census undecided on the tilted brick at every width, so there is no body it would serve.
- *A place method on every scalar* (`Real::place()`). It hands every generic body a decision on a midpoint (`decide(at_place(mid(x)))`), which is the #701 evasion with better manners.
- *The nominal `f64` build passed down from the driver.* It is no member of the leaf, and it changes every door's signature.
- *A compound bound on the side reader with an entry.* It forces `CertifiedBounds` up the shell-role chain and takes duals out of Booleans' shell roles.

**Confidence.**
- The facts as checked: sure.
- #990 does not reach a place: likely, since it rests on reading the sentence's own bullet.
- The design is sound: likely; its theorem is the reader's existing one.
- A dual loses nothing with a riding ray at `Dual<f64>`: likely.
- Dual-over-box behaviour is not measured.

## For the orchestrator

- **M14 confirms the fixed ray's soundness** (no `Side(In)` anywhere) but shows modest reach. It moves the tilted brick's reader from 0.05 to 0.1, and to 0.25 on the gentler tilt only with the turn taken along the ray. The slab and the lined brick already read at all four widths.
- **The decisive finding is A.** Tier 3′'s census is undecided on the tilted brick from ±0.05 (`pm_census_ef_residual`, `bool_contact_vertex`). So the tilted item cannot be what blocks the deletion: the census is, and nothing finishes there whatever check 7 reads. I would file that as a blocker row on the deletion, beside or replacing the tilted item.
- **On my first-round answer**, M14 removes its last use: where embedding passes, the fixed ray already reads, and where it fails, nothing finishes. Not reopened.
- **Turn taken along the ray (B2): keep it out of this unit.** It drops the lined brick's bottom cap from 0.45 to 0.25. Take it separately, with its own rows.
- **`brick_at`'s refusals are a separate row** (edges with no stated lines: `sai_advance`, Graze). They are not about the place.
- **Not checked:**
  - Whether a dual ever runs the side reader over a box in a path that matters.
  - Whether `Body` has a value view. I did not need one.
  - Whether the gate scripts treat a sole-bound door called from a concrete `AtRestPolicy` impl as needing an entry. I recommend writing the entry regardless.
  - The 11-19 `[ev]` PR's text.
  - `geom::places`' pin `the_step_to_a_place_is_this_files_own`: a ray constructor would have to live in that file, or the pin moves.

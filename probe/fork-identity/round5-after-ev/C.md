## For Ev

**Recommendation now (likely): option (b).** The structural question ("is this value zero for every value of the parameters?") has an answer only at the symbolic tier. At `f64`, `Interval` and `Dual` it always answers "no", so those lanes always take the general spelling, which PR 6990 already does. Where a number is exactly zero because the kernel wrote it that way, that fact is carried by a type or a variant, never read back off the number.

Terms:
- **Numeric scalars:** `f64`, `Interval`, and `Dual` (a value plus its derivatives).
- **Symbolic tier:** E12's test that an expression is the zero polynomial. It runs only at `Sym`, the scalar that carries a value alongside its expression.
- **Band:** the ε allowance within which a numeric margin reads `Zero` (`[verdicts]`).

**Your discomfort is right (sure).** At the numeric scalars the design's only notion of zero is the band, and `[verdicts]` says bit-equality is never the test. The recommendation I signed would have ratified "`f64` answers yes at exactly 0" as a kernel meaning. That gives the numeric scalars an exactness the rest of the design denies them. It is also the fork's own defect one scalar down: `(x + w) − w − x` reads `0.0` at one valuation and `2.8e-17` at another. For the mint this costs nothing, because both arms are the same bits at `f64`. But the definition was general, and the audit below shows what it would ratify for everyone else.

**What `is_exact_zero`'s callers read today** (all checked):
1. **A number the kernel wrote to encode structure.**
   - an iso line's held axis (`row_of`, `iso_image_row`, `PatchClass` on the slope `pl`);
   - a ray's start being 0 (`curves.rs`).

   These are a variant hiding in a number. The held axis should be the pcurve's variant (the held-line representation from fork 271), and the ray's "starts at origin" should be part of its form.
2. **A choice of spelling read off a computed value.**
   - `arc_rim_normal_weight` (`w0 − w2`) and `one_value` (held weights equal);
   - the torus hub (`major`);
   - `PatchClass` on a net whose control xs are equal;
   - the `Interval` shortcut in `cylinder_rim_axial`.

   Each one's general spelling is sound. At `Sym` the structural question keeps the closed spelling; at the numeric scalars, the general spelling.
3. **A decision about what gets built, taken by bit-equality.**
   - the chart corner or side a value sits exactly at (`spline_known`, `description::chart_side`);
   - whether row ends equal stated ends, which refuses (`pcurve_cache` ~9184);
   - the bulge's line-or-arc choice (`profile::bulge_leg`);
   - the zero-chord `+x` stand-in (`path::chord_director`).

   These are `[verdicts]` violations today. Each becomes a margined decision, or reads the structure in a type.

**The three final states.**
- **(a) As in the PR.** Every lane gets one definition. But it ratifies bit-equality as a meaning at the numeric scalars, and leaves group 3 able to decide what is built from a coincidence at `f64`. Rejected.
- **(b) Recommended.** The question means something only where structure exists. At the numeric scalars "zero" means only the band's margined `Zero`. In every lane:
  - no function reads exactness off a number;
  - group 1 becomes types;
  - group 2 takes the general spelling at numeric scalars and the closed one at `Sym` when the tier proves it;
  - group 3 becomes decisions.

  The mint, the `Dual` sensitivity and the corpus rows come out as in the PR, with no derivative clause needed. Cost: an `f64` iso line found by a value is no longer an iso line. That is correct: only a type can say "this is held". Reversible by narrowing.
- **(c) The band's `Zero` at the numeric scalars.** This is a decision, so the question stops being "not a decision":
  - it adds a log row the symbolic lane does not have;
  - it escalates inside (ε, Kε), where the general spelling would have been sound;
  - at `Dual` a band-Zero on `W₁ − W₂` stores a constant axis, losing a derivative (the coincidence problem again).

  It belongs where the band already belongs: in group 3's decisions, not in the structural question. Rejected for the question.

**Replacement `[scalars]` bullet:**
> - **The structural question**: `identically_zero` asks whether a value is zero as a function of the document's variables. Only `Sym` can answer: yes where the symbolic tier proves the expression the zero function, by rules that read no value and a budget no session sets (ERROR-DESIGN E12). `f64`, `Interval` and `Dual` hold no expression and always answer no, so a numeric lane takes the general spelling; at a numeric scalar "zero" means only the band's `Zero` ([verdicts]). The question is not a decision: it reads no sign and no band, counts and records nothing, and defers nothing. Its answer may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]). A branch that changes what is built is a decision. A value the kernel writes as zero to encode structure is carried by its type, never read back off the number.

**E12 mechanism bullet:** unchanged except one sentence. Replace its pointer "(`is_exact_zero`, DESIGN [scalars])" with "(`identically_zero`, DESIGN [scalars]; the numeric scalars answer no)".

## For the orchestrator

- **Moved, from (a) to (b):** Ev's comment plus the per-caller audit (21 call sites, in three groups). My round-3 move to one door rested on "protection by definition". Under (b) that protection is stronger: at a numeric scalar no exact-zero read exists at all.
- **The `Dual` derivative clause** becomes unnecessary under (b) and should go.
- **Renaming to `identically_zero` and retiring `is_exact_zero`** follows: groups 1 and 3 stop calling it. The `pcurve_cache` doc on `one_value` says "read as structure … on the value channel at Sym", which is wrong under (b) and needs rewriting.
- **Group 3 as off-question rows:** `spline_known::corners`, `description::chart_side`, the `pcurve_cache` ~9184 ends check and `bulge_leg` decide what gets built by bit-equality. That is a `[verdicts]` violation today, independent of this fork. File it under DECIDE, and the held-axis variant under fork 271's row.
- **Not checked:** whether `cylinder_rim_axial`'s `[0, 0]` shortcut is ever hit in production. Under (b) it falls through to its own decision, which should be equivalent.
- **No-op:** I checked out `origin/boxfinish/ev-identity-question` read-only, then returned to the original detached HEAD. Nothing was changed.

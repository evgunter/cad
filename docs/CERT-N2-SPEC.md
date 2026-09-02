# CERT-N2 — H2: the merge's residue, one lane (S99, S101, S102, S103, S116(b))

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md` §CERT-N;
difficulty logged at spec: **M**; **ADV** on S99). Read
`docs/prompts/implementer-discipline.md` in full before starting. The
Track N table's `H2` row and findings `S99`, `S101`, `S102`, `S103`
and `S116(b)` in `docs/SMELL-SCAN-2026-08.md` are the primary
specification (`S100` was closed by CERT-N1 — read `geom/src/
scalar_lift.rs`'s new module docs first, and `geom/src/net.rs`'s
channel contract, `ControlPoint::channels`, which CERT-N1 minted).
Branch `cert/n2-merge-residue`.

## The rows, in order

1. **S99 — `is_placeholder` reads one channel while the crate doc
   promises all of them (ADV).** `geom/src/net.rs`'s predicate is
   `control.iter().all(|p| <first channel>.is_poison())`; `lib.rs`'s
   discriminator says a placeholder's every control point is
   ALL-poison and a described net carrying poison must fail loudly,
   never masquerade. A described net whose every point has a poisoned
   `x` and finite `y`/`z` reads as the benign placeholder at ~25
   consumer sites (`step-export/src/writer.rs:44`, `topo/src/props.rs:660`,
   `mesh/src/trimmed.rs:186`, `geom-brep/src/certify.rs:999`, …). Widen
   the predicate to every channel (`channels()` makes this one
   expression), RED-FIRST on exactly that net in both lanes. Then the
   blast radius, which is why this is ADV: enumerate every consumer
   (re-derive the ~25; the count is the body's claim), and for each say
   what changes — a net that read as placeholder and now reads as
   described enters the consumer's DESCRIBED arm; show that arm poisons
   loudly (or refuses) rather than returning a finite answer, with a
   row per consumer class. The described-net-with-poisoned-`x` case is
   by construction one nothing currently constructs — construct it at
   the door that can (a `from_f64` lift of a net with one NaN
   coordinate; CERT-N1's poison row is the seed) and drive it through
   the top of each consumer class. Say what a placeholder with one
   FINITE channel (the mirror) now answers and whether that is right.
2. **S101** — the merge's prose sweep deleted the cross-reference at
   `geom/src/curves/nurbs.rs:684-687` (the `/ w_min` upper bound's
   deliberate opposite lives at `step-import/src/recognize.rs:422`)
   instead of re-aiming it; restore the fact with the right pointer.
   Then the class: every site that sweep touched where the identifier
   was what the sentence hung on — re-derive from the merge commit's
   diff, hit list with dispositions; `curves/boxes.rs:8` and
   `surfaces/boxes.rs:4`'s "the geometry crates" plural go with it.
3. **S102** — two copy-sites the merge's justification was about:
   `surfaces/nurbs.rs:4-11` declares the payload-level conventions
   "stated once there and once here" — hoist them to one home (the
   crate doc or the curve module) and point; `surfaces.rs:26-30`'s
   "shared helper" bullet spells the `radial`/`tangential` formula
   without naming `crate::azimuth` — one home in `azimuth.rs`, and the
   two halves' headers agree who documents the frame.
4. **S103** — the iso-curve placement rule ("extraction belongs to the
   EdgeGeometry layer") lives only in `geom-brep/src/nurbs_iso.rs:19-41`,
   in the crate that obeys it; the crate that could violate it (`geom`)
   says nothing. Put the rule (or its pointer) where the next extractor
   would be written — `geom/src/surfaces/nurbs.rs` and the crate doc —
   in a form that does not become a second spelling (a pointer at the
   payload, the rule at one home). `nurbs_iso.rs` is Track Q's: a
   pointer-only edit there is filed, not made, unless the one home must
   move.
5. **S116(b)** — three modules named `projection`, two `boxes`, two
   `nurbs` in `geom/src/`, with `use crate::projection::{…}` inside
   `surfaces/projection.rs` naming a different thing; and
   `azimuth::frame` returning `(radial, tangential)` — two `Vec3<T>`
   whose transposed destructure compiles silently, which
   `azimuth.rs:64-80` concedes and covers indirectly. Decide the module
   naming with a measurement (how many `use` lines disambiguate by
   path today) and rename or argue; give `frame`'s result a shape a
   transposition cannot compile (a named struct with two fields, or
   two doors — the header says two doors, the call sites take three
   shapes: say which and why).

## Fence and posture

- **Fence (rule 1):** `crates/geom/src/` and its tests, plus
  `crates/geom-core/src/spline/` if S99's widening reaches `net.rs`'s
  substrate. S99's ~25 consumer sites are OUT of fence
  (`step-export`, `topo`, `mesh`, `geom-brep`): you do not edit them;
  you construct the case and DRIVE it through their public doors from
  tests in your fence where reachable, and file a row per owning track
  for any consumer whose described arm answers finitely on the newly
  described net (that is a wrong answer, and the filing must say so
  loudly — a soundness defect filed is not the same as a style row).
- ε: the predicate is a poison test with no tolerance; `CI-Config:
  lane=both eps=1e-12` with the argument stated (both lanes carry
  consumers; the interval lane's poison is `[NaN, NaN]` and the
  widening must see it); three-ε local sweep.
- Review: standard v6 dual, ADVERSARIAL on S99 — the reviewers execute
  the blast radius: construct nets the lane did not and ask what each
  consumer class answers.
- Landing (rule 3): delete `H2`'s closed members and findings `S99`,
  `S101`, `S102`, `S103` and the (b) bullet of `S116` member by member;
  `H2` is deleted when all five close; relocate standing rules (S103's
  placement rule to its one home in code) before deleting text.
- No `Co-Authored-By`; rows spelled out; push early to
  `cert/n2-merge-residue`; the lane rules in full.

## Acceptance

- `is_placeholder` all-channel, red-first, consumer blast radius
  enumerated with a row per class and filings for any finite answer;
  S101's pointer restored and the sweep class swept; S102's two homes
  one; S103's rule reachable from `geom`; S116(b) decided by
  measurement with `frame`'s shape transposition-proof.
- Sweep obligation: other single-channel reads standing in for
  all-channel claims in the fence (a `.x`/first-channel test beside a
  doc that says "every coordinate"); hit list, blind spots stated.
- Deviations stated; D2-addendum classification for the widened
  predicate (row 3 poison posture unchanged in kind, its DOMAIN
  widened — say so) and for `frame`'s new shape (nothing minted in the
  state taxonomy).

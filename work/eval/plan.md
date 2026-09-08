# EVAL — the evaluation seat (plan)

**STATUS: OPEN (2026-09-06).** Opened in the tracker-wide cut of
2026-09-06 (`docs/WORK-TRACKS-2026-09.md`, addendum 2). Live state is
`work/eval/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`eval/`** — unit branches
`eval/<unit>-<slug>`, orchestrator branch `eval/orchestrator`.
Away-channel tag `(EVAL orchestrator)`. A/B ordinal band
**EVAL = 3000–3099**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

## Why this is not DOCM

DOCM's charter is the document model: the persisted recipe vocabulary,
the `DocEdit` set, document identity, the frames and selectors the
viewer consumes. Its `paths` name those files and stop there. What
evaluates a document — the wiring from `Node` to kernel verb, the
verdict bracket, the content key, the birth-record emitters that turn
kernel naming records into `StableName`s, and the verb seat the
kernel exposes — was SEAT's while SEAT ran and Track V's before that,
and since 2026-09-06 has been nobody's. This program is that owner.
Where a question is about what a node MEANS (may `Transform` take
`Instances`; what a node's log is), the decision is shared with DOCM
on an `[ev]` PR; where it is about how the seat is wired, it is this
program's.

## Review posture

The CIW/CHROME posture: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, plus a correctness arm where a
unit moves what a document evaluates to (units 3 and 6 below). No A/B
rows; the band is claimed for bookkeeping.

## Unit order

E, first:

1. `affine-lift-has-a-second-home-in-anchor-embed-affine` + `D368` —
   one lane in `eval/anchor.rs`: `embed_affine` and its two callers
   retire into `SketchPlane::map`; the hand-lifted `Vec3` the row cites
   is the walk inside `map_affine`, whose `from_f64` instance WAS
   `embed_affine`, so `D368` closes by construction; whether the
   fallible direction wants a kernel `try_map` is put to PROPS by note,
   and `map_affine` stays local, parked on that answer.
2. `node-tag-space-census-blind-to-tags-outside-sentinels` — the tag
   space declared once as a closed enum with `ALL`, the sites reading
   from it, the sentinels retired.
3. `emit-blend-restates-the-kernels-own-arguments` — the two
   re-derivations in `emit_blend.rs` replaced by citations of the
   kernel type that owns each; the coverage sentence made to agree
   with the kernel side.
4. `D367` — `declare_all` and `rem_apply` through one accept funnel so
   `applied.maintenance` is returned rather than dropped (`refactor.rs`
   is FIX's, by announced seam).
5. `two-public-verb-types-verbs-and-profile` — a stated convention in
   both crates' module docs, or the rename on the profile side
   (S-BOOL's glob, announced); rides whichever unit next opens either
   surface, and this is the first.

D, each an `[ev]` PR, announced on DOCM's board:

6. `transform-refuses-a-patterns-instances-value` — does `Transform`
   (and the other single-body placers) accept `Instances`, or is the
   fence intended and the mate walk refuses the shape in its own
   voice. A small PR either way once ruled.
7. `bracket-scope-is-run-op-not-the-node` — what a node's log means:
   the profile pre-pass and the mate solve decide before any bracket
   opens. A design choice made with the pre-pass's owner; the build
   moves every Profile node's log and the verdict-log goldens.

Standing, not units: `D360` (sweep topo refusal enums by variant name;
a rule this program's lanes read first). Deferred with its
ratification cited: `two-verb-seats-do-not-compose` (#1345 items
(2)/(3), `crates/verbs/README.md` §5), which reopens when a replay
consumer arrives.

## Exit shape

The five E units land, the two rulings are answered and their builds
land, the deferred row is either reopened by a consumer or still
deferred; the walk convention applies.

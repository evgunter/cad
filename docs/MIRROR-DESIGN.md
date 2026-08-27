# Patterns & mirror — the reflection-instancing design conversation

**Status: RATIFIED (Evan's sign-off on PR #909, with a P1 conversation round — u ↦ −u per Evan's agreement). The status line lagged the ratification until 2026-08-27 — the sign-off itself is the PR record.**
(VERBS program). Proposals P1–P6. Substrate anchors verified on main
2026-08-21. The headline finding reframes the register's row: **the
patterns half is largely SHIPPED** — `Node::Pattern` (linear/
circular/explicit, structural count, `Instance(i)` naming per the
naming-doc obligation) and the ratified-and-shipped `PlacedUnion`
group boolean already exist, and they are part-level (the node takes
any body-denoting input; there is no part/assembly fork). What is
genuinely open is **mirror** — reflection instancing — plus two
rulings the shipped half exposed. ASSEMBLY-DESIGN A6 already
ratifies the frame ("mirror is a pattern whose frame is improper";
the D9 conv. 4 equivariance premise becomes a **named prerequisite
of the mirror implementation unit — audited then, not assumed**);
this conversation is that audit's design half.

## P1 — The chart-handedness convention (the ruling that most needs Evan)

Every analytic surface computes `v_ref = axis × u_ref` — right-
handed by construction, **computed, never stored** — so a reflection
M (det −1) cannot be absorbed into the stored frame. Two lawful
conventions, each a reparameterization of the honest mirrored
surface M∘S:

- **Map the frame forward** (`axis' = M·axis`, `u_ref' = M·u_ref`):
  then `v_ref' = −M·v_ref` and the stored surface equals M∘S
  **reparameterized u ↦ −u** — the periodic/angular coordinate
  reverses.
- **Negate the axis** (`axis' = −M·axis`): the stored surface equals
  M∘S **reparameterized v ↦ −v** — the linear coordinate reverses
  (cylinder height, sphere/torus latitude, the cone's NAPPE sign,
  whose apex-relative meaning other code consults).

Either way, every stored parameter-space payload transforms with the
choice: `IsoCurve { u, v0, v1 }`, `Seam`, `param_start/param_end`,
pcurves, chart regions — the payloads `transform_rigid` currently
carries as map-invariant, an invariance that is a det = +1 theorem,
not a general one.

**The chart is user-invisible, so ONE convention suffices (Evan's
#909 question, answered).** For a given mirror plane M — any
orientation — both conventions represent the SAME point set M∘S
exactly; they differ only in how the stored chart names its points.
Nothing user-facing reads chart values: stable names are role-based
birth records; selectors take kinds and datum distances, never
parameters (SELECT-DESIGN); recipes store intent parameters; STEP
exports frames + pcurves validly under either. The one chart-shaped
entity a user can see — a periodic surface's SEAM edge — is carried
topologically by the mirror (it is a real edge, mapped like every
edge), and both conventions agree with the carried seam by
construction. The "u↦−u is natural for a vertical mirror, v↦−v for
upside-down" intuition lives in the MIRROR PLANE the user supplies —
which is fully general — not in the bookkeeping: mirroring across a
plane perpendicular to a cylinder's axis under u↦−u simply maps the
axis to its negation, same point set. Supporting both would double
the parameter-payload rewriting surface and the equivariance audit
for zero user-visible difference. Both conventions are involutive
(mirror∘mirror restores the original chart), so no round-trip
argument separates them either. Caveat recorded: if a user-facing
parameter door ever opens (evaluate-at-(u,v) through the API), the
convention becomes documented behavior at that door — documented,
not per-plane.

**Recommendation: u ↦ −u** (map the frame forward). The v
coordinate carries structural meaning consulted by invariants
(nappe, hemisphere, height sign) and negating the axis would move
that meaning; the u coordinate is periodic, and the payloads u-
reversal touches — seams, loop windings — are exactly the things a
mirror must rewrite anyway (loop orientation reverses under
reflection regardless of convention). Honest counterargument: u-
reversal touches the seam vocabulary, which is the subtler
machinery, and pcurve u-coordinates negate mod the period at every
cache. D9 conv. 4's "no left-hand rules" does not pick a side here
— both conventions are deterministic and frame-covariant; the rule
it does impose is that whichever is chosen must be applied
uniformly, with the residual sites documented.

## P2 — Mirror gets its own door; `transform_rigid` stays rigid

`transform_rigid`'s contract, name, seven-row K-ledger
(`transform_rigid_det_plus_one` among them), typed `NotRigid`
refusal, and its sense-invariance proof (the M5 S10 audit — "there
is no such branch to write here today") are all written around
det = +1, and two suites pin the det = −1 refusal. Widening it
invalidates a documented audit result. **Recommendation**: a sibling
door (`mirror_body`, or `transform_isometry` with the improper
branch) that composes: reflect geometry per P1's convention +
rewrite parameter payloads + the outward-normal flip `revert`
already implements exactly once per face (plane arm: negate stored
normal; curved arm: flip `sense`) + re-certify carriers and re-mint
witnesses per the transform layer's existing discipline. The +V
tier-3 invariant is the acceptance instrument: a reflection without
the flip produces `NegativeVolume`, and the correct door must not.
One claim the survey flags as **checkable, not assumable** (conv. 4
verbatim): whether reflection's loop-orientation reversal and the
normal flip cancel so that no half-edge reversal is needed — the
mirror unit proves or refutes it at the site.

## P3 — The equivariance audit's boundary (Evan owns the scope)

The survey enumerates ~15 orientation-sensitive site classes (sense
bits, plane-normal storage, chart handedness, circle/ellipse
winding, parameter payloads, pcurves/loop winding, 2-D chart
orientation, dihedral signs, +V, boolean orbit handedness, split-
rule equivariance — the one site already PROVEN, iso fingerprints,
readback triads, selection rules, STL/STEP export). Conv. 4's own
text: do not cite the kernel as equivariant without checking at the
site, and documented residuals are the sanctioned escape.

**Recommendation**: the audit's boundary is "every site a mirrored
body's data actually traverses", executed as the mirror unit's
checklist (the table above is the checklist's seed), with documented
residuals permitted where a candidate-swapping symmetry makes
equivariance impossible — the S8 rung-3 precedent. The alternative
(a residual-free mandate) makes the unit open-ended. Evan's call.

## P4 — Pattern-of-a-body stays the kernel truth (feature patterns are sugar)

This kernel patterns **bodies**: `Node::Pattern` → N placed bodies;
`PlacedUnion` → one certified-disjoint fused body; a hole pattern is
`PlacedUnion(cutter, rule)` → `Subtract`. Conventional CAD patterns
**features**, and the divergence shows in naming (`Instance(i)`
wraps the body's names, not a feature's roles). **Recommendation**:
keep body semantics as the kernel truth — it is already ratified
through GROUP-BOOLEAN A′ and shipped — and treat feature-pattern
spelling as recipe-generator sugar that LOWERS to body patterns,
added when a consumer demands it. The mental-model cost is real and
recorded; the alternative (a kernel feature-replication node) re-
runs arbitrary ops N times inside evaluation and buys nothing the
lowering doesn't, at the price of a second instancing semantics.

## P5 — Derivable items, adopted here unless Evan objects

- **`SegPat` instance-index predicate**: the selector cannot say
  "instance 7" except as a full exact path, which GROUP-BOOLEAN's
  "one-row selector addresses ball i's cavity face" promise assumed.
  Small additive `SegPat` arm; no ratified text blocks it.
- **Mirror × STEP**: import already refuses det = −1 by ratified
  choice (M7-4); export policy is A8's recorded residue and follows
  P1 mechanically (`advanced_face.same_sense` maps `Face::sense`).
  Deferred with its owner, not re-opened here.
- **Multi-output-body pattern masters** stay typed-refused
  (`name_pattern`'s recorded deferral) — nothing upstream can
  produce one; the refusal text already says so.

## P6 — Consequence for hole features: the register row is stale

KERNEL-VERBS lists hole features as "blocked mainly on patterns".
After PlacedUnion shipped, the substrate exists: counterbore =
stepped cutter × `PlacedUnion` × `Subtract`, today. What actually
remains: (a) the sugar-node vocabulary (follows P4's ruling), (b)
face-tied placements (GROUP-BOOLEAN's "staged, not riding" item),
(c) overlapping cutters, which sit behind G8's residual (the
multi-solid boolean operand — cross-program, LIB's register). The
register's row is corrected to say this by the PR that ratifies this
doc.

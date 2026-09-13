# BLEND — the blend kernel and the profile fillet door (plan)

**STATUS: OPEN (2026-09-06).** Successor to FILLET (closed 2026-09-06,
`docs/DOC-LEDGER.md` sweep 7), opened in the tracker-wide cut of
2026-09-06 (`docs/WORK-TRACKS-2026-09.md`, addendum 2). Live state is
`work/blend/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`blend/`** — unit branches
`blend/<unit>-<slug>`; the orchestrator branch is the session's designated
branch (`work/blend/log.md`, the readiness read) — the remote
`blend/orchestrator` is S-BLEND's and stays untouched.
Away-channel tag `(BLEND orchestrator)`. A/B ordinal band
**BLEND = 2900–2999**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

## Charter

FILLET finished the bands (the material-adding closed rim, the annulus
band with hostless crossings, the ruled band with its transverse
cut-off) and left, filed one file each, what its reviews measured on
the way: shapes the surgery refuses at a gate that is about the wrong
property, an unmeasured orientation in the ladder phase, two
must-carry arms with one rule and two policies, recourse sentences
that under-describe their doors, and a profile fillet door whose
output its own validator refuses. The substrate — `crates/sweep/README.md`
(BLEND-VOCAB V1–V4, ARMS3 A3-1…3 as amended by H4/H5/H7),
`crates/profile/README.md` (the `NoCornerOfPair` envelope),
`crates/topo/README.md` (`rim_of`) — is ratified and cited, never
re-litigated.

Territory: `crates/sweep/src/*` less `loft.rs` (S-BOOL's). The profile
fillet door is S-BOOL's glob and is edited here by announced seam, as
FILLET did.

## Review posture

Inherited from FILLET: full v6 dual with Fable specs for the H units;
E units take a single style review against
`docs/prompts/reviewer-style-lane.md`, no A/B row.

## Unit order

E, first — each one PR, the fix in the item:

1. `escalated-recourse-dispatch-has-no-coaxiality-arm` — one arm in
   `BlendError::Escalated`'s Display, one trio-family row.
2. `sweep-top-field-docs-make-the-spatial-claim-capend-shed` — the
   `Extruded.top` doc (and `Lofted.top`'s twin, S-BOOL's file, by
   note); prose or the rename to the sweep vector's ends.
3. `blend-recourses-under-describe-their-doors` §1 — the admitted-pair
   roster stated once and `FILLET3_SPINE_KIND_RECOURSE` derived from
   or checked against it.
4. `rim-seed-finders-disagree-on-at-this-radius` — one homed seed
   finder in `test_support.rs` with one stated tolerance and one
   reason; the ten copies deleted (tests: S-TCOST's glob, demos:
   Track X, both by announced seam).
5. `sweep-doc-comments-cite-tests-unenforced` — decide the cheap
   instrument or stop citing rows in prose; normalise the sixteen
   sites.

H, in dependency order:

6. `ring-clearance-refuses-a-nested-trim-circle` +
   `hostless-rim-on-a-ringed-host-refuses` — one unit: the
   outer-boundary circle arm takes the containment form, and the
   hostless annulus gets the same closed-form clearance; the two
   fixtures in the items are the acceptance rows. Spec
   `docs/BLEND-6-SPEC.md`; block BLEND-B1 slot 0.
7. `closed-chain-junctions-pair-with-a-rotated-link` — the junction
   list of a closed chain pairs each vertex with a link that does not
   touch it; a pristine three-arc rim refuses `ChainNotG1`. Found by
   unit 1's review; block BLEND-B1 slot 1, ahead of the ladder unit.
   Spec `docs/BLEND-7-SPEC.md`.
8. `ladder-rim-phase-may-retire-a-new-split-key` — a fixture whose
   meridian runs pole-to-rim first, then the `split_rim` guard shape
   applied to the ladder path. Spec `docs/BLEND-8-SPEC.md`; block
   BLEND-B2 slot 0.
9. `smooth-arm-siblings-disagree-on-the-in-band-case` — the in-band
   policy is the predicate's documented contract (in-band escalates
   typed; a behaviour change for revolve), one wrapper beside
   `tangent_second_order` (announced to PROPS). Spec
   `docs/BLEND-9-SPEC.md`; block BLEND-B1 slot 2.
10. `path-fillet-door-validator-tangency-disagree` — which side is
    right across four decades of turn angle; the door's stored arc or
    the validator's tangency test moves. Spec `docs/BLEND-10-SPEC.md`;
    block BLEND-B2 slot 1 (S-BOOL seam announced at dispatch).
11. `overrun-attribution-picks-the-first-candidate` — a stated rule
    for which corner-side candidate is reported, the FILLET-ATTR
    shape one level down: the nearest fit. Spec `docs/BLEND-11-SPEC.md`;
    block BLEND-B2 slot 2 (S-BOOL seam announced at dispatch).
12. `fillet-escalation-site-has-no-producer` — disposition (1): a
    fillet arm on `PathError::Escalated` keyed on the predicate name
    (BLEND-10's shape), one map for name → sentence, the producerless
    `ProfileError` site arm retired; a door change, so it follows 10
    and 11. Spec `docs/BLEND-12-SPEC.md`; block BLEND-B3 slot 0 (S-BOOL
    seam announced at dispatch); 14 is slot 1, 15 is slot 2 and
    dispatches after 12 merges.
14. `blend-contact-edges-mint-the-intrinsic-description-without-the-rule`
    — a blend's contact edge takes its description from the must-carry
    rule (BLEND-9's residue: the fourth site minting the intrinsic
    description outside the rule, measured Positive everywhere on the
    corpus and not definite by construction). Spec `docs/BLEND-14-SPEC.md`;
    block BLEND-B3.
15. `escalation-recourse-dispatch-has-three-homes` — disposition (2):
    one gap sentence at every unknown-name arm, a roster row per crate
    over every decided name, the dispatch order pinned; follows 12
    (which retires the third table). Spec `docs/BLEND-15-SPEC.md`;
    block BLEND-B3 (S-BOOL seam announced at dispatch).
13. `S90-impl` — stays blocked in fact on the lane-trait split `H5`
    names (PROPS' ground); this program owes the per-read
    classification of the nineteen bracket reads so the day `H5`
    lands the tightening is one PR. Last.

D, as `[ev]`:

- `ambiguity-k-below-the-cap-rim-crossover` — should `Tol` carry a K
  floor at all, and is it the crossover or a kernel-wide argument.
  Kernel tolerance policy; opened as an `[ev]` PR when Ev is next
  available and never resolved by implementing.

Closed at opening as a record: `curved-single-host-rim-refuses-at-the-half-band-gate`
(the shape arises through `kef`, refuses at the half-band gate on
both routes, and the statement lives at `HostSide`'s doc).

## Exit shape

The twelve land (with 14 and 15, promoted from the slate's residues on
2026-09-13, beside them; 13 stays blocked on PROPS' H5 and is walked as
such), the ruling is answered, Track T is empty; the walk convention
applies.

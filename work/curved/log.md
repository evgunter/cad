# CURVED log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/curved/plan.md`. A/B band 2200–2299
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose CURVED section is the
charter this plan restates. Opens at VERBS' exit. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `census-at-rest-two-boolean-lane-premises` from `work/mate/`
- `overlap-lane-boundary-crossing-cuts` from `work/mate/`
- `dev1-cylinder-sphere-circle-locus-arm` from `work/mate/`
- `m9-3-semantic-residues` from `work/mate/`
- `torus-declared-rest-lane-banked` from `work/mate/`
- `cylindrical-rest-pair-hits-planar-merge` from `work/mate/`
- `signed-penetration-depth` from `work/m10/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Opened for dispatch (2026-09-04)

New orchestrator (this session runs CURVED and TRIM). VERBS' walk was
ratified at #1793 and its directory swept (closure commit c1e7ea195);
S-MATE was swept into MSOLVE. Both named opening conditions hold.
Actions in the opening PR, on Ev's in-chat answers (sequencing is the
orchestrator's; design forks are Ev's):

- Adopted all fourteen VERBS re-homes from `work/issues/` by `git mv`
  (ids unchanged): `VERBS-C5ARMS`, `VERBS-CONE`,
  `arc-aware-point-in-loop`, `boolean-refuses-on-arc-carrier-not-arc`,
  `c5-plane-torus-cone-cylinder-arms`,
  `circle-residual-harmonics-needs-torus-arm`,
  `declared-cusps-second-order-wedge-arm`, `pierce-ring-has-no-join-arm`,
  `pinch-carrying-machinery-valence-4` (parked on SEAT-6),
  `plane-cone-elliptic-section-split-refusal`,
  `plane-nurbs-ssi-misblames-control-net`,
  `ssi-lever-arm-min-fold-hides-poison`,
  `torus-operand-boxes-span-whole-ring`,
  `verbs-1031b-assigner-checker-divergence` (kept here: the merge-door
  lane owns the same arc-winding machinery).
  `sphere-flux-arm-refuses-partial-bands` went to PROPS (a props unit).
- The S-BOOL fence written on both sides (`work/bool/program.md`
  keep_out, this program's paths/keep_out); the shared `boolean/*` and
  `splitting/*` globs stay claimed by both, the prose fence decides.
- Decision (unilateral, sequencing): first dispatch is `VERBS-C5ARMS`
  PR-2 (cone×cylinder; specced, small, the one executable VERBS
  remainder), then `cylindrical-rest-pair-hits-planar-merge` (the
  honest typed skip) and `torus-operand-boxes-span-whole-ring`.
- Decision (Ev, in-chat): the RIMCAP torus half / spiric carrier
  design conversation opens NOW as an `[ev]` PR, in parallel with the
  first units, joined with #1377's pinch machinery as one doc.
- Decision (Ev, in-chat): S-BOOL's ceded items (operand reach, the
  containment doors, the graft/boolean-declarations singles) are
  negotiated over the away channel for an early handover rather than
  waiting for S-BOOL's exit.
- Orchestrator branch `curved/orchestrator` (this program's state-sync
  for both programs rides here; TRIM's own log names it too). The
  away channel is armed on `curved/,trim/` with `@ trim` as an extra
  address; repo-wide NEW ISSUE/PR lines are narrowed to summons
  (`CAD_CHANNEL_NEW_EVENTS=summons`, added to the script this PR — Ev's
  in-chat ask: PR noise off, "@ curved" mentions must reach me).

## First dispatches and the design conversation (2026-09-04, later)

- **Block CURVED-B1** drawn branch-side (`curved/b1-block`): byte 53 ⇒
  fable at slot 2. Slot 0 = VERBS-C5ARMS PR-2 (Opus), dispatched on
  `curved/c5arms-2`; slot 1 REORDERED to CURVED-TORUS PR-1 (Opus) on
  `curved/torus-box` because its spec ratified first (#1874); slot 2 =
  the merge-door unit (Fable), spec lane still measuring.
- **`docs/CURVED-TORUS-SPEC.md` ratified (#1874)** with three binding
  refutations: the torus implicit is the linearized form (no harmonic
  triple; PR-2 encloses by subdivision with a certified `f2`); a
  boundary-tight box does NOT retire lily wall 1 (a concentric coplanar
  disc lies in every AABB of the larger circle) — PR-1's acceptance is
  a RE-AIM and #1488 is re-scoped accordingly; MATE-7a's "one function
  away" was measured on the coincident full-torus pair, not the lily.
  `Torus` on `boolean_arm_exists` is this program's third torus unit,
  filed as `torus-operand-gate-admission`. Rulings in the spec's §9.
- **`[ev]` #1858 opened**: `docs/CURVED-SPIRIC-DESIGN.md` (ruling item
  `spiric-carrier-ruling`). Measured: the shell rim is always the
  two-oval spiric regime (the double point is unreachable from the
  shell door); the pinch family's curves are ellipses already carried
  — RIMCAP's "one design doc" premise refuted; the general rung is
  blocked for tori by one function (the C9 ring's missing `sqrt`).
  Ev's first answer: a special-case kind like `Ellipse` (Q1=(b),
  Q2=(b1)); asked why (c2) reads as rejected — replied that (c2) is
  the general rung and stays the route for pairs without a closed
  form, and asked whether the ring `sqrt` (Q3(ii)) should open now
  (reading (B)) or after the variant ships (reading (A), recommended).
  Sign-off on Q3(i)/Q4/Q5/Q6 requested by 👍 (watchlist entry).
- **Handover request to S-BOOL** posted on #1835 (`@ s-bool`); no
  reply yet; nothing dispatches on those files until it comes.
- **Operations.** Six Fable/Opus lanes ran concurrently on an 8-core,
  9 GB box shared with another orchestrator (load 25–40): cold builds
  serialized on the machine mutex with waits of 1–2.5 h; two spec
  lanes could not take their one measurement and pre-registered it as
  the implementer's opening act instead. Three Fable spec lanes were
  killed by a transient Fable 429 and resumed by SendMessage without
  loss. The pre-push rustfmt hook outlived the SSH connection under
  this load ("Connection to github.com closed by remote host") and one
  ratification commit was silently not pushed — #1865 merged without
  its rulings, carried by #1876. Docs-only pushes from the orchestrator
  now use `--no-verify`; implementer briefs say what to do. GraphQL on
  the shared GitHub account is periodically exhausted; PR create/merge
  go through the REST API.

## VERBS-C5ARMS PR-2 merged (2026-09-05) — the first CURVED unit

PR #1864, ordinal 2200, sample #143; block CURVED-B1 slot 0 concluded.
The dual (R1 Opus, R2 Fable) both MERGEABLE-AFTER-FIXES; adjudication
on the PR (comment 5551363172); fifteen union items all taken. The
substantive design change from review: admission on the STATION
`|R·cot α|` against `extent` rather than an angular guard (R1's
measured ~101 ε off-surface mint at α = 1e-10 / extent = 100). The
pair is EXCLUDED from the A/B tally under 3(e) — R2 was killed twice
by the account's usage limit and resumed. Class findings filed:
`teapot-walls-have-no-suite-row`,
`c5-gate-admits-every-pose-of-an-implemented-pair`; log-only: the
predicate-dimension audit's counts go stale with every new name and
nothing re-takes them; three coaxiality policies now coexist in
`intersect.rs` (declared-and-verified, measured-and-refused,
measured-and-admitted). Operations: the `CI-Config` commit-trailer
path was deleted on main (`eeb912512`, 2026-09-04) — briefs must stop
mentioning it; the k-lint draw is retired and all five rows run.

## Doc rot relayed by TOPO (2026-09-06)

`docs/CURVED-MERGEDOOR-SPEC.md:185` cites `is_arena_fault (:447-452)`
as untouched; TOPO's D265 (PR 2013) rewrites that function (now
`crates/topo/src/merge_faces.rs:583-618`) to classify the merge door's
own refusals. The spec's premise — a non-`Op` variant is never an
arena fault — still holds; the line cite and "untouched" do not.
Found by D265's reviews; the file is this program's.

## CURVED-TORUS PR-1 merged (2026-09-07)

PR #1907, ordinal 2201, sample #152; block CURVED-B1 slot 1 concluded.
The dual (R1 Fable, R2 Opus) both MERGEABLE-AFTER-FIXES; adjudication
on the PR (comment 5563489727); twelve union items all taken. The
substantive change from review: the box lane carries the Decide lane's
ring and wrap guards (R1's lone-circle-loop face, a latent wrong box).
R1's MAJOR is a tally candidate (no reviewer interruption). Filed at
adjudication: `sphere-operand-box-is-the-whole-ball` (CURVED),
`pcurve-chart-box-is-looser-than-harmonic-extent` (TRIM). Logged, not
filed: `ARC_SAMPLES` over a full `2π` of `v` makes the `v` charge
dominate (1.2e-3 m vs 3.6e-4 m on the lily; 2 % of the tube radius
against an 8 mm headline) — a follow-up if a consumer measures the
need; the sibling `!= 1.0` rational gates across three programs'
fences (from TRIM-1's dual) are over-strict by that unit's own insight.
Operations (2026-09-05/06): a Fable-side limit blocked this session and
every Fable lane for ~24 h; three lanes were resumed from transcript
without loss. `work.py --selftest`'s date-pinned fixture expired on
2026-09-07 UTC and reddened one run before main cleared it.

## Announced from LIB (2026-09-09): a derive word on `SurfaceKind` and `CarrierRelation`

LIB-MIRROR (PR #2271) adds `Hash` to `geom_brep::SurfaceKind` (`intersect.rs:109`) and `topo::CarrierRelation` (`boolean/carrier_eq.rs:64`) so the Python tag mirrors match their Rust derives under Ev's (A) ruling on `[ev]` #2265; nothing else in either file moves.

## Resumed after a six-day outage (2026-09-13)

A Fable-side limit killed the merge-door lane mid-fix-pass on
2026-09-07 and blocked this session until 2026-09-13; the account's
weekly window has since reset (Ev, in-chat: proceed with the rest of
the slate). Main moved ~3000 commits meanwhile; two items were filed
onto this slate by other orchestrators and are acknowledged:
`axis-coincident-lap-trips-the-planar-join-invariant` (a box lap whose
plane contains the cylinder axis reaches `chord_join.rs`'s planar-lane
conic guard through the public subtract door — the operand-reach lane)
and LIB's derive announcement on `SurfaceKind`/`CarrierRelation`.
Block CURVED-B2 drawn branch-side (byte 22 ⇒ fable at slot 1): slot 0 =
CURVED-TORUS PR-2 (Opus) on `curved/torus-arm`; slot 1 = the spiric
carrier unit (Fable; spec lane opened); slot 2 banks. The merge-door
lane (B1 slot 2, Fable) resumes its re-scope. Operations: a merge of
main into the block branch conflicted in the A/B log and the
orchestrator committed the markers before noticing — repaired by a
resolving commit (union of both appended sections), never rewritten;
the whole-tree marker grep is now part of every merge here.

## Announced seam from TOPO (2026-09-14): the `loop_shape` change is on your ground too

`crates/topo/src/boolean/*` is claimed by both S-BOOL and this
program, so the announcement written on `work/bool/log.md` for
2026-09-14 — `contain::loop_shape` and `LoopShape` going
`pub(crate)`, and `LoopShape::Parity` splitting into `Polygon` and
`ArcParity` with `contfp`'s behaviour unchanged — is repeated here
rather than left visible from one side only. Read it there; nothing
differs. TOPO PR 2529, branch `topo/tier3-ring-nesting`. No action
asked. Signed (TOPO, the ring-nesting lane).

## Announced from TOPO (2026-09-14): `docs/CURVED-MERGEDOOR-SPEC.md`'s citations of `merge_faces.rs` moved; its design did not

TOPO `D263` (PR 2548, `topo/d263-placeholder-regime`) gives the merge
door a third surface kind, `topo::MergeKind { Plane, Curved,
Placeholder }`: `group_regime` is now `group_contract` (returning
`GroupContract::Runs { regime, kind } | SetAside`), `GroupKindSplit`'s
fields are `face, kind, other, other_kind`, and a `Nurbs` net in
`NetState::Poisoned` refuses `MergeCoplanarError::PoisonedSurfaceDescription`
before any group forms. The spec's citations of `group_regime`
"untouched" and of `GroupKindSplit`'s old lines were re-worded on that
branch — a description that moved, not a design change; the spec's
shape and fences are as they were. One note was added beside the
plan's "one non-planar kind on both sides → a skip record" arm: it
must read `MergeKind`, not `SurfaceKind` alone, or a declared
placeholder pair would be skipped where the door refuses it. The
finding is recorded on `cylindrical-rest-pair-hits-planar-merge`
(`## Read MergeKind at the classification`). No action asked. Signed
(TOPO, the D263 fix pass).

## Merge-door and torus-arm duals adjudicated (2026-09-14)

Merge-door (PR #2105, ordinal 2202): both arms MERGEABLE-AFTER-FIXES
and both EXECUTED the honesty crux the same way — the bore chord
pre-exists on the merge base (with the door reverted, the record
suppressed, and the whole recorded-faces walk removed, scene A still
refuses `JoinDesync`; with `describe_minted_edges` skipped the volume
backstop rejects the same body); the STOP-2 re-scope stands. Fixes:
the record's liveness invariant pinned (the PR's "M6 not observable"
was false — a sub-period curved run commits through the kind-agnostic
same-key rung), no record for a pair with zero live faces, sphere/torus
rows, the vacuous `merge_groups` assertions replaced, dedup at the
door, the mechanism and additivity history corrected. Torus arm
(PR #2535, ordinal 2203): both arms MERGEABLE-AFTER-FIXES; the bound
and enclosure held under both arms' random dense oracles, but the
shipped rows could not see a wrong bound on an ordinary fixture (the
PR's "M2 merely loose" was false in the unsafe direction — real
escapes on random fixtures); a torus family in the dense-sampling
soundness suite and a random-configuration row are the fix; the spec
amended (the monotonicity theorem, the charge table at the arc-scoped
`f2`). Both fix passes dispatched. Pacing: **the spiric PR-1a dual is
HELD for next week's budget**; PR-1a itself continues.

## CURVED-MERGEDOOR merged (2026-09-14) — block CURVED-B1 concludes

PR #2105, ordinal 2202, sample #192; block CURVED-B1's last slot. The
dual's fix pass took all eleven items (fourteen rows, fifteen mutants
red); the record is keyed off the declaration and documented so. Next
on the merge-door lane: `rest-zip-seam-chord-on-cylinder-wall` (the zip
defect the door had hidden). Operations: the fix pass's push produced
no `synchronize` run and the lane dispatched the workflow by hand (the
render lanes skip on a dispatch); the state-sync push re-rolls a real
run before the merge.

## CURVED-MERGEDOOR close-out (2026-09-14)

Block CURVED-B1 concluded at #2105's merge: slot 2 recorded on
`curved/b1-block` (1c10adfa7) and the block record folded to main in
#2578 (6781958cb), which also deleted `docs/CURVED-MERGEDOOR-SPEC.md`
per the ledger. Lane and target reclaimed. CURVED-B2's draw stays
branch-side. Running: torus-arm fix pass (#2535), TRIM-3 PR-2 dual
(#2554). Held for the weekly reset: spiric PR-1a (#2566) and TRIM-2
PR-1 (#2564) duals.

## CURVED-TORUS PR-2 merged (2026-09-15) — block CURVED-B2 slot 0 concludes

PR #2535, ordinal 2203, sample #200. The dual (R1 Fable, R2 Opus) both
MERGEABLE-AFTER-FIXES; adjudication on the PR (comment 5662086549);
twelve union items all taken. The substantive change from review: the
suite had no random dense-oracle row for the torus, so four planted
defects passed every shipped row — a torus family and a random row now
red them; the PR's "M2 merely loose" sentence was wrong in the unsafe
direction and is withdrawn in the body. Headline bilateral — no tally
candidate; R2 paused once by the usage limit (3(e)). The spec's
2026-09-14 amendments stand; `docs/CURVED-TORUS-SPEC.md` is now fully
delivered (PR-1 #1907, PR-2 #2535) and leaves `docs/` per the ledger in
the post-merge docs PR. Filed by the unit:
`the-chord-dip-charge-has-two-homes`. Operations: the account behind
this session changed at Ev's re-login (2026-09-15); the weekly window
on the new account sits at ~80% with its reset 2026-09-18 17:00Z, so
the held duals (spiric PR-1a, TRIM-2 PR-1) stay held until that reset.

## Pacing under the weekly budget (2026-09-15)

Both of this orchestrator's first blocks are concluded (CURVED-B1 at
#2105, TRIM-B1 at #2554); CURVED-B2 slot 0 concluded at #2535. Open
against the budget: the spiric PR-1a dual (#2566, CURVED-B2 slot 1,
FABLE) and the TRIM-2 PR-1 dual (#2564, TRIM-B2 slot 0). The account's
weekly window sits at 83 % with its reset at 2026-09-18 17:00Z; both
duals stay HELD until that reset, then dispatch together (each dual
plus fix pass has cost 0.6–1.0 M tokens on this program). No new
implementer dispatch before the reset; CURVED-B2 slot 2's unit is
chosen at dispatch from the plan's lanes (leading candidates:
`equator-seam-reauthor-refuses-the-hollowed-elbow`, then spiric
PR-1b once 1a merges). Idle lanes: none; every finished lane and
target reclaimed (44 G free).

## Spiric PR-1a dual dispatched (2026-09-15)

Ev, in-chat after the box's reboot: finish the open duals and get all
state on main before the usage limit. The hold is lifted: ordinal 2204
claimed (PR #2669); byte 226, parity 0 ⇒ R1 Opus, R2 Fable; frozen
head `e9ef3ae3b`; briefs stored with sha256; both reviewers dispatched
together with TRIM-2 PR-1's pair (four lanes on the width-1 slot).
Session monitors re-armed after the crash; nothing of this program's
was running when the box went down.

## Spiric PR-1a dual adjudicated (2026-09-18)

Ev, in-chat: the usage limit has reset. R2 (Fable) had been killed by a
model-side 429 during the hold and was resumed from transcript; both
arms MERGEABLE-AFTER-FIXES with no MAJOR (R1 Opus 4 MINOR/8 NOTE,
rubric 4/4/3/4/5; R2 Fable 3 MINOR/3 NOTE, rubric 3/4/3/4/5). No wrong
number in the shipped kernel: both arms re-derived the closed forms by
hand, reproduced every PR mutant with its exact payload, and both
demonstrated M7 on the vessel (STOP-1 ruling 4 closed). Converged:
row 2 cannot see M3 on the vessel (`R/r′ = 1.33`; needs `R ≥ 3r`),
`edge_pose`'s `Pose` contract, M5's reader (R2's wide-bracket Interval
row; the shipped Interval box row is its f64 twin bit-for-bit), the
Python-mirror contradiction, the refuted row names. Unique: R2 the dead
`torus_boundary` arm (the wall takes the uncached-pcurve quadrature
door until 1b) — claims-class; R1 four unexercised census arms and the
missing deciding constructor. Adjudication on the PR (comment
5733631165); thirteen items; fix pass dispatched. Tally: no candidate;
R2 killed once by a 429, and R1 disclosed reading the arm's name in the
STOP-1 ruling comment (an orchestrator text — instrument leak noted:
STOP rulings on a PR under review must not name the slot's arm) — 3(e)
excludes the pair. **Slate change**: S-BOOL exited (walk 2026-09-16)
and re-homed its residue by file — CURVED received ~30 items,
including `cosurface-disjoint-curved-walls-refuse` and
`boolean-declarations-has-no-geometric-producer` from the plan's
"S-BOOL's today" lanes; the handover the plan waited on happened by
exit rather than reply. The plan is re-cut after the two open units
merge.

## CURVED-SPIRIC PR-1a merged (2026-09-19) — block CURVED-B2 slot 1 concludes

PR #2566, ordinal 2204, sample #221. Fix pass from the dual: all
thirteen items taken; the one behaviour-shaped change is the deciding
constructor `Curve3::spiric(...)` (four named regime predicates on the
audit; `mint_carrier` mints through it; bit-identity re-run identical).
The main merge crossed main's new `InfSpeed` speed meter in five files
(spiric arms answer `InfSpeed::new(minor_radius)`) and the lune's
renamed premise `props_meridian_great`; the merged head's real
`pull_request` run is the verification of record (a dispatch run on
the conflicting pre-merge head had its render lanes skipped). Next in
the lane: PR-1b (`Pcurve::Spiric` + STEP) on `curved/spiric-1b`, then
`equator-seam-reauthor-refuses-the-hollowed-elbow`. Instrument note
for the A/B log: STOP rulings posted on a PR under review must not name
the slot's arm (R1 read "FABLE" in comment 5662340417).

## Spiric PR-1b dispatched (2026-09-19)

Ev, in-chat: proceed to the next dispatch. CURVED-B2 slot 2 = spiric
PR-1b (`Pcurve::Spiric` + STEP, spec §3/§5, rows 9–10, the C4 README
line 1a deferred), pre-draw M / STRUCTURAL logged branch-side, arm
OPUS by the block's draw; brief stored (sha256 758dec8323…); lane
`curved-spiric-1b`, branch `curved/spiric-1b`. The brief carries 1a's
adjudication notes that are 1b's to close (the wall's payload once the
cache lands; the `param_on` anchor branch if a longer-span fixture
appears). Operations: the session's Monitor tool expires every 30
minutes (a harness bug, Ev in-chat) — the four watchdog scripts run
detached into one event log and the orchestrator waits on it with
one-shot background commands.

## Spiric PR-1b delivered; dual dispatched (2026-09-19)

PR #2861 (head b0afaf200, run 35435836485 green; `render drift (uv)`
neutral — the uv-montage legend row). The variant, the certification
arm (deviation 1: a banded structural compare, `certify` being a
`Decide` door), fourteen consumer sites, STEP export with the
`FILE_DESCRIPTION` sentence. Opening measurement found the whole torus
face's cache set had been EMPTY at the merge base (the missing arm made
`mint_faces` clear the face, circle rims included) — a regression 1a
left and 1b closes. Finding filed against the ratified §5:
`spiric-step-spline-bound-is-second-order` (the export's `ε/4` gate is
unreachable on real fixtures; a sharper `sup‖C″‖` candidate would move
mesh chord counts) — ruled after the dual. Dual: ordinal 2205, byte
240 ⇒ R1 Opus, R2 Fable.

## Spiric PR-1b dual adjudicated (2026-09-20)

R2 (Fable) killed once by a 429 and resumed after the reset; both arms
MERGEABLE-AFTER-FIXES (R1 Opus 2 MAJOR/7 MINOR/7 NOTE, rubric
4/4/2/4/4; R2 Fable 1 MAJOR/5 MINOR/7 NOTE, rubric 4/4/3/5/5). Every
PR number reproduced on both sides; both hand re-derived the images;
both confirmed the filed STEP finding and the sharper `sup‖C″‖`
candidate (76×, not rescuing `ε/4`). Bilateral headline: two new
predicates decide dimensionless quantities through `over_lever`
(divide) where the spec says levered — false audit rows, no verdict
moved. Also bilateral: the identity's chart-equals-carrier premise
gated by nothing (a drifted chart certifies with envelope 0), three
silent mutants, the "bit-equal" doc. R1 alone: `mirror_v` on a
`SpiricImage::Wall` yields a wrong locus and the every-kind
involution census was not extended — code-class, unreachable in-tree,
a tally CANDIDATE (R2 reached the site from the style side); pair
EXCLUDED under 3(e) (R2's 429). Adjudication on the PR (comment
5746969779); fourteen items; fix pass dispatched. Spec note for the
ledger at deletion: §3's `nurbs_tighten` sentence was wrong (the
harmonic arm does not skip; the lane refused, correctly).

## The cut (2026-09-20)

Ev, in-chat: "i sure put a lot on your plate … do you think it'd make
sense to break them up into smaller tracks, leaving only a chunk sized
to be finished in this session in the original curved and trim?" —
yes, and "you can do the split — no need for an [ev] pr since it is
mostly moving issues around". Done in one commit: **REACH**
(`work/reach/`, band 6000–6099) takes the boolean lanes and S-BOOL's
residue, 49 items; **TANG** (`work/tang/`, 6100–6199) the
declared-tangency and germ/pierce lanes and the pinch design, 8;
**CHART** (`work/chart/`, 6200–6299) the three SSI drive-bys with
TRIM's chart-side residue. CURVED keeps four items: the spiric unit
(PR-1b in its fix pass), the equator-seam re-author, the C5 demo
half and a ledger fix; its exit is the spiric carrier delivered and
the elbow hollowed. Plans and program files re-cut; the former lane
list and the S-BOOL fence are in this plan's history. **Protocol v7**
(Ev, 2026-09-19, recorded in the A/B log's banding entry) read at the
cut: the dual runs on triaged-in units only from here; the
equator-seam unit is E–M and runs opus/opus outside it unless its
spec finds a decision.

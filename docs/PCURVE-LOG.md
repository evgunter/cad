# PCURVE log — edge-description unification

Narrative record; the plan is `docs/PCURVE-PLAN.md`. Convention as
in the other programs: seam entries at pipeline seams, unit entries
at merges, the tail is the live state.

## Opening state (2026-08-27, proposed)

Opened at M9's close on Evan's direction ("having a pcurve track as
a concurrent program sounds good"), as a third concurrent program
beside LIB and VERBS. The subject is what M9-D ratified and
deliberately did not schedule.

**Why it needed a home rather than a slot on the next milestone.**
U2 was ratified 2026-08-15 (#514) with scheduling delegated — "a
post-M9 kernel candidate, orchestrator-scheduled" — and never got a
tracking issue. At M9's exit walk it was the largest carried item
with no owner and no issue, visible only as a sentence in a design
doc. That is the same shape as #1027's rule one level up: a ratified
WORK ITEM with no durable home is as invisible as a finding with
none.

Slate at opening: P-1 the migration itself (binding constraint:
adoption's bitwise reproduction and D9 bit-replay), P-2 #498's
`General` home, P-3 lily wall 8's flip. Adjacent and measured but
unowned: #1058 item 3 (per-call whole-body pcurve mint, 16–23 ms at
3–6 charts, lever named). Explicitly excluded: the germ-chord SSI
lane, #968/#1059's operand-gate questions, M10's clearance
certificate.

Carried in from M9 by ratification rather than assumption: **Q3 is
settled** — the authority record is per-edge KERNEL data, its
pushback window having closed unexercised when Evan ratified the M9
exit walk (#1041). OQ4 is not re-opened.

Sequencing note recorded at opening: M9-3 minted its emission shapes
1:1-mappable onto (surface, pcurve) precisely so this migration
would be mechanical at the join lane's seams. Whether that held is
P-1's first measurement, and it is the cheapest early test of
whether the design pass paid for itself.

Next actions: Evan's ratification of the plan; then P-1's substrate
exploration before any spec.

## P-1a MERGED (#1073 at 9fa321d4, 2026-08-27) — one conventional description exists

The program's first unit. `EdgeDescription` collapses D2's conventional
variants to ONE `Chart(ChartCurve{surface, pcurve, seam})` arm plus a
fenced `Scaffold`; `Pcurve::General` lands at the Fitted grade through a
door that runs `run_fitted_checks` verbatim; D1's unified meter carries
the two seam predicates as a periodic-chart obligation rather than a
second form; `EdgeAuthority` is stored per edge for P-1b's tier-3
predicates to read. The `Copy` loss went to BORROW, so the 22 deref
sites stay untouched and P-1b pays them against a type it can borrow.

**The unit's real finding, reached three ways.** The collapse changed
the seam meter's QUANTITY, not its bits: `implicit_residual`'s cone arm
measures the perpendicular distance to the generator, `|C − S(P)|` the
radial chord, ratio `sec α`. R1 proved an edge that certifies on main
now ESCALATES; R2 reached the same finding wider (sphere, cone
verdict-flipping, and cylinder at r = 1e-4 — so the row's r = 2 fixture
was exactly where the move rounds away); the implementer's own drift
sweep found it a third time at the coarse decade. **Disposition: the
collapsed quantity is the RIGHT one, re-baselined deliberately** — and
the decisive ground was verified rather than argued, that the pcurve
CACHE lane already imposed exactly this statement on exactly this
geometry through exactly this mint. The collapse does not invent a rule;
it removes a place where the kernel said two different things about one
edge. The price is pinned as an assertion of the new behaviour, so
re-certifying it later costs an argument.

**Two lessons this unit bought, both now standing rules.** A detached
job whose evidence is superseded still takes the mutex — its own
footnote battery starved three review lanes for 2h18m
(`memories/agent-lane-operations.md`, #1085). And a tripwire pinned at
ROUND anchors is blind to exactly what it exists to catch: the delta
round falsified the implementer's "no-op, not a survivor" claim by
showing two re-associations coincide at 0, 1.7, 1e3, 1e6 while
separating at ~52% and ~1.2% of arbitrary anchors. Round numbers are
where re-associations agree.

The dual (ordinal 84, v6's first randomized-slot draw) is EXCLUDED from
the tally and the twelve: R2's batteries were killed twice, so the arms
were not comparable. Findings fully fixed, simply not scored.

Board: **P-1b next** — the consumers, the transience fence (`ops.rs`'s
two mint sites and fillet's six strut sites), `nurbs_iso_derive`'s
conventional arms, tier 3 switching onto `EdgeAuthority`, and the shim
field plus 22 deref sites to pay. P-2 (#498's `General` home) follows.

## Census gap 2 MERGED (#1080, 2026-08-27) — the flush seat certifies

Not PCURVE work — a standalone kernel unit (#943's gap 2, issue
#1063) carried by this orchestrator alongside the program, recorded
here because this is where the orchestrator's narrative lives.

The obvious way to draw a seated part — a post flush under a shelf —
could not certify. Two independently authored coincident planes share
no chart, so Door 2 had nowhere to compute the overlap and declined;
the demo had been authored with an INSET, its comment saying flush ≡
inset until this landed. It now takes the pair's shared **world
carrier** as the chart, and the demo's bench seats are flush with the
A5 gate ASSERTING `Certified` rather than printing it.

**The unit's real content is the lemma, and it is written where the
next reader meets it** (at `world_carrier`, not in a PR body): both
chart maps are isometries, so when `carrier_agreement` decides Zero,
ψ = φ_B⁻¹ ∘ φ_A is within ε a plane isometry, and every downstream
quantity is a Euclidean invariant. Orientation — the one thing a
reflection does not fix, and exactly the field an opposed `Rest` pair
differs in — is absorbed **structurally** by CCW normalization before
the machinery runs. Stated honestly beside it: the *certified*
answers are invariant, the *refusal boundary* is not exactly, because
`ray_parity`'s schedule rotates with the frame. The reviewer judged
that concession **the right size** after supplying the rotations the
shipped corpus never exercised.

**U-R2's ratified justification was FALSE and is corrected.** The
design argued a plane's world embedding "does not have chart
parameters"; `Surface::Plane` carries `u_ref`, and `world_carrier`
returns `s_a` — it picks A's frame as the REPRESENTATIVE. What
licenses the choice is frame-invariance of the answer, and "exact" is
too strong besides: `decide`'s `Ok(Zero)` is `|m| ≤ zero`, not
bit-zero. The honest claim is **certified everywhere within ε**.

**What lifting the scope constraint bought.** Evan cleared the lane to
edit M9-2's code; auditing the whole door then found TWO more
asymmetries beyond the reviewer's one — `collinear_offset` also
one-sided, and `overlap_of_regions`' containment arm refusing on one
relation's `None` before consulting the other. The fix
(`|r|` → `min(|r|,|s|)`) is the union-max of the candidate margins,
so it is symmetric by construction and never less definite: nothing
that certified before stops certifying.

The dual (ordinal 200 — the first banded claim) is EXCLUDED from the
tally and the twelve, because the orchestrator relaxed R1's method
under mutex saturation and not R2's. The fault is the orchestrator's;
the findings are real and fixed.

## Census gap 2's band-edge row went RED on main, and why (#1102, fixed #1108, 2026-08-27)

**The unit above merged with a stated coverage gap, and the gap was
the row's own subject.** #1080's CI drew `EPS=default`; the row it
added is *about* what happens across the ε band. Main went red at the
1e-12 draw within the day, and two other programs inherited it. The
orchestrator's fault, stated as a rule so it binds the next unit: **a
stated coverage gap is a blocker when the untested axis is the row's
own subject.** Adjacent gaps are follow-ups; the subject is a blocker.
Hosted CI draws ONE ε per run from the seed, so a green check is
evidence about that draw and nothing else.

**The defect was exactness, not drift.** The orchestrator's hypothesis
was that `(10ε).atan().to_degrees()` drifts at tight ε so the fixture
misses the threshold. Measured, the round trip is **exact to 0 ulp at
1e-9 and 1e-12** and loses 5e-16 relative at 1e-6. `decide` calls
`|m| ≥ K·ε` definite, so a fixture placed exactly ON the threshold has
its side chosen by whatever the kernel's arithmetic adds downstream —
Newell plane, chart projection, CCW normalization, scaling — and that
residue moves with ε. The `k = 10` cell is the whole incident:

| ε | A→B | B→A | old row |
|---|---|---|---|
| 1e-6 | Escalated | Escalated | green |
| 1e-9 | Ok(PositiveArea) | Escalated | green **by luck**, one entry in `seen` |
| 1e-12 | Ok(PositiveArea) | Ok(PositiveArea) | `seen` empty → **red** |

So the row was a coin-flip at *every* ε; default merely landed heads.
In one sentence: a row whose subject is *"the band edge is where noise
decides the verdict"* placed a single fixture on that edge, and so
depended for its own liveness on the coin-flip it exists to document.

**The replacement is stronger, not weaker** — the fix pass was told
not to weaken the row to make it pass. Operating point fixed, scale
swept (`sin θ = kε`, k ∈ {5,8,9,10,11,12,20}), asserting three
properties none of which depends on where a cell lands: no pair is
ever certified two DIFFERENT answers; co-escalating orders agree in
margin MAGNITUDE to the derived noise bound `1e3·EPSILON/sin θ`
(a wrong lever separates them by a length RATIO instead — #1080's
MAJOR); and a certify-versus-refuse split may occur ONLY within that
bound of a threshold. The third is a **confinement** claim the old row
could not state at all, and it fails on a split in the band's
interior. Non-vacuity is structural rather than lucky: k = 5, 8, 9 sit
strictly inside `(ε, K·ε)` at every ε, so the asserted
`escalations >= 3` cannot go quiet unless the band itself moved.
Verified locally at 1e-6 / 1e-9 / 1e-12, 11 passed 0 failed at each,
before merge.

**Lane operations, recorded because it cost ~50 minutes.** `-x` waits
for ALL slots and flock has no queue, so single-slot jobs arriving
AFTER an `-x` waiter is armed still beat it. A one-lane courtesy yield
therefore does not clear a path for an exclusive job — that needs a
machine-wide quiet period. The blocked lane pushed and opened its PR
while waiting, since hosted CI needs no local slot.

## P-1a's six-anchor bits row: the stated residue was already closed, unannounced (2026-08-29)

P-1a's PR body (#1073) and P-1b's spec both flagged the same gap:
`certify::tests::d2_the_mint_arithmetic_is_pinned_in_bits` — the D2
mint tripwire, `crates/geom-brep/src/certify.rs` — had only ever drawn
the interval compile lane, never default, for a row whose entire
subject is bit-level reproduction. P-1b's spec named the fix directly:
*"if this unit's heads draw default, say so, since that closes a gap
P-1a flagged honestly."* Its own heads never drew default —
`extrude_interval.rs` in the diff pinned `LANE=interval` throughout —
and the PR body said so and left it open.

**Measured before acting, per this repo's standing rule that a
refusal's text is not evidence of its cause: the row was never
gated.** No `cfg(feature = "interval")` anywhere in `certify.rs`;
`cargo test -p geom-brep --lib d2_the_mint_arithmetic_is_pinned_in_bits`
with no `interval` feature passes locally, unsurprising for a row that
mints and compares plain `f64` bits. The gap was purely a lane draw
that never landed on default — until it did.

**It already drew default, one commit before merge, unrelated to this
row.** P-1b's tip commit (`3e82959b`, "interval lane: the census row
behind the feature gate…") carries `CI-Config: lane=both` for its own
reasons — surfacing a different interval-gated row. That trailer
overrides `_forces_interval`'s pin per-dimension (`ci-filter.py`,
`decorate`), so its hosted run drew **both** lanes. In the default
half — [`test (eps = 1e-12, 1/2)`](https://github.com/evgunter/cad/actions/runs/33238268448/job/99064063771)
of [run 33238268448](https://github.com/evgunter/cad/actions/runs/33238268448) —
the row ran and passed:

```
PASS [   0.006s] ( 439/2086) geom-brep certify::tests::d2_the_mint_arithmetic_is_pinned_in_bits
```

`change filter`'s own output on that run reports
`CONFIG_SOURCE=lane:commit-trailer …` — the draw was requested, not
sampled, so this is not a lucky coin-flip masquerading as coverage.
`3e82959b` is the second parent of `9b8e9013` (`#1107`'s merge into
`main`), and `certify.rs`'s tripwire body is byte-identical between
that commit and `main`'s current tip — the six anchors that ran there
are the six anchors on `main` today. The residue named in both PR
bodies closed on its own, one commit before the merge that shipped it,
and nobody said so. Recorded here so the log's tail is honest about
it: **closed, not open.**

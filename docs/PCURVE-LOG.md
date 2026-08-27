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

---
id: shell-doors-take-tolerance-beside-tol
kind: issue
title: shell / shell_open take a raw tolerance: f64 beside tol: Tol, and the acceptance that no verb takes a Band beside a Tol has no mechanical guard
status: closed
opened: 2026-08-31
closed: 2026-09-05
github: 1409
refs: [1399, LIB-G17, shell-needs-shellnaming-birth-channel]
---

## From GitHub issue 1409

Opened 2026-08-31; 0 comments.

(SEAT orchestrator) Two related findings from SEAT-1's dual review (PR #1399), filed as one issue because both are about the verb doors' tolerance vocabulary.

**1. The instance.** `topo::shell` and `topo::shell_open` still take a raw `tolerance: f64` parameter beside the `tol: Tol` witness (`crates/topo/src/shell.rs`, the doors' signatures). That is the exact shape SEAT-1 just removed for `Band`: a second tolerance-flavored argument whose relationship to the committed global is the caller's problem. Whether this `tolerance` is genuinely independent (a per-call offset fitting budget, say) or another derivable-from-`Tol` value needs measuring before any change — SEAT-1's rule was "drop the parameter only after proving every caller passes the canonical derivation". Out of SEAT-1's ratified scope (VERB-SEAT-DESIGN §1 S4 names `Band` only), so recorded here rather than widened into that unit.

**2. The guard gap.** SEAT-1's acceptance clause — *no public kernel verb takes a `Band` beside a `Tol`* — is now a measurement with no mechanical guard: a future verb could reintroduce the parameter and nothing goes red. If the parameter-lint machinery (`docs/PARAM-LINT-SPEC.md` / `tools/k-lint`) grows signature rules at some point, this is a candidate row; until then this issue is the durable record that the invariant is unguarded by design, not by oversight.

## Home

`work/seat/` — the verb doors' tolerance vocabulary is SEAT's §1 ground (band derivation at operation entry) and both halves are SEAT-1's own disclosed residue.

## Ruled (Ev, PR 1904, 2026-09-05): (i) — derive at the door, and ε never travels as an `f64`

The shell doors drop `tolerance: f64`; the fit target IS ε_precision,
the one global ε D4 names. Ev's refinement binds the whole chain: **the
tolerance is passed as the ZST witness `Tol`, never as an `f64`** — so
it shows in every signature between the shell door and the one site
that classifies the residual, and no arithmetic can be done on it that
would make two callers effectively use different epsilons. The one
`tol.eps()` read lives at the classification site inside the fit
engine. The measurement that answered Ev's question (tier-3 validation
re-certifies every `Approx` face against the RUN's ε, so a looser mint
is a typed refusal, never loose geometry; every caller passed `1e-6`
into analytic offsets that ignore it) is in this file's history at the
`[ev]` PR. Unit: SEAT-9 (`docs/SEAT-9-SPEC.md`), block SEAT-B3. The
NURBS fit's COST at ε ≈ 1e-9 is the offset-fit owner's measurement,
reported by the unit, not gated on.

## Closed (2026-09-05, SEAT-9)

**Half 1, the instance, is executed.** `topo::shell` and
`topo::shell_open` no longer take a `tolerance: f64`: the witness is
the only tolerance either takes, and it travels down the offset chain
— `replace_face_offset`, `replace_faces_offset`, `mint_offset`,
`PropsQuadLane::approx_offset_surface` and
`PropsQuadLane::recertify_approx` — with the value read once, at
`topo::props::fit_precision`, the last kernel-side door before
`geom-brep`'s fit engine. Thirteen of the fifteen `FIT_TOL` constants
retired with the parameter; the two that remain name the FIT ENGINE's
target rather than a shell door's, and the boundary that leaves them is
argued in SEAT-9's PR body with the measurement that forced it (the
`bowed()` fixture reaches ε = 1e-9 in 3 refinement rounds and REFUSES
at 1e-12, so an engine that took only the witness could not be measured
at all and its probe suite would red on CI's tightest eps row).

**Half 2, the guard gap, is answered in kind rather than by the
parameter lint.** `crates/topo/tests/shell_tolerance_chain.rs` is a
source census over the chain: no signature on it may take an `f64`
whose name reads as a tolerance, and the fit target must be read at one
site. It reds on a planted second read (demonstrated) and on a
reintroduced parameter (demonstrated, on a real `eps: f64` before the
sentinels were placed). Its blind spots are stated in its own header.
`docs/PARAM-LINT-SPEC.md` / `tools/k-lint` still grow no signature
rules, and this census is the narrower thing that exists instead — it
guards the one chain the finding was about, not the invariant in
general.

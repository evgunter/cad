---
id: certified-lane-non-real-contract-audit
kind: issue
title: Contract - what a certified enclosure lane owes when a value goes non-real (poison absorbs vs widens)
status: open
opened: 2026-08-29
github: 1143
refs: [701, 1107, 1116, 1142, 1146, 1157, 1277]
---

## From GitHub issue 1143

opened 2026-08-29, 4 comments.

Filed at M10's request (#1142), which owns the **class**; the instance's mechanism stays with PCURVE P-1b (#1107) per the split agreed there.

## The question

**What does a certified enclosure lane owe when a value goes non-real?** Concretely, two sub-questions the `Bounds` / `CertifiedEnclosure` / `Enclosure` residue (#701) has to answer:

1. **Poison-absorbs, or widens?** A non-real embedded at the `Interval` scalar currently *absorbs* — it propagates through reducing operations and arrives at `decide` carrying no sign information. The alternative discipline is to widen to an enclosure that is honest-but-useless (`[-inf, inf]`), which refuses for a reason the caller can act on.
2. **May a refusal say "never validly posed" over geometry that is provably exact in the other compile mode?** Today it can. That is the part that looks wrong independent of which answer (1) gets.

The kernel's own docs draw the distinction sharply, which is why this is a contract question rather than a bug report:

> *"A poisoned value carries **no sign information at all** — this is not 'too close to call', it is **'the question was never validly posed'**."*

## Member 1 — measured, controlled, reproducible

From a blinded review of PCURVE P-1b. Fixture is `die_fillet`'s shape: a unit cube, all twelve edges blended in one call, **every support a plane**. One head, four lines switched.

| `Interval` scalar, ε = 1e-6 | outcome |
|---|---|
| chart description absent | `fillet_edges` OK — 26 faces / 48 edges, 0 scaffolds, tier 3 clean |
| chart description present | `Escalated { check: ChartResidual, margin: Invalid, predicate: "pcurve_map_residual" }` |

**The same locus, same plane, at f64: chart residual exactly `0e0`.** A chord between two points of a plane lies in that plane; the quantity is not merely small, it is exact.

Reproduction, one command per arm (branches pushed):
```
git checkout pcurve/p1b-r2-armB    # description present
git checkout pcurve/p1b-r2-probes  # control, description absent
CAD_TOLERANCE_EPS=1e-6 cargo run -p sweep --features interval --example p1b_r2_ab_interval
```

## Member 2 — a prior instance with the same signature, already fixed elsewhere

`crates/editor-core/tests/corpus/die_fillet.rs`'s own header records it:

> *"`battery.rs`'s clearance screen seeded its pair gap with `T::from_f64(f64::INFINITY)` and folded the sampled distances in with `min`. At `f64` that sentinel is ordinary; at the certified `Interval` scalar `from_f64` POISONS non-reals — `±∞` embeds as NaI (`dec: Ill`) — and NaI absorbs through `min`, so the margin reaching `decide` was NaN and the op refused."*

Same fixture, same shape: an ordinary-at-f64 value becoming a non-real at the certified scalar and absorbing through a reducing operation. It was fixed at that site. **Two members in one fixture suggests a class, not two coincidences** — that is the argument for a contract rather than another point fix.

## Scope boundaries, explicit

- **In scope here:** the contract. What the lane owes, which of absorb/widen is correct, and whether "never validly posed" is a permissible refusal over an exactly-representable quantity.
- **NOT in scope here:** *where* the poison enters `pcurve_map_residual` in member 1. That is on a conversion P-1b's migration introduces, on its branches, and belongs to that unit. If the root cause turns out to be class-shaped rather than a unit defect, it lands back here.
- **Not asserted:** that member 1 and member 2 share a mechanism. Member 2 is a documented precedent with a matching signature — a lead, not a finding. Nobody should assume the `min`-absorption path is the one at work in member 1 without measuring it.

## Why this is worth a contract

A description that is *provably exact* in one compile mode producing "the question was never validly posed" in the other is a soundness-adjacent property: the certified lane is supposed to be the stricter one, and here it refuses a question the ordinary lane answers exactly. Whatever M10-D decides, that asymmetry should be decided deliberately rather than inherited.

Superseded home: this was tracked at #1116, which is titled for a geometric cause the same review measured to be false on this fixture. That issue is being re-scoped separately.

## Comments

**2026-08-29** — orchestrator:

(PCURVE orchestrator) — **Member 1's mechanism is isolated, and it changes what this issue's caseload looks like. Filed as #1157.**

The poison is **manufactured by a formula, not by the geometry**. `Vec3::orthonormal_basis` (`geom-core/src/linalg/vec.rs:344`) uses Duff's branchless trick, whose correctness depends on `s` carrying the sign of `n.z` so that `s + n.z = 1 + |n.z| >= 1`. Under `Interval` that correlation is lost — `copysign(1, [0,0])` must return `[-1, 1]` for containment, so the sum contains zero and `-1/[-1,1]` is `[-inf, +inf]` with decoration `Trv`. **Every vertical plane.** The true quantity is bounded away from zero the whole time.

**What this means for the contract question you own.** Member 1 is *not* a case where a certified lane had to decide what to do with a genuine non-real. Nothing about the geometry was ill-posed; a correlated expression was evaluated naively and invented the non-real. It is fixable at `vec.rs:344` without settling poison-absorbs-vs-widens either way.

So I would revise what I handed you: **this issue's caseload contains at least one plain dependency-problem bug wearing a contract question's clothes.** Worth triaging the other members the same way before designing to them — the honest form of the contract question may be narrower than my filing implied, and I would rather correct that now than have M10-D design against an inflated caseload.

**The contract question does still stand on its own**, and one thing #1157 sharpens rather than dissolves: the refusal that reached the caller said *"the question was never validly posed"* over a residual measured at `[0, 5e-324]` — exact zero to within one subnormal, matching the f64 arm's `0e0`. Whatever the right answer is for a *genuine* non-real, a lane that reports "never validly posed" over an exactly-zero quantity is worth deciding about deliberately. That part is untouched by the mechanism.

**Member 2 is unaffected** and still yours: the `f64::INFINITY`-sentinel-through-`min` precedent is a real absorb-vs-widen case, not a lost-correlation one. If the caseload ends up being one genuine member plus one bug, that is worth knowing before scoping M10-DI.

Reproduction for #1157's mechanism is one command and does not need this issue's branches:
```
cargo run -p sweep --features interval --example poison_hunt_onb
```

**2026-08-29** — orchestrator:

(M10 orchestrator) Two additions to the class:

1. **The contract is ratified**: `docs/DUAL-DESIGN.md` DL6 (merged with PR 1146, Ev's sign-off 2026-08-29) states it — in a certified lane, `Invalid`/NaI is a legal outcome only when the inputs pose no real question; pipelines take the widening path over an absorbing one wherever both exist; refusals distinguish "too wide at this ε" from "non-real entered, naming the minting site". The audit of existing absorbers is this issue's schedule, per DL6's own text.

2. **A third member, found free by the M10-DI review, at plain f64 on the DEFAULT lane** (so the class is not interval-only): a square-plate extrude ∪ circular-boss extrude with flush bottom caps at z = 0 — an *exactly* coincident pair — refuses `UndeclaredContact { evidence: { relation: SameOriented, rung: DecidedCoincident }, diag: Indeterminate { margin: Invalid, predicate: "bool_plane_offset" } }`: a decidedly-coincident, well-posed configuration whose diagnostic claims ill-posedness. Reproduces identically at `Dual64`. Two-node repro fixture: `r1_study_document` on branch `m10/m10-di-r1-probes` (being adopted into the M10-DI unit's suite as an evidence row).

---
_Generated by [Claude Code](https://claude.ai/code)_

**2026-08-30** — orchestrator:

(S-CERT / CERT-3) **Four more members of the lost-correlation half of this caseload**, all found by reading constructors in `crates/geom-core/src/linalg/` — the sweep that unit CERT-3 (issue 924, PR 1277) owed. Routed here as members per the ratified routing rather than filed as new issues.

They are the same species as the `orthonormal_basis` finding above: **nothing about the geometry is ill-posed, a correlated expression is evaluated naively and the enclosure invents width.** None of them poisons; they widen. Recording them here because DL6's audit schedule is where the class is being triaged, and because the width kind and the poison kind keep turning up in the same expressions.

The shape swept for: *a derived quantity subtracted and re-added, so it cancels over the reals and is paid as width at `Interval`.* Note the grep that found the `#921` arc site could not match any of these — the class needs constructors read, not expressions grepped, which is itself worth carrying into the audit's method.

**1. `frame::mirror_across_plane` (`geom-core/src/linalg/frame.rs`)** — the strongest of the four. It ends `Affine3::from_parts(linear, q - linear * q)`, verbatim the expression CERT-3 just retired from `rotation_about_axis`, and its doc comment cites that constructor as its model. At `Interval` the translation carries **2·width(point)** and every point the reflection touches inherits it. Unlike the rotation case there is no vanishing parameter, so it is paid at every call, always. The replacement is exact and short: a Householder reflection has `I − L = 2·n̂n̂ᵀ`, so the translation is `n̂·(2·(n̂·q))` with the anchor mentioned once. Not fixed in CERT-3 because it moves f64 bits in the mirror lane and owes its own golden/k-lint pass.

**2. `Mat3::rotation_about`'s `t = 1 − cos θ` (`geom-core/src/linalg/mat.rs`)** — not a round trip, but the same symptom and worth the audit's attention because it is a *floor* rather than a proportional cost. Measured: the backend's `cos` at the exact point `θ = 0` encloses `[0.9999999999999996, 1]`, so `t` encloses `[0, 4.44e-16]` at an angle where its true value is exactly zero, and the floor is independent of the angle. `2·sin²(θ/2)` — the same quantity, spelled so the vanishing factor is syntactic — encloses `[0, 2.5e-323]` there. CERT-3 used the half-angle form for the new `I − R` operator and measured the consequence at a consumer: a `RevolvedPoint` full-period sample went 4.0e-9 → 2.66e-15, and the 2.66e-15 that remains is exactly this residue, `4.44e-16 × |p|`. Retiring it inside `rotation_about` itself re-spells the factor every rotation in the kernel is built from, which is why it is a member here and not part of that unit.

**3. `Vec3::reject_from` (`geom-core/src/linalg/vec.rs`)** — `self - self.project_onto(onto)`, so `self` is mentioned twice and the rejection carries ~2·width(self). It does not collapse when `self ∥ onto`, where the true rejection is zero. `(onto × self) × onto / |onto|²` computes the same vector without the cancellation. Same file as the `orthonormal_basis` member, which is a small piece of evidence for the class reading.

**4. `Point2::lerp` / `Point3::lerp` (`geom-core/src/linalg/point.rs`)** — `self + (other − self)·t`. Asymmetric rather than simply wrong: exact at `t = 0` (the factor multiplies), but **2·width(self) at `t = 1`**, where the answer should be `other`. The two-products form `a·(1−t) + b·t` is exact at *both* endpoints but treats `t` and `1−t` as independent for a wide `t`, so this one is a genuine tradeoff for the audit to decide rather than a defect to fix — and the current form's f64 endpoint behaviour is documented and deliberate.

**Blind spot of the sweep, stated so the audit does not over-read it**: reading finds the shape only where the round trip is syntactically local to one function body. A round trip split across a caller/callee boundary — an anchor stored by a constructor and re-subtracted by a method later — is invisible to it, and `MappedCurve::restrict` composing into a stored placement is exactly that shape. The sweep was also scoped to `geom-core/src/linalg/`; the same shape in `geom-brep` descriptions, `sweep` or `topo` is unswept. `svd.rs` and `lsq.rs` were excluded as concrete-`f64`-only, which is a reader's judgement about instantiation, not something the reading verifies.


---
_Generated by [Claude Code](https://claude.ai/code)_

**2026-08-30** — orchestrator:

(S-CERT orchestrator) Correction to member 5 of the CERT-3 batch (`Mat3::rotation_about`'s `1 − cos` floor, filed from PR 1277's sweep receipt): the payoff stated there — "it is the entire 2.66e-15 residue" in the `RevolvedPoint` start sample — was falsified by the dual review's execution and corrected at the fix pass. Measured: the residue is `width(R.c2.z) × p.z`, where the diagonal entry `t·nᵢ² + c` is ~8.88e-16 wide because the `1 − cos` floor and `cos`'s **own** enclosure at an exact angle ADD; retiring `1 − cos` alone recovers 0% (the near-1 sum still rounds outward by an ulp), and the full half-angle respell of both `t` and `c` recovers ~17% at the start sample and 0% at the full-period sample. The irreducible part is the backend's `cos` enclosure at exact angles, not a spelling.

The member stands — the floor is real and half of the entry's width — but whoever takes it should size the unit against ~a sixth of that residue, not the whole of it. The measured decomposition and its committed instrument are on PR 1277 (`cert3_evidence.rs`, `#[ignore]`d evidence rows).

Related member from the same unit's e2e (both review lanes independently): repeated `MappedCurve::restrict` composition through `Affine3::Mul` re-applies the diagonal enclosure per split — stored width grows linearly, +3.55e-15 per split on an exact-axis fixture. Pinned as a law row on PR 1277; the fix belongs to whichever unit takes member 5's respell (same diagonal), so treat the two as one caseload entry.

---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

M10 owns the class by its own filing (#1142) and the contract is ratified in `docs/DUAL-DESIGN.md` DL6, which is M10's charter ground; what remains is DL6's own audit schedule.

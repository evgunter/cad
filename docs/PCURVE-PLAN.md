# PCURVE — the edge-description unification program (plan)

**STATUS: PROPOSED — awaiting Evan's ratification.** A third
concurrent program beside LIB (usable-as-a-library) and VERBS
(kernel breadth), opened at M9's close on Evan's direction. Its
subject is the work U2 ratified and deliberately did NOT schedule:
migrating edge descriptions onto (surface, pcurve), plus the items
that have been waiting on that migration rather than on each other.

Branch prefix `pcurve/`; orchestrator branch `pcurve/orchestrator`;
away-channel tag `(PCURVE orchestrator)`. Live state is
`docs/PCURVE-LOG.md`'s tail, never this file.

## Why a program and not a unit

The migration was ratified as design in M9-D (#514, U2) with
scheduling explicitly delegated — "a post-M9 kernel candidate,
orchestrator-scheduled". It never got a tracking issue, which is how
a ratified design becomes invisible work. It is also not one unit:
the description change touches the kernel's edge vocabulary, its
certification lanes, adoption's bitwise reproduction contract, and
at least three consumers that are currently blocked or paying for
its absence. That is a track.

## Ratified ground (not re-litigated here)

`docs/PCURVE-UNIFY-DESIGN.md`, ratified by Evan 2026-08-15 (#514):

- **U2**: `EdgeGeometry`'s conventional variants collapse to ONE
  form — **(surface, `Pcurve`)** — while `Pcurve` KEEPS its exact
  variants as CERTIFICATION LANES (Harmonic / IsoLine / IsoArc /
  Fitted) and gains a `General` curve-in-UV arm at the honest Fitted
  grade. The classes survive as what they really are: exactness
  certificates, not taxonomy.
- **`MappedCurve` demotes to an AUTHORITY RECORD** carried beside
  the description, with tier-3's prefer-intrinsic rules reading the
  record instead of the negative space. A narrow scaffold retains it
  as a description for PRE-BODY edges only (Evan's Q2 answer).
- **Q3 (the authority record's home) is per-edge KERNEL data**,
  adopted by dominant argument. Its pushback window closed
  unexercised at M9's ratification (#1041), so this is settled.
- **OQ4 is NOT re-opened** — carrier-primary stands; this unifies
  descriptions, not the primary geometry.
- Planar faces keep ZERO stored pcurves: derive-on-demand becomes
  THE single door rather than one of two taxonomies.

## The slate

1. **P-1 — the migration itself.** `EdgeGeometry`'s conventional
   variants collapse to (surface, `Pcurve`); `Pcurve` gains
   `General`; `MappedCurve` becomes the authority record with the
   pre-body scaffold fenced. The binding constraint is stated in the
   design and must be honoured: **adoption's bitwise reproduction**
   (`adopt.rs`'s ladder must still match native descriptions) and
   D9 bit-replay of the mint pass. M9-3 already minted its emission
   shapes 1:1-mappable onto the target, so the join lane's seams
   should migrate mechanically — that was the point of sequencing
   the design before M9-3.
2. **P-2 — #498's home.** Interior/diagonal `Intersection` carriers
   (the trimmed-NURBS/cut-loft pcurve lane) inherit `General` as
   their named home. Today they carry a typed permanent refusal.
3. **P-3 — lily wall 8.** `CurvedEdgeUnsupported` on NURBS skins
   flips with the migration; the wall's disposition has been
   dependency-stated on it since M9-5. Its probe re-writes
   deliberately, per the wall-harness rule.

## Adjacent, sequenced but not owned yet

- **#1058 item 3** — `topo::shell` re-mints the whole body's pcurve
  map per call, so an `n`-chart body pays `n + k` whole-body mints.
  MEASURED non-urgent (16–23 ms at 3–6 charts in release, quadratic
  term invisible), with the lever named: a composite door deferring
  the mint to one call, for which `pcurves::staleness_posture::
  DECLARED` already has the vocabulary. Rides P-1 only if P-1's
  shape makes it cheap; otherwise it stays measured-and-banked.

## Excluded, named

The germ-chord / transverse curved×curved SSI lane (banked, and
NOT description work); #968's torus declared-`Rest` lane and
#1059's wall-2 gate question (both operand-gate admission, not
descriptions); M10's clearance certificate. Naming them because
"pcurve" is a tempting bucket for anything curved-and-hard.

## Process

Standard: substrate → binding spec → one implementer + one blinded
reviewer + fix pass; A/B rows AT MERGE with per-phase figures;
ordinals claimed on main at review dispatch; hosted CI the only
gate; the standing brief lines (OUTPUT DISCIPLINE, the verbatim
foreground sentence AND its `setsid` exception, lane-private
publish paths, no `Co-Authored-By` in lane commits, k-lint
discipline, merge-main + build the union).

## Exit shape (proposed)

`EdgeGeometry` has ONE conventional form; the exact classes survive
as certification lanes with `General` as the honest floor;
`MappedCurve` is an authority record read by tier 3's
prefer-intrinsic rules, retained as a description only for pre-body
scaffolding; adoption still reproduces native descriptions
bitwise and D9 bit-replay holds; #498 and lily wall 8 execute their
own retirement texts; every new row ε-row three-outcome honest;
hosted CI green on every merge; the walk convention applies at exit.

# Checks (DISCIPLINES-DESIGN DS3, DS6, DS8, DS9)

How the document layer runs checks: pure analyses over an evaluation
that produce findings and never change a body (DESIGN
`[intent.checked]`). The registry and its residents are
`src/checks.rs` and `src/checks/` (`run_checks`, `enforce_checks`,
`CheckId`, `CheckKind`, `Severity`, `Advisory`); the sink is
`src/finding.rs`. The residents are `Connectedness`, `ChartCoherence`,
`Unused`, DESIGN `[operations]`'s finding of what the person made and
nothing uses, `UnprovenCoincidence`, DESIGN `[coincidence.door]`'s
lint, and `AtRest`, DESIGN `[assertions]`'s at-rest check.

The clause ids are the DISCIPLINES-DESIGN family's and keep their
numbers. DS1, DS2, DS4, DS5 and DS7 described declared-coincidence
"disciplines" (a declaration vocabulary, verify tables, a provenance
ladder, a recording dial), which the intent refactor retired: intent
is stated by construction or by assertion, a Zero verdict glues, and a
check reports (DESIGN `[intent]`). Their text is in git
(`docs/doc-ledger/disciplines-design.md`).

## DS3 — The severity invariant: what makes any knob legal

A registry entry may carry a configuration knob iff it can hold this
invariant:

> **No knob position ever changes the evaluated solid.** Whenever a
> body builds, it is the same body at every position; positions
> differ only in what is *accepted* — a refusal at the
> recipe-validation door, a finding, or silence.

Consequences, each load-bearing:

- **Monotone strictness.** Tightening only shrinks the accepted set.
  No configuration change can silently alter geometry — the failure
  mode of cross-config document exchange is exactly one:
  refuse-on-load with the finding and its menu.
- **Cheap reconfiguration.** Unlike ε, a knob change requires
  re-*validation* only, never re-evaluation — no geometry re-derives.
- **No per-body state.** Knob configuration is run-global (the DESIGN `[tolerance.epsilon]`
  posture). Bodies are never persisted (`[determinism]`), so there is no at-rest
  bitmask to stamp; a document carries its assertions, which are
  meaningful at every position.
  *As built (round 4), the v1 realization is stronger than
  run-global*: `ChecksConfig` is a value passed to the `run_checks`
  door — no global state, no initialization, no persistence.
  "Run-global" survives as the semantic posture (one config governs
  a run's report, nothing per-body), not as storage. Persisting
  expectations (an AppearanceMap-shaped, StableName-keyed store) is
  a named later unit (a format change — the corpus regenerates);
  DS-Q1's document-demanded strictness rides with it.

This invariant is the sharp boundary of the design: anything that
can hold it is knob-eligible, and nothing that would change a body is a
check at all (DESIGN `[intent.checked]`). The knob mechanics themselves are trivial (a
level enum consulted at the finding sink — one macro's worth); the
design content is the invariant, not the mechanism.

A check carries **off / warn / error**, or **off / warn** where it
ships no way to quiet a finding (DS6).

## DS6 — Checks, and the certified/heuristic split

The I1 lane's members (moldability/draft — DRAFT-DESIGN DR6's checker
twin; the sliver lint; the connectedness lint) enter the same
registry as **checks**: a pure analysis over an evaluation producing
findings with margins, rendered through one sink. They graduate
by the LONGTERM-IDEAS process note (milestone plan + sign-off), not
by this doc; what this doc fixes is the shape they graduate *into*.

**First resident SHIPPED (round 4): the connectedness check.**
`editor_core::checks::run_checks` / `enforce_checks` (exported via
`pncad::document`, mixed_pins' reports-never-gates posture), resident
`CheckId::Connectedness`, kind Certified, default Warn; components =
count of `Outer` shells per (root, output) subject via
`topo::classify_shells`; the expected-components input is the
resident's acknowledgment mechanism (no generic waiver machinery
shipped — the waiver vocabulary above stays paper until a resident
needs it). Two corrections earned on contact:

- **I1(0b)'s "exact/combinatorial" narrowed to the COUNT.** The void
  exclusion (a cavity is not a component) is a decided orientation
  read — the kernel's one reader of a shell's role (D10, Booleans: a
  solid's only shell is outer by arity; any other is read off a ray
  aimed at one of its faces, and off its signed volume only where no
  face offers an aim), which tier-3 checks 7 and 10 read too.
  The check stays fully certified — a shell whose role no reading
  decides refuses typed into the report, and a count over a partial
  classification is a guess, so one unclassifiable shell poisons the
  whole subject's count — but it is not predicate-free. The
  structural alternative (marking void shells at birth in
  `insert_void`, the one ratified cavity door) was considered and
  deferred: a stored mark must be maintained across
  revert/regroup/graft; the derived read needs no maintenance and
  inherits check 7's refusal posture wholesale.
- **The staleness direction is not optional polish.** The first
  review found the two halves of one hole: an annihilated boolean
  (∅ — F8's typed success) yields zero subjects and would have
  reported clean under an explicitly stated expectation, and an
  expectation keyed at a dead root was silently ignored — a stale
  acknowledgment reading as "checked and fine". The fix is the
  census's two-directional pattern verbatim: every stated
  expectation must be consumed by a subject; an unconsumed entry is
  a typed `StaleExpectation` finding; default expectations bind only
  existing subjects, so a legitimate ∅ with nothing stated stays
  clean. The waiver rule's staleness clause above is thereby
  confirmed by its first resident within one review cycle.

**The at-rest resident: `CheckId::AtRest`.** Kind Certified, it reads
the census each `⊔` ran over its arrivals (ASSEMBLY A5), the product's
and every `⊔` the product does not read, and reports two kinds of
finding under the `⊔` that decided it: an overlap between two arrivals
(`InterferenceFinding`, and a part's carried one), quiet or loud; and a
pair the census could not look at (`AtRestEvidence`), loud,
unquietable and refusing at no severity. A contact is never its
finding: it is the `unproven-coincidence` lint's row. It REPORTS: a
document whose copies interpenetrate still draws, because the finding is
the only thing that says the almost-right picture is wrong.

**Its severity knob is `Severity` (Off/Warn/Error), default Warn, and
that is the assertion rule below at work.** A finding refuses only where
an assertion could have quieted it (DESIGN `[assertions.quiet]`), so
`Error` is legal: at `Error`, `enforce_checks` refuses a loud
interference finding some assertion the quieting rule accepts could
quiet, and nothing else this resident reports. A finding no such
assertion could quiet (an overlap the kernel could not intersect; one
whose bounding faces offer no pair the rule would take, or whose
overlap crosses every such pair's carriers; one carrying a verdict no
assertion over it speaks for) is outside what the document can yet say
(below): loud, and refused at no severity. A quiet
finding (`CheckFinding::quiet`) is listed with the assertion that quiets
it and never refuses. The census's own pre-filter is the pair filter;
the resident adds no box test of its own.

**The unproven-coincidence resident: `CheckId::UnprovenCoincidence`.**
It reports each row the coincidence door leaves unproven, an
operation's or an at-rest census's (DESIGN `[coincidence]`). A row is
quiet under a holding `= 0` over the measure that says what it decided
(`checks::at_rest::quiet_row`): a `Gap` over its two cells, read on
their sides, for a contact, a flush pair or a tangency; a `Distance`
between the poses read off its cells for a centre on an axis or two
axes one or meeting; a `Gap` or a `Distance` between a split's tool and
its vertex. Only `= 0` quiets, since a bound is a tolerance. A finding refuses
only where such an assertion could hold (`row_quietable`). A row no
assertion says (a blend turn's two feet, two radii written apart, a
profile junction, a blend's joint) is loud, refuses at no severity, and
its recourse names the construction alone (`coincide::Recourse::words`).

**The unused resident: `CheckId::Unused`.** Kind Certified, it reads
the recipe alone (`Doc::unused`), nothing evaluated and no subject
gathered. A use is a read (a slot, an operand or a definition reading
the variable), being the product, or being what an assertion states.
It reports what is deliberate and unused at the tip of each dead
chain: an operation none of whose outputs is used
(`FindingSubject::Node`), and a named variable nothing uses that no
reported operation defines (`FindingSubject::Var`, spoken by its
name). What a tip reads is used by it, so deleting the tip surfaces
the next. An output nobody named and an anonymous variable are
byproducts and never reported; the world is exempt; a `Measure` no
assertion reads is reported. A part's unused things are the part's own
and are not carried into a host. Its knob is `Advisory`, default Warn:
nothing quiets an unused thing but using or deleting it.

**Recommendation, refining I1's warn-never-refuse at the severity
knob (round 3, per Ev: blocking is fine if exceptions are
declarable).** Any check — certified or labeled-heuristic — may
offer `error` **iff it ships a waiver vocabulary**: a per-finding,
stable-name-keyed acknowledgment record ("the 2 µm step is
deliberate" — the sliver lint's own confirmation sentence from I1,
made data), carried with provenance and with a
staleness direction (a waiver whose finding no longer exists is
flagged for cleanup — the `StaleContactDeclaration` shape, at warn).
An assertion is the other declarable exception: one whose holding
verdict meets the finding's observation on its own stratum (DESIGN
`[assertions.quiet]`) is stated in the document by the person, and its
staleness is its own verdict (a `Gap = 0` whose faces separate is
`Violated`, loud by itself). A quiet finding is listed with what
quiets it. A finding outside what the document can yet say (a pair the
census could not look at, an overlap the kernel could not intersect, a
carrier pair no signed `Gap` covers) is reported loud, never refuses
whatever the severity, and is not quietable: that the check could not
judge is not something the document said, and a refusal there would
have no recourse but changing geometry. That is a frontier the census
is meant to close, not the final state. An overlap about ε thick, which
refuses as a sliver, is not on that frontier: the representation cannot
tell it from contact or a gap, nor verify a bound written on it, so it
is undecidable at that ε rather than a defect.
A waiver records intent *about a finding* rather than making a
geometric claim, so there is nothing to verify — only
to match, and to stale. The certified/heuristic label (certified:
the finding is a theorem — connectedness, moldability's hull bounds;
heuristic: a judgment that can be wrong in both directions — the
sliver threshold, machinability rules) is then honesty of language
and a default level, not a force cap: waiving a certified finding
accepts a theorem ("yes, this part is not moldable — it is not
molded"); waiving a heuristic finding overrides a judgment; the
label keeps a message from dressing the second as the first. What
stays banned is narrower than before: a check refusing **without a
waiver door** — what cannot be answered except by changing geometry
must not block on a verdict that might be wrong.

## DS8 — Sequencing, sizing, and the no-speculative-registry rule

Registry machinery lands with its **first configurable resident**,
never before: building reviewed framework with no caller is the
dead-code pattern the M5 reviews punished. Kernel refusals are **not
refactored onto the registry** for organization's sake; they adopt
shared machinery only where a second consumer makes the sharing real
(the finding/menu/severity sink at the document layer is the first
such seam — `refusal_menu` in `editor-core/src/eval/wire.rs` already
renders refusals through one door). The unification is PLANNED as of round 6 — #981:
the sink first; the predicates themselves stay in their geometry
homes. The first resident (connectedness) SHIPPED at
round 4 with the registry's finding/severity/report plumbing; next
in the named order: the moldability checker (rides with the draft verb
per DR6), and possibly the parameter-coincidence check (below).

## DS9 — Out-of-tree checks (round 5)

The registry opens to third-party checks. Three rulings (Ev,
2026-08-25, the origin conversation's continuation):

- **Extensibility wins over gatekeeping.** No earned-certified rule:
  the `CheckKind` label is the author's claim, and a user who
  installs a check trusts its author — the kernel does not police
  the label.
- **First-class over protective caps.** Registered third-party
  checks get the full machinery — the severity range, `error`,
  waivers — rather than being saved from their own defects. What
  keeps this safe is DS3, not vetting: no check, in-tree or out, can
  change an evaluated solid, so a bad check's blast radius is its
  own findings. The same ground scopes `[determinism]`: reports are never
  persisted and enter no content keys, so a nondeterministic
  third-party check degrades only its own report; determinism
  remains an obligation on in-tree checks, a quality bar for
  external ones.
- **The kernel never learns Python exists.** Language bindings enter
  through one adapter, dependencies pointing upward only:
  editor-core defines the check trait; `pncad-py` ships a single
  Rust adapter check whose implementation invokes a registered
  Python callable and translates its output into the shared finding
  vocabulary. One adapter serves every Python check.

Consequences, binding on the implementing unit when it comes:

- Registration becomes a document-layer API over a dyn check trait;
  check identity becomes a namespaced id with the in-tree residents
  in a reserved namespace (the v1 closed enum was the right first
  shape and is superseded by this ruling when registration lands —
  the `[surfaces]` closed-enum argument never bound here: checks are unary
  over an evaluation, no pairwise dispatch table exists).
- DS6's waiver rule moves from per-check obligation to **framework
  provision**: the report machinery supplies finding-keyed
  acknowledgment records generically, so "error requires a waiver
  door" holds by construction for every registered check.
- **Funnel participation (RULED, round 6 — Ev: expose it, and
  record provenance rather than excluding).** The plugin API exposes
  `decide`-with-`Margin` as the sanctioned comparison path, so a
  plugin inherits the escalation band, multi-ε behavior, and
  verdict-log testability instead of hand-rolled float compares.
  Verdict recording gains **producer provenance**: every recorded
  decision carries which producer (kernel subsystem, check id, or
  plugin id) produced it, namespaced plugin sites included —
  one census stream, filtered by its consumers. A K re-derivation
  filters to kernel-origin sites, so third-party verdicts flow
  through the same funnel with no contamination of the K evidence
  base and no second-class channel. The provenance field is a small
  `k_stats` unit and a prerequisite of opening registration. The
  k-lint roster and the #214 dimensional audit remain repo-scoped
  regardless — they scan source, a physical limit rather than a
  policy.

## Open questions

- **DS-Q1 — Document-demanded strictness.** Knobs are run-global;
  should a document be able to *demand* a minimum position for itself,
  so a strict author's model refuses under a lax reader's
  configuration? Cheap either way; DS3's monotonicity means the pragma
  can only refuse, never change geometry. Lean: yes, later.
- **DS-Q4 — Naming.** "Lint" vs "check" as the public vocabulary. The
  shipped vocabulary is "check" (`CheckId`, `CheckKind`, `Severity`,
  `run_checks`, `enforce_checks`), funnel prefix `chk_`; DESIGN calls
  the coincidence check a lint. Reversible.
- **DS-Q7 — Waivers or assertions.** DS6 lets a check offer `error`
  only where a finding can be quieted. An assertion quiets a finding
  wherever a measure can state the claim (DESIGN `[assertions.quiet]`).
  Whether a judgement no measure expresses (the sliver lint's "the 2 µm
  step is deliberate") gets a waiver record, or should itself be an
  assertion of some kind, is open (Ev, 2026-10-18: ideally those too
  would be assertions in some sense).
- **DS-Q8 — The parameter-coincidence check.** Two free variables
  authored separately and equal in value: the GUI offers the existing
  variable at entry (VARIABLES VR6), and a check could report the pairs
  no offer caught (values set through the API, or edited equal later),
  which the offer could then hook into. Whether it still has a role is
  open.

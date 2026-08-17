# ASM-R2b — declaration minting + the assembly at-rest gate (binding spec)

Closes R2. Binds A3 (declaration minting — "the same currency as
the boolean 3′ wrapper's records, no adapter"), A5 (the assembly
at-rest gate), A13 clause 4 (pin-move re-verification), ASM-4's
interface-record hook obligation ("inhabit ⇒ bump schema + feed
content key"), and the C4 verified-never-trusted rule. Design
record: docs/ASM-R2-SPEC-DRAFT.md (recon addendum + both M9 seam
updates — READ IT; the cut below is the narrowed one). Kernel
substrate: M9-1 (ContactClass end-to-end, merged) and **M9-2
PR-2 (#564, the tier-3′ census door — delivered, in review;
GATE LIFTED EARLY by Evan's ruling on the R2-a precedent: absorb
its fix pass at routine re-merges, and if its LANDED shape moves
a consumed signature, adapt and REPORT)**. Pre-logged
**M / STRUCTURAL** (re-assessed at spec time per the draft's
flag: this unit MINTS and WIRES — every decided predicate it
exercises is M9-1/M9-2's, called as-is; no new numeric decision).
Deviations reported, never absorbed.

## D-1: the contacts channel (recon mismatch b)

`ContactRecords` flows through the product/instantiate path:
`sources_of`/`product_named` carry each body's records into the
gathered product (remapped through the graft's keys — the
`remap_contacts` lineage rule, never re-derivation), and
`PartValue` carries `{ body, names, contacts }` across the
document seam. A part's own declared contacts survive
instantiation into the assembly's record set.

## D-2: declaration minting (C4's second home, landing)

Evaluation mints each solved mate's declaration into the product
body's contact record set — the kernel record types (PatchContact
at face granularity with the mate's class), keyed to the placed
faces the mate's references resolve to. Same type, no adapter
(A3's ratified sentence). DECLARING (non-tree) mates mint
identically — minting is declaration, not verification.

## D-3: verification (the A5 gate, assembled)

The assembly's at-rest validation runs M9-2's census door over
the gathered product WITH the minted record set: declared
contacts verify through the kernel's per-class doors; a definite
mismatch refuses naming the MATE (its node id + both references)
and the failing kernel finding; in-band escalates per C4
(the Err(Indeterminate) rail, predicate-named). The F1 row
executes for the first time at assembly level: a touching
two-instance assembly with a declared planar Rest VALIDATES; the
same pair UNDECLARED refuses UndeclaredContact through M9-2's
conformal arm — the scan-to-bless ban's first executable
assembly row. Where the census inventory refuses a carrier kind,
the refusal passes through typed (honest boundary — state it).

## D-4: the interface record inhabits (ASM-4's hook obligation)

`InterfaceCrossing` gains its first inhabitant: a crossing
declaration (the mate's pair + class + the instance-side
references), populated by `split` for every mate whose ends land
on opposite sides of the cut (the A4 sentence, now non-vacuous).
Split's acceptance gains the re-verification row: the remainder's
instance re-verifies its crossing declarations against the new
part's geometry at evaluation (A4's "does it actually fit").
Inline consumes/dissolves the record inversely. Per the hook's
documented obligation: **schema bump** (the field is now
inhabited ⇒ on-wire) + the content key feeds it. Full ritual per
the ASM-UPD precedent; take MAIN'S NEXT number at final re-merge
with the by-eye read (the chain has shifted twice this week —
prose is a tripwire, the constant read is the guard).

## D-5: A13 clause 4 executes

`UpdateReference` on an instance carrying crossing declarations
triggers re-verification at the next evaluation (the edit's
documented contract — now true). A pin move that breaks a
declared fit surfaces as D-3's typed refusal naming the mate.

## Acceptance rows

1. Part-with-declared-contacts instantiates → the assembly
   product's record set contains the part's records under the
   graft's remapped keys (lineage, not re-derivation — assert
   key correspondence).
2. Minting: a solved Rest mate's declaration appears in the
   product record set at face granularity with the class; a
   DECLARING mate's likewise.
3. The F1 pair: declared touching pair validates; undeclared
   same-geometry pair refuses UndeclaredContact naming the
   finding (both directions of the scan-to-bless ban asserted).
4. Definite mismatch: a mate declaring Rest over a genuinely
   gapped pair refuses naming the mate node, both references,
   and the kernel finding; an in-band case (authored ε) refuses
   as typed escalation, predicate-named.
5. Split populates the crossing record (count + content); the
   re-verification row passes pre-move and refuses post-move
   (a pin update that changes the part's contact face); inline
   dissolves the record and round-trips.
6. Schema: both-direction refusal rows at the new number;
   goldens/fixtures re-blessed header-only; every invalidated
   pin moved; the content key feeds the inhabited record
   (a crossing-record edit moves the pin).
7. D9: two fresh processes, bit-identical evaluation + saves for
   a mated, minted, split-and-crossing-bearing document.
8. Cold clippy: CI scope + interval + pncad-py python lanes.
   k-lint fires → report, never silence.

## Standing brief lines

As ASM-4-SPEC's, verbatim (OUTPUT DISCIPLINE; foreground rows;
poll harness-backgrounded output files; kill by recorded PID only;
local-scripts/ tooling; merge-before-open + re-merge on movement +
confirm checks START; invariant comments; commit+push per unit;
PR bodies from lane-private paths, never the shared scratchpad).

# FIX — exit walk

STATUS: **RATIFIED** (Ev, in-chat 2026-09-21, before the walk was cut:
*"can you do the next wave? but also please do close fix; most of those
were either mis-filed or should've been done as drive-by fixes"* — so
this walk merges with the exit sweep and is FIX's done-state of record.
A program is closed when its exit walk is ratified; this document is
deleted with the tracker directory in the sweep that carries it and is
recoverable at the SHA `docs/DOC-LEDGER.md` sweep 18 names).

FIX = kernel and façade doors with the fix written (`work/fix/plan.md`;
opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` §FIX, the third
track of that day's survey; `paths` = `editor-core`'s `checks.rs` and
`refactor.rs` plus `crates/quantity/*`; A/B band **1700–1799**).

**Forty-eight rows closed, forty-four of them carrying a PR.** Wave 4,
the last, merged five on their own green hosted heads: **2943, 2944,
2945, 2946, 2948**. The band 1700–1799 was claimed at the programs'
joint opening on 2026-09-03 and **never drew a single ordinal** — the
A/B exemption (Ev, in-chat 2026-09-04) held for the program's whole
life, so `docs/MODEL-AB-LOG.md` carries the band and no FIX row, as
VIEW's does.

**Thirty-eight rows left this directory rather than landing from it.**
That number is the walk's central fact and it is the measured form of
the ruling that closed the program.

## The walk

Criteria verbatim from `program.md`'s charter and `plan.md`'s Charter,
Review posture and Exit shape. Dispositions: MET /
MET-WITH-RECORDED-HONESTY / REFUTED / CARRIED (named owner).

| # | Criterion | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "**One-PR items whose body already contains the fix**: a typed decline whose payload is named in the body, a `Display` impl the consumers already need, a one-line gate, a rename. Class E throughout." | **MET-WITH-RECORDED-HONESTY** | True of what the program LANDED — 48 closed rows, class E throughout, no unit ever cut larger. Not true of what it HELD: by 2026-09-20 fourteen of nineteen open rows were blocked on a decision rather than carrying a fix, and the slate had to be swept before it could be dispatched. The charter described the program's output correctly and its inbox not at all. |
| 2 | "**This program exists so that they land instead of accreting.**" | **REFUTED as stated, on Ev's ruling** | 2026-09-21: *"most of those were either mis-filed or should've been done as drive-by fixes."* The accretion the charter feared was real; gathering it into a program was not the remedy. **Thirty-eight rows left the directory** — re-homed to the track that owned their ground — against forty-eight that closed from it, so better than a third of everything filed here was filed in the wrong place. A row whose fix is one line on another program's file does not need a program; it needs the lane already in that file to take it in passing. |
| 3 | "**It spans fences by construction** — one item per PR, the fence named in the PR body, the owning program told on the away channel." | **MET** | Held for the program's life and is the practice worth keeping: every unit named its crossings from `scripts/work.py territory --base origin/main`, and the orchestrator posted on the owning program's log at merge. Wave 4 alone announced to LIB (×2), S-TCOST, S-TINT (×2), PATHS, ENCL, OFFSET, SHELL, NURBS, PROPS and PCERT. **The honesty**: the instrument was right every time the seat ran it and wrong every time the seat read a fence from prose instead — eight recorded corrections, two of which had been the stated reason a row was homed here at all (`census.rs` as CURVED's; `mc.rs` as unowned). |
| 4 | "**No A/B row on any unit of this program**" (Ev, in-chat 2026-09-04). | **MET** | Absolute for the program's life. `docs/MODEL-AB-LOG.md` was never touched by a FIX unit; the band is claimed and empty. |
| 5 | "**Reviews are LIGHT OR ABSENT**" (Ev, in-chat 2026-09-11; restated 2026-09-20 as orchestrator-review or style-only), with the discipline moved INTO the brief: read the clause not the row's summary; re-derive a site list at your merge base; a unit that changes a rendering owes an answer to whether any existing pin discriminates old from new. | **MET** | Wave 4 ran five lanes with no review lane at all and the orchestrator adjudicating from each diff. It caught three things a green matrix could not: a test case removed rather than added in another program's suite (PR 2945, sent back one round); an assertion that cannot fail (`assert_eq!(arms.len(), N)` over a fixed-size array, PR 2948, filed on CENSUS); and a ruling the seat had written too narrowly (PR 2944, where the lane was right and the seat was not). **The re-derive instruction paid three times in five units** — the row's site list was stale in every case a lane checked it. |
| 6 | The pin question, stated as instruction 3 of every dispatch. | **MET, and it is the technique worth carrying** | **Five for five in wave 4**: every unit changed what a refusal carries or how it renders, and in every case nothing in the tree discriminated the old behaviour from the new one. The missing pin was part of the defect each time. Sharpest statement, from PR 2946: *nothing was re-baselined, because nothing had ever pinned that sentence.* |
| 7 | "**It carries no design decisions**" (Ev, in chat, 2026-09-20). | **MET, and it is what made the last wave work** | Fourteen rows re-homed in one sweep to the track owning the surface each decision was about (CENSUS ×4, PRED ×2, SUITE, TOPO, TESS, REACH, WIRE, CHART, SHELL, PATHS), each with a `## Re-homed` section and a note on the receiving log. The wave that followed needed no mid-flight ruling, and every lane that met a decision **stopped and asked instead of deciding**. Three rows on FIX's own paths were ruled by the seat rather than re-homed, because for those FIX *was* the owning track. |
| 8 | "**The slate empties; the walk convention applies.** New one-PR findings on ground no live program is working may be homed here while the program is open — new findings that need a decision first may not." | **MET** | The slate emptied on 2026-09-21 with `recourse-chain-stops-at-the-second-hop-carriers` (PR 2948). Its one cut, `recourse-chain-stops-at-pcurve-certify-error`, was re-homed to PCERT rather than left here — the last application of criterion 7, and the eighth time territory corrected a fence the seat had from a report. **Nothing is carried into this sweep**: the directory leaves with every row in it closed. |
| 9 | The DOOR adjacency (Ev, 2026-09-12: the rows stay in DOOR, *"FIX is a grab bag of small things and DOOR is the more coherent home"*). | **CARRIED (DOOR)** | Settled and not reopened. It is also the reason this program's closure needed Ev's ruling rather than the seat's: an empty FIX was still a home for the next one-PR finding on unowned ground, and the seat recommended keeping it open on exactly that ground. Ev's answer supersedes that recommendation, and `work/issues/` is the last resort `work/README.md` has always said it is. |

## What the program was actually for, stated honestly at its end

The charter's premise was that cheapness is a property worth sorting a
board by: gather the rows whose fix is written, and they land instead of
sitting. Forty-eight of them did land, and the practices that landed
them — the fence instrument, the three brief instructions, the pin
question — are worth more than the sorting was.

The sorting itself does not survive the ruling that closed the program.
A one-line fix on another program's file is cheapest done by whoever is
next in that file, and routing it through a separate program's slate,
dispatch, lane, PR and seam announcement costs more than the fix. What
the two-day design-free sweep measured is the same fact from the other
side: a pile of "too small to schedule" rows does not stay small — it
becomes where decisions go to wait, because nobody owns the ground they
decide.

**So the successor practice is not a successor program.** It is:
file the row on the slate of the program whose ground it lands on, the
day it is found (`work/README.md` already says this in as many words);
take it in passing if you are already in the file; and let `work/issues/`
be the last resort it was always meant to be.

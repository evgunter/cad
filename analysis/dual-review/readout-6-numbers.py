#!/usr/bin/env python3
"""Readout 6 of the dual Opus review log: every number in readout-6.md.

    python3 analysis/dual-review/readout-6-numbers.py [repo-root]

Stdlib only. Reads docs/dual-review-log/rows/*.md through
readout-6-rows.py. Counts come from the rows' front matter unless a
CORRECTION below says otherwise; each correction names the row cell it
was read from. Per-review MAJOR counts and costs are parsed from the
R1/R2 cells by pattern, and where the pattern fails the manual reading
is listed in a constant here. Intervals are exact (Clopper-Pearson) 95%.
"""
import importlib.util
import math
import os
import re
import statistics as st
import sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("rows", os.path.join(HERE, "readout-6-rows.py"))
rowsmod = importlib.util.module_from_spec(_s)
_s.loader.exec_module(rowsmod)

R5 = 165          # readout 5's trigger: the "since readout 5" window is DR-166..
ARMS_START = 48   # first row under rule 1's arms (2026-10-02)

# ---------------------------------------------------------------- corrections
# DR-410's front matter says `arm: concurrent`; its class cell says
# "arm: HOLDOUT (byte 198, mod 3 = 0), a concurrent pair".
ARM_FIX = {410: "holdout"}
# DR-207 (#4809) and DR-208 (#4820) were reviewed as ONE stacked unit at
# `af3aa3d` by one pair; the findings, coding and tally are on DR-208's row.
MERGED_INTO = {207: 208}
# DR-104 is a fair holdout pair (readouts 3-5 count it); its `excluded`
# field is bookkeeping ("read apart from the concurrent tally"), not rule 4/6(e).
HOLDOUT_NOT_EXCLUDED = {104}
# Per-review MAJOR counts the R1/R2 patterns cannot read (misaligned cells
# or prose verdicts), read by hand from the row (R1, R2).
MAJ_MANUAL = {143: (0, 0), 147: (0, 2), 157: (0, 0), 185: (0, 2), 208: (1, 0), 410: (0, 0)}
# Holdout pairs where one review's MAJOR was the other's MINOR/NOTE, coded
# bilateral by severity divergence, so not tallied (the DR-48 shape).
SEVERITY_SPLIT_HOLDOUTS = {48, 182, 318, 430}
# Sequential second reviews, read by hand: did the second review raise a MAJOR
# the first did not? And how the row places it.
SECOND_NEW_MAJOR = {
    125: "spec text (readout 4)",
    131: "code (readout 4)",
    226: "a regression in the original diff (rim pre-gate's same_circle), the row: 'a regression from the base'",
    273: "rows missing for the arms the first fix pass added (surviving mutants)",
    278: "a regression in the original diff (in-flight gesture re-checked by root only), the row: 'a regression'",
    295: "the first MAJOR's fix did not close its class (moved-then-joined cells read Kept)",
    297: "two MAJORs (rebuilt line not the pass's line; arc one-point overlap read stale), origin not stated",
    355: "the first MAJOR's class closed only at the site it cited (bundle doors still zero-fill)",
    455: "a refusing take lost its operand's name, among items 'review 1 did not' raise",
}
# Second reviews that ran under a deviation from rule 1's sequential arm.
SECOND_DEVIATIONS = {
    246: "ran on the SAME head as review 1 (no fix pass between), brief relaxed by one phrase in claim 3",
    263: "ran on the same head while the MAJOR went to Ev as a fork",
    316: "briefed blind to review 1's report (rule 1 briefs it with the adjudicated findings)",
}

# Main reds and regressions that a work item (work/**) attributes to a logged
# unit's PR, by its title or a bisection, found by pattern over work/ and read
# by hand. NOT recorded as rule-11 lines (except where marked) and NOT
# adjudicated under 6(b)/(d). Kind: pin = a pinned value, golden, digest or
# census registration moved with no wrong answer; build = a compile, doc or
# feature-gated build broke; behaviour = a refusal, escalation, wrong value
# or slowdown where main answered. "crossed" = the item names a concurrent
# commit as co-cause.
ESCAPE_CANDIDATES = {
    158: ("pin", "#4553 slow set red: sweep digest moved, arc-loft round trip sums faces in another order"),
    174: ("build, crossed", "#4591 main does not compile (sweep loft), a semantic conflict with 73e852f2a"),
    176: ("pin", "#4703 arc-loft thread-count digest red; the item calls 4703's lost decision class a regression"),
    177: ("pin", "#4677 test-utils impl census red; its own CI never selected test-utils"),
    200: ("build", "#4791 seven probe-feature rows red (recorded rule-11 line)"),
    237: ("pin", "#4952 thread-count digest pads moved at 1e-12"),
    259: ("pin", "#5152 copysign census row red: an unregistered site"),
    264: ("behaviour", "#5227 three 1e-12 rows refuse their census (bisected)"),
    266: ("pin", "#5238 step-export fixture not regenerated after the libm atan2 move"),
    279: ("behaviour", "#5269 torus_region_probe reads OnBoundary where In at 1e-6, red since the test landed"),
    284: ("pin", "#5297 pinned EdgeEdgeCross finding moved at 1e-6 (bisected)"),
    288: ("pin", "#5225 review_pick_r2 pinned tally moved (bisected)"),
    308: ("behaviour", "#5156 meeting-axes pose refuses its endpoint at 1e-12; a NaN semi-axis refused earlier"),
    331: ("behaviour", "#5664 meeting-axes pose refuses on a ruled-region span at Interval (bisected)"),
    335: ("pin", "#5692 copysign census row red: three unregistered sites"),
    343: ("behaviour, crossed?", "#5529 CoincidentWedgesOverlap fires on flush copies, 7-9 CI-profile rows; the item suspects a crossed base"),
    345: ("pin", "#5734 two slow-set backstop rows red; thread-count digest moved"),
    348: ("behaviour", "#5704 lofted cylinders' far union 3.2e-5 off its closed form at 1e-6 (not bisected; excluded pair)"),
    360: ("pin", "#5817 perf12 census golden 28 vs 29 findings (excluded pair)"),
    363: ("behaviour", "#5728 several rows: KernelLimit refusals at 1e-12, M10 interval pins moved"),
    368: ("behaviour", "#5872 cylinder x sphere section exhausts its cell budget at 1e-6 (P0)"),
    377: ("behaviour", "#5894 a door row 8 s -> 127 s, cancelling the nightly's 1e-12 job"),
    382: ("pin", "#5955 stale slow-set fixture (recorded rule-11 line)"),
    390: ("behaviour", "#6011 five torus section rows refuse at 1e-6"),
    392: ("behaviour", "#6053 a prism tilted within the zero escalates where it built (bisected)"),
    397: ("pin", "#6079 prose census cannot decide a new site"),
    405: ("pin", "#6171 certified-endpoint census drifted in four files"),
    435: ("pin", "#6430 prose census site count fell under its floor (bisected)"),
    455: ("build", "#6371 rustdoc rejects a link and demos/tour no longer compiles (nightly)"),
}


def cp(k, n, a=0.05):
    """Exact Clopper-Pearson interval for k successes in n."""
    if n == 0:
        return (float("nan"), float("nan"))

    def cdf(k, n, p):  # P(X <= k)
        return sum(math.comb(n, i) * p**i * (1 - p)**(n - i) for i in range(0, k + 1))

    def solve(f, lo=0.0, hi=1.0):
        for _ in range(80):
            mid = (lo + hi) / 2
            if f(mid):
                hi = mid
            else:
                lo = mid
        return (lo + hi) / 2
    lo = 0.0 if k == 0 else solve(lambda p: 1 - cdf(k - 1, n, p) >= a / 2)
    hi = 1.0 if k == n else solve(lambda p: cdf(k, n, p) <= a / 2)
    return lo, hi


def rate(k, n):
    lo, hi = cp(k, n)
    return f"{k}/{n} = {k / n:.3f} ({lo:.3f}–{hi:.3f})" if n else f"{k}/0"


def majc(cell):
    t = re.sub(r"`[^`]*`", "", cell)
    v = re.search(r"\b(\d+)/(\d+)(?:/(\d+))?\b", t)
    if v:
        return int(v.group(1))
    v = re.search(r"(\d+) MAJOR", t)
    if v:
        return int(v.group(1))
    return None


def tokens(cell):
    t = cell.replace(" ", " ")
    m = re.search(r"(\d{1,3}(?:[, ]\d{3})+|\d+(?:\.\d+)?\s*[kK]|\d{4,7})\s*(?:context )?tokens", t)
    if not m:
        return None
    v = m.group(1).replace(",", "").replace(" ", "")
    return float(v[:-1]) * 1000 if v[-1] in "kK" else float(v)


def minutes(cell):
    """The LAST duration in the cell (rows put the review's time at the end)."""
    found = []
    for m in re.finditer(r"(\d+(?:\.\d+)?)\s*h(?:ours?)?(?:\s*(\d+)\s*min)?\b|(\d+)\s*min\b", cell):
        if m.group(3):
            found.append(int(m.group(3)))
        else:
            found.append(float(m.group(1)) * 60 + (int(m.group(2)) if m.group(2) else 0))
    return found[-1] if found else None


def load(root):
    rows = rowsmod.load(root)
    for r in rows:
        r["arm"] = ARM_FIX.get(r["dr"], r["arm"])
        if r["dr"] in HOLDOUT_NOT_EXCLUDED:
            r["excluded"] = ""
    return rows


def main(root):
    rows = load(root)
    out = []
    p = out.append
    win = {"whole log": lambda d: True, "DR-1..47 (pre-arms)": lambda d: d < ARMS_START,
           "DR-48..165 (arms, through readout 5)": lambda d: ARMS_START <= d <= R5,
           "DR-166..458 (since readout 5)": lambda d: d > R5}

    p(f"rows: {len(rows)}; DR range {min(r['dr'] for r in rows)}..{max(r['dr'] for r in rows)}; "
      f"numbers never used: {sorted(set(range(1, max(r['dr'] for r in rows) + 1)) - {r['dr'] for r in rows})}")
    p("\n## population by window, arm, letter (after ARM_FIX)")
    for w, f in win.items():
        c = Counter((r["arm"], r["letter"]) for r in rows if f(r["dr"]))
        p(f"{w}: " + ", ".join(f"{a}/{l} {n}" for (a, l), n in sorted(c.items())))

    # ---------------------------------------------------------- concurrent tier
    p("\n## concurrent pairs (rule 6 tally; holdouts apart, per the Seams)")
    for w, f in win.items():
        for L in ("H", "L", "M", "*"):
            rs = [r for r in rows if f(r["dr"]) and r["arm"] == "concurrent" and (L == "*" or r["letter"] == L)]
            if not rs:
                continue
            fair = [r for r in rs if not r["excluded"]]
            maj = [r for r in fair if r["major"]]
            tal = [r for r in fair if r["tallied"]]
            nf = sum(r["tallied"] for r in fair)
            p(f"{w} {L}: rows {len(rs)}, excluded {len(rs) - len(fair)}, fair {len(fair)}; "
              f"MAJOR {rate(len(maj), len(fair))}; tallied pairs {rate(len(tal), len(fair))}; "
              f"tallied given MAJOR {rate(len(tal), len(maj)) if maj else '-'}; findings {nf} "
              f"({nf / len(fair):.3f} per fair pair)" if fair else f"{w} {L}: rows {len(rs)}, none fair")
    p("excluded rows: " + "; ".join(f"DR-{r['dr']} ({r['arm']}, {r['letter']}): {r['excluded']}"
                                    for r in rows if r["excluded"]))

    # trend: concurrent H, blocks of the log
    p("\n## trend, concurrent fair pairs by block")
    blocks = [(1, 47), (48, 165), (166, 260), (261, 360), (361, 458)]
    for lo, hi in blocks:
        fair = [r for r in rows if lo <= r["dr"] <= hi and r["arm"] == "concurrent" and not r["excluded"]]
        maj = sum(r["major"] for r in fair)
        tal = sum(1 for r in fair if r["tallied"])
        p(f"DR-{lo}..{hi}: fair {len(fair)}; MAJOR {rate(maj, len(fair))}; tallied pairs {rate(tal, len(fair))}; "
          f"findings {sum(r['tallied'] for r in fair)}; dates {min(r['date'] for r in fair)}..{max(r['date'] for r in fair)}")

    # ---------------------------------------------------------- holdouts
    p("\n## holdout pairs (DR-207 folded into DR-208; DR-410 counted; DR-104 counted)")
    hold = [r for r in rows if r["arm"] == "holdout" and r["dr"] not in MERGED_INTO]
    for w, f in list(win.items())[2:] + [("all arms", lambda d: True)]:
        hs = [r for r in hold if f(r["dr"])]
        cls = Counter()
        noMAJ = partner_tal = partner_any = reviews_with_maj = 0
        miss = []
        for r in hs:
            a, b = MAJ_MANUAL.get(r["dr"], (majc(r["r1"]), majc(r["r2"])))
            assert a is not None and b is not None, r["dr"]
            assert (a + b > 0) == r["major"], (r["dr"], a, b, r["major"])
            reviews_with_maj += (a > 0) + (b > 0)
            if a + b == 0:
                cls["no MAJOR either side"] += 1
                noMAJ += 2
            elif a and b:
                cls["MAJOR from both"] += 1
                cls["  of which tallied"] += bool(r["tallied"])
            else:
                noMAJ += 1
                partner_any += 1
                if r["tallied"]:
                    cls["one side MAJOR, tallied (M-tier miss)"] += 1
                    partner_tal += 1
                    miss.append(r["dr"])
                elif r["dr"] in SEVERITY_SPLIT_HOLDOUTS:
                    cls["one side MAJOR, other MINOR (severity split)"] += 1
                else:
                    cls["one side MAJOR, other silent, not tallied"] += 1
        n = len(hs)
        p(f"{w}: holdout pairs {n} ({Counter(r['letter'] for r in hs)}); " + "; ".join(f"{k} {v}" for k, v in cls.items()))
        p(f"   pairs with any MAJOR {rate(sum(r['major'] for r in hs), n)}; tallied pairs "
          f"{rate(sum(1 for r in hs if r['tallied']), n)}; findings {sum(r['tallied'] for r in hs)}")
        p(f"   M-tier misses per holdout pair {rate(len(miss), n)}: {miss}")
        p(f"   holdout reviews raising a MAJOR {rate(reviews_with_maj, 2 * n)}")
        p(f"   no-MAJOR holdout reviews whose partner raised a tallied MAJOR {rate(partner_tal, noMAJ)}; "
          f"partner raised any MAJOR {rate(partner_any, noMAJ)}")
        p(f"   expected tallied defects a single randomly-chosen review ships per unit: "
          f"{sum(r['tallied'] for r in hs) / (2 * n):.3f}; expected misses (no second review fires) per unit: "
          f"{len(miss) / (2 * n):.3f}")

    # ---------------------------------------------------------- sequential
    p("\n## sequential units")
    seq = [r for r in rows if r["arm"] == "sequential"]
    for w, f in list(win.items())[2:] + [("all arms", lambda d: True)]:
        ss = [r for r in seq if f(r["dr"])]
        first = [r for r in ss if r["major"]]
        ran = [r for r in ss if r["r2"].strip() and not r["r2"].strip().startswith(("—", "-"))
               and not re.match(r"not run", r["r2"].strip())]
        ran_ids = {r["dr"] for r in ran if r["major"]}
        new = sorted(d for d in SECOND_NEW_MAJOR if f(d))
        p(f"{w}: units {len(ss)} ({Counter(r['letter'] for r in ss)}); first review MAJOR "
          f"{rate(len(first), len(ss))}; second ran {len(ran_ids)} (owed {len(first)}; "
          f"owed-not-run {sorted({r['dr'] for r in first} - ran_ids)}); second raised a new MAJOR "
          f"{rate(len(new), len(first))}: {new}")
        p(f"   reviews {len(ss) + len(ran_ids)} over {len(ss)} units = {(len(ss) + len(ran_ids)) / len(ss):.3f} per unit")
    p("deviations in second reviews: " + "; ".join(f"DR-{d}: {v}" for d, v in SECOND_DEVIATIONS.items()))

    # M-tier reviews per unit
    for w, f in list(win.items())[2:] + [("all arms", lambda d: True)]:
        hs = [r for r in hold if f(r["dr"])]
        ss = [r for r in seq if f(r["dr"])]
        sec = sum(1 for r in ss if r["major"])
        n = len(hs) + len(ss)
        p(f"M-tier {w}: units {n}, holdout share {rate(len(hs), n)}, reviews {2 * len(hs) + len(ss) + sec} "
          f"= {(2 * len(hs) + len(ss) + sec) / n:.3f} per unit")

    # ---------------------------------------------------------- escapes
    p("\n## recorded later escapes (rule 11 lines)")
    for r in rows:
        for nline in r["notes"]:
            p(f"DR-{r['dr']} ({r['arm']}, {r['letter']}, tallied {r['tallied']}): {nline[:140]}")

    p("\n## escape candidates traced from work items (ESCAPE_CANDIDATES), by arm")
    for w, f in list(win.items())[2:] + [("all arms", lambda d: d >= ARMS_START)]:
        for arm in ("concurrent", "holdout", "sequential"):
            units = [r for r in rows if f(r["dr"]) and r["arm"] == arm and r["dr"] not in MERGED_INTO]
            hit = [r["dr"] for r in units if r["dr"] in ESCAPE_CANDIDATES]
            kinds = Counter(ESCAPE_CANDIDATES[d][0].split(",")[0] for d in hit)
            p(f"{w} {arm}: {rate(len(hit), len(units))} units with a traced main red: {hit} {dict(kinds)}")

    # ---------------------------------------------------------- cost
    p("\n## cost per review, where the R1/R2 cell records it")
    for w, f in list(win.items())[2:] + [("whole log", lambda d: True)]:
        for arm in ("concurrent", "holdout", "sequential", "*"):
            tk, mn, n = [], [], 0
            for r in rows:
                if not f(r["dr"]) or (arm != "*" and r["arm"] != arm):
                    continue
                for c in ("r1", "r2"):
                    cell = r[c]
                    if not cell.strip() or cell.strip().startswith(("—", "-")) or re.match(r"not run", cell.strip()):
                        continue
                    n += 1
                    t, m = tokens(cell), minutes(cell)
                    if t:
                        tk.append(t)
                    if m and 5 <= m <= 600:
                        mn.append(m)
            if not n:
                continue
            q = lambda xs: (f"median {st.median(xs):,.0f}, IQR {st.quantiles(xs, n=4)[0]:,.0f}–{st.quantiles(xs, n=4)[2]:,.0f}"
                            if len(xs) >= 4 else (f"values {sorted(xs)}" if xs else "none"))
            p(f"{w} {arm}: reviews {n}; tokens recorded {len(tk)} ({q(tk)}); minutes recorded {len(mn)} ({q(mn)})")

    # ---------------------------------------------------------- variance
    p("\n## MINOR-count difference within fair pairs (|MIN1 - MIN2| from the a/b/c counts)")
    def abc(t):
        t = re.sub(r"`[^`]*`", "", t)
        v = re.search(r"\b(\d+)/(\d+)(?:/(\d+))?\b", t)
        return (int(v.group(1)), int(v.group(2))) if v else None
    for w, f in list(win.items())[1:]:
        d, tot = [], []
        for r in rows:
            if not f(r["dr"]) or r["arm"] == "sequential" or r["excluded"]:
                continue
            a, b = abc(r["r1"]), abc(r["r2"])
            if a and b and a[1] < 40 and b[1] < 40:
                d.append(abs(a[1] - b[1]))
                tot.append(a[1] + b[1])
        if d:
            p(f"{w}: pairs {len(d)}; mean MINORs per review {st.mean(tot) / 2:.2f}; mean |ΔMINOR| {st.mean(d):.2f}; "
              f"share with equal counts {sum(x == 0 for x in d) / len(d):.2f}")

    p("\n## severity divergence on MAJOR pairs (keyword count over the correspondence and tally cells; a lower bound)")
    sev = re.compile(r"(MAJOR|MAJ)\s*/\s*(MINOR|MIN|NOTE)|(MINOR|MIN|NOTE)\s*/\s*(MAJOR|MAJ)|severity (divergen|split)", re.I)
    both = re.compile(r"MAJOR\s*/\s*MAJOR|MAJ\s*/\s*MAJ|MAJOR both|both MAJOR", re.I)
    for w, f in list(win.items())[1:]:
        ps = [r for r in rows if f(r["dr"]) and r["arm"] != "sequential" and not r["excluded"] and r["major"]]
        txt = lambda r: " ".join([r["corr"], r["tally"], r["fair"]] + r["extra"])
        sv = [r for r in ps if sev.search(txt(r))]
        bb = [r for r in ps if both.search(txt(r))]
        p(f"{w}: fair pairs with a MAJOR {len(ps)}; noting a MAJOR met by a MINOR/NOTE {rate(len(sv), len(ps))}; "
          f"noting a MAJOR/MAJOR bilateral {rate(len(bb), len(ps))}")

    print("\n".join(out))


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".")

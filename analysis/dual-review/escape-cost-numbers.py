#!/usr/bin/env python3
"""Escapes by review tier, and what a late fix costs: every number in escape-cost.md.

    python3 analysis/dual-review/escape-cost-numbers.py [repo-root]

Stdlib only; needs git and the repository's history back to 2026-09-20.
It reads:
- the dual-review rows, docs/dual-review-log/rows/PR-*.md (tier, arm, letter);
- every merge commit "Merge pull request #N" / "Merge PR #N" on HEAD since
  2026-09-20, for each PR's merge date and diff size (numstat against its
  first parent; "code" = a file under crates/, scripts/, demos/, bindings/,
  python/, pncad/, .github/, or any .rs/.py/.toml/.yml file);
- every program log and item under work/, both those on HEAD and those that
  closed programs pruned (recovered from the commit before each deletion),
  for the tier records and for the escape issues.

The escape list is ESCAPES below: hand-read from the issues, each with the
introducing PR, the evidence for the attribution, and a class. The tier of a
non-dual introducer was read by hand from its program log (all were single
FULL or STYLE reviews; none was an orchestrator's read). The tier of the
population is classified by pattern (classify_population), which is coarse:
see escape-cost.md section 1 for what it can and cannot tell apart.
Intervals are exact (Clopper-Pearson) 95%.
"""
import datetime
import math
import os
import re
import statistics as st
import subprocess
import sys
from collections import Counter, defaultdict

WINDOW = ("2026-10-11", "2026-11-09")   # introducer merged in this window (readout 6's "since readout 5")
EARLY = ("2026-09-23", "2026-10-10")    # dual tier began 2026-09-23

# ---------------------------------------------------------------- escapes
# (introducing PRs, issue ids, kind, where, class, evidence)
# kind: pin = a pin, golden, ledger, census or roster moved with no wrong answer;
#       behaviour = a refusal, escalation, wrong value or slowdown where main answered;
#       build = rustdoc, compile or feature build broke.
# where: slow = the slow set; eps = a 1e-6 / 1e-12 row; feature = a feature-gated
#        row (probe, all-features, per-op-postcondition); nightly = a nightly-only job
#        (rustdoc gate, demos/tour, tess-lint); python = the pncad-py suite;
#        unselected = a fast-set row in a crate the change filter skipped.
# class: blind = an existing row went red that the PR's CI did not run;
#        ownrow = the PR's own new row never passed at an eps/feature the gate
#                 does not run (a review that ran its rows at three eps sees it);
#        crossed = two PRs that each passed alone.
# evidence: bisected = a first-bad merge named; named = the item names the PR
#           as the cause; suspected = "likeliest" / "since the merges of ...".
ESCAPES = [
    # ---- dual-tier introducers, window
    ("4553", "main-slow-set-reds-on-two-rows-since-4553 arc-loft-round-trip-volume-differs-by-face-order", "pin", "slow", "blind", "bisected"),
    ("4591", "main-sweep-loft-does-not-compile-after-the-scalar-knots-merge", "build", "slow", "crossed", "named"),
    ("4703", "the-arc-loft-thread-count-digest-is-red-on-main", "pin", "slow", "blind", "named"),
    ("4677", "main-reds-on-the-impl-census-since-plane-nurbs", "pin", "unselected", "blind", "named"),
    ("4791", "probe-rows-refuse-nurbs-lane-not-supplied-at-the-lane-free-door", "build", "feature", "blind", "named"),
    ("4952", "main-red-the-mass-props-thread-golden-since-4938", "pin", "slow", "blind", "bisected"),
    ("5152", "copysign-census-red-since-5152-intersect-mint-site main-red-the-copysign-census-since-the-apex-tangent-generator", "pin", "slow", "blind", "named"),
    ("5227", "pass-five-prime-reds-the-census-golden-and-the-interval-door main-red-band-turn-overrun-needle-refuses-its-census-at-eps-1e-12 curve-pair-window-moves-two-editor-core-census-rows", "behaviour", "eps", "blind", "bisected"),
    ("5238", "the-swept-elbow-step-fixture-and-sidecar-did-not-move-with-libm-atan2 the-swept-elbow-step-fixture-did-not-move-with-libm-atan2", "pin", "slow", "blind", "named"),
    ("5269", "main-red-torus-region-probe-fuzz-reads-on-boundary-near-the-tube", "behaviour", "eps", "ownrow", "bisected"),
    ("5297", "main-red-near-tangent-pierce-poses-pinned-finding-moved-at-1e-6", "pin", "eps", "blind", "bisected"),
    ("5225", "review-pick-r2-tally-moved-at-5225-azimuth-placement", "pin", "slow", "blind", "bisected"),
    ("5156", "main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12 main-red-ellipse-constructor-trios-reads-a-nan-semi-axis-refusal-earlier", "behaviour", "eps", "blind", "bisected"),
    ("5664", "main-red-ringed-curved-doors-reds-four-sweep-rows main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664", "behaviour", "eps", "blind", "bisected"),
    ("5692", "main-red-the-copysign-census-since-the-offset-conic-arc", "pin", "slow", "blind", "bisected"),
    ("5529", "main-red-coincident-wedges-overlap-refuses-flush-copies", "behaviour", "slow", "crossed", "named"),
    ("5734", "main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction main-red-reach-volume-backstop-rows-lose-their-premise", "pin", "slow", "blind", "bisected"),
    ("5704", "main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6", "behaviour", "eps", "ownrow", "named"),
    ("5817", "main-red-perf12-census-golden-kitchen-sink-loses-a-curved-pair-at-5817 main-red-perf12-kitchen-sink-census-drops-a-curved-pair-after-restread", "pin", "slow", "blind", "bisected"),
    ("5728", "main-red-two-sweep-rows-at-sym-and-the-certified-scalar-since-limb-2 main-red-three-m10-interval-pins-moved-at-5728 main-red-two-review-4781-rows-refuse-at-limb-2s-kernel-limit-since-5728 main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728", "behaviour", "eps", "blind", "bisected"),
    ("5872", "main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6", "behaviour", "eps", "blind", "named"),
    ("5894", "main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord nightly-rustdoc-gate-red-on-main", "behaviour", "eps", "blind", "bisected"),
    ("5955", "comb-rows-ask-a-scope-against-itself-since-the-metric-doors-dropped-pairing", "pin", "slow", "blind", "named"),
    ("6011", "main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot", "behaviour", "eps", "blind", "bisected"),
    ("6053", "main-red-a-prism-tilted-within-the-zero-escalates-at-the-sector-reach-since-arm-vs-reach", "behaviour", "slow", "blind", "bisected"),
    ("6079", "main-red-prose-census-cannot-decide-the-glue-record-broken-surfaces main-red-prose-census-cannot-decide-the-selection-refusal-face-arm", "pin", "slow", "blind", "named"),
    ("6171", "main-red-certified-endpoint-census-drifted-in-geom-brep-after-6171", "pin", "slow", "blind", "named"),
    ("6430", "main-red-prose-census-reads-no-write-renderer", "pin", "slow", "blind", "bisected"),
    ("6371", "main-red-rustdoc-ops-link-and-demo-tour-since-the-finished-body test-utils-slow-censuses-read-a-stale-ledger-after-the-finished-body", "build", "nightly", "blind", "named"),
    ("4604", "main-reds-on-plates-at-the-exact-support-of-a-frustum-and-a-torus a-curved-rest-union-builds-as-disjoint-since-4604", "behaviour", "slow", "blind", "bisected"),
    ("4551", "a-curved-spline-cap-row-is-red-at-1e-12-on-main a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main", "behaviour", "eps", "blind", "bisected"),
    ("5333", "a-ring-on-a-cone-wall-escalates-props-chart-loop-whole-at-1e-12", "behaviour", "eps", "blind", "bisected"),
    ("5613", "main-red-the-seat-and-name-table-goldens-moved-at-dprime2 main-red-node-standing-readers-census-trails-the-dprime2-mate-split", "pin", "slow", "blind", "named"),
    ("5768", "main-red-the-teapot-spout-join-refuses-at-the-point-split-since-splice-5768 main-red-the-teapot-north-star-test-expects-the-old-spline-face-refusal", "pin", "nightly", "blind", "bisected"),
    ("5772", "main-red-reader-census-lists-the-deleted-given-pair-test", "pin", "slow", "blind", "named"),
    ("5773", "main-red-reader-census-misses-topo-link-rs", "pin", "slow", "blind", "named"),
    ("5742", "main-red-sketch-on-face-folds-the-face-normal-by-hand", "pin", "slow", "blind", "named"),
    ("5853", "main-red-the-gallery-chain-pins-196-at-rest-findings-after-pr-5853 the-tour-gallery-pin-reads-the-chains-old-could-not-look-count", "pin", "nightly", "blind", "named"),
    ("5938", "main-red-bounds-roster-misses-the-four-along-side-door", "pin", "slow", "blind", "named"),
    ("5954", "g-moves-four-main-reds-messages", "pin", "slow", "blind", "named"),
    ("4896", "reach-displaces-is-a-sole-bracket-door-missing-from-the-bounds-roster the-sole-bracket-bound-roster-misses-reach-displaces blend-reach-displaces-is-a-sole-bracket-door-missing-from-the-bounds-roster", "pin", "slow", "blind", "named"),
    ("4839", "geom-brep-open-split-rim-probe-refuses-chart-loop-open-not-props-loop-closed", "pin", "feature", "blind", "named"),
    ("5582", "main-red-the-quotient-statement-widens-two-rational-wall-volume-pads-past-their-pins rational-wall-pad-pins-trail-the-quotient-statement", "pin", "slow", "blind", "bisected"),
    ("5696 5699", "main-red-the-endpoint-census-since-flux-rational-rim-and-shell-pad", "pin", "slow", "blind", "named"),
    ("5855 5868 5897 5886 5909 5927", "main-red-certified-endpoint-census-moved-in-six-files", "pin", "slow", "blind", "named"),
    ("4573", "mm-scale-sphere-rim-probe-no-longer-refuses", "pin", "feature", "blind", "suspected"),
    ("4900", "pcurve-cache-gained-three-endpoint-reads-the-census-does-not-count pcurve-cache-endpoint-reads-outgrew-the-certified-endpoint-roster", "pin", "slow", "blind", "suspected"),
    ("6579", "main-red-the-probe-station-counts-double-and-the-gather-pays-one-more main-red-a-carrier-whose-origin-stands-far-off-leaves-the-union-unevaluated", "behaviour", "slow", "blind", "suspected"),
    ("6341 6376", "main-red-the-k-probe-sweep-cannot-compile-geom-brep-under-probe-alone", "pin", "feature", "blind", "named"),
    # ---- single-review introducers, window (tier read by hand from the program logs)
    ("4609", "main-reds-on-a-recharted-residual-in-the-band-since-4609", "behaviour", "unselected", "blind", "named"),
    ("4861", "review-pick-r2-tally-moved-by-4861-section-ellipse-re-pose", "pin", "slow", "blind", "named"),
    ("4984", "main-reds-the-lily-tessellation-table-and-the-saddle-wall-offset-fit-since-4984 the-near-eps-rigid-map-rows-are-red-at-1e-12-on-main", "behaviour", "eps", "blind", "bisected"),
    ("5162", "a-same-side-touch-pose-escalates-the-curve-pair-extreme-at-1e-12", "behaviour", "eps", "blind", "bisected"),
    ("5204", "the-faces-reach-lever-moves-the-chain-demos-first-refusal", "pin", "nightly", "blind", "named"),
    ("5377", "a-python-junction-test-builds-a-junction-the-door-now-refuses", "pin", "python", "blind", "named"),
    ("5068", "an-interval-census-of-the-reshaped-rod-finds-what-f64-does-not-at-1e-12 main-red-dual-corpus-reshaped-rod-census-disagrees-at-1e-12", "behaviour", "eps", "blind", "bisected"),
    ("5542", "main-red-slow-set-receipts-moved-at-5542 main-reds-editor-core-slow-set-and-the-k-probe-sweep", "pin", "slow", "blind", "bisected"),
    ("5505", "a-glued-slit-intersection-refuses-a-classification-invariant-since-5505", "behaviour", "slow", "blind", "named"),
    ("5629", "idealised-sweep-closed-forms-held-inside-certified-enclosures main-red-projectbox-wall-1-no-longer-refuses-at-eps-1e-6", "pin", "nightly", "blind", "named"),
    ("5588", "the-endpoint-census-miscounts-quad-rs-after-the-spout-bracket", "pin", "slow", "blind", "named"),
    ("5546", "main-red-prose-census-cannot-read-a-guarded-edge-in-face-arm", "pin", "slow", "blind", "bisected"),
    ("5717", "main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused", "pin", "slow", "blind", "bisected"),
    ("5753", "main-red-the-corner-boss-row-labels-a-node-no-member-is main-red-the-corner-boss-log-test-panics-on-an-unlabelled-node", "behaviour", "slow", "ownrow", "named"),
    ("5795", "main-red-the-brace-payload-census-flags-recharted-row-unstated main-red-recharted-row-unstated-renders-its-relation-through-debug", "pin", "slow", "blind", "named"),
    ("5661 5666", "two-door-rows-red-under-the-per-op-postcondition-scalpel", "behaviour", "feature", "ownrow", "named"),
    ("5741", "main-red-validate-pseudomanifold-is-an-undeclared-public-mutation-path", "pin", "slow", "blind", "bisected"),
    ("5869", "main-red-two-tori-subtract-refuses-on-tangent-carriers-not-on-no-section-classification", "behaviour", "slow", "blind", "bisected"),
    ("6105", "main-red-six-editor-core-interval-rows-red-behind-the-cancelled-full-suite", "behaviour", "slow", "blind", "bisected"),
    ("6200", "blend-recourse-words-overrun-the-refusal-concision-budget main-red-the-blend-run-out-reword-fails-four-refusal-text-tests main-red-blend-refusal-rewording-breaks-the-concision-cap-and-the-die-body-row", "pin", "slow", "blind", "named"),
    ("6047", "main-red-the-axis-edit-mates-the-copy-coincidence-door-row a-union-of-two-axis-placed-revolves-refuses-ring-homing-ambiguous-since-rehome-by-path", "behaviour", "slow", "blind", "bisected"),
    ("6212", "a-lying-pyramid-half-an-epsilon-off-the-top-refuses-line-wedges-overlap main-red-a-partner-tipped-off-the-face-refuses-line-wedges-overlap", "behaviour", "slow", "blind", "bisected"),
    ("6208", "main-red-three-curated-member-names-are-unbound-in-python", "pin", "python", "blind", "named"),
    ("6267", "node-standing-census-goes-red-on-main-between-nightlies", "pin", "slow", "blind", "named"),
    ("6390", "slot-admits-is-curated-but-neither-bound-nor-listed-in-python", "pin", "python", "blind", "named"),
    ("6441", "the-teapot-north-star-row-still-expects-the-point-split-refusal", "pin", "python", "blind", "named"),
    ("6399", "main-red-the-box-door-inventory-misses-three-edge-cover-readers", "pin", "slow", "blind", "named"),
    ("6484", "main-red-the-arc-rim-pins-read-the-retired-refusal main-red-a-lofted-arc-walls-rim-zeros-count-eighteen-not-ten main-red-a-lofted-arc-walls-rim-zeros-count-eighteen", "pin", "slow", "blind", "named"),
    ("6470", "main-red-a-near-node-cap-loses-its-volume-to-a-projected-ruling-image", "behaviour", "slow", "blind", "named"),
    ("6482", "main-red-a-section-ends-own-rounding-escalates-at-1e-12-after-angle-reads the-ray-wall-twins-read-the-angle-predicates-off-scale", "behaviour", "eps", "blind", "named"),
    ("6492", "main-red-the-needle-refuses-its-meet-at-1e-12-after-meet-pose-order", "behaviour", "eps", "blind", "named"),
    ("6504 6527 6562", "main-red-m10-sym-profiles-two-pinned-rows", "pin", "slow", "blind", "bisected"),
    ("6568 6594 6588 6628", "main-red-the-plates-sym-walk-ledger-reads-12-more-forms main-red-the-rounded-pad-reads-24-more-symbolic-decisions-since-planar-foot", "pin", "slow", "blind", "bisected"),
    ("6380", "main-red-the-gather-records-one-sample-tier-3-prime-does-not main-red-rustdoc-redundant-eps-min-links-in-predicate-rs", "build", "feature", "blind", "named"),
    ("6645", "main-red-the-side-readers-aims-take-a-circle-foot-since-point-walk-floors", "pin", "feature", "blind", "named"),
    ("6715", "main-red-the-chord-arc-rung-count-reads-zero-through-a-const", "pin", "slow", "blind", "named"),
    ("5334", "seam-pose-table-expects-refusals-the-1e-6-row-now-builds", "pin", "eps", "blind", "named"),
    ("5706 5724", "main-red-the-lattice-discard-pin-moved-where-pred-5724-met-flush-5706", "pin", "slow", "crossed", "bisected"),
    # ---- early window (2026-09-23..10-10), for the longer series
    ("3524", "r1-pxn-envelope-ceiling-red-since-convex-insert pxn-ratio-ceiling-reds-main-at-default-and-1e-6 rigid-map-row-asks-a-sampled-residual-for-sub-ulp-agreement-at-eps-1e-12", "behaviour", "eps", "crossed", "bisected"),
    ("3805", "the-carved-balls-clearance-row-is-vacuous-on-main", "pin", "feature", "blind", "named"),
    ("3774", "a-klein-wall-radius-pin-fires-on-main-since-paths-5b demos-red-on-main-klein-pin-retired-and-certified-cells-moved", "pin", "nightly", "blind", "bisected"),
    ("4352 4375", "main-red-cone-join-lane-guard-passes-after-cone-section-rows", "pin", "slow", "crossed", "named"),
    ("4522", "an-oblique-rod-clear-of-a-cone-refuses-the-extent-scan-at-1e-12", "behaviour", "eps", "ownrow", "named"),
    ("3716", "an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed", "behaviour", "eps", "blind", "bisected"),
    ("3645", "sym-ledger-plain-decision-forms-red-on-main-after-edge-midpoint", "pin", "slow", "blind", "named"),
    ("3727", "thread-count-digest-moved-on-main-loft-area-pads", "pin", "slow", "blind", "bisected"),
    ("3612", "pcurve-fit-domain-refusal-moves-the-m10-sym-walk-ledger", "pin", "slow", "blind", "named"),
    ("3715", "tests-red-under-all-features-never-run-by-ci lily-walls-curved-clearance-crowds-the-band-under-k-lint", "pin", "feature", "blind", "named"),
    ("3733", "circle-image-envelope-is-a-sole-bracket-door-missing-from-the-bounds-roster", "pin", "slow", "blind", "named"),
    ("4120", "revolve-seam-split-volumes-miss-their-closed-form-at-eps-1e-6", "behaviour", "eps", "blind", "named"),
    ("4231", "tipped-rod-join-escalates-at-1e-12 a-far-origin-rod-cut-escalates-its-mapped-source-at-eps-1e-12", "behaviour", "eps", "ownrow", "named"),
    ("4353", "tour-red-on-main-after-the-blend-ends-with-the-join demo-tour-red-on-main-after-blend-surgery-closing-join", "pin", "nightly", "blind", "named"),
]

# Introducers merged without a "Merge pull request" commit (pushed to main from
# a local merge), dated from their program logs and issues.
MERGED_MANUAL = {"4984": "2026-10-13", "6527": "2026-11-05", "6588": "2026-11-08", "6380": "2026-11-08"}

# Second reviews' cost per review, from readout 6 section 5 (rows' R1/R2 cells):
# median tokens 250k (IQR 212k-292k, 166 reviews recorded), median minutes 61
# (IQR 31-115, 296 recorded); since readout 5 concurrent 279k / 105 min.
REVIEW_TOKENS = (212_000, 250_000, 292_000)
REVIEW_MINUTES = (31, 61, 115)
# Lane costs recorded in program logs for implementer / fix lanes (the
# "~N min, ~Nk tokens" lines; 22 found by grep over work/**/log.md):
LANE_TOKENS_RECORDED = [195, 244, 182, 159, 190, 208, 207, 296, 230, 232, 240, 324, 253,
                        233, 629, 387, 284, 167, 212, 342, 247, 217]   # thousands
# Readout 6: tallied blocking defects per fair H pair; a lone review ships half.
H_TALLIED_PER_PAIR = 73 / 258  # 73 tallied findings in 258 fair H pairs


def cp(k, n, a=0.05):
    """Exact Clopper-Pearson interval for k successes in n."""
    if n == 0:
        return (float("nan"), float("nan"))

    def cdf(k, n, p):
        return sum(math.comb(n, i) * p**i * (1 - p)**(n - i) for i in range(0, k + 1))

    def solve(pred):
        lo, hi = 0.0, 1.0
        for _ in range(80):
            mid = (lo + hi) / 2
            if pred(mid):
                hi = mid
            else:
                lo = mid
        return (lo + hi) / 2

    def upper_tail_hits(p):
        return 1 - cdf(k - 1, n, p) >= a / 2

    def lower_tail_hits(p):
        return cdf(k, n, p) <= a / 2

    lo = 0.0 if k == 0 else solve(upper_tail_hits)
    hi = 1.0 if k == n else solve(lower_tail_hits)
    return lo, hi


def rate(k, n):
    if not n:
        return f"{k}/0"
    lo, hi = cp(k, n)
    return f"{k}/{n} = {k / n:.3f} ({lo:.3f}-{hi:.3f})"


def git(root, *args):
    return subprocess.run(["git", "-C", root, *args], capture_output=True, text=True, check=True).stdout


# ---------------------------------------------------------------- data
def dual_rows(root):
    rows = {}
    d = os.path.join(root, "docs/dual-review-log/rows")
    for f in os.listdir(d):
        m = re.match(r"PR-(\d+)\.md$", f)
        if not m:
            continue
        t = open(os.path.join(d, f)).read()
        fm = dict(re.findall(r"^(\w+): (.*)$", t.split("---")[1], re.M))
        dm = re.search(r"^\|\s*(\d{4}-\d{2}-\d{2})", t.split("---", 2)[2], re.M)
        fm["date"] = dm.group(1) if dm else ""
        rows[m.group(1)] = fm
    # readout 6's corrections: DR-410 is a holdout; DR-207 folds into DR-208.
    for v in rows.values():
        if v.get("dr") == "410":
            v["arm"] = "holdout"
    return rows


def pr_stats(root):
    out = {}
    log = git(root, "log", "--merges", "--since=2026-09-20", "--format=%H|%ad|%s", "--date=short", "HEAD")
    for line in log.splitlines():
        h, d, s = line.split("|", 2)
        m = re.match(r"Merge (?:pull request|PR) #(\d+)", s)
        if not m or m.group(1) in out:
            continue
        add = dele = code = files = 0
        for row in git(root, "diff", "--numstat", h + "^1", h).splitlines():
            a, b, f = row.split("\t", 2)
            files += 1
            if re.match(r"(crates|scripts|demos|bindings|python|pncad|\.github)/", f) or f.endswith((".rs", ".py", ".toml", ".yml")):
                code += 1
                add += int(a) if a.isdigit() else 0
                dele += int(b) if b.isdigit() else 0
        branch = s.split("evgunter/")[-1].strip() if "evgunter/" in s else ""
        out[m.group(1)] = {"date": d, "branch": branch, "files": files, "codefiles": code, "lines": add + dele}
    return out


def work_corpus(root):
    """Every work/ markdown file on HEAD, plus each pruned file's last content."""
    texts = {}
    base = os.path.join(root, "work")
    for prog in os.listdir(base):
        pdir = os.path.join(base, prog)
        if os.path.isdir(pdir):
            for f in os.listdir(pdir):
                if f.endswith(".md"):
                    texts[f"work/{prog}/{f}"] = open(os.path.join(pdir, f)).read()
    log = git(root, "log", "HEAD", "--format=C %H", "--name-only", "--diff-filter=D", "--", "work/*")
    cur = None
    for line in log.splitlines():
        if line.startswith("C "):
            cur = line[2:]
        elif line.strip() and line not in texts:
            try:
                texts[line] = git(root, "show", f"{cur}^:{line}")
            except subprocess.CalledProcessError:
                pass
    return texts


SINGLE = re.compile(r"single[- ](full|style|review|tier)|tier[:* ]+\**(single|full|style)|\b(full|style) single\b"
                    r"|\b(full|style)[- ]review\b|style lane|Tier SINGLE", re.I)
ORCH = re.compile(r"orchestrator'?s (own )?read|no review lane|tier[:* ]+orchestrator", re.I)


def classify_population(stats, rows, texts):
    """Tier per code PR: dual (has a row), single / orch / both (a log or item
    paragraph naming the PR or its branch carries that tier's words), or
    unrecorded."""
    paras = []
    for t in texts.values():
        paras.extend(re.split(r"\n(?=- |#|\n)", t))
    by_pr, by_branch = defaultdict(list), defaultdict(list)
    for i, e in enumerate(paras):
        for n in set(re.findall(r"(?:#|\bPRs? |\bPR-)(\d{4})\b", e)):
            by_pr[n].append(i)
        for b in set(re.findall(r"`?([a-z0-9]+/[a-z0-9][a-z0-9._-]+)`?", e)):
            by_branch[b].append(i)
    tier = {}
    for n, v in stats.items():
        if not v["codefiles"]:
            continue
        if n in rows:
            tier[n] = "dual"
            continue
        cand = [paras[i] for i in by_pr.get(n, []) + by_branch.get(v["branch"], [])]
        s = any(SINGLE.search(e) for e in cand)
        o = any(ORCH.search(e) for e in cand)
        tier[n] = "both" if s and o else "single" if s else "orch" if o else "unrecorded"
    return tier


def issue_meta(texts):
    meta = {}
    for path, t in texts.items():
        if not re.search(r"^kind:\s*issue", t, re.M):
            continue
        idm = re.search(r"^id:\s*(\S+)", t, re.M)
        iid = idm.group(1) if idm else os.path.basename(path)[:-3]

        def field(name, t=t):
            m = re.search(r"^" + name + r":\s*(.*)$", t, re.M)
            return m.group(1).strip() if m else ""

        fixes = set()
        if field("pr").isdigit():
            fixes.add(field("pr"))
        for m in re.finditer(r"(?:Fixed|Closed|Greened|Re-pinned|fixed)(?: on main)? (?:by|in) (?:PR |#)(\d{4})", t):
            fixes.add(m.group(1))
        meta[iid] = {"opened": field("opened"), "closed": field("closed"), "priority": field("priority"),
                     "fix": sorted(fixes), "red_title": bool(re.search(r"red|regress", field("title"), re.I))}
    return meta


def days(a, b):
    if not a or not b:
        return None
    return (datetime.date.fromisoformat(b[:10]) - datetime.date.fromisoformat(a[:10])).days


def q(xs):
    xs = sorted(x for x in xs if x is not None)
    if not xs:
        return "n/a"
    if len(xs) < 4:
        return f"median {st.median(xs):g} (n={len(xs)})"
    qs = st.quantiles(xs, n=4)
    return f"median {st.median(xs):g} (IQR {qs[0]:g}-{qs[2]:g}, n={len(xs)})"


# ---------------------------------------------------------------- report
def main(root):
    out = []
    p = out.append
    rows = dual_rows(root)
    stats = pr_stats(root)
    texts = work_corpus(root)
    tier = classify_population(stats, rows, texts)
    meta = issue_meta(texts)

    def tier_of(n):
        if n in rows:
            r = rows[n]
            return f"dual-{r['arm']}-{r['letter']}"
        return "single"   # every non-dual introducer was read by hand: single FULL/STYLE

    def merged(n):
        if n in rows and rows[n]["date"]:
            return rows[n]["date"]
        if n in MERGED_MANUAL:
            return MERGED_MANUAL[n]
        if n in stats:
            return stats[n]["date"]
        found = git(root, "log", "HEAD", "--merges", "-E", f"--grep=#{n}\\b", "--format=%ad", "--date=short")
        return found.split()[-1] if found.split() else ""

    def in_win(d, w):
        return bool(d) and w[0] <= d <= w[1]

    # unit-level escape table
    units = defaultdict(lambda: {"issues": set(), "kinds": set(), "where": set(), "cls": set(), "ev": set()})
    for prs, issues, kind, where, cls, ev in ESCAPES:
        for n in prs.split():
            u = units[n]
            u["issues"].update(issues.split())
            u["kinds"].add(kind)
            u["where"].add(where)
            u["cls"].add(cls)
            u["ev"].add(ev)
            u["multi"] = len(prs.split()) > 1
    missing = sorted({i for _, iss, *_ in ESCAPES for i in iss.split()} - set(meta))
    p(f"escape issues not found in work/ history: {missing or 'none'}")

    for wname, w in (("WINDOW", WINDOW), ("EARLY", EARLY)):
        p(f"\n## population, code PRs merged {w[0]}..{w[1]}")
        pop = [n for n in tier if in_win(stats[n]["date"], w)]
        c = Counter(tier[n] for n in pop)
        p(f"code PRs: {len(pop)}; by recorded tier: {dict(c)}")
        dual_pop = [n for n, r in rows.items() if in_win(r["date"], w) and r.get("dr") != "207"]
        p(f"dual units by row date: {len(dual_pop)} (rows with a merge commit found: {sum(n in stats for n in dual_pop)})")
        arm = Counter(f"{rows[n]['arm']}-{rows[n]['letter']}" for n in dual_pop)
        p(f"dual by arm-letter: {dict(arm)}")
        lines = {t: [stats[n]["lines"] for n in pop if tier[n] == t] for t in c}
        p(f"merged PRs found as merge commits in the window: {sum(1 for n in stats if in_win(stats[n]['date'], w))}")
        for t, xs in lines.items():
            p(f"  lines changed, {t}: median {st.median(xs):g}, total {sum(xs)}")

        esc = {n: u for n, u in units.items() if in_win(merged(n), w)}
        p(f"\n### escapes by tier ({wname}); units with >=1 attributed red/regression")
        groups = {
            "dual, all": [n for n in dual_pop],
            "dual H concurrent": [n for n in dual_pop if rows[n]["arm"] == "concurrent" and rows[n]["letter"] == "H"],
            "dual M holdout": [n for n in dual_pop if rows[n]["arm"] == "holdout"],
            "dual M sequential": [n for n in dual_pop if rows[n]["arm"] == "sequential"],
        }
        single_rec = [n for n in pop if tier[n] in ("single", "both")]
        single_all = [n for n in pop if tier[n] in ("single", "both", "unrecorded")]
        single_sub = [n for n in single_all if stats[n]["lines"] >= 200]
        for g, ns in groups.items():
            k = sum(1 for n in ns if n in esc)
            beh = sum(1 for n in ns if n in esc and "behaviour" in esc[n]["kinds"])
            firm = sum(1 for n in ns if n in esc and esc[n]["ev"] - {"suspected"} and not esc[n]["multi"])
            p(f"  {g}: any {rate(k, len(ns))}; behaviour {rate(beh, len(ns))}; sole+not-suspected {rate(firm, len(ns))}")
        s_esc = [n for n in esc if n not in rows]
        s_beh = [n for n in s_esc if "behaviour" in esc[n]["kinds"]]
        s_firm = [n for n in s_esc if esc[n]["ev"] - {"suspected"} and not esc[n]["multi"]]
        for g, den in (("single, recorded tier", single_rec), ("single, recorded+unrecorded", single_all),
                       ("single, recorded+unrecorded, >=200 lines", single_sub)):
            p(f"  {g}: any {rate(len(s_esc), len(den))}; behaviour {rate(len(s_beh), len(den))}; "
              f"sole+not-suspected {rate(len(s_firm), len(den))}")
        dual_esc = [n for n in esc if n in rows and n in stats]
        dl = sum(stats[n]["lines"] for n in dual_pop if n in stats)
        sl = sum(stats[n]["lines"] for n in single_all)
        p(f"  per 100k changed lines: dual {100000 * len(dual_esc) / dl:.1f} ({len(dual_esc)} in {dl}); "
          f"single {100000 * len(s_esc) / sl:.1f} ({len(s_esc)} in {sl})")
        # size-matched comparison
        p("  by size band (lines changed), units with an escape / units:")
        for lo, hi in ((0, 500), (500, 1500), (1500, 4000), (4000, 10 ** 9)):
            dn = [n for n in dual_pop if n in stats and lo <= stats[n]["lines"] < hi]
            sn = [n for n in single_all if lo <= stats[n]["lines"] < hi]
            p(f"    {lo}-{hi if hi < 10 ** 9 else 'inf'}: dual {rate(sum(n in esc for n in dn), len(dn))}; "
              f"single {rate(sum(n in esc for n in sn), len(sn))}")
        p("  class of the escapes (units; a unit can carry two):")
        for t in ("dual", "single"):
            ns = [n for n in esc if (n in rows) == (t == "dual")]
            p(f"    {t}: kinds {dict(Counter(k for n in ns for k in esc[n]['kinds']))}; "
              f"where {dict(Counter(k for n in ns for k in esc[n]['where']))}; "
              f"class {dict(Counter(k for n in ns for k in esc[n]['cls']))}")

    # ---- cost of the late fixes (window)
    p("\n## cost of the attributed escapes (introducer in WINDOW)")
    esc = {n: u for n, u in units.items() if in_win(merged(n), WINDOW)}
    events = {}
    for prs, issues, kind, where, cls, _ev in ESCAPES:
        if any(in_win(merged(n), WINDOW) for n in prs.split()):
            events[prs + "|" + issues.split()[0]] = (prs, issues.split(), kind, where, cls)
    p(f"escape events: {len(events)}; issues filed for them: {sum(len(v[1]) for v in events.values())}")
    dup = [len(v[1]) for v in events.values()]
    p(f"issues per event: {q(dup)}; events filed more than once: {sum(d > 1 for d in dup)}")
    prio = Counter()
    per_kind = defaultdict(lambda: defaultdict(list))
    fixes_seen = set()
    for prs, iss, kind, _where, _cls in events.values():
        k = per_kind[kind]
        intro = max(merged(n) for n in prs.split())
        opened = min((meta[i]["opened"] for i in iss if i in meta and meta[i]["opened"]), default="")
        closed = max((meta[i]["closed"] for i in iss if i in meta and meta[i]["closed"]), default="")
        for key, v in (("detect", days(intro, opened)), ("open", days(opened, closed)),
                       ("red", days(intro, closed)), ("filings", len(iss))):
            k[key].append(v)
            per_kind["all"][key].append(v)
        k["p0"].append(any(meta.get(i, {}).get("priority") == "P0" for i in iss))
        per_kind["all"]["p0"].append(k["p0"][-1])
        prio.update(meta[i]["priority"] or "none" for i in iss if i in meta)
        for i in iss:
            for f in meta.get(i, {}).get("fix", []):
                if f in stats and f not in fixes_seen and f not in prs.split():
                    fixes_seen.add(f)
                    k["fix_lines"].append(stats[f]["lines"])
                    per_kind["all"]["fix_lines"].append(stats[f]["lines"])
    p(f"priority of the issues: {dict(prio)}")
    for kind in ("all", "pin", "behaviour", "build"):
        k = per_kind[kind]
        p(f"  {kind}: events {len(k['filings'])}; filings/event {q(k['filings'])}; "
          f"filed more than once {sum(x > 1 for x in k['filings'])}; with a P0 filing {sum(k['p0'])}")
        p(f"    days merge->first issue {q(k['detect'])}; issue->closed {q(k['open'])}; merge->closed {q(k['red'])}")
        p(f"    fix PRs named: lines changed {q(k['fix_lines'])}")
    intro_lines = [stats[n]["lines"] for n in esc if n in stats]
    p(f"introducers' own lines changed: {q(intro_lines)}")

    # ---- the ratio, by a stated model over the recorded lane costs
    p("\n## ratio: a late escape against a second review and against an up-front fix")
    lane = sorted(LANE_TOKENS_RECORDED)
    p(f"lane tokens recorded in logs (k): {q(lane)}")
    lo_r, mid_r, hi_r = REVIEW_TOKENS
    p(f"one Opus review: {lo_r // 1000}k-{hi_r // 1000}k tokens, median {mid_r // 1000}k; minutes {REVIEW_MINUTES}")
    lq = st.quantiles(lane, n=4)
    lane_lo, lane_mid, lane_hi = lq[0] * 1000, st.median(lane) * 1000, lq[2] * 1000
    fil = {kd: st.mean(per_kind[kd]["filings"]) for kd in ("pin", "behaviour")}
    # late: each filing costs a triage/bisect share of a lane; a pin is re-blessed by a
    # small share of a lane; a behaviour fix is a lane plus a review of the fix.
    # up front: the same defect fixed inside the unit's own fix pass.
    model = {
        "pin": {"late": (fil["pin"] * 0.25 * lane_lo + 0.1 * lane_lo,
                         fil["pin"] * 0.25 * lane_mid + 0.25 * lane_mid,
                         fil["pin"] * 0.5 * lane_hi + 0.5 * lane_hi),
                "upfront": (5_000, 15_000, 40_000)},
        "behaviour": {"late": (fil["behaviour"] * 0.25 * lane_lo + 0.5 * lane_lo,
                               fil["behaviour"] * 0.5 * lane_mid + lane_mid + mid_r,
                               fil["behaviour"] * lane_hi + lane_hi + hi_r),
                      "upfront": (30_000, 90_000, 150_000)},
    }
    for kd, m in model.items():
        lo, mid, hi = m["late"]
        ulo, umid, uhi = m["upfront"]
        p(f"  {kd}: late ~{mid / 1000:.0f}k ({lo / 1000:.0f}k-{hi / 1000:.0f}k) tokens = "
          f"{mid / mid_r:.1f} ({lo / hi_r:.1f}-{hi / lo_r:.1f}) reviews; up-front fix ~{umid // 1000}k "
          f"({ulo // 1000}k-{uhi // 1000}k); late/up-front {mid / umid:.0f}x ({lo / uhi:.1f}x-{hi / ulo:.0f}x)")
    n_pin = len(per_kind["pin"]["filings"])
    n_beh = len(per_kind["behaviour"]["filings"])
    mix = (n_pin * model["pin"]["late"][1] + n_beh * model["behaviour"]["late"][1]) / (n_pin + n_beh)
    p(f"  mix-weighted late escape: ~{mix / 1000:.0f}k tokens = {mix / mid_r:.1f} reviews")
    h_ship = H_TALLIED_PER_PAIR / 2
    p(f"H: tallied blocking findings per fair pair {H_TALLIED_PER_PAIR:.3f}; a lone review ships ~{h_ship:.2f} per unit; "
      f"review tokens per finding bought {mid_r / h_ship / 1e6:.2f}M")
    blo, bmid, bhi = model["behaviour"]["late"]
    p(f"  late-fix cost of the {h_ship:.2f} shipped per H unit, if each surfaced like a behaviour escape: "
      f"~{h_ship * bmid / 1000:.0f}k ({h_ship * blo / 1000:.0f}k-{h_ship * bhi / 1000:.0f}k) tokens, "
      f"against {mid_r // 1000}k for the second review; break-even late cost per shipped defect "
      f"{mid_r / h_ship / 1000:.0f}k tokens = {mid_r / h_ship / bmid:.1f}x a typical behaviour escape")
    return "\n".join(out)


if __name__ == "__main__":
    print(main(sys.argv[1] if len(sys.argv) > 1 else "."))

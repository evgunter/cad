#!/usr/bin/env python3
"""Build the HTML report from results.json."""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import analyze
import svglib as S
from svglib import esc, FABLE, OPUS

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "results.json")))


def c(key, name):
    return R[key]["contrasts"][name]["summary"]


def draws(key, name):
    return R[key]["contrasts"][name]["draws"]


def item(label, key, name, color=OPUS, sub=None, n=None):
    s = c(key, name)
    return {"label": label, "med": s["median"], "lo": s["q025"],
            "hi": s["q975"], "lo80": s["q10"], "hi80": s["q90"],
            "color": color, "sub": sub, "n": n or R[key]["n"]}


def pct(x):
    return "%.0f%%" % (100 * x)


def prob_worse(key, name):
    """P(opus rate > fable rate) for count models = P(coef>0)."""
    return c(key, name)["p_gt0"]


CSS = """
:root{
  color-scheme: light;
  --page:#f9f9f7; --surface-1:#fcfcfb;
  --text-primary:#0b0b0b; --text-secondary:#52514e; --muted:#898781;
  --grid:#e1e0d9; --axis:#c3c2b7; --border:rgba(11,11,11,0.10);
  --series-1:#2a78d6; --series-2:#eb6834;
  --good:#0ca30c; --warn:#fab219; --crit:#d03b3b;
  --card:#ffffff;
}
@media (prefers-color-scheme: dark){
  :root:not([data-theme="light"]){
    color-scheme: dark;
    --page:#0d0d0d; --surface-1:#1a1a19;
    --text-primary:#ffffff; --text-secondary:#c3c2b7; --muted:#898781;
    --grid:#2c2c2a; --axis:#383835; --border:rgba(255,255,255,0.10);
    --series-1:#3987e5; --series-2:#d95926;
    --card:#161615;
  }
}
:root[data-theme="dark"]{
  color-scheme: dark;
  --page:#0d0d0d; --surface-1:#1a1a19;
  --text-primary:#ffffff; --text-secondary:#c3c2b7; --muted:#898781;
  --grid:#2c2c2a; --axis:#383835; --border:rgba(255,255,255,0.10);
  --series-1:#3987e5; --series-2:#d95926;
  --card:#161615;
}
*{box-sizing:border-box}
body{
  margin:0; background:var(--page); color:var(--text-primary);
  font-family:system-ui,-apple-system,"Segoe UI",sans-serif;
  line-height:1.55; font-size:16px;
}
.wrap{max-width:1080px;margin:0 auto;padding:40px 24px 90px}
h1{font-size:31px;line-height:1.2;margin:0 0 6px;letter-spacing:-0.02em}
h2{font-size:22px;margin:52px 0 6px;letter-spacing:-0.01em;
   padding-top:22px;border-top:1px solid var(--border)}
h3{font-size:17px;margin:30px 0 4px}
p{margin:10px 0;max-width:74ch}
.sub{color:var(--text-secondary);font-size:15px;margin-top:0}
.lede{font-size:17px;color:var(--text-secondary)}
figure{margin:20px 0 8px;background:var(--surface-1);
  border:1px solid var(--border);border-radius:12px;padding:18px 20px 8px;
  overflow-x:auto}
figcaption{color:var(--text-secondary);font-size:13.5px;margin-top:2px;
  padding-bottom:8px;max-width:78ch}
.chart{display:block;overflow:visible}
.chart .grid{stroke:var(--grid);stroke-width:1}
.chart .axis{stroke:var(--axis);stroke-width:1}
.chart .sep{stroke:var(--grid);stroke-width:1}
.chart .refline{stroke:var(--axis);stroke-width:1.5;stroke-dasharray:4 4}
.chart .tick{fill:var(--muted);font-size:12px}
.chart .axlab{fill:var(--text-secondary);font-size:13px}
.chart .rowlab{fill:var(--text-primary);font-size:13.5px}
.chart .rowsub{fill:var(--muted);font-size:11.5px}
.chart .val{fill:var(--text-secondary);font-size:12.5px;
  font-variant-numeric:tabular-nums}
.chart .ct{fill:var(--text-primary);font-size:14px;font-weight:600}
.legend{display:flex;gap:18px;flex-wrap:wrap;margin:2px 0 10px;
  font-size:13.5px;color:var(--text-secondary)}
.lg{display:inline-flex;align-items:center;gap:7px}
.sw{width:11px;height:11px;border-radius:3px;display:inline-block}
.tiles{display:grid;grid-template-columns:repeat(auto-fit,minmax(158px,1fr));
  gap:12px;margin:22px 0 6px}
.tile{background:var(--surface-1);border:1px solid var(--border);
  border-radius:12px;padding:14px 16px}
.tile .k{font-size:26px;font-weight:600;letter-spacing:-0.02em;
  font-variant-numeric:tabular-nums}
.tile .l{font-size:12.5px;color:var(--text-secondary);margin-top:2px}
table{border-collapse:collapse;width:100%;font-size:13.5px;margin:12px 0}
th,td{text-align:left;padding:7px 10px;border-bottom:1px solid var(--border)}
th{color:var(--text-secondary);font-weight:600;font-size:12.5px;
   text-transform:uppercase;letter-spacing:0.03em}
td.n,th.n{text-align:right;font-variant-numeric:tabular-nums}
details{margin:10px 0 18px;background:var(--surface-1);
  border:1px solid var(--border);border-radius:10px;padding:10px 16px}
summary{cursor:pointer;font-size:13.5px;color:var(--text-secondary);
  font-weight:600}
details[open] summary{margin-bottom:6px}
.callout{background:var(--surface-1);border:1px solid var(--border);
  border-left:3px solid var(--series-1);border-radius:8px;
  padding:14px 18px;margin:20px 0}
.callout.warn{border-left-color:var(--warn)}
.callout.crit{border-left-color:var(--crit)}
.callout p{margin:6px 0}
.callout .h{font-weight:600;font-size:14px;margin-bottom:2px}
code{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:0.9em;
  background:var(--page);padding:1px 5px;border-radius:4px}
.foot{margin-top:60px;padding-top:18px;border-top:1px solid var(--border);
  color:var(--muted);font-size:13px}
em.q{color:var(--text-primary);font-style:normal;font-weight:600}
"""


def tiles(ts):
    out = ['<div class="tiles">']
    for k, l in ts:
        out.append('<div class="tile"><div class="k">%s</div>'
                   '<div class="l">%s</div></div>' % (k, l))
    out.append("</div>")
    return "".join(out)


def table(headers, rows, numeric=()):
    o = ["<table><thead><tr>"]
    for i, h in enumerate(headers):
        o.append('<th class="n">%s</th>' % esc(h) if i in numeric
                 else "<th>%s</th>" % esc(h))
    o.append("</tr></thead><tbody>")
    for r in rows:
        o.append("<tr>")
        for i, v in enumerate(r):
            o.append('<td class="n">%s</td>' % esc(v) if i in numeric
                     else "<td>%s</td>" % esc(v))
        o.append("</tr>")
    o.append("</tbody></table>")
    return "".join(o)


def ci_txt(key, name, dec=2):
    s = c(key, name)
    f = "%." + str(dec) + "f"
    return (f + " (95%% CrI " + f + " to " + f + ")") % (
        s["median"], s["q025"], s["q975"])


def build():
    rows = analyze.load()
    Q = analyze.v2_quality(rows)
    A = analyze.v2_all(rows)
    nf = sum(1 for r in Q if r["arm"] == "fable")
    no = sum(1 for r in Q if r["arm"] == "opus")

    H = []
    add = H.append

    add('<div class="wrap">')
    add("<h1>Opus 5 vs Fable 5: a Bayesian re-analysis of the CAD "
        "implementation A/B log</h1>")
    add('<p class="sub">%d blinded dispatches under protocol v2 (blocked '
        'randomization) &middot; %d fable / %d opus &middot; generated from '
        '<code>docs/MODEL-AB-LOG.md</code></p>' % (len(Q), nf, no))

    add('<p class="lede">The log\'s own readouts said "no evidence either arm '
        'is worse" without fitting a model. This does fit models — one per '
        'outcome, each estimating how much the arm shifts it — so the answer '
        'comes with an interval instead of a vibe. The short version: '
        '<strong>every quality interval comfortably contains "no '
        'difference", and they are wide enough that this dataset could not '
        'have detected a moderate effect if one existed.</strong> The one '
        'place the data does speak is cost.</p>')

    # ---------- tiles
    maj = c("maj_raw", "rate_ratio_opus_over_fable")
    tok = c("tok_impl", "ratio_opus_over_fable")
    add(tiles([
        ("%d" % len(Q), "blinded dispatches modelled"),
        ("%d / %d" % (nf, no), "fable / opus rows"),
        ("%.2f&times;" % maj["median"], "MAJOR-finding rate, opus vs fable"),
        ("%s" % pct(maj["p_gt0"]), "posterior P(opus has more MAJORs)"),
        ("%.2f&times;" % tok["median"], "implementation tokens, opus vs fable"),
    ]))

    add('<div class="callout"><div class="h">How to read every interval in '
        'this report</div><p>Each estimate is a posterior median with a 95%% '
        'credible interval. For rate and cost outcomes the scale is '
        '<em class="q">multiplicative</em>: 1.0 means the arms are identical, '
        '2.0 means opus produces twice the rate, 0.5 means half. For rating '
        'outcomes the scale is <em class="q">additive</em> rubric points and '
        'the reference is 0. Priors are centred on "no difference" and are '
        'weakly informative, so with n this small they pull estimates toward '
        '1.0 (or 0) — an interval that excludes the reference has to earn '
        'it.</p></div>')

    # ================= HEADLINE =================
    add("<h2>1. Is either arm producing worse work?</h2>")
    add("<p>Six outcomes, each modelled separately with the arm and the "
        "pre-logged S/M/L difficulty as predictors. Counts use Poisson "
        "regression on the log-rate scale; the plotted quantity is the rate "
        "ratio.</p>")

    q_items = [
        item("MAJOR findings", "maj_raw", "rate_ratio_opus_over_fable",
             sub="the headline defect count"),
        item("MINOR findings", "min_raw", "rate_ratio_opus_over_fable"),
        item("NOTE findings", "note_raw", "rate_ratio_opus_over_fable"),
        item("All findings pooled", "findings_total",
             "rate_ratio_opus_over_fable"),
        item("Silent deviations", "silent_count",
             "rate_ratio_opus_over_fable",
             sub="the metric the protocol weights worst"),
        item("Consequential MAJORs (strict)", "maj_conseq_lower",
             "rate_ratio_opus_over_fable",
             sub="blinded recode, ambiguous cases excluded"),
        item("Consequential MAJORs (broad)", "maj_conseq_upper",
             "rate_ratio_opus_over_fable",
             sub="blinded recode, ambiguous cases counted as defects"),
    ]
    add("<figure>")
    add(S.legend([("← fable better ·  rate ratio, opus ÷ fable  · opus better →",
                   OPUS)]))
    add(S.forest(q_items, "rate ratio (opus ÷ fable), log scale",
                 ref=1.0, log=True, xlo=0.06, xhi=16,
                 ticks=[0.125, 0.25, 0.5, 1, 2, 4, 8]))
    add('<figcaption>Dot = posterior median, thick band = 80% credible '
        'interval, thin line = 95%. The dashed line at 1.0 is "the arms are '
        'identical". Every interval crosses it.</figcaption>')
    add("</figure>")

    add("<p>No quality outcome separates the arms. The MAJOR-finding rate "
        "ratio is %s, meaning the data is equally consistent with opus "
        "producing a third as many MAJORs as fable and with it producing "
        "roughly twice as many. Silent deviations — the metric the protocol "
        "deliberately weights worst — give %s, an even wider interval, "
        "because there are only %d of them in the whole v2 sample.</p>"
        % (ci_txt("maj_raw", "rate_ratio_opus_over_fable"),
           ci_txt("silent_count", "rate_ratio_opus_over_fable"),
           int(sum(analyze.y_silent(r) or 0 for r in Q))))

    add("<figure>")
    add(S.legend([("MAJOR findings", OPUS), ("silent deviations", FABLE)]))
    add(S.density(
        [{"label": "MAJOR findings", "samples": draws("maj_raw",
                                                      "log_rate_ratio"),
          "color": OPUS,
          "ci": (c("maj_raw", "log_rate_ratio")["q025"],
                 c("maj_raw", "log_rate_ratio")["q975"])},
         {"label": "silent deviations",
          "samples": draws("silent_count", "log_rate_ratio"), "color": FABLE,
          "ci": (c("silent_count", "log_rate_ratio")["q025"],
                 c("silent_count", "log_rate_ratio")["q975"])}],
        "rate ratio, opus ÷ fable (log scale)", ref=0.0, log_axis=True,
        ticks=[math.log(v) for v in (0.125, 0.25, 0.5, 1, 2, 4, 8)],
        title="Posterior distributions, not just intervals"))
    add('<figcaption>Both posteriors are centred essentially on 1.0 and are '
        'very wide. The bar under each curve is its 95% interval. A dataset '
        'that had detected a difference would show a curve with little mass '
        'on the far side of the dashed line.</figcaption>')
    add("</figure>")

    # raw data
    add("<h3>The underlying counts</h3>")
    add("<p>Model output is only as good as what went in, so here is every "
        "modelled dispatch as a dot.</p>")
    groups = []
    for d in ("S", "M", "L"):
        pts = []
        for r in Q:
            if r["difficulty"] != d:
                continue
            v = analyze.y_maj(r)
            if v is None:
                continue
            pts.append({"v": v, "arm": r["arm"],
                        "tip": "%s — %s, %s, %d MAJOR"
                               % (r["row_id"], r["arm"], d, int(v))})
        groups.append({"label": "%s  (n=%d)" % (d, len(pts)), "points": pts})
    add("<figure>")
    add(S.legend([("fable", FABLE), ("opus", OPUS)]))
    add(S.dotstrip(groups, "MAJOR findings recorded for the dispatch",
                   title="Every v2 blinded dispatch, by difficulty"))
    add('<figcaption>Vertical position is jitter only. Hover a dot for the '
        'row id. Most dispatches on both arms recorded zero MAJORs.'
        '</figcaption>')
    add("</figure>")

    add(arm_summary_table(Q))

    # ================= DIFFICULTY =================
    add("<h2>2. Does the answer depend on S / M / L?</h2>")
    add("<p>This was the specific question the log's protocol asked to be read "
        "stratified. Fitting a separate arm effect inside each difficulty "
        "band:</p>")
    dif_items = [
        item("Small tasks", "maj_by_difficulty", "opus_in_S"),
        item("Medium tasks", "maj_by_difficulty", "opus_in_M"),
        item("Large tasks", "maj_by_difficulty", "opus_in_L"),
    ]
    add("<figure>")
    add(S.forest(dif_items, "MAJOR-finding rate ratio (opus ÷ fable), "
                 "log scale", ref=1.0, log=True, xlo=0.02, xhi=50,
                 ticks=[0.03, 0.125, 0.5, 1, 2, 8, 32],
                 title="Arm effect on MAJORs within each difficulty band"))
    add('<figcaption>Splitting 45 rows three ways leaves very little in each '
        'band, which is exactly what these intervals show. Nothing here is '
        'distinguishable from no effect.</figcaption>')
    add("</figure>")

    rub_items = [
        item("Small tasks", "rubric_by_difficulty", "opus_in_S", color=FABLE),
        item("Medium tasks", "rubric_by_difficulty", "opus_in_M", color=FABLE),
        item("Large tasks", "rubric_by_difficulty", "opus_in_L", color=FABLE),
    ]
    add("<figure>")
    add(S.forest(rub_items, "difference in mean rubric rating (opus − fable), "
                 "rubric points", ref=0.0, log=False, xlo=-2.0, xhi=2.0,
                 ticks=[-2, -1, -0.5, 0, 0.5, 1, 2],
                 title="Arm effect on the rubric within each difficulty band"))
    add('<figcaption>Same picture on the subjective quality ratings. The '
        'rubric runs 1–5, so an effect worth caring about would be at least '
        'half a point; the intervals admit effects that large in both '
        'directions.</figcaption>')
    add("</figure>")

    add("<p>The honest reading is that this design cannot answer the "
        "stratified question. The difficulty split was a good idea for "
        "balance — and it worked, the arms are near-even in every band — but "
        "it does not create the statistical power to detect a "
        "difficulty-dependent effect.</p>")

    # ================= RUBRIC =================
    add("<h2>3. Do the individual quality dimensions differ?</h2>")
    add("<p>The rubric scores three things separately: idiom/structure, "
        "whether tests pin the real contract, and doc/comment honesty. If the "
        "arms differ in <em>character</em> rather than in overall quality, "
        "this is where it would show.</p>")
    rb = [
        item("Idiom / structure", "rubric_idiom", "opus_minus_fable",
             color=FABLE),
        item("Test quality", "rubric_tests", "opus_minus_fable", color=FABLE),
        item("Doc / comment honesty", "rubric_docs", "opus_minus_fable",
             color=FABLE),
        item("All three averaged", "rubric_mean", "opus_minus_fable",
             color=OPUS, sub="the pooled rubric score"),
    ]
    add("<figure>")
    add(S.forest(rb, "difference in rating (opus − fable), rubric points",
                 ref=0.0, log=False, xlo=-1.2, xhi=1.2,
                 ticks=[-1, -0.5, -0.25, 0, 0.25, 0.5, 1]))
    add('<figcaption>All four intervals straddle zero. The averaged score is '
        'the tightest estimate in the whole report and still spans roughly '
        '±0.4 rubric points.</figcaption>')
    add("</figure>")
    add(rubric_table(Q))

    # ================= COST =================
    add("<h2>4. Cost: the one place the data does say something</h2>")
    add("<p>Tokens and wall-clock were recorded for only part of the log — "
        "the M5 readout flagged this as a data-quality failure. But what "
        "survives is <strong>exactly balanced</strong>: 12 opus and 12 fable "
        "rows record an implementation-phase token figure.</p>")

    cost_items = [
        item("Implementation tokens", "tok_impl", "ratio_opus_over_fable",
             sub="impl phase only, n=%d, balanced 12/12" % R["tok_impl"]["n"]),
        item("Total recorded tokens", "tok_total", "ratio_opus_over_fable",
             sub="impl+fix+review where recorded; phases conflated"),
        item("Implementation wall-clock", "h_impl", "ratio_opus_over_fable",
             sub="active impl hours"),
        item("Wall-clock, all rows", "h_any", "ratio_opus_over_fable",
             sub="+ covariate for gap-contaminated figures"),
    ]
    add("<figure>")
    add(S.forest(cost_items, "cost ratio (opus ÷ fable), log scale",
                 ref=1.0, log=True, xlo=0.1, xhi=4,
                 ticks=[0.125, 0.25, 0.5, 1, 2, 4]))
    add('<figcaption>Below 1.0 means opus cost less. This is the only family '
        'of outcomes where the intervals lean consistently to one side.'
        '</figcaption>')
    add("</figure>")

    add("<p>%s</p>" % cost_para())

    add("<figure>")
    add(S.legend([("fable", FABLE), ("opus", OPUS)]))
    add(cost_dotstrip(A))
    add('<figcaption>Every dispatch that recorded an implementation token '
        'figure. Hover for the row id and difficulty.</figcaption>')
    add("</figure>")

    add('<div class="callout warn"><div class="h">Read the cost result more '
        'sceptically than the quality ones</div><p>The 24 rows that carry a '
        'token figure are not a random sample of the 51 — they are the rows '
        'whose orchestrator happened to write the number down, concentrated '
        'in particular stretches of the project. Balanced arms do not fix '
        'selection on <em>when</em> a row was recorded, because unit scope '
        'and project phase both drifted over time. Treat this as the '
        'best available estimate, not a clean one.</p></div>')

    # ================= DEBUGGER VS DESIGNER =================
    add("<h2>5. “Opus debugs better, Fable designs better”</h2>")
    add('<div class="callout crit"><div class="h">The clean version of this '
        'test does not exist in this dataset</div><p>By protocol, <em '
        'class="q">all design work, all specs, and all reviews are Fable, on '
        'both arms</em>. There is zero arm variation on the design axis, so '
        'no amount of modelling can compare the models as designers here. '
        'What follows are two proxies built from the implementation rows, and '
        'they should be read as hypothesis-generating at best.</p></div>')

    add("<p>A blinded labeller — which never saw the arm column — coded each "
        "unit's character (building new machinery vs diagnosing and repairing "
        "existing code) and rated how much open-ended design judgment and how "
        "much diagnostic work it demanded. If the folk claim were true and "
        "large, the arm effect should change sign between those regimes.</p>")

    dd = [
        item("Arm × build-new-machinery", "char_maj",
             "opus_x_build_new", sub="MAJOR rate; ratio of ratios"),
        item("Arm × diagnostic load", "mod_debug_maj", "interaction",
             sub="MAJOR rate, per +1 debug-load point"),
        item("Arm × design load", "mod_design_maj", "interaction",
             sub="MAJOR rate, per +1 design-load point"),
    ]
    add("<figure>")
    add(S.forest(dd, "interaction effect on MAJOR rate (ratio scale, log)",
                 ref=1.0, log=True, xlo=0.06, xhi=16,
                 ticks=[0.125, 0.25, 0.5, 1, 2, 4, 8],
                 title="Does the arm effect change with task character?"))
    add('<figcaption>1.0 means the arm effect is the same in both regimes — '
        'i.e. no specialization. These are interaction terms, the '
        'hardest thing to estimate at small n, and it shows.</figcaption>')
    add("</figure>")

    add("<figure>")
    add(S.forest([
        item("Arm × build-new-machinery", "char_rubric",
             "opus_x_build_new", color=FABLE, sub="rubric points"),
        item("Arm × diagnostic load", "mod_debug_rubric", "interaction",
             color=FABLE, sub="rubric points per +1 debug-load"),
        item("Arm × design load", "mod_design_rubric", "interaction",
             color=FABLE, sub="rubric points per +1 design-load"),
    ], "interaction effect on rubric rating (points)", ref=0.0, log=False,
        xlo=-1.5, xhi=1.5, ticks=[-1.5, -1, -0.5, 0, 0.5, 1, 1.5],
        title="Same interactions, subjective quality ratings"))
    add('<figcaption>Nothing separates from zero.</figcaption>')
    add("</figure>")

    add("<p>A second proxy asks whether the fix pass — the closest thing in "
        "the log to a pure debugging episode — went differently by arm:</p>")
    add("<figure>")
    add(S.forest([
        item("Any red-gate round during the unit", "gate_red_any",
             "odds_ratio_opus_over_fable", sub="odds ratio"),
        item("Fix pass converged cleanly", "converged",
             "odds_ratio_opus_over_fable", sub="odds ratio"),
        item("Implementer self-caught an error", "self_caught",
             "odds_ratio_opus_over_fable", sub="odds ratio, no difficulty term"),
        item("Bigger fix pass needed", "fix_size",
             "odds_ratio_bigger_fix_opus", sub="ordered logit, 0=none..4=heavy"),
    ], "odds ratio (opus ÷ fable), log scale", ref=1.0, log=True,
        xlo=0.04, xhi=25, ticks=[0.06, 0.25, 1, 4, 16],
        title="Fix-pass behaviour as a debugging proxy"))
    add('<figcaption>Binary outcomes at n≈45 with rare events produce very '
        'wide odds ratios. None of these is informative about the folk '
        'claim.</figcaption>')
    add("</figure>")

    add("<p><strong>Verdict on the folk claim: untestable here, and not "
        "even weakly supported by the proxies.</strong> Every interaction "
        "interval spans "
        "“substantially better” to “substantially worse” "
        "in both regimes. That is not evidence against the claim either — it "
        "is the signature of a dataset with no power to address it. Testing "
        "it properly would need dispatches where the arm actually varies on "
        "design and diagnosis tasks, which this protocol deliberately "
        "prevents.</p>")

    # ================= POWER / CAVEATS =================
    add("<h2>6. What this sample could and could not have detected</h2>")
    add(power_section(Q))

    add("<h2>7. Caveats that outrank the statistics</h2>")
    add(caveats())

    # ================= APPENDIX =================
    add("<h2>Appendix</h2>")
    add(appendix(Q))

    add('<div class="foot">Generated from <code>docs/MODEL-AB-LOG.md</code> by '
        '<code>analysis/model-ab/</code>. Models fit with a hand-written '
        'adaptive Metropolis sampler (no numpy/scipy available on this '
        'machine); every model’s convergence diagnostics are in the '
        'appendix, and the sampler is cross-checked against deterministic '
        'grid quadrature. Scratch branch, not for merge.</div>')
    add("</div>")
    return "\n".join(H)


def arm_summary_table(Q):
    ac = R["maj_raw"]["arm_counts"]
    rowsout = []
    for d in ("S", "M", "L", "all"):
        for arm in ("fable", "opus"):
            sel = [r for r in Q if r["arm"] == arm
                   and (d == "all" or r["difficulty"] == d)]
            majs = [analyze.y_maj(r) for r in sel
                    if analyze.y_maj(r) is not None]
            sil = [analyze.y_silent(r) for r in sel
                   if analyze.y_silent(r) is not None]
            rub = [analyze.y_rubric_mean(r) for r in sel
                   if analyze.y_rubric_mean(r) is not None]
            rowsout.append([
                d if arm == "fable" else "", arm, str(len(sel)),
                "%d" % sum(majs) if majs else "-",
                "%.2f" % (sum(majs) / len(majs)) if majs else "-",
                "%d" % sum(sil) if sil else "0",
                "%.2f" % (sum(rub) / len(rub)) if rub else "-",
            ])
    return ("<details><summary>Table view &mdash; raw counts by arm and "
            "difficulty</summary>"
            + table(["difficulty", "arm", "rows", "MAJOR total",
                     "MAJOR mean", "silent devs", "rubric mean"],
                    rowsout, numeric=(2, 3, 4, 5, 6))
            + "</details>")


def rubric_table(Q):
    out = []
    for name, f in (("idiom", analyze.y_idiom), ("tests", analyze.y_tests),
                    ("docs", analyze.y_docs),
                    ("mean", analyze.y_rubric_mean)):
        row = [name]
        for arm in ("fable", "opus"):
            v = [f(r) for r in Q if r["arm"] == arm and f(r) is not None]
            row.append("%.2f  (n=%d)" % (sum(v) / len(v), len(v)) if v else "-")
        key = {"idiom": "rubric_idiom", "tests": "rubric_tests",
               "docs": "rubric_docs", "mean": "rubric_mean"}[name]
        row.append(ci_txt(key, "opus_minus_fable"))
        out.append(row)
    return ("<details><summary>Table view &mdash; rubric means and modelled "
            "differences</summary>"
            + table(["dimension", "fable mean", "opus mean",
                     "modelled difference (opus − fable)"], out)
            + "</details>")


def cost_para():
    t = c("tok_impl", "ratio_opus_over_fable")
    h = c("h_impl", "ratio_opus_over_fable")
    ha = c("h_any", "ratio_opus_over_fable")
    p_cheaper = 1.0 - t["p_gt0"]
    p_faster = 1.0 - h["p_gt0"]
    return ("Implementation token spend comes out at a ratio of %s — the "
            "posterior puts %s of its mass below 1.0, i.e. probability %s "
            "that opus dispatches consumed fewer implementation tokens than "
            "fable ones. Implementation wall-clock points the same way at %s "
            "(probability %s that opus was faster), and the all-rows "
            "wall-clock model, which adds a covariate for gap-contaminated "
            "figures, gives %s. These are the only intervals in the report "
            "that are close to excluding no-difference, and even they do not "
            "cleanly exclude it."
            % (ci_txt("tok_impl", "ratio_opus_over_fable"), pct(p_cheaper),
               pct(p_cheaper), ci_txt("h_impl", "ratio_opus_over_fable"),
               pct(p_faster), ci_txt("h_any", "ratio_opus_over_fable")))


def cost_dotstrip(A):
    groups = []
    for d in ("S", "M", "L"):
        pts = []
        for r in A:
            if r["difficulty"] != d:
                continue
            v = analyze.y_tok_impl(r)
            if v is None:
                continue
            pts.append({"v": v, "arm": r["arm"],
                        "tip": "%s — %s, %s, %gk impl tokens"
                               % (r["row_id"], r["arm"], d, v)})
        if pts:
            groups.append({"label": "%s  (n=%d)" % (d, len(pts)),
                           "points": pts})
    return S.dotstrip(groups, "implementation tokens (thousands)",
                      title="Recorded implementation token spend")


def power_section(Q):
    maj = c("maj_raw", "log_rate_ratio")
    width = maj["q975"] - maj["q025"]
    rub = c("rubric_mean", "opus_minus_fable")
    total_maj = int(sum(analyze.y_maj(r) or 0 for r in Q))
    o = []
    o.append("<p>The useful question is not “is there a difference?” "
             "but “what sizes of difference would this sample have "
             "caught?” The whole v2 sample contains <strong>%d MAJOR "
             "findings across %d dispatches</strong>. That is the binding "
             "constraint: a Poisson rate estimated from %d events simply "
             "cannot be pinned down tightly.</p>"
             % (total_maj, len(Q), total_maj))
    o.append(tiles([
        ("%.2f&times;" % math.exp(maj["q025"]),
         "lower end of the MAJOR-rate interval"),
        ("%.2f&times;" % math.exp(maj["q975"]),
         "upper end of the same interval"),
        ("%.1f&times;" % math.exp(width),
         "ratio of upper to lower &mdash; the width of our ignorance"),
        ("&plusmn;%.2f" % max(abs(rub["q025"]), abs(rub["q975"])),
         "rubric points still plausible either way"),
    ]))
    o.append("<p>In plain terms: if one model really did produce, say, 30%% "
             "more MAJOR findings than the other, this experiment would very "
             "likely have finished with exactly the picture it produced — "
             "intervals centred near 1.0 and far too wide to notice. The "
             "log's own conclusion (“no evidence either arm is worse; a "
             "large effect would probably have shown”) is <strong>"
             "consistent with the modelled posteriors</strong>, and this "
             "analysis mainly adds the second half: how large “large” "
             "has to be before this design could see it.</p>")
    return "".join(o)


def caveats():
    return """
<p>These are inherited from the log, and no amount of modelling fixes
them. They are listed because they dominate the uncertainty that the
credible intervals actually quantify.</p>
<ul>
<li><strong>The reviewer was never blinded to itself.</strong> The same
orchestrator model reviewed both arms, and the log documents that review
depth varied a lot between units. Findings counts measure
<em>review intensity &times; implementation quality</em>, and this
analysis cannot separate the two factors.</li>
<li><strong>Difficulty is one orchestrator's pre-flip guess.</strong> The
log itself notes that unit scope varied by more than an order of
magnitude inside a single difficulty letter. The blinded labeller's
independent scope rating agrees with the S/M/L label only loosely.</li>
<li><strong>Outcomes are graded on a subjective 1–5 rubric</strong> by the
same reviewer that produced the finding counts, so the outcome measures
are correlated with each other in ways the separate models do not
capture.</li>
<li><strong>Fix passes were sometimes run by the implementer and
sometimes applied by the orchestrator</strong>, which contaminates the
fix-pass proxies in section 5 specifically.</li>
<li><strong>Cost figures are missing not at random</strong> — recorded
for 26 of 51 v2 rows, concentrated by project phase.</li>
<li><strong>Rows 36, 38 and 40 have missing rubric or silent-deviation
data</strong>, and row 40 is an L-difficulty row, so the milestone's most
informative endpoint is partly absent. Complete-case analysis was used
throughout; nothing was imputed.</li>
</ul>
<p>The single change that would most improve a future readout is not a
better model — it is <em class="q">measuring the reviewer</em>: send the
same implementation to two independent reviewers and record both. Without
that, reviewer variance and implementation quality stay confounded no
matter how many rows accumulate.</p>
"""


def appendix(Q):
    o = []
    # diagnostics
    dr = []
    for k, v in sorted(R.items()):
        worst_r = max(d["rhat"] for d in v["diagnostics"])
        worst_e = min(d["ess"] for d in v["diagnostics"])
        dr.append([k, v["title"], str(v["n"]), "%.4f" % worst_r,
                   "%.0f" % worst_e,
                   "ok" if (worst_r < 1.01 and worst_e > 400) else "CHECK"])
    o.append("<details><summary>Model inventory and MCMC convergence "
             "diagnostics (%d models)</summary>" % len(R))
    o.append("<p>Worst split-R-hat and smallest effective sample size across "
             "all parameters of each model. R-hat below 1.01 and ESS above "
             "400 is the acceptance bar used here.</p>")
    o.append(table(["key", "model", "n", "worst R-hat", "min ESS", "status"],
                   dr, numeric=(2, 3, 4)))
    o.append("</details>")

    # prior sensitivity
    o.append("<details><summary>Prior sensitivity on the headline "
             "result</summary>")
    o.append("<p>The MAJOR-count model refit with the arm-coefficient prior "
             "halved and doubled in width. If the conclusion moved with the "
             "prior, the data would not be driving it.</p>")
    o.append(table(["prior on arm coefficient", "rate ratio (median)",
                    "95% CrI"],
                   [["half width  N(0, 0.4)",
                     "%.2f" % c("maj_prior_half",
                                "rate_ratio_opus_over_fable")["median"],
                     "%.2f – %.2f" % (
                         c("maj_prior_half",
                           "rate_ratio_opus_over_fable")["q025"],
                         c("maj_prior_half",
                           "rate_ratio_opus_over_fable")["q975"])],
                    ["as used     N(0, 0.8)",
                     "%.2f" % c("maj_raw",
                                "rate_ratio_opus_over_fable")["median"],
                     "%.2f – %.2f" % (
                         c("maj_raw", "rate_ratio_opus_over_fable")["q025"],
                         c("maj_raw", "rate_ratio_opus_over_fable")["q975"])],
                    ["double width N(0, 1.6)",
                     "%.2f" % c("maj_prior_double",
                                "rate_ratio_opus_over_fable")["median"],
                     "%.2f – %.2f" % (
                         c("maj_prior_double",
                           "rate_ratio_opus_over_fable")["q025"],
                         c("maj_prior_double",
                           "rate_ratio_opus_over_fable")["q975"])],
                    ["negative binomial, N(0, 0.8)",
                     "%.2f" % c("maj_raw_nb",
                                "rate_ratio_opus_over_fable")["median"],
                     "%.2f – %.2f" % (
                         c("maj_raw_nb", "rate_ratio_opus_over_fable")["q025"],
                         c("maj_raw_nb",
                           "rate_ratio_opus_over_fable")["q975"])]],
                   numeric=(1,)))
    o.append("<p>The median barely moves; only the interval width responds, "
             "exactly as it should. The negative-binomial fit, which allows "
             "overdispersion beyond Poisson, agrees.</p>")
    o.append("</details>")

    # method
    o.append("""<details><summary>Method, and what was excluded</summary>
<p><strong>Scope.</strong> Protocol v2 rows only (blocked randomization,
row 11 onward). The 10 protocol-v1 rows used an independent coin flip per
dispatch, landed 7&ndash;3, and come from an earlier project phase; Evan
chose to exclude them entirely rather than pool them.</p>
<p><strong>Excluded from quality models, retained for cost:</strong> the
six v2 rows with no blinded review lane (CI-infrastructure rows 16, 41,
42 and the orchestrator-review / eyeball rows MONTAGE, MV2, GUARD).
Their finding counts do not mean the same thing as a blinded review's.</p>
<p><strong>Blinded hand-coding.</strong> Four labelling passes (MAJOR
severity, task character, fix-pass convergence, cost parsing) were run
against a generated extract with the arm column removed and every
residual model name redacted, verified to contain zero leaks before the
labellers started. The labellers were instructed not to open the source
log.</p>
<p><strong>Sampler.</strong> This machine has no numpy, scipy, pymc or
pip, so the models are fit by a hand-written componentwise adaptive
random-walk Metropolis sampler, 4 chains &times; 6000 retained draws.
<code>validate_sampler.py</code> refits the headline model by
deterministic 2-D grid quadrature and compares marginals as an
independent correctness check.</p>
<p><strong>Reproduce:</strong> <code>python3 analyze.py &amp;&amp;
python3 report.py</code> from <code>analysis/model-ab/</code>.</p>
</details>""")

    o.append("""<details><summary>Judgment calls made while building
this</summary>
<p>Every decision that could have gone another way is written up in
<code>analysis/model-ab/DECISIONS.md</code>, with the reasoning, so any of
them can be overturned by re-running. The ones most likely to matter:
the definition of a &ldquo;consequential&rdquo; MAJOR (fit twice, as a
bound, because the blinded coder marked 9 of 40 MAJORs ambiguous); the
choice of implementation-phase-only token spend as the cost outcome; and
complete-case handling of the log's missing cells.</p>
</details>""")
    return "".join(o)


HTML_SHELL = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Opus 5 vs Fable 5 — Bayesian A/B re-analysis</title>
<style>%s</style>
</head>
<body>
%s
</body>
</html>
"""


def main():
    body = build()
    with open(os.path.join(HERE, "report.html"), "w") as f:
        f.write(HTML_SHELL % (CSS, body))
    with open(os.path.join(HERE, "artifact.html"), "w") as f:
        f.write("<title>Opus 5 vs Fable 5 — Bayesian A/B re-analysis</title>\n"
                "<style>%s</style>\n%s" % (CSS, body))
    print("wrote report.html and artifact.html")


if __name__ == "__main__":
    main()

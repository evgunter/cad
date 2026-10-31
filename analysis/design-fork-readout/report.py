"""results.json -> report.html (self-contained, theme-aware, inline SVG)."""
import json, sys

HERE = sys.argv[1] if len(sys.argv) > 1 else "."
R = json.load(open(f"{HERE}/results.json"))
P, X = R["pooled"], R["extra"]["pooled"]
PP = R["per_pass"]


def pct(v, d=0):
    return f"{100 * v:.{d}f}%"


def ci(s, d=0):
    return f"{100 * s['median']:.{d}f}% <span class=\"ci\">[{100 * s['lo']:.{d}f}, {100 * s['hi']:.{d}f}]</span>"


# ---------- charts ----------

def density_chart():
    d = P["h2h_density"]; s = P["h2h"]
    W, H, L, Rt, T, B = 880, 250, 40, 20, 20, 44
    sx = lambda x: L + x * (W - L - Rt)
    ym = max(d["y"]) * 1.08
    sy = lambda y: H - B - y / ym * (H - T - B)
    pts = " ".join(f"{sx(x):.1f},{sy(y):.1f}" for x, y in zip(d["x"], d["y"]))
    band = [(x, y) for x, y in zip(d["x"], d["y"]) if s["lo"] <= x <= s["hi"]]
    area = (f"{sx(band[0][0]):.1f},{sy(0):.1f} " +
            " ".join(f"{sx(x):.1f},{sy(y):.1f}" for x, y in band) +
            f" {sx(band[-1][0]):.1f},{sy(0):.1f}")
    o = [f'<svg viewBox="0 0 {W} {H}" width="100%" class="chart" role="img" '
         f'aria-label="Posterior for the share of head-to-head forks Ev decided Opus\'s way">']
    for t in (0, .25, .5, .75, 1):
        o.append(f'<line x1="{sx(t):.1f}" y1="{T}" x2="{sx(t):.1f}" y2="{H-B}" class="grid"/>')
        o.append(f'<text x="{sx(t):.1f}" y="{H-B+18}" class="tick" text-anchor="middle">{int(t*100)}%</text>')
    o.append(f'<line x1="{L}" y1="{H-B}" x2="{W-Rt}" y2="{H-B}" class="axis"/>')
    o.append(f'<polygon points="{area}" fill="var(--opus)" fill-opacity="0.18"/>')
    o.append(f'<polyline points="{pts}" fill="none" stroke="var(--opus)" stroke-width="2.2"/>')
    o.append(f'<line x1="{sx(.5):.1f}" y1="{T}" x2="{sx(.5):.1f}" y2="{H-B}" class="refline"/>')
    o.append(f'<line x1="{sx(s["median"]):.1f}" y1="{sy(0)}" x2="{sx(s["median"]):.1f}" y2="{T+6}" stroke="var(--opus)" stroke-width="1.5"/>')
    o.append(f'<text x="{sx(s["median"])+6:.1f}" y="{T+16}" class="val">median {pct(s["median"])}</text>')
    o.append(f'<text x="{sx(.5)-6:.1f}" y="{T+16}" class="val" text-anchor="end">even</text>')
    o.append(f'<text x="{sx(0):.1f}" y="{H-6}" class="axlab">← Fable\'s way</text>')
    o.append(f'<text x="{sx(1):.1f}" y="{H-6}" class="axlab" text-anchor="end">Opus\'s way →</text>')
    o.append("</svg>")
    return "\n".join(o)


def mix_chart():
    cats = [("opus", "Opus only", "var(--opus)"),
            ("fable", "Fable only", "var(--fable)"),
            ("both", "Both (they agreed)", "var(--both)"),
            ("mixed", "Part of each", "var(--mixed)"),
            ("neither", "Neither", "var(--neither)")]
    W, rowh, L, Rt = 880, 46, 120, 10
    H = 2 * rowh + 20
    o = [f'<svg viewBox="0 0 {W} {H}" width="100%" class="chart" role="img" '
         f'aria-label="Which first recommendation Ev took, per coding pass">']
    for i, p in enumerate(("p1", "p2")):
        c = PP[p]["cats"]; n = sum(c.get(k, 0) for k, *_ in cats)
        y = 8 + i * rowh; x = L
        o.append(f'<text x="0" y="{y+19}" class="rowlab">Pass {i+1}</text>')
        o.append(f'<text x="0" y="{y+33}" class="rowsub">{"Opus" if p=="p1" else "Fable"} coders · n={n}</text>')
        for k, lab, col in cats:
            w = c.get(k, 0) / n * (W - L - Rt)
            o.append(f'<rect x="{x:.1f}" y="{y}" width="{max(w-2,0):.1f}" height="30" rx="3" fill="{col}"/>')
            if w > 34:
                o.append(f'<text x="{x+w/2-1:.1f}" y="{y+20}" class="inbar" text-anchor="middle">{c.get(k,0)}</text>')
            x += w
    o.append("</svg>")
    leg = "".join(f'<span class="lg"><span class="sw" style="background:{col}"></span>{lab}</span>'
                  for _, lab, col in cats)
    return f'<div class="legend">{leg}</div>' + "\n".join(o)


def forest(rows, title):
    W, L, Rt, rowh, T = 880, 330, 70, 40, 10
    H = T + rowh * len(rows) + 34
    sx = lambda x: L + x * (W - L - Rt)
    o = [f'<svg viewBox="0 0 {W} {H}" width="100%" class="chart" role="img" aria-label="{title}">']
    for t in (0, .25, .5, .75, 1):
        o.append(f'<line x1="{sx(t):.1f}" y1="{T}" x2="{sx(t):.1f}" y2="{H-30}" class="grid"/>')
        o.append(f'<text x="{sx(t):.1f}" y="{H-12}" class="tick" text-anchor="middle">{int(t*100)}%</text>')
    o.append(f'<line x1="{sx(.5):.1f}" y1="{T}" x2="{sx(.5):.1f}" y2="{H-30}" class="refline"/>')
    for i, (lab, sub, s, col) in enumerate(rows):
        y = T + i * rowh + rowh / 2
        o.append(f'<text x="0" y="{y-2:.1f}" class="rowlab">{lab}</text>')
        o.append(f'<text x="0" y="{y+12:.1f}" class="rowsub">{sub}</text>')
        o.append(f'<line x1="{sx(s["lo"]):.1f}" y1="{y:.1f}" x2="{sx(s["hi"]):.1f}" y2="{y:.1f}" stroke="{col}" stroke-width="3" stroke-linecap="round"/>')
        o.append(f'<circle cx="{sx(s["median"]):.1f}" cy="{y:.1f}" r="5.5" fill="{col}" stroke="var(--card)" stroke-width="1.5"/>')
        o.append(f'<text x="{W-Rt+8}" y="{y+4:.1f}" class="val">{pct(s["median"])}</text>')
    o.append("</svg>")
    return "\n".join(o)


def trend_chart():
    W, H, L, Rt, T, B = 880, 260, 46, 20, 16, 44
    tr = R["trend"]
    tmax = max(max(tr[p]["curve"]["t"]) for p in tr)
    sx = lambda t: L + t / tmax * (W - L - Rt)
    sy = lambda v: H - B - v * (H - T - B)
    o = [f'<svg viewBox="0 0 {W} {H}" width="100%" class="chart" role="img" '
         f'aria-label="Share of head-to-head forks decided Opus\'s way, over time">']
    for v in (0, .25, .5, .75, 1):
        o.append(f'<line x1="{L}" y1="{sy(v):.1f}" x2="{W-Rt}" y2="{sy(v):.1f}" class="grid"/>')
        o.append(f'<text x="{L-8}" y="{sy(v)+4:.1f}" class="tick" text-anchor="end">{int(v*100)}%</text>')
    o.append(f'<line x1="{L}" y1="{sy(.5):.1f}" x2="{W-Rt}" y2="{sy(.5):.1f}" class="refline"/>')
    for wk in range(0, int(tmax) + 1, 7):
        lab = ["24 Sep", "1 Oct", "8 Oct", "15 Oct", "22 Oct", "29 Oct"][wk // 7] if wk // 7 < 6 else ""
        o.append(f'<text x="{sx(wk):.1f}" y="{H-B+18}" class="tick" text-anchor="middle">{lab}</text>')
    styles = {"p1": ("var(--opus)", ""), "p2": ("var(--fable)", ' stroke-dasharray="6 4"')}
    for p in ("p1", "p2"):
        c = tr[p]["curve"]; col, dash = styles[p]
        up = " ".join(f"{sx(t):.1f},{sy(v):.1f}" for t, v in zip(c["t"], c["hi"]))
        dn = " ".join(f"{sx(t):.1f},{sy(v):.1f}" for t, v in reversed(list(zip(c["t"], c["lo"]))))
        o.append(f'<polygon points="{up} {dn}" fill="{col}" fill-opacity="0.10"/>')
        md = " ".join(f"{sx(t):.1f},{sy(v):.1f}" for t, v in zip(c["t"], c["med"]))
        o.append(f'<polyline points="{md}" fill="none" stroke="{col}" stroke-width="2"{dash}/>')
    for t, y in tr["p1"]["points"]:
        yy = sy(0.97) if y else sy(0.03)
        o.append(f'<circle cx="{sx(t):.1f}" cy="{yy:.1f}" r="3" fill="{"var(--opus)" if y else "var(--fable)"}" fill-opacity="0.55"/>')
    o.append(f'<text x="{L}" y="{H-6}" class="axlab">fork date (pass-1 codes shown as ticks: top = Opus\'s way, bottom = Fable\'s)</text>')
    o.append("</svg>")
    return "\n".join(o)


# ---------- page ----------

n_dec = PP["p1"]["decided"]
h1, h2 = PP["p1"]["h2h"], PP["p2"]["h2h"]
ag = PP["p1"]["agree"]; nag = sum(ag.values())
ev = PP["p1"]["ev_vs"]; nev = sum(ev.values())
K = R["kappa"]
tr = R["trend"]
ex = R["extra"]["per_pass"]

def onesided(p):
    d = ex[p]["logged_vs_coder"]
    tot = sum(v for k, v in d.items() if k.split("|")[0] in ("opus", "fable"))
    keep = d.get("opus|opus", 0) + d.get("fable|fable", 0)
    return tot, keep
lg1, kp1 = onesided("p1"); lg2, kp2 = onesided("p2")

forest_main = forest([
    ("Head-to-head", f"Ev took exactly one first report · n={h1[0]+h1[1]} / {h2[0]+h2[1]}", P["h2h"], "var(--opus)"),
    ("Overall lean", "includes 'part of each' and 'neither' rows", P["lean"], "var(--opus)"),
    ("Head-to-head, protocol-clean", "drops the rows with a blinding deviation", P["strict_h2h"], "var(--opus)"),
    ("Log's own match cell", "written after unblinding; one-sided cells only", P["logged_h2h"], "var(--muted)"),
    (f"First half (before {X['mid_date'][5:]})", "head-to-head", X["early"], "var(--opus)"),
    (f"Second half", "head-to-head", X["late"], "var(--opus)"),
    ("Control: label A over label B", "should sit at 50%", P["position"], "var(--muted)"),
], "Share decided Opus's way")

forest_style = forest([
    ("Challenged ratified text", "when exactly one did, it was Opus", P["ratified_disc"], "var(--opus)"),
    ("Rejected the framing", "when exactly one did, it was Opus", P["frame_disc"], "var(--opus)"),
    ("Stated the higher confidence", "when they differed, it was Opus", P["conf_disc"], "var(--opus)"),
    ("Moved in reconciliation", "when exactly one moved, it was Opus", P["mover_opus"], "var(--opus)"),
    ("Ev sided with the text-challenger", "head-to-head rows where one challenged", X["challenger"], "var(--muted)"),
    ("Ev sided with the framing-rejecter", "head-to-head rows where one rejected", X["framer"], "var(--muted)"),
], "How the two designers differ")


def conf_row(m, c):
    a = ex["p1"]["conf_hit"].get(f"{m}:{c}", [0, 0]); b = ex["p2"]["conf_hit"].get(f"{m}:{c}", [0, 0])
    return f"<td class=n>{a[0]}/{a[0]+a[1]}</td><td class=n>{b[0]}/{b[0]+b[1]}</td>"


html = f"""<title>Design-fork Designer Readout</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono&family=IBM+Plex+Sans:wght@400;600&family=Source+Serif+4:opsz,wght@8..60,600&display=swap">
<style>
/* Layout: one reading column, figures full-bleed inside it; same report family as the model-A/B readout. */
:root{{
  color-scheme: light;
  --page:#f6f7f9; --surface-1:#fdfdfe; --card:#ffffff;
  --text-primary:#10141a; --text-secondary:#4a5361; --muted:#7a8494;
  --grid:#e3e7ee; --axis:#c2c9d4; --border:rgba(16,20,26,0.11);
  --opus:#2a6fd0; --fable:#e0632b; --both:#7d8ea6; --mixed:#b9a3d6; --neither:#d3d8e0;
  --onbar:#ffffff;
  --display:"Source Serif 4",Charter,Georgia,serif;
  --body:"IBM Plex Sans",system-ui,-apple-system,"Segoe UI",sans-serif;
  --mono:"IBM Plex Mono",ui-monospace,Menlo,monospace;
}}
@media (prefers-color-scheme: dark){{
  :root:not([data-theme="light"]){{
    color-scheme: dark;
    --page:#0c0e12; --surface-1:#14171d; --card:#181c23;
    --text-primary:#eef1f6; --text-secondary:#b3bcc9; --muted:#838d9c;
    --grid:#252a33; --axis:#363d49; --border:rgba(255,255,255,0.11);
    --opus:#4f8ff0; --fable:#ee7a45; --both:#7b8aa0; --mixed:#8f78b3; --neither:#3a414d;
    --onbar:#0c0e12;
  }}
}}
:root[data-theme="dark"]{{
  color-scheme: dark;
  --page:#0c0e12; --surface-1:#14171d; --card:#181c23;
  --text-primary:#eef1f6; --text-secondary:#b3bcc9; --muted:#838d9c;
  --grid:#252a33; --axis:#363d49; --border:rgba(255,255,255,0.11);
  --opus:#4f8ff0; --fable:#ee7a45; --both:#7b8aa0; --mixed:#8f78b3; --neither:#3a414d;
  --onbar:#0c0e12;
}}
*{{box-sizing:border-box}}
body{{margin:0;background:var(--page);color:var(--text-primary);font-family:var(--body);
  line-height:1.6;font-size:16px;-webkit-font-smoothing:antialiased}}
.wrap{{max-width:980px;margin:0 auto;padding-block:36px 80px;
  padding-left:max(16px, env(safe-area-inset-left, 0px));padding-right:max(16px, env(safe-area-inset-right, 0px))}}
h1{{font-family:var(--display);font-size:clamp(28px,5vw,38px);line-height:1.15;margin:0 0 8px;font-weight:600;text-wrap:balance}}
h2{{font-family:var(--display);font-size:24px;margin:52px 0 8px;font-weight:600;padding-top:22px;border-top:1px solid var(--border);text-wrap:balance}}
p,li{{max-width:70ch}} p{{margin:11px 0}}
.sub{{color:var(--text-secondary);font-size:14.5px;margin-top:0}}
.lede{{font-size:17px}}
.ci{{color:var(--muted);font-size:0.9em;font-variant-numeric:tabular-nums}}
figure{{margin:18px 0 8px;background:var(--surface-1);border:1px solid var(--border);border-radius:12px;padding:16px 18px 8px;overflow-x:auto}}
figcaption{{color:var(--text-secondary);font-size:13.5px;padding-bottom:8px;max-width:78ch}}
.chart{{display:block;overflow:visible;min-width:560px}}
.chart .grid{{stroke:var(--grid);stroke-width:1}} .chart .axis{{stroke:var(--axis)}}
.chart .refline{{stroke:var(--axis);stroke-width:1.5;stroke-dasharray:4 4}}
.chart .tick{{fill:var(--muted);font-size:12px}} .chart .axlab{{fill:var(--text-secondary);font-size:13px}}
.chart .rowlab{{fill:var(--text-primary);font-size:14px}} .chart .rowsub{{fill:var(--muted);font-size:11.5px}}
.chart .val{{fill:var(--text-secondary);font-size:13px;font-variant-numeric:tabular-nums}}
.chart .inbar{{fill:var(--onbar);font-size:13px;font-weight:600}}
.legend{{display:flex;gap:16px;flex-wrap:wrap;margin:0 0 10px;font-size:13.5px;color:var(--text-secondary)}}
.lg{{display:inline-flex;align-items:center;gap:7px}} .sw{{width:11px;height:11px;border-radius:3px;display:inline-block}}
.tiles{{display:grid;grid-template-columns:repeat(auto-fit,minmax(170px,1fr));gap:12px;margin:22px 0 6px}}
.tile{{background:var(--surface-1);border:1px solid var(--border);border-radius:12px;padding:14px 16px;min-width:0}}
.tile .k{{font-family:var(--display);font-size:28px;font-weight:600;font-variant-numeric:tabular-nums}}
.tile .l{{font-size:12.5px;color:var(--text-secondary)}}
.tbl{{overflow-x:auto}}
table{{border-collapse:collapse;width:100%;font-size:14px;margin:12px 0;font-variant-numeric:tabular-nums}}
th,td{{text-align:left;padding:7px 10px;border-bottom:1px solid var(--border)}}
th{{color:var(--text-secondary);font-weight:600;font-size:12px;text-transform:uppercase;letter-spacing:0.04em}}
td.n,th.n{{text-align:right}}
.callout{{background:var(--surface-1);border:1px solid var(--border);border-radius:10px;padding:14px 18px;margin:20px 0}}
.callout .h{{font-family:var(--display);font-weight:600;font-size:16px;margin-bottom:2px}}
code{{font-family:var(--mono);font-size:0.86em;background:var(--surface-1);border:1px solid var(--border);padding:0 4px;border-radius:4px}}
.foot{{margin-top:56px;padding-top:16px;border-top:1px solid var(--border);color:var(--muted);font-size:13px}}
:focus-visible{{outline:2px solid var(--opus);outline-offset:2px}}
</style>
<div class="wrap">
<h1>Opus vs Fable designers: which first report did Ev's decision take?</h1>
<p class="sub">First readout of the design-fork experiment · {R['n_rows']} rows of <code>docs/DESIGN-FORK-LOG.md</code> (24 Sep – 30 Oct 2026), {n_dec} decided · two independent blinded coding passes · Beta(1,1) priors throughout</p>

<p class="lede">Ev's decisions lean slightly toward the Opus designer's first report, but the lean is not established. Where Ev's decision took exactly one designer's first recommendation, it was Opus's <strong>{ci(P['h2h'])}</strong> of the time, with {pct(P['h2h']['p_gt_ref'])} posterior probability that the true share is above even. Counting every decided fork, the lean shrinks to {ci(P['lean'])}. The more consistent differences are in how the two designers work: Opus more often challenges ratified text, and Fable more often states high confidence.</p>

<div class="tiles">
<div class="tile"><div class="k">{pct(P['h2h']['median'])}</div><div class="l">head-to-head forks decided Opus's way</div></div>
<div class="tile"><div class="k">{pct(P['h2h']['p_gt_ref'])}</div><div class="l">P(Ev favours Opus's first report)</div></div>
<div class="tile"><div class="k">{h1[0]+h1[1]}/{h2[0]+h2[1]}</div><div class="l">head-to-head rows (pass 1 / pass 2)</div></div>
<div class="tile"><div class="k">{pct(ag['full']/nag)}</div><div class="l">forks where the first reports fully agreed</div></div>
<div class="tile"><div class="k">κ {K['coder_match']['kappa']:.2f}</div><div class="l">coder agreement on the match code</div></div>
</div>

<div class="callout"><div class="h">What "first report" means here</div>
<p>The protocol records each designer's recommendation before either sees the other's report. Most forks then go through reconciliation, and the PR carries the converged position: Ev accepted it as carried in {pct(ev['accepted']/nev)} of decided forks, amended it in {pct(ev['amended']/nev)} and rejected it in {pct(ev['rejected']/nev)}. So "Ev took Opus's first report" mostly means that reconciliation converged on Opus's opening position and Ev then approved it. This measures whose first instinct survived, not a preference Ev expressed between two named options. Ev never saw the model names.</p></div>

<h2>1. The head-to-head</h2>
<p>A row counts as head-to-head when Ev's decision matches one designer's first recommendation and not the other's, as judged by blinded coders comparing the <code>ev</code> cell with the two first reports. Pass 1 found {h1[0]} decided Opus's way and {h1[1]} Fable's; pass 2 found {h2[0]} and {h2[1]}. The posterior below is the equal mixture of the two passes' Beta posteriors, so disagreement between the coders widens it rather than being settled by fiat.</p>
<figure>{density_chart()}
<figcaption>Posterior for the share of head-to-head forks decided Opus's way. The shaded band is the 95% credible interval; the dashed line is even.</figcaption></figure>

<figure>{mix_chart()}
<figcaption>Every decided fork, by which first recommendation Ev's decision took. The head-to-head rows are the first two segments. "Part of each" means Ev took one designer on some sub-questions and the other on the rest.</figcaption></figure>

<p>Taking each designer on its own, Ev's decision took Opus's first report (alone or shared) in {ci(P['hit_opus'])} of decided forks and Fable's in {ci(P['hit_fable'])}. The difference, {100*P['hit_diff']['median']:+.1f} points <span class="ci">[{100*P['hit_diff']['lo']:+.1f}, {100*P['hit_diff']['hi']:+.1f}]</span>, includes zero. Neither designer's first report is what Ev ends up with most of the time. Forks are usually multi-part, and the final answer is assembled.</p>

<h2>2. Robustness</h2>
<figure>{forest_main}
<figcaption>Posterior medians and 95% credible intervals for the share decided Opus's way under different cuts. Grey rows are checks, not effects.</figcaption></figure>
<ul>
<li><strong>Label position is a clean control.</strong> Ev took label A {ci(P['position'])} of the time. The urandom labels work, and no position effect is hiding a model effect.</li>
<li><strong>The log's own match cell leans further toward Opus</strong> ({ci(P['logged_h2h'])}). That cell was written after unblinding, so it is worth checking for bias. Its one-sided cells mostly agree with the coders: of the {lg1} rows the log credits to one designer alone, blinded pass 1 keeps {kp1} on that designer and pass 2 keeps {kp2} of {lg2}. The difference comes from rows the log calls "both" (usually "both, after reconciliation"). The coders re-split those against the first reports: {ex['p1']['logged_vs_coder'].get('both|opus',0)} to Opus and {ex['p1']['logged_vs_coder'].get('both|fable',0)} to Fable in pass 1, {ex['p2']['logged_vs_coder'].get('both|opus',0)} and {ex['p2']['logged_vs_coder'].get('both|fable',0)} in pass 2. That split is even, so it dilutes the lean rather than reversing it. There is no sign that the unblinded writer shaded the cell toward either model. The forks the log itself reads as clean wins favour Opus more than the marginal ones do.</li>
<li><strong>Dropping the protocol deviations</strong> (row 2, whose blinding broke, and row 212, with no byte drawn) changes nothing: {ci(P['strict_h2h'])}.</li>
<li><strong>Coder reliability is good.</strong> The Opus-run and Fable-run coding passes agree on {pct(K['coder_match']['agree'])} of match codes (Cohen's κ {K['coder_match']['kappa']:.2f}), {pct(K['agree_first']['agree'])} on first-report agreement (κ {K['agree_first']['kappa']:.2f}) and {pct(K['movement']['agree'])} on reconciliation movement (κ {K['movement']['kappa']:.2f}). Their head-to-head counts differ ({h1[0]}:{h1[1]} vs {h2[0]}:{h2[1]}) mostly on rows one pass calls "part of each".</li>
</ul>

<h3>Over time</h3>
<figure>{trend_chart()}
<figcaption>Logistic regression of head-to-head outcome on fork date, fitted by grid quadrature (N(0,1.5²) intercept, N(0,1²) per-week slope). Solid: pass-1 codes; dashed: pass-2 codes. Bands are 95% credible intervals.</figcaption></figure>
<p>The two passes disagree on the direction of any drift. The odds ratio per week is {tr['p1']['or_per_week']['median']:.2f} <span class="ci">[{tr['p1']['or_per_week']['lo']:.2f}, {tr['p1']['or_per_week']['hi']:.2f}]</span> in pass 1 and {tr['p2']['or_per_week']['median']:.2f} <span class="ci">[{tr['p2']['or_per_week']['lo']:.2f}, {tr['p2']['or_per_week']['hi']:.2f}]</span> in pass 2. Both intervals contain 1, so there is no trend.</p>

<h2>3. How the designers differ</h2>
<p>The fields each first report records (confidence, framing rejected, ratified text challenged) separate the two models more cleanly than Ev's decisions do. Each estimate is taken over the rows where the two designers differed on that field.</p>
<figure>{forest_style}
<figcaption>Upper four rows: on forks where the designers differed on the field, the share where Opus was the one. Lower two: on head-to-head forks where exactly one designer challenged or rejected, the share where Ev's decision went that designer's way.</figcaption></figure>
<ul>
<li><strong>Opus challenges ratified text more.</strong> Opus did so on {pct(P['ratified_opus']['median'])} of its first reports and Fable on {pct(P['ratified_fable']['median'])}. When exactly one challenged, it was Opus {ci(P['ratified_disc'])}, the clearest difference in the data. Challenging does not reliably win: on head-to-head forks Ev went the challenger's way {ci(X['challenger'])} of the time.</li>
<li><strong>Both designers reject the framing almost always</strong> (Opus {pct(P['frame_opus']['median'])}, Fable {pct(P['frame_fable']['median'])}). The field barely discriminates. In the few head-to-head forks where only one rejected it, Ev sided with the rejecter {ci(X['framer'])}, from about nine rows.</li>
<li><strong>Fable states high confidence more.</strong> Nearly every report says "likely". When the two confidences differed, the higher one was Fable's on {pct(1-P['conf_disc']['median'])} of forks. The table below shows how often each designer's report was taken, by stated confidence. Opus's rare "high" calls were taken most of the time; Fable's "high" calls were taken a little over half the time. The counts are too small to call a calibration difference.</li>
<li><strong>Neither designer concedes more.</strong> Most reconciliations move both. When only one moved, it was Opus {ci(P['mover_opus'])}.</li>
</ul>
<div class="tbl"><table>
<tr><th>Designer</th><th>Stated confidence</th><th class=n>Taken (pass 1)</th><th class=n>Taken (pass 2)</th></tr>
<tr><td>Opus</td><td>likely</td>{conf_row('opus','2')}</tr>
<tr><td>Opus</td><td>high</td>{conf_row('opus','3')}</tr>
<tr><td>Fable</td><td>likely</td>{conf_row('fable','2')}</tr>
<tr><td>Fable</td><td>high</td>{conf_row('fable','3')}</tr>
</table></div>

<h2>4. What this does and does not show</h2>
<ul>
<li><strong>A weak lean, not a result.</strong> At about 80 head-to-head forks the interval still spans roughly 44–68%. If the true share is near 56%, ruling out "even" at the 95% level takes roughly 270 head-to-head forks, about three times the current count. At the current pace of about two head-to-head forks a day, that is around three more months.</li>
<li><strong>The outcome is Ev's judgment, not ground truth.</strong> "Ev took Opus's report" says whose answer Ev chose, not whose design held up later. A follow-up could score decided forks by whether the chosen design was later revised.</li>
<li><strong>Forks are not independent.</strong> Several rows share a program and an orchestrator, and some forks follow from earlier ones. The Beta model treats them as exchangeable, so the intervals are somewhat too narrow.</li>
<li><strong>Reconciliation is part of the treatment.</strong> Because Ev usually decides on the converged recommendation, a designer that argues its first position persuasively in reconciliation will score well here even if Ev would have chosen differently between the raw first reports.</li>
</ul>

<h2>Method</h2>
<p><code>parse_log.py</code> splits the 230 table rows (some fork numbers repeat after merge collisions, so rows are keyed by line). <code>blind_extract.py</code> drops the A/B column, redacts every model name and fails if any survives. Eight agents then coded the four extract chunks against <code>CODING-PROMPT.md</code>, a full pass by Opus coders and a full pass by Fable coders. None could see the mapping. <code>key.py</code> reads the mapping from each row's A/B column, checks it against the urandom byte's parity (no contradictions), and is the only file that names the models. <code>analyze.py</code> joins the two and fits every proportion as a Beta(1,1)–binomial and the outcome mix as a Dirichlet. Each estimate is reported pooled over both passes. Everything lives on <code>analysis/design-fork/readout-1</code>.</p>

<div class="foot">Off-file readout under the design-fork protocol's rule 7: kept on an <code>analysis/</code> branch, not summarised anywhere agents read. An orchestrator with a design fork in flight should not read it.</div>
</div>
"""
open(f"{HERE}/report.html", "w").write(html)
print("wrote report.html", len(html))

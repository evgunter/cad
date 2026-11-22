"""results.json -> report.html (self-contained, theme-aware). Run after analyze.py."""
import json, sys

HERE = sys.argv[1] if len(sys.argv) > 1 else "."
R = json.load(open(f"{HERE}/results.json"))
I, V = R["implementer"], R["reviewer"]
P = V["paired (concurrent units)"]


def rr(d):
    a = d["adjusted_rate_ratio_fable_over_opus"]
    return dict(est=a["est"], lo=a["lo"], hi=a["hi"])


def lr(d):
    a = d["letter_adjusted_ratio_fable_over_opus"]
    return dict(est=a["ratio"], lo=a["lo"], hi=a["hi"])


def nn(d):
    return f'{d["Fable 5.2"]["n"]} Fable / {d["Opus 5.5"]["n"]} Opus'


rev_forest = [
    dict(label="MAJORs raised per review", **rr(V["MAJORs raised"]), n=nn(V["MAJORs raised"]), group="quality"),
    dict(label="DEMONSTRATED MAJORs raised", **rr(V["DEMONSTRATED MAJORs raised"]), n=nn(V["DEMONSTRATED MAJORs raised"]), group="quality"),
    dict(label="MINORs raised", **rr(V["MINORs raised"]), n=nn(V["MINORs raised"]), group="quality"),
    dict(label="NOTEs raised", **rr(V["NOTEs raised"]), n=nn(V["NOTEs raised"]), group="quality"),
    dict(label="Review wall clock", **lr(V["review wall clock (ms)"]), n=nn(V["review wall clock (ms)"]), group="cost"),
    dict(label="Review cost (USD)", **lr(V["review cost (USD, where given)"]), n=nn(V["review cost (USD, where given)"]), group="cost"),
    dict(label="Review tokens (harness total)", **lr(V["review tokens (harness total, where given)"]), n=nn(V["review tokens (harness total, where given)"]), group="cost"),
]
imp_forest = [
    dict(label="MAJORs received per review", **rr(I["MAJORs per review"]), n=nn(I["MAJORs per review"]), group="quality"),
    dict(label="DEMONSTRATED MAJORs received", **rr(I["DEMONSTRATED MAJORs per review"]), n=nn(I["DEMONSTRATED MAJORs per review"]), group="quality"),
    dict(label="MINORs received", **rr(I["MINORs per review"]), n=nn(I["MINORs per review"]), group="quality"),
    dict(label="Implementation wall clock", **lr(I["implementation wall clock (ms)"]), n=nn(I["implementation wall clock (ms)"]), group="cost"),
    dict(label="Implementation cost (USD)", **lr(I["implementation cost (USD, where given)"]), n=nn(I["implementation cost (USD, where given)"]), group="cost"),
    dict(label="Implementation tokens", **lr(I["implementation tokens (harness total, where given)"]), n=nn(I["implementation tokens (harness total, where given)"]), group="cost"),
]
pairs = [dict(pr=p["pr"], impl=p["impl"].split()[0], uf=p["uni_f"], uo=p["uni_o"], mf=p["f_major"], mo=p["o_major"],
              umf=p["umaj_f"], umo=p["umaj_o"]) for p in P["rows"]]
cells = R["cells"]
df = R["design_fork"]

DATA = dict(rev_forest=rev_forest, imp_forest=imp_forest, pairs=pairs,
            df=[dict(label="Two designers (Opus + Fable)", **df["5.2 / two designers"]["match"]),
                dict(label="Three designers (2 Opus + Fable)", **df["5.2 / three designers"]["match"])])


def pct(x):
    return f"{100 * x:.0f}%"


def ci(d, f=lambda x: f"{x:.2f}"):
    return f'{f(d["est"])} <span class="ci">[{f(d["lo"])}–{f(d["hi"])}]</span>'


def prop_row(label, d, note=""):
    return (f'<tr><th scope="row">{label}</th><td class="num">{d["fable"]} <span class="ci">({pct(d["fable_rate"])})</span></td>'
            f'<td class="num">{d["opus"]} <span class="ci">({pct(d["opus_rate"])})</span></td>'
            f'<td class="num">{100 * d["diff_median"]:+.0f} pts <span class="ci">[{100 * d["lo"]:+.0f}, {100 * d["hi"]:+.0f}]</span></td>'
            f'<td>{note}</td></tr>')


rub = I["rubric (1-5, the reviewer's)"]


def rub_row(k, label):
    d = rub[k]
    e = d["diff_fable_minus_opus"]
    return (f'<tr><th scope="row">{label}</th><td class="num">{d["Fable 5.2"]["mean"]:.2f} <span class="ci">(n={d["Fable 5.2"]["n"]})</span></td>'
            f'<td class="num">{d["Opus 5.5"]["mean"]:.2f} <span class="ci">(n={d["Opus 5.5"]["n"]})</span></td>'
            f'<td class="num">{e["est"]:+.2f} <span class="ci">[{e["lo"]:+.2f}, {e["hi"]:+.2f}]</span></td><td></td></tr>')


pop = R["population"]
c4 = {k: v for k, v in cells.items() if "/" in k and "interaction" not in k}
inter = cells["interaction (log rate ratio of the reviewer gap on Fable code vs Opus code)"]
dfp = df["two-designer discordant: P(Ev sided with Fable)"]
ci_red = I["hosted CI red on the branch's own row at the first pushed head"]
rv_t = V["review wall clock (ms)"]
rv_c = V["review cost (USD, where given)"]

html = open(f"{HERE}/template.html").read()
subs = {
    "DATA_JSON": json.dumps(DATA),
    "N_ROWS": str(pop["rows"]),
    "IMP_F": str(pop["implementer_fair"]["Fable 5.2"]), "IMP_O": str(pop["implementer_fair"]["Opus 5.5"]),
    "REV_F": str(pop["reviews_fair"]["Fable 5.2"]), "REV_O": str(pop["reviews_fair"]["Opus 5.5"]),
    "N_PAIRS": str(P["n_pairs"]),
    "MAJ_RR": ci(rr(V["MAJORs raised"])),
    "MAJ_F": f'{V["MAJORs raised"]["Fable 5.2"]["total"]}', "MAJ_O": f'{V["MAJORs raised"]["Opus 5.5"]["total"]}',
    "NM_F": V["verdict NOT-MERGEABLE-AS-IS"]["fable"], "NM_O": V["verdict NOT-MERGEABLE-AS-IS"]["opus"],
    "UNI_F": f'{P["unilateral_findings"]["fable_total"]:.0f}', "UNI_O": f'{P["unilateral_findings"]["opus_total"]:.0f}',
    "UNI_N": str(P["unilateral_findings"]["n"]), "UNI_WIN": str(P["unilateral_findings"]["fable_more"]),
    "UMAJ_F": f'{P["unilateral_majors"]["fable_total"]:.0f}', "UMAJ_O": f'{P["unilateral_majors"]["opus_total"]:.0f}',
    "PT_RATIO": f'{P["time_ratio"]["ratio"]:.2f}', "PC_RATIO": f'{P["cost_ratio"]["ratio"]:.2f}',
    "PTOK_RATIO": f'{P["token_ratio"]["ratio"]:.2f}', "PC_N": str(P["cost_ratio"]["n"]), "PTOK_N": str(P["token_ratio"]["n"]),
    "RV_T_F": f'{rv_t["Fable 5.2"]["median"] / 60000:.0f}', "RV_T_O": f'{rv_t["Opus 5.5"]["median"] / 60000:.0f}',
    "RV_C_F": f'{rv_c["Fable 5.2"]["median"]:.2f}', "RV_C_O": f'{rv_c["Opus 5.5"]["median"]:.2f}',
    "IMP_MAJ_RR": ci(rr(I["MAJORs per review"])),
    "PROP_ROWS": "".join([
        prop_row("Unit drew a NOT-MERGEABLE verdict", I["unit drew a NOT-MERGEABLE verdict [all]"]),
        prop_row("Unit drew any MAJOR", I["unit drew any MAJOR [all]"]),
        prop_row("Hosted CI red on the branch's own row, first pushed head", ci_red,
                 f'unknown or not run: {ci_red["unknown_or_not_run"]["Fable 5.2"]} Fable, {ci_red["unknown_or_not_run"]["Opus 5.5"]} Opus'),
        prop_row("At least one SILENT spec deviation", I["at least one SILENT spec deviation"]),
    ]),
    "RUB_ROWS": rub_row("idiom", "Rubric: idiom and structure") + rub_row("tests", "Rubric: test quality") + rub_row("docs", "Rubric: doc and comment honesty"),
    "C_OO": f'{c4["Opus impl / Opus rev"]["major_per_review"]:.2f}', "N_OO": str(c4["Opus impl / Opus rev"]["n"]),
    "C_OF": f'{c4["Opus impl / Fable rev"]["major_per_review"]:.2f}', "N_OF": str(c4["Opus impl / Fable rev"]["n"]),
    "C_FO": f'{c4["Fable impl / Opus rev"]["major_per_review"]:.2f}', "N_FO": str(c4["Fable impl / Opus rev"]["n"]),
    "C_FF": f'{c4["Fable impl / Fable rev"]["major_per_review"]:.2f}', "N_FF": str(c4["Fable impl / Fable rev"]["n"]),
    "INTER": f'{inter["ratio"]:.1f}', "INTER_LO": f'{inter["lo"]:.1f}', "INTER_HI": f'{inter["hi"]:.1f}',
    "DF_F": str(dfp["fable"]), "DF_O": str(dfp["opus"]), "DF_LO": pct(dfp["lo"]), "DF_HI": pct(dfp["hi"]),
    "DF_N2": str(df["5.2 / two designers"]["n"]), "DF_N3": str(df["5.2 / three designers"]["n"]),
    "DRAWS_N": str(R["draws"]["n"]), "DRAWS_E": str(R["draws"]["even"]),
    "ORCH_F": str(pop["orchestrator"]["Fable 5.2"]), "ORCH_O": str(pop["orchestrator"]["Opus 5.5"]),
}
for k, v in subs.items():
    html = html.replace("{{" + k + "}}", v)
assert "{{" not in html, html[html.index("{{"):html.index("{{") + 40]
open(f"{HERE}/report.html", "w").write(html)
print("wrote report.html", len(html))

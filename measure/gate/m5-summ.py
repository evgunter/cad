import glob,os,json,re,collections,statistics as st
M=os.path.expanduser('~/.local/share/cad-work/gate/walk')
pop=json.load(open(M+'/population.json'))
dear={t for b,t in pop['dear']}; floor={t for b,t in pop['m5_floor']}; m3={t for b,t in pop['m3']}
asks=[]
for f in glob.glob(M+'/log-m5/*.log'):
    pend=None
    for l in open(f,errors='replace'):
        p=l.rstrip('\n').split('\t')
        if p[0]=='M5': pend=p
        elif p[0]=='WALK' and pend:
            m=dict(kv.split('=',1) for kv in pend[2:]); w=dict(kv.split('=',1) for kv in p[2:])
            asks.append((p[1],m,w)); pend=None
def group(t,w):
    if t in dear: return 'dear'
    if t in floor and w['first_depth']=='40' and w['verdict']!='Crosses': return 'floor'
    return None
for g in ['dear','floor']:
    sel=[(t,m,w) for t,m,w in asks if group(t,w)==g]
    reasons=collections.Counter(); same=collections.Counter()
    for t,m,w in sel:
        for k,(n,s) in eval(m['split_reasons'].replace('{','{').replace(')',')')).items():
            reasons[k]+=n; same[k]+=s
    tot=sum(float(w['secs']) for t,m,w in sel)
    sv=sum(float(m['m5b_after'].split(':')[1]) for t,m,w in sel); ss=sum(int(m['m5b_after'].split(':')[0]) for t,m,w in sel)
    pr=[m['m5b_pairs'] for t,m,w in sel]
    depths=[d for t,m,w in sel for d in eval(m['m5b_depths'])]
    print(f'== {g}: {len(sel)} asks, {tot:.1f} s')
    for k,n in reasons.most_common(): print(f'   {n:7d} splits  {k:60s} same-as-parent {same[k]}')
    st_s=sum(float(m['strict_after'].split(':')[1]) for t,m,w in sel); st_n=sum(int(m['strict_pairs']) for t,m,w in sel)
    st_d=[d for t,m,w in sel for d in eval(m['strict_depths'])]
    print(f'   M5b strict (Structural part, or margin/separation wholly in band): {st_n} refusing pairs; depth median {st.median(st_d) if st_d else "-"}; spent below it {st_s:.1f} s = {st_s/tot*100 if tot else 0:.1f}%')
    print(f'   M5b repeat (every split below repeats its parent exactly): refusing pairs reaching a fixed reason: {sum(int(x.split("/")[0]) for x in pr)} of {sum(int(x.split("/")[1]) for x in pr)}; depth median {st.median(depths) if depths else "-"} (min {min(depths) if depths else "-"}, max {max(depths) if depths else "-"}); spent below it: {ss} splits, {sv:.1f} s = {sv/tot*100 if tot else 0:.1f}% of the asks')
    if g=='dear':
        for t,m,w in sorted(sel,key=lambda x:-float(x[2]['secs'])):
            print(f'    {float(w["secs"]):6.1f}s {t[:50]:50s} fixed {m["m5b_pairs"]} depths {m["m5b_depths"][:60]} repeat-saves {m["m5b_after"]} strict {m["strict_pairs"]} {m["strict_depths"][:40]} strict-saves {m["strict_after"]}')
cr=[(t,m,w) for t,m,w in asks if t in m3 and w['verdict']=='Crosses' and int(w['refusing_pairs'])>0]
print('== M5c:',len(cr),'crossings after a refusal')
c=collections.Counter(); ds=[]
for t,m,w in cr:
    d=dict(kv.split('=') for kv in m['m5c'].split(',')) if m['m5c']!='none' else {}
    ds.append(int(d.get('depth',-1))); c[(d.get('fresh'),d.get('touching_split_after_first_refusal_in_lineage'))]+=1
print('   depths',sorted(ds)); print('   (fresh subtree root, touching split after first refusal in lineage):',dict(c))

import re,json,sys
S=sys.argv[1]
# blind label -> model, from the analysis-branch ledgers
MAP={'u6972':{'A':'Opus','B':'Fable'},'box-region':{'A':'Fable','B':'Opus'},'anchored-conic':{'A':'Fable','B':'Opus'}}
def sev(s):
    s2=s.lower()
    if re.search(r'not raised|never (mentioned|raised)|^not mentioned|no defect raised|^—$|^-$|^$',s2.strip()): return None
    for k in ['major','minor','note','style']:
        if re.search(r'\b'+k,s2): return k.upper()
    return 'UNLABELLED'
out={}
for u in MAP:
    L=open(f'{S}/{u}/blind-coding.md').read().split('\n')
    hdr=None;rows=[]
    for l in L:
        if l.startswith('|') and ('Code' in l) and hdr is None:
            hdr=[c.strip() for c in l.strip('|').split('|')]; continue
        if hdr and l.startswith('|') and re.match(r'\|\s*\d+[a-z]?\s*\|',l):
            c=[x.strip() for x in l.strip().strip('|').split('|')]
            if len(c)<len(hdr): continue
            d=dict(zip(hdr,c))
            ia=[i for i,h in enumerate(hdr) if h.startswith('A')][0]; ib=[i for i,h in enumerate(hdr) if h.startswith('B')][0]
            code=d['Code']
            if 'UNILATERAL-A' in code or 'UNILATERAL A' in code: src='A'
            elif 'UNILATERAL-B' in code or 'UNILATERAL B' in code: src='B'
            elif 'BILATERAL' in code: src='both'
            else: src='?'
            rows.append(dict(row=c[0],src=src,src_model={'A':MAP[u]['A'],'B':MAP[u]['B'],'both':'both'}.get(src,'?'),
                             label_A=sev(c[ia]),label_B=sev(c[ib]),code=code[:80]))
        elif hdr and rows and not l.startswith('|') and l.strip()=='' : pass
    for r in rows:
        r['label_opus']=r['label_A'] if MAP[u]['A']=='Opus' else r['label_B']
        r['label_fable']=r['label_B'] if MAP[u]['A']=='Opus' else r['label_A']
    out[u]=rows
json.dump(out,open(f'{S}/key.json','w'),indent=1)
for u,rs in out.items():
    print(u,len(rs))
    for r in rs: print('  ',r['row'],r['src_model'],'O:',r['label_opus'],'F:',r['label_fable'],'|',r['code'][:50])

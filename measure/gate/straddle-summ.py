import glob,os,json,collections
def parse(k):
    t=k.split('|'); share=t[0]; t=t[1:]
    def take(t):
        if not t: return '',t
        if t[0].startswith('unpresented:'): return t[0]+'|'+t[1],t[2:]
        return t[0],t[1:]
    anc,t=take(t); first,t=take(t)
    return share,anc,first
M=os.path.expanduser('~/.local/share/cad-work/gate/walk')
pop=json.load(open(M+'/population.json')); dear={t for b,t in pop['dear']}
for grp in ['dear','floor']:
    tot=collections.Counter(); sub=collections.defaultdict(lambda:[0,0.0]); mins=[]; secs=0; n=0
    for f in glob.glob(M+'/log-m6/*.log'):
        for l in open(f,errors='replace'):
            p=l.rstrip('\n').split('\t')
            if p[0]!='STRADDLE': continue
            if (p[1] in dear)!=(grp=='dear'): continue
            d=dict(kv.split('=',1) for kv in p[2:]); n+=1; secs+=float(d['secs'])
            for item in eval(d['splits']):
                k,v=item.rsplit('=',1); tot[k]+=int(v)
            mins+= [float(x) for x in eval(d['share_nothing_min_hi'])]
            for k,(a,b) in eval(d['refusing_spend_by_first_touching_ancestor']).items(): sub[k][0]+=a; sub[k][1]+=b
    T=sum(tot.values())
    print(f'== {grp}: {n} refusing/crossing asks, {secs:.1f} s; straddling distance splits {T}')
    agg=collections.Counter()
    for k,v in tot.items():
        share,anc,first=parse(k)
        agg[(share, 'descends from a failed touching pair' if anc!='no-touching-ancestor' else 'no touching ancestor')]+=v
    for k,v in agg.most_common(): print(f'   {v:8d} ({v/T*100:5.1f}%) {k}')
    byanc=collections.Counter()
    for k,v in tot.items():
        share,anc,first=parse(k)
        if anc!='no-touching-ancestor': byanc[anc]+=v
    print('   by nearest touching ancestor reason:'); [print(f'      {v:8d} {k}') for k,v in byanc.most_common(8)]
    if mins: print('   share-nothing face pairs: least straddling sep hi: n',len(mins),'min %.3e median %.3e max %.3e'%(min(mins),sorted(mins)[len(mins)//2],max(mins)))
    S=sum(v[1] for v in sub.values())
    print('   refusing pairs\' spend by FIRST touching ancestor:'); [print(f'      {v[1]:9.1f} s ({v[1]/S*100 if S else 0:5.1f}%) {v[0]:8d} splits {k}') for k,v in sorted(sub.items(),key=lambda kv:-kv[1][1])[:8]]

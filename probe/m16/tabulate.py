import sys,collections,re
def norm(c):
    return re.sub(r'@\d+','',c)
def load(files,ctxre=None):
    rows=[]
    for f in files:
        for l in open(f, errors='replace'):
            p=l.rstrip('\n').split('\t')
            if p[0]!='ASK' or len(p)<18: continue
            if ctxre and not re.search(ctxre,p[1]): continue
            rows.append(dict(ctx=p[1],pred=p[2],today=p[3],vlo=float(p[4]),vhi=float(p[5]),zero=p[6]=='true',plain_ns=int(p[7]),ladder_ns=int(p[8]),pc=p[9],size=p[10],plo=p[12],phi=p[13],ec=p[14],early_ns=int(p[15]),members=p[16],wit=p[17]))
    return rows
def final(r):
    if r['pc'] in ('Positive','Negative','Zero'): return 'plain:'+r['pc']
    if r['pc']=='EMPTY': return 'EMPTY(plain)'
    if r['ec'] in ('Positive','Negative','Zero'): return 'early:'+r['ec']
    if r['ec']=='EMPTY': return 'EMPTY(early)'
    return None
def table(rows,title,top=10):
    print('==',title,'asks not signed by the value:',len(rows))
    by=collections.Counter(r['today'] for r in rows); print('  today:',dict(by))
    und=[r for r in rows if r['today']=='undecided']
    for name,grp in (('UNDECIDED today',und),('value zero-in-band today',[r for r in rows if r['today']=='value-zero-in-band']),('ladder-zero today',[r for r in rows if r['today']=='ladder-zero'])):
        if not grp: continue
        print('  --',name,len(grp))
        print('     zero plain form:',sum(r['zero'] for r in grp))
        pc=collections.Counter(norm(r['pc']) for r in grp); print('     plain:',dict(pc.most_common()))
        ec=collections.Counter(norm(r['ec']) for r in grp if r['ec']!='-'); print('     early (where plain did not decide):',dict(ec.most_common()))
        fin=collections.Counter(final(r) or 'none' for r in grp); print('     reading (plain, else early):',dict(fin.most_common()))
        pn=sum(r['plain_ns'] for r in grp); ln=sum(r['ladder_ns'] for r in grp); en=sum(r['early_ns'] for r in grp)
        print('     plain build %.1f ms total, %.1f us/ask; ladder (after plain) %.1f ms, %.1f us/ask; early build where taken %.1f ms'%(pn/1e6,pn/1e3/len(grp),ln/1e6,ln/1e3/len(grp),en/1e6))
    print('  -- by predicate (undecided today), top',top)
    bp=collections.defaultdict(list)
    for r in und: bp[r['pred']].append(r)
    for pred,g in sorted(bp.items(),key=lambda kv:-len(kv[1]))[:top]:
        fin=collections.Counter((final(r) or ('declines' if r['pc'].startswith('declines') and (r['ec']=='-' or r['ec'].startswith('declines')) else 'straddles/sliver')) for r in g)
        print('     %-34s %5d  %s'%(pred,len(g),dict(fin.most_common())))
    emp=[r for r in rows if r['pc']=='EMPTY' or r['ec']=='EMPTY']
    print('  EMPTY intersections:',len(emp))
    for r in emp[:20]: print('    ',r['ctx'],r['pred'],r['today'],'value',(r['vlo'],r['vhi']),'plain',r['pc'],(r['plo'],r['phi']),'early',r['ec'])
    # soundness
    n=bad=na=0; badrows=[]
    for r in rows:
        if not r['members']: continue
        fin=final(r)
        if not fin or fin.startswith('EMPTY'): continue
        want=fin.split(':')[1]
        ms=r['members'].split(',')
        if any(m.startswith('NA') for m in ms): na+=1; continue
        n+=1
        if any(m.split(':')[0]!=want for m in ms): bad+=1; badrows.append((r['ctx'],r['pred'],r['today'],fin,(r['plo'],r['phi']),r['members']))
    print('  soundness: sampled',n,'asks x5 members; differing',bad,'; not evaluable',na)
    seen=collections.Counter()
    for b in badrows:
        k=(b[1],b[3]); seen[k]+=1
        if seen[k]<=2: print('    DIFF',b)
    print('    DIFF kinds:',dict(seen))
if __name__=='__main__':
    table(load(sys.argv[2:], sys.argv[1] if sys.argv[1]!='-' else None), sys.argv[1])

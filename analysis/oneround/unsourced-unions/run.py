import subprocess,sys,os
sys.path.insert(0,'/tmp/claude-0/mut'); from mutants import M
os.chdir('/home/user/mut'); env=dict(os.environ,CARGO_TARGET_DIR='/home/user/tgt-mut')
only=sys.argv[1:]
for (i,f,old,new,pkg,flt) in M:
    if only and i not in only: continue
    s=open(f).read(); n=s.count(old)
    if n<1: print(i,'NOMATCH',flush=True); continue
    open(f,'w').write(s.replace(old,new))
    cmd=f"cargo nextest run {pkg} --no-fail-fast -E '{flt}' 2>&1"
    out=subprocess.run(cmd,shell=True,capture_output=True,text=True,env=env).stdout
    open(f'/tmp/claude-0/mut/{i}.out','w').write(out)
    fails=sorted(set(l.split()[-1] for l in out.splitlines() if l.strip().startswith('FAIL [')))
    summ=[l for l in out.splitlines() if 'Summary' in l or 'error[' in l or 'error:' in l][:3]
    print(i,f'matches={n}','KILLED' if fails else 'survived',fails,summ,flush=True)
    subprocess.run(['git','checkout','--',f])

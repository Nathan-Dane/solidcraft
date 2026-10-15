#!/usr/bin/env python3
"""Review page for every icon: where it sits in the app, accept it as final, leave notes.

  python3 assets/icons/ui/design/tools/icon_status.py --serve        # review page on http://127.0.0.1:8770, saves design/review.json
  python3 assets/icons/ui/design/tools/icon_status.py                # writes target/icon-status.html (static: choices stay in the browser)
  python3 assets/icons/ui/design/tools/icon_status.py --notes        # icons that have notes and are not final (the work list)
  python3 assets/icons/ui/design/tools/icon_status.py --csv          # also rewrites design/inventory.csv
  python3 assets/icons/ui/design/tools/icon_status.py --summary      # counts only

Standard library only. The page is grouped by toolbar tab and panel (and, for icons without a toolbar
button, by where the app shows them), with a slim vertical marker where a group starts. Each card has a
check mark (accept as final) and a note button (the text area unfolds over the page, so it takes no
space). `design/review.json` keeps the choices:

  {"final": {"<icon id>": "<sha256 of its master svg(s) when it was accepted>"}, "notes": {"<icon id>": "text"}}

A final icon must not change: `lint_icons.py` fails when its master differs from the stored hash, and
`tools/author/gen.py` skips it. Un-accept it on the page to work on it again.
Icon ids come from the command registry (`.icon("id")`, `.at("TAB", "PANEL")`), the names the UI passes to
`icons::paint`, plus every master and alias. The status of an id (done / stand-in / placeholder) is kept for
--summary and inventory.csv.
"""
import sys, os, re, glob, html, json, hashlib, collections, tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
D = os.path.dirname(HERE)                       # .../assets/icons/ui/design
ROOT = os.path.abspath(os.path.join(D, '..', '..', '..', '..'))
REVIEW = D + '/review.json'
args = sys.argv[1:]
out_path = args[args.index('--out') + 1] if '--out' in args else os.path.join(ROOT, 'target', 'icon-status.html')

def master_hash(master):
    h = hashlib.sha256()
    for f in sorted(glob.glob(D + '/svg/*/' + master + '.svg')):
        h.update(os.path.relpath(f, D).encode()); h.update(open(f, 'rb').read())
    return h.hexdigest()
def load_review():
    try: r = json.load(open(REVIEW))
    except (OSError, ValueError): r = {}
    fin = r.get('final', {})
    if isinstance(fin, list): fin = {i: '' for i in fin}
    return {'final': dict(fin), 'notes': {k: v for k, v in r.get('notes', {}).items() if v.strip()}}

def scan():
    """Everything derived from the repo: rows, summary."""
    pal = tomllib.load(open(D + '/palette.toml', 'rb'))
    alias_raw = tomllib.load(open(D + '/aliases.toml', 'rb')).get('alias', {})
    stand_in = set(tomllib.load(open(D + '/status.toml', 'rb')).get('stand_in', []))
    masters = {}
    for f in sorted(glob.glob(D + '/svg/*/*.svg')):
        fam = os.path.basename(os.path.dirname(f)); n = os.path.basename(f)[:-4]
        if fam != 'small': masters[n] = (fam, f)
    uses = collections.defaultdict(list)   # id -> command ids and "ui:<file>"
    where = collections.defaultdict(list)  # id -> [(tab, panel, command)]
    for f in glob.glob(ROOT + '/crates/engine/src/**/*.rs', recursive=True):
        t = open(f).read()
        ms = list(re.finditer(r'(?:CommandSpec::new|planned!)\(\s*"([^"]+)"\s*,\s*"([^"]*)"', t))
        for i, m in enumerate(ms):
            end = ms[i + 1].start() if i + 1 < len(ms) else len(t)
            chunk = '\n'.join(t[m.end():end].split('\n')[:14])
            ic = re.search(r'\.icon\(\s*"([^"]+)"', chunk)
            if ic:
                uses[ic.group(1)].append(m.group(1))
                at = re.search(r'\.at\(\s*"([^"]+)"\s*,\s*"([^"]+)"', chunk)
                if at: where[ic.group(1)].append((at.group(1), at.group(2), m.group(1)))
    for f in glob.glob(ROOT + '/crates/ui-egui/src/*.rs'):
        t = open(f).read()
        for call in re.finditer(r'icons::paint[a-z_]*\((.*?);', t, re.S):
            for name in re.findall(r'"([a-z][a-z0-9_]*)"', call.group(1)):
                uses[name].append('ui:' + os.path.basename(f))
    mk = ROOT + '/assets/icons/ui/source/markers.rs'
    if os.path.exists(mk):
        for name in re.findall(r'=> "(c_[a-z_]+)"', open(mk).read()): uses[name].append('ui:sketch constraint glyph')
    def alias_of(n):
        v = alias_raw.get(n)
        if v is None: return n, ''
        return (v['master'], v.get('accent', '')) if isinstance(v, dict) else (v, '')
    targets = {alias_of(a)[0] for a in alias_raw}
    rows = []
    for n in sorted(set(uses) | set(masters) | set(alias_raw)):
        m, acc = alias_of(n)
        if m in masters and n != 'placeholder': status = 'stand-in' if m in stand_in else 'done'
        else: status, m = 'placeholder', 'placeholder'
        cmds = [u for u in uses.get(n, []) if not u.startswith('ui:')]
        rows.append(dict(id=n, status=status, master=m, accent=acc, commands=len(cmds), example=cmds[0] if cmds else '',
                         ui=sorted({u[3:] for u in uses.get(n, []) if u.startswith('ui:')}), unused=not uses.get(n),
                         where=where.get(n, []), alias_target=n in targets))
    cnt = collections.Counter(r['status'] for r in rows if r['id'] != 'placeholder')
    cmd_cnt = collections.Counter()
    for r in rows:
        if r['id'] != 'placeholder': cmd_cnt[r['status']] += r['commands']
    summary = '%d icon ids: %d done, %d stand-in, %d placeholder (%d commands: %d / %d / %d).' % (
        sum(cnt.values()), cnt['done'], cnt['stand-in'], cnt['placeholder'], sum(cmd_cnt.values()), cmd_cnt['done'], cmd_cnt['stand-in'], cmd_cnt['placeholder'])
    return rows, masters, pal, summary

rows, masters, pal, summary = scan()
roles = pal['roles']; accents = pal.get('accents', {})
print(summary)
if '--summary' in args: sys.exit(0)

if '--notes' in args:
    rv = load_review(); n = 0
    for r in rows:
        note = rv['notes'].get(r['id'])
        if note and r['id'] not in rv['final']:
            n += 1; print('\n%s  (master %s%s)\n  %s' % (r['id'], r['master'], ', ' + r['accent'] + ' accent' if r['accent'] else '', note.replace('\n', '\n  ')))
    print('\n%d icon(s) with notes to work on; %d final of %d.' % (n, len(rv['final']), len([r for r in rows if r['id'] != 'placeholder'])))
    sys.exit(0)

if '--csv' in args:
    with open(D + '/inventory.csv', 'w') as f:
        f.write('icon,status,master,accent,commands,example_command,used_by_ui\n')
        for r in rows:
            if r['id'] == 'placeholder': continue
            f.write('%s,%s,%s,%s,%d,%s,"%s"\n' % (r['id'], r['status'], r['master'] if r['status'] != 'placeholder' else '', r['accent'], r['commands'], r['example'], ';'.join(r['ui'])))
    print('wrote', os.path.relpath(D + '/inventory.csv', ROOT))

# --- grouping: toolbar tab / panel, else where the app shows the icon -------------------------
TABS = ['SOLID', 'SURFACE', 'MESH', 'SHEET METAL', 'PLASTIC', 'RENDER', 'SKETCH']
PANELS = ['CREATE', 'MODIFY', 'ASSEMBLE', 'POSITION', 'CONFIGURE', 'CONSTRUCT', 'CONSTRAINTS', 'SETUP', 'SCENE', 'IMAGE', 'INSPECT', 'INSERT', 'SELECT', 'FINISH SKETCH']
UI_NAMES = {'browser.rs': 'Browser', 'timeline.rs': 'Timeline', 'timeline_left.rs': 'Timeline', 'context_menu.rs': 'Context menus', 'update.rs': 'Update banner',
            'sketch constraint glyph': 'Sketch constraint glyphs'}
def rank(l, v): return l.index(v) if v in l else len(l)
def group_of(r):
    if r['where']:
        tab, panel, _ = min(r['where'], key=lambda w: (rank(TABS, w[0]), rank(PANELS, w[1])))
        return (rank(TABS, tab), rank(PANELS, panel), tab, panel)
    if r['ui']:
        names = sorted({UI_NAMES.get(u, u.replace('.rs', '').replace('_', ' ').title()) for u in r['ui']})
        return (50, 0, 'IN THE APP', ' · '.join(names))
    if r['commands']: return (60, 0, 'MENUS & PALETTE', 'commands without a toolbar button')
    return (70, 0, 'UNUSED', 'no app use yet')

def colours(theme, accent):
    m = {r['light'].lower(): r[theme] for r in roles.values()}
    for rn, v in accents.get(accent, {}).items():
        m[roles[rn]['light'].lower()] = v[theme]
    return m, roles['line'][theme]
def themed(svg, theme, accent):
    m, line = colours(theme, accent)
    svg = re.sub(r'#[0-9a-fA-F]{6}', lambda x: m.get(x.group(0).lower(), x.group(0)), svg)
    return svg.replace('currentColor', line)

def build_page(serving):
    rows, masters, pal, summary = scan()
    rv = load_review()
    shown = [r for r in rows if r['id'] != 'placeholder' and not (r['unused'] and r['alias_target'] and r['id'] in masters)]
    shown.sort(key=lambda r: (group_of(r), r['id']))
    cards = []; last = None; tabs_seen = []
    for r in shown:
        g = group_of(r)
        if last is None or g[:2] != last[:2]:
            first_of_tab = last is None or g[2] != last[2]
            if first_of_tab and g[2] not in tabs_seen: tabs_seen.append(g[2])
            txt = (g[2] + ' ▸ ' + g[3]) if first_of_tab else g[3]
            cards.append('<i class="gm%s" data-tab="%s" title="%s">%s</i>' % (' tab' if first_of_tab else '', html.escape(g[2]), html.escape(g[2] + ' ▸ ' + g[3]), html.escape(txt)))
        last = g
        svg_f = open(masters[r['master']][1]).read()
        arts = {}
        for theme in ('light', 'dark'):
            arts[theme] = themed(svg_f, theme, r['accent'])
        small = glob.glob(D + '/svg/small/' + r['master'] + '.svg')
        sm = {t: themed(open(small[0]).read(), t, r['accent']) for t in ('light', 'dark')} if small else None
        def art(cls, s): return '<span class="%s">%s</span>' % (cls, s)
        inner = art('al', arts['light']) + art('ad', arts['dark'])
        if sm: inner += art('sl', sm['light']) + art('sd', sm['dark'])
        src = ('= ' + r['master']) if r['master'] != r['id'] else ''
        tip = '; '.join('%s ▸ %s: %s' % w for w in r['where']) or ', '.join(r['ui']) or ('%d command(s)' % r['commands'])
        extra = ((' <i class="acc">%s accent</i>' % r['accent']) if r['accent'] else '') + (' <i class="st">%s</i>' % r['status'] if r['status'] != 'done' else '')
        cards.append('<div class="card" data-id="%s" data-tab="%s" data-k="%s" title="%s"><div class="art%s">%s</div><div class="meta"><b>%s</b>%s%s<br><small>%s</small></div>'
                     '<button class="ck" title="Accept as final">✓</button><button class="nt" title="Notes">✎</button></div>' % (
            html.escape(r['id']), html.escape(g[2]), html.escape(r['id'] + ' ' + r['example'] + ' ' + r['accent'] + ' ' + g[3].lower()), html.escape(tip),
            ' hs' if sm else '', inner, html.escape(r['id']), ('<span class="src"> ' + html.escape(src) + '</span>') if src else '', extra, html.escape(r['example'] or ', '.join(r['ui']) or '')))
    tabchips = ''.join('<button class="chip" data-tab="%s">%s</button>' % (html.escape(t), html.escape(t.title())) for t in tabs_seen)
    state = json.dumps({'final': sorted(rv['final']), 'notes': rv['notes'], 'serving': serving})
    return TEMPLATE.replace('@@SUMMARY@@', html.escape(summary)).replace('@@CARDS@@', ''.join(cards)).replace('@@TABS@@', tabchips).replace('@@STATE@@', state)

TEMPLATE = r'''<!doctype html><meta charset=utf-8><title>SolidCraft icon review</title>
<style>
:root{--bg:#f4f6f9;--fg:#1d2733;--card:#fff;--line:#d5dbe3;--mut:#6b7a8a;--sz:48px}
body.dark{--bg:#1e2530;--fg:#dbe4ee;--card:#262f3c;--line:#3a4657;--mut:#8a9db0}
body{font:12px system-ui,sans-serif;margin:0;background:var(--bg);color:var(--fg)}
header{position:sticky;top:0;background:var(--card);border-bottom:1px solid var(--line);padding:6px 12px;z-index:5;display:flex;flex-wrap:wrap;gap:4px 10px;align-items:center}
h1{font-size:14px;margin:0}.sum{color:var(--mut)}
button,input{font:inherit;color:inherit;padding:2px 8px;border:1px solid var(--line);border-radius:6px;background:var(--card);cursor:pointer}
button.on{background:#236ead;color:#fff;border-color:#236ead}
.grp{display:flex;gap:3px;align-items:center}.grp small{color:var(--mut)}
main{display:flex;flex-wrap:wrap;gap:6px;padding:8px 12px;align-items:stretch}
.gm{flex:0 0 11px;writing-mode:vertical-rl;transform:rotate(180deg);font:600 8px/11px system-ui;letter-spacing:.04em;color:var(--mut);background:var(--line);border-radius:3px;overflow:hidden;white-space:nowrap;text-overflow:ellipsis;text-transform:uppercase;font-style:normal;max-height:120px;align-self:stretch}
.gm.tab{background:#236ead;color:#fff}
.card{position:relative;flex:0 0 118px;background:var(--card);border:1px solid var(--line);border-radius:7px}
.art{display:flex;justify-content:center;align-items:center;padding:8px 0 2px;min-height:calc(var(--sz) + 10px)}
.art svg{width:var(--sz);height:var(--sz)}
.art .ad,.art .sd,.art .sl{display:none}
body.dark .art .al{display:none}body.dark .art .ad{display:inline}
body.small .art.hs .al,body.small .art.hs .ad{display:none}
body.small:not(.dark) .art.hs .sl{display:inline}body.small.dark .art.hs .sd{display:inline}
.meta{padding:2px 6px 5px;line-height:1.35;word-break:break-all}.meta small{color:var(--mut);font-size:10px}
.src,.acc,.st{font-size:10px;color:#a0500a;font-style:normal}.src{color:var(--mut)}.st{color:#c0392b}
.ck,.nt{position:absolute;top:3px;width:20px;height:20px;padding:0;line-height:18px;border-radius:50%;font-size:12px;color:var(--mut);opacity:.55}
.ck{right:3px}.nt{left:3px;border-radius:5px}
.card:hover .ck,.card:hover .nt{opacity:1}
.card.final{border-color:#2e8b57}.card.final .art,.card.final .meta{opacity:.6}
.card.final .ck{background:#2e8b57;color:#fff;border-color:#2e8b57;opacity:1}
.card.final .nt{display:none}
.card.hasnote{border-color:#d9a21b}.card.hasnote .nt{background:#d9a21b;color:#fff;border-color:#d9a21b;opacity:1}
.pop{position:absolute;left:0;top:100%;z-index:9;width:250px;background:var(--card);border:1px solid #236ead;border-radius:7px;padding:6px;box-shadow:0 6px 18px #0004;display:none}
.card.open .pop{display:block}.card.open{z-index:9}
.pop textarea{width:100%;box-sizing:border-box;height:90px;font:12px system-ui;color:inherit;background:var(--bg);border:1px solid var(--line);border-radius:5px}
#msg{color:#c0392b}
</style>
<header><h1>Icon review</h1><span class=sum id=cnt></span><span id=msg></span>
<span class=grp><button data-f=all class=on>All</button><button data-f=open>Open</button><button data-f=notes>Notes</button><button data-f=final>Final</button></span>
<span class=grp><button id=tab-all class="chip on">Everything</button>@@TABS@@</span>
<span class=grp><button id=dk>Dark</button><small>size</small><button class=sz data-s=16>16</button><button class=sz data-s=24>24</button><button class="sz on" data-s=48>48</button><button class=sz data-s=64>64</button></span>
<input id=q placeholder="filter id, command, panel"><button id=exp title="Download review.json">Export</button>
<span class=sum>@@SUMMARY@@</span></header>
<main>@@CARDS@@</main>
<script>
const S=@@STATE@@;let f='all',tab='';const q=document.getElementById('q'),msg=document.getElementById('msg');
const fin=new Set(S.final),notes=S.notes||{};
if(!S.serving){try{const l=JSON.parse(localStorage.getItem('icon-review')||'null');if(l){fin.clear();l.final.forEach(i=>fin.add(i));Object.assign(notes,l.notes)}}catch(e){}}
function body(){return JSON.stringify({final:[...fin].sort(),notes:Object.fromEntries(Object.entries(notes).filter(e=>e[1].trim()))})}
let t;function save(){clearTimeout(t);t=setTimeout(async()=>{try{localStorage.setItem('icon-review',body())}catch(e){}
 if(!S.serving){msg.textContent='not saved to disk (run with --serve, or Export)';return}
 try{const r=await fetch('/api/review',{method:'POST',body:body()});msg.textContent=r.ok?'':'save failed'}catch(e){msg.textContent='save failed: server not running'}},250)}
const cards=[...document.querySelectorAll('.card')],marks=[...document.querySelectorAll('.gm')];
function paint(c){const id=c.dataset.id;c.classList.toggle('final',fin.has(id));c.classList.toggle('hasnote',!!(notes[id]||'').trim()&&!fin.has(id))}
function run(){let n=0,nf=0,nn=0;const vis=new Set();
 cards.forEach(c=>{const id=c.dataset.id,isf=fin.has(id),hn=!!(notes[id]||'').trim();paint(c);if(isf)nf++;else if(hn)nn++;
  const ok=(f=='all'||(f=='open'&&!isf)||(f=='notes'&&hn&&!isf)||(f=='final'&&isf))&&(!tab||c.dataset.tab==tab)&&c.dataset.k.includes(q.value.toLowerCase());
  c.style.display=ok?'':'none';if(ok)vis.add(c)});
 // group markers: show a marker only when a card follows it before the next marker
 marks.forEach(m=>{let e=m.nextElementSibling,any=false;while(e&&!e.classList.contains('gm')){if(e.style.display!='none')any=true;e=e.nextElementSibling}m.style.display=any?'':'none'});
 document.getElementById('cnt').textContent=nf+' final · '+nn+' with notes · '+(cards.length-nf-nn)+' open of '+cards.length}
cards.forEach(c=>{const id=c.dataset.id;
 const pop=document.createElement('div');pop.className='pop';pop.innerHTML='<textarea placeholder="What to change in this icon…"></textarea>';c.appendChild(pop);
 const ta=pop.querySelector('textarea');ta.value=notes[id]||'';
 ta.oninput=()=>{notes[id]=ta.value;paint(c);run();save()};
 c.querySelector('.ck').onclick=()=>{fin.has(id)?fin.delete(id):fin.add(id);c.classList.remove('open');run();save()};
 c.querySelector('.nt').onclick=e=>{const o=!c.classList.contains('open');cards.forEach(x=>x.classList.remove('open'));c.classList.toggle('open',o);if(o)ta.focus();e.stopPropagation()};
 pop.onclick=e=>e.stopPropagation()});
document.addEventListener('click',()=>cards.forEach(x=>x.classList.remove('open')));
document.addEventListener('keydown',e=>{if(e.key=='Escape')cards.forEach(x=>x.classList.remove('open'))});
document.querySelectorAll('[data-f]').forEach(b=>b.onclick=()=>{f=b.dataset.f;document.querySelectorAll('[data-f]').forEach(x=>x.classList.toggle('on',x==b));run()});
document.querySelectorAll('.chip').forEach(b=>b.onclick=()=>{tab=b.dataset.tab||'';document.querySelectorAll('.chip').forEach(x=>x.classList.toggle('on',x==b));run()});
document.querySelectorAll('.sz').forEach(b=>b.onclick=()=>{const s=+b.dataset.s;document.documentElement.style.setProperty('--sz',s+'px');document.body.classList.toggle('small',s<=24);document.querySelectorAll('.sz').forEach(x=>x.classList.toggle('on',x==b))});
document.getElementById('dk').onclick=function(){document.body.classList.toggle('dark');this.classList.toggle('on')};
document.getElementById('exp').onclick=()=>{const a=document.createElement('a');a.href=URL.createObjectURL(new Blob([body()],{type:'application/json'}));a.download='review.json';a.click()};
q.oninput=run;run();
</script>'''

if '--serve' in args:
    import http.server
    port = int(args[args.index('--serve') + 1]) if len(args) > args.index('--serve') + 1 and args[args.index('--serve') + 1].isdigit() else 8770
    alias_raw = tomllib.load(open(D + '/aliases.toml', 'rb')).get('alias', {})
    def master_of(i):
        v = alias_raw.get(i)
        return (v['master'] if isinstance(v, dict) else v) if v else i
    class H(http.server.BaseHTTPRequestHandler):
        def log_message(self, *a): pass
        def _send(self, code, body, ctype):
            b = body.encode(); self.send_response(code); self.send_header('Content-Type', ctype); self.send_header('Content-Length', str(len(b))); self.send_header('Cache-Control', 'no-store'); self.end_headers(); self.wfile.write(b)
        def do_GET(self):
            if self.path.split('?')[0] in ('/', '/index.html'): self._send(200, build_page(True), 'text/html; charset=utf-8')
            else: self._send(404, 'not found', 'text/plain')
        def do_POST(self):
            if self.path != '/api/review': return self._send(404, 'not found', 'text/plain')
            try:
                n = int(self.headers.get('Content-Length', '0'))
                if n > 4_000_000: raise ValueError('too large')
                inc = json.loads(self.rfile.read(n))
                old = load_review()['final']
                fin = {i: (old.get(i) or master_hash(master_of(i))) for i in inc.get('final', []) if re.fullmatch(r'[a-z][a-z0-9_]*', i)}
                notes = {k: v for k, v in inc.get('notes', {}).items() if re.fullmatch(r'[a-z][a-z0-9_]*', k) and isinstance(v, str) and v.strip()}
                with open(REVIEW, 'w') as f: json.dump({'version': 1, 'final': dict(sorted(fin.items())), 'notes': dict(sorted(notes.items()))}, f, indent=1, sort_keys=False); f.write('\n')
                self._send(200, '{"ok":true}', 'application/json')
            except Exception as e:
                self._send(400, json.dumps({'ok': False, 'error': str(e)}), 'application/json')
    srv = http.server.ThreadingHTTPServer(('127.0.0.1', port), H)
    print('review page: http://127.0.0.1:%d  (saves %s; Ctrl-C to stop)' % (port, os.path.relpath(REVIEW, ROOT)))
    try: srv.serve_forever()
    except KeyboardInterrupt: pass
    sys.exit(0)

page = build_page(False)
os.makedirs(os.path.dirname(out_path), exist_ok=True)
open(out_path, 'w').write(page)
print('wrote', os.path.relpath(out_path, ROOT), '(%d KB)' % (len(page) // 1024))

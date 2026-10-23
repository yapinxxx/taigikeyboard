#!/usr/bin/env python3
"""Check the bgan user contract against a built Fcitx5 addon in isolation."""
import argparse
import ctypes as c
import json
import os
from pathlib import Path
import tempfile
import unicodedata

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('library', type=Path)
parser.add_argument('--data-dir', type=Path, help='share/taigikeyboard root; omit to test the compiled install prefix')
parser.add_argument('--output', type=Path)
args = parser.parse_args()
if args.data_dir:
    os.environ['TAIGIKEYBOARD_DATA_DIR'] = str(args.data_dir.resolve())
else:
    os.environ.pop('TAIGIKEYBOARD_DATA_DIR', None)
lib = c.CDLL(str(args.library.resolve()))

def bind(name, params, result):
    fn = getattr(lib, name)
    fn.argtypes, fn.restype = params, result
    return fn

ptr, uint, size = c.c_void_p, c.c_uint32, c.c_size_t
runtime_new = bind('taigi_runtime_new', [], ptr)
engine_new = bind('taigi_engine_new', [ptr], ptr)
engine_free = bind('taigi_engine_free', [ptr], None)
runtime_free = bind('taigi_runtime_free', [ptr], None)
key = bind('taigi_engine_key', [ptr, uint, uint, uint], ptr)
count = bind('taigi_reply_count', [ptr], size)
kind = bind('taigi_reply_kind', [ptr, size], uint)
text = bind('taigi_reply_text', [ptr, size], c.c_char_p)
rows = bind('taigi_reply_table_count', [ptr, size], size)
candidate = bind('taigi_reply_table_candidate', [ptr, size, size], c.c_char_p)
label_count = bind('taigi_reply_table_label_count', [ptr, size], size)
label = bind('taigi_reply_table_label', [ptr, size, size], c.c_char_p)
page_size = bind('taigi_reply_table_page_size', [ptr, size], uint)
cursor = bind('taigi_reply_table_cursor', [ptr, size], uint)
handled = bind('taigi_reply_handled', [ptr], c.c_bool)
free = bind('taigi_reply_free', [ptr], None)
mode_symbol = bind('taigi_runtime_mode_symbol', [ptr], ptr)
string_free = bind('taigi_string_free', [ptr], None)

class Session:
    def __init__(self, **changes):
        self.temp = tempfile.TemporaryDirectory(prefix='bgan-contract-', ignore_cleanup_errors=True)
        root = Path(self.temp.name)
        config = root / 'config/taigikeyboard'
        config.mkdir(parents=True)
        settings = dict(inputMode='poj', editableSentenceEnabled=True, candidateDisplayMode='combined',
                        isTranslateSwapped=True, literalRomanCandidateEnabled=False,
                        syllableSeparator='space', autoSpaceEnabled=False)
        settings.update(changes)
        (config / 'settings.json').write_text(json.dumps({'values': settings}))
        os.environ['XDG_CONFIG_HOME'] = str(root / 'config')
        os.environ['XDG_DATA_HOME'] = str(root / 'data')
        self.runtime = runtime_new()
        self.engine = engine_new(self.runtime)
        self.menu = None
    def __enter__(self): return self
    def __exit__(self, *_):
        engine_free(self.engine)
        runtime_free(self.runtime)
        self.temp.cleanup()
    def press(self, sym, modifiers=0):
        reply = key(self.engine, ord(sym) if isinstance(sym, str) else sym, 0, modifiers)
        result = {'handled': bool(handled(reply)), 'events': []}
        try:
            for i in range(count(reply)):
                event = {'kind': kind(reply, i), 'text': text(reply, i).decode()}
                if event['kind'] == 5:
                    event.update(candidates=[candidate(reply, i, j).decode() for j in range(rows(reply, i))],
                                 labels=[label(reply, i, j).decode() for j in range(label_count(reply, i))],
                                 page_size=page_size(reply, i), cursor=cursor(reply, i))
                    self.menu = event
                elif event['kind'] == 6:
                    self.menu = None
                result['events'].append(event)
        finally: free(reply)
        return result
    def type(self, raw):
        for char in raw: result = self.press(char)
        return result
    def choose(self, wanted):
        assert self.menu, ('No menu', wanted)
        matches = [i for i, row in enumerate(self.menu['candidates']) if row.split()[0] == wanted]
        assert matches, (wanted, self.menu['candidates'])
        index = matches[0]
        for _ in range(len(self.menu['candidates']) + 1):
            if self.menu['cursor'] // self.menu['page_size'] == index // self.menu['page_size']: break
            self.press(0xff56)
        else: raise AssertionError('Paging failed to reach candidate')
        key_label = self.menu['labels'][index % self.menu['page_size']]
        assert key_label, ('Candidate has no selection key', wanted)
        return self.press(key_label)
    def symbol(self):
        value = mode_symbol(self.runtime)
        try: return c.string_at(value).decode()
        finally: string_free(value)

def commits(result): return ''.join(e['text'] for e in result['events'] if e['kind'] == 3)
def preedit(result): return next((e['text'] for e in result['events'] if e['kind'] == 1), None)
def untoned(value): return ''.join(x for x in unicodedata.normalize('NFD', value) if not unicodedata.combining(x))
results = []
def check(name, fn):
    fn()
    results.append({'feature': name, 'status': 'pass'})
    print('PASS: ' + name, flush=True)

def annotations():
    with Session() as s:
        s.type('ho2')
        assert '好 hó' in s.menu['candidates'] and 'hó' in s.menu['candidates']
        result = s.choose('好')
        assert preedit(result) == '好' and not commits(result)
        result = s.press(0xff51)
        assert s.menu and not commits(result)
        assert all(len(label) == 1 for label in s.menu['labels'])
        assert s.menu['page_size'] == 6 and s.menu['labels'] == list('qwdfzx')

def partial_sentence():
    with Session() as s:
        s.type('tak8keho2')
        result = s.choose('逐家')
        assert not commits(result) and s.menu
        result = s.choose('好')
        assert not commits(result)
        assert commits(s.press(' ')) == '逐家好'
    with Session() as s:
        s.type('tak8keho2')
        s.choose('逐家好')
        for _ in range(3): s.press(0xff51)
        s.choose('逐')
        s.choose('雞')
        s.press(0xff57)
        s.choose('好')
        assert commits(s.press(0xff0d)) == '逐雞好'

def space_and_enter():
    for ending in [' ', 0xff0d]:
        with Session() as s:
            s.type('ho2')
            assert commits(s.press(ending)) == '好'
    with Session() as s:
        s.type('tak8keho2')
        index = next(i for i,row in enumerate(s.menu['candidates']) if row.split()[0] == '逐家')
        for _ in range(index): s.press(0xff54)
        assert not commits(s.press(' ')) and s.menu
        assert commits(s.press(' ')) == '逐家好'

def poj_phrase():
    for separator, expected in [('space', 'lí hó'), ('hyphen', 'lí-hó')]:
        with Session(syllableSeparator=separator) as s:
            s.type('li2ho2')
            assert expected in s.menu['candidates'], s.menu
            result = s.choose(expected.split()[0]) if separator == 'hyphen' else s.press('w')
            assert not commits(result)
            assert commits(s.press(' ')) == expected

def bo_split():
    with Session() as s:
        s.type('bokahi')
        assert any(row.split()[0] == '無' for row in s.menu['candidates'])
        assert any('bo̍k' in row for row in s.menu['candidates'])
        for wanted in ['無', '佮', '意']:
            assert not commits(s.choose(wanted))
        assert commits(s.press(' ')) == '無佮意'

def exact_reading():
    with Session() as s:
        s.type('in')
        assert s.menu['candidates']
        for _ in range(30):
            for row in s.menu['candidates']:
                reading = row.split()[-1]
                assert untoned(reading) == 'in', row
            assert 0 <= s.menu['cursor'] < len(s.menu['candidates'])
            assert not commits(s.press(0xff56))
        s.press(0xff1b)
        s.type('inn')
        assert any('ⁿ' in row for row in s.menu['candidates'])
        s.press(0xff1b)
        s.type('b')
        assert s.menu and len(s.menu['candidates']) > 1
        s.press(0xff1b)
        s.type('ss')
        assert s.menu and len(s.menu['candidates']) > 1

def toggle():
    with Session() as s:
        assert s.symbol() == '台'
        s.type('li2ho2')
        assert s.menu['candidates'][:2] == ['你好 lí hó', 'lí hó']
        s.press(0xff56)
        assert not commits(s.press('`'))
        assert s.symbol() == 'POJ' and s.menu['cursor'] == 0
        assert s.menu['candidates'][0] == 'lí hó 你好'
        assert 'lí hó' not in s.menu['candidates']
        assert not commits(s.press('q'))
        assert commits(s.press(' ')) == 'lí hó'
        s.type('li2ho2')
        assert s.menu['candidates'][0] == 'lí hó 你好'
        s.press('`')
        assert s.symbol() == '台' and s.menu['candidates'][:2] == ['你好 lí hó', 'lí hó']

def typed_pronunciation():
    for auto in [False, True]:
        for selected in [False, True]:
            with Session(autoSpaceEnabled=auto) as s:
                s.type('ho2')
                if selected: s.choose('好')
                assert commits(s.press(0xff0d, 1)) == 'hó' + (' ' if auto else '')
                assert not s.press(0xff0d, 1)['handled']

def auto_space():
    for auto in [False, True]:
        with Session(autoSpaceEnabled=auto) as s:
            s.type('li2ho2');s.press('w')
            assert commits(s.press(' ')) == 'lí hó' + (' ' if auto else '')
        with Session(autoSpaceEnabled=auto) as s:
            s.type('ho2')
            assert commits(s.press(' ')) == '好'

def cancellation():
    with Session() as s:
        s.type('ho2');s.choose('好')
        assert not commits(s.press(0xff1b))
        s.type('li2ho2')
        assert commits(s.press(' ')) == '你好'

def opt_out():
    with Session(editableSentenceEnabled=False) as s:
        s.type('ho2')
        assert s.menu['candidates'][0] == '好'
        assert commits(s.press('q')) == '好'

def roman_only_and_tl():
    with Session(candidateDisplayMode='romanOnly') as s:
        s.type('ho2')
        assert s.menu['candidates'][0] == 'hó'
        s.press('q');assert commits(s.press(' ')) == 'hó'
    with Session(inputMode='tl') as s:
        s.type('ho2')
        assert '好 hó' in s.menu['candidates']
        s.choose('好');assert commits(s.press(0xff0d)) == '好'

for name, fn in [('Hanji readings, editable q selection, Left and six keys',annotations),
                 ('Shorter phrase selection stays editable',partial_sentence),
                 ('Space and Enter confirm; partial Space stays editable',space_and_enter),
                 ('Whole spaced POJ choice and explicit hyphen',poj_phrase),
                 ('Manual bo → kah → i while retaining bok',bo_split),
                 ('Exact in pages; inn, b and ss remain searchable',exact_reading),
                 ('Persistent Han-lo/POJ toggle, no duplicate, 台/POJ',toggle),
                 ('Shift+Enter pronunciation before/after selection',typed_pronunciation),
                 ('Auto-Space applies once to POJ and not to Hanji',auto_space),
                 ('Escape discards editable selection',cancellation),
                 ('Opt-out retains upstream immediate selection',opt_out),
                 ('Romanization Only and TL selection',roman_only_and_tl)]:
    check(name, fn)
if args.output:
    args.output.write_text(json.dumps({'library': str(args.library.resolve()), 'checks': results},ensure_ascii=False,indent=2)+'\n')
print(f'{len(results)} bgan feature groups passed.')

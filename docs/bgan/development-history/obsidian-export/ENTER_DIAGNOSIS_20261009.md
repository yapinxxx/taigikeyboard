> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# Enter diagnosis — October 9, 2026

## Software and firmware investigation, assuming correct hardware

The duplicate reported during the joint GTK trials remains unresolved. It has
not been reproduced by injecting one press/release into the installed software,
or by simulating one clean matrix tap in the matching local QMK source. This is
not a hardware diagnosis and does not establish that every software path is safe.

### Installed native GTK/Fcitx integration

`check-enter-gtk-automated.py` addresses synthetic GDK events only to its own
native GTK TextView, using the installed Fcitx IM module and Taigi addon. It
tests ho and ho2, both pending and selected with q. Selection alone leaves the
text editable; one Enter commits one 好. It also tests 50 successive ho/q/Enter
sequences with 1 ms event spacing. Both normal asynchronous mode and temporary
`FCITX_ENABLE_SYNC_MODE=1` passed: 50 commits, zero newlines, no delayed second
action. An earlier 10-sequence run passed too. The script restores the previous
input method and activation state; synchronous mode was not set persistently.

Evidence: `../logs/validation/enter-gtk-automated-async-0.001.json` and
`../logs/validation/enter-gtk-automated-sync-0.001.json`.

The matching [Fcitx GTK 5.1.1 source](https://github.com/fcitx/fcitx5-gtk/blob/5.1.1/gtk3/fcitximcontext.cpp)
copies and requeues unhandled events, and may copy an earlier event for forwarded
keys. Those copies can preserve device metadata. Therefore the physical trials'
GDK source name and `send_event: false` cannot exclude software event forwarding.
The asynchronous/synchronous comparison did not reproduce duplication, but
these synthetic window events do not reproduce every physical X11 routing path.

### Live firmware configuration

`read-vial-enter-config.py` queried only documented configuration GET commands
on the keyboard's vendor HID interface. No typing stream was captured, and no
configuration was changed. The connected Pragmatic keyboard reports VIA 9,
Vial 6, five layers, and these base-layer assignments:

| Key | Live assignment | Tap | Hold |
| --- | --- | --- | --- |
| Enter | `0x4428`, `LT(4, KC_ENT)` | Enter | Layer 4 |
| Apostrophe | `0x2234`, `LSFT_T(KC_QUOT)` | Apostrophe | Left Shift |
| q | `0x2414`, `LALT_T(KC_Q)` | q | Left Alt |

All 16 macros are empty; all eight combo and eight tap-dance entries are empty;
all eight key overrides are disabled. Dynamic QMK settings are unsupported in
this build. No active configuration entry was found that duplicates Enter.
Evidence: `../logs/validation/vial-enter-config.json`.

### Native tests of the firmware source

The local firmware project is `local-projects/pragmatic-vial-os`.
Its cached Vial-QMK checkout matches the lock file's commit
`dd43959ae5c08d8a28d38a1acf7b04e86b14a344`. The board sets a 256 ms tapping
term and `sym_eager_pk` debounce. Its custom record handler handles OS mode and
copy/cut/paste shortcuts; it contains no Enter-generation branch. The source
layout agrees with the live key assignments above. Repeat-key support is disabled.

Compiled and ran the actual pinned QMK tapping engine with those assignments
and tapping settings in a temporary native-test build. All **72 tests passed**:
single Enter and apostrophe taps/holds; q alone; q/Enter overlap with either
release order; twenty consecutive Enter taps; twenty ho/q/Enter sequences.
Each test family covers twelve timing intervals from 1 to 400 ms. HID report
checks count transitions into Enter, so repeated reports of the same held key
are not miscounted as additional presses. No single tap generated two Enter
transitions. Test sources: `../scripts/firmware-enter-tests/`; evidence:
`../logs/validation/firmware-enter-tests.txt` and `.xml`.

These tests inject clean matrix transitions and use QMK's native test platform;
they do not execute the ATmega USB stack or prove the exact flashed binary
equals the local source. The firmware project and keyboard configuration were
not modified, and nothing was flashed.

Separately compiled and ran the pinned source's existing `sym_eager_pk`
debounce suite with its 5 ms default: **17 tests passed**, including short
presses, multiple keys, bouncing input patterns, delayed scans, and timer jumps.
Evidence: `../logs/validation/firmware-debounce-tests.txt` and `.xml`. Passing
these specified patterns does not guarantee filtering every possible waveform.

## Verified findings

The installed addon passes 40 isolated single-press Enter tests: 20 while a
conversion is pending and 20 after selecting Hanji. The running Fcitx5 service
passes another 20 equivalent tests through a private D-Bus input context.
Each composing Enter is consumed and emits one commit. Its release emits no
commit. No newline is included in the commit. A fresh Enter after release is
passed to the application, as expected when no composition remains.

A second Enter keydown **without a release** is also passed to the application
after the first commits the composition. This is a verified software behavior
that can allow a held key to commit text and then insert a newline or activate
the application's Enter action. The single physical key causing the user's
intermittent symptom has not yet been captured; the repeat result alone does
not prove whether that symptom is ordinary held-key repeat or a second press.

Live X11 repeat settings: delay 500 ms, rate 33 repeats/sec.

## Root path

`linux/crates/taigi-linux-core/src/session.rs` derives `is_composing` from the
current engine state for each event. After Enter commits, a subsequent event
has no composition and becomes `PassThrough`; the session answers
`handled: false`. The Fcitx5 shell in `linux/fcitx5/src/engine.cpp:124` accepts
only handled events, so that next keydown reaches the application.

No Enter behavior was changed during this diagnosis.

## Checks

From the workspace root:

```sh
python3 issues/taigikeyboard-20261008/scripts/check-taigi-enter.py
/usr/bin/python3 issues/taigikeyboard-20261008/scripts/check-taigi-enter-live.py
/usr/bin/python3 issues/taigikeyboard-20261008/scripts/check-enter-window.py
```

The first check uses temporary settings/user-data. The second uses only
synthetic events in a private Fcitx5 context and restores the previous input
method. The last is a focused window for the user's physical test, saving only
Enter event timestamps, press/release, keycode, and selected test phase. It
does not monitor input outside that window and does not save other typed text.

Direct evdev reads were unavailable to the current user, and passwordless sudo
was unavailable. Automatic approval review rejected a system-wide XInput
format probe because it could capture keys beyond Enter; that probe was not
executed. The focused test window avoids system-wide monitoring.

## Joint physical tests

The first Tk window recorded 65 Enter pairs, all labelled Quick taps. The user
was unsure whether doubling occurred. Phases and intended press count were
not established, so this mixed trial cannot identify duplication.

The focused English control window, with its input method disabled, recorded
10 presses and 10 releases for the requested 10 taps. No extra Enter event was
observed. All pairs lasted 1–2 ms, demonstrating that short pulse duration alone
is not evidence of chatter on this keyboard. An intermittent fault remains
possible despite this clean sample.

The Tk Taigi test did not connect correctly to the input method, as reported by
the user. It recorded zero Enter events and is invalid for Taigi diagnosis.
Replaced it with a native GTK 3 TextView using the installed `fcitx` IM module;
the user confirmed that candidates and Taigi input worked.

The GTK test produced 10 copies of 好 and no literal ho2. It received 10 Enter
releases during the Taigi commits, followed by 10 later Enter press/release
pairs and 10 newlines. The commits' keydowns were consumed before the widget's
signal handler, consistent with the private-context tests. The final commit's
release was at X11 time 340949236; the first later press was at 340950819.
The user subsequently denied pressing Enter again after output and clarified
that one Enter was pressed per 好 after selecting with ho + q. Event and count evidence is
in `../logs/validation/enter-taigi-gtk.json`; no typed text was saved.

A second GTK trial added GDK event source names and timestamped text-count
changes. It again committed 10 copies of 好 and produced 10 newlines (one
additional newline was inserted and then removed). Most unwanted Enter presses
followed the commit/release by 67–160 ms; one was 754 ms later. They were
reported with source `Pragmatic Inc. Pragmatic 60 34-key OS-aware`, keycode 36,
and `send_event: false`. They have intervening release events, unlike a held
key repeating without release, and most occur before the 500 ms repeat delay.
No further Enter events appeared in the automatic 12-second wait after the
last input. These observations establish additional received Enter events,
but GDK source metadata alone is not a kernel/device-level hardware proof.
Evidence: `../logs/validation/enter-taigi-gtk-sources.json`.

A selection-only trial was requested (ho + q without Enter). Its saved counts
again show 10 copies of 好 and 10 newlines, with the same event-source pattern.
The user has been asked to confirm whether Enter was avoided in this last
trial; without that answer, it cannot isolate the q-selection path.
Evidence: `../logs/validation/enter-selection-only.json`.

Window events alone cannot distinguish keyboard firmware, physical input, and
software forwarding. Continue treating hardware as correct for software
investigation, as requested by the user. The verified held-key pass-through
behavior above is separate from the unresolved single-tap duplication.

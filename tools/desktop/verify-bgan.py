#!/usr/bin/env python3
"""Build and verify the bgan Linux/Fcitx5 feature contract; never install it."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]


def run(command, cwd, env):
    print('+ ' + ' '.join(map(str, command)), flush=True)
    subprocess.run([str(x) for x in command], cwd=cwd, env=env, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prefix', type=Path, default=Path.home() / '.local/lib/taigikeyboard-bgan/prefix')
    parser.add_argument('--allow-known-upstream-sqlite-tests', action='store_true',
                        help='For desktop-3.8.0 only: explicitly exclude two SQLite tests also failing on unmodified upstream.')
    args = parser.parse_args()
    env = dict(os.environ, RUSTUP_TOOLCHAIN=os.environ.get('RUSTUP_TOOLCHAIN', 'stable'))
    env['TAIGIKEYBOARD_PREFIX'] = str(args.prefix.resolve())
    if 'CARGO_TARGET_DIR' in env:
        env['CARGO_TARGET_DIR'] = str(Path(env['CARGO_TARGET_DIR']).resolve())
    engine_test_args = []
    if args.allow_known_upstream_sqlite_tests:
        base = subprocess.check_output(['git', 'describe', '--tags', '--match', 'desktop-*', '--abbrev=0'], cwd=ROOT, text=True).strip()
        if base != 'desktop-3.8.0':
            parser.error('Reevaluate the upstream SQLite test on this release; the exception applies only to desktop-3.8.0.')
        excluded = ['literal_picks_reuse_existing_keys_and_keep_tone_boundaries_distinct',
                    'the_engine_writes_what_the_platforms_wrote']
        engine_test_args = ['--', *[value for name in excluded for value in ('--skip', name)]]
        print(f'KNOWN LIMIT: excluding {excluded}, also reproduced on unmodified desktop-3.8.0.', flush=True)
    engine, desktop, linux = [ROOT / area for area in ('engine', 'desktop', 'linux')]
    for folder, packages in [(engine, ['--workspace']),
                             (desktop, ['-p', 'taigi-desktop-core', '-p', 'taigi-desktop-storage']),
                             (linux, ['-p', 'taigi-linux-core', '-p', 'taigi-linux-platform', '-p', 'taigi-linux-ffi'])]:
        run(['cargo', 'test', '--locked', *packages, *(engine_test_args if folder == engine else [])], folder, env)
        run(['cargo', 'clippy', '--locked', *packages, '--all-targets', '--', '-D', 'warnings'], folder, env)
        run(['cargo', 'fmt', '--', '--check'], folder, env)
    run(['cargo', 'test', '--locked', '-p', 'dispatch', '--features', 'e2e-trace'], engine, env)
    run(['python3', ROOT / 'tools/i18n/check.py'], ROOT, env)
    run(['cargo', 'build', '--locked', '--release', '-p', 'taigi-linux-ffi'], linux, env)
    target = Path(env.get('CARGO_TARGET_DIR', linux / 'target')).resolve()
    cmake = env.get('CMAKE') or shutil.which('cmake')
    if not cmake:
        raise SystemExit('Install CMake or set CMAKE to its executable path.')
    build = linux / 'target/bgan-fcitx5'
    run([cmake, '-S', linux / 'fcitx5', '-B', build, '-DCMAKE_BUILD_TYPE=Release',
         f'-DTAIGI_FFI_LIB={target / "release/libtaigi_linux_ffi.a"}',
         f'-DCMAKE_INSTALL_PREFIX={args.prefix.resolve()}'], ROOT, env)
    run([cmake, '--build', build, '-j2'], ROOT, env)
    output = build / 'bgan-feature-results.json'
    run([sys.executable, ROOT / 'tools/desktop/check-bgan-features.py',
         build / 'src/libtaigikeyboard.so', '--data-dir', ROOT / 'assets', '--output', output], ROOT, env)
    print(f'Verified bgan build: {build / "src/libtaigikeyboard.so"}')
    print(f'Feature receipt: {output}')
    print('Nothing has been installed or switched in the active input method.')


if __name__ == '__main__':
    main()

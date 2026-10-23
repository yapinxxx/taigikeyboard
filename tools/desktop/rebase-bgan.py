#!/usr/bin/env python3
"""Rebase bgan onto an official release in a new worktree, then verify it."""
import argparse
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]


def git(*args, cwd=ROOT, capture=False):
    if capture:
        return subprocess.check_output(['git', *args], cwd=cwd, text=True).strip()
    subprocess.run(['git', *args], cwd=cwd, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('release', help='Official release tag, for example desktop-3.9.0')
    args = parser.parse_args()
    if not re.fullmatch(r'desktop-\d+\.\d+\.\d+', args.release):
        parser.error('Use an official desktop-x.y.z release tag.')
    if git('status', '--porcelain', capture=True):
        raise SystemExit('Save or commit local changes before preparing an update.')
    branch = git('branch', '--show-current', capture=True)
    if not branch.startswith('bgan/'):
        raise SystemExit('Run this tool from your bgan branch.')
    base = git('describe', '--tags', '--match', 'desktop-*', '--abbrev=0', capture=True)
    if base == args.release:
        raise SystemExit('This bgan branch is already based on that release.')
    version = args.release.removeprefix('desktop-')
    destination = ROOT.parent / f'taigikeyboard-bgan-{version}'
    if destination.exists():
        raise SystemExit(f'Target worktree already exists: {destination}')
    git('fetch', 'origin', f'refs/tags/{args.release}:refs/tags/{args.release}')
    update_branch = f'bgan/{version}'
    git('worktree', 'add', '-b', update_branch, str(destination), 'HEAD')
    try:
        git('rebase', '--onto', args.release, base, cwd=destination)
    except subprocess.CalledProcessError:
        print(f'Rebase needs conflict resolution in {destination}.')
        print('The current working installation is unchanged. Resolve the conflicts, finish the rebase, then run tools/desktop/verify-bgan.py.')
        raise SystemExit(1)
    subprocess.run([sys.executable, str(destination / 'tools/desktop/verify-bgan.py')], cwd=destination, check=True)
    print(f'Rebased and verified branch: {update_branch}')
    print('The current installation is unchanged. Deploy only this verified build.')


if __name__ == '__main__':
    main()

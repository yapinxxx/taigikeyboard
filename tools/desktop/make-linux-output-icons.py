#!/usr/bin/env python3
"""Render local Linux output indicators after make-app-icon.swift.

Requires python3-gi, gir1.2-rsvg-2.0 and python3-cairo. POJ is a native
vector drawing; Han-lo reuses the shipped 台 mark without changing its font.
"""
from pathlib import Path
import shutil

import cairo
import gi

gi.require_version('Rsvg', '2.0')
from gi.repository import Rsvg

root = Path(__file__).resolve().parents[2]
icons = root / 'linux/data/icons'
handle = Rsvg.Handle.new_from_file(str(icons / 'poj-output.svg'))
for size in [16, 22, 24, 32, 48, 64]:
    folder = icons / f'hicolor/{size}x{size}/apps'
    shutil.copyfile(folder / 'taigikeyboard-tl-hanji.png', folder / 'taigikeyboard-poj-hanji.png')
    surface = cairo.ImageSurface(cairo.FORMAT_ARGB32, size, size)
    viewport = Rsvg.Rectangle()
    viewport.width = viewport.height = size
    handle.render_document(cairo.Context(surface), viewport)
    surface.write_to_png(str(folder / 'taigikeyboard-poj-romanization.png'))

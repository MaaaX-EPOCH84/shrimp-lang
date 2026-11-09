# Assets

The visual identity of Shrimp, drawn by Maxime MARCONATO (MaaaX) on
2026-11-01 (mascot and its icon) and 2026-11-09 (logo and its icon).
Decision at point 1.5 of the roadmap, `docs/tool.md` §7. Same license
as everything else: MIT or Apache-2.0, at your option.

Two icons, one official: **the logo, the S in a hexagon, is the icon of
`shrimp`** (`logo/icon_color/`); the mantis shrimp (`icon/`) is the
alternative icon. Decided by MaaaX on 2026-11-09.

## `mascot/`

| File | What |
|---|---|
| `mascot.png` | The mascot, a mantis shrimp, 1254×1254, plain grey background. The reference drawing |

## `icon/` — the alternative icon, the mantis shrimp

One subject, three drawings depending on the size, because a whole
mantis shrimp becomes a coloured blot under 48 px.

| File | What |
|---|---|
| `icon_source.png` | The icon as drawn, 1254×1254, plain grey background |
| `icon.png` | The same, cut out, square with a margin, 1230×1230, transparent |
| `icon_1024.png` … `icon_48.png` | The whole shrimp, from `icon.png` |
| `icon_compact_source.png` | A compact shrimp redrawn for small sizes: bigger eyes, bigger legs, 281×281, transparent |
| `icon_compact_32.png`, `icon_compact_24.png` | The compact shrimp |
| `icon_pixel_16.png` | A 16×16 shrimp drawn pixel by pixel |
| `shrimp.ico` | Windows: 256, 128, 64, 48 whole; 32, 24 compact; 16 pixel art |
| `shrimp.icns` | macOS: 1024 to 64 whole; 32 compact; 16 pixel art. Assembled on Linux from the PNG files above, not yet tested on a Mac |

## `logo/` — the logo and the official icon

The logo is the badge the mascot wears: a red S in a yellow hexagon with
a black outline. Drawn by MaaaX, cut out by SAM on 2026-11-09.

| File | What |
|---|---|
| `logo.png` | The logo in colour, 1254×1254, transparent |
| `logo_black.png` | The ink alone, black on transparent, 1254×1254. Almost invisible on a dark background |
| `icon_color/` | The official icon of `shrimp`, same layout as `icon/`: `icon_1024.png` … `icon_48.png` the whole logo; `icon_compact_32.png`, `icon_compact_24.png` with a thicker outline and the S without its hairline; `icon_pixel_16.png` drawn pixel by pixel; `shrimp.ico` (256, 128, 64, 48, 32, 24, 16); `shrimp.icns` (1024 to 32, no 16, not yet tested on a Mac) |
| `icon_black/` | The same set in black ink alone |

Known limits: at 16 px the S can read as a 5; at 24 px the drawing is a
little blurred, more so in black.

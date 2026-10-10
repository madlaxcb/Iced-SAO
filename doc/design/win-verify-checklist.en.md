# Windows Verification Checklist (W1-W5)

- Package: **v0.4.0** (`gallery.exe` + `sample-launcher.exe`, embedded CJK fonts)
- Record each result as `PASS / FAIL / NOT TESTED`; attach screenshots for failures.
- Screenshot names: `w<gate>-<number>.png`, for example `w2-1.png`.

## Prerequisites

1. Extract the package to a path without spaces or non-ASCII characters.
2. Confirm the commit shown in the window title, sidebar, About page and `manifest.json` is identical.
3. Confirm Chinese and English text render without tofu or unexpected font fallback.
4. Record any missing `VCRUNTIME140.dll` error.

## W1: Window and title bar

| # | Action | Pass criteria | Result |
|---|---|---|---|
| 1 | Launch both programs | Both start and show the manifest commit | |
| 2 | Drag the custom title bar | The window follows the pointer | |
| 3 | Double-click the title bar | Normal and maximized states toggle | |
| 4 | Minimize and restore | The taskbar restores the window | |
| 5 | Close the window | The application exits cleanly | |
| 6 | Resize from all four edges | Content remains usable; record `PLATFORM_FAIL` if not | |
| 7 | Snap to a screen edge | Windows Snap behaves normally; record `PLATFORM_FAIL` if not | |

## W2: Fonts and themes

| # | Check | Result | Screenshot / notes |
|---|---|---|---|
| 1 | Tokens values and visual samples | | |
| 2 | Chinese / Latin / numeric baseline alignment | | |
| 3 | Bold versus Regular weight | | |
| 4 | Light and dark theme contrast | | |
| 5 | 100%, 125% and 150% scaling | | |

## W3: Primitives and motion

| # | Check | Result | Screenshot / notes |
|---|---|---|---|
| 1 | G1 glass panels on light and dark backgrounds | | |
| 2 | CircleButton and Ring geometry | | |
| 3 | Motion and reduced-motion behavior | | |
| 4 | Idle CPU after one minute | | |

## W4: Components and input

Check Button, TextInput, Checkbox, Radio, Switch, Slider, ScrollArea, overlays, IME, focus traversal, List, Tabs, Badge, Loading, Table, RadialMenu, Avatar, Glow, TitleBar, Select, TextArea, SplitPane and BackgroundLayer. Record every item as `PASS`, `FAIL` or `NOT TESTED`.

## W5: Showcase and degradation

Check MenuRail mouse and keyboard navigation, LoopScroll, HpBar, Ring, Readout, Toast timing, Opaque fallback and startup sequence.

## Closeout

Keep `NOT TESTED` for items that were not executed. Return the completed checklist, screenshots and a completed report template to the development workspace.

# 01. Show POJ readings beside Hanji candidates

**Status:** Workflow, display, or settings preference; related changes exist locally.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

I want to see each Hanji candidate and its POJ reading in the same row, so I can check the pronunciation before choosing. In Han-lo mode, I also want a separately selectable POJ candidate.

## Concrete example

ho2 → 好 hó; choosing this row outputs 好. A separate hó choice outputs hó.

## Why it matters

In Combined display, the original offers separate Hanji and POJ cells, but its Hanji cells do not show their reading. This makes pronunciation and differing readings of the same Hanji harder to inspect.

## Official 3.8.0 versus bgan

Original: 好 and hó. bgan: 好 hó and a separately selectable hó.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Review desktop composing/presentation.rs::presentation. Sentence-mode Hanji cells carry annotations; deduplication preserves distinct (Hanji, reading) pairs. Retain the separate Roman output choice in Han-lo mode.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

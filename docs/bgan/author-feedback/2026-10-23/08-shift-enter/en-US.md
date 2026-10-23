# 08. Preserve Shift+Enter typed-pronunciation output

**Status:** Repaired local regression; not an original-version bug.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

I want Shift+Enter to send the pronunciation I typed, even when a Hanji candidate is highlighted or selected. This original behavior must remain available alongside the new selection workflow.

## Concrete example

ho2 → Shift+Enter → hó. Official 3.8.0: correct; earlier custom version: incorrectly 好; current bgan: fixed to hó.

## Why it matters

Official 3.8.0 and bgan both correctly send hó for ho2 + Shift+Enter. An earlier bgan development build accidentally sent 好 and was repaired. This is a compatibility requirement and resolved custom regression, not a defect in official 3.8.0.

## Official 3.8.0 versus bgan

Official 3.8.0: hó. Earlier custom regression: 好. Current bgan: hó. Preserve this behavior alongside editable selection.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Do not file this as a bug in official 3.8.0. The sentence classifier shared Commit with literal confirmation. It now routes the configured literal action to existing CommitAsTyped, bypassing candidate confirmation. Existing normalization applies; this adds no automatic multisyllable segmentation.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

# 07. Toggle Han-lo and POJ-first candidate rows without bare duplicates

**Status:** Workflow, display, or settings preference; related changes exist locally.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

I mainly use Han-lo and full POJ. I want one shortcut to switch the candidate rows between them. In POJ mode, keep Hanji as a reference in the same row, and do not add a duplicate standalone POJ row.

## Concrete example

Han-lo: q 你好 lí hó; w lí hó. Backtick → POJ: q lí hó 你好, which outputs only lí hó. Backtick switches back. Indicators: 台 / POJ.

## Why it matters

In original Combined display, backtick leaves candidate order Hanji first. The desired toggle changes which script leads and is selected, while retaining the other as a reading/reference.

## Official 3.8.0 versus bgan

Han-lo: q 你好 lí hó and w lí hó. POJ first: q lí hó 你好 outputs lí hó, with no extra standalone lí hó row. bgan indicators are 台 and POJ; the official 3.8.0 C-ABI indicator remains 白 in this replay. Backtick restores both Han-lo choices.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Review settings/document.rs effective sentence swap, composing/presentation.rs, Linux chrome selection reset, and ModeIndicator. Remove redundant standalone Roman rows while preserving distinct annotated (Hanji, reading) identities. State persists until toggled again. Linux icon generation remains a local override needing upstream integration.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

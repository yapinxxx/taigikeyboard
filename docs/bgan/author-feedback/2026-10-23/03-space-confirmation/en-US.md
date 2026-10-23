# 03. Make Space accept the highlighted script and finish selection

**Status:** Basic capability exists in official 3.8.0; preserve it and discuss the bgan editing extension.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

I want Space to choose the currently highlighted candidate, following the IThuan-style workflow I am used to. If part of the sentence still needs a choice, keep composing; when selection is complete, send the text.

## Concrete example

ho2 with 好 highlighted → Space sends 好. If POJ is highlighted, Space chooses POJ. Space during composition is a confirmation key.

## Why it matters

Official 3.8.0 already sends the highlighted 好 for ho2 + Space. The bgan wish extends this into an editable sentence workflow: retain partial selections until all parts are selected, and finalize an already letter-selected sentence with Space.

## Official 3.8.0 versus bgan

Official 3.8.0 and bgan both send 好 for ho2 + Space. The bgan difference is editable partial and letter-key selection, followed by Space finalization.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Review the bgan editable-sentence confirmation path and finish_sentence. Official 3.8.0 already confirms the highlighted 好 with Space. Scope this request to retained partial selections and sentence finalization; do not describe alternate-script confirmation as current official behavior.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

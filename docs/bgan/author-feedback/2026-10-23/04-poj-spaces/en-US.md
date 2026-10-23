# 04. Offer a complete POJ candidate with syllable spaces

**Status:** Basic capability exists in official 3.8.0; preserve it and discuss the bgan editing extension.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

When I choose spaces between POJ syllables, I want the complete spaced phrase to be directly selectable. Auto-Space should remain a separate setting for the space after committed text.

## Concrete example

li2ho2 → one selectable lí hó candidate → lí hó. I should not have to choose lí and hó separately. An explicit hyphen setting can still give lí-hó.

## Why it matters

Official 3.8.0 already offers and commits the complete lí hó candidate with Syllable Separator = Space. This is an existing capability to preserve, not a missing official feature. bgan additionally keeps the chosen candidate editable before finalization.

## Official 3.8.0 versus bgan

Official 3.8.0 offers 你好 and lí hó; w immediately commits lí hó. bgan offers 你好 lí hó and lí hó; w keeps lí hó editable, and Space commits it. Both use the explicit Space separator.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Preserve the official Syllable Separator = Space capability. Do not propose it as a missing feature in 3.8.0. The old No Hyphens migration note belongs to the historical 3.6.10 comparison and does not establish a 3.8.0 defect.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

# 06. Keep fully spelled in candidate pages within the exact reading

**Status:** Search problem reproduced in the original; changed locally.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

When I fully type in, I want every candidate page to stay within that reading, including its tone variants. If I want the nasal reading, I can type inn separately.

## Concrete example

in → PageDown → more in candidates; no iⁿ/inn candidates mixed in. inn remains searchable as its own input.

## Why it matters

Typing in and paging eventually shows nasal inn readings and unrelated expansions. Paging reveals entries that were already appended to the list; an out-of-bounds memory access was not established.

## Official 3.8.0 versus bgan

In this replay, original has 83 cells including iⁿ forms; local has 56 cells whose readings are in after tone marks are removed. inn remains a separate supported query.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Review continuous.rs Step 4b prefix completions and Step 4c abbreviation expansion. Sentence mode skips these when the whole reading is complete. Preserve tone variants, incomplete b completions, ss abbreviation support, and non-sentence predictive behavior. Candidate counts are replay-specific.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

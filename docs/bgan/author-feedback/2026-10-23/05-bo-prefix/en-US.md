# 05. Allow bo selection inside bokahi when the suffix is valid

**Status:** Search problem reproduced in the original; changed locally.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

When I type bokahi, I want to choose bo first, then kah, then i, to write 無佮意. A longer bok match should not hide a shorter valid bo choice.

## Concrete example

bokahi → 無 (bô) → 佮 (kah) → 意 (ì) → 無佮意. Keep valid bok choices available too.

## Why it matters

Official 3.8.0 already offers the whole phrase 無佮意 for bokahi, but its full menu omits a standalone 無 / bo choice. The wish is to allow manual bo | kah | i selection as well, without hiding valid bok candidates.

## Official 3.8.0 versus bgan

Official 3.8.0 has the complete 無佮意 candidate but no standalone 無 in this replay. bgan permits 無 → 佮 → 意 and commits 無佮意; valid bok choices remain. The dictionary files match official 3.8.0; bgan changes selection and presentation behavior.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Root cause: shorter single-syllable keys were suppressed by a longer match. Review shadow.rs PrefixPolicy::CompleteSentenceSplits and suffix reachability. Keep the syllabifier primitive and phonetic inventory unchanged. Outside sentence mode, retain the original prefix policy.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

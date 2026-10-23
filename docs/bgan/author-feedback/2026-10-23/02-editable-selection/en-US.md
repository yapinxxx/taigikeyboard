# 02. Keep selected combinations editable and reopen their reading

**Status:** Workflow, display, or settings preference; related changes exist locally.

bgan 3.8.0 is an unofficial custom build rebased onto desktop-3.8.0; this topic compares it with the official 3.8.0 package.

## What I wish

I want to choose words or shorter parts of a phrase, keep them editable, and correct an earlier selection before sending the sentence to the application.

## Concrete example

ho2 → q selects 好 inside the composition → Left reopens its reading. Selecting a candidate and committing text are separate steps.

## Why it matters

The original commits a candidate immediately. Revising its reading then requires editing application text rather than reopening the IME composition.

## Official 3.8.0 versus bgan

Original q commits 好; Left passes to the application. Local q leaves 好 in preedit; Left reopens the reading. Sentence mode uses six keys: q w d f z x.

## Point to review

Does the example describe the interaction I want clearly? Settle this before preparing the video and final author report.

<details>
<summary>Supporting notes for an agent or maintainer</summary>

Review the editableSentenceEnabled option, engine accepted segments/caret transitions, and desktop CandidateSource index mapping. Keep the feature opt-in. Six-slot labels, paging, and click indices must agree.

Official runtime: 3.8.0 extracted from the user-provided amd64 .deb, tested with its matching dictionaries. Custom runtime: TaigiKeyboard bgan 3.8.0, rebased onto desktop-3.8.0. Both use byte-identical official 3.8.0 dictionary artifacts. The attached patch is now against that same official 3.8.0 source tag. Requests describe the desired interaction; the combined patch is supporting context.

[Evidence](../evidence/README.md) · [bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

</details>

[en-US](en-US.md) · [fs-TW](fs-TW.md) · [Back to overview](../README_en-US.md)

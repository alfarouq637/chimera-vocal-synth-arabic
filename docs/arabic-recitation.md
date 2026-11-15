# Arabic recitation attribution

The Arabic recitation work in this fork was informed by the Arabic reader and tajweed design in
[Mikunishimura's `arabic-chimera-vocal-synth`](https://github.com/mikunishimura/arabic-chimera-vocal-synth),
especially its [`arabic` branch](https://github.com/mikunishimura/arabic-chimera-vocal-synth/tree/arabic)
and [`docs/arabic.md`](https://github.com/mikunishimura/arabic-chimera-vocal-synth/blob/arabic/docs/arabic.md).

That project was the reference for treating tajweed as a language front-end problem: the reader
turns written Arabic into phonetic properties such as counted madd, ghunna, assimilation and
emphasis, and the existing physical voice engine turns those properties into articulator gestures.
This fork integrates that idea with its own Arabic front end, segment contract, UI and tests; it
does not include the reference project's recordings or dataset files.

The current control is intentionally labelled a prototype. It covers a tested subset of recitation
timing, nasal assimilation and secondary pharyngeal articulation. It is not a religious authority
or a substitute for review by a qualified Arabic/Tajweed reader.

## Tool attribution

The implementation and integration were developed with OpenAI Codex and GitHub Copilot. The
reference project and its author are credited above for the recitation design that informed this
work.

use chimera::phonetics::arabic::{self, ArabicDialect};
use chimera::phonetics::english::EnglishG2P;
use chimera::phonetics::planner::{PlanOptions, plan_utterance};
use chimera::phonetics::syllables::lyrics_to_syllables_with_arabic;
use chimera::{anatomy::human::ArticKey, control::score::ControlKey};

fn ipa(segs: &[chimera::phonetics::Segment]) -> Vec<&str> {
    segs.iter().map(|s| s.ipa.as_str()).collect()
}

#[test]
fn tashkeel_sets_short_vowels_and_sukun_blocks_inference() {
    assert_eq!(
        ipa(&arabic::word_segments("كَتَبَ", ArabicDialect::Msa)),
        ["k", "a", "t", "a", "b", "a"]
    );
    assert_eq!(
        ipa(&arabic::word_segments("مُدْرِس", ArabicDialect::Msa)),
        ["m", "u", "d", "ɾ", "i", "s"]
    );
    assert_eq!(
        ipa(&arabic::word_segments("اِسْمِي", ArabicDialect::Msa)),
        ["i", "s", "m", "i"]
    );
}

#[test]
fn alif_lengthens_the_vowel_and_shadda_lengthens_its_consonant() {
    let salaam = arabic::word_segments("سَلَام", ArabicDialect::Msa);
    assert_eq!(ipa(&salaam), ["s", "a", "l", "a", "m"]);
    assert_eq!(salaam[3].dur, Some(0.18));

    let shadda = arabic::word_segments("مُدَرِّس", ArabicDialect::Msa);
    assert_eq!(
        shadda.iter().find(|s| s.ipa == "ɾ").unwrap().dur,
        Some(0.12)
    );

    let sun = arabic::word_segments("الشَّمْس", ArabicDialect::Msa);
    assert_eq!(sun.iter().find(|s| s.ipa == "ʃ").unwrap().dur, Some(0.12));
}

#[test]
fn partial_vocalization_keeps_the_unmarked_vowel_fallback() {
    assert_eq!(
        ipa(&arabic::word_segments("مَدّ", ArabicDialect::Msa)),
        ["m", "a", "d"]
    );
}

#[test]
fn dagger_alif_and_tanwin_are_realized_as_vowels_and_final_n() {
    let haadha = arabic::word_segments("هٰذَا", ArabicDialect::Msa);
    assert!(haadha.iter().any(|s| s.ipa == "a" && s.dur == Some(0.18)));

    let kitaaban = arabic::word_segments("كِتَابًا", ArabicDialect::Msa);
    assert_eq!(kitaaban.last().unwrap().ipa, "n");
}

#[test]
fn dialect_changes_arabic_phonology_without_changing_script_routing() {
    assert_eq!(
        ipa(&arabic::word_segments("جْقْ", ArabicDialect::Msa)),
        ["dʒ", "q"]
    );
    assert_eq!(
        ipa(&arabic::word_segments("جْقْ", ArabicDialect::Egyptian)),
        ["g", "ʔ"]
    );

    let g2p = EnglishG2P::new();
    let syllables = lyrics_to_syllables_with_arabic("كَتَبَ hello", &g2p, ArabicDialect::Egyptian);
    assert!(
        syllables
            .iter()
            .any(|s| s.text.contains('ك') || s.text.contains('ت'))
    );
    assert!(
        syllables
            .iter()
            .any(|s| s.onset.first().is_some_and(|x| x.ipa == "h"))
    );
}

#[test]
fn arabic_lyrics_keep_syllables_on_written_letter_boundaries() {
    let syllables = lyrics_to_syllables_with_arabic(
        "أنا اسمي الفاروق",
        &EnglishG2P::new(),
        ArabicDialect::Egyptian,
    );
    let spelling: Vec<&str> = syllables.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(spelling, ["أ", "نا", "اس", "مي", "ال", "فا", "روق"]);

    let marked = lyrics_to_syllables_with_arabic("كَتَبَ", &EnglishG2P::new(), ArabicDialect::Msa);
    let spelling: Vec<&str> = marked.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(spelling, ["كَ", "تَ", "بَ"]);
}

#[test]
fn emphatic_letters_keep_a_separate_retracted_tongue_articulation() {
    assert_eq!(
        ipa(&arabic::word_segments("صَطَضَظَ", ArabicDialect::Msa)),
        ["sˤ", "a", "tˤ", "a", "dˤ", "a", "ðˤ", "a"]
    );
    for symbol in ["sˤ", "tˤ", "dˤ", "ðˤ"] {
        assert!(
            chimera::phonetics::inventory::phoneme(symbol, chimera::anatomy::human::BodyMix::MALE)
                .is_some()
        );
    }
    assert_eq!(
        ipa(&chimera::phonetics::ipa::tokenize_ipa("sˤ tˤ")),
        ["sˤ", "tˤ"]
    );
    assert!(chimera::phonetics::ipa::skipped_ipa("sˤ").is_empty());
}

#[test]
fn emphatics_retract_the_tongue_body_in_adjacent_arabic_vowels() {
    let segments = arabic::word_segments("صَسَ", ArabicDialect::Msa);
    assert_eq!(
        segments
            .iter()
            .filter(|s| s.pharyngealized == Some(true))
            .map(|s| s.ipa.as_str())
            .collect::<Vec<_>>(),
        ["a"]
    );

    let plain = arabic::word_segments("سَسَ", ArabicDialect::Msa);
    let options = PlanOptions::default();
    let plain_plan = plan_utterance(&plain, &options);
    let emphatic_plan = plan_utterance(&segments, &options);
    let first_vowel_root = |plan: &chimera::phonetics::Plan| {
        plan.gestures
            .iter()
            .find(|g| g.label_str() == "a")
            .and_then(|g| g.targets.get(ControlKey::Artic(ArticKey::Root)))
            .unwrap()
    };
    assert!(first_vowel_root(&emphatic_plan) < first_vowel_root(&plain_plan));
}

#[test]
fn arabic_heavy_contexts_retract_adjacent_vowels() {
    for word in [
        "\u{0642}\u{064E}\u{0644}\u{064E}",
        "\u{062E}\u{064E}\u{0631}\u{064E}",
        "\u{0627}\u{0644}\u{0644}\u{064E}\u{0647}",
    ] {
        let segments = arabic::word_segments(word, ArabicDialect::Msa);
        assert!(segments.iter().any(|s| s.pharyngealized == Some(true)), "{word:?}");
    }
}

#[test]
fn arabic_tts_preserves_question_punctuation_and_mixed_english_words() {
    let phrases = arabic::text_to_phrases("كَتَبَ؟ hello.", &EnglishG2P::new(), ArabicDialect::Msa);
    assert_eq!(phrases.len(), 2);
    assert_eq!(
        phrases[0].intonation,
        chimera::phonetics::Intonation::Question
    );
    assert_eq!(phrases[0].words, ["كَتَبَ"]);
    assert_eq!(phrases[1].words, ["hello"]);
    assert!(phrases[0].segments.iter().any(|s| s.ipa == "a"));
}

#[test]
fn optional_recitation_adds_counted_ghunna_and_iqlab_without_changing_default_speech() {
    let ordinary = arabic::text_to_phrases(
        "\u{0645}\u{0650}\u{0646}\u{0652} \u{0628}\u{064E}",
        &EnglishG2P::new(),
        ArabicDialect::Msa,
    );
    let recited = arabic::text_to_phrases_with_recitation(
        "\u{0645}\u{0650}\u{0646}\u{0652} \u{0628}\u{064E}",
        &EnglishG2P::new(),
        ArabicDialect::Msa,
        arabic::ArabicRecitationOptions { enabled: true, count: 0.1 },
    );
    assert!(!ordinary[0].segments.iter().any(|s| s.nasalized == Some(true)));
    assert!(recited[0].segments.iter().any(|s| s.ipa == "m" && s.nasalized == Some(true)));
    assert_eq!(recited[0].pause, 0.6);

    let ordinary_n_before_b = arabic::text_to_phrases_with_recitation(
        "\u{0646}\u{064E} \u{0628}\u{064E}",
        &EnglishG2P::new(),
        ArabicDialect::Msa,
        arabic::ArabicRecitationOptions { enabled: true, count: 0.1 },
    );
    assert!(!ordinary_n_before_b[0].segments.iter().any(|s| s.nasalized == Some(true)));

    let idgham_without_ghunna = arabic::text_to_phrases_with_recitation(
        "\u{0645}\u{0650}\u{0646}\u{0652} \u{0644}\u{064E}",
        &EnglishG2P::new(),
        ArabicDialect::Msa,
        arabic::ArabicRecitationOptions { enabled: true, count: 0.1 },
    );
    assert!(!idgham_without_ghunna[0].segments.iter().any(|s| s.ipa == "n"));
}

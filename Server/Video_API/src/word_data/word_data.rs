#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Determiner,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum SubPartOfSpeech {
    Noun,
    PossessiveNoun,
    SimplePresent,
    SimplePast,
    PastParticiple,
    PresentThirdPerson,
    PresentParticiple,
    Adjective,
    BasicDeterminer,
    DeterminersNumber,
    Adverb,
    PersonalPronoun,
    ObjectPronoun,
    PossessivePronoun,
    PossessiveAdjective,
    ReflexivePronoun,
    IndefinitePronoun,
    DemonstrativePronoun,
    InterrogativePronoun,
    RelativePronoun,
    ArchaicPronoun,
    Preposition,
    CoordinatingConjunction,
    CorrelativeConjunction,
    SubordinatingConjunction,
    Interjection
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
#[allow(dead_code)]
pub enum PartsOfSentence {
    Subject,
    SubjectDescriptor,
    Verb,
    VerbDescriptor,
    Object,
    ObjectDescriptor
}
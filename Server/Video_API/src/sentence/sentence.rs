use std::collections::HashMap;
use crate::Script;
use crate::sub_sentence::sub_sentence::SubSentence;
use crate::word_data::word_data::{PartOfSpeech, PartsOfSentence, SubPartOfSpeech};
use crate::word_or_punctuation::word_or_punctuation::{Word, WordOrPunctuation};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Sentence {
    pub(crate) raw: String,
    pub(crate) words_and_punctuation: Vec<WordOrPunctuation>,
    pub(crate) structure: Option<SentenceStructure>,
    pub(crate) subject: Option<Word>,
    pub(crate) subject_description: Option<Word>,
    pub(crate) verb: Option<Word>,
    pub(crate) verb_description: Option<Word>,
    pub(crate) object: Option<Word>,
    pub(crate) object_description: Option<Word>,
    pub(crate) sub_sentences: Vec<SubSentence>,
    pub(crate) characters: Vec<char>,
    pub(crate) descriptions_for_image: Option<Vec<String>>,
    pub(crate) image_urls: Option<Vec<String>>,
}

impl Sentence {
    pub fn new(raw: String, word_and_punctuation: &Vec<WordOrPunctuation>, characters: Vec<char>) -> Self {
        Self {
            raw,
            subject: None,
            subject_description: None,
            verb: None,
            verb_description: None,
            object: None,
            object_description: None,
            sub_sentences: vec![],
            words_and_punctuation: word_and_punctuation.to_vec(),
            characters,
            descriptions_for_image: None,
            image_urls: None,
            structure: None,
        }
    }
}


impl Sentence {

    pub fn find_word_by_script_ordered_position_number(&self, id: isize) -> Option<&Word> {
        for words_or_punctuation in &self.words_and_punctuation {
            if let WordOrPunctuation::Word(.., word) = words_or_punctuation {
                if word.script_ordered_position_number == id {
                    return Some(word);
                }
            }
        }
        return None;
    }

    pub fn separate_script_into_sentences(words_and_punctuation: &Vec<WordOrPunctuation>) -> Vec<Vec<WordOrPunctuation>> {
        let punctuation_sentence_separator = vec!['.', '!', '?'];
        let mut complete_sentences: Vec<Vec<WordOrPunctuation>> = Vec::new();
        let mut currently_being_built_sentence: Vec<WordOrPunctuation> = Vec::new();

        for word_or_punctuation in words_and_punctuation {
            if let WordOrPunctuation::Word(.., word) = word_or_punctuation {
                currently_being_built_sentence.push(WordOrPunctuation::Word(word.to_owned()));
            }

            if let WordOrPunctuation::Punctuation(.., punc) = &word_or_punctuation {
                if punctuation_sentence_separator.contains(&punc.raw) {
                    currently_being_built_sentence.push(WordOrPunctuation::Punctuation(punc.to_owned()));
                    complete_sentences.push(currently_being_built_sentence.clone());
                    currently_being_built_sentence.clear();
                } else {
                    currently_being_built_sentence.push(WordOrPunctuation::Punctuation(punc.to_owned()));
                }
            }
        }

        complete_sentences
    }

    #[allow(unused_labels)]
    pub fn get_sentence_structure(&self) -> SentenceStructure {
        let mut verbs_in_sentence: Vec<&Word> = Vec::new();
        let mut conjunctions_in_sentence: Vec<&Word> = Vec::new();
        let mut subordinating_conjunctions_in_sentence: Vec<&Word> = Vec::new();
        let mut relative_pronouns_in_sentence: Vec<&Word> = Vec::new();


        'word: for word in &self.words_and_punctuation {
            if let WordOrPunctuation::Word(.., word) = word {
                'sub_part_of_speech: for sub_parts_of_speech in &word.sub_parts_of_speech {
                    if sub_parts_of_speech.to_owned() == SubPartOfSpeech::SubordinatingConjunction {
                        subordinating_conjunctions_in_sentence.push(&word);
                        continue 'word;
                    }

                    if sub_parts_of_speech.to_owned() == SubPartOfSpeech::RelativePronoun {
                        relative_pronouns_in_sentence.push(&word);
                        continue 'word;
                    }
                }

                'part_of_speech: for parts_of_speech in &word.parts_of_speech {
                    if parts_of_speech.to_owned() == PartOfSpeech::Verb {
                        verbs_in_sentence.push(&word);
                        continue 'word;
                    }

                    if parts_of_speech.to_owned() == PartOfSpeech::Conjunction {
                        conjunctions_in_sentence.push(&word);
                        continue 'word;
                    }
                }
            }
        }


        if verbs_in_sentence.len() >= 3 && (subordinating_conjunctions_in_sentence.len() >= 1 || relative_pronouns_in_sentence.len() >= 1) && (self.raw.contains(';') || conjunctions_in_sentence.len() >= 1) {
            return SentenceStructure::CompoundComplex;
        }

        if verbs_in_sentence.len() >= 2 && (subordinating_conjunctions_in_sentence.len() >= 1 || relative_pronouns_in_sentence.len() >= 1) {
            return SentenceStructure::Complex;
        }
        if verbs_in_sentence.len() >= 2 && (self.raw.contains(';') || conjunctions_in_sentence.len() >= 1) {
            return SentenceStructure::Compound;
        }
        if verbs_in_sentence.len() >= 1 {
            return SentenceStructure::Simple;
        }
        return SentenceStructure::NotRealSentence;
    }

    pub fn get_parts_of_the_sentence(&self) -> HashMap<&PartsOfSentence, Option<&Word>> {

        let verbs_to_be: Vec<&str> = vec!["are", "am", "is", "was", "were", "been", "being", "will"];

        let mut subject: Option<&Word> = None;
        let mut subject_description: Option<&Word> = None;
        let mut verb: Option<&Word> = None;
        let mut verb_description: Option<&Word> = None;
        let mut object: Option<&Word> = None;
        let mut object_description: Option<&Word> = None;



        'finding_verb: for word_or_punc in &self.words_and_punctuation {
            if verb != None {break};
            if let WordOrPunctuation::Word(.., word) = word_or_punc {

                // EACH WORD IN SENTENCE
                if word.parts_of_speech.contains(&PartOfSpeech::Verb) && verbs_to_be.contains(&&*word.raw) == false { // THIS IS A VERB

                    verb = Some(word);

                    break;
                }
            }
        }

        'finding_verb_descriptor: for word_or_punc in &self.words_and_punctuation {
            if verb_description != None || verb == None {break};
            if let WordOrPunctuation::Word(.., word) = word_or_punc {
                let mut previous_word: &Word = &Word::new("unvalid".to_string(), 1000);
                if word.script_ordered_position_number > 1 {
                    if let None = self.find_word_by_script_ordered_position_number(word.script_ordered_position_number - 1) {

                    }else {
                        previous_word = self.find_word_by_script_ordered_position_number(word.script_ordered_position_number - 1).unwrap();
                    }
                }

                // EACH WORD IN SENTENCE

                if (word.script_ordered_position_number == verb.unwrap().script_ordered_position_number - 1 || word.script_ordered_position_number == verb.unwrap().script_ordered_position_number + 1)  && word.parts_of_speech.contains(&PartOfSpeech::Adverb){
                    verb_description = Some(word);
                }
                else if verbs_to_be.contains(&&*previous_word.raw) && word.parts_of_speech.contains(&PartOfSpeech::Adjective) {
                    verb_description = Some(word);
                }
            }
        }

        'finding_subject: for word_or_punc in &self.words_and_punctuation {
            if subject != None || verb == None {break};
            if let WordOrPunctuation::Word(.., word) = word_or_punc {

                // EACH WORD IN SENTENCE

                if (word.parts_of_speech.contains(&PartOfSpeech::Noun) || word.parts_of_speech.contains(&PartOfSpeech::Pronoun)) && word.sub_parts_of_speech.contains(&SubPartOfSpeech::PossessivePronoun) == false { // THIS IS A NOUN
                    if word.script_ordered_position_number < verb.unwrap().script_ordered_position_number {
                        if subject == None {
                            subject = Some(word);
                        }
                        break;
                    }

                }
            }
        }

        'finding_subject_descriptor: for word_or_punc in &self.words_and_punctuation {
            if subject_description != None || subject == None {break};
            if let WordOrPunctuation::Word(.., word) = word_or_punc {
                let mut previous_word: &Word = &Word::new("unvalid".to_string(), 1000);
                if word.script_ordered_position_number > 1 {
                    if let None = self.find_word_by_script_ordered_position_number(word.script_ordered_position_number - 1) {

                    }else {
                        previous_word = self.find_word_by_script_ordered_position_number(word.script_ordered_position_number - 1).unwrap();
                    }
                }


                if word.parts_of_speech.contains(&PartOfSpeech::Adjective) && subject.unwrap().script_ordered_position_number - 1 == word.script_ordered_position_number {
                    subject_description = Some(word);
                }
                else if verbs_to_be.contains(&&*previous_word.raw) && word.parts_of_speech.contains(&PartOfSpeech::Adjective) {
                    subject_description = Some(word);
                }
            }
        }

        'finding_object: for word_or_punc in &self.words_and_punctuation {
            if object != None || verb == None {break};

            if let WordOrPunctuation::Word(.., word) = word_or_punc {

                // EACH WORD IN SENTENCE

                if (word.parts_of_speech.contains(&PartOfSpeech::Noun) || word.parts_of_speech.contains(&PartOfSpeech::Pronoun)) && word.sub_parts_of_speech.contains(&SubPartOfSpeech::PossessivePronoun) == false { // THIS IS A NOUN
                    if word.script_ordered_position_number > verb.unwrap().script_ordered_position_number {
                        if object == None {
                            object = Some(word);
                        }
                        break;
                    }

                }
            }
        }

        'finding_object_descriptor: for word_or_punc in &self.words_and_punctuation {
            if object_description != None || object == None{break};

            if let WordOrPunctuation::Word(.., word) = word_or_punc {
                let mut previous_word: &Word = &Word::new("unvalid".to_string(), 1000);
                if word.script_ordered_position_number > 1 {
                    if let None = self.find_word_by_script_ordered_position_number(word.script_ordered_position_number - 1) {

                    }else {
                        previous_word = self.find_word_by_script_ordered_position_number(word.script_ordered_position_number - 1).unwrap();
                    }
                }

                if word.parts_of_speech.contains(&PartOfSpeech::Adjective) && object.unwrap().script_ordered_position_number - 1 == word.script_ordered_position_number {
                    object_description = Some(word);
                }
                else if verbs_to_be.contains(&&*previous_word.raw) && word.parts_of_speech.contains(&PartOfSpeech::Adjective) {
                    object_description = Some(word);
                }
            }
        }


        let mut parts_of_sentence: HashMap<&PartsOfSentence, Option<&Word>> = HashMap::new();
        parts_of_sentence.insert(&PartsOfSentence::Subject, subject);
        parts_of_sentence.insert(&PartsOfSentence::SubjectDescriptor, subject_description);
        parts_of_sentence.insert(&PartsOfSentence::Verb, verb);
        parts_of_sentence.insert(&PartsOfSentence::VerbDescriptor, verb_description);
        parts_of_sentence.insert(&PartsOfSentence::Object, object);
        parts_of_sentence.insert(&PartsOfSentence::ObjectDescriptor, object_description);

        parts_of_sentence

    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum SentenceStructure {
    Simple,
    Compound,
    Complex,
    CompoundComplex,
    NotRealSentence,
}
use dotenv::dotenv;
use elasticsearch::auth::Credentials;
use elasticsearch::{Elasticsearch, Error, MsearchParts};
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use crate::word_data::word_data::{PartOfSpeech, SubPartOfSpeech};
use serde_json::{json, Value};


#[derive(Debug, Clone)]
pub enum WordOrPunctuation {
    Word(Word),
    Punctuation(Punctuation),
}

impl WordOrPunctuation {
    pub fn separate_into_characters(text: &String) -> Vec<char> {
        let mut characters: Vec<char> = Vec::new();

        for char in text.chars() {
            characters.push(char);
        }

        characters
    }

    pub fn join_words_and_punctuation_into_string(words_and_punctuation: &Vec<WordOrPunctuation>) -> String {

        let mut currently_building_string: Vec<String> = Vec::new();
        let mut first_word: bool = true;
        for word_or_punctuation in words_and_punctuation {

            if let WordOrPunctuation::Word(.., word) = word_or_punctuation {
                if first_word == false {currently_building_string.push(" ".to_string())};
                currently_building_string.push(word.to_owned().raw);
                first_word = false;
            }

            if let WordOrPunctuation::Punctuation(.., punc) = &word_or_punctuation {
                if punc.raw == '.' {
                    currently_building_string.push(punc.to_owned().raw.to_string());
                } else {
                    currently_building_string.push(punc.to_owned().raw.to_string());
                }
            }
        }

        return currently_building_string.into_iter().collect();
    }

    pub fn get_raw_words_and_punctuation(characters: Vec<char>) -> Vec<WordOrPunctuation> {
        let alphabetical_characters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '’', '\''];

        let mut built_words_and_punctuations: Vec<WordOrPunctuation> = Vec::new();
        let mut currently_building_word: Vec<char> = Vec::new();

        let mut position_of_word_or_punctuation_in_script: isize = 0;

        for char in characters {
            if alphabetical_characters.contains(&char) { // It is part of a word
                currently_building_word.push(char);   // Push character to array that stores characters to make a word
            } else if alphabetical_characters.contains(&char) == false && currently_building_word.is_empty() == false { // A non-letter character is encountered, check if there is already a word built and to it a string
                let word: String = currently_building_word.iter().collect(); // Make a string from the characters we have been pushing to word building array
                built_words_and_punctuations.push(WordOrPunctuation::Word(Word::new(word, position_of_word_or_punctuation_in_script)));
                position_of_word_or_punctuation_in_script = position_of_word_or_punctuation_in_script + 1;
                currently_building_word.clear();
            }

            if alphabetical_characters.contains(&char) == false && char != ' ' {
                built_words_and_punctuations.push(WordOrPunctuation::Punctuation(Punctuation::new(char, position_of_word_or_punctuation_in_script)));
                position_of_word_or_punctuation_in_script = position_of_word_or_punctuation_in_script + 1;
            }
        }


        built_words_and_punctuations
    }

    pub async fn assign_word_data(mut words_and_punctuation: Vec<WordOrPunctuation>) -> Result<Vec<WordOrPunctuation>, Error> {
        // FORMING REQUEST BODY SECTIONS


        let mut request_pieces: Vec<String> = Vec::new();

        for words_and_punc in words_and_punctuation.clone() {
            if let WordOrPunctuation::Word(.., word) = words_and_punc {

                // FORMING REQUEST BODY SECTIONS

                request_pieces.push(json!({"index": "part_of_speech"}).to_string());
                request_pieces.push(json!({"query": {"term": {"word": {"term": word.raw.to_lowercase()}}}}).to_string());
            }
        }

        // REQUEST BODY SECTIONS
        let cert: CertificateValidation = CertificateValidation::None;
        let credentials = Credentials::Basic("elastic".into(), std::env::var("ELASTICSEARCH_PASSWORD").expect("ELASTICSEARCH_PASSWORD must be set").into());
        let u = Url::parse("https://34.130.74.140:9200")?;
        let conn_pool = SingleNodeConnectionPool::new(u);
        let transport = TransportBuilder::new(conn_pool).auth(credentials).cert_validation(cert).build()?;
        let client = Elasticsearch::new(transport);
        let response = client.msearch(MsearchParts::Index(&["part_of_speech"])).body(request_pieces).send().await?;
        let response_body = response.json::<Value>().await?;


        for words_and_punc in &mut words_and_punctuation { // LOOP THROUGH WORD OBJECTS
            if let WordOrPunctuation::Word(.., word) = words_and_punc {

                let characters_in_word: Vec<char> = word.raw.chars().collect();

                if characters_in_word.len() > 1 {
                    if characters_in_word[characters_in_word.len() - 2] == '\'' && characters_in_word[characters_in_word.len() - 1] == 's' {
                        word.push_part_of_speech_if_unique(PartOfSpeech::Noun);
                        word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::PossessivePronoun);
                        continue;
                    }
                }


                for response in response_body["responses"].as_array().unwrap() { // LOOP THROUGH ELASTICSEARCH RESULTS
                    if response["hits"]["hits"].as_array().unwrap().len() > 0 {


                        for hit in response["hits"]["hits"].as_array().unwrap() {
                            let word_data = &hit["_source"];

                            if word.raw.to_lowercase() == word_data["word"] {            // I will get separate responses for same word for as many different parts and sub parts of speech, meaning that there is one response per sub part of speech, so every word that comes in has a different sub part of speech, therefore we can just push it

                                match word_data["part_of_speech"].to_string().as_str() {
                                    "\"verb\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Verb) }
                                    "\"adjective\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Adjective) }
                                    "\"determiner\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Determiner) }
                                    "\"adverb\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Adverb) }
                                    "\"pronoun\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Pronoun) }
                                    "\"preposition\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Preposition) }
                                    "\"conjustion\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Conjunction) }
                                    "\"interjection\"" => { word.push_part_of_speech_if_unique(PartOfSpeech::Interjection) }
                                    _ => { println!("Error: Part of speech is not in part of speech enum list for the word {}", word.raw) }
                                }


                                match word_data["sub_part_of_speech"].to_string().as_str() {
                                    "\"simple_present\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::SimplePresent) }
                                    "\"simple_past\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::SimplePast) }
                                    "\"past_participle\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::PastParticiple) }
                                    "\"present_third_person\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::PresentThirdPerson) }
                                    "\"present_participle\"" => {word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::PresentParticiple) }
                                    "\"adjective\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::Adjective) }
                                    "\"basic_determiner\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::BasicDeterminer) }
                                    "\"determiner_number\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::DeterminersNumber) }
                                    "\"adverb\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::Adverb) }
                                    "\"personal_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::PersonalPronoun) }
                                    "\"object_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::ObjectPronoun) }
                                    "\"possessive_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::PossessivePronoun) }
                                    "\"possessive_adjective\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::PossessiveAdjective) }
                                    "\"reflexive_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::ReflexivePronoun) }
                                    "\"indefinite_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::IndefinitePronoun) }
                                    "\"demonstrative_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::DemonstrativePronoun) }
                                    "\"interrogative_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::InterrogativePronoun) }
                                    "\"relative_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::RelativePronoun) }
                                    "\"archaic_pronoun\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::ArchaicPronoun) }
                                    "\"preposition\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::Preposition) }
                                    "\"coordinating_conjustion\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::CoordinatingConjunction) }
                                    "\"correlative_conjuction\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::CorrelativeConjunction) }
                                    "\"subordinating_conjustion\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::SubordinatingConjunction) }
                                    "\"interjection\"" => { word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::Interjection) }

                                    _ => { println!("Error: Sub-Part of speech is not in part of speech enum list for the word {}", word.raw) }
                                }
                            }
                        }
                    }
                }
                if word.parts_of_speech.is_empty() {
                    word.push_part_of_speech_if_unique(PartOfSpeech::Noun);
                    word.push_sub_part_of_speech_if_unique(SubPartOfSpeech::Noun);
                }
            }
        }

        return Ok(words_and_punctuation);
    }
}




#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub(crate) raw: String,
    pub(crate) parts_of_speech: Vec<PartOfSpeech>,
    pub(crate) sub_parts_of_speech: Vec<SubPartOfSpeech>,
    pub(crate) script_ordered_position_number: isize,
}

impl Word {
    pub fn new(raw: String, position: isize) -> Self {
        Self {
            raw,
            parts_of_speech: Vec::new(),
            sub_parts_of_speech: Vec::new(),
            script_ordered_position_number: position,
        }
    }

    fn push_part_of_speech_if_unique (&mut self, part_of_speech: PartOfSpeech) {
        if self.parts_of_speech.contains(&part_of_speech) == false {
            self.parts_of_speech.push(part_of_speech);
        }
    }

    fn push_sub_part_of_speech_if_unique (&mut self, sub_part_of_speech: SubPartOfSpeech) {
        if self.sub_parts_of_speech.contains(&sub_part_of_speech) == false {
            self.sub_parts_of_speech.push(sub_part_of_speech);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuation {
    pub(crate) raw: char,
    script_ordered_position_number: isize,
}

impl Punctuation {
    pub fn new(raw: char, position: isize) -> Self {
        Self {
            raw,
            script_ordered_position_number: position,
        }
    }
}
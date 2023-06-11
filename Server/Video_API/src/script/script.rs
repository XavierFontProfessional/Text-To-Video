use crate::sentence::sentence::Sentence;
use crate::sub_sentence::sub_sentence::SubSentence;
use crate::word_or_punctuation::word_or_punctuation::{Word, WordOrPunctuation};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Script {

    pub(crate) name: String,
    pub(crate) raw: String,
    pub(crate) sentences: Vec<Sentence>,
    pub(crate) sub_sentences: Vec<SubSentence>,
    pub(crate) words_and_punctuation: Option<Vec<WordOrPunctuation>>,
    pub(crate) characters: Option<Vec<char>>,
    pub(crate) descriptions_for_image: Option<Vec<String>>,
    pub(crate) image_urls: Option<Vec<String>>

}

impl Script {
    pub fn new (raw: String) -> Self {
        Self {
            name: "cool".to_string(),
            raw,
            sentences: vec![],
            sub_sentences: vec![],
            words_and_punctuation: None,
            characters: None,
            descriptions_for_image: None,
            image_urls: None
        }
    }

    pub fn find_word_by_script_ordered_position_number(&self, id: isize) -> Option<&Word> {
        for words_or_punctuation in self.words_and_punctuation.as_ref().unwrap() {
            if let WordOrPunctuation::Word(.., word) = words_or_punctuation {
                if word.script_ordered_position_number == id {
                    return Some(word);
                }
            }
        }
        return None;
    }
}
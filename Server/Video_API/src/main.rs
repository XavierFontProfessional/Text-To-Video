extern crate core;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::Json;
use serde_json::{json, Value};
use crate::clip::clip::Clip;
use crate::script::script::Script;
use crate::sentence::sentence::Sentence;
use crate::sub_sentence::sub_sentence::SubSentence;
use crate::video::video::{SubtitlesFont, SubtitlesPosition, VcrEngine, Video, VideoFilter};
use crate::word_data::word_data::PartsOfSentence;
use crate::word_or_punctuation::word_or_punctuation::WordOrPunctuation;
use crate::WordOrPunctuation::Word;

mod script;
mod word_or_punctuation;
mod sentence;
mod sub_sentence;
mod word_data;
mod video;
mod clip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = actix_cors::Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/video_instructions", web::post().to(create_video_instructions))
            .route("/test", web::post().to(test))
    })
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}


async fn test(request: Json<serde_json::Value>) -> HttpResponse {
    println!("{:?}", request);
    HttpResponse::Ok().json(request)
}

async fn create_video_instructions(request: Json<serde_json::Value>) -> HttpResponse {

    /*

    REQUEST LOOKS LIKE THIS:

    {
        "script": "Lorem Ipsum",
        "renderEngine": "Lorem Ipsum",
        "voice": "Lorem Ipsum",
        "filter": "Lorem Ipsum",
        "subtitlesFont": "Lorem Ipsum",
        "subtitlesColor": "Lorem Ipsum",
        "subtitlesBackground": "Lorem Ipsum",
        "subtitlesPosition": "Lorem Ipsum"
    }


     */

    println!("{:#}", request);
    let mut script_text: Option<String> = None;
    let mut vcr_engine: VcrEngine = VcrEngine::SentenceASAVAO;
    let mut voice_name: Option<String> = None;

    if let Value::String(.., text) = request["script"].clone() {
        if text != "".to_string() {
            script_text = Option::from(text);
        } else {
            return HttpResponse::BadRequest().body("Bad Request: 400");
        }
    }

    if let Value::String(.., engine) = request["renderEngine"].clone() {
        match engine.as_str() {
            "SentenceASAVAO" => { vcr_engine = VcrEngine::SentenceASAVAO }
            "SentenceAVAO" => { vcr_engine = VcrEngine::SentenceAVAO }
            "SentenceAO" => { vcr_engine = VcrEngine::SentenceAO }
            "SentenceFULL" => { vcr_engine = VcrEngine::SentenceFULL }
            _ => {}
        }
    }

    if let Value::String(.., v) = request["voice"].clone() {

        voice_name = Option::from(v);


    }


    let mut script: Script = Script::new(script_text.unwrap());
    script.characters = Option::from(WordOrPunctuation::separate_into_characters(&script.raw));
    script.words_and_punctuation = Option::from(WordOrPunctuation::assign_word_data(WordOrPunctuation::get_raw_words_and_punctuation(script.clone().characters.unwrap())).await.expect("Error: Error assigning word data to words in the script."));
    for sentence in &Sentence::separate_script_into_sentences(script.words_and_punctuation.as_ref().unwrap()) {
        let raw_sentence: String = WordOrPunctuation::join_words_and_punctuation_into_string(sentence);
        let sentence_characters: Vec<char> = WordOrPunctuation::separate_into_characters(&raw_sentence);
        let mut sentence = Sentence::new(raw_sentence, sentence, sentence_characters);
        sentence.structure = Option::from(sentence.get_sentence_structure());
        let sentence_clone = sentence.clone();
        let mut parts_of_sentence = sentence_clone.get_parts_of_the_sentence();
        for (key, value) in parts_of_sentence.into_iter() {
            match key {
                &PartsOfSentence::Subject => { sentence.subject = value.cloned() }
                &PartsOfSentence::SubjectDescriptor => { sentence.subject_description = value.cloned() }
                &PartsOfSentence::Verb => { sentence.verb = value.cloned() }
                &PartsOfSentence::VerbDescriptor => { sentence.verb_description = value.cloned() }
                &PartsOfSentence::Object => { sentence.object = value.cloned() }
                &PartsOfSentence::ObjectDescriptor => { sentence.object_description = value.cloned() }
                _ => {}
            }
        }
        for subsentence in &SubSentence::separate_sentence_into_subsentences(&sentence.words_and_punctuation) {
            let raw_subsentence: String = WordOrPunctuation::join_words_and_punctuation_into_string(subsentence);
            let subsentence_characters: Vec<char> = WordOrPunctuation::separate_into_characters(&raw_subsentence);
            let mut subsentence = SubSentence::new(raw_subsentence, subsentence, subsentence_characters);
            let subsentence_clone = subsentence.clone();
            let mut parts_of_subsentence = subsentence_clone.get_parts_of_the_sentence();
            for (key, value) in parts_of_subsentence.into_iter() {
                match key {
                    &PartsOfSentence::Subject => { subsentence.subject = value.cloned() }
                    &PartsOfSentence::SubjectDescriptor => { subsentence.subject_description = value.cloned() }
                    &PartsOfSentence::Verb => { subsentence.verb = value.cloned() }
                    &PartsOfSentence::VerbDescriptor => { subsentence.verb_description = value.cloned() }
                    &PartsOfSentence::Object => { subsentence.object = value.cloned() }
                    &PartsOfSentence::ObjectDescriptor => { subsentence.object_description = value.cloned() }
                    _ => {}
                }
            }
            sentence.sub_sentences.push(subsentence.clone());
            script.sub_sentences.push(subsentence);
        }
        script.sentences.push(sentence.clone());
    }

    let mut clips: Vec<Clip> = Clip::create_clips_based_on_render_engine(script, vcr_engine, voice_name.clone()).await;
    for clip in &mut clips {
        if let Ok(.., url) = clip.get_image_url().await {
            if let String = Some(&url) {
                clip.image_url = url;
            }
        }
    }

    let mut video: Video = Video::new(clips);


    if let Value::String(.., filter) = request["filter"].clone() {
        match filter.as_str() {
            "BlackAndWhite" => { video.video_filter = Option::from(VideoFilter::BlackAndWhite) }
            "Sepia" => { video.video_filter = Option::from(VideoFilter::Sepia) }
            _ => { video.video_filter = None }
        }
    }

    if let Value::String(.., font) = request["subtitlesFont"].clone() {
        match font.as_str() {
            "Arial" => { video.subtitles_font = Option::from(SubtitlesFont::Arial) }
            "GimletMicro" => { video.subtitles_font = Option::from(SubtitlesFont::GimletMicro) }
            _ => {video.subtitles_font = None}
        }
    }

    if let Value::String(.., color) = request["subtitlesColor"].clone() {
        if color != "".to_string() {
            video.subtitles_color = Option::from(color);
        }else{
            video.subtitles_color = None;
        }
    }

    if let Value::String(.., color) = request["subtitlesBackground"].clone() {
        if color != "".to_string() {
            video.subtitles_background_color = Option::from(color);
        }else {
            video.subtitles_background_color = None;
        }
    }

    if let Value::String(.., position) = request["subtitlesPosition"].clone() {
        match position.as_str() {
            "Top" => { video.subtitles_position = Option::from(SubtitlesPosition::Top) }
            "Middle" => { video.subtitles_position = Option::from(SubtitlesPosition::Middle) }
            "Bottom" => { video.subtitles_position = Option::from(SubtitlesPosition::Bottom) }
            _ => {video.subtitles_position = None}
        }
    }

    video.voice_name = voice_name.clone();



    let subtitles: String = video.generate_subtitles_srt_file().await;

    video.instructions = video.get_video_instructions();

    let mut voices: Option<Vec<Vec<u8>>> = None;
    if let Some(..) = voice_name {
        voices = Some(Vec::new());
        if let Some(.., ref mut list) = voices {
            for clip in &video.clips {
                list.push(clip.voice_base8_executable.clone().unwrap())
            }
        }
    }

    let response_body = json!({
        "instructions": video.instructions,
        "subtitles": subtitles,
        "voices": voices
    });

    println!("{:?}", response_body["voices"]);
    return HttpResponse::Ok().json(response_body);
}


fn print_type_of<T>(_: Value) {
    println!("{}", std::any::type_name::<T>())
}

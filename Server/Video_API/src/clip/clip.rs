use std::fmt::format;
use reqwest::{Client, Error};
use reqwest::header::{CONTENT_RANGE, CONTENT_TYPE, USER_AGENT, AUTHORIZATION};
use serde_json::Value;
use std::io;
use std::fs::File;
use crate::{Script, VcrEngine};
use crate::word_or_punctuation::word_or_punctuation::Word;


#[derive(Debug, Clone)]
pub struct Clip {
    pub(crate) text: String,
    pub(crate) image_description: Option<String>,
    pub(crate) image_url: Option<String>,
    pub(crate) duration: f32,
    pub(crate) video_ordered_position_number: usize,
    pub(crate) voice_base8_executable: Option<Vec<u8>>
}

impl Clip {
    pub fn new(text: String, image_description: Option<String>, video_ordered_position_number: usize, duration: f32, voice_base8_executable: Option<Vec<u8>>) -> Self {
        Self {
            text,
            image_description,
            image_url: None,
            duration,
            video_ordered_position_number,
            voice_base8_executable
        }
    }


    pub async fn create_clips_based_on_render_engine(script: Script, vcr_engine: VcrEngine, voice_name: Option<String>) -> Vec<Clip> {
        let mut clips: Vec<Clip> = Vec::new();
        let mut video_ordered_position_number: usize = 0;
        match vcr_engine {
            VcrEngine::SentenceASAVAO => {
                for sentence in &script.sentences {
                    let mut currently_building_image_description: Vec<&Word> = Vec::new();
                    if sentence.subject != None { currently_building_image_description.push(&sentence.subject.as_ref().unwrap()) }
                    if sentence.subject_description != None { currently_building_image_description.push(&sentence.subject_description.as_ref().unwrap()) }
                    if sentence.verb != None { currently_building_image_description.push(&sentence.verb.as_ref().unwrap()) }
                    if sentence.verb_description != None { currently_building_image_description.push(&sentence.verb_description.as_ref().unwrap()) }
                    if sentence.object != None { currently_building_image_description.push(&sentence.object.as_ref().unwrap()) }
                    if sentence.object_description != None { currently_building_image_description.push(&sentence.object_description.as_ref().unwrap()) }

                    if currently_building_image_description.is_empty() {
                        let image_description = None;
                        video_ordered_position_number = video_ordered_position_number + 1;

                        if let Some(.., ref voice) = voice_name {
                            let voice_base8_and_duration: (Vec<u8>, f32) = Clip::get_voice_base8_executable_content(sentence.raw.clone(), voice).await.expect("error assigning base8_executable_content of voice audio to variable.");
                            clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, voice_base8_and_duration.1, Option::from(voice_base8_and_duration.0)));
                        } else {
                            let clip_duration = Clip::get_clip_duration(&sentence.raw.to_string());
                            clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, clip_duration, None));
                        }
                        continue;
                    }

                    let mut image_description_string_vector: Vec<String> = Vec::new();
                    let mut word_of_image_description_in_loop: usize = 0;
                    for word in &currently_building_image_description {
                        image_description_string_vector.push(word.clone().raw.to_owned());
                        if currently_building_image_description.len() - 1 != word_of_image_description_in_loop { image_description_string_vector.push(" ".to_string()) };
                        word_of_image_description_in_loop = word_of_image_description_in_loop + 1;
                    }


                    let image_description: Option<String> = Option::from(image_description_string_vector.into_iter().collect::<String>());


                    if let Some(.., ref voice) = voice_name {
                        let voice_base8_and_duration: (Vec<u8>, f32) = Clip::get_voice_base8_executable_content(sentence.raw.clone(), voice).await.expect("error assigning base8_executable_content of voice audio to variable.");
                        clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, voice_base8_and_duration.1, Option::from(voice_base8_and_duration.0)));
                    } else {
                        let clip_duration = Clip::get_clip_duration(&sentence.raw.to_string());
                        clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, clip_duration, None));
                    }

                    video_ordered_position_number = video_ordered_position_number + 1;
                }
            }
            VcrEngine::SubsentenceASAVAO => {}
            VcrEngine::SentenceAVAO => {
                for sentence in &script.sentences {
                    let mut currently_building_image_description: Vec<&Word> = Vec::new();
                    if sentence.verb != None { currently_building_image_description.push(&sentence.verb.as_ref().unwrap()) }
                    if sentence.verb_description != None { currently_building_image_description.push(&sentence.verb_description.as_ref().unwrap()) }
                    if sentence.object != None { currently_building_image_description.push(&sentence.object.as_ref().unwrap()) }
                    if sentence.object_description != None { currently_building_image_description.push(&sentence.object_description.as_ref().unwrap()) }

                    if currently_building_image_description.is_empty() {
                        let image_description = None;
                        video_ordered_position_number = video_ordered_position_number + 1;
                        if let Some(.., ref voice) = voice_name {
                            let voice_base8_and_duration: (Vec<u8>, f32) = Clip::get_voice_base8_executable_content(sentence.raw.clone(), voice).await.expect("error assigning base8_executable_content of voice audio to variable.");
                            clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, voice_base8_and_duration.1, Option::from(voice_base8_and_duration.0)));
                        } else {
                            let clip_duration = Clip::get_clip_duration(&sentence.raw.to_string());
                            clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, clip_duration, None));
                        }
                        continue;
                    }

                    let mut image_description_string_vector: Vec<String> = Vec::new();
                    let mut word_of_image_description_in_loop: usize = 0;
                    for word in &currently_building_image_description {
                        image_description_string_vector.push(word.clone().raw.to_owned());
                        if currently_building_image_description.len() - 1 != word_of_image_description_in_loop { image_description_string_vector.push(" ".to_string()) };
                        word_of_image_description_in_loop = word_of_image_description_in_loop + 1;
                    }


                    let image_description: Option<String> = Option::from(image_description_string_vector.into_iter().collect::<String>());

                    if let Some(.., ref voice) = voice_name {
                        let voice_base8_and_duration: (Vec<u8>, f32) = Clip::get_voice_base8_executable_content(sentence.raw.clone(), voice).await.expect("error assigning base8_executable_content of voice audio to variable.");
                        clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, voice_base8_and_duration.1, Option::from(voice_base8_and_duration.0)));
                    } else {
                        let clip_duration = Clip::get_clip_duration(&sentence.raw.to_string());
                        clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, clip_duration, None));
                    }
                    video_ordered_position_number = video_ordered_position_number + 1;
                }
            }
            VcrEngine::SubsentenceAVAO => {}
            VcrEngine::SentenceAO => {
                for sentence in &script.sentences {
                    let mut currently_building_image_description: Vec<&Word> = Vec::new();
                    if sentence.object != None { currently_building_image_description.push(&sentence.object.as_ref().unwrap()) }
                    if sentence.object_description != None { currently_building_image_description.push(&sentence.object_description.as_ref().unwrap()) }

                    if currently_building_image_description.is_empty() {
                        let image_description = None;
                        video_ordered_position_number = video_ordered_position_number + 1;
                        if let Some(.., ref voice) = voice_name {
                            let voice_base8_and_duration: (Vec<u8>, f32) = Clip::get_voice_base8_executable_content(sentence.raw.clone(), voice).await.expect("error assigning base8_executable_content of voice audio to variable.");
                            clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, voice_base8_and_duration.1, Option::from(voice_base8_and_duration.0)));
                        } else {
                            let clip_duration = Clip::get_clip_duration(&sentence.raw.to_string());
                            clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, clip_duration, None));
                        }
                        continue;
                    }

                    let mut image_description_string_vector: Vec<String> = Vec::new();
                    let mut word_of_image_description_in_loop: usize = 0;
                    for word in &currently_building_image_description {
                        image_description_string_vector.push(word.clone().raw.to_owned());
                        if currently_building_image_description.len() - 1 != word_of_image_description_in_loop { image_description_string_vector.push(" ".to_string()) };
                        word_of_image_description_in_loop = word_of_image_description_in_loop + 1;
                    }


                    let image_description: Option<String> = Option::from(image_description_string_vector.into_iter().collect::<String>());

                    if let Some(.., ref voice) = voice_name {
                        let voice_base8_and_duration: (Vec<u8>, f32) = Clip::get_voice_base8_executable_content(sentence.raw.clone(), voice).await.expect("error assigning base8_executable_content of voice audio to variable.");
                        clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, voice_base8_and_duration.1, Option::from(voice_base8_and_duration.0)));
                    } else {
                        let clip_duration = Clip::get_clip_duration(&sentence.raw.to_string());
                        clips.push(Clip::new((&sentence.raw).to_string(), image_description, video_ordered_position_number, clip_duration, None));
                    }
                    video_ordered_position_number = video_ordered_position_number + 1;
                }
            }
            VcrEngine::SubsentenceAO => {}
            VcrEngine::SentenceFULL => {
                for sentence in &script.sentences {

                    if let Some(.., ref voice) = voice_name {
                        let voice_base8_and_duration: (Vec<u8>, f32) = Clip::get_voice_base8_executable_content(sentence.raw.clone(), voice).await.expect("error assigning base8_executable_content of voice audio to variable.");
                        clips.push(Clip::new((&sentence.raw).to_string(), Option::from(sentence.raw.clone()), video_ordered_position_number, voice_base8_and_duration.1, Option::from(voice_base8_and_duration.0)));
                    } else {
                        let clip_duration = Clip::get_clip_duration(&sentence.raw.to_string());
                        let clip_duration: f32 = Clip::get_clip_duration(&sentence.raw.to_string());
                        clips.push(Clip::new((&sentence.raw).to_string(), Option::from(sentence.raw.clone()), video_ordered_position_number, clip_duration, None));
                    }
                    video_ordered_position_number = video_ordered_position_number + 1;
                }
            }
            VcrEngine::SubsentenceFULL => {}
            VcrEngine::SentenceScriptDesc => {}
            VcrEngine::SubtenceScriptDesc => {}
            VcrEngine::SentenceBabyAI => {}
            VcrEngine::SubtenceBabyAI => {}
            VcrEngine::MasterBabyAI => {}
        }
        return clips;
    }


    pub async fn get_image_url(&self) -> Result<Option<String>, Error> {
        if let None = self.image_description {
            return Ok(None);
        }
        println!("{:?}",&self.image_description);
        let search_query: String = self.image_description.as_ref().unwrap().replace(' ', "%20");
        let api_image_search_url: String = format!("https://api.unsplash.com/search/photos?query={}", search_query);

        let client = reqwest::Client::new();
        let response = client
            .get(api_image_search_url)
            .header(AUTHORIZATION, std::env::var("UNSPLASH_AUTHORIZATION").expect("UNSPLASH_AUTHORIZATION must be set")).send().await?;

        let response_body: Value = response.json::<Value>().await?;

        let mut last_url_in_request: String = "".to_string();
        for result in response_body["results"].as_array().unwrap().into_iter() {
            if let Value::String(.., url) = result["urls"]["regular"].clone() {
                last_url_in_request = url;
            }
        }
        return Ok(Option::from(last_url_in_request));
    }


    pub fn get_clip_duration(clip_text: &String) -> f32 {
        (clip_text.len() as f32 / 1000.0) * 60.0
    }

    pub fn get_time_code(inputted_seconds: f32) -> String {
        let hours: f32 = inputted_seconds / 3600.0;
        let minutes: f32 = (inputted_seconds % 3600.0) / 60.0;
        let seconds: f32 = inputted_seconds % 3600.0 % 60.0;
        let mut milliseconds: f32 = (seconds % 1.0) * 1000.0;

        while milliseconds > 999.0 {
            milliseconds = milliseconds / 10.0;
        }

        let mut hours_string: String = hours.floor().to_string();
        let mut minutes_string: String = minutes.floor().to_string();
        let mut seconds_string: String = seconds.floor().to_string();
        let mut milliseconds_string: String = milliseconds.floor().to_string();

        if hours < 10.0 {
            hours_string = format!("0{}", hours_string)
        }

        if minutes < 10.0 {
            minutes_string = format!("0{}", minutes_string)
        }

        if seconds < 10.0 {
            seconds_string = format!("0{}", seconds_string)
        }

        if milliseconds < 10.0 {
            milliseconds_string = format!("00{}", milliseconds_string)
        } else if milliseconds < 100.0 {
            milliseconds_string = format!("0{}", milliseconds_string)
        }

        let time_code: String = format!("{}:{}:{},{}", hours_string, minutes_string, seconds_string, milliseconds_string);
        time_code
    }



    async fn get_voice_base8_executable_content(clipText: String, voice_name: &String) -> Result<(Vec<u8>, f32), Error>  {

        let voice: &str = voice_name;




        let sample_rate_khz: u32 = 16;
        let bit_rate_kbps: u32 = 128;
        let channels: &str = "mono";



        let url: String = "https://eastus.tts.speech.microsoft.com/cognitiveservices/v1".to_string();

        let body: String = format!(r#"
    <speak version='1.0' xml:lang='en-US'>
    <voice xml:lang='en-US' xml:gender='Female' name='{}'>
         {}
    </voice>
</speak>
"#, voice, clipText);




        let mut res = Client::new()
            .post(url)
            .header( "Ocp-Apim-Subscription-Key", std::env::var("AZURE_TTS_OCP-APIM-SUBSCRIPTION_KEY").expect("AZURE_TTS_OCP-APIM-SUBSCRIPTION_KEY must be set"))
            .header(CONTENT_TYPE, "application/ssml+xml")
            .header("X-Microsoft-OutputFormat",format!("audio-{}khz-{}kbitrate-{}-mp3", sample_rate_khz, bit_rate_kbps, channels))
            .header(USER_AGENT, "curl")
            .body(body)
            .send().await?.bytes().await?;


        let vecot: Vec<u8> = res.as_ref().to_owned();
        let audio_duration: f32 = (vecot.len() as f32 *8.0) / (bit_rate_kbps as f32 *1000.0);
        println!("audio duration: {}", audio_duration);

        Ok((vecot, audio_duration))
    }

}
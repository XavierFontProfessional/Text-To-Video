use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::format;
use reqwest::Error;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use tokio::fs;
use crate::clip::clip::Clip;
use crate::Script;
use crate::word_or_punctuation::word_or_punctuation::Word;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Video {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) instructions: String,
    pub(crate) clips: Vec<Clip>,
    pub(crate) video_filter: Option<VideoFilter>,
    pub(crate) subtitles_font: Option<SubtitlesFont>,
    pub(crate) subtitles_color: Option<String>,
    pub(crate) subtitles_background_color: Option<String>,
    pub(crate) subtitles_position: Option<SubtitlesPosition>,
    pub(crate) music: Option<String>,
    pub(crate) voice_name: Option<String>,
}

impl Video {
    pub fn new(clips: Vec<Clip>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "name goes here".to_string(),
            instructions: "".to_string(),
            clips,
            video_filter: None,
            subtitles_font: None,
            subtitles_color: None,
            subtitles_background_color: None,
            subtitles_position: None,
            music: None,
            voice_name: None,
        }
    }

    pub fn get_video_instructions(&self) -> String {
        ///
        let mut picture_inputs: Vec<String> = Vec::new();
        'decide_and_push_clip_picture_input: for clip in &self.clips {
            if let None = clip.image_url {
                for looking_for_previous_clip_with_image_url in self.clips.iter().rev() {
                    if looking_for_previous_clip_with_image_url.video_ordered_position_number < clip.video_ordered_position_number { // Found a previous clip
                        if let Some(..) = looking_for_previous_clip_with_image_url.clone().image_url {
                            let image_url = &looking_for_previous_clip_with_image_url.image_url.as_ref().unwrap();
                            let inputting_clip_command: String = format!("-loop 1 -t {} -i {}", clip.duration.to_string(), image_url);
                            picture_inputs.push(inputting_clip_command);
                            break;
                        }
                    }
                }
                let inputting_clip_command: String = format!("-loop 1 -t {} -i {}", clip.duration.to_string(), &"https://www.reuters.com/resizer/s1H-W74v_1-MMUDJ4VlcswcmksQ=/1200x0/filters:quality(80)/cloudfront-us-east-2.images.arcpublishing.com/reuters/CQFGZNVKCZJF7PROHV42QLV6JE.jpg".to_string());
                picture_inputs.push(inputting_clip_command);
            } else if let Some(..) = clip.image_url.clone() {
                let clip_with_image_url = clip.clone().image_url;
                let image_url = &clip_with_image_url.unwrap();
                let inputting_clip_command: String = format!("-loop 1 -t {} -i {}", clip.duration.to_string(), image_url);
                picture_inputs.push(inputting_clip_command);
            }
        }
        let picture_inputs_command: String = picture_inputs.join(" ");


        let mut video_filter_input: Option<String> = None;
        if let Some(filter) = &self.video_filter {
            video_filter_input = match filter {
                VideoFilter::BlackAndWhite => { Option::from(String::from("colorbalance=-0.4:-0.2:0.2:-0.4:-0.2:0.2:0:0:0.0")) }
                VideoFilter::Sepia => { Option::from(String::from("colorbalance=:1:.1:.1:0:0:0:0:0:.1:.1:.1:0")) }
            }
        } //

        let mut subtitles_font_input: Option<HashMap<String, String>> = None;
        if let Some(font) = &self.subtitles_font {
            if let Some(..) = &self.subtitles_color {
                if let Some(..) = &self.subtitles_position {
                    subtitles_font_input = Some(HashMap::new());

                    if let Some(ref mut subtitles_font_input_unwrapped) = subtitles_font_input {
                        subtitles_font_input_unwrapped.insert(String::from("fonts_directory"), String::from("fontsdir={fonts_directory}"));
                        subtitles_font_input_unwrapped.insert(String::from("font_style"), format!("FontName={}",
                                                                                                  match font {
                                                                                                      SubtitlesFont::Arial => { "Arial" }
                                                                                                      SubtitlesFont::GimletMicro => { "Gimlet Micro" }
                                                                                                  }
                        ));
                    }
                }
            }
        }             //

        let mut subtitles_color_input: Option<String> = None;
        if let Some(color) = &self.subtitles_color {
            if let Some(..) = &self.subtitles_font {
                if let Some(..) = &self.subtitles_position {
                    subtitles_color_input = Option::from(format!("PrimaryColour=&H{}&", color));
                }
            }
        }                   //
        ///
        let mut subtitles_background_input: Option<String> = None;
        if let Some(color) = &self.subtitles_background_color {
            if let Some(..) = &self.subtitles_font {
                if let Some(..) = &self.subtitles_color {
                    if let Some(..) = &self.subtitles_position {
                        subtitles_background_input = Option::from(format!("-f lavfi -t {} -i color=c={}:s=1080x600", &self.get_video_length(), color));
                    }
                }
            }
        }       //

        let mut subtitles_position_input: Option<HashMap<String, Option<String>>> = None;
        if let Some(position) = &self.subtitles_position {
            if let Some(..) = &self.subtitles_color {
                if let Some(..) = &self.subtitles_font {
                    subtitles_position_input = Some(HashMap::new());
                    if let Some(ref mut subtitles_position_input_unwrapped) = subtitles_position_input {
                        subtitles_position_input_unwrapped.insert(String::from("subtitles_text_position"),
                                                                  match position {
                                                                      SubtitlesPosition::Top => { Option::from(6.to_string()) }
                                                                      SubtitlesPosition::Middle => { Option::from(10.to_string()) }
                                                                      SubtitlesPosition::Bottom => { Option::from(2.to_string()) }
                                                                  },
                        );


                        if let Some(..) = &self.subtitles_background_color {
                            subtitles_position_input_unwrapped.insert(String::from("subtitles_background_position"),
                                                                      match position {
                                                                          SubtitlesPosition::Top => { Option::from(0.to_string()) }
                                                                          SubtitlesPosition::Middle => { Option::from(((1620 - (300 / 2)) / 2).to_string()) }
                                                                          SubtitlesPosition::Bottom => { Option::from(1620.to_string()) }
                                                                      },
                            );
                        }
                    }
                }
            }
        } //
        ///
        let mut music_input: Option<String> = None;
        if let Some(music_url) = &self.music {
            music_input = Option::from(format!("-i {}", music_url));
        }
        ///
        let mut voice_inputs: Option<Vec<String>> = None;
        if let Some(..) = &self.voice_name {
            voice_inputs = Some(Vec::new());
            if let Some(ref mut voice_inputs_unwrapped) = voice_inputs {
                'push_clip_voice_input: for clip in &self.clips {
                    voice_inputs_unwrapped.push(format!("-i voice{}", clip.video_ordered_position_number));
                }
            }
        }

        let mut voice_inputs_command: Option<String> = None;
        if let Some(.., ref voices) = voice_inputs {
            voice_inputs_command = Option::from(voices.join(" "));
        }
        ///
////////////////////////////////////////////
        let mut ffmpeg_inputs: Vec<String> = Vec::new();
        ffmpeg_inputs.push(picture_inputs_command);

        if let Some(.., command) = voice_inputs_command {
            ffmpeg_inputs.push(command);
        }

        if let Some(.., ref command) = music_input {
            ffmpeg_inputs.push(command.to_owned());
        }

        if let Some(.., ref command) = subtitles_background_input {
            ffmpeg_inputs.push(command.to_owned());
        }

        let ffmpeg_inputs: String = ffmpeg_inputs.join(" ");
///////////////////////////////////////////////
        let mut ffmpeg_complex_filter: Vec<String> = Vec::new();
        ffmpeg_complex_filter.push(String::from("-filter_complex"));
        ffmpeg_complex_filter.push(String::from(" "));

        /// SET VIDEO ASPECT RATIO
        for clip in &self.clips {
            ffmpeg_complex_filter.push(format!("[{}:v]scale=1080:1920,setsar=1[{}];", clip.video_ordered_position_number, clip.video_ordered_position_number));
        }

        /// CONCATENATE VIDEO
        let mut picture_concatenation_input: Vec<String> = Vec::new();
        for clip in &self.clips {
            picture_concatenation_input.push(format!("[{}]", clip.video_ordered_position_number));
        }
        ffmpeg_complex_filter.push(format!("{}concat=n={}[concatenatedvideo];", picture_concatenation_input.into_iter().collect::<String>(), &self.clips.len()));

        /// CONCATENATE VOICES
        if let Some(..) = voice_inputs {
            let mut voice_concatenation_inputs: Vec<String> = Vec::new();
            for clip in &self.clips {
                voice_concatenation_inputs.push(format!("[{}]", clip.video_ordered_position_number + &self.clips.len()));
            }
            ffmpeg_complex_filter.push(format!("{}concat=n={}:v=0:a=1[concatenatedvoices];", voice_concatenation_inputs.into_iter().collect::<String>(), &self.clips.len()));
        }


        /// OVERLAY SUBTITLES BG (with position)
        if let Some(..) = subtitles_background_input {
            if let Some(.., ref has_position) = subtitles_position_input {
                if let Some(.., position) = &has_position["subtitles_background_position"] {
                    if let Some(..) = voice_inputs {
                        if let Some(..) = music_input { // has voice and music
                            ffmpeg_complex_filter.push(format!("[{}:v]format=argb,\"geq=r='r(X,Y)'\":\"a='0.75*alpha(X,Y)'[withOpacitySet];\"", (&self.clips.len() * 2+1)));
                            ffmpeg_complex_filter.push(format!("[concatenatedvideo][withOpacitySet]overlay=shortest=0:{}[withsubtitlesbg];", position));
                        } else { // has voice
                            ffmpeg_complex_filter.push(format!("[{}:v]format=argb,\"geq=r='r(X,Y)'\":\"a='0.75*alpha(X,Y)'[withOpacitySet];\"", (&self.clips.len() * 2)));
                            ffmpeg_complex_filter.push(format!("[concatenatedvideo][withOpacitySet]overlay=shortest=0:{}[withsubtitlesbg];", position));
                        }
                    } else {
                        if let Some(..) = music_input { // has music
                            ffmpeg_complex_filter.push(format!("[{}:v]format=argb,\"geq=r='r(X,Y)'\":\"a='0.75*alpha(X,Y)'[withOpacitySet];\"", (&self.clips.len() + 1)));
                            ffmpeg_complex_filter.push(format!("[concatenatedvideo][withOpacitySet]overlay=shortest=0:{}[withsubtitlesbg];", position));
                        } else { // Has nothing (NO voice and NO music)
                            ffmpeg_complex_filter.push(format!("[{}:v]format=argb,\"geq=r='r(X,Y)'\":\"a='0.75*alpha(X,Y)'[withOpacitySet];\"", (&self.clips.len())));
                            ffmpeg_complex_filter.push(format!("[concatenatedvideo][withOpacitySet]overlay=shortest=0:{}[withsubtitlesbg];", position));
                        }
                    }
                }
            }
        }

        /// ADD SUBTITLES (with font, color, position)
        if let Some(.., ref style) = subtitles_font_input {
            if let Some(.., ref color) = subtitles_color_input {
                if let Some(ref has_position) = subtitles_position_input {
                    if let Some(.., position) = &has_position["subtitles_text_position"] {
                        if let Some(..) = subtitles_background_input {
                            ffmpeg_complex_filter.push(format!("[withsubtitlesbg]subtitles={{subtitles_path}}:{}:\"force_style='{},{},Outline=0,Shadow=0,Alignment={}'[concatenatedvideowithsubtitles]\"", style["fonts_directory"], style["font_style"], color, position));
                        } else {  // NO BG
                            ffmpeg_complex_filter.push(format!("[concatenatedvideo]subtitles={{subtitles_path}}:{}:\"force_style='{},{},Outline=0,Shadow=0,Alignment={}'[concatenatedvideowithsubtitles]\"", style["fonts_directory"], style["font_style"], color, position));
                        }
                    }
                }
            }
        }

        let mut ffmpeg_complex_filter: String = ffmpeg_complex_filter.into_iter().collect();
        ///TODO: Take out last semi-collin programmatically
        // ffmpeg_complex_filter.pop();


        ///////////////////////////////////
        ///TODO: Outputs are obviously messed up, fix them
        let mut ffmpeg_media_mapping: Vec<String> = Vec::new();
        ffmpeg_media_mapping.push(String::from("-map"));
        if let Some(..) = subtitles_font_input {
            if let Some(..) = subtitles_color_input {
                if let Some(..) = subtitles_background_input {
                    ffmpeg_media_mapping.push(String::from("[concatenatedvideowithsubtitles]:v"));
                } else {
                    ffmpeg_media_mapping.push(String::from("[concatenatedvideowithsubtitles]:v"));
                }
            }
        } else {
            ffmpeg_media_mapping.push(String::from("[concatenatedvideowithsubtitles]:v"));
        }
        if let Some(..) = voice_inputs {
            ffmpeg_media_mapping.push(String::from("-map"));
            ffmpeg_media_mapping.push(String::from("[concatenatedvoices]:a"));
        }

        let ffmpeg_media_mapping = ffmpeg_media_mapping.join(" ");

        //////////////////////////////////
        let mut ffmpeg_outputting: Vec<String> = Vec::new();
        ffmpeg_outputting.push(String::from("-y "));
        let ffmpeg_outputting: String = ffmpeg_outputting.join(" ");

        ////////////////////////////////
        let final_ffmpeg_command: Vec<String> = vec![ffmpeg_inputs, ffmpeg_complex_filter, ffmpeg_media_mapping, ffmpeg_outputting];
        let final_ffmpeg_command: String = final_ffmpeg_command.join(" ");

        return final_ffmpeg_command;
    }


    pub async fn generate_subtitles_srt_file(&self) -> String {
        let mut subtitle_command_per_clip: Vec<String> = Vec::new();
        let mut video_length_so_far_seconds: f32 = 0.0;

        // let mut styling_opening_tag: String = format!("<font size=\"16px\" color=\"{}\">", &self.subtitles_color);
        // let mut styling_closing_tag: String = "</font>".to_string();

        for clip in &self.clips {
            // subtitle_command_per_clip.push(format!("{}\n{} --> {}\n{}{}{}\n\n", clip.video_ordered_position_number, Clip::get_time_code(video_length_so_far_seconds), Clip::get_time_code(video_length_so_far_seconds + clip.duration), styling_opening_tag, clip.text, styling_closing_tag));
            subtitle_command_per_clip.push(format!("{}\n{} --> {}\n{}\n\n", clip.video_ordered_position_number, Clip::get_time_code(video_length_so_far_seconds), Clip::get_time_code(video_length_so_far_seconds + clip.duration), clip.text));
            video_length_so_far_seconds = video_length_so_far_seconds + clip.duration;
        }
        subtitle_command_per_clip.concat()
    }

    pub fn get_clip_by_video_ordered_position_number(&self, position: usize) -> Option<&Clip> {
        for clip in &self.clips {
            if clip.video_ordered_position_number == position {
                return Some(&clip);
            }
        }
        return None;
    }


    pub fn get_video_length(&self) -> f32 {
        let mut video_duration: f32 = 0.0;
        for clip in &self.clips {
            video_duration = video_duration + clip.duration;
        }
        video_duration
    }
}

#[derive(Debug, Clone)]
pub enum VcrEngine {
    SentenceASAVAO,
    SubsentenceASAVAO,
    SentenceAVAO,
    SubsentenceAVAO,
    SentenceAO,
    SubsentenceAO,
    SentenceFULL,
    SubsentenceFULL,
    SentenceScriptDesc,
    SubtenceScriptDesc,
    SentenceBabyAI,
    SubtenceBabyAI,
    MasterBabyAI,
}

#[derive(Debug, Clone)]
pub enum VideoFilter {
    BlackAndWhite,
    Sepia,
}

#[derive(Debug, Clone)]
pub enum SubtitlesFont {
    Arial,
    GimletMicro,
}

#[derive(Debug, Clone)]
pub enum SubtitlesPosition {
    Top,
    Middle,
    Bottom,
}
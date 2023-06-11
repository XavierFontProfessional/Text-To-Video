//
//  ScriptToVideoPage.swift
//  AppleFontEnd
//
//  Created by Maemi on 2022-08-01.
//

import SwiftUI
import AVKit
import SnapToScroll
import ffmpegkit




struct ScriptToVideoPage: View {
    
    
    
    
    
    
    @State var textFieldContent: String = ""
    @State var textFieldIsActive: Bool = false
    @State var bringVideoStylePreviewToFront: Bool = false
    
    
    @State var filterChosenOption: String = "BlackAndWhite"
    @State var fontChosenOption: String = "Arial"
    @State var colorChosenOption: String = "000000"
    @State var backgroundChosenOption: String = "000000"
    @State var positionChosenOption: String = "Middle"
    @State var musicChosenOption: String = "https://cdn.pixabay.com/download/audio/2022/08/02/audio_884fe92c21.mp3?filename=inspiring-cinematic-ambient-116199.mp3"
    @State var voiceChosenOption: String = "en-US-SaraNeural"
    
    @State var filterIsActive: Bool = false
    @State var subtitlesIsActive: Bool = true
    @State var subtitlesBackgroundIsActive: Bool = true
    @State var musicIsActive: Bool = true
    @State var voiceIsActive: Bool = true
    
    @State var showSubtitleColorPickerOverlay: Bool = false
    @State var subtitleColorFromPicker: Color = Color.white
    @State var showBackgroundColorPickerOverlay: Bool = false
    @State var backgroundColorFromPicker: Color = Color.gray
    
    @State var chosenRenderEngine: RenderEngine = RenderEngine.SentenceASAVAO
    
    
    
    var body: some View {
        
        GeometryReader{ geometry in
            
            
            NavigationView{
                ZStack(alignment: .topLeading){
                    // TOP BG COLOR
                    VStack{
                        Color.teal.frame(height: geometry.size.height * 0.5).edgesIgnoringSafeArea(.top)
                        Spacer()
                    }
                    
                    // Video Preview
                    VStack (spacing: geometry.size.height * 0.02){
                        ZStack(alignment: .topLeading){
                            HStack{
                                VideoPlayer(player: AVPlayer(url:  URL(string: "https://bit.ly/swswift")!))
                                    .frame(width:(geometry.size.height * 0.3341232227)/1.7777777778, height: geometry.size.height * 0.3341232227)
                                    .cornerRadius(10)
                                    .shadow(color: .gray, radius: 6, x: 0, y: 3)
                                    .onTapGesture {
                                        bringVideoStylePreviewToFront = true
                                    }
                                Spacer()
                            }
                            
                            HStack{
                                Spacer()
                                VStack{
                                    Spacer().frame(height: geometry.size.height * 0.07701421801)
                                    
                                    
                                    OptionsScrollView(filterChosenOption: $filterChosenOption, fontChosenOption: $fontChosenOption, colorChosenOption: $colorChosenOption, backgroundChosenOption: $backgroundChosenOption, positionChosenOption: $positionChosenOption, musicChosenOption: $musicChosenOption, voiceChosenOption: $voiceChosenOption, filterIsActive: $filterIsActive, subtitlesIsActive: $subtitlesIsActive, subtitlesBackgroundIsActive: $subtitlesBackgroundIsActive, musicIsActive: $musicIsActive, voiceIsActive: $voiceIsActive, showSubtitleColorPickerOverlay: $showSubtitleColorPickerOverlay, subtitleColorFromPicker: $subtitleColorFromPicker, showBackgroundColorPickerOverlay: $showBackgroundColorPickerOverlay, backgroundColorFromPicker: $backgroundColorFromPicker).frame(width: geometry.size.width * 0.6743589744, height: geometry.size.height * 0.2263033175, alignment: .topLeading)
                                }
                            }.onTapGesture {
                                bringVideoStylePreviewToFront = false
                            }
                            
                            if bringVideoStylePreviewToFront == true {
                                HStack{
                                    VideoPlayer(player: AVPlayer(url:  URL(string: "https://bit.ly/swswift")!))
                                        .frame(width: geometry.size.width * 0.4076923077, height: geometry.size.height * 0.3341232227)
                                        .cornerRadius(10)
                                        .shadow(color: .gray, radius: 6, x: 0, y: 3)
                                    Spacer()
                                }
                            }
                            
                        }.frame(alignment: .topLeading)
                        
                        ZStack(alignment: .topLeading) {
                            TextEditor(text: $textFieldContent)
                                .font(.custom("Helvetica", size: 18))
                                .cornerRadius(10)
                                .frame(height: geometry.size.height * 0.2736966825)
                                .shadow(color: .gray, radius: 6, x: 0, y: 3)
                                .onTapGesture {
                                    textFieldIsActive = true
                                }
                            
                            
                            if textFieldIsActive == false {
                                
                                Text(chosenRenderEngine.rawValue)
                                    .foregroundColor(Color.gray)
                                    .font(.custom("Helvetica", size: 18))
                                    .padding()
                                
                            }
                        }.frame(height: geometry.size.height * 0.2736966825)
                        
                        RenderEngineSlider(chosenRenderEngine: $chosenRenderEngine)
                            .frame(height: geometry.size.height * 0.07345971564, alignment: .topLeading)
                            .mask (
                                Color.white
                                    .frame(height: geometry.size.height)
                            )
                            .shadow(color: .gray, radius: 6, x: 0, y: 3)

                        
                        
                        HStack{
                            Spacer()
                            Button(action: {
                                let videoId: UUID = UUID()
                                
                                let url = URL(string: "http://192.168.1.104:8080/video_instructions")!
                                var request = URLRequest(url: url)
                                request.setValue("application/json", forHTTPHeaderField: "Content-Type")
                                request.httpMethod = "POST"
                                
//                                if colorChosenOption == "custom" {
//                                    colorChosenOption = hexStringFromColor(color: UIColor(subtitleColorFromPicker))
//                                }
                                
                                
                                
                                let videoInstructionsRequest: VideoInstructionsRequest = VideoInstructionsRequest(script: textFieldContent, renderEngine: chosenRenderEngine.rawValue, filter: filterIsActive ? filterChosenOption : nil, subtitlesFont: subtitlesIsActive ? fontChosenOption : nil, subtitlesColor: subtitlesIsActive ? colorChosenOption : nil, subtitlesBackground: subtitlesIsActive && subtitlesBackgroundIsActive ? backgroundChosenOption : nil, subtitlesPosition:  subtitlesIsActive ? positionChosenOption : nil, music: musicIsActive ? musicChosenOption : nil, voice: voiceIsActive ? voiceChosenOption : nil)
                                
                                do {
                                    let jsonData = try JSONEncoder().encode(videoInstructionsRequest)
                                    
                                    
                                    request.httpBody = jsonData
                                    let task = URLSession.shared.dataTask(with: request) { (data, response, error) in
                                        
                                        // Check for Error
                                        if let error = error {
                                            print("Error took place \(error)")
                                            return
                                        }
                      
                                        //////
                                        ///
                                        
                                        // Convert HTTP Response Data to a String
                                        if let data = data, let _ = String(data: data, encoding: .utf8) {
                                            
                                            let videoInstructionsAndSubtitlesResponse: VideoInstructionsAndSubtitlesResponse = try! JSONDecoder().decode(VideoInstructionsAndSubtitlesResponse.self, from: data);
                                            let subtitlesPath = writeSubtitles(content: videoInstructionsAndSubtitlesResponse.subtitles);
                                            var command = videoInstructionsAndSubtitlesResponse.instructions.replacingOccurrences(of: "{subtitles_path}", with: subtitlesPath.path);
                                            command = command.replacingOccurrences(of: "{fonts_directory}", with: "/Users/maemi/Downloads/Open_Sans/static/OpenSans");
                                            command = command.replacingOccurrences(of: "Arial", with: "OpenSans-Medium");
                                            if let voices = videoInstructionsAndSubtitlesResponse.voices {
                                                let voicesUrls: [URL] = writeVoices(content: voices)
                                                for (i, voiceUrl) in voicesUrls.enumerated() {
                                                    command = command.replacingOccurrences(of: "voice\(i)", with: voiceUrl.path);
                                                }
                                            }
                                            
                                            
                                            
                                            print("FINAL COMMAND: \(command)");
                                             ffmpegCommandExcecuterAsync(command: command, fileName: String(videoId.uuidString))
                                            
                                        }
                                    }
                                    task.resume()
                                } catch { print(error) }
                                
                            }, label: {
                                Text("Generate Video")
                                    .padding()
                                    .foregroundColor(.white)
                                    .background(.teal)
                                    .cornerRadius(5)
                                    .shadow(color: .gray, radius: 6, x: 0, y: 3)
                            })
                        }
                        
                        
                        
                    }.padding(geometry.size.width * 0.06794871795)
                    if showSubtitleColorPickerOverlay == true {
                        ColorPicker("", selection: $subtitleColorFromPicker)
                    }
                    
                    if showBackgroundColorPickerOverlay == true {
                        ColorPicker("", selection: $backgroundColorFromPicker)
                    }
                    
                    
                    
                    
                    
                }.navigationTitle("Text to Video").navigationViewStyle(StackNavigationViewStyle())
                    .overlay {
                        ColorPicker("", selection: $subtitleColorFromPicker)
                            .labelsHidden()
                        //                                                    .opacity(0.015)
                    }
            }.navigationViewStyle(StackNavigationViewStyle())
            
        }
        
    }
    
    
    struct ScriptToVideoPage_Previews: PreviewProvider {
        static var previews: some View {
            ScriptToVideoPage()
        }
    }
    
    
    
    
    struct VideoInstructionsRequest: Codable {
        let script: String?
        let renderEngine: String?
        let filter: String?
        let subtitlesFont: String?
        let subtitlesColor: String?
        let subtitlesBackground: String?
        let subtitlesPosition: String?
        let music: String?
        let voice: String?
    }
    
    struct VideoInstructionsAndSubtitlesResponse: Codable {
        let instructions: String
        let subtitles: String
        let voices: [[UInt8]]?
    }
    
    
    
    
    
    func ffmpegCommandExcecuterAsync(command: String, fileName: String) {
        guard let outputPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first else { return }
        let output = outputPath.appendingPathComponent("\(fileName).mp4")
        try? FileManager.default.removeItem(at: output)
        FFmpegKit.executeAsync("\(command)  \(output.path)") { session in
            guard let session = session else {
                print("!! Invalid session")
                return
            }
            guard let returnCode = session.getReturnCode() else {
                print("!! Invalid return code")
                return
            }
            print("FFmpeg process exited with state \(FFmpegKitConfig.sessionState(toString: session.getState()) ?? "Unknown") and rc \(returnCode).\(session.getFailStackTrace() ?? "Unknown")")
            
            DispatchQueue.main.async {
                if ReturnCode.isSuccess(returnCode) {
                    print("Video at \(output.path)")
                } else {
                    print("Error creating video...")
                }
            }
        } withLogCallback: { logs in
            guard logs != nil else { return }
            // CALLED WHEN SESSION PRINTS LOGS
        } withStatisticsCallback: { stats in
            guard stats != nil else { return }
            // CALLED WHEN SESSION GENERATES STATISTICS
        }
    }
    
    
    
    
    func writeSubtitles(content: String) -> URL {
        let paths = FileManager.default.urls(for: .cachesDirectory, in: .userDomainMask)
        let path = paths[0]
        
        let url = path.appendingPathComponent("subtitles.srt")
        
        do {
            try content.write(to: url, atomically: true, encoding: .utf8)
//            let input = try String(contentsOf: url)
        } catch {
            print(error.localizedDescription)
        }
        return url
    }
    
    func writeVoices(content: [[UInt8]]) -> [URL] {
        let paths = FileManager.default.urls(for: .cachesDirectory, in: .userDomainMask)
        let path = paths[0]
        var voicesUrls: [URL] = []
        for (i, voice_instruction) in content.enumerated() {
            let data: Data = Data(voice_instruction);
            let url = path.appendingPathComponent("audio\(i).mp3")
            voicesUrls.append(url)
        do {
            try data.write(to: url)
//            let input = try String(contentsOf: url)
        } catch {
            print(error.localizedDescription)
        }
        }
        return voicesUrls
    }
}


extension Color {
    var uiColor: UIColor { .init(self) }
    typealias RGBA = (red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat)
    var rgba: RGBA? {
        var (r, g, b, a): RGBA = (0, 0, 0, 0)
        return uiColor.getRed(&r, green: &g, blue: &b, alpha: &a) ? (r, g, b, a) : nil
    }
    var hexaRGB: String? {
        guard let (red, green, blue, _) = rgba else { return nil }
        return String(format: "#%02x%02x%02x",
                      Int(red * 255),
                      Int(green * 255),
                      Int(blue * 255))
    }
    var hexaRGBA: String? {
        guard let (red, green, blue, alpha) = rgba else { return nil }
        return String(format: "#%02x%02x%02x%02x",
                      Int(red * 255),
                      Int(green * 255),
                      Int(blue * 255),
                      Int(alpha * 255))
    }
}


func hexStringFromColor(color: UIColor) -> String {
    let components = color.cgColor.components
    let r: CGFloat = components?[0] ?? 0.0
    let g: CGFloat = components?[1] ?? 0.0
    let b: CGFloat = components?[2] ?? 0.0
    
    let hexString = String.init(format: "#%02lX%02lX%02lX", lroundf(Float(r * 255)), lroundf(Float(g * 255)), lroundf(Float(b * 255)))
    print(hexString)
    return hexString
}

public enum RenderEngine: String, Codable {
    case SentenceASAVAO
    case SentenceAVAO
    case SentenceAO
    case SentenceFULL
    case SentenceBabyAI
}

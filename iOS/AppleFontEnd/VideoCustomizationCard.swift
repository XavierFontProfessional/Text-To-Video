////
////  VideoCustomizationCard.swift
////  AppleFontEnd
////
////  Created by Maemi on 2022-08-02.
////
//
//import SwiftUI
//import AVFoundation
//
//var player: AVAudioPlayer?
//
//
//
//
//struct VideoCustomizationCard: View {
//    let id: String?
//    @State var selectedIOption: Int = 1
//    
//    
//    let customizationTitle: String?
//    
//    
//    @State var isActive: Bool = false
//
//    
//    
//    var body: some View {
//        GeometryReader { geometry in
//            VStack(){
//                
//                Toggle(customizationTitle!, isOn: $isActive)
//                
//                
//                switch customizationTitle {
//                case "Filter":
//                    HStack{
//                        CustomizationOptionImage(customizationCardId: 1, id: 1, overlay: nil, backgoundImage: "Green_Building_Black_and_White", width: geometry.size.width * 0.31, height: geometry.size.height * 0.743, selectedIOption: $selectedIOption)
//                        CustomizationOptionImage(customizationCardId: 1, id: 2, overlay: nil, backgoundImage: "Green_Building_Retro_Vintage", width: geometry.size.width * 0.31, height: geometry.size.height * 0.743, selectedIOption: $selectedIOption)
//                        CustomizationOptionCruz(customizationCardId: 1, id: 3, width: geometry.size.width * 0.31, height: geometry.size.height * 0.743, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                    }
//                case "Font Style":
//                    HStack{
//                        VStack{
//                            CustomizationOptionText(customizationCardId: 2, id: 1, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438)
//                            CustomizationOptionText(customizationCardId: 2, id: 2, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438)
//                            
//                        }
//                        VStack{
//                            CustomizationOptionText(customizationCardId: 2, id: 3, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438)
//                            CustomizationOptionText(customizationCardId: 2, id: 4, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438)
//                            
//                        }
//                        CustomizationOptionCruz(customizationCardId: 2, id: 5, width: geometry.size.width * 0.31, height: geometry.size.height * 0.743, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                    }
//                case "Font Color":
//                    HStack{
//                        CustomizationOptionSolidColor(customizationCardId: 3, id: 1, color: Color.white, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                        CustomizationOptionSolidColor(customizationCardId: 3, id: 2, color: Color.black, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                        CustomizationOptionColorPicker(customizationCardId: 3, id: 3, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                    }
//                    
//                case "Subtitles Background":
//                    HStack{
//                        CustomizationOptionSolidColor(customizationCardId: 4, id: 1, color: Color.white, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                        CustomizationOptionSolidColor(customizationCardId: 4, id: 2, color: Color.gray, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                        CustomizationOptionColorPicker(customizationCardId: 4, id: 3, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                    }
//                case "Subtitles Position":
//                    HStack{
//                        CustomizationOptionSubtitlePosition(customizationCardId: 5, id: 1, position: SubtitlePosition.Top, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                        CustomizationOptionSubtitlePosition(customizationCardId: 5, id: 2, position: SubtitlePosition.Middle, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                        CustomizationOptionSubtitlePosition(customizationCardId: 5, id: 3, position: SubtitlePosition.Bottom, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                    }
//                case "Music":
//                    HStack{
//                        VStack{
//                            CustomizationOptionImage(customizationCardId: 6, id: 1, overlay: nil, backgoundImage: "Green_Building_Black_and_White", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            CustomizationOptionImage(customizationCardId: 6, id: 2, overlay: nil, backgoundImage: "Green_Building_Black_and_White", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            
//                        }
//                        VStack{
//                            CustomizationOptionImage(customizationCardId: 6, id: 3, overlay: nil, backgoundImage: "Green_Building_Black_and_White", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            CustomizationOptionImage(customizationCardId: 6, id: 4, overlay: nil, backgoundImage: "Green_Building_Black_and_White", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            
//                        }
//                        CustomizationOptionCruz(customizationCardId: 6, id: 5, width: geometry.size.width * 0.31, height: geometry.size.height * 0.743, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                    }
//                case "Voice":
//                    HStack{
//                        VStack{
//                            CustomizationOptionImage(customizationCardId: 7, id: 1, overlay: nil, backgoundImage: "male_ai_voice_image_1", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            CustomizationOptionImage(customizationCardId: 7, id: 2, overlay: nil, backgoundImage: "female_ai_voice_image_1", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            
//                        }
//                        VStack{
//                            
//                            CustomizationOptionImage(customizationCardId: 7, id: 3, overlay: nil, backgoundImage: "female_ai_voice_image_1", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            CustomizationOptionImage(customizationCardId: 7, id: 4, overlay: nil, backgoundImage: "female_ai_voice_image_2", width: geometry.size.width * 0.31, height: geometry.size.height * 0.3438, selectedIOption: $selectedIOption)
//                            
//                        }
//                        CustomizationOptionCruz(customizationCardId: 7, id: 5, width: geometry.size.width * 0.31, height: geometry.size.height * 0.743, selectedIOption: $selectedIOption).frame(width: geometry.size.width * 0.31, height: geometry.size.height * 0.743)
//                    }
//                case .none:
//                    Text("error")
//                case .some(_):
//                    Text("error")
//                }
//            }
//            
//        }
//        
//        
//    }
//    
//    
//    
//    
//}
//
//struct VideoCustomizationCard_Previews: PreviewProvider {
//    static var previews: some View {
//        VideoCustomizationCard(id: "1", customizationTitle: "Title")
//    }
//}
//
//
//
//
//
//
//
//
//
//////////////////
/////
/////
/////
/////
/////
/////
/////
//
//struct VerticalLine: Shape {
//    func path(in rect: CGRect) -> Path {
//        var path = Path()
//        
//        path.move(to: CGPoint(x: rect.midX, y: rect.maxY))
//        path.addLine(to: CGPoint(x: rect.midX, y: rect.minY))
//        
//        return path
//    }
//}
//
//
//struct HorizontalLine: Shape {
//    func path(in rect: CGRect) -> Path {
//        var path = Path()
//        
//        path.move(to: CGPoint(x: rect.minX, y: rect.midY))
//        path.addLine(to: CGPoint(x: rect.maxX, y: rect.midY))
//        
//        return path
//    }
//}
//
//struct Cruz: View {
//    var body: some View {
//        ZStack{
//            VerticalLine()
//                .stroke(.gray, style: StrokeStyle(lineWidth: 5, lineCap: .round, lineJoin: .round))
//            HorizontalLine()
//                .stroke(.gray, style: StrokeStyle(lineWidth: 5, lineCap: .round, lineJoin: .round))
//        }
//    }
//}
//

//
//
//
//func playSound(soundName: String) {
//    guard let path = Bundle.main.path(forResource: soundName, ofType:"mp3") else {
//        return }
//    let url = URL(fileURLWithPath: path)
//    
//    do {
//        player = try AVAudioPlayer(contentsOf: url)
//        player?.play()
//        
//    } catch let error {
//        print(error.localizedDescription)
//    }
//}
//
//
//public enum CardArrangement {
//    case TwoRow
//    case OneRow
//    case automation
//}
//
////protocol CustomizationOption {
//////    var selected: Bool {get}
////}

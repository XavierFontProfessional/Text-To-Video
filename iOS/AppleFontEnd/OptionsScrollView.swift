//
//  ContentView.swift
//  iOSFontEnd
//
//  Created by Maemi on 2022-08-07.
//

import SwiftUI
import SnapToScroll
import AVFoundation

var player: AVAudioPlayer?

struct OptionsScrollView: View {
    
    @Binding var filterChosenOption: String
    @Binding var fontChosenOption: String
    @Binding var colorChosenOption: String
    @Binding var backgroundChosenOption: String
    @Binding var positionChosenOption: String
    @Binding var musicChosenOption: String
    @Binding var voiceChosenOption: String
    
    @Binding var filterIsActive: Bool
    @Binding var subtitlesIsActive: Bool
    @Binding var subtitlesBackgroundIsActive: Bool
    @Binding var musicIsActive: Bool
    @Binding var voiceIsActive: Bool
    
    @Binding var showSubtitleColorPickerOverlay: Bool
    @Binding var subtitleColorFromPicker: Color
    @Binding var showBackgroundColorPickerOverlay: Bool
    @Binding var backgroundColorFromPicker: Color
    
    
    
    var body: some View {
        
        GeometryReader { geometry in
            HStack{
                
                
                
                ForEach(
                    [
                        VideoCustomization(title: "Filter", layout: VideoCustomizationLayout.OneByThree, options: [
                            VideoCutomizationOption(id: "black_and_white_filter", activeOption: $filterChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "Green_Building_Black_and_White", width: geometry.size.width * 0.32, height: geometry.size.height * 0.76735)), customizationIsActive: $filterIsActive),
                            VideoCutomizationOption(id: "vintage_filter", activeOption: $filterChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "Green_Building_Retro_Vintage", width: geometry.size.width * 0.32, height: geometry.size.height * 0.76735)), customizationIsActive: $filterIsActive),
                            VideoCutomizationOption(id: "custom", activeOption: $filterChosenOption, optionView: AnyView(CustomizationOptionCruz()), customizationIsActive: $filterIsActive)
                        ], isActive: $filterIsActive, activeOption: $filterChosenOption),
                        VideoCustomization(title: "Subtitles Font", layout: VideoCustomizationLayout.TwoByTwoPlusOneLargeAtTheEnd, options: [
                            VideoCutomizationOption(id: "Arial", activeOption: $fontChosenOption, optionView: AnyView(CustomizationOptionText()), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "Arial", activeOption: $fontChosenOption, optionView: AnyView(CustomizationOptionText()), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "Arial", activeOption: $fontChosenOption, optionView: AnyView(CustomizationOptionText()), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "Arial", activeOption: $fontChosenOption, optionView: AnyView(CustomizationOptionText()), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "custom", activeOption: $fontChosenOption, optionView: AnyView(CustomizationOptionColorPicker()), customizationIsActive: $subtitlesIsActive)
                        ], isActive: $subtitlesIsActive, activeOption: $fontChosenOption),
                        VideoCustomization(title: "Subtitles Color", layout: VideoCustomizationLayout.OneByThree, options: [
                            VideoCutomizationOption(id: "ffffff", activeOption: $colorChosenOption, optionView: AnyView(CustomizationOptionSolidColor(color: .white)), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "000000", activeOption: $colorChosenOption, optionView: AnyView(CustomizationOptionSolidColor(color: .black)), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "custom", activeOption: $colorChosenOption, optionView: AnyView(CustomizationOptionColorPicker()
                                                                                                                        
                                                                                                                       ), customizationIsActive: $subtitlesIsActive)
                        ], isActive: $subtitlesIsActive, activeOption: $colorChosenOption),
                        VideoCustomization(title: "Subtitles Background", layout: VideoCustomizationLayout.OneByThree, options: [
                            VideoCutomizationOption(id: "FFFFFF", activeOption: $backgroundChosenOption, optionView: AnyView(CustomizationOptionSolidColor(color: .white)), customizationIsActive: $subtitlesBackgroundIsActive),
                            VideoCutomizationOption(id: "808080", activeOption: $backgroundChosenOption, optionView: AnyView(CustomizationOptionSolidColor(color: .gray)), customizationIsActive: $subtitlesBackgroundIsActive),
                            VideoCutomizationOption(id: "custom", activeOption: $backgroundChosenOption, optionView:  AnyView(CustomizationOptionColorPicker()), customizationIsActive: $subtitlesBackgroundIsActive)
                        ], isActive: $subtitlesBackgroundIsActive, activeOption: $backgroundChosenOption),
                        VideoCustomization(title: "Subtitles Position", layout: VideoCustomizationLayout.OneByThree, options: [
                            VideoCutomizationOption(id: "Top", activeOption: $positionChosenOption, optionView: AnyView(CustomizationOptionSubtitlePosition(position: .Top)), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "Middle", activeOption: $positionChosenOption, optionView: AnyView(CustomizationOptionSubtitlePosition(position: .Middle)), customizationIsActive: $subtitlesIsActive),
                            VideoCutomizationOption(id: "Bottom", activeOption: $positionChosenOption, optionView: AnyView(CustomizationOptionSubtitlePosition(position: .Bottom)), customizationIsActive: $subtitlesIsActive)
                        ], isActive: $subtitlesIsActive, activeOption: $positionChosenOption),
                        VideoCustomization(title: "Music", layout: VideoCustomizationLayout.TwoByTwoPlusOneLargeAtTheEnd, options: [
                            VideoCutomizationOption(id: "https://cdn.pixabay.com/download/audio/2022/07/26/audio_112f2d606c.mp3?filename=cinematic-time-lapse-115672.mp3", activeOption: $musicChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "dreams-X2", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $musicIsActive),
                            VideoCutomizationOption(id: "https://cdn.pixabay.com/download/audio/2022/07/26/audio_112f2d606c.mp3?filename=cinematic-time-lapse-115672.mp3", activeOption: $musicChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "dreams-X2", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $musicIsActive),
                            VideoCutomizationOption(id: "https://cdn.pixabay.com/download/audio/2022/07/26/audio_112f2d606c.mp3?filename=cinematic-time-lapse-115672.mp3", activeOption: $musicChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "dreams-X2", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $musicIsActive),
                            VideoCutomizationOption(id: "https://cdn.pixabay.com/download/audio/2022/07/26/audio_112f2d606c.mp3?filename=cinematic-time-lapse-115672.mp3", activeOption: $musicChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "dreams-X2", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $musicIsActive),
                            VideoCutomizationOption(id: "custom", activeOption: $musicChosenOption, optionView: AnyView(CustomizationOptionCruz()), customizationIsActive: $musicIsActive),
                        ], isActive: $musicIsActive, activeOption: $musicChosenOption),
                        VideoCustomization(title: "Voice", layout: VideoCustomizationLayout.TwoByTwoPlusOneLargeAtTheEnd, options: [
                            VideoCutomizationOption(id: "en-US-JasonNeural", activeOption: $voiceChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "male_ai_voice_image_1", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $voiceIsActive),
                            VideoCutomizationOption(id: "en-US-NancyNeural", activeOption: $voiceChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "female_ai_voice_image_1", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $voiceIsActive),
                            VideoCutomizationOption(id: "en-US-DavisNeural", activeOption: $voiceChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "male_ai_voice_image_2", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $voiceIsActive),
                            VideoCutomizationOption(id: "en-US-SaraNeural", activeOption: $voiceChosenOption, optionView: AnyView(CustomizationOptionImage(overlay: nil, backgoundImage: "female_ai_voice_image_2", width: geometry.size.width * 0.32, height: geometry.size.height * 0.3525)), customizationIsActive: $voiceIsActive),
                            VideoCutomizationOption(id: "custom", activeOption: $voiceChosenOption, optionView: AnyView(CustomizationOptionCruz()), customizationIsActive: $voiceIsActive),
                        ], isActive: $voiceIsActive, activeOption: $voiceChosenOption)
                    ]
                ){cust in
                    
                    VStack {
                        Toggle(cust.title, isOn: cust.$isActive)
                        if cust.layout == VideoCustomizationLayout.OneByThree{
                            HStack(spacing: geometry.size.width * 0.025){
                                ForEach(cust.options){option in
                                    option
                                }
                            }
                        }else{
                            
                            HStack(spacing: geometry.size.width * 0.025){
                                VStack{
                                    cust.options[0]
                                    cust.options[1]
                                }
                                VStack{
                                    cust.options[2]
                                    cust.options[3]
                                }
                                cust.options[4]
                            }
                            
                        }
                    }
                    .frame(width: geometry.size.width, height: geometry.size.height, alignment: .topLeading)
                    .padding(7.5)
                    .background(.white)
                    .cornerRadius(10)
                    .shadow(color: .gray, radius: 6, x: 0, y: 3)
                    
                    
                }
            }
            .frame(width: geometry.size.width, height: geometry.size.height)
            .modifier(ScrollingHStackModifier(items: 7, itemWidth: geometry.size.width, itemSpacing: 10))
        }
    }
    
}




//
//struct OptionsScrollView_Previews: PreviewProvider {
//    static var previews: some View {
//        OptionsScrollView()
//    }
//}

struct VideoCustomization: Identifiable {
    
    
    let id: UUID = UUID()
    let title: String
    let layout: VideoCustomizationLayout
    let options: [VideoCutomizationOption]
    @Binding var isActive: Bool
    @Binding var activeOption: String
    
    
}



enum VideoCustomizationLayout {
    case OneByThree
    case TwoByTwoPlusOneLargeAtTheEnd
}



struct VideoCutomizationOption: View, Identifiable {
    let id: String
    @Binding var activeOption: String
    @State var isSelected: Bool = false
    let optionView: AnyView
    @Binding var customizationIsActive: Bool
    
    
    var body: some View{
        optionView
            .cornerRadius(5)
            .shadow(color: .gray, radius: 6, x: 0, y: 3)
            .border(activeOption == self.id && customizationIsActive ? .red: .clear, width: 5)
        
            .simultaneousGesture(TapGesture().onEnded{
                if customizationIsActive == false {
                    customizationIsActive.toggle()
                }
                activeOption = self.id
            })
    }
}



struct ScrollingHStackModifier: ViewModifier {
    
    
    @State private var scrollOffset: CGFloat
    @State private var dragOffset: CGFloat
    
    var items: Int
    var itemWidth: CGFloat
    var itemSpacing: CGFloat
    
    init(items: Int, itemWidth: CGFloat, itemSpacing: CGFloat) {
        self.items = items
        self.itemWidth = itemWidth
        self.itemSpacing = itemSpacing
        
        // Calculate Total Content Width
        let contentWidth: CGFloat = CGFloat(items) * itemWidth + CGFloat(items - 1) * itemSpacing
        let screenWidth = UIScreen.main.bounds.width
        
        // Set Initial Offset to first Item
        let initialOffset = (contentWidth/2.0) - (screenWidth/2.0) + ((screenWidth - itemWidth) / 2.0)
        
        self._scrollOffset = State(initialValue: initialOffset)
        self._dragOffset = State(initialValue: 0)
        
    }
    
    func body(content: Content) -> some View {
        content
            .offset(x: scrollOffset + dragOffset, y: 0)
            .gesture(DragGesture()
                .onChanged({ event in
                    dragOffset = event.translation.width
                })
                    .onEnded({ event in
                        // Scroll to where user dragged
                        scrollOffset += event.translation.width
                        dragOffset = 0
                        
                        // Now calculate which item to snap to
                        let contentWidth: CGFloat = CGFloat(items) * itemWidth + CGFloat(items - 1) * itemSpacing
                        let screenWidth = UIScreen.main.bounds.width
                        
                        // Center position of current offset
                        let center = scrollOffset + (screenWidth / 2.0) + (contentWidth / 2.0)
                        
                        // Calculate which item we are closest to using the defined size
                        var index = (center - (screenWidth / 2.0)) / (itemWidth + itemSpacing)
                        
                        // Should we stay at current index or are we closer to the next item...
                        if index.remainder(dividingBy: 1) > 0.5 {
                            index += 1
                        } else {
                            index = CGFloat(Int(index))
                        }
                        
                        // Protect from scrolling out of bounds
                        index = min(index, CGFloat(items) - 1)
                        index = max(index, 0)
                        
                        // Set final offset (snapping to item)
                        let newOffset = index * itemWidth + (index - 1) * itemSpacing - (contentWidth / 2.0) + (screenWidth / 2.0) - ((screenWidth - itemWidth) / 2.0) + itemSpacing
                        
                        // Animate snapping
                        withAnimation {
                            scrollOffset = newOffset
                        }
                        
                        
                    })
                     
            )
    }
}
/////
///
///
///
///
///
///


struct CustomizationOptionImage: View {
    let id: UUID = UUID()
    
    
    let overlay: AnyView?
    let backgoundImage: String?
    let width: CGFloat?
    let height: CGFloat?
    
    
    
    var body: some View {
        if overlay != nil {
            ZStack{
                IImage(backgoundImage: backgoundImage!, width: width, height: height)
            }
        }else {
            IImage(backgoundImage: backgoundImage!, width: width, height: height)
        }
        
    }
    
    struct IImage: View {
        let backgoundImage: String?
        let width: CGFloat?
        let height: CGFloat?
        
        var body: some View {
            Image(backgoundImage!)
                .resizable()
                .scaledToFill()
                .frame(width: width, height: height, alignment: .center)
                .clipped()
            
            
        }
    }
    
}

struct CustomizationOptionSolidColor: View {
    var color: Color?
    
    
    var body: some View {
        color
    }
}

struct CustomizationOptionSubtitlePosition: View {
    
    let id: UUID = UUID()
    var position: SubtitlePosition?
    
    
    
    
    
    
    var body: some View {
        GeometryReader { innerGeometry in
            ZStack{
                Color.white
                switch position! {
                case .Top:
                    VStack{
                        Spacer().frame(height: innerGeometry.size.height * 0.1)
                        SubtitlePositionIcon().frame(width: innerGeometry.size.width * 0.7)
                        Spacer()
                    }
                case .Middle:
                    VStack{
                        Spacer()
                        SubtitlePositionIcon().frame(width: innerGeometry.size.width * 0.7)
                        Spacer()
                    }
                case .Bottom:
                    VStack{
                        Spacer()
                        SubtitlePositionIcon().frame(width: innerGeometry.size.width * 0.7)
                        Spacer().frame(height: innerGeometry.size.height * 0.1)
                        
                    }
                }
                
                
            }
            
            
            
        }
    }
    
    
}



struct CustomizationOptionText: View {
    let id: UUID = UUID()
    var body: some View {
        ZStack{
            Color.white
            Text("Blah")
        }
        
    }
    
    
}


struct CustomizationOptionCruz: View {
    let id: UUID = UUID()
    
    
    var body: some View {
        GeometryReader{ geometry in
            ZStack{
                Color.white
                Cruz().frame(width: geometry.size.width * 0.5, height: geometry.size.width * 0.5, alignment: .center)
            }
            
        }
    }
}




struct CustomizationOptionColorPicker: View {
    let id: UUID = UUID()
    var body: some View {
        ZStack{
            Rectangle()
                .fill(
                    AngularGradient(gradient: Gradient(colors: [.yellow, .orange, .red, .pink, .purple, .blue, .cyan, .green, .yellow]), center: .center, startAngle: .zero, endAngle: .degrees(360))
                )
        }
    }
}


struct Cruz: View {
    var body: some View {
        ZStack{
            VerticalLine()
                .stroke(.gray, style: StrokeStyle(lineWidth: 5, lineCap: .round, lineJoin: .round))
            HorizontalLine()
                .stroke(.gray, style: StrokeStyle(lineWidth: 5, lineCap: .round, lineJoin: .round))
        }
    }
}

struct VerticalLine: Shape {
    func path(in rect: CGRect) -> Path {
        var path = Path()
        
        path.move(to: CGPoint(x: rect.midX, y: rect.maxY))
        path.addLine(to: CGPoint(x: rect.midX, y: rect.minY))
        
        return path
    }
}


struct HorizontalLine: Shape {
    func path(in rect: CGRect) -> Path {
        var path = Path()
        
        path.move(to: CGPoint(x: rect.minX, y: rect.midY))
        path.addLine(to: CGPoint(x: rect.maxX, y: rect.midY))
        
        return path
    }
}

struct SubtitlePositionIcon: View {
    var body: some View {
        GeometryReader{ innerGeometry in
            VStack{
                HorizontalLine()
                    .stroke(.gray, style: StrokeStyle(lineWidth: 5, lineCap: .round, lineJoin: .round))
                HorizontalLine()
                    .stroke(.gray, style: StrokeStyle(lineWidth: 5, lineCap: .round, lineJoin: .round))
                HorizontalLine()
                    .stroke(.gray, style: StrokeStyle(lineWidth: 5, lineCap: .round, lineJoin: .round))
                    .frame(width: innerGeometry.size.width * 0.5)
            }
        }.frame(height: 30)
    }
}



enum SubtitlePosition {
    case Top
    case Middle
    case Bottom
}


func playSound(soundName: String) {
    guard let path = Bundle.main.path(forResource: soundName, ofType:"mp3") else {
        return }
    let url = URL(fileURLWithPath: path)
    
    do {
        player = try AVAudioPlayer(contentsOf: url)
        player?.play()
        
    } catch let error {
        print(error.localizedDescription)
    }
}

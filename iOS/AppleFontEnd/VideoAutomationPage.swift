//
//  VideoAutomationPage.swift
//  AppleFontEnd
//
//  Created by Maemi on 2022-08-02.
//

import SwiftUI
import AVKit
import SnapToScroll

struct VideoAutomationPage: View {
    
   
    @State private var videosPerMonthSliderValue:Float = 32
    @State private var videosPerMonth:Int = 32
    @State var customPlanPrice: Float = 0.00
    
    @State var chosenRenderEngine: RenderEngine = RenderEngine.SentenceASAVAO

    
    var body: some View {
        GeometryReader{ geometry in
            
            
            NavigationView{
                VStack{
                    HStack{
                        Text("Video Creation Engine")
                        Spacer()
                        Text("Choose one or more")
                    }
                    
                    RenderEngineSlider(chosenRenderEngine: $chosenRenderEngine)
                        .frame(height: geometry.size.height * 0.1398104265)
                        .mask (
                            Color.white
                                .frame(height: geometry.size.height)
                        )
                        .shadow(color: .gray, radius: 6, x: 0, y: 3)
                    
                    Spacer().frame(height: geometry.size.height * 0.05)
                    HStack{
                        Text("Videos per month")
                        Spacer()
                        Text(String(videosPerMonth))
                    }
                    Slider(value: Binding(get: {
                        self.videosPerMonthSliderValue
                    }, set: { (newVal) in
                        self.videosPerMonthSliderValue = newVal
                        self.videosPerMonth = Int(floor(self.videosPerMonthSliderValue))
                    }), in: 0...100)
                    /////
                    Spacer().frame(height: geometry.size.height * 0.05)
                    ///
                    ZStack(alignment: .topLeading){
                        HStack{
                            VideoPlayer(player: AVPlayer(url:  URL(string: "https://bit.ly/swswift")!))
                                .frame(width:(geometry.size.height * 0.3341232227)/1.7777777778, height: geometry.size.height * 0.3341232227)
                                .cornerRadius(10)
                                .shadow(color: .gray, radius: 6, x: 0, y: 3)
                            Spacer()
                        }
                        
                        HStack{
                            Spacer()
                            VStack{
                                Spacer().frame(height: geometry.size.height * 0.0175)
                                
                            }
                        }
                        
                    }.frame(alignment: .topLeading)
                    ///
                    //////
                    Spacer()
                    ///
                    ///
                    HStack(){
                        Spacer()
                        HStack(spacing: 0){
                            Text("$\(String(calculatePricePlan())) USD").padding(10)
                            
                            
                            Button(action: { }, label: {
                                Text("Purchase Plan")
                                    .padding()
                                    .foregroundColor(.white)
                                    .background(.teal)
                            })
                        }
                        .background(.white)
                        .cornerRadius(15)
                        .shadow(color: .gray, radius: 3, x: 0, y: 3)
                        
                    }
                }
                .padding()
                .navigationTitle("Automation")
            }.navigationViewStyle(StackNavigationViewStyle())
            
        }
    }
    
    func calculatePricePlan() -> Float {
        return (round((Float(self.videosPerMonth) * 0.24) * 100) / 100.0)
    }
}

struct VideoAutomationPage_Previews: PreviewProvider {
    static var previews: some View {
        VideoAutomationPage()
    }
}

extension View {
    func border(_ color: Color, width: CGFloat, cornerRadius: CGFloat) -> some View {
        overlay(RoundedRectangle(cornerRadius: cornerRadius).stroke(color, lineWidth: width))
    }
}

//
//  RenderEngineSlider.swift
//  AppleFontEnd
//
//  Created by Maemi on 2022-08-17.
//

import SwiftUI

struct RenderEngineSlider: View {
    @Binding var chosenRenderEngine: RenderEngine
    var body: some View {
        GeometryReader{ geometry in
            HStack(spacing: 0){
                ForEach(
                    [
                        RenderEngineBanner(backgroundImage: "cool_bg", renderEngine: chosenRenderEngine.rawValue, fontColor: Color.white),
                        RenderEngineBanner(backgroundImage: "cool_bg", renderEngine: chosenRenderEngine.rawValue, fontColor: Color.white),
                        RenderEngineBanner(backgroundImage: "cool_bg", renderEngine: chosenRenderEngine.rawValue, fontColor: Color.white),
                        RenderEngineBanner(backgroundImage: "cool_bg", renderEngine: chosenRenderEngine.rawValue, fontColor: Color.white),
                        RenderEngineBanner(backgroundImage: "cool_bg", renderEngine: chosenRenderEngine.rawValue, fontColor: Color.white)
                           ]
                ){ renderEngineSlide in
                    renderEngineSlide
                        .frame(width: geometry.size.width, height: geometry.size.height)
                        .cornerRadius(10)
                }
        }
            .frame(width: geometry.size.width, height: geometry.size.height)
            .modifier(RenderEngineScrollingHStackModifier(items: 5, itemWidth: geometry.size.width, itemSpacing: 0, renderEngine: $chosenRenderEngine))

    }
    }
}

//struct RenderEngineSlider_Previews: PreviewProvider {
//    static var previews: some View {
//        RenderEngineSlider(chosenRenderEngine: RenderEngine.ASAVAO)
//    }
//}

struct RenderEngineBanner: View, Identifiable {
    var id: UUID = UUID()
    
    let backgroundImage: String
    let renderEngine: String
    let fontColor: Color
    var body: some View {
        ZStack(){
            Image(backgroundImage)
                .resizable()
                .scaledToFill()
            Text(renderEngine)
                .foregroundColor(fontColor)
                
        }
    }
}




struct RenderEngineScrollingHStackModifier: ViewModifier {
    
    
    
    @State private var scrollOffset: CGFloat
    @State private var dragOffset: CGFloat
    
    var items: Int
    var itemWidth: CGFloat
    var itemSpacing: CGFloat
    @Binding var chosenRenderEngine: RenderEngine
    
    init(items: Int, itemWidth: CGFloat, itemSpacing: CGFloat, renderEngine: Binding<RenderEngine>) {
        self.items = items
        self.itemWidth = itemWidth
        self.itemSpacing = itemSpacing
        self._chosenRenderEngine = renderEngine
        
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
                        
                        switch index {
                        case 4:
                            self.chosenRenderEngine = RenderEngine.SentenceASAVAO
                        case 3:
                            self.chosenRenderEngine = RenderEngine.SentenceAVAO
                        case 2:
                            self.chosenRenderEngine = RenderEngine.SentenceAO
                        case 1:
                            self.chosenRenderEngine = RenderEngine.SentenceFULL
                        case 0:
                            self.chosenRenderEngine = RenderEngine.SentenceBabyAI
                        default:
                            self.chosenRenderEngine = RenderEngine.SentenceASAVAO
                        }
                    })
                     
            )
    }
}

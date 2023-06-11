//
//  ContentView.swift
//  AppleFontEnd
//
//  Created by Maemi on 2022-08-01.
//

import SwiftUI
import AVKit

struct ContentView: View {
    
    @State var textFieldContent: String = ""
    
    var body: some View {
        TabView{
            StoredGeneratedVideosPage().tabItem({
                Image(systemName: "photo.fill.on.rectangle.fill")
            })
            
            
            ScriptToVideoPage().tabItem({
                Image(systemName: "folder.fill")
            })
           VideoAutomationPage().tabItem({
                Image(systemName: "folder.fill")
            })
        }
    }
    
    
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}


struct CornerRadiusStyle: ViewModifier {
    var radius: CGFloat
    var corners: UIRectCorner
    
    struct CornerRadiusShape: Shape {
        
        var radius = CGFloat.infinity
        var corners = UIRectCorner.allCorners
        
        func path(in rect: CGRect) -> Path {
            let path = UIBezierPath(roundedRect: rect, byRoundingCorners: corners, cornerRadii: CGSize(width: radius, height: radius))
            return Path(path.cgPath)
        }
    }
    
    func body(content: Content) -> some View {
        content
            .clipShape(CornerRadiusShape(radius: radius, corners: corners))
    }
}

extension View {
    func cornerRadius(_ radius: CGFloat, corners: UIRectCorner) -> some View {
        ModifiedContent(content: self, modifier: CornerRadiusStyle(radius: radius, corners: corners))
    }
}

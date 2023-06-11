//
//  StoredGeneratedVideosPage.swift
//  AppleFontEnd
//
//  Created by Maemi on 2022-08-02.
//

import SwiftUI
import AVKit


//
struct StoredGeneratedVideosPage: View {
    var storedVideosPaths: [URL] = FileManager.default.listFiles()!
    let columns = [GridItem(.flexible()), GridItem(.flexible())]
    var body: some View {
        NavigationView{
            ScrollView {
                LazyVGrid(columns: columns) {
                    ForEach(storedVideosPaths, id: \.self){videoPath in
                        VideoPlayer(player: AVPlayer(url:  videoPath))
                            .frame(width: 720/4, height: 1080/4)
                            .cornerRadius(5)
                    }
                }
            }.navigationTitle("My Videos")
        }.navigationViewStyle(StackNavigationViewStyle())
        
    }
}

extension FileManager {
    func listFiles() -> [URL]? {
        let base = FileManager.default.urls(for: .documentDirectory, in: .allDomainsMask).first!
        return try? FileManager.default.contentsOfDirectory(at: base, includingPropertiesForKeys: nil)
    }
    //    func listFilesPaths() -> [String?] {
    //        var paths: [String?] = []
    //        for url in self.listFiles()!{
    //            paths.append(url.path)
    //        }
    //        return paths
    //    }
}







class FolderMonitor {
    // MARK: Properties
    
    /// A file descriptor for the monitored directory.
    private var monitoredFolderFileDescriptor: CInt = -1
    /// A dispatch queue used for sending file changes in the directory.
    private let folderMonitorQueue = DispatchQueue(label: "FolderMonitorQueue", attributes: .concurrent)
    /// A dispatch source to monitor a file descriptor created from the directory.
    private var folderMonitorSource: DispatchSourceFileSystemObject?
    /// URL for the directory being monitored.
    let url: Foundation.URL
    
    var folderDidChange: (() -> Void)?
    // MARK: Initializers
    init(url: Foundation.URL) {
        self.url = url
    }
    // MARK: Monitoring
    /// Listen for changes to the directory (if we are not already).
    func startMonitoring() {
        guard folderMonitorSource == nil && monitoredFolderFileDescriptor == -1 else {
            return
            
        }
        // Open the directory referenced by URL for monitoring only.
        monitoredFolderFileDescriptor = open(url.path, O_EVTONLY)
        // Define a dispatch source monitoring the directory for additions, deletions, and renamings.
        folderMonitorSource = DispatchSource.makeFileSystemObjectSource(fileDescriptor: monitoredFolderFileDescriptor, eventMask: .write, queue: folderMonitorQueue)
        // Define the block to call when a file change is detected.
        folderMonitorSource?.setEventHandler { [weak self] in
            self?.folderDidChange?()
        }
        // Define a cancel handler to ensure the directory is closed when the source is cancelled.
        folderMonitorSource?.setCancelHandler { [weak self] in
            guard let strongSelf = self else { return }
            close(strongSelf.monitoredFolderFileDescriptor)
            strongSelf.monitoredFolderFileDescriptor = -1
            strongSelf.folderMonitorSource = nil
        }
        // Start monitoring the directory via the source.
        folderMonitorSource?.resume()
    }
    /// Stop listening for changes to the directory, if the source has been created.
    func stopMonitoring() {
        folderMonitorSource?.cancel()
    }
}

















//        GeometryReader{ geometry in
//
//
//            NavigationView{
//                ScrollView{
//                    LazyVGrid(columns: [GridItem(.adaptive(minimum: 150))], spacing: geometry.size.width * 0.055){
//                        ForEach(1...27, id: \.self){ value in
//                            //                                    AsyncImage(url: URL(string: "https://i.picsum.photos/id/838/200/300.jpg?hmac=yns6FqTn8FmJq3qluHDmnjn6X4x-rC4lGjZVUIMknuI")!){ image in
//                            //                                        image
//                            //                                            .resizable()
//                            //                                    } placeholder: {
//                            //                                        ProgressView()
//                            //                                    }
//                            VStack{
//                                Spacer()
//                                HStack(alignment: .bottom){
//                                    Image(systemName: "square.and.arrow.up").font(.system(size: 26))
//                                        .frame(width: 40, height: 52.5, alignment: .trailing)
//                                    Image(systemName: "square.and.arrow.down").font(.system(size: 17))
//                                        .frame(width: 15, height: 42.5)
//                                    Spacer()
//                                    Spacer()
//                                        .frame(width: 26, height: 26)
//                                        .background(.white)
//                                        .cornerRadius(10, corners: [.topLeft])
//
//
//                                        .overlay(
//                                            Rectangle()
//                                                .stroke(Color.purple, lineWidth: 10)
//                                                .cornerRadius(10, corners: [.topLeft])
//                                        )
//                                    //                                            RoundedRectangle(cornerRadius: 8)
//                                    //                                                .stroke(Color.purple, lineWidth: 5)
//                                    //                                                .frame(width: 26, height: 26)
//
//
//                                }
//                                .frame(height: 60, alignment: .bottom)
//                                .padding(0)
//                            }.frame(
//                                width: geometry.size.width * 0.4210526316,
//                                height: geometry.size.height * 0.336492891
//                            )
//                            .cornerRadius(10)
//                            .overlay(
//                                RoundedRectangle(cornerRadius:10)
//                                    .stroke(Color.purple, lineWidth: 5)
//                            )
//                        }
//                    }.padding()
//                }.navigationTitle("My Videos")
//
//            }.navigationViewStyle(StackNavigationViewStyle())
//        }
//    }
//
//
//
//}


struct StoredGeneratedVideosPage_Previews: PreviewProvider {
    static var previews: some View {
        StoredGeneratedVideosPage()
    }
}





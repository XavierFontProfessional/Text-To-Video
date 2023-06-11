# Text-To-Video

Text-To-Video is an innovative project that transforms text input into a visually engaging video. This process utilizes Rust's Actix Web, one of the world's fastest HTTP server frameworks, to handle requests and generate video instructions with optimal performance and minimal server load. The main feature of the project is the offloading of the rendering process to the client-side, thus enabling high server availability.

The project is compatible with iOS and Android platforms as of June 2023.

## How it Works

1. **Request Processing**: The server accepts a JSON request from the client containing various parameters including the script, renderEngine, voice, filter, subtitlesFont, subtitlesColor, subtitlesBackground, and subtitlesPosition.

2. **Script and Voice Processing**: The server extracts the script text and voice name from the request, proceeding with processing if valid.

3. **Script Analysis**: The script is broken down into its constituent parts (words, punctuations, characters), and sentences are identified and classified into parts such as subject, verb, object, and their descriptors.

4. **Clip Creation**: Clips are created from the script based on the specified render engine. For each clip, an image URL is fetched and assigned.

5. **Video Assembly**: A new Video object is created using the generated clips. The video filter, subtitles font, color, background color, and position are set based on the request parameters.

6. **Subtitle and Voice Generation**: The server generates the subtitles file and extracts the voice data in base-8 format from each clip.

7. **Instruction Generation**: The server generates instructions for the client to render the video, along with the subtitles and voice data. This data package is sent back to the client as a JSON response.

8. **Video Rendering**: The client uses the instructions to render the video in the browser, which comprises of the visual clips, custom audio, and subtitles as per the specified parameters.

## Getting Started

To get started with the project, follow these steps:

1. Clone the repository to your local machine.
2. Navigate to the project's root directory.
3. Run your desired script file.

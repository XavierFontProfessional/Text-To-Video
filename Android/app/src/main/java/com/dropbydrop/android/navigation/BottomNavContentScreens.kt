package com.dropbydrop.android.navigation

import android.annotation.SuppressLint
import android.content.ContentValues.TAG
import android.content.Context
import android.net.Uri
import android.util.Log
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.colorResource
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.navigation.NavController
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.arthenica.ffmpegkit.FFmpegKit
import com.arthenica.ffmpegkit.ReturnCode
import com.dropbydrop.android.R
import com.dropbydrop.android.http.VideoInstructionsAPIService
import com.dropbydrop.android.http.VideoInstructionsRequest
import com.dropbydrop.android.ui.theme.AndroidTheme
import com.google.android.exoplayer2.ExoPlayer
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.ui.StyledPlayerView
import dev.chrisbanes.snapper.ExperimentalSnapperApi
import dev.chrisbanes.snapper.rememberSnapperFlingBehavior
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.File
import java.io.IOException
import java.io.InputStream


val myCLient: VideoInstructionsAPIService = VideoInstructionsAPIService.create()


@Composable
fun StVScreen() {
    // Active and Inactive Video Options
    val hasFilter: MutableState<Boolean> = remember { mutableStateOf(true) }
    val hasSubtitles: MutableState<Boolean> = remember { mutableStateOf(true) }
    val hasSubtitlesBackground: MutableState<Boolean> = remember { mutableStateOf(true) }
    val hasMusic: MutableState<Boolean> = remember { mutableStateOf(true) }
    val hasVoice: MutableState<Boolean> = remember { mutableStateOf(true) }

    // Chosen Video Options
    var chosenFilterId: MutableState<Int> = remember { mutableStateOf(1) }
    var chosenFontStyleId: MutableState<Int> = remember { mutableStateOf(1) }
    var chosenFontColorId: MutableState<Int> = remember { mutableStateOf(1) }
    var chosenSubtitlesBackgroundId: MutableState<Int> = remember { mutableStateOf(1) }
    var chosenSubtitlesPositionId: MutableState<Int> = remember { mutableStateOf(1) }
    var chosenMusicId: MutableState<Int> = remember { mutableStateOf(1) }
    var chosenVoiceId: MutableState<Int> = remember { mutableStateOf(1) }

    val videoOptions: List<VideoOptionData> = listOf<VideoOptionData>(
        VideoOptionData(
            id = 1, hasFilter, title = "Filters", activeOptionId = chosenFilterId, options =  listOf<OptionOfVideoOptionData>(
                OptionOfVideoOptionData(
                    1, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    2, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    3,"vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                )
            )
        ),
        VideoOptionData(
            id = 2, isActive = hasSubtitles, title = "Font Style", activeOptionId = chosenFontStyleId, options = listOf<OptionOfVideoOptionData>(
                OptionOfVideoOptionData(
                    1, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    2, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    3, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                )
            )
        ),
        VideoOptionData(
            id = 3, isActive = hasSubtitles, title = "Font Color", activeOptionId = chosenFontColorId, options = listOf<OptionOfVideoOptionData>(
                OptionOfVideoOptionData(
                    1, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    2, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    3, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    4, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    5, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                )
            )
        ),
        VideoOptionData(
            id = 4, isActive = hasSubtitlesBackground, title = "Subtitles Background", activeOptionId = chosenSubtitlesBackgroundId, options = listOf<OptionOfVideoOptionData>(
                OptionOfVideoOptionData(
                    1, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    2, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    3, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    4, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    5, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                )
            )
        ),
        VideoOptionData(
            id = 5, isActive = hasSubtitles, title = "Subtitles Position", activeOptionId = chosenSubtitlesPositionId, options = listOf<OptionOfVideoOptionData>(
                OptionOfVideoOptionData(
                    1, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    2, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    3, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    4, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    5, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                )
            )
        ),
        VideoOptionData(
            id = 6, isActive = hasMusic, title = "Music", activeOptionId = chosenMusicId, options = listOf<OptionOfVideoOptionData>(
                OptionOfVideoOptionData(
                    1, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    2, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    3, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    4, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    5, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                )
            )
        ),
        VideoOptionData(
            id = 7, isActive = hasVoice, title = "Voice", activeOptionId = chosenVoiceId, options = listOf<OptionOfVideoOptionData>(
                OptionOfVideoOptionData(
                    1, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    2, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    3, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    4, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                ),
                OptionOfVideoOptionData(
                    5, "vintage", painterResource(
                        id = R.drawable.brown_door
                    )
                )
            )
        )
    )
    val renderOptions: List<RenderOptionData> = listOf<RenderOptionData>(
        RenderOptionData(
            "Larry", painterResource(
                id = R.drawable.brown_door
            )
        ),
        RenderOptionData(
            "Joooo", painterResource(
                id = R.drawable.brown_door
            )
        ),
        RenderOptionData(
            "WakananataAVAO", painterResource(
                id = R.drawable.brown_door
            )
        ),
        RenderOptionData(
            "SimonSer", painterResource(
                id = R.drawable.brown_door
            )
        )
    )

    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(30.dp)
    ) {
        VideoOptions(videoOptions)
        /// SCRIPT FIELD

        var inputtedScript by remember { mutableStateOf("Type here...") }

        Surface(
            color = Color.White,
            modifier = Modifier
                .height(231.dp)
                .width(337.dp),
            shape = RoundedCornerShape(10.dp),
            elevation = 6.dp
        ) {
            BasicTextField(
                value = inputtedScript,
                onValueChange = { inputtedScript = it },
                modifier = Modifier.padding(10.dp)
            )
        }

        RenderOptions(renderOptions)
        GenerateButton(inputtedScript)
    }
}

@Composable
fun RenderOption(renderOptionData: RenderOptionData) {
    Surface(
        color = Color.White,
        modifier = Modifier
            .height(62.dp)
            .width(337.dp),
        shape = RoundedCornerShape(10.dp),
    ) {
        Text(renderOptionData.name)
    }
}


@OptIn(ExperimentalSnapperApi::class)
@Composable
fun RenderOptions(renderOptions: List<RenderOptionData>) {
    val lazyListState: LazyListState = rememberLazyListState()
    LazyRow(
        horizontalArrangement = Arrangement.spacedBy(10.dp),
        state = lazyListState,
        flingBehavior = rememberSnapperFlingBehavior(lazyListState),
        modifier = Modifier
            .padding(start = 27.dp, end = 26.5.dp)
            .shadow(4.dp)
    ) {
        items(renderOptions) { renderOption ->
            RenderOption(renderOption)
        }
    }
}

data class RenderOptionData(
    var name: String,
    var image: Painter
)

@Composable
fun VideoOption(optionData: VideoOptionData) {
    Surface(
        color = Color.White,
        modifier = Modifier
            .height(191.dp)
            .width(263.dp),
        shape = RoundedCornerShape(10.dp),
        elevation = 6.dp
    ) {
        Column() {
            Row(horizontalArrangement = Arrangement.SpaceBetween){
                Text(text = optionData.title)
                Switch(checked = optionData.isActive.value, onCheckedChange = {optionData.isActive.value = it})
            }
            Row(
                verticalAlignment = Alignment.Bottom,
                horizontalArrangement = Arrangement.SpaceEvenly,
                modifier = Modifier
                    .fillMaxSize()
                    .padding(bottom = 33.dp)
            ) {
                if (optionData.options.size == 3) {

                    optionData.options.forEach { option ->
                        VideoOptionOption(
                            id = option.id,
                            parentData = optionData,
                            image = option.image
                        )
                    }

                } else if (optionData.options.size == 5) {

                    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                        VideoOptionOption(
                            id = optionData.options[0].id,
                            parentData = optionData,
                            image = optionData.options[0].image
                        )
                        VideoOptionOption(
                            id = optionData.options[1].id,
                            parentData = optionData,
                            image = optionData.options[1].image
                        )
                    }
                    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                        VideoOptionOption(
                            id = optionData.options[2].id,
                            parentData = optionData,
                            image = optionData.options[2].image
                        )
                        VideoOptionOption(
                            id = optionData.options[3].id,
                            parentData = optionData,
                            image = optionData.options[3].image
                        )
                    }
                    VideoOptionOption(
                        id = optionData.options[4].id,
                        parentData = optionData,
                        image = optionData.options[4].image
                    )
                }
            }
        }
    }
}


@Composable
fun VideoOptionOption(id: Int, parentData: VideoOptionData, image: Painter) {
    var borderColorForActiveAndInactive: MutableState<Color> = remember {
        mutableStateOf(Color.Transparent)
    }
    if (parentData.activeOptionId.value == id) {
        borderColorForActiveAndInactive.value = Color.Red
    } else {
        borderColorForActiveAndInactive.value = Color.Transparent
    }
    if (parentData.options.size == 3 || (parentData.options.size == 5 && id == 5)) { // It is tall item
        Surface(
            color = Color.White,
            modifier = Modifier
                .height(121.dp)
                .width(76.dp)
                .clickable {
                    parentData.activeOptionId.value = id
                    println(parentData.activeOptionId.value)
                }
                .border(BorderStroke(2.dp, borderColorForActiveAndInactive.value)),
            shape = RoundedCornerShape(10.dp),
            elevation = 6.dp
        ) {
            Image(painter = image, contentDescription = "")
        }
    } else if (parentData.options.size == 5 && id < 5) { // It is short item
        Surface(
            color = Color.White,
            modifier = Modifier
                .height(56.dp)
                .width(76.dp)
                .clickable {
                    parentData.activeOptionId.value = id
                    println(parentData.activeOptionId.value)
                }
                .border(BorderStroke(2.dp, borderColorForActiveAndInactive.value)),
            shape = RoundedCornerShape(10.dp),
            elevation = 18.dp
        ) {
            Image(
                modifier = Modifier.fillMaxSize(),
                painter = image,
                contentDescription = "",
                contentScale = ContentScale.Crop
            )
        }
    }
}

data class VideoOptionData(
    var id: Int,
    var isActive: MutableState<Boolean>,
    var activeOptionId: MutableState<Int>,
    var title: String,
    var options: List<OptionOfVideoOptionData>
)

data class OptionOfVideoOptionData(
    var id: Int,
    var name: String,
    var image: Painter
)

@OptIn(ExperimentalSnapperApi::class)
@Composable
fun VideoOptions(videoOptions: List<VideoOptionData>) {
    val lazyListState: LazyListState = rememberLazyListState()
    LazyRow(
        contentPadding = PaddingValues(start = 70.dp, bottom = 8.dp, end = 8.dp, top = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(10.dp),
        state = lazyListState,
        flingBehavior = rememberSnapperFlingBehavior(lazyListState),
    ) {
        items(videoOptions) { videoOption ->
            VideoOption(videoOption)
        }
    }
}

@Composable
fun ScriptTextField() {


}



@Composable
fun GenerateButton(inputtedScript: String) {
    val coroutineScope = rememberCoroutineScope()
    val context = LocalContext.current
    Button(
        onClick = {

            val request: VideoInstructionsRequest = VideoInstructionsRequest(null, "SentenceAVAO", inputtedScript, "FFFFFF", "000000", "Arial", "Bottom", "en-US-SaraNeural")

            coroutineScope.launch {

              val response =
                  withContext(Dispatchers.Default) {
                      myCLient.createProducts(request)
                  }

                val subtitlesPath =
                    withContext(Dispatchers.Default) {
                        writeSubtitlesFile(
                            context,
                            response?.subtitles
                        )
                    }

                val voicesPaths: List<String> =
                    withContext(Dispatchers.Default) {
                        writeVoicesFile(context, response?.voices)
                    }
                renderVideoFromInstructions(response!!.instructions, subtitlesPath, voicesPaths, context)
            }



            /////////



        },
        modifier = Modifier
            .width(133.dp)
            .height(46.dp),
    ) {
        Text(text = "Press")
    }
}




@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    AndroidTheme {
        ScriptTextField()
    }
}

@Composable
fun GalleryScreen() {
    val context = LocalContext.current
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(colorResource(id = R.color.teal_700))
            .wrapContentSize(Alignment.Center)
    ) {
        ReadDataFile()
    }
}

@Composable
fun AutomationScreen() {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(colorResource(id = R.color.teal_700))
            .wrapContentSize(Alignment.Center)
    ) {
        VideoView()
    }
}

@Composable
fun NavigationGraph(navController: NavHostController) {
    NavHost(navController, startDestination = BottomNavItem.Gallery.screen_route) {
        composable(BottomNavItem.Gallery.screen_route) {
            GalleryScreen()
        }
        composable(BottomNavItem.StV.screen_route) {
            StVScreen()
        }
        composable(BottomNavItem.Automation.screen_route) {
            AutomationScreen()
        }
    }
}

@Composable
fun BottomNavigation(navController: NavController) {
    val items = listOf(
        BottomNavItem.Gallery,
        BottomNavItem.StV,
        BottomNavItem.Automation
    )
    BottomNavigation(
        backgroundColor = colorResource(id = R.color.teal_200),
        contentColor = Color.Black
    ) {
        val navBackStackEntry by navController.currentBackStackEntryAsState()
        val currentRoute = navBackStackEntry?.destination?.route
        items.forEach { item ->
            BottomNavigationItem(
                icon = { Icon(painterResource(id = item.icon), contentDescription = item.title) },
                label = {
                    Text(
                        text = item.title,
                        fontSize = 9.sp
                    )
                },
                selectedContentColor = Color.Black,
                unselectedContentColor = Color.Black.copy(0.4f),
                alwaysShowLabel = true,
                selected = currentRoute == item.screen_route,
                onClick = {
                    navController.navigate(item.screen_route) {

                        navController.graph.startDestinationRoute?.let { screen_route ->
                            popUpTo(screen_route) {
                                saveState = true
                            }
                        }
                        launchSingleTop = true
                        restoreState = true
                    }
                }
            )
        }
    }
}

@SuppressLint("UnusedMaterialScaffoldPaddingParameter")
@Composable
fun MainScreenView() {
    val navController = rememberNavController()
    Scaffold(
        bottomBar = { BottomNavigation(navController = navController) }
    ) {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colors.background

        ) {
            NavigationGraph(navController = navController)
        }
    }
}

@Composable
fun VideoView() {
    val context = LocalContext.current
    val fileDirectory = LocalContext.current.filesDir
    val file = File(fileDirectory, "new2.mp4");

    val exoPlayer = ExoPlayer.Builder(LocalContext.current)
        .build()
        .also { exoPlayer ->
            val mediaItem = MediaItem.Builder()
                .setUri(Uri.parse(file.path))
                .build()
            exoPlayer.setMediaItem(mediaItem)
            exoPlayer.prepare()
        }

    DisposableEffect(
        AndroidView(factory = {
            StyledPlayerView(context).apply {
                player = exoPlayer
            }
        }, modifier = Modifier.height(500.dp))
    ) {
        onDispose { exoPlayer.release() }
    }
}

suspend fun renderVideoFromInstructions (
    instructions: String,
    subtitlesPath: String,
    voicesPaths: List<String>,
    context: Context
) {
    var command = instructions.replace("{subtitles_path}", subtitlesPath)
    command = command.replace("{fonts_directory}", "res/font/arial.ttf")
//    val session = FFmpegKit.execute("-loop 1 -t 0.65999997 -i https://images.unsplash.com/photo-1527186504227-0a47da29a0d0?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwzNDIzMjd8MHwxfHNlYXJjaHwxMHx8anVtcHxlbnwwfHx8fDE2NjU0NTY4NDg&ixlib=rb-1.2.1&q=80&w=1080 -loop 1 -t 0.53999996 -i https://images.unsplash.com/photo-1486218119243-13883505764c?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=MnwzNDIzMjd8MHwxfHNlYXJjaHwxMHx8cnVufGVufDB8fHx8MTY2NTQ1Njg0OA&ixlib=rb-1.2.1&q=80&w=1080 -filter_complex [0:v]scale=1080:1920,setsar=1[0];[1:v]scale=1080:1920,setsar=1[1];[0][1]concat=n=2[concatenatedvideowithsubtitles] -map [concatenatedvideowithsubtitles]:v -y ${context.filesDir}/new.mp4")
   voicesPaths.forEachIndexed { i, voicePath ->
       command = command.replace("voice$i", voicePath)
   }

    Log.d(TAG, "HERE IS THE FINAL INSTRUCTION ---::::---->>>> $command")

    val session = FFmpegKit.execute("$command ${context.filesDir}/new2.mp4")

    if (ReturnCode.isSuccess(session.returnCode)) {
        println("GREAT! File saved at ${context.filesDir}/new2.mp4")
        // SUCCESS
    } else if (ReturnCode.isCancel(session.returnCode)) {

        // CANCEL
    } else {

        // FAILURE
        Log.d(
            TAG,
            String.format(
                "Command failed with state %s and rc %s.%s",
                session.state,
                session.returnCode,
                session.failStackTrace
            )
        )
    }
}

suspend fun writeSubtitlesFile (context: Context, subtitles: String?): String {
    val file = File(context.filesDir, "subtitles.srt")
    file.appendText(subtitles!!)
    return file.path
}

suspend fun writeVoicesFile (context: Context, rawVoicesBytesArray: List<List<Int>>?): List<String>  {

    var voicesPaths: MutableList<String> = mutableListOf()

    rawVoicesBytesArray!!.forEachIndexed {i,  voiceBinary ->
        val file = File(context.filesDir, "audio$i.mp3")

        val bArr: ByteArray = ByteArray(voiceBinary.size) { index -> voiceBinary[index].toByte() }
        file.writeBytes(bArr)

        voicesPaths.add(file.path)
    }


    return voicesPaths
}

@Composable
fun ReadDataFile() {
    println("Read Data File")
    Text("Read Data File")
    val context = LocalContext.current
    try {
        val inputStream: InputStream = context.assets.open("new2.mp4")
        val size: Int = inputStream.available()
        val buffer = ByteArray(size)
        inputStream.read(buffer)
        var string = String(buffer)
        println(string)
        //Text(string)      // ERROR: Try catch is not supported around composable function invocations
    } catch (e: IOException) {
        e.printStackTrace()
        println("Error")
    }
}
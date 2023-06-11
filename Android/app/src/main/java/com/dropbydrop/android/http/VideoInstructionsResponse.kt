package com.dropbydrop.android.http

import kotlinx.serialization.Serializable

@Serializable
data class VideoInstructionsResponse(
    val instructions: String,
    val subtitles: String?,
    val voices: List<List<Int>>?
)
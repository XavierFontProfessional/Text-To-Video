package com.dropbydrop.android.http

import kotlinx.serialization.Serializable

@Serializable
data class VideoInstructionsRequest(
    val filter: String?,
    val renderEngine: String,
    val script: String,
    val subtitlesBackground: String?,
    val subtitlesColor: String?,
    val subtitlesFont: String?,
    val subtitlesPosition: String?,
    val voice: String?
)
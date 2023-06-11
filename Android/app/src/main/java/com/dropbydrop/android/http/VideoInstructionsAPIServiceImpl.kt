package com.dropbydrop.android.http

import io.ktor.client.*
import io.ktor.client.features.*
import io.ktor.client.request.*

class VideoInstructionsAPIServiceImpl( private val client: HttpClient) : VideoInstructionsAPIService {
    override suspend fun getProducts(): VideoInstructionsResponse {
        TODO("Not yet implemented")
    }

    override suspend fun createProducts(productRequest: VideoInstructionsRequest): VideoInstructionsResponse? {
        return try {

            client.post<VideoInstructionsResponse> {
                url(ApiRoutes.PRODUCTS)
                body = productRequest
            }
        } catch (ex: RedirectResponseException) {
            // 3xx - responses
            println("Error: ${ex.response.status.description}")
            null
        } catch (ex: ClientRequestException) {
            // 4xx - responses
            println("Error: ${ex.response.status.description}")
            null
        } catch (ex: ServerResponseException) {
            // 5xx - response
            println("Error: ${ex.response.status.description}")
            null
        }
    }

}

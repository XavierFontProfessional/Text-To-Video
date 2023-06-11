package com.dropbydrop.android.navigation
import androidx.compose.material.Icon
import com.dropbydrop.android.R

sealed class BottomNavItem(var title:String, var icon:Int, var screen_route:String){

    object StV : BottomNavItem("StV", R.drawable.ic_launcher_foreground,"StV")
    object Automation: BottomNavItem("Automation", R.drawable.ic_launcher_foreground,"Automation")
    object Gallery: BottomNavItem("Gallery", R.drawable.ic_launcher_foreground,"Gallery")

}
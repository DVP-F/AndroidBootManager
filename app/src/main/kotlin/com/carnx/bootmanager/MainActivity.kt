package com.carnx.bootmanager

import android.annotation.SuppressLint
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.height
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.unit.dp
import androidx.compose.ui.tooling.preview.Preview

class MainActivity : ComponentActivity() {

    private val repo = BootRepository()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        //enableEdgeToEdge()

        setContent {
            BootManagerTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Main(
                        repo = repo,
                        modifier = Modifier.padding(innerPadding)
                    )
                }
            }
        }
    }
}

@SuppressLint("UnusedBoxWithConstraintsScope") // ignore 'dead' code
@Composable
fun ResponsiveLayoutBox(
    rowAlignment: Alignment.Vertical = Alignment.Top,
    rowArrangement: Arrangement.Horizontal = Arrangement.Start,
    colAlignment: Alignment.Horizontal = Alignment.Start,
    colArrangement: Arrangement.Vertical = Arrangement.Top,
    content: @Composable () -> Unit
) {
    // Rearrange in case of landscape view
    BoxWithConstraints (
        modifier = Modifier,
        contentAlignment = Alignment.Center // maintain alignment "relative to super"
    ) {
        val isLandscape = maxWidth > maxHeight
        if (isLandscape) {
            Row (
                modifier = Modifier.fillMaxWidth(),
                verticalAlignment = rowAlignment,
                horizontalArrangement = rowArrangement
            ) {
                content()
            }
        } else {
            Column (
                modifier = Modifier.fillMaxHeight(),
                verticalArrangement = colArrangement,
                horizontalAlignment = colAlignment
            ) {
                content()
            }
        }
    }
}

@Composable
fun Main(
    repo: BootRepository,
    modifier: Modifier = Modifier
) {
    // define UI

    var status by remember {
        // remapping to more useful labels. 0 = A ; 1 = B
        mutableStateOf(repo.currentSlot())
    }

    Box(
        modifier = modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        val configuration = LocalConfiguration.current
        val screenWidth = configuration.screenWidthDp.dp
        val screenHeight = configuration.screenHeightDp.dp

        Column(
            modifier = Modifier
                .width(screenWidth * 0.8f)
                .height(screenHeight * 0.8f)
                .padding(16.dp, 16.dp, 16.dp, 32.dp)
            ,
            verticalArrangement = Arrangement.spacedBy(12.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {

            Row (
                modifier = Modifier,
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Text(text = status)
                Button (
                    onClick = {}
                    )
                {
                    Text(text = "Reload ꭷⳲල")
                }
            }

            ResponsiveLayoutBox (
                // preserve alignment
                rowAlignment = Alignment.CenterVertically,
                rowArrangement = Arrangement.spacedBy(12.dp),
                colAlignment = Alignment.CenterHorizontally,
                colArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Button(
                    onClick = {status = if (repo.switchTo(slot=0)) "Set boot slot to A" else "Failed to load"},
                    )
                {
                    Text("Set to slot A (0)")
                }

                Button(
                    onClick = {status = if (repo.switchTo(slot=0)) "Set boot slot to A" else "Failed to load"},
                    )
                {
                    Text("Set to slot B (1)")
                }
            }
        }
    }
}

//* preview within Android Studio.
//! Update on significant changes only.
@Preview(showBackground = true)
@Composable
fun MainPreview() {
    BootManagerTheme {
        Column(
            modifier = Modifier.padding(16.dp, 16.dp, 16.dp, 32.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            Text("Current boot slot: A")
            Button(onClick = {}) {
                Text("Set to slot A (0)")
            }
            Button(onClick = {}) {
                Text("Set to slot B (1)")
            }
        }
    }
}
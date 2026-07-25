package com.carnx.bootmanager

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.compose.ui.tooling.preview.Preview
import com.carnx.bootmanager.BootManagerTheme

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

@Composable
fun Main(
    repo: BootRepository,
    modifier: Modifier = Modifier
) {
    var status by remember {
        mutableStateOf("Current boot slot: ${repo.currentSlot()}")
    }
    Box(
        modifier = modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Column(
            modifier = modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {

            Text(text = status)

            Button(onClick = {
                status = if (repo.switchTo(0))
                    "Set slot A"
                else
                    "Failed"
            }) {
                Text("Set to slot A (0)")
            }

            Button(onClick = {
                status = if (repo.switchTo(1))
                    "Set slot B"
                else
                    "Failed"
            }) {
                Text("Set to slot B (1)")
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
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            Text("Current slot: A")
            Button(onClick = {}) {
                Text("Boot slot A")
            }
            Button(onClick = {}) {
                Text("Boot slot B")
            }
        }
    }
}
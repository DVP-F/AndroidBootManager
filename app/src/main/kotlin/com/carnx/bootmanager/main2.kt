package com.carnx.bootmanager

import android.os.Bundle
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val repo = BootRepository()
        val status = TextView(this)
        val a = Button(this).apply { text = "Boot slot A" }
        val b = Button(this).apply { text = "Boot slot B" }

        status.text = "Current slot: ${repo.currentSlot()}"

        a.setOnClickListener {
            status.text = if (repo.switchTo(0)) "Set slot A" else "Failed"
        }

        b.setOnClickListener {
            status.text = if (repo.switchTo(1)) "Set slot B" else "Failed"
        }

        setContentView(
            LinearLayout(this).apply {
                orientation = LinearLayout.VERTICAL
                addView(status)
                addView(a)
                addView(b)
            }
        )
    }
}
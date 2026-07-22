plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("com.github.willir.rust.cargo-ndk-android")
}

android {
    namespace = "com.carnx.bootslot"
    compileSdk = 34
    
    defaultConfig {
        applicationId = "com.carnx.bootslot"
        minSdk = 29
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"
    }
    
    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                          "proguard-rules.pro"
            )
        }
        debug {
            isMinifyEnabled = false
        }
    }
    
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    
    kotlinOptions {
        jvmTarget = "17"
    }
}

cargoNdk {
    module = "../rust"
    apiLevel = 29
    targets = ["arm64", "arm", "x86_64", "x86"]
    buildTypes {
        debug {
            buildType = "debug"
        }
        release {
            buildType = "release"
        }
    }
}

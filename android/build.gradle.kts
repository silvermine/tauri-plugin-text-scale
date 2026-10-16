import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
}

// Match on the project directory as well as the path: a consuming Tauri app may
// have its own `:lib` module.
val isStandaloneLibBuild = findProject(":lib")?.projectDir == file("lib")

android {
    namespace = "org.silvermine.plugin.textscale"
    compileSdk = 37

    defaultConfig {
        minSdk = 23

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    // A Tauri app includes only this module, so it compiles the :lib sources
    // directly. AGP 9 needs this inside the `android` block.
    if (!isStandaloneLibBuild) {
        sourceSets {
            named("main") {
                java.srcDir("lib/src/main/java")
            }
        }
    }
}

kotlin {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_17
    }
}

dependencies {
    // Standalone build: depend on :lib as a separate module so its unit tests
    // can run on the JVM without the Android framework or the Tauri Android API.
    if (isStandaloneLibBuild) {
        implementation(project(":lib"))
    }
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    if (findProject(":tauri-android") != null) {
        implementation(project(":tauri-android"))
    }
}

## Table of Contents

- [Overview](#overview)
- [Basic Rust concepts](#basic-rust-concepts)
- [Project-specific patterns](#project-specific-patterns)
- [Android build optimisation](#android-build-optimisation)
- [JNI feedback bridge](#jni-feedback-bridge)
- [Useful commands for developers](#useful-commands-for-developers)
- [Updating app version notes](#updating-app-version-notes)
- [Releasing the application notes](#releasing-the-application-notes)
- [Resizing app logo for Android compatibility](#resizing-app-logo-for-android-compatibility)
  - [Creating the adaptive icon structure](#create-adaptive-icon-structure)
  - [Using GIMP to resize the logo](#using-gimp-to-resize-the-logo)

---

## Overview

This document provides an overview of the Rust concepts and project-specific patterns used in Lexivo to help new developers get up to speed quickly.

It
complements what is documented in the [README](/README.md).

The [RUST_PROGRAMMING_TUTORIAL.md](/docs/RUST_PROGRAMMING_TUTORIAL.md) outlines how to programme using Rust.

---

### Basic Rust concepts

#### `struct` (Structure)

A `struct` is a custom data type that lets you group together related values. In Lexivo, the most important struct is `LexivoApp` (found in `src/app.rs`), which holds the entire state of the game (scores, current puzzle, settings, etc.).

#### `enum` (Enumeration)

Enums allow you to define a type by enumerating its possible variants. We use them for states that are mutually exclusive:

- `Difficulty`: Easy, Medium, or Hard.
- `Screen`: Which view is currently active (Start, Game, Settings).
- `GameMode`: QuickPlay or DailyChallenge.

#### `impl` (Implementation)

`impl` blocks are where you define the functions (methods) associated with a `struct` or `enum`.

- `impl LexivoApp`: Contains the logic for checking answers, starting games and updating the state.
- `impl eframe::App for LexivoApp`: Implements the required interface for the UI framework to draw the app and save its state.

#### `mod` (Module)

Modules help organise code into different files.

- `pub mod app;` in `lib.rs` tells Rust to look for a file named `app.rs` and make its contents available.
- `#[cfg(test)] mod app_tests;` defines a module that only exists when running tests.

#### `use crate::...`

The `crate` keyword refers to the root of the current project.

- `use crate::types::Difficulty;` means "go to the project root, find the `types` module and import the `Difficulty` enum."

---

## Project-specific patterns

### UI/Logic split (`include!`)

To keep the code manageable, we've split the main application file:

- **`src/app.rs`**: Contains the "Business Logic" — state definitions, data loading and game rules.

- **`src/app_ui.rs`**: Contains the "View" — how the buttons, labels and tiles are actually drawn on the screen.

- We use `include!("app_ui.rs");` at the bottom of `app.rs` to merge them during compilation, allowing the UI code to access private fields of the app state without complex boilerplate.

### Data persistence (`serde`)

The `#[derive(Deserialize, Serialize)]` attribute on our structs allows them to be automatically converted to and from JSON. This is how Lexivo saves your high scores and settings to your device between sessions.

### The "Daily seed" system

Daily challenges need to be the same for everyone on a given day. We achieve this by:

1. Taking the current Unix timestamp.
2. Calculating how many days have passed since 1970.
3. Using that number as a "Seed" for our Random Number Generator (RNG).
   This ensures that `shuffle_slice` always produces the same puzzle order for every player until the next day starts.

### Cross-platform bridging

Lexivo is a "Native" app on Android but written in Rust.

- **`src/lib.rs`**: Contains the `android_main` entry point.
- **`src/platform_feedback.rs`**: Uses the `jni` (Java Native Interface) crate to talk to the Java-based `FeedbackBridge.java` for things like haptic feedback (vibration) which Rust cannot easily do directly on Android.

---

## Android build optimisation

Building Rust for all four Android ABIs (`arm64-v8a`, `armeabi-v7a`, `x86_64`, `x86`) on every Gradle sync or build can be very slow. We've implemented several optimizations:

### Single ABI development builds

In `gradle.properties`, we've enabled `singleAbiDev=true`. This tells Gradle to:

1. **Detect** the ABI of the currently connected device via ADB.
2. **Filter** the build to only compile Rust for that specific architecture.
3. **Strip** other ABIs from the final APK to keep it small during development.

If auto-detection fails, you can override it in `gradle.properties` (e.g., `devAbi=x86_64`) or via the command line:

```bash
./gradlew assembleDebug -PsingleAbiDev=true -PdevAbi=x86_64
```

### Build lifecycle hooking

We don't manually run `cargo-ndk`. Instead, the `app/build.gradle` defines custom `Exec` tasks that are hooked into the standard Android lifecycle (`preDebugBuild` and `preReleaseBuild`). This ensures that the Rust shared libraries are always up-to-date before the APK is packaged.

---

## JNI feedback bridge

Because Lexivo is a `NativeActivity` app, the main entry point is Rust. However, certain Android APIs (like `Vibrator` or `MediaPlayer`) are easier to use from Java.

1. **Rust side (`src/platform_feedback.rs`)**: Uses the JNI crate to find the `FeedbackBridge` class, attach to the JVM and call static methods.
2. **Java side (`FeedbackBridge.java`)**: A pure Java class that handles the heavy lifting of talking to Android System Services. It uses `AssetFileDescriptor` to play sounds directly from the APK assets without extraction.

---

## Useful commands for developers

- **Format everything**: `cargo fmt`
- **Check for common mistakes**: `cargo clippy`
- **Run all tests**: `cargo test`
- **Build for Android**: `cd android && ./gradlew assembleDebug`

---

---

## Updating app version notes

To update the app to a higher version:

1. Insert a new entry in the [CHANGELOG.md](/CHANGELOG.md) for the new app version.
2. In [app's build.gradle](/android/app/build.gradle) file, increment `versionCode` to the next number
   and update `versionName` with the new version of the app.
3. Add a text file to the `changelogs` directory for the Fastlane metadata in
   [fastlane/metadata/android/en-GB/changelogs](/fastlane/metadata/android/en-GB/changelogs). The
   name of the text file should be a number higher than the current number of the existing text
   file.

---

## Releasing the application notes

A keystore is used to store the signing key required for Android app releases.

### Create a keystore file

> Replace `keystore.jks` and `key-alias` with your desired values.

```bash
keytool -genkeypair \
  -v \
  -keystore keystore.jks \
  -alias key-alias \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000
```

### Encode the keystore file

The decoding of the file occurs in the GitHub repository workflow.

> Replace `keystore.jks` and `keystore.base64` with your file names.

```bash
base64 -i keystore.jks | tr -d '\n' > keystore.base64
```

### Update the example keystore file in the project

Rename the example keystore file [keystore.properties.example](/android/app/src/keystore.properties.example)
to `keystore.properties`. Then, update the values of the variables - `storeFile`, `storePassword`,
`keyAlias` and `keyPassword` with the values used above.

### Create environment secrets in GitHub repository

The following secrets should be created. Their values should match the values used above.

```txt
KEY_ALIAS
KEY_PASSWORD
KEYSTORE_CONTENT
KEYSTORE_FILE_NAME
KEYSTORE_PASSWORD
```

### Create app release

Push a tag matching the pattern `v*` to the `main` branch. The
[release.yml](/.github/workflows/release.yml) GitHub workflow will then automatically build the
release APK and bundle, sign them using the provided keystore and upload the artefacts.

Example tag for a release:

```bash
git checkout main
git tag v1.0.0
git push origin v1.0.0
```

#### Rebuild an existing tag

> Replace v1.0 with the actual tag version. Ensure that all commits has been pushed to GitHub
> before running the command below so that the tag would include the latest changes

```bash
git tag -f v1.0 && git push origin v1.0 --force
```

---

## Resizing app logo for Android compatibility

### Create adaptive icon structure

The **`ic_launcher.xml` and `ic_launcher_round.xml` are the adaptive-icon definitions**, while the actual white background and padded logo live separately in your project.

A typical structure is:

```text
app/
└── src/
    └── main/
        └── res/
            │
            ├── drawable/
            │   └── ic_launcher_foreground.png   ← or a XML file
            │
            ├── mipmap-anydpi-v26/
            │   ├── ic_launcher.xml
            │   └── ic_launcher_round.xml
            │
            └── mipmap-.../
            │    └── legacy PNG icons like mipmap-mdpi/ic_launcher.png etc
            │
            ├── values/
            │   └── colours.xml    ← includes a colour resource for ic_launcher_background
```

#### Step 1. Create `ic_launcher.xml`

Create:

```text
app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
```

Put this inside:

```xml
<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background" />
    <foreground android:drawable="@drawable/ic_launcher_foreground" />
</adaptive-icon>
```

---

#### Step 2. Create `ic_launcher_round.xml`

Create:

```text
app/src/main/res/mipmap-anydpi-v26/ic_launcher_round.xml
```

Put the **same thing** inside it as the previous step:

```xml
<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background" />
    <foreground android:drawable="@drawable/ic_launcher_foreground" />
</adaptive-icon>
```

The distinction is that Android may use `ic_launcher` for the normal icon and `ic_launcher_round` where a launcher specifically requests the round variant.

---

#### Step 3. Create a white background for the app icon

Create:

```text
app/src/main/res/values/colours.xml
```

Place this inside of it:

```xml
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="ic_launcher_background">#FFFFFF</color>
</resources>
```

So:

```text
@color/ic_launcher_background
          │
          ▼
    #FFFFFF
```

This gives the adaptive icon a **white background**.

---

#### Step 4. The foreground i.e. the logo with transparent padding around it

Create:

```text
app/src/main/res/drawable/ic_launcher_foreground.png
```

For example:

```text
res/
└── drawable/
    └── ic_launcher_foreground.png
```

That PNG should contain **your logo with transparent space around it**.

Refer to [Using GIMP to resize the logo](#using-gimp-to-resize-the-logo) to properly size your logo within the 512 × 512 PNG.

For example, imagine a PNG that is 512 × 512.

Don't make the logo occupy all 512 × 512:

```text
┌─────────────────────┐
│█████████████████████│
│█████████████████████│
│█████████████████████│
│█████████████████████│
└─────────────────────┘
```

Instead, make the actual logo smaller to about 350 x 350 size:

```text
┌─────────────────────┐
│                     │
│                     │
│       ███████       │
│      █████████      │
│       ███████       │
│                     │
│                     │
└─────────────────────┘
```

The transparent area is intentional.

Then:

```xml
<foreground android:drawable="@drawable/ic_launcher_foreground" />
```

means:

> Take this image and put it on top of the white background.

---

#### Step 5. What about the existing `mipmap-*` PNGs?

This is the part that can be confusing.

You may currently have:

```text
mipmap-mdpi/ic_launcher.png
mipmap-hdpi/ic_launcher.png
mipmap-xhdpi/ic_launcher.png
mipmap-xxhdpi/ic_launcher.png
mipmap-xxxhdpi/ic_launcher.png
```

Those are **legacy launcher icons**.

The adaptive icon:

```text
mipmap-anydpi-v26/ic_launcher.xml
```

is used on **Android 8.0/API 26 and newer**.

The `-v26` is significant:

```text
mipmap-anydpi-v26
              ↑
         Android 8+
```

For older Android versions, the PNGs are used.

So if you want your application to look correct on **older Android versions too**, you should update the legacy PNGs as well.

But you don't necessarily need to manually create five different logos.

You can generate the appropriate density PNGs from your source artwork.

---

#### Step 6. Important note: Adaptive icons have a "safe zone"

Android adaptive icons aren't simply:

```text
┌───────────────┐
│               │
│     LOGO      │
│               │
└───────────────┘
```

The launcher can apply different masks:

```text
       square
    ┌───────────┐
    │           │
    │   LOGO    │
    │           │
    └───────────┘

       circle
       ╭───────╮
      /         \
     |   LOGO    |
      \         /
       ╰───────╯

    rounded square
    ╭───────────╮
    │           │
    │   LOGO    │
    │           │
    ╰───────────╯
```

The launcher controls the final mask.

Therefore, your logo needs sufficient padding so that it remains visually comfortable under different masks.

---

### Using GIMP to resize the logo

A 512×512 size logo or of any other size can be resized with GIMP while keeping the canvas at 512×512 while making the actual logo smaller by adding transparent padding around it. It can be done using GIMP.

#### Expected result

Start with:

```text
512 × 512 PNG
┌──────────────────────────────┐
│                              │
│      ██████████████████      │
│      ██████████████████      │
│      ██████████████████      │
│                              │
└──────────────────────────────┘
```

Make it something like:

```text
512 × 512 PNG
┌──────────────────────────────┐
│                              │
│                              │
│          ████████            │
│          ████████            │
│          ████████            │
│                              │
│                              │
└──────────────────────────────┘
```

The area around the logo remains **transparent**.

Then Android puts that foreground over the white background.

#### Step 1. Open your logo in GIMP

In GIMP:

**File → Open**

Select your 512×512 PNG.

First check:

**Image → Image Properties**

You should see:

```text
Width: 512 px
Height: 512 px
```

---

#### Step 2. Make sure the image has transparency

Look at the Layers panel.

Right-click your logo layer.

If you see:

**Add Alpha Channel**

click it.

If you instead see:

**Remove Alpha Channel**

then the layer already has transparency.

---

#### Step 3. Duplicate the original layer

Before changing anything, duplicate the layer to have a backup if anything goes wrong:

**Layer → Duplicate Layer**

You'll now have something like:

```text
Layers
────────────────────
Logo copy       ← work on this
Logo            ← original backup
────────────────────
```

You can hide the original layer by clicking its eye icon.

---

#### Step 4. Scale the logo itself using Layer

Select the logo layer and use:

**Layer → Scale Layer**

You'll get a dialog.

If your current logo layer is 512×512, you can change it to something smaller.
350 px x 350 px is the recommended resize option.

For example:

```text
Width: 350 px
Height: 350 px
```

Make sure the **chain/link icon is locked**, so the aspect ratio stays correct.

Then click:

**Scale**

Now your logo layer is 350×350.

---

#### Step 5. Centre the smaller logo

Use:

**Layer → Layer to Image Size**

This makes the layer's canvas 512×512 again while preserving the smaller logo.

Then use:

**Alignment Tool**

or manually position the logo in the center.

An easier method in recent GIMP versions is:

**Tools → Transform Tools → Align**

Select the logo layer and align it:

```text
Horizontal: Center
Vertical: Center
```

You want:

```text
512 × 512 canvas

┌──────────────────────────────┐
│                              │
│                              │
│           ██████             │
│          ████████            │
│           ██████             │
│                              │
│                              │
└──────────────────────────────┘
```

---

#### Step 6. Alternatively, use GIMP's Scale Tool

Select the **Scale Tool**:

**Tools → Transform Tools → Scale**

Then click on your logo.

You can interactively shrink it.

For example, start with:

```text
512 × 512
```

and experiment with:

```text
400 × 400
350 × 350
300 × 300
```

until the visual size looks right.

Remember: **the actual logo doesn't necessarily need to be a particular pixel size**. What matters is how large it appears once Android applies its launcher mask.

---

#### Step 7. Important: Don't make the white background part of the PNG

This is particularly important for the adaptive icon setup.

Your final foreground PNG should look like:

```text
ic_launcher_foreground.png

512 × 512

┌──────────────────────────────┐
│                              │
│                              │
│           YOUR               │
│           LOGO               │
│                              │
│                              │
└──────────────────────────────┘

       transparent
```

**Not:**

```text
┌──────────────────────────────┐
│            WHITE             │
│                              │
│           LOGO               │
│                              │
│            WHITE             │
└──────────────────────────────┘
```

The white comes from:

```xml
<background android:drawable="@color/ic_launcher_background" />
```

---

#### Step 8. Export it from GIMP

Once you're happy with the size:

**File → Export As**

Name it:

```text
ic_launcher_foreground.png
```

Put it in:

```text
app/src/main/res/drawable/
```

So you have:

```text
app/
└── src/
    └── main/
        └── res/
            └── drawable/
                └── ic_launcher_foreground.png
```

When exporting, make sure you're keeping the **alpha/transparency**.

If GIMP shows an option related to saving transparency, don't remove it.

---

#### Step 9. What size should the logo be?

For a 512×512 foreground, start around:

##### Option A — moderately small

```text
Logo: ~350 × 350
Canvas: 512 × 512
```

##### Option B — noticeably smaller

```text
Logo: ~300 × 300
Canvas: 512 × 512
```

##### Option C — quite small

```text
Logo: ~250 × 250
Canvas: 512 × 512
```

---

### Using Android Studio's image asset studio

1. Right-click on the res folder.

2. Select New > Image Asset.

3. For Path, select your high-quality source image (the one in drawable or on your computer).

4. Under the Foreground Layer tab, use the Scaling slider.
   
   ◦ Slide it to the left (e.g., to 60% or 70%) to make the logo look smaller.
   ◦ Android Studio will show you a "Safe Zone" circle to make sure your logo doesn't get cut off.

5. Click Next and then Finish.

---


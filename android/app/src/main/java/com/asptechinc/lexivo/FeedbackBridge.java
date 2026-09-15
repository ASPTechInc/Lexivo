package com.asptechinc.lexivo;

import android.content.Context;
import android.content.res.AssetFileDescriptor;
import android.media.AudioAttributes;
import android.media.SoundPool;
import android.os.Build;
import android.os.VibrationEffect;
import android.os.Vibrator;
import android.os.VibratorManager;
import android.util.Log;

import java.util.HashMap;
import java.util.Map;

/**
 * A utility bridge that allows the Rust game engine to trigger Android-specific hardware and media feedback.
 * This class is optimised using SoundPool for low-latency audio and cached system services.
 * Android Studio's Java/Kotlin static analysis highlights AndroidFeedbackBridge.java and its
 * methods as unused so suppress the warning.
 */
@SuppressWarnings("unused")
public final class FeedbackBridge {
    private static final String TAG = "LexivoFeedback";

    private static SoundPool soundPool;
    private static final Map<String, Integer> soundMap = new HashMap<>();
    private static Vibrator cachedVibrator;
    private static boolean initialized = false;

    private FeedbackBridge() {
    }

    /**
     * Initialises the feedback bridge by preloading sound assets and caching system services.
     * Should be called once during application startup.
     */
    public static synchronized void init(Context context) {
        if (initialized || context == null) {
            return;
        }

        Log.d(TAG, "Initializing FeedbackBridge...");

        // Initialize SoundPool
        AudioAttributes attrs = new AudioAttributes.Builder()
                .setUsage(AudioAttributes.USAGE_GAME)
                .setContentType(AudioAttributes.CONTENT_TYPE_SONIFICATION)
                .build();

        soundPool = new SoundPool.Builder()
                .setMaxStreams(3)
                .setAudioAttributes(attrs)
                .build();

        // Preload sound effects
        loadSound(context, "correct-buzzer-sound-effect.mp3");
        loadSound(context, "wrong-buzzer-sound-effect.mp3");

        // Cache Vibrator
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            VibratorManager manager = (VibratorManager) context.getSystemService(Context.VIBRATOR_MANAGER_SERVICE);
            if (manager != null) {
                cachedVibrator = manager.getDefaultVibrator();
            }
        } else {
            //noinspection deprecation
            cachedVibrator = (Vibrator) context.getSystemService(Context.VIBRATOR_SERVICE);
        }

        initialized = true;
        Log.d(TAG, "FeedbackBridge initialized successfully");
    }

    private static void loadSound(Context context, String fileName) {
        String path = "sound-effects/" + fileName;
        try (AssetFileDescriptor afd = context.getAssets().openFd(path)) {
            int soundId = soundPool.load(afd, 1);
            soundMap.put(fileName, soundId);
        } catch (Exception e) {
            Log.e(TAG, "Failed to pre-load sound: " + path, e);
        }
    }

    /**
     * Triggers both haptic (vibration) and audio feedback.
     */
    public static void triggerAnswerFeedback(Context context, boolean correct, boolean soundEnabled) {
        if (!initialized) {
            init(context);
        }

        try {
            vibrate(correct ? 35L : 75L);

            if (soundEnabled) {
                String soundName = correct ? "correct-buzzer-sound-effect.mp3" : "wrong-buzzer-sound-effect.mp3";
                playSound(soundName);
            }
        } catch (Throwable error) {
            Log.e(TAG, "Error in triggerAnswerFeedback", error);
        }
    }

    private static void vibrate(long durationMs) {
        if (cachedVibrator == null || !cachedVibrator.hasVibrator()) {
            return;
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            cachedVibrator.vibrate(VibrationEffect.createOneShot(durationMs, VibrationEffect.DEFAULT_AMPLITUDE));
        } else {
            // Deprecated in API 26, but we check SDK_INT above
            //noinspection deprecation
            cachedVibrator.vibrate(durationMs);
        }
    }

    private static void playSound(String soundName) {
        Integer soundId = soundMap.get(soundName);
        if (soundId != null && soundPool != null) {
            soundPool.play(soundId, 1.0f, 1.0f, 1, 0, 1.0f);
        } else {
            Log.w(TAG, "Sound not loaded: " + soundName);
        }
    }
}

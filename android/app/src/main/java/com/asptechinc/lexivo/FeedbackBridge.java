package com.asptechinc.lexivo;

import android.content.res.AssetFileDescriptor;
import android.content.Context;
import android.media.MediaPlayer;
import android.os.Build;
import android.os.VibrationEffect;
import android.os.Vibrator;
import android.os.VibratorManager;
import android.util.Log;

/**
 * A utility bridge that allows the Rust game engine to trigger Android-specific hardware and media feedback.
 * This class is invoked via JNI from {@code platform_feedback.rs}.
 */
public final class FeedbackBridge {
    private static final String TAG = "LexivoFeedback";

    private FeedbackBridge() {
    }

    /**
     * Triggers both haptic (vibration) and audio feedback based on whether an answer was correct.
     *
     * @param context      The Android application context.
     * @param correct      True if the user's answer was correct, false otherwise.
     * @param soundEnabled Whether sound effects are currently enabled in the app settings.
     */
    public static void triggerAnswerFeedback(Context context, boolean correct, boolean soundEnabled) {
        Log.d(TAG, "triggerAnswerFeedback called: correct=" + correct + ", soundEnabled=" + soundEnabled);
        if (context == null) {
            Log.e(TAG, "Context is null in triggerAnswerFeedback");
            return;
        }

        try {
            vibrate(context, correct ? 35L : 75L);

            if (soundEnabled) {
                playBuzzer(context, correct);
            } else {
                Log.d(TAG, "Sound disabled; skipping buzzer playback");
            }
        } catch (Throwable error) {
            Log.e(TAG, "Unhandled error in triggerAnswerFeedback", error);
        }
    }

    /**
     * Vibrates the device for a specified duration.
     * Uses the {@link VibratorManager} on API 31+ and falls back to {@link Vibrator} on older versions.
     */
    private static void vibrate(Context context, long durationMs) {
        try {
            Vibrator vibrator;
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                VibratorManager manager = (VibratorManager) context.getSystemService(Context.VIBRATOR_MANAGER_SERVICE);
                vibrator = manager != null ? manager.getDefaultVibrator() : null;
            } else {
                vibrator = (Vibrator) context.getSystemService(Context.VIBRATOR_SERVICE);
            }

            if (vibrator == null || !vibrator.hasVibrator()) {
                Log.d(TAG, "No vibrator available on this device");
                return;
            }

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                vibrator.vibrate(VibrationEffect.createOneShot(durationMs, VibrationEffect.DEFAULT_AMPLITUDE));
            } else {
                vibrator.vibrate(durationMs);
            }
        } catch (Exception error) {
            Log.w(TAG, "Vibration failed", error);
        }
    }

    /**
     * Attempts to play a buzzer sound effect from the application assets.
     */
    private static void playBuzzer(Context context, boolean correct) {
        String[] candidates = correct
                ? new String[] { "sound-effects/correct-buzzer.mp3", "sound-effects/correct-buzzer-sound-effect.mp3" }
                : new String[] { "sound-effects/incorrect-buzzer.mp3", "sound-effects/wrong-buzzer-sound-effect.mp3" };

        for (String path : candidates) {
            if (playAssetFile(context, path)) {
                Log.d(TAG, "Played buzzer asset: " + path);
                return;
            }
        }

        Log.w(TAG, "No buzzer asset could be played for correct=" + correct);
    }

    /**
     * Plays a media file directly from the APK assets using {@link AssetFileDescriptor}.
     * This avoids the need to extract the file to temporary storage.
     */
    private static boolean playAssetFile(Context context, String assetPath) {
        MediaPlayer player = new MediaPlayer();
        try (AssetFileDescriptor afd = context.getAssets().openFd(assetPath)) {
            player.setDataSource(afd.getFileDescriptor(), afd.getStartOffset(), afd.getLength());
            player.setOnCompletionListener(MediaPlayer::release);
            player.setOnErrorListener((mp, what, extra) -> {
                Log.e(TAG, "MediaPlayer error for " + assetPath + " what=" + what + " extra=" + extra);
                mp.release();
                return true;
            });
            player.prepare();
            player.start();
            return true;
        } catch (Exception error) {
            Log.w(TAG, "Failed to play asset: " + assetPath, error);
            player.release();
            return false;
        }
    }
}

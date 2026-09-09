package com.asptechinc.lexivo;

import android.app.DownloadManager;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.database.Cursor;
import android.net.Uri;
import android.os.Build;
import android.os.Environment;
import android.util.Log;
import android.widget.Toast;
import androidx.core.content.FileProvider;
import java.io.File;

/**
 * A bridge to handle application updates on Android.
 * This class handles downloading and triggering the system package installer for APKs.
 */
public final class UpdateBridge {
    private static final String TAG = "LexivoUpdate";

    private UpdateBridge() {}

    /**
     * Downloads an APK from the given URL and prompts for installation once finished.
     *
     * @param context The Android application context.
     * @param url     The URL to the APK file (e.g. GitHub release asset).
     */
    public static void downloadAndInstallUpdate(final Context context, String url) {
        Log.d(TAG, "downloadAndInstallUpdate called for URL: " + url);
        if (context == null || url == null || url.isEmpty()) {
            Log.e(TAG, "Invalid context or URL in downloadAndInstallUpdate");
            return;
        }

        try {
            DownloadManager.Request request = new DownloadManager.Request(Uri.parse(url));
            request.setTitle("Lexivo Update");
            request.setDescription("Downloading latest version...");
            request.setNotificationVisibility(DownloadManager.Request.VISIBILITY_VISIBLE_NOTIFY_COMPLETED);
            
            // Save to external files dir (accessible via FileProvider)
            String fileName = "Lexivo-Update.apk";
            File destinationFile = new File(context.getExternalFilesDir(Environment.DIRECTORY_DOWNLOADS), fileName);
            if (destinationFile.exists()) {
                destinationFile.delete();
            }
            request.setDestinationUri(Uri.fromFile(destinationFile));

            final DownloadManager manager = (DownloadManager) context.getSystemService(Context.DOWNLOAD_SERVICE);
            if (manager == null) {
                Log.e(TAG, "DownloadManager not available");
                return;
            }

            final long downloadId = manager.enqueue(request);
            Toast.makeText(context, "Update download started...", Toast.LENGTH_SHORT).show();

            // Register a receiver to handle the completion
            BroadcastReceiver onComplete = new BroadcastReceiver() {
                @Override
                public void onReceive(Context ctx, Intent intent) {
                    long id = intent.getLongExtra(DownloadManager.EXTRA_DOWNLOAD_ID, -1);
                    if (downloadId == id) {
                        Log.d(TAG, "Download completed for ID: " + id);
                        installDownloadedApk(ctx, manager, id);
                        ctx.unregisterReceiver(this);
                    }
                }
            };

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                context.registerReceiver(onComplete, new IntentFilter(DownloadManager.ACTION_DOWNLOAD_COMPLETE), Context.RECEIVER_NOT_EXPORTED);
            } else {
                context.registerReceiver(onComplete, new IntentFilter(DownloadManager.ACTION_DOWNLOAD_COMPLETE));
            }

        } catch (Exception error) {
            Log.e(TAG, "Failed to start download", error);
            Toast.makeText(context, "Failed to start update download", Toast.LENGTH_LONG).show();
        }
    }

    private static void installDownloadedApk(Context context, DownloadManager manager, long downloadId) {
        DownloadManager.Query query = new DownloadManager.Query();
        query.setFilterById(downloadId);
        try (Cursor cursor = manager.query(query)) {
            if (cursor != null && cursor.moveToFirst()) {
                int statusIndex = cursor.getColumnIndex(DownloadManager.COLUMN_STATUS);
                if (statusIndex != -1 && DownloadManager.STATUS_SUCCESSFUL == cursor.getInt(statusIndex)) {
                    int uriIndex = cursor.getColumnIndex(DownloadManager.COLUMN_LOCAL_URI);
                    if (uriIndex != -1) {
                        String uriString = cursor.getString(uriIndex);
                        if (uriString != null) {
                            Uri uri = Uri.parse(uriString);
                            String path = uri.getPath();
                            if (path != null) {
                                installUpdate(context, new File(path));
                            }
                        }
                    }
                } else {
                    Log.e(TAG, "Download was not successful or status column missing");
                }
            }
        } catch (Exception e) {
            Log.e(TAG, "Error checking download status", e);
        }
    }

    private static void installUpdate(Context context, File apkFile) {
        if (!apkFile.exists()) {
            Log.e(TAG, "APK file does not exist at: " + apkFile.getAbsolutePath());
            return;
        }

        try {
            Uri apkUri = FileProvider.getUriForFile(
                context,
                context.getPackageName() + ".fileprovider",
                apkFile
            );

            Intent intent = new Intent(Intent.ACTION_VIEW);
            intent.setDataAndType(apkUri, "application/vnd.android.package-archive");
            intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION);
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);

            Log.d(TAG, "Starting package installer for URI: " + apkUri);
            context.startActivity(intent);
        } catch (Exception error) {
            Log.e(TAG, "Failed to launch package installer", error);
            Toast.makeText(context, "Failed to launch installer", Toast.LENGTH_LONG).show();
        }
    }
}

# Privacy policy

Last updated: 09-09-2026

Lexivo is a free and open source software (FOSS) cross-platform word puzzle application.
This privacy policy explains what data the app handles and how.

## Summary

- Lexivo does not require account sign-in.
- Lexivo does not run any analytics or advertising trackers.
- Lexivo stores your game data locally on your device.
- The app operates entirely offline for core gameplay.
- A network connection is used only when you manually check for application updates or choose to access external links.

## Data we process

Lexivo stores user-generated game state and preferences locally, such as:

- High scores and leaderboard history.
- Current game progress (current puzzle, win streaks).
- Game settings (sound, animations, theme, text size).
- Selected difficulty levels and puzzle counts.

This data is stored locally using platform-native storage (e.g. SharedPreferences on Android, LocalStorage on Web or local files on Desktop) via the `eframe` persistence mechanism.

## Data sharing

Lexivo does not transmit your game data to any external servers. All progress and scores remain on your device.

## Network connectivity and update checks

Lexivo includes a feature to check for application updates. The app makes a request to the **GitHub API** (`api.github.com`) to compare your current version with the latest release when you initiate an update check. During this process:

- No personal data or device identifiers are transmitted by Lexivo.
- Standard network metadata (such as your IP address) is processed by GitHub according to their own privacy policies.

Additionally, Lexivo includes links to its source code, support pages and documentation (e.g. GitHub, Buy Me a Coffee). These links are opened in your device's default web browser.

## Permissions

Lexivo requires minimal permissions to operate, primarily focusing on:
- **Internet access**: Only used for update checks and opening external links in the browser.
- **Local storage**: Used for saving your progress and settings.

## Children

Lexivo is a general-purpose word game suitable for all ages. It does not collect any personally identifiable information from any users, including children.

## Open source transparency

You can inspect the source code to verify data handling behaviour because Lexivo is FOSS:

- Repository: https://github.com/ASPTechInc/Lexivo

## Changes to this policy

This policy may be updated over time to reflect app changes. The latest version is always kept in the official repository.

## Contact

For privacy questions or concerns:

- Email: 318654411+ASPTechInc@users.noreply.github.com

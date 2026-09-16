use super::*;
use crate::types::UpdateStatus;
use crate::utils::daily_seed_for_day;

#[test]
fn score_round_trips_through_json() {
    // Verifies that the app score is correctly preserved during JSON serialization and deserialization.
    let app = LexivoApp {
        score: 42,
        ..LexivoApp::default()
    };

    let json = serde_json::to_string(&app).expect("serialize app");
    let restored: LexivoApp = serde_json::from_str(&json).expect("deserialize app");

    assert_eq!(restored.score, 42);
}

#[test]
fn answer_validation_ignores_case_and_whitespace() {
    // Verifies that normalization correctly handles case sensitivity and leading/trailing whitespace.
    assert_eq!(normalise_answer("  nurse  "), "NURSE");
    assert_eq!(normalise_answer("nUrSe"), "NURSE");
}

#[test]
fn answer_validation_handles_empty_input_and_punctuation() {
    // Verifies that normalization correctly handles empty strings and non-alphanumeric characters.
    assert_eq!(normalise_answer(""), "");
    assert_eq!(normalise_answer("   "), "");
    assert_eq!(normalise_answer("nurse!"), "NURSE");
    assert_eq!(normalise_answer("n-u-r-s-e"), "NURSE");
}

#[test]
fn timer_expiration_marks_game_over() {
    let mut app = LexivoApp {
        screen: Screen::Game,
        show_game_instructions: false,
        timer_enabled: true,
        time_left: 0.1,
        ..LexivoApp::default()
    };

    app.update_timer(0.2);

    assert!(app.game_over);
    assert_eq!(app.time_left, 0.0);
}

#[test]
fn daily_challenge_uses_selected_question_count() {
    let mut app = LexivoApp::default();
    app.set_question_count(20);
    app.start_daily_challenge();

    assert_eq!(app.challenge_total, 20);
    assert!(app.daily_challenge_active);
}

#[test]
fn all_mode_uses_full_puzzle_pool() {
    let mut app = LexivoApp::default();
    app.set_question_count(0);
    app.start_daily_challenge();

    assert_eq!(app.challenge_total, app.puzzles.len());
}

#[test]
fn daily_seed_is_stable_for_the_same_day() {
    assert_eq!(
        daily_seed_for_day(1_700_000_000),
        daily_seed_for_day(1_700_000_000)
    );
}

#[test]
fn daily_seed_changes_for_different_days() {
    assert_ne!(
        daily_seed_for_day(1_700_000_000),
        daily_seed_for_day(1_700_000_000 + 86_400)
    );
}

#[test]
fn daily_seed_changes_by_difficulty_on_same_day() {
    let easy = daily_seed_for_difficulty(Difficulty::Easy, 1_700_000_000);
    let medium = daily_seed_for_difficulty(Difficulty::Medium, 1_700_000_000);
    let hard = daily_seed_for_difficulty(Difficulty::Hard, 1_700_000_000);

    assert_ne!(easy, medium);
    assert_ne!(easy, hard);
    assert_ne!(medium, hard);
}

#[test]
fn start_screen_selects_mode_and_question_count() {
    let mut app = LexivoApp::default();
    app.set_question_count(30);
    app.set_mode(GameMode::DailyChallenge);
    app.start_selected_mode();

    assert_eq!(app.selected_question_count, 30);
    assert!(app.daily_challenge_active);
}

#[test]
fn challenge_progress_increments_after_correct_answer() {
    let mut app = LexivoApp::default();
    app.set_question_count(2);
    app.start_daily_challenge();

    let original = app.puzzles[app.current_index].answer.clone();
    app.user_input = original.clone();
    app.check_answer();

    assert_eq!(app.challenge_progress, 1);
    #[expect(clippy::cast_possible_truncation)]
    let bonus = (app.time_left.ceil() as u32).max(1);
    assert_eq!(app.score, 10 + bonus);
}

#[test]
fn challenge_ends_cleanly_when_configured_count_is_reached() {
    let mut app = LexivoApp::default();
    app.set_question_count(1);
    app.start_daily_challenge();

    let original = app.puzzles[app.current_index].answer.clone();
    app.user_input = original;
    app.check_answer();

    assert!(app.game_over);
    assert!(!app.daily_challenge_active);
    assert_eq!(app.challenge_progress, 1);
}

#[test]
fn quick_play_resets_round_state() {
    let mut app = LexivoApp {
        score: 999,
        streak: 4,
        time_left: 1.0,
        ..LexivoApp::default()
    };
    app.set_mode(GameMode::QuickPlay);

    app.start_selected_mode();

    assert_eq!(app.score, 0);
    assert_eq!(app.streak, 0);
    assert_eq!(app.time_left, 60.0);
    assert!(!app.daily_challenge_active);
}

#[test]
fn daily_challenge_keeps_mode_state_and_count() {
    // Verifies that starting a Daily Challenge correctly sets the app mode and question count.
    let mut app = LexivoApp::default();
    app.set_question_count(30);
    app.set_mode(GameMode::DailyChallenge);
    app.start_selected_mode();

    assert!(app.daily_challenge_active);
    assert_eq!(app.challenge_total, 30);
    assert_eq!(app.selected_mode, GameMode::DailyChallenge);
}

#[test]
fn exactly_zero_seconds_marks_game_over() {
    // Verifies that the game ends immediately when the timer hits zero.
    let mut app = LexivoApp {
        screen: Screen::Game,
        show_game_instructions: false,
        timer_enabled: true,
        time_left: 0.0,
        ..LexivoApp::default()
    };

    app.update_timer(0.0);

    assert!(app.game_over);
}

#[test]
fn game_over_blocks_scoring() {
    // Verifies that no points can be earned once the game is over.
    let mut app = LexivoApp {
        game_over: true,
        user_input: "NURSE".to_owned(),
        score: 10,
        ..LexivoApp::default()
    };

    app.check_answer();

    assert_eq!(app.score, 10);
}

#[test]
fn next_puzzle_resets_timer() {
    // Verifies that moving to a new puzzle resets the round timer to the selected limit.
    let mut app = LexivoApp {
        time_left: 1.0,
        ..LexivoApp::default()
    };
    app.next_puzzle();

    assert_eq!(app.time_left, 60.0);
}

#[test]
fn question_count_sets_expected_timer_lengths() {
    let mut app = LexivoApp::default();

    app.set_question_count(10);
    assert_eq!(app.time_left, 60.0);
    assert!(app.timer_enabled);

    app.set_question_count(20);
    assert_eq!(app.time_left, 120.0);
    assert!(app.timer_enabled);

    app.set_question_count(30);
    assert_eq!(app.time_left, 180.0);
    assert!(app.timer_enabled);
}

#[test]
fn all_questions_disables_timer() {
    let mut app = LexivoApp::default();
    app.set_question_count(0);

    assert!(!app.timer_enabled);
    assert_eq!(app.time_left, 0.0);
}

#[test]
fn hint_reaching_zero_points_triggers_game_over() {
    let mut app = LexivoApp {
        score: 5,
        ..LexivoApp::default()
    };

    app.reveal_hint_letter();

    assert_eq!(app.score, 0);
    assert!(app.game_over);
    assert_eq!(app.message, "Game over! Score reached zero.");
}

#[test]
fn leaderboard_includes_high_score_and_sorts_descending() {
    // Verifies that the leaderboard correctly tracks and sorts high scores in descending order.
    let mut app = LexivoApp {
        score: 30,
        ..LexivoApp::default()
    };
    app.record_score();
    app.score = 50;
    app.record_score();
    app.score = 40;
    app.record_score();

    assert_eq!(app.leaderboard, vec![50, 40, 30]);
}

#[test]
fn leaderboard_trims_to_top_five() {
    // Verifies that the leaderboard only retains the top five highest scores.
    let mut app = LexivoApp::default();
    for value in [1, 2, 3, 4, 5, 6] {
        app.score = value;
        app.record_score();
    }

    assert_eq!(app.leaderboard.len(), 5);
    assert_eq!(app.leaderboard[0], 6);
    assert_eq!(app.leaderboard[4], 2);
}

#[test]
fn reset_progress_clears_stats_without_changing_round_state() {
    let mut app = LexivoApp {
        score: 37,
        best_score: 90,
        streak: 4,
        best_streak: 11,
        leaderboard: vec![90, 70, 50],
        user_input: "ABCD".to_owned(),
        time_left: 12.5,
        game_over: true,
        ..LexivoApp::default()
    };

    app.reset_progress();

    assert_eq!(app.score, 37);
    assert_eq!(app.best_score, 0);
    assert_eq!(app.streak, 0);
    assert_eq!(app.best_streak, 0);
    assert!(app.leaderboard.is_empty());
    assert_eq!(app.user_input, "ABCD");
    assert_eq!(app.time_left, 12.5);
    assert!(app.game_over);
    assert_eq!(app.message, "Progress reset");
}

#[test]
fn hint_reveals_one_letter_and_costs_points() {
    // Verifies that using a hint reveals a letter and deducts the appropriate number of points.
    let mut app = LexivoApp {
        score: 20,
        ..LexivoApp::default()
    };

    app.reveal_hint_letter();

    assert_eq!(app.score, 15);
    assert_eq!(app.revealed_answer_indices.len(), 1);
}

#[test]
fn hint_requires_enough_points() {
    let mut app = LexivoApp {
        score: 4,
        ..LexivoApp::default()
    };

    app.reveal_hint_letter();

    assert_eq!(app.score, 4);
    assert!(app.revealed_answer_indices.is_empty());
}

#[test]
fn theme_mode_defaults_to_system() {
    // Verifies that the application theme defaults to the system setting on first launch.
    let app = LexivoApp::default();
    assert_eq!(app.theme_mode, ThemeMode::System);
}

#[test]
fn theme_mode_round_trips_through_json() {
    let app = LexivoApp {
        theme_mode: ThemeMode::Dark,
        ..LexivoApp::default()
    };

    let json = serde_json::to_string(&app).expect("serialize app");
    let restored: LexivoApp = serde_json::from_str(&json).expect("deserialize app");

    assert_eq!(restored.theme_mode, ThemeMode::Dark);
}

#[test]
fn scramble_word_handles_single_char() {
    // A single character word should always remain unchanged after scrambling.
    let mut state = 12345;
    assert_eq!(scramble_word("A", &mut state), "A");
}

#[test]
fn scramble_word_handles_identical_chars() {
    let mut state = 12345;
    assert_eq!(scramble_word("AAAAA", &mut state), "AAAAA");
}

#[test]
fn scramble_word_changes_word() {
    // Verifies that a standard word is scrambled to a different arrangement while preserving characters.
    let mut state = 12345;
    let original = "LEXIVO";
    let scrambled = scramble_word(original, &mut state);
    assert_ne!(original, scrambled);

    // Character set must be the same
    let mut original_chars: Vec<char> = original.chars().collect();
    let mut scrambled_chars: Vec<char> = scrambled.chars().collect();
    original_chars.sort_unstable();
    scrambled_chars.sort_unstable();
    assert_eq!(original_chars, scrambled_chars);
}

#[test]
fn shuffle_slice_empty_and_single() {
    let mut state = 12345;
    let mut empty: Vec<i32> = vec![];
    shuffle_slice(&mut empty, &mut state);
    assert!(empty.is_empty());

    let mut single = vec![1];
    shuffle_slice(&mut single, &mut state);
    assert_eq!(single, vec![1]);
}

#[test]
fn shuffle_slice_is_deterministic_with_seed() {
    // Verifies that the shuffle result is consistent when using the same starting seed.
    let seed = 42;
    let mut state1 = seed;
    let mut items1 = vec![1, 2, 3, 4, 5];
    shuffle_slice(&mut items1, &mut state1);

    let mut state2 = seed;
    let mut items2 = vec![1, 2, 3, 4, 5];
    shuffle_slice(&mut items2, &mut state2);

    assert_eq!(items1, items2);
}

#[test]
fn normalise_answer_filters_special_chars() {
    assert_eq!(normalise_answer("word-123!"), "WORD123");
    assert_eq!(normalise_answer("  multiple   words  "), "MULTIPLEWORDS");
}

#[test]
fn remove_char_at_drops_only_selected_slot() {
    assert_eq!(LexivoApp::remove_char_at("WORD", 1), "WRD");
    assert_eq!(LexivoApp::remove_char_at("WORD", 0), "ORD");
    assert_eq!(LexivoApp::remove_char_at("WORD", 3), "WOR");
}

#[test]
fn update_status_transitions() {
    let mut app = LexivoApp::default();
    assert_eq!(app.update_status, UpdateStatus::Idle);

    // Manually setting status because check_for_updates is asynchronous
    app.update_status = UpdateStatus::Checking;
    assert_eq!(app.update_status, UpdateStatus::Checking);

    app.update_status = UpdateStatus::Available {
        version: "1.1.0".to_owned(),
        url: "https://link".to_owned(),
    };

    if let UpdateStatus::Available { version, .. } = &app.update_status {
        assert_eq!(version, "1.1.0");
    } else {
        panic!("Expected Available status");
    }
}

#[test]
fn revealed_indices_are_ignored_in_manual_input() {
    let mut app = LexivoApp {
        revealed_answer_indices: vec![1, 3],
        puzzles: vec![Puzzle {
            source: "ESRUN".to_owned(),
            hint: "Medical professional".to_owned(),
            answer: "NURSE".to_owned(),
        }],
        current_index: 0,
        ..LexivoApp::default()
    };
    // If answer is "NURSE" (len 5), max manual input should be 3

    assert_eq!(app.max_manual_input_len(), 3);

    app.user_input = "ABC".to_owned();
    app.sync_manual_input();
    assert_eq!(app.user_input, "ABC");

    app.user_input = "ABCD".to_owned();
    app.sync_manual_input();
    assert_eq!(app.user_input, "ABC");
}

#[test]
fn answer_slots_merges_manual_and_revealed() {
    let app = LexivoApp {
        puzzles: vec![Puzzle {
            source: "ESRUN".to_owned(),
            hint: "Medical professional".to_owned(),
            answer: "NURSE".to_owned(),
        }],
        current_index: 0,
        revealed_answer_indices: vec![0, 2], // 'N' and 'R'
        user_input: "US".to_owned(),
        ..LexivoApp::default()
    };

    let slots = app.answer_slots();
    // Expected: [Some('N'), Some('U'), Some('R'), Some('S'), None]
    assert_eq!(slots[0], Some('N'));
    assert_eq!(slots[1], Some('U'));
    assert_eq!(slots[2], Some('R'));
    assert_eq!(slots[3], Some('S'));
    assert_eq!(slots[4], None);
}

#[test]
fn composed_answer_requires_all_slots_filled() {
    let mut app = LexivoApp {
        puzzles: vec![Puzzle {
            source: "ESRUN".to_owned(),
            hint: "Medical professional".to_owned(),
            answer: "NURSE".to_owned(),
        }],
        current_index: 0,
        revealed_answer_indices: vec![0], // 'N'
        ..LexivoApp::default()
    };

    // Only 1 manual char, need 4 more
    app.user_input = "U".to_owned();
    assert_eq!(app.composed_answer_from_slots(), None);

    app.user_input = "URSE".to_owned();
    assert_eq!(app.composed_answer_from_slots(), Some("NURSE".to_owned()));
}

#[test]
fn hint_reveal_clears_overlapping_manual_input() {
    let mut app = LexivoApp {
        score: 50,
        puzzles: vec![Puzzle {
            source: "SOURCE".to_owned(),
            hint: "HINT".to_owned(),
            answer: "ABC".to_owned(),
        }],
        current_index: 0,
        user_input: "XY".to_owned(), // Manual input for slots 0 and 1
        ..LexivoApp::default()
    };

    // Force reveal of index 1
    while !app.revealed_answer_indices.contains(&1) {
        app.score = 50;
        app.reveal_hint_letter();
    }

    // If index 1 was revealed, user_input should have shrunk
    assert!(app.user_input.len() < 2);
}

#[test]
fn leaderboard_deduplicates_scores() {
    let mut app = LexivoApp {
        score: 100,
        ..LexivoApp::default()
    };
    app.record_score();
    app.score = 100;
    app.record_score();

    assert_eq!(app.leaderboard, vec![100]);
}

#[test]
fn back_navigation_confirmation_logic() {
    let mut app = LexivoApp {
        screen: Screen::Start,
        ..LexivoApp::default()
    };

    // Start screen -> Leaderboard (no confirm)
    app.go_to_screen(Screen::Leaderboard);
    app.request_back_navigation();
    assert_eq!(app.screen, Screen::Start);
    assert!(!app.show_back_confirm);

    // Game screen (active) -> Back (needs confirm)
    app.screen = Screen::Game;
    app.game_over = false;
    app.request_back_navigation();
    assert_eq!(app.screen, Screen::Game);
    assert!(app.show_back_confirm);

    app.confirm_back_navigation();
    assert_eq!(app.screen, Screen::Start);
    assert!(!app.show_back_confirm);
}

#[test]
fn confirm_reset_progress_clears_state() {
    let mut app = LexivoApp {
        best_score: 500,
        streak: 10,
        best_streak: 15,
        leaderboard: vec![500, 400],
        ..LexivoApp::default()
    };

    app.confirm_reset_progress();

    assert_eq!(app.best_score, 0);
    assert_eq!(app.streak, 0);
    assert_eq!(app.best_streak, 0);
    assert!(app.leaderboard.is_empty());
    assert_eq!(app.message, "Progress reset");
}

#[test]
fn message_toast_lifecycle() {
    let mut app = LexivoApp::default();
    app.set_message("Hello");
    assert_eq!(app.message, "Hello");
    assert!(app.message_timer > 0.0);

    app.update_timer(2.0);
    assert_eq!(app.message, "Hello");

    app.update_timer(1.1); // Total 3.1s
    assert_eq!(app.message, "");
}

#[test]
fn difficulty_reload_resets_state() {
    let mut app = LexivoApp {
        score: 100,
        current_difficulty: Difficulty::Easy,
        ..LexivoApp::default()
    };

    // Load medium
    app.load_difficulty(Difficulty::Medium)
        .expect("load difficulty");
    assert_eq!(app.current_difficulty, Difficulty::Medium);
    assert_eq!(app.current_index, 0);
    assert!(app.user_input.is_empty());
}

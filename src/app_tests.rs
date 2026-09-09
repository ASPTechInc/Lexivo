use super::*;

#[test]
fn score_round_trips_through_json() {
    let mut app = LexivoApp::default();
    app.score = 42;

    let json = serde_json::to_string(&app).expect("serialize app");
    let restored: LexivoApp = serde_json::from_str(&json).expect("deserialize app");

    assert_eq!(restored.score, 42);
}

#[test]
fn answer_validation_ignores_case_and_whitespace() {
    assert_eq!(normalise_answer("  nurse  "), "NURSE");
    assert_eq!(normalise_answer("nUrSe"), "NURSE");
}

#[test]
fn answer_validation_handles_empty_input_and_punctuation() {
    assert_eq!(normalise_answer(""), "");
    assert_eq!(normalise_answer("   "), "");
    assert_eq!(normalise_answer("nurse!"), "NURSE");
    assert_eq!(normalise_answer("n-u-r-s-e"), "NURSE");
}

#[test]
fn timer_expiration_marks_game_over() {
    let mut app = LexivoApp::default();
    app.time_left = 0.1;

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
        daily_seed_for_day(1_700_000_001)
    );
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
    assert_eq!(app.score, 10 + (app.time_left.ceil() as u32).max(1));
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
    let mut app = LexivoApp::default();
    app.score = 999;
    app.streak = 4;
    app.time_left = 1.0;
    app.set_mode(GameMode::QuickPlay);

    app.start_selected_mode();

    assert_eq!(app.score, 0);
    assert_eq!(app.streak, 0);
    assert_eq!(app.time_left, 20.0);
    assert!(!app.daily_challenge_active);
}

#[test]
fn daily_challenge_keeps_mode_state_and_count() {
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
    let mut app = LexivoApp::default();
    app.time_left = 0.0;

    app.update_timer(0.0);

    assert!(app.game_over);
}

#[test]
fn game_over_blocks_scoring() {
    let mut app = LexivoApp::default();
    app.game_over = true;
    app.user_input = "NURSE".to_string();
    app.score = 10;

    app.check_answer();

    assert_eq!(app.score, 10);
}

#[test]
fn next_puzzle_resets_timer() {
    let mut app = LexivoApp::default();
    app.time_left = 1.0;
    app.next_puzzle();

    assert_eq!(app.time_left, 20.0);
}

#[test]
fn leaderboard_includes_high_score_and_sorts_descending() {
    let mut app = LexivoApp::default();
    app.score = 30;
    app.record_score();
    app.score = 50;
    app.record_score();
    app.score = 40;
    app.record_score();

    assert_eq!(app.leaderboard, vec![50, 40, 30]);
}

#[test]
fn leaderboard_trims_to_top_five() {
    let mut app = LexivoApp::default();
    for value in [1, 2, 3, 4, 5, 6] {
        app.score = value;
        app.record_score();
    }

    assert_eq!(app.leaderboard.len(), 5);
    assert_eq!(app.leaderboard[0], 6);
    assert_eq!(app.leaderboard[4], 2);
}

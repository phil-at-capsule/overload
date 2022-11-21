#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

enum Muscle {
  Cardio,
  Neck,
  Traps,
  FrontDelt,
  SideDelt,
  RearDelt,
  Bicep,
  Tricep,
  Forearms,
  UpperBack,
  LowerBack,
  Pecs,
  Abs,
  Obliques,
  Glutes,
  Quads,
  Hamstrings,
  Calfs
}

struct ExerciseType {
  name: String,
  machine: bool,
  major: Vec<Muscle>,
  minor: Vec<Muscle>,
}

struct TimeDuration {
  time: u8,
}

struct RepRange {
  start: u8,
  end: u8,
}

struct RepDuration {
  sets: u8,
  rep_range: RepRange,
}

enum Duration {
  Time(TimeDuration),
  Rep(RepDuration),
}

struct Exercise {
  exercise: ExerciseType,
  duration: Duration,
}

struct Session {
  location: String,
  exercises: Vec<Exercise>
}

struct Period {
  sessions: Vec<Session>
}

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

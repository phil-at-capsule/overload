#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde_json;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
struct ExerciseType {
  name: String,
  machine: bool,
  major: Vec<Muscle>,
  minor: Vec<Muscle>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeDuration {
  time: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct RepRange {
  start: u8,
  end: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct RepDuration {
  sets: u8,
  rep_range: RepRange,
}

#[derive(Serialize, Deserialize, Debug)]
enum Duration {
  Time(TimeDuration),
  Sets(RepDuration),
}

#[derive(Serialize, Deserialize, Debug)]
struct Exercise {
  exercise: ExerciseType,
  duration: Duration,
}

#[derive(Serialize, Deserialize, Debug)]
struct Session {
  location: String,
  exercises: Vec<Exercise>
}

#[derive(Serialize, Deserialize, Debug)]
struct Period {
  sessions: Vec<Session>
}

#[tauri::command]
fn print() -> String {
  "print".to_string()
}

fn main() {
  let path = "../db/periods.json";
  let data = fs::read_to_string(path).expect("Unable to read file");
  let periods: Vec<Period> = serde_json::from_str(&data).expect("blah");
  println!("{periods:?}")
  // tauri::Builder::default()
  //     .invoke_handler(tauri::generate_handler![
  //       print
  //     ])
  //   .run(tauri::generate_context!())
  //   .expect("error while running tauri application");
}

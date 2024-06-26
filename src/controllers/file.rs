// #[cfg(test)]
// mod file_integration_should {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn save_dive_results_file() {
//         // Given
//         let dive_plan = "test_file_1.json";
//         let results = vec![DiveStage::default()];

//         // When
//         upsert_dive_results(dive_plan, &results);

//         // Then
//         assert!(fs::metadata(dive_plan).is_ok());
//         assert!(fs::metadata(dive_plan).unwrap().len() != 0);
//     }

//     #[test]
//     fn save_dive_planner_state_file() {
//         // Given
//         let dive_planner_state_file_name = "test_file_2.json";
//         let dive_planner = DivePlanner::default();

//         // When
//         upsert_dive_planner_state(dive_planner_state_file_name, &dive_planner);

//         // Then
//         assert!(fs::metadata(dive_planner_state_file_name).is_ok());
//         assert!(fs::metadata(dive_planner_state_file_name).unwrap().len() != 0);
//     }

//     #[test]
//     fn handle_loading_an_empty_dive_planner_state() {
//         // Given
//         let file_name = "non_existant_file.json";
//         let expected_dive_planner = DivePlanner::default();

//         // When
//         let dive_planner = read_dive_planner_state(file_name);

//         // Then
//         assert_eq!(expected_dive_planner, dive_planner);
//     }

//     #[test]
//     fn create_a_file_saving_and_loading_dive_planner_state() {
//         // Given
//         let file_name = "test_file_3.json";
//         let expected_dive_planner = DivePlanner::default();

//         // When
//         upsert_dive_planner_state(file_name, &expected_dive_planner);
//         let dive_planner = read_dive_planner_state(file_name);

//         // Then
//         assert_eq!(expected_dive_planner, dive_planner);
//     }
// }

use crate::models::piggy::Piggy;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

pub fn is_existing_file(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

pub fn create_default_piggy_configuration_file() {
    let piggy = Piggy::default();

    let mut file = File::create("piggy.json").expect("Can't create file.");
    let json =
        serde_json::ser::to_string_pretty(&piggy).expect("Can't parse application data to string");
    write!(file, "{}", json).expect("Can't update file with application data");
}

pub fn read_piggy_configuration_file(file_path: &str) -> Piggy {
    let contents = get_file_contents(file_path);

    if contents.is_empty() {
        return Piggy::default();
    }

    serde_json::from_str(&contents).expect("Can't parse file contents to application data")
}

fn get_file_contents(file_name: &str) -> String {
    let mut contents = String::new();

    if let Ok(mut file) = File::open(file_name) {
        file.read_to_string(&mut contents).expect("Can't read file");
    }

    contents
}

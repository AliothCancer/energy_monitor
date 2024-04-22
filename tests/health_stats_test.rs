
// Import the health_stats function from the main module
#[path = "../src/main.rs"]
mod main;

// Import the necessary modules
use main::health_stats;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    

    #[test]
    fn test_health_stats() {
        // Set up mock battery statistics
        let charge_full = "3000"; // Example values for charge_full and charge_full_design
        let charge_full_design = "3500"; // Example values for charge_full and charge_full_design

        // Create a temporary directory for the test
        let temp_dir = tempdir::TempDir::new("test_data").expect("Failed to create temporary directory");
        let data_dir = temp_dir.path().join("data");
        fs::create_dir(&data_dir).expect("Failed to create data directory");

        // Set up the data file path
        let file_path = data_dir.join("battery_stats.csv");

        // Call the health_stats function
        let result = health_stats();

        // Assert that the function executed without errors
        assert!(result.is_ok(), "health_stats function returned an error: {:?}", result);

        // Assert that the CSV file was created
        assert!(file_path.exists(), "CSV file was not created");

        // Read the contents of the CSV file
        let content = fs::read_to_string(&file_path).expect("Failed to read CSV file");

        // Assert that the CSV file contains the expected data
        assert!(content.contains("Date,Charge_Full,Charge_Full_Design,Battery_Health"));
        assert!(content.contains("Charge_Full,3500,3000,0.857"));

        // Clean up: delete the temporary directory
        temp_dir.close().expect("Failed to delete temporary directory");
    }
}

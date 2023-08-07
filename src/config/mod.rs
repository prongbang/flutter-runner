use serde::de::DeserializeOwned;
use serde_yaml;

pub fn read_yaml<T>(file_path: &str) -> T
    where
        T: DeserializeOwned,
{
    // Parse yml to struct
    let file = std::fs::File::open(file_path).expect("Could not open file.");
    let config: T = serde_yaml::from_reader(file).expect("Could not read values.");
    config
}
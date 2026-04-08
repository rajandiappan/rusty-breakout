use std::fs;
use std::path::Path;

pub fn load_high_score(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Ok(0);
    }

    let contents = fs::read_to_string(path)?;
    let trimmed = contents.trim();

    if trimmed.is_empty() {
        return Ok(0);
    }

    Ok(trimmed.parse::<u32>()?)
}

pub fn save_high_score(path: &str, score: u32) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(path, score.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir()
            .join("rusty_breakout_tests")
            .join(format!("{name}_{timestamp}.txt"))
    }

    #[test]
    fn test_missing_high_score_file_defaults_to_zero() {
        let path = unique_temp_path("missing_high_score");
        let result = load_high_score(path.to_str().expect("utf-8 temp path")).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_high_score_round_trip() {
        let path = unique_temp_path("high_score_round_trip");
        let path_str = path.to_str().expect("utf-8 temp path");

        save_high_score(path_str, 4242).unwrap();
        let loaded = load_high_score(path_str).unwrap();
        assert_eq!(loaded, 4242);

        let _ = fs::remove_file(path);
    }
}

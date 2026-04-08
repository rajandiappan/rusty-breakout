use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementId {
    // Skill achievements
    Sharpshooter,    // Destroy 100 bricks in a level
    RapidFire,       // Complete a level in under 60 seconds
    PerfectClear,    // Complete all 5 levels without losing a life
    Speedrunner,     // Beat game in under 5 minutes
    MultiBallMaster, // Keep 3 balls active for 30 consecutive seconds

    // Collection achievements
    PowerUpHoarder, // Collect 5 power-ups in one level
    LuckyBreak,     // Trigger 3 power-ups in 5 seconds
    TimeBender,     // Use slow-time power-up 50 times total

    // Exploration achievements
    ThemeCollector,   // Unlock all 5 color themes
    HardcoreChampion, // Complete hard difficulty
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: AchievementId,
    pub name: String,
    pub description: String,
    pub progress: u32,
    pub target: u32,
    pub unlocked: bool,
}

impl Achievement {
    pub fn new(id: AchievementId) -> Self {
        match id {
            AchievementId::Sharpshooter => Achievement {
                id,
                name: "Sharpshooter".to_string(),
                description: "Destroy 100 bricks in a single level".to_string(),
                progress: 0,
                target: 100,
                unlocked: false,
            },
            AchievementId::RapidFire => Achievement {
                id,
                name: "Rapid Fire".to_string(),
                description: "Complete a level in under 60 seconds".to_string(),
                progress: 0,
                target: 1,
                unlocked: false,
            },
            AchievementId::PerfectClear => Achievement {
                id,
                name: "Perfect Clear".to_string(),
                description: "Complete all 5 levels without losing a life".to_string(),
                progress: 0,
                target: 1,
                unlocked: false,
            },
            AchievementId::Speedrunner => Achievement {
                id,
                name: "Speedrunner".to_string(),
                description: "Beat the game in under 5 minutes".to_string(),
                progress: 0,
                target: 1,
                unlocked: false,
            },
            AchievementId::MultiBallMaster => Achievement {
                id,
                name: "Multi-Ball Master".to_string(),
                description: "Keep 3 balls active for 30 consecutive seconds".to_string(),
                progress: 0,
                target: 30,
                unlocked: false,
            },
            AchievementId::PowerUpHoarder => Achievement {
                id,
                name: "Power-Up Hoarder".to_string(),
                description: "Collect 5 power-ups in one level".to_string(),
                progress: 0,
                target: 5,
                unlocked: false,
            },
            AchievementId::LuckyBreak => Achievement {
                id,
                name: "Lucky Break".to_string(),
                description: "Trigger 3 power-ups in 5 seconds".to_string(),
                progress: 0,
                target: 1,
                unlocked: false,
            },
            AchievementId::TimeBender => Achievement {
                id,
                name: "Time Bender".to_string(),
                description: "Use slow-time power-up 50 times total".to_string(),
                progress: 0,
                target: 50,
                unlocked: false,
            },
            AchievementId::ThemeCollector => Achievement {
                id,
                name: "Theme Collector".to_string(),
                description: "Unlock all 5 color themes".to_string(),
                progress: 0,
                target: 5,
                unlocked: false,
            },
            AchievementId::HardcoreChampion => Achievement {
                id,
                name: "Hardcore Champion".to_string(),
                description: "Complete hard difficulty".to_string(),
                progress: 0,
                target: 1,
                unlocked: false,
            },
        }
    }

    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    pub fn unlock(&mut self) {
        if !self.unlocked {
            self.unlocked = true;
            self.progress = self.target;
        }
    }

    #[allow(dead_code)]
    pub fn progress_percentage(&self) -> f32 {
        if self.target == 0 {
            0.0
        } else {
            (self.progress.min(self.target) as f32 / self.target as f32) * 100.0
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementManager {
    pub achievements: HashMap<AchievementId, Achievement>,
}

#[allow(dead_code)]
impl AchievementManager {
    pub fn new() -> Self {
        let mut achievements = HashMap::new();
        for id in all_achievement_ids() {
            achievements.insert(id, Achievement::new(id));
        }
        AchievementManager { achievements }
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
            let contents = fs::read_to_string(path)?;
            let achievements: HashMap<AchievementId, Achievement> =
                serde_json::from_str(&contents)?;
            Ok(AchievementManager { achievements })
        } else {
            Ok(AchievementManager::new())
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let json = serde_json::to_string_pretty(&self.achievements)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn unlock(&mut self, id: AchievementId) {
        if let Some(achievement) = self.achievements.get_mut(&id) {
            achievement.unlock();
        }
    }

    pub fn is_unlocked(&self, id: AchievementId) -> bool {
        self.achievements
            .get(&id)
            .map(|a| a.is_unlocked())
            .unwrap_or(false)
    }

    pub fn increment_progress(&mut self, id: AchievementId, amount: u32) {
        if let Some(achievement) = self.achievements.get_mut(&id) {
            if !achievement.is_unlocked() {
                achievement.progress = (achievement.progress + amount).min(achievement.target);
                if achievement.progress >= achievement.target {
                    achievement.unlock();
                }
            }
        }
    }

    pub fn get_unlocked_count(&self) -> usize {
        self.achievements
            .values()
            .filter(|a| a.is_unlocked())
            .count()
    }

    pub fn get_total_count(&self) -> usize {
        self.achievements.len()
    }

    pub fn get_all_achievements(&self) -> Vec<Achievement> {
        let mut achievements: Vec<_> = self.achievements.values().cloned().collect();
        achievements.sort_by_key(|a| a.name.clone());
        achievements
    }
}

fn all_achievement_ids() -> Vec<AchievementId> {
    vec![
        AchievementId::Sharpshooter,
        AchievementId::RapidFire,
        AchievementId::PerfectClear,
        AchievementId::Speedrunner,
        AchievementId::MultiBallMaster,
        AchievementId::PowerUpHoarder,
        AchievementId::LuckyBreak,
        AchievementId::TimeBender,
        AchievementId::ThemeCollector,
        AchievementId::HardcoreChampion,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(name: &str) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir()
            .join("rusty_breakout_tests")
            .join(format!("{name}_{timestamp}.json"))
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn test_achievement_creation() {
        let ach = Achievement::new(AchievementId::Sharpshooter);
        assert_eq!(ach.name, "Sharpshooter");
        assert_eq!(ach.target, 100);
        assert!(!ach.is_unlocked());
    }

    #[test]
    fn test_achievement_unlock() {
        let mut ach = Achievement::new(AchievementId::RapidFire);
        assert!(!ach.is_unlocked());
        ach.unlock();
        assert!(ach.is_unlocked());
    }

    #[test]
    fn test_achievement_manager() {
        let mut manager = AchievementManager::new();
        assert_eq!(manager.get_unlocked_count(), 0);
        manager.unlock(AchievementId::RapidFire);
        assert_eq!(manager.get_unlocked_count(), 1);
    }

    #[test]
    fn test_progress_percentage() {
        let mut ach = Achievement::new(AchievementId::Sharpshooter);
        ach.progress = 50;
        assert!(ach.progress_percentage() > 49.0 && ach.progress_percentage() < 51.0);
    }

    #[test]
    fn test_achievement_manager_save_load_round_trip() {
        let path = unique_temp_path("achievement_round_trip");
        let mut manager = AchievementManager::new();
        manager.increment_progress(AchievementId::TimeBender, 7);
        manager.unlock(AchievementId::RapidFire);

        manager.save_to_file(&path).unwrap();
        let loaded = AchievementManager::load_from_file(&path).unwrap();

        assert_eq!(
            loaded
                .achievements
                .get(&AchievementId::TimeBender)
                .expect("time bender exists")
                .progress,
            7
        );
        assert!(
            loaded.is_unlocked(AchievementId::RapidFire),
            "rapid fire should remain unlocked after reload"
        );

        let _ = std::fs::remove_file(path);
    }
}

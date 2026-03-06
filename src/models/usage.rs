use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum UsageLevel {
    Never,
    Rare,
    Occasional,
    Heavy,
}

impl fmt::Display for UsageLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UsageLevel::Heavy => write!(f, "Beaucoup"),
            UsageLevel::Occasional => write!(f, "De temps en temps"),
            UsageLevel::Rare => write!(f, "Rarement"),
            UsageLevel::Never => write!(f, "Jamais"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Usage {
    id: Option<i64>,
    subscription_id: Option<i64>,
    member_id: Option<i64>,
    component_name: String,
    usage_level: UsageLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usate_level_display() {
        assert_eq!(UsageLevel::Heavy.to_string(), "Beaucoup");
        assert_eq!(UsageLevel::Occasional.to_string(), "De temps en temps");
        assert_eq!(UsageLevel::Rare.to_string(), "Rarement");
        assert_eq!(UsageLevel::Never.to_string(), "Jamais");
    }

    #[test]
    fn test_usage_level_ordering() {
        assert!(UsageLevel::Heavy > UsageLevel::Occasional);
        assert!(UsageLevel::Occasional > UsageLevel::Rare);
        assert!(UsageLevel::Rare > UsageLevel::Never);
    }

    #[test]
    fn test_usage() {
        let usage = Usage {
            id: Some(1),
            subscription_id: Some(2),
            member_id: Some(3),
            component_name: "test".to_string(),
            usage_level: UsageLevel::Heavy,
        };

        assert_eq!(usage.id, Some(1));
        assert_eq!(usage.subscription_id, Some(2));
        assert_eq!(usage.member_id, Some(3));
        assert_eq!(usage.component_name, "test");
        assert_eq!(usage.usage_level, UsageLevel::Heavy);
    }
}

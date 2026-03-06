#[derive(Debug, Clone, PartialEq, Eq)]
enum MigrationEffort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq)]
struct Alternative {
    id: Option<i32>,
    name: String,
    covers_need: String,
    price_monthly: f64,
    has_family_plan: bool,
    family_price_monthly: Option<f64>,
    european: bool,
    open_source: bool,
    self_hostable: bool,
    data_location: String,
    migration_effort: MigrationEffort,
    url: String,
    notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternative() {
        let alternative = Alternative {
            id: None,
            name: "Example".to_string(),
            covers_need: "Example".to_string(),
            price_monthly: 10.0,
            has_family_plan: true,
            family_price_monthly: Some(8.0),
            european: true,
            open_source: true,
            self_hostable: true,
            data_location: "Example".to_string(),
            migration_effort: MigrationEffort::Low,
            url: "Example".to_string(),
            notes: None,
        };

        assert_eq!(alternative.id, None);
        assert_eq!(alternative.name, "Example");
        assert_eq!(alternative.covers_need, "Example");
        assert_eq!(alternative.price_monthly, 10.0);
        assert_eq!(alternative.has_family_plan, true);
        assert_eq!(alternative.family_price_monthly, Some(8.0));
        assert_eq!(alternative.european, true);
        assert_eq!(alternative.open_source, true);
        assert_eq!(alternative.self_hostable, true);
        assert_eq!(alternative.data_location, "Example");
        assert_eq!(alternative.migration_effort, MigrationEffort::Low);
        assert_eq!(alternative.url, "Example");
        assert_eq!(alternative.notes, None);
    }
}

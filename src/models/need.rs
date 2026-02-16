#[derive(Debug, Clone, PartialEq, Eq)]
struct Need {
    id: Option<i64>,
    name: String,
    family: String,
    is_essential: bool,
}

impl Need {
    fn builder(name: String, family: String, is_essential: bool) -> NeedBuilder {
        NeedBuilder::new(name, family, is_essential)
    }
}

struct NeedBuilder {
    id: Option<i64>,
    name: String,
    family: String,
    is_essential: bool,
}

impl NeedBuilder {
    fn new(name: String, family: String, is_essential: bool) -> Self {
        Self {
            id: None,
            name,
            family,
            is_essential,
        }
    }

    fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    fn build(self) -> Need {
        Need {
            id: self.id,
            name: self.name,
            family: self.family,
            is_essential: self.is_essential,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_need(is_essential: bool) -> Need {
        Need::builder(
            "Test Need".to_string(),
            "Test Family".to_string(),
            is_essential,
        )
        .build()
    }

    #[test]
    fn test_need() {
        let need = make_test_need(true);

        assert_eq!(need.id, None);
        assert_eq!(need.name, "Test Need");
        assert_eq!(need.family, "Test Family");
        assert_eq!(need.is_essential, true);
    }

    #[test]
    fn test_builder_with_optional_fields() {
        let need = Need::builder("Test Need".to_string(), "Test Family".to_string(), true)
            .with_id(0)
            .build();

        assert_eq!(need.name, "Test Need");
        assert_eq!(need.family, "Test Family");
        assert!(need.is_essential);
        assert_eq!(need.id, Some(0));
    }
}

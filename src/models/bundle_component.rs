#[derive(Debug, Clone)]
struct BundleComponent {
    id: Option<i64>,
    subscription_id: i64,
    name: String,
    need_id: i64,
    individual_price: Option<f64>,
    allocated_cost: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bundle_component() {
        let component = BundleComponent {
            id: Some(1),
            subscription_id: 42,
            name: "Apple Music".to_string(),
            need_id: 10,
            individual_price: Some(10.99),
            allocated_cost: Some(3.66),
        };

        assert_eq!(component.name, "Apple Music");
        assert_eq!(component.subscription_id, 42);
        assert_eq!(component.need_id, 10);
        assert_eq!(component.individual_price, Some(10.99));
        assert_eq!(component.allocated_cost, Some(3.66));
    }

    #[test]
    fn test_allocated_cost_optional() {
        let component = BundleComponent {
            id: None,
            subscription_id: 42,
            name: "Apple TV+".to_string(),
            need_id: 11,
            individual_price: Some(9.99),
            allocated_cost: None,
        };

        assert!(component.allocated_cost.is_none());
    }
}

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
    fn test_bundle_component() {}
}

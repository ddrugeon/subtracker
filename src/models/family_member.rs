use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyMember {
    id: Option<i64>,
    name: String,
    created_at: NaiveDate,
}

impl FamilyMember {
    pub fn new(name: String) -> FamilyMember {
        FamilyMember {
            id: None,
            name,
            created_at: chrono::Utc::now().date_naive(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_family_member(created_at: NaiveDate) -> FamilyMember {
        FamilyMember {
            id: None,
            name: "John Doe".to_string(),
            created_at,
        }
    }

    #[test]
    fn test_family_member() {
        let family_member = make_test_family_member(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());

        assert!(family_member.id.is_none());
        assert_eq!(family_member.name, "John Doe");
        assert_eq!(
            family_member.created_at,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
    }
}

use std::{fmt, str::FromStr};

use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frequency {
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFrequencyError;

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Frequency::Monthly => write!(f, "Mensuel"),
            Frequency::Quarterly => write!(f, "Trimestriel"),
            Frequency::Yearly => write!(f, "Annuel"),
        }
    }
}

impl FromStr for Frequency {
    type Err = ParseFrequencyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Mensuel" => Ok(Frequency::Monthly),
            "Trimestriel" => Ok(Frequency::Quarterly),
            "Annuel" => Ok(Frequency::Yearly),
            _ => return Err(ParseFrequencyError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Archived,
}

impl fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubscriptionStatus::Active => write!(f, "Active"),
            SubscriptionStatus::Archived => write!(f, "Archivée"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSubscriptionStatusError;

impl FromStr for SubscriptionStatus {
    type Err = ParseSubscriptionStatusError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(SubscriptionStatus::Active),
            "Archivée" => Ok(SubscriptionStatus::Archived),
            _ => Err(ParseSubscriptionStatusError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentSource {
    Apple,
    BankTransfer,
    CreditCard,
    DirectDebit,
    PayPal,
    Other,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePaymentSourceError;

impl fmt::Display for PaymentSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentSource::Apple => write!(f, "Apple"),
            PaymentSource::BankTransfer => write!(f, "BankTransfer"),
            PaymentSource::CreditCard => write!(f, "CreditCard"),
            PaymentSource::DirectDebit => write!(f, "DirectDebit"),
            PaymentSource::PayPal => write!(f, "PayPal"),
            PaymentSource::Other => write!(f, "Other"),
        }
    }
}

impl FromStr for PaymentSource {
    type Err = ParsePaymentSourceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Apple" => Ok(PaymentSource::Apple),
            "BankTransfer" => Ok(PaymentSource::BankTransfer),
            "CreditCard" => Ok(PaymentSource::CreditCard),
            "DirectDebit" => Ok(PaymentSource::DirectDebit),
            "PayPal" => Ok(PaymentSource::PayPal),
            "Other" => Ok(PaymentSource::Other),
            _ => return Err(ParsePaymentSourceError),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subscription {
    pub id: Option<u64>,
    pub name: String,
    pub provider: Option<String>,
    pub amount: f64,
    pub frequency: Frequency,
    pub is_bundle: bool,
    pub is_family_plan: bool,
    pub payment_source: PaymentSource,
    pub start_date: NaiveDate,
    pub renewal_date: Option<NaiveDate>,
    pub status: SubscriptionStatus,
    pub notes: Option<String>,
}

impl Subscription {
    pub fn builder(
        name: String,
        amount: f64,
        frequency: Frequency,
        start_date: NaiveDate,
    ) -> SubscriptionBuilder {
        SubscriptionBuilder::new(name, amount, frequency, start_date)
    }

    pub fn monthly_cost(&self) -> f64 {
        match self.frequency {
            Frequency::Monthly => self.amount,
            Frequency::Quarterly => self.amount / 3.0,
            Frequency::Yearly => self.amount / 12.0,
        }
    }
}

pub struct SubscriptionBuilder {
    id: Option<u64>,
    name: String,
    amount: f64,
    frequency: Frequency,
    start_date: NaiveDate,
    provider: Option<String>,
    is_bundle: bool,
    is_family_plan: bool,
    payment_source: PaymentSource,
    renewal_date: Option<NaiveDate>,
    status: SubscriptionStatus,
    notes: Option<String>,
}

impl SubscriptionBuilder {
    fn new(name: String, amount: f64, frequency: Frequency, start_date: NaiveDate) -> Self {
        SubscriptionBuilder {
            name,
            amount,
            frequency,
            start_date,
            id: None,
            provider: None,
            is_bundle: false,
            is_family_plan: false,
            payment_source: PaymentSource::Other,
            renewal_date: None,
            status: SubscriptionStatus::Active,
            notes: None,
        }
    }
    pub fn with_id(mut self, id: Option<u64>) -> Self {
        self.id = id;
        self
    }

    pub fn with_provider(mut self, provider: String) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn with_bundle(mut self, is_bundle: bool) -> Self {
        self.is_bundle = is_bundle;
        self
    }

    pub fn with_family_plan(mut self, is_family_plan: bool) -> Self {
        self.is_family_plan = is_family_plan;
        self
    }

    pub fn with_payment_source(mut self, payment_source: PaymentSource) -> Self {
        self.payment_source = payment_source;
        self
    }

    pub fn with_renewal_date(mut self, renewal_date: NaiveDate) -> Self {
        self.renewal_date = Some(renewal_date);
        self
    }

    pub fn with_status(mut self, status: SubscriptionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }

    pub fn build(self) -> Subscription {
        Subscription {
            id: self.id,
            name: self.name,
            provider: self.provider,
            amount: self.amount,
            frequency: self.frequency,
            is_bundle: self.is_bundle,
            is_family_plan: self.is_family_plan,
            payment_source: self.payment_source,
            start_date: self.start_date,
            renewal_date: self.renewal_date,
            status: self.status,
            notes: self.notes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper : crée un abonnement par défaut pour les tests
    fn make_test_subscription(amount: f64, frequency: Frequency) -> Subscription {
        Subscription::builder(
            "Test".to_string(),
            amount,
            frequency,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        )
        .build()
    }

    #[test]
    fn test_create_subscription() {
        let subscription = make_test_subscription(100.0, Frequency::Monthly);
        assert_eq!(subscription.id, None);
        assert_eq!(subscription.name, "Test");
        assert_eq!(subscription.amount, 100.0);
        assert_eq!(subscription.frequency, Frequency::Monthly);
        assert_eq!(subscription.monthly_cost(), 100.0);
        assert_eq!(subscription.is_bundle, false);
        assert_eq!(subscription.is_family_plan, false);
        assert_eq!(subscription.payment_source, PaymentSource::Other);
        assert_eq!(
            subscription.start_date,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
        assert_eq!(subscription.status, SubscriptionStatus::Active);
    }

    #[test]
    fn test_subscription_optional_fields() {
        // Créer une Subscription avec notes = None, renewal_date = None
        // Vérifier que les champs optionnels sont bien None
        let subscription = make_test_subscription(100.0, Frequency::Monthly);

        assert!(subscription.id.is_none());
        assert!(subscription.provider.is_none());
        assert!(subscription.renewal_date.is_none());
    }

    #[test]
    fn test_monthly_cost_from_monthly() {
        // Un abonnement à 10.99€/mois → monthly_cost = 10.99
        //
        let subscription = make_test_subscription(10.99, Frequency::Monthly);

        assert_eq!(subscription.monthly_cost(), 10.99);
    }

    #[test]
    fn test_monthly_cost_from_yearly() {
        // Un abonnement à 99.00€/an → monthly_cost = 8.25
        let subscription = make_test_subscription(99.0, Frequency::Yearly);
        assert_eq!(subscription.monthly_cost(), 8.25);
    }

    #[test]
    fn test_monthly_cost_from_quarterly() {
        // Un abonnement à 30.00€/trimestre → monthly_cost = 10.00
        let subscription = make_test_subscription(30.0, Frequency::Quarterly);
        assert_eq!(subscription.monthly_cost(), 10.0);
    }

    #[test]
    fn test_frequency_display() {
        // Frequency::Monthly s'affiche "Mensuel"
        // Frequency::Yearly s'affiche "Annuel"
        // Frequency::Quarterly s'affiche "Trimestriel"
        let monthly = Frequency::Monthly;
        let yearly = Frequency::Yearly;
        let quarterly = Frequency::Quarterly;

        assert_eq!(monthly.to_string(), "Mensuel");
        assert_eq!(yearly.to_string(), "Annuel");
        assert_eq!(quarterly.to_string(), "Trimestriel");
    }

    #[test]
    fn test_subscription_status_default() {
        // Un nouvel abonnement a le statut Active par défaut
        let subscription = Subscription::builder(
            "Test Subscription".to_string(),
            40.0,
            Frequency::Quarterly,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        )
        .build();

        assert_eq!(subscription.status, SubscriptionStatus::Active);
    }

    #[test]
    fn test_builder_with_optional_fields() {
        let sub = Subscription::builder(
            "Apple One".to_string(),
            34.99,
            Frequency::Monthly,
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        )
        .with_id(Some(0))
        .with_provider("Apple".to_string())
        .with_bundle(true)
        .with_family_plan(true)
        .with_payment_source(PaymentSource::Apple)
        .with_notes("Bundle Premium".to_string())
        .build();

        assert_eq!(sub.id, Some(0));
        assert_eq!(sub.provider, Some("Apple".to_string()));
        assert!(sub.is_bundle);
        assert!(sub.is_family_plan);
        assert_eq!(sub.payment_source, PaymentSource::Apple);
        assert_eq!(sub.notes, Some("Bundle Premium".to_string()));
    }
}

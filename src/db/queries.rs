use anyhow::Result;
use rusqlite::{Connection, Row};

use crate::models::subscription::{self, Frequency, PaymentSource, Subscription};
use chrono::NaiveDate;

fn extract_subscription_from_row(row: &Row) -> rusqlite::Result<Subscription> {
    let id: u64 = row.get::<_, i64>(0)? as u64;
    let name: String = row.get(1)?;
    let provider: Option<String> = row.get(2)?;
    let amount: f64 = row.get(3)?;
    let frequency_str: String = row.get(4)?;
    let is_bundle: bool = row.get(5)?;
    let is_family_plan: bool = row.get(6)?;
    let payment_source_str: String = row.get(7)?;
    let start_date_str: String = row.get(8)?;
    let renewal_date_str: Option<String> = row.get(9)?;
    let notes: Option<String> = row.get(10)?;

    let frequency = frequency_str.parse::<Frequency>().map_err(|_| {
        rusqlite::Error::InvalidColumnType(3, "frequency".into(), rusqlite::types::Type::Text)
    })?;

    let start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d").map_err(|_| {
        rusqlite::Error::InvalidColumnType(7, "start_date".into(), rusqlite::types::Type::Text)
    })?;

    let renewal_date = renewal_date_str
        .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| {
            rusqlite::Error::InvalidColumnType(
                8,
                "renewal_date".into(),
                rusqlite::types::Type::Text,
            )
        })?;

    let payment_source = payment_source_str.parse::<PaymentSource>().map_err(|_| {
        rusqlite::Error::InvalidColumnType(6, "payment_source".into(), rusqlite::types::Type::Text)
    })?;

    let mut builder = Subscription::builder(name, amount, frequency, start_date)
        .with_id(Some(id))
        .with_bundle(is_bundle)
        .with_family_plan(is_family_plan)
        .with_payment_source(payment_source);

    if let Some(p) = provider {
        builder = builder.with_provider(p);
    }
    if let Some(d) = renewal_date {
        builder = builder.with_renewal_date(d);
    }
    if let Some(n) = notes {
        builder = builder.with_notes(n);
    }

    Ok(builder.build())
}

fn insert_subscription(conn: &Connection, subscription: subscription::Subscription) -> Result<i64> {
    conn.execute(
        "INSERT INTO subscriptions(
            name, provider, amount, frequency, monthly_cost,
            is_bundle, is_family_plan, payment_source,
            start_date, renewal_date, status, notes
        ) VALUES (
            :name, :provider, :amount, :frequency, :monthly_cost,
            :is_bundle, :is_family_plan, :payment_source,
            :start_date, :renewal_date, :status, :notes
        )",
        rusqlite::named_params! {
            ":name": subscription.name,
            ":provider": subscription.provider,
            ":amount": subscription.amount,
            ":frequency": subscription.frequency.to_string(),
            ":monthly_cost": subscription.monthly_cost(),
            ":is_bundle": subscription.is_bundle,
            ":is_family_plan": subscription.is_family_plan,
            ":payment_source": subscription.payment_source.to_string(),
            ":start_date": subscription.start_date.to_string(),
            ":renewal_date": subscription.renewal_date.map(|d| d.to_string()),
            ":status": subscription.status.to_string(),
            ":notes": subscription.notes,
        },
    )?;
    Ok(conn.last_insert_rowid())
}

fn list_subscriptions(conn: &Connection) -> Result<Vec<subscription::Subscription>> {
    let mut stmt = conn.prepare(
        "SELECT id,
                name,
                provider,
                amount,
                frequency,
                is_bundle,
                is_family_plan,
                payment_source,
                start_date,
                renewal_date,
                notes
         FROM subscriptions",
    )?;

    let rows = stmt.query_map([], extract_subscription_from_row)?;

    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

fn get_subscription(conn: &Connection, id: i64) -> Result<Option<subscription::Subscription>> {
    let result = conn.query_row(
        "SELECT id,
                name,
                provider,
                amount,
                frequency,
                is_bundle,
                is_family_plan,
                payment_source,
                start_date,
                renewal_date,
                notes
         FROM subscriptions
         WHERE id = ?1",
        [id],
        extract_subscription_from_row,
    );

    match result {
        Ok(sub) => Ok(Some(sub)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn update_subscription(conn: &Connection, subscription: subscription::Subscription) -> Result<()> {
    todo!()
}

fn delete_subscription(conn: &Connection, id: i64) -> Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migration::run_migrations;
    use crate::models::subscription::Frequency;
    use crate::models::subscription::PaymentSource;
    use crate::models::subscription::Subscription;
    use chrono::NaiveDate;
    use rusqlite::Connection;

    /// Helper : crée une base en mémoire avec les migrations appliquées
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();

        conn
    }

    // --- INSERT ---

    #[test]
    fn test_insert_subscription_returns_id() {
        let conn = setup_test_db();

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

        let result = insert_subscription(&conn, sub).unwrap();
        assert!(result > 0);
    }

    #[test]
    fn test_insert_subscription_increments_id() {
        let conn = setup_test_db();

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

        let sub1_id = insert_subscription(&conn, sub.clone()).unwrap();
        let sub2_id = insert_subscription(&conn, sub).unwrap();
        assert_eq!(sub1_id + 1, sub2_id);
    }

    #[test]
    fn test_insert_subscription_with_optional_fields_none() {
        // Insérer un abonnement sans notes, sans renewal_date
        let conn = setup_test_db();

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
        .build();

        let sub1_id = insert_subscription(&conn, sub.clone()).unwrap();
        assert!(sub1_id > 0);
    }

    // --- SELECT ---

    #[test]
    fn test_list_subscriptions_empty() {
        let conn = setup_test_db();

        let subscriptions = list_subscriptions(&conn).unwrap();
        assert_eq!(subscriptions.len(), 0)
    }

    #[test]
    fn test_list_subscriptions_returns_all() {
        // Insérer 3 abonnements
        // list_subscriptions retourne un Vec de taille 3
        let conn = setup_test_db();

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

        let sub1_id = insert_subscription(&conn, sub.clone()).unwrap();
        let sub2_id = insert_subscription(&conn, sub.clone()).unwrap();
        let sub3_id = insert_subscription(&conn, sub.clone()).unwrap();

        let subscriptions = list_subscriptions(&conn).unwrap();
        assert_eq!(subscriptions.len(), 3)
    }

    #[test]
    fn test_get_subscription_by_id() {
        let conn = setup_test_db();

        let sub = Subscription::builder(
            "Netflix".to_string(),
            19.99,
            Frequency::Monthly,
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        )
        .build();

        let sub_id = insert_subscription(&conn, sub).unwrap();

        let returned_subscription = get_subscription(&conn, sub_id).unwrap().unwrap();
        assert_eq!(returned_subscription.name, "Netflix");
        assert_eq!(returned_subscription.amount, 19.99);
    }

    #[test]
    fn test_get_subscription_not_found() {
        // get_subscription(999) retourne None
        let conn = setup_test_db();

        let subscription = get_subscription(&conn, 999).unwrap();
        assert_eq!(subscription, None);
    }

    // --- UPDATE ---

    #[test]
    fn test_update_subscription_changes_name() {
        // Insérer "Netflix Standard"
        // Modifier le nom en "Netflix Premium"
        // get_subscription → name == "Netflix Premium"
    }

    #[test]
    fn test_update_subscription_changes_amount() {
        // Insérer un abonnement à 13.49€
        // Modifier le montant à 19.99€
        // Vérifier que le montant ET le monthly_cost sont mis à jour
    }

    // --- DELETE ---

    #[test]
    fn test_delete_subscription() {
        // Insérer un abonnement
        // Le supprimer
        // get_subscription → None
    }

    #[test]
    fn test_delete_subscription_not_in_list() {
        // Insérer 2 abonnements
        // Supprimer le premier
        // list_subscriptions retourne 1 seul abonnement (le deuxième)
    }

    // --- CAS LIMITES ---

    #[test]
    fn test_insert_subscription_special_characters() {
        // Insérer un abonnement avec un nom contenant des apostrophes
        // ex: "L'Équipe" — vérifier que ça ne casse pas le SQL
    }

    #[test]
    fn test_monthly_cost_calculated_correctly() {
        // Insérer un abonnement annuel à 99€
        // Vérifier que monthly_cost en base = 8.25
    }
}

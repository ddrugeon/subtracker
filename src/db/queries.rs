use anyhow::Result;
use rusqlite::Connection;

use crate::models::subscription;

fn insert_subscription(conn: Connection, subscription: subscription::Subscription) -> Result<i64> {
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

fn list_subscriptions(conn: Connection) -> Result<Vec<subscription::Subscription>> {
    todo!()
}

fn get_subscription(conn: Connection, id: i64) -> Result<Option<subscription::Subscription>> {
    todo!()
}

fn update_subscription(conn: Connection, subscription: subscription::Subscription) -> Result<()> {
    todo!()
}

fn delete_subscription(conn: Connection, id: i64) -> Result<()> {
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
        // Insérer un abonnement
        // Vérifier que l'id retourné est > 0
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

        let result = insert_subscription(conn, sub).unwrap();
        assert!(result > 0);
    }

    #[test]
    fn test_insert_subscription_increments_id() {
        // Insérer deux abonnements
        // Vérifier que le deuxième id = premier id + 1
    }

    #[test]
    fn test_insert_subscription_with_optional_fields_none() {
        // Insérer un abonnement sans notes, sans renewal_date
        // Vérifier que l'insertion ne crashe pas
    }

    // --- SELECT ---

    #[test]
    fn test_list_subscriptions_empty() {
        // Base vide → retourne un Vec vide
    }

    #[test]
    fn test_list_subscriptions_returns_all() {
        // Insérer 3 abonnements
        // list_subscriptions retourne un Vec de taille 3
    }

    #[test]
    fn test_get_subscription_by_id() {
        // Insérer un abonnement "Netflix" à 19.99€
        // get_subscription(id) retourne Some(subscription)
        // Vérifier que name == "Netflix" et amount == 19.99
    }

    #[test]
    fn test_get_subscription_not_found() {
        // get_subscription(999) retourne None
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

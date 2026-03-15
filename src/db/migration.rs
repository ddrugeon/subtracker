use std::path::Path;

use anyhow::Result;
use rusqlite::Connection;

const CREATE_FAMILY_MEMBERS: &str = "
      CREATE TABLE IF NOT EXISTS family_members (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL,
          created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      );
  ";
const CREATE_NEEDS: &str = "
      CREATE TABLE IF NOT EXISTS needs (
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         name TEXT NOT NULL,
         family TEXT NOT NULL,
         essential BOOLEAN DEFAULT FALSE,
         created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      );
";

const CREATE_SUBSCRIPTIONS: &str = "CREATE TABLE IF NOT EXISTS subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider TEXT,
    amount REAL NOT NULL,
    frequency TEXT NOT NULL,
    monthly_cost REAL NOT NULL,
    is_bundle BOOLEAN DEFAULT FALSE,
    is_family_plan BOOLEAN DEFAULT FALSE,
    payment_source TEXT,
    start_date DATE,
    renewal_date DATE,
    status TEXT DEFAULT 'active',
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
";

const CREATE_BUNDLE_COMPONENTS: &str = "CREATE TABLE IF NOT EXISTS bundle_components (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    name TEXT NOT NULL,
    need_id INTEGER NOT NULL REFERENCES needs(id),
    individual_price REAL,
    allocated_cost REAL,
    UNIQUE(subscription_id, name)
);
";

const CREATE_SUBSCRIPTION_NEEDS: &str = "CREATE TABLE IF NOT EXISTS subscription_needs (
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    need_id INTEGER NOT NULL REFERENCES needs(id),
    PRIMARY KEY (subscription_id, need_id)
);
";

const CREATE_USAGE_RATINGS: &str = "CREATE TABLE IF NOT EXISTS usage_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    component_name TEXT,
    member_id INTEGER NOT NULL REFERENCES family_members(id),
    rating TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(subscription_id, component_name, member_id)
);
";

const CREATE_PRICE_HISTORY: &str = "CREATE TABLE IF NOT EXISTS price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    amount REAL NOT NULL,
    frequency TEXT NOT NULL,
    recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
";

const MIGRATIONS: &[&str] = &[
    CREATE_FAMILY_MEMBERS,
    CREATE_NEEDS,
    CREATE_SUBSCRIPTIONS,
    CREATE_BUNDLE_COMPONENTS,
    CREATE_SUBSCRIPTION_NEEDS,
    CREATE_USAGE_RATINGS,
    CREATE_PRICE_HISTORY,
];

/// Ouvre (ou crée) la base SQLite au chemin donné.
/// Crée le répertoire parent si nécessaire.
pub fn open_database(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(Connection::open(path)?)
}

/// Active les clés étrangères et crée toutes les tables (idempotent).
pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    for stmt in MIGRATIONS {
        conn.execute_batch(stmt)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::TempDir;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();

        conn
    }

    #[test]
    fn test_migration_creates_all_tables() {
        let conn = setup_test_db();

        let expected_tables = [
            "family_members",
            "needs",
            "subscriptions",
            "bundle_components",
            "subscription_needs",
            "usage_ratings",
            "price_history",
        ];

        for table in expected_tables {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "La table '{table}' est manquante");
        }
    }

    #[test]
    fn test_migration_is_idempotent() {
        // Ouvrir une base en mémoire
        // Exécuter la migration DEUX FOIS
        // Vérifier que ça ne crashe pas (IF NOT EXISTS)
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();
    }

    #[test]
    fn test_foreign_keys_enabled() {
        // Ouvrir une base en mémoire
        // Exécuter la migration
        // Vérifier que PRAGMA foreign_keys retourne 1
        let conn = setup_test_db();
        let value: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
            .unwrap();
        assert_eq!(value, 1, "Les foreign_keys ne sont pas activées");
    }

    #[test]
    fn test_data_directory_created() -> Result<()> {
        // Appeler la fonction d'ouverture de base avec un chemin dans un dossier temporaire
        // Vérifier que le dossier a été créé
        // (utilise tempfile ou std::env::temp_dir pour ne pas polluer le projet)
        let tmp = TempDir::new()?;
        let db_path = tmp.path().join("subdir").join("db.sqlite");

        let _conn = open_database(&db_path)?;

        assert!(
            db_path.parent().unwrap().exists(),
            "Le répertoire parent n'a pas été créé"
        );
        Ok(())
    }
}

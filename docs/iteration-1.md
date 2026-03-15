# Itération 1 — Fondations

## Objectif

À la fin de cette itération, tu disposes d'une application TUI fonctionnelle dans laquelle tu peux :

- Naviguer entre des écrans (vides pour l'instant)
- Ajouter, lister, modifier et supprimer des abonnements en base SQLite
- Charger le catalogue de services depuis un fichier TOML

Pas de logique métier avancée (doublons, recommandations…). On pose les briques.

---

## Prérequis

Avant de commencer, assure-toi d'avoir :

- Rust installé via [rustup](https://rustup.rs/)
- Un éditeur avec rust-analyzer (VS Code, Helix, Zed…)
- SQLite installé sur ta machine (`sqlite3` en CLI pour inspecter la base)

Ressources de référence à garder sous le coude :

- [The Rust Book](https://doc.rust-lang.org/book/) — la référence officielle
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — apprendre par la pratique
- [Documentation Ratatui](https://ratatui.rs/tutorials/) — tutoriels TUI
- [Documentation rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) — API SQLite
- [Documentation serde](https://serde.rs/) — sérialisation

---

## Approche TDD

Pour cette itération, on adopte une approche **Test-Driven Development**. Le cycle pour chaque tâche est :

1. **🔴 Rouge** — Écrire les tests décrits dans la tâche. Ils ne compilent pas ou échouent. C'est normal.
2. **🟢 Vert** — Écrire le minimum de code pour que les tests passent. Pas de perfectionnisme, juste faire passer les tests.
3. **🔵 Refactor** — Nettoyer le code, améliorer les noms, extraire des fonctions. Les tests doivent toujours passer.

**Commandes utiles :**

- `cargo test` — lance tous les tests
- `cargo test nom_du_test` — lance un test spécifique
- `cargo test -- --nocapture` — affiche les `println!` dans les tests
- `cargo test -p subtracker -- models` — lance les tests d'un module spécifique

**Toutes les tâches n'ont pas de tests :**

- **Tâches 1-2** (setup) : pas de TDD, c'est de la configuration
- **Tâches 3-6** (modèles, DB, TOML) : **TDD complet** — les tests sont fournis, tu les écris en premier
- **Tâche 7** (shell TUI) : pas de TDD, le rendu est visuel
- **Tâches 8-10** (écrans TUI) : **TDD partiel** — on teste la logique métier, pas le rendu

**Règle d'or :** avant d'écrire la moindre ligne d'implémentation, écris les tests de la tâche. Si tu es tenté de coder d'abord, résiste. Les tests te guident vers le bon design.

### Lecture recommandée

- Rust Book chapitre 11 : [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

## Tâche 1 — Initialiser le projet Cargo

> **Statut : ✅ Terminée**

### Objectif

Créer le projet Rust, configurer les dépendances, vérifier que tout compile.

### Ce que tu vas apprendre

- Le système de build Cargo (Cargo.toml, workspace, crates)
- La gestion des dépendances en Rust (crates.io)
- La structure d'un projet Rust (src/main.rs, modules)

### À faire

1. `cargo init subtracker`
2. Ajouter les dépendances dans `Cargo.toml` :
   - `ratatui`, `crossterm` (TUI)
   - `rusqlite` avec la feature `bundled` (SQLite)
   - `serde`, `serde_derive`, `toml` (sérialisation TOML)
   - `chrono` (dates)
   - `anyhow` (gestion d'erreurs)
3. Écrire un `main.rs` minimal qui affiche "SubTracker" dans le terminal et quitte
4. Vérifier que `cargo build` et `cargo run` fonctionnent

### Indices

- La feature `bundled` de rusqlite embarque SQLite dans le binaire — pas besoin d'installer SQLite comme dépendance système.
- `anyhow` te permet d'utiliser `Result<()>` partout sans définir tes propres types d'erreur au début. C'est parfait pour un MVP.

### Critères de validation

- [x] `cargo build` compile sans erreur
- [x] `cargo run` affiche quelque chose et quitte proprement
- [x] `cargo clippy` ne remonte aucun warning

### Lecture recommandée

- Rust Book chapitre 1 : [Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- Rust Book chapitre 7 : [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

## Tâche 2 — Structurer les modules

> **Statut : ✅ Terminée**

### Objectif

Mettre en place l'arborescence de modules du projet telle que définie dans le PRD.

### Ce que tu vas apprendre

- Le système de modules Rust (`mod`, `pub`, `use`)
- La différence entre `mod.rs` et le style fichier (Rust 2018+)
- La visibilité (`pub`, `pub(crate)`, privé par défaut)

### À faire

Créer l'arborescence complète suivante :

```
src/
├── main.rs                    # Point d'entrée — déclare tous les modules racine
├── app.rs                     # État global de l'application (écran actif, mode, flags)
├── config.rs                  # Configuration (chemins des fichiers, paramètres)
│
├── models/                    # Un fichier par entité du domaine
│   ├── mod.rs                 # Déclare et ré-exporte les sous-modules
│   ├── subscription.rs        # Struct Subscription + enum Frequency, PaymentSource, SubscriptionStatus
│   ├── need.rs                # Struct Need
│   ├── family_member.rs       # Struct FamilyMember
│   ├── bundle_component.rs    # Struct BundleComponent
│   ├── usage.rs               # Struct UsageRating + enum UsageLevel
│   └── alternative.rs         # Struct Alternative (pour la désérialisation TOML)
│
├── db/                        # Couche d'accès aux données
│   ├── mod.rs                 # Déclare les sous-modules
│   ├── migrations.rs          # Création des tables (CREATE TABLE IF NOT EXISTS)
│   └── queries.rs             # Fonctions CRUD (insert, list, get, update, delete)
│
├── services/                  # Logique métier
│   ├── mod.rs                 # Déclare les sous-modules
│   ├── catalog.rs             # Chargement du catalog.toml + recherche/autocomplete
│   ├── duplicates.rs          # Détection des doublons par besoin
│   ├── recommendations.rs     # Matching alternatives par besoin
│   ├── projections.rs         # Calcul des scénarios d'économies sur 12 mois
│   └── report.rs              # Génération du récapitulatif mensuel
│
└── ui/                        # Un fichier par écran de la TUI
    ├── mod.rs                 # Déclare les sous-modules
    ├── home.rs                # Dashboard (écran d'accueil avec indicateurs et graphiques)
    ├── subscriptions.rs       # Liste des abonnements + formulaire ajout/édition
    ├── needs.rs               # Vue croisée besoins × services
    ├── duplicates.rs          # Écran de détection des doublons
    ├── recommendations.rs     # Alternatives EU / open source par besoin
    ├── projections.rs         # Scénarios de projection d'économies
    ├── monthly_report.rs      # Récapitulatif mensuel
    ├── family.rs              # Gestion des membres + matrice d'usage
    └── quickstart.rs          # Parcours onboarding (premier lancement)
```

**Pour chaque fichier :**

1. Chaque `mod.rs` déclare ses sous-modules avec `pub mod` et ré-exporte les types principaux si nécessaire
2. Chaque fichier de module contient au minimum un commentaire `// TODO` ou une struct/fonction vide pour que le compilateur soit content
3. `main.rs` déclare les modules racine : `mod app;`, `mod config;`, `mod models;`, `mod db;`, `mod services;`, `mod ui;`
4. `cargo build` compile toujours

**Exemple concret pour `models/mod.rs` :**

```rust
pub mod subscription;
pub mod need;
pub mod family_member;
pub mod bundle_component;
pub mod usage;
pub mod alternative;
```

Fais de même pour `db/mod.rs`, `services/mod.rs` et `ui/mod.rs`.

### Indices

- En Rust, un dossier `ui/` a besoin d'un `ui/mod.rs` (ou `ui.rs` à côté du dossier) pour être reconnu comme module.
- Commence par tout mettre en `pub` pour ne pas te battre avec la visibilité au début. Tu pourras restreindre plus tard.
- Attention : un module déclaré dans `mod.rs` mais sans fichier correspondant ne compilera pas.

### Pièges courants

- Oublier de déclarer un sous-module dans le `mod.rs` parent
- Confondre `mod` (déclaration) et `use` (import)
- Circular dependencies entre modules (Rust ne le permet pas)

### Critères de validation

- [x] L'arborescence correspond au PRD
- [x] `cargo build` compile
- [x] `cargo clippy` est propre

### Lecture recommandée

- Rust Book chapitre 7.2 : [Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

---

## Tâche 3 — Définir les structs du modèle de données

> **Statut : ✅ Terminée**

### Objectif

Traduire le schéma SQL du PRD en structs Rust. Tu ne touches pas encore à SQLite ici — c'est la modélisation pure.

### Ce que tu vas apprendre

- Les structs et enums en Rust
- Les traits `derive` (Debug, Clone, PartialEq)
- Les enums comme types somme (pour les fréquences, niveaux d'usage…)
- Le type `Option<T>` pour les champs optionnels

### À faire

Créer les structs suivantes dans `src/models/` :

1. **`Subscription`** — id, name, provider, amount, frequency, monthly_cost, is_bundle, is_family_plan, payment_source, start_date, renewal_date, status, notes
2. **`Need`** — id, name, family, essential
3. **`FamilyMember`** — id, name
4. **`BundleComponent`** — id, subscription_id, name, need_id, individual_price, allocated_cost
5. **`UsageRating`** — id, subscription_id, component_name, member_id, rating
6. **`PriceHistory`** — id, subscription_id, amount, frequency, recorded_at

Pour les champs à valeurs discrètes, créer des enums :

- `Frequency` : Monthly, Yearly, Quarterly
- `UsageLevel` : Heavy, Occasional, Rare, Never
- `SubscriptionStatus` : Active, Archived
- `PaymentSource` : Card, PayPal, DirectDebit, Apple, Other

### Indices

- Pense à `#[derive(Debug, Clone)]` sur toutes tes structs — tu en auras besoin pour le debug et la TUI.
- Pour les enums, regarde comment implémenter `Display` pour pouvoir les afficher facilement.
- Les champs optionnels (notes, dates…) utilisent `Option<String>`, `Option<NaiveDate>`, etc.
- Ne te préoccupe pas de serde pour l'instant, on l'ajoutera quand on lira le TOML.

### Questions à te poser

- Comment représenter le `monthly_cost` qui est un calcul dérivé de `amount` et `frequency` ? Faut-il le stocker ou le calculer à la volée ? Quels sont les avantages de chaque approche ?
- Comment gérer le lien entre `BundleComponent` et `Need` ? Par id ? Par référence ? Qu'est-ce qui est le plus pratique en Rust ?

### Tests à écrire EN PREMIER (TDD)

Crée un bloc `#[cfg(test)] mod tests { ... }` dans chaque fichier de modèle. Écris ces tests avant d'implémenter les structs — ils ne compileront pas, c'est normal. Le cycle est : **test rouge → implémentation → test vert → refactor**.

**Dans `models/subscription.rs` :**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_subscription() {
        // Créer une Subscription avec tous les champs obligatoires
        // Vérifier que les champs sont accessibles et ont les bonnes valeurs
    }

    #[test]
    fn test_subscription_optional_fields() {
        // Créer une Subscription avec notes = None, renewal_date = None
        // Vérifier que les champs optionnels sont bien None
    }

    #[test]
    fn test_monthly_cost_from_monthly() {
        // Un abonnement à 10.99€/mois → monthly_cost = 10.99
    }

    #[test]
    fn test_monthly_cost_from_yearly() {
        // Un abonnement à 99.00€/an → monthly_cost = 8.25
    }

    #[test]
    fn test_monthly_cost_from_quarterly() {
        // Un abonnement à 30.00€/trimestre → monthly_cost = 10.00
    }

    #[test]
    fn test_frequency_display() {
        // Frequency::Monthly s'affiche "Mensuel"
        // Frequency::Yearly s'affiche "Annuel"
        // Frequency::Quarterly s'affiche "Trimestriel"
    }

    #[test]
    fn test_subscription_status_default() {
        // Un nouvel abonnement a le statut Active par défaut
    }
}
```

**Dans `models/need.rs` :**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_need() {
        // Créer un Need avec name = "Streaming musique", family = "Divertissement"
        // Vérifier les valeurs
    }

    #[test]
    fn test_need_essential_default() {
        // Un besoin est non-essentiel par défaut
    }
}
```

**Dans `models/usage.rs` :**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_level_display() {
        // UsageLevel::Heavy s'affiche "Beaucoup"
        // UsageLevel::Occasional s'affiche "De temps en temps"
        // UsageLevel::Rare s'affiche "Rarement"
        // UsageLevel::Never s'affiche "Jamais"
    }

    #[test]
    fn test_usage_level_ordering() {
        // Heavy > Occasional > Rare > Never
        // (utile plus tard pour trier par usage)
    }
}
```

**Dans `models/bundle_component.rs` :**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bundle_component() {
        // Créer un BundleComponent pour "Apple Music" dans Apple One
        // Vérifier que individual_price et allocated_cost sont bien renseignés
    }

    #[test]
    fn test_allocated_cost_optional() {
        // allocated_cost peut être None (pas encore calculé)
    }
}
```

**Réflexion TDD :** en écrivant ces tests, tu vas naturellement te poser des questions de design. Par exemple, le `monthly_cost` : est-ce un champ stocké ou une méthode calculée ? Le test `test_monthly_cost_from_yearly` implique qu'il existe une logique de calcul quelque part. À toi de décider si c'est un `impl Subscription { fn monthly_cost(&self) -> f64 }` ou un champ pré-calculé. Les deux approches sont valides, chacune a ses avantages.

### Critères de validation

- [x] Tous les tests compilent et passent au vert
- [x] Toutes les structs compilent
- [x] Les enums couvrent toutes les valeurs possibles
- [x] `cargo test` passe intégralement

### Lecture recommandée

- Rust Book chapitre 5 : [Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
- Rust Book chapitre 6 : [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- Rust Book chapitre 10.2 : [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- Rust Book chapitre 11 : [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

## Tâche 4 — Créer la base SQLite et les migrations

### Objectif

Initialiser la base de données SQLite et créer toutes les tables au premier lancement.

### Ce que tu vas apprendre

- Utiliser rusqlite pour ouvrir/créer une base SQLite
- Exécuter du SQL depuis Rust
- Gérer le chemin du fichier de base de données
- La gestion d'erreurs avec `Result` et `anyhow`

### À faire

1. Dans `src/db/`, créer une fonction qui ouvre (ou crée) la base SQLite
2. Créer une fonction de migration qui exécute les `CREATE TABLE IF NOT EXISTS` du PRD
3. Appeler cette migration au démarrage de l'application dans `main.rs`
4. Vérifier que le fichier `subtracker.db` est créé avec les bonnes tables

### Indices

- `rusqlite::Connection::open("path/to/db")` crée le fichier s'il n'existe pas.
- Utilise `execute_batch` pour exécuter plusieurs statements SQL d'un coup.
- Pour le chemin du fichier, commence simple : `./data/subtracker.db`. Tu pourras utiliser la crate `directories` plus tard pour un chemin XDG-compliant.
- Pense à activer les foreign keys avec `PRAGMA foreign_keys = ON`.

### Pièges courants

- Oublier le `IF NOT EXISTS` — la migration crashera au deuxième lancement.
- Ne pas gérer l'erreur si le dossier `data/` n'existe pas — pense à `std::fs::create_dir_all`.
- SQLite n'a pas de type `BOOLEAN` natif — c'est stocké comme INTEGER (0/1).

### Tests à écrire EN PREMIER (TDD)

Crée un fichier de tests dans `src/db/`. Pour les tests de base de données, tu utiliseras une base en mémoire (`Connection::open_in_memory()`) pour chaque test — chaque test a sa propre base, isolée.

**Dans `db/migrations.rs` :**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migration_creates_all_tables() {
        // Ouvrir une base en mémoire
        // Exécuter la migration
        // Vérifier que chaque table existe en interrogeant sqlite_master
        // Tables attendues : family_members, needs, subscriptions,
        //   bundle_components, subscription_needs, usage_ratings, price_history
    }

    #[test]
    fn test_migration_is_idempotent() {
        // Ouvrir une base en mémoire
        // Exécuter la migration DEUX FOIS
        // Vérifier que ça ne crashe pas (IF NOT EXISTS)
    }

    #[test]
    fn test_foreign_keys_enabled() {
        // Ouvrir une base en mémoire
        // Exécuter la migration
        // Vérifier que PRAGMA foreign_keys retourne 1
    }

    #[test]
    fn test_data_directory_created() {
        // Appeler la fonction d'ouverture de base avec un chemin dans un dossier temporaire
        // Vérifier que le dossier a été créé
        // (utilise tempfile ou std::env::temp_dir pour ne pas polluer le projet)
    }
}
```

**Indice TDD :** le test `test_migration_creates_all_tables` va te forcer à trouver comment vérifier l'existence d'une table en SQLite. Cherche du côté de `SELECT name FROM sqlite_master WHERE type='table'`. C'est un bon exercice SQL + rusqlite.

### Critères de validation

- [X] Tous les tests passent
- [X] Au premier `cargo run`, la base est créée
- [X] Tu peux ouvrir le fichier avec `sqlite3 data/subtracker.db` et voir les tables (`.tables`)
- [X] Un deuxième `cargo run` ne crashe pas (idempotent)

### Lecture recommandée

- Rust Book chapitre 9 : [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [rusqlite getting started](https://docs.rs/rusqlite/latest/rusqlite/#usage)

---

## Tâche 5 — Implémenter le CRUD des abonnements

### Objectif

Pouvoir ajouter, lister, modifier et supprimer des abonnements en base depuis le code Rust. Pas d'interface TUI encore — on teste via des tests unitaires ou un main temporaire.

### Ce que tu vas apprendre

- Les requêtes préparées avec rusqlite (INSERT, SELECT, UPDATE, DELETE)
- Le mapping entre les résultats SQL et tes structs Rust
- Les closures en Rust (utilisées par rusqlite pour mapper les rows)
- L'ownership et le borrowing (ça va commencer à piquer ici 😉)

### À faire

1. Dans `src/db/queries.rs`, implémenter :
   - `insert_subscription(conn, subscription) -> Result<i64>` (retourne l'id)
   - `list_subscriptions(conn) -> Result<Vec<Subscription>>`
   - `get_subscription(conn, id) -> Result<Option<Subscription>>`
   - `update_subscription(conn, subscription) -> Result<()>`
   - `delete_subscription(conn, id) -> Result<()>`
2. Écrire des tests pour chaque fonction

### Indices

- `conn.execute()` pour INSERT/UPDATE/DELETE, `conn.prepare()` + `query_map()` pour SELECT.
- Le mapping d'une row SQL vers ta struct se fait dans une closure : `|row| Ok(Subscription { id: row.get(0)?, name: row.get(1)?, ... })`.
- C'est ici que tu vas rencontrer le borrow checker pour la première fois sérieusement. Si `conn` est emprunté mutablement par une opération, tu ne peux pas l'emprunter à nouveau en même temps.
- Pour les tests, rusqlite permet de créer une base en mémoire : `Connection::open_in_memory()`. C'est parfait pour les tests.

### Pièges courants

- Confondre les indices de colonnes dans `row.get(n)` — une erreur d'index donne une erreur runtime, pas compile time.
- Oublier de `collect()` le résultat de `query_map()` — ça retourne un itérateur, pas un Vec.
- Les types SQL et Rust ne correspondent pas toujours directement. `REAL` → `f64`, `TEXT` → `String`, `INTEGER` → `i64`.

### Tests à écrire EN PREMIER (TDD)

C'est la tâche la plus riche en tests. Chaque test crée sa propre base en mémoire, exécute la migration, puis teste une opération CRUD.

**Astuce :** tu vas probablement créer une fonction helper `setup_test_db()` qui ouvre une base en mémoire et applique les migrations. Tu la réutiliseras dans tous tes tests.

**Dans `db/queries.rs` :**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;
    use rusqlite::Connection;

    /// Helper : crée une base en mémoire avec les migrations appliquées
    fn setup_test_db() -> Connection {
        // À toi de jouer
    }

    // --- INSERT ---

    #[test]
    fn test_insert_subscription_returns_id() {
        // Insérer un abonnement
        // Vérifier que l'id retourné est > 0
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
```

**Réflexion TDD :** en écrivant ces tests, tu vas devoir décider de la signature exacte de tes fonctions. Par exemple, `insert_subscription` prend-elle une `&Subscription` ou une struct dédiée `NewSubscription` (sans id) ? Les deux approches existent — le test te force à trancher avant de coder.

### Critères de validation

- [ ] Tous les tests passent avec `cargo test`
- [ ] Tu peux insérer un abonnement et le retrouver par id
- [ ] La liste retourne tous les abonnements insérés
- [ ] La modification met à jour les champs
- [ ] La suppression retire l'abonnement de la liste

### Lecture recommandée

- Rust Book chapitre 4 : [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- Rust Book chapitre 13.1 : [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [rusqlite query examples](https://docs.rs/rusqlite/latest/rusqlite/struct.Statement.html#method.query_map)

---

## Tâche 6 — Charger le catalogue et les alternatives depuis TOML

### Objectif

Lire les fichiers `catalog.toml` et `alternatives.toml` et les désérialiser en structs Rust.

### Ce que tu vas apprendre

- La sérialisation / désérialisation avec serde
- Les attributs `#[derive(Deserialize)]` et `#[serde(...)]`
- Lire un fichier et parser du TOML
- La gestion des erreurs de parsing

### À faire

1. Créer les structs pour le catalogue (`CatalogService`, `CatalogComponent`) et les alternatives (`Alternative`) avec les dérivations serde
2. Créer une struct englobante pour chaque fichier (`Catalog`, `Alternatives`)
3. Dans `src/services/catalog.rs`, implémenter :
   - `load_catalog(path) -> Result<Catalog>`
   - `load_alternatives(path) -> Result<Alternatives>`
4. Créer un `catalog.toml` et un `alternatives.toml` de test avec quelques entrées (tu peux reprendre les exemples du PRD)
5. Écrire des tests qui vérifient le parsing

### Indices

- La crate `toml` s'utilise avec `toml::from_str::<T>(&content)` où `T` implémente `Deserialize`.
- Pour lire un fichier : `std::fs::read_to_string(path)`.
- Les noms de champs TOML utilisent des snake_case par convention, donc ça matche directement avec les conventions Rust.
- Si un champ TOML est optionnel, utilise `Option<T>` dans ta struct + `#[serde(default)]`.

### Pièges courants

- La structure TOML `[[alternatives]]` (double bracket) représente un tableau de tables. Ta struct englobante doit avoir un champ `alternatives: Vec<Alternative>`.
- Les tables imbriquées dans TOML (comme les composantes d'un service dans le catalogue) nécessitent une attention à la structure de désérialisation. Teste avec un fichier minimal d'abord.
- Une erreur de type dans le TOML (string au lieu de nombre) donne une erreur serde peu lisible au début — lis-la attentivement.

### Tests à écrire EN PREMIER (TDD)

Pour les tests de parsing TOML, tu vas utiliser des chaînes TOML en dur dans les tests plutôt que des fichiers. Ça rend les tests autonomes et rapides.

**Dans `services/catalog.rs` :**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_service() {
        // Parser un TOML minimal avec un seul service non-bundle
        // Vérifier : name, provider, amount, frequency, needs
        let toml_content = r#"
            [[services]]
            name = "Netflix Standard"
            provider = "Netflix"
            amount = 13.49
            frequency = "monthly"
            is_bundle = false
            is_family_plan = false
            needs = ["Streaming vidéo"]
        "#;
        // Parser et vérifier les champs
    }

    #[test]
    fn test_parse_bundle_with_components() {
        // Parser un service bundle avec des composantes
        // Vérifier que les composantes sont bien rattachées
        // Vérifier : nombre de composantes, noms, besoins, prix individuels
        // Utilise Apple One comme exemple
    }

    #[test]
    fn test_parse_multiple_services() {
        // Parser un TOML avec 3 services
        // Vérifier que la liste contient 3 éléments
    }

    #[test]
    fn test_parse_empty_catalog() {
        // Un TOML sans services : "services = []" ou équivalent
        // Doit retourner un catalogue vide, pas une erreur
    }

    #[test]
    fn test_parse_invalid_toml_returns_error() {
        // Un TOML mal formé → Result::Err
        let bad_toml = "ceci n'est pas du TOML valide [[[";
        // Vérifier que le résultat est une erreur
    }

    #[test]
    fn test_parse_missing_required_field() {
        // Un service sans le champ "name" → erreur de désérialisation
    }

    #[test]
    fn test_load_catalog_from_file() {
        // Créer un fichier TOML temporaire (avec std::fs::write dans un tempdir)
        // Appeler load_catalog(path)
        // Vérifier que le résultat est correct
    }

    #[test]
    fn test_load_catalog_file_not_found() {
        // Appeler load_catalog avec un chemin inexistant
        // Vérifier que ça retourne une erreur (pas un panic)
    }
}
```

**Dans `services/catalog.rs` (section alternatives) ou un fichier dédié :**

```rust
#[cfg(test)]
mod alternative_tests {
    use super::*;

    #[test]
    fn test_parse_alternative_european() {
        // Parser une alternative européenne (Deezer)
        // Vérifier : european = true, open_source = false, self_hostable = false
    }

    #[test]
    fn test_parse_alternative_open_source() {
        // Parser une alternative open source (Navidrome)
        // Vérifier : open_source = true, self_hostable = true, price = 0.0
    }

    #[test]
    fn test_parse_alternative_with_family_plan() {
        // Parser une alternative avec offre famille
        // Vérifier que family_price_monthly est Some(prix)
    }

    #[test]
    fn test_parse_alternative_without_family_plan() {
        // Parser une alternative sans offre famille
        // Vérifier que family_price_monthly est None
    }

    #[test]
    fn test_alternatives_grouped_by_need() {
        // Parser plusieurs alternatives couvrant le même besoin
        // Filtrer par besoin "Streaming musique"
        // Vérifier qu'on obtient les bonnes alternatives (Deezer, Qobuz, Navidrome)
    }
}
```

**Indice TDD :** le test `test_parse_bundle_with_components` va te confronter à la complexité de la structure TOML imbriquée. C'est le test le plus formateur — il te forcera à bien modéliser la relation service/composantes dans tes structs serde. Commence par écrire le TOML attendu dans le test, puis déduis la struct qui peut le désérialiser.

### Critères de validation

- [ ] Tous les tests passent
- [ ] Le catalogue se charge et contient les services attendus
- [ ] Les alternatives se chargent avec tous les champs
- [ ] Une erreur dans le TOML remonte une erreur claire
- [ ] Un fichier manquant remonte une erreur (pas de panic)

### Lecture recommandée

- [Serde guide](https://serde.rs/)
- [TOML crate documentation](https://docs.rs/toml/latest/toml/)
- Rust Book chapitre 12 : [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) (pour la lecture de fichiers)

---

## Tâche 7 — Mettre en place le shell TUI avec Ratatui

### Objectif

Avoir une application TUI qui démarre, affiche un layout avec navigation par onglets entre des écrans vides, et se quitte proprement avec `q`.

### Ce que tu vas apprendre

- Le pattern d'application TUI (boucle d'événements)
- Le setup/teardown du terminal avec crossterm (alternate screen, raw mode)
- Le système de layout de Ratatui (Rect, Layout, Constraint)
- La gestion des événements clavier

### À faire

1. Dans `src/app.rs`, créer une struct `App` qui contient :
   - L'écran actif (un enum : Dashboard, Subscriptions, Needs, Duplicates, Alternatives, Projections, MonthlyReport, Family)
   - Un flag `running: bool`
2. Dans `src/main.rs`, implémenter la boucle principale :
   - Setup du terminal (alternate screen, raw mode)
   - Boucle : lire événement → mettre à jour l'état → dessiner
   - Teardown propre à la sortie (même en cas d'erreur !)
3. Implémenter un layout basique :
   - Une barre d'onglets en haut (les noms des écrans)
   - Un espace central (vide pour l'instant, juste le nom de l'écran actif)
   - Une barre de statut en bas (raccourcis disponibles)
4. Navigation avec `Tab` / `Shift+Tab` entre les écrans
5. Quitter avec `q`

### Indices

- Ratatui utilise un pattern "immediate mode" : tu redessines tout l'écran à chaque frame. Pas de gestion d'état du rendu.
- Le setup/teardown du terminal est critique. Si ton app plante sans teardown, ton terminal sera dans un état bizarre. Utilise un pattern avec `Drop` ou un guard pour garantir le cleanup.
- Pour les layouts, `Layout::default().direction(Direction::Vertical).constraints([...])` est ton ami.
- Pour les onglets, regarde le widget `Tabs` de Ratatui.

### Pièges courants

- Oublier le teardown du terminal en cas de panic — ton terminal sera cassé. Regarde comment installer un panic hook qui restore le terminal.
- La boucle d'événements doit avoir un timeout (ex: 250ms) sinon elle bloque indéfiniment en attendant un événement clavier.
- Ne pas dessiner à chaque itération de la boucle — dessine seulement quand il y a un événement ou un tick.

### Critères de validation

- [ ] L'application démarre et affiche le layout
- [ ] Tab / Shift+Tab navigue entre les écrans
- [ ] Le nom de l'écran actif s'affiche au centre
- [ ] `q` quitte proprement
- [ ] Si tu fais `panic!()` dans le code, le terminal se restore quand même

### Lecture recommandée

- [Ratatui — Getting Started](https://ratatui.rs/tutorials/hello-world/)
- [Ratatui — Counter App Tutorial](https://ratatui.rs/tutorials/counter-app/) (pattern recommandé)
- [Ratatui — Layout](https://ratatui.rs/concepts/layout/)

---

## Tâche 8 — Écran de liste des abonnements

### Objectif

Le premier vrai écran : afficher la liste des abonnements depuis la base SQLite, avec possibilité de naviguer dans la liste.

### Ce que tu vas apprendre

- Le widget `Table` de Ratatui (colonnes, rows, sélection)
- La gestion de l'état dans l'app (state partagé entre l'app et la TUI)
- Connecter la couche DB à la couche UI
- Le concept de `TableState` pour le suivi de la sélection

### À faire

1. Dans `src/ui/subscriptions.rs`, implémenter le rendu de la liste :
   - Tableau avec colonnes : Nom, Montant, Fréquence, Coût mensuel, Statut
   - Sélection avec flèches haut/bas
   - Le nombre total d'abonnements et le coût mensuel total en pied de tableau
2. L'app charge les abonnements depuis SQLite au démarrage et les garde en mémoire
3. La navigation `↑`/`↓` déplace la sélection

### Indices

- `ratatui::widgets::Table` prend des `Row` qui contiennent des `Cell`. Regarde les exemples dans la doc.
- `TableState` garde l'index de la ligne sélectionnée. Tu le passes au moment du rendu avec `render_stateful_widget`.
- Pour le coût mensuel, pense à ta logique de conversion : un abonnement à 99€/an = 8,25€/mois.
- Sépare bien la logique de rendu (ui/) de la logique métier (services/) et des données (db/).

### Tests de la logique (pas du rendu)

Le rendu TUI est difficile à tester automatiquement, mais la logique derrière l'écran est parfaitement testable. Sépare bien les deux.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_monthly_cost() {
        // Donner une liste de 3 abonnements (10€ + 15€ + 8.25€)
        // Vérifier que le total = 33.25€
    }

    #[test]
    fn test_total_monthly_cost_empty_list() {
        // Liste vide → total = 0.0
    }

    #[test]
    fn test_total_annual_cost() {
        // Même liste → total annuel = 33.25 * 12 = 399.0€
    }

    #[test]
    fn test_subscriptions_sorted_by_cost_descending() {
        // Donner une liste non triée
        // Trier par coût mensuel décroissant
        // Vérifier l'ordre
    }

    #[test]
    fn test_only_active_subscriptions_in_list() {
        // 3 actifs + 1 archivé → la liste filtrée contient 3 éléments
    }

    #[test]
    fn test_format_amount_display() {
        // 19.99 → "19,99€"
        // 8.25 → "8,25€"
        // 0.0 → "0,00€"
        // (vérifie ta logique de formatage des montants)
    }
}
```

### Critères de validation

- [ ] Les tests de logique passent
- [ ] L'écran affiche un tableau avec des colonnes lisibles
- [ ] Les données viennent de SQLite (insère quelques abonnements de test)
- [ ] La navigation haut/bas fonctionne
- [ ] Le coût mensuel total s'affiche

### Lecture recommandée

- [Ratatui — Table widget](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html)
- [Ratatui examples — table](https://github.com/ratatui/ratatui/blob/main/examples/table.rs)

---

## Tâche 9 — Formulaire d'ajout d'un abonnement

### Objectif

Pouvoir ajouter un abonnement depuis la TUI, avec un formulaire simple.

### Ce que tu vas apprendre

- La gestion de la saisie texte dans une TUI (c'est plus compliqué qu'on croit !)
- Les machines à état pour gérer les modes (navigation vs saisie)
- Le widget `Paragraph` pour les champs de saisie
- La crate `tui-input` ou l'implémentation manuelle d'un input

### À faire

1. Quand l'utilisateur appuie sur `a` dans la liste, basculer en mode "ajout"
2. Afficher un formulaire avec les champs : Nom, Montant, Fréquence (sélection), Bundle oui/non, Offre famille oui/non
3. Navigation entre les champs avec Tab
4. Validation avec Enter (insert en base et retour à la liste)
5. Annulation avec Escape

### Indices

- Tu vas avoir besoin de distinguer deux modes dans ton app : `Normal` (navigation) et `Editing` (saisie). Les mêmes touches font des choses différentes selon le mode.
- Pour les champs de sélection (Fréquence, oui/non), pas besoin de saisie texte — juste `←`/`→` pour changer la valeur.
- La saisie texte est le point le plus délicat. Tu peux commencer avec un `String` et gérer les `KeyCode::Char(c)` manuellement, ou utiliser `tui-input` qui gère le curseur pour toi.
- Le calcul du `monthly_cost` se fait automatiquement à partir de `amount` et `frequency`.

### Pièges courants

- En raw mode, tu reçois chaque touche individuellement. Backspace, flèches, tout doit être géré manuellement.
- Ne pas oublier de vider le formulaire après la validation.
- La gestion des modes est source de bugs subtils — assure-toi que Escape revient toujours en mode Normal.

### Tests de la logique de validation du formulaire

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_form_valid() {
        // Formulaire complet avec nom, montant > 0, fréquence → Ok
    }

    #[test]
    fn test_validate_form_empty_name() {
        // Nom vide → Err avec message explicite
    }

    #[test]
    fn test_validate_form_negative_amount() {
        // Montant négatif → Err
    }

    #[test]
    fn test_validate_form_zero_amount() {
        // Montant = 0 → Err (un abonnement gratuit n'a pas de sens ici)
    }

    #[test]
    fn test_validate_form_invalid_amount_format() {
        // Montant saisi = "abc" → Err (pas un nombre)
    }

    #[test]
    fn test_parse_amount_with_comma() {
        // "19,99" → 19.99 (l'utilisateur peut saisir avec une virgule à la française)
    }

    #[test]
    fn test_parse_amount_with_dot() {
        // "19.99" → 19.99
    }

    #[test]
    fn test_monthly_cost_calculation_on_submit() {
        // Formulaire avec amount = 99.0, frequency = Yearly
        // Après validation, monthly_cost = 8.25
    }
}
```

**Indice TDD :** ces tests te poussent à extraire une fonction `validate_form(...)` et une fonction `parse_amount(input: &str) -> Result<f64>` séparées du rendu TUI. C'est exactement le bon réflexe — la logique de validation doit être indépendante de l'interface.

### Critères de validation
- [ ] On peut saisir du texte dans les champs
- [ ] La validation insère en base et rafraîchit la liste
- [ ] Escape annule sans rien persister
- [ ] Le coût mensuel se recalcule

### Lecture recommandée

- [tui-input crate](https://docs.rs/tui-input/latest/tui_input/)
- [Ratatui — How to use widgets](https://ratatui.rs/how-to/widgets/)

---

## Tâche 10 — Édition et suppression

### Objectif

Compléter le CRUD dans la TUI : modifier un abonnement existant et le supprimer/archiver.

### Ce que tu vas apprendre

- Réutiliser un composant (le formulaire) pour l'édition
- Les dialogues de confirmation
- Le pattern "sélection → action" dans une TUI

### À faire

1. `e` sur un abonnement sélectionné → ouvre le formulaire pré-rempli avec les données existantes
2. `d` sur un abonnement sélectionné → affiche un dialogue de confirmation "Archiver cet abonnement ? (o/n)"
3. Confirmation → archive en base (soft delete : status = "archived"), rafraîchit la liste
4. La liste ne montre que les abonnements actifs par défaut

### Indices

- Pour le formulaire d'édition, tu peux réutiliser exactement le même composant que pour l'ajout. La seule différence c'est qu'il est pré-rempli et que la validation fait un UPDATE au lieu d'un INSERT.
- Pour le dialogue de confirmation, un simple `Popup` centré suffit. Ratatui n'a pas de widget popup natif, mais tu peux en créer un facilement avec un `Clear` + `Block` + `Paragraph` rendu par-dessus le layout principal.
- Pense à ajouter un mode `Confirming` à ta machine à état.

### Tests de la logique d'édition et d'archivage

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_subscription_changes_status() {
        // Insérer un abonnement actif
        // L'archiver
        // get_subscription → status == Archived
    }

    #[test]
    fn test_archived_subscription_excluded_from_active_list() {
        // Insérer 3 abonnements, en archiver 1
        // list_active_subscriptions → 2 résultats
    }

    #[test]
    fn test_archived_subscription_still_in_full_list() {
        // Insérer 3 abonnements, en archiver 1
        // list_all_subscriptions → 3 résultats (si tu implémentes cette variante)
    }

    #[test]
    fn test_edit_preserves_id() {
        // Insérer un abonnement, récupérer son id
        // Le modifier (changer le nom)
        // Vérifier que l'id n'a pas changé
    }

    #[test]
    fn test_edit_updates_monthly_cost_on_frequency_change() {
        // Insérer un abonnement mensuel à 10€ (monthly_cost = 10€)
        // Modifier la fréquence en annuel (amount = 10€/an)
        // Vérifier que monthly_cost est recalculé (= 0.83€)
    }

    #[test]
    fn test_form_prefill_from_existing_subscription() {
        // Créer une Subscription connue
        // Convertir en données de formulaire (FormData ou équivalent)
        // Vérifier que tous les champs sont correctement pré-remplis
    }
}
```

**Indice TDD :** le test `test_form_prefill_from_existing_subscription` va te pousser à créer un type intermédiaire (ex : `FormData`) qui fait le pont entre une `Subscription` (base) et le formulaire TUI (strings). C'est un pattern courant et utile — le formulaire travaille avec des `String`, la base avec des types forts.

### Critères de validation

- [ ] Tous les tests de logique passent
- [ ] L'édition charge les données existantes dans le formulaire
- [ ] La sauvegarde met à jour la base
- [ ] La suppression demande confirmation
- [ ] Les abonnements archivés disparaissent de la liste
- [ ] `cargo test` passe intégralement

---

## Récapitulatif des concepts Rust par tâche

| Tâche | Concepts clés |
|---|---|
| 1. Init Cargo | Cargo, crates, dépendances |
| 2. Modules | mod, pub, use, visibilité |
| 3. Structs | Structs, enums, derive, Option, traits |
| 4. SQLite | Result, anyhow, gestion de fichiers |
| 5. CRUD | Closures, ownership, borrowing, lifetimes |
| 6. TOML | Serde, désérialisation, lecture fichier |
| 7. Shell TUI | Boucle d'événements, pattern matching, Drop |
| 8. Liste | Widgets stateful, état partagé |
| 9. Formulaire | Machine à état, saisie clavier, modes |
| 10. Édition | Réutilisation, popups, confirmation |

---

## Comment travailler ensemble

Pour chaque tâche :

1. **Tu lis la tâche et les ressources**
2. **Tu codes** — prends le temps de chercher, d'expérimenter, de te tromper
3. **Tu me montres ton code quand :**
   - Tu bloques et tu ne comprends pas une erreur du compilateur
   - Tu as fini et tu veux un review
   - Tu as un doute sur un choix d'architecture
4. **Je te donne du feedback** sur l'idiomatique Rust, les patterns, les améliorations possibles
5. **On passe à la tâche suivante**

N'hésite jamais à me poser des questions "bêtes" — les erreurs du compilateur Rust sont déroutantes au début, et c'est normal. Le borrow checker va te frustrer, c'est un passage obligé. L'important c'est de comprendre *pourquoi* il refuse, pas juste de trouver le fix.

Bon courage, et amuse-toi ! 🦀
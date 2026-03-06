# SubTracker -- Document d'architecture

## 1. Vue d'ensemble

SubTracker est une application TUI (Terminal User Interface) de pilotage des abonnements numériques d'un foyer. Elle fonctionne 100% en local, sans aucune dépendance cloud, selon une philosophie KISS.

### 1.1 Objectifs architecturaux

- **Simplicité** : architecture en couches claires, flux de données unidirectionnel
- **Zéro dépendance externe** : toutes les données restent locales (SQLite + fichiers TOML)
- **Synchrone** : aucun runtime asynchrone, I/O bloquant uniquement
- **Idiomatique Rust** : exploiter le système de types, les enums, le pattern matching et le modèle de propriété

### 1.2 Stack technique

| Composant | Technologie | Justification |
|---|---|---|
| Langage | Rust (edition 2024) | Apprentissage + performance + écosystème CLI/TUI |
| Interface | Ratatui + Crossterm | Standard des TUI Rust, charts intégrés |
| Persistance | SQLite via rusqlite (bundled) | Fichier unique, backup trivial, pas de serveur |
| Configuration | TOML via serde | Lisible, éditable manuellement, idiomatique Rust |
| Gestion d'erreurs | anyhow | Propagation d'erreurs simplifiée |
| Dates | chrono | Manipulation des dates et calculs temporels |

---

## 2. Architecture en couches

```
┌─────────────────────────────────────────────────────────────────┐
│                         main.rs                                 │
│              Point d'entrée, init terminal, boucle              │
│              d'événements, restauration terminal                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────────┐
│                         app.rs                                  │
│              État applicatif, navigation entre écrans,          │
│              dispatch des événements clavier                    │
└──────┬───────────────────────────────────────────────┬──────────┘
       │                                               │
┌──────▼──────────────┐                    ┌───────────▼──────────┐
│      ui/            │                    │     services/        │
│  Rendu des écrans   │                    │  Logique métier      │
│  Widgets Ratatui    │◄──── données ──────│  Calculs, détection  │
│  Gestion formulaire │                    │  Recommandations     │
└─────────────────────┘                    └───────────┬──────────┘
                                                       │
                                           ┌───────────▼──────────┐
                                           │       db/            │
                                           │  Accès SQLite        │
                                           │  Migrations, CRUD    │
                                           └───────────┬──────────┘
                                                       │
                                           ┌───────────▼──────────┐
                                           │     models/          │
                                           │  Structs & enums     │
                                           │  Partagés entre      │
                                           │  toutes les couches  │
                                           └──────────────────────┘
```

### 2.1 Règles de dépendances entre couches

| Couche | Peut dépendre de | Ne doit pas dépendre de |
|---|---|---|
| `main.rs` | `app`, `config`, `db` (init) | `ui`, `services` directement |
| `app.rs` | `ui`, `services`, `models`, `config` | `db` directement |
| `ui/` | `models` (lecture seule pour affichage) | `db`, `services` |
| `services/` | `db`, `models`, `config` | `ui`, `app` |
| `db/` | `models` | `ui`, `services`, `app` |
| `models/` | aucune (structs pures) | toutes les autres couches |

**Principe** : les couches supérieures dépendent des couches inférieures, jamais l'inverse. Les modèles sont partagés transversalement.

---

## 3. Description des modules

### 3.1 `main.rs` -- Point d'entrée

Responsabilités :
- Initialisation du terminal (mode raw, alternate screen via Crossterm)
- Ouverture de la connexion SQLite et exécution des migrations
- Chargement de la configuration
- Lancement de la boucle d'événements principale
- Restauration du terminal à la sortie (y compris en cas de panic)

```
main()
  ├── setup_terminal()
  ├── db::migration::run(&conn)
  ├── App::new(conn, config)
  ├── loop { poll events → app.handle_event() → app.render(terminal) }
  └── restore_terminal()
```

### 3.2 `app.rs` -- État applicatif

Structure centrale qui maintient :
- L'écran actif (`ActiveScreen` enum)
- Le flag `running: bool`
- L'état de chaque écran (sélection courante, formulaire en cours, etc.)
- La connexion SQLite (ou un handle partagé)

```rust
enum ActiveScreen {
    Dashboard,
    Subscriptions,
    Needs,
    Duplicates,
    Recommendations,
    Projections,
    MonthlyReport,
    Family,
    QuickStart,
}
```

L'`App` orchestre le cycle rendu/événement :
1. Reçoit un événement clavier de la boucle principale
2. Dispatch à l'écran actif ou gère la navigation globale (Tab, q, ?)
3. L'écran actif peut déclencher des appels aux services
4. Les services accèdent à la base via `db/`
5. Le rendu utilise les données mises à jour

### 3.3 `config.rs` -- Configuration

Gère les chemins vers les fichiers de données :
- `data/subtracker.db` -- base SQLite
- `data/catalog.toml` -- catalogue de services connus
- `data/alternatives.toml` -- base d'alternatives EU/open source

Les chemins respectent les conventions du système via le crate `directories` (prévu post-itération 1).

### 3.4 `models/` -- Modèle de domaine

Structs et enums Rust représentant les entités métier. Aucune logique, seulement des données.

#### Entités principales

```
Subscription
├── id: Option<i64>
├── name: String
├── provider: String
├── amount: f64              (montant en euros)
├── frequency: Frequency     (Monthly | Yearly | Quarterly)
├── monthly_cost: f64        (coût mensuel calculé)
├── is_bundle: bool
├── is_family_plan: bool
├── payment_source: Option<String>
├── start_date: Option<NaiveDate>
├── renewal_date: Option<NaiveDate>
├── status: Status           (Active | Archived)
├── notes: Option<String>
├── created_at: NaiveDateTime
└── updated_at: NaiveDateTime

BundleComponent
├── id: Option<i64>
├── subscription_id: i64
├── name: String
├── need_id: i64
├── individual_price: Option<f64>
└── allocated_cost: Option<f64>

Need
├── id: Option<i64>
├── name: String
├── family: String           (ex: "Divertissement", "Productivité")
└── essential: bool

FamilyMember
├── id: Option<i64>
├── name: String
└── created_at: NaiveDateTime

UsageRating
├── subscription_id: i64
├── component_name: Option<String>
├── member_id: i64
└── rating: Rating           (Heavy | Occasional | Rare | Never)

Alternative
├── name: String
├── covers_need: String
├── price_monthly: f64
├── has_family_plan: bool
├── family_price_monthly: Option<f64>
├── european: bool
├── open_source: bool
├── self_hostable: bool
├── data_location: String
├── migration_effort: MigrationEffort  (Low | Medium | High)
├── url: String
└── notes: Option<String>
```

#### Enums

```rust
enum Frequency { Monthly, Yearly, Quarterly }
enum Status { Active, Archived }
enum Rating { Heavy, Occasional, Rare, Never }
enum MigrationEffort { Low, Medium, High }
```

#### Relations entre entités

```
FamilyMember ──┐
               │ N:M
               ▼
Subscription ◄──── UsageRating
    │
    ├── 1:N ──→ BundleComponent ──→ Need
    │
    └── N:M ──→ Need  (via subscription_needs pour les non-bundles)

Need ◄──── Alternative (lien logique via covers_need = need.name)
```

### 3.5 `db/` -- Couche de persistance

#### `db/migration.rs`

Création et migration du schéma SQLite. Le schéma comporte 7 tables :

| Table | Rôle |
|---|---|
| `family_members` | Membres du foyer |
| `needs` | Besoins (streaming musique, cloud, etc.) |
| `subscriptions` | Abonnements avec tous leurs attributs |
| `bundle_components` | Composantes d'un bundle avec besoin et coût réparti |
| `subscription_needs` | Lien N:M abonnement-besoin (hors bundles) |
| `usage_ratings` | Usage par membre et par service/composante |
| `price_history` | Historique des changements de prix |

Stratégie : vérification de la version du schéma au démarrage, migration incrémentale si nécessaire.

#### `db/queries.rs`

Opérations CRUD pour chaque entité. Utilise `rusqlite::Connection` en synchrone.

Pattern type pour une requête :
```rust
pub fn list_subscriptions(conn: &Connection) -> Result<Vec<Subscription>> {
    let mut stmt = conn.prepare("SELECT ... FROM subscriptions WHERE status = 'active'")?;
    let rows = stmt.query_map([], |row| { /* mapping row → Subscription */ })?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}
```

Les tests de la couche `db/` utilisent `Connection::open_in_memory()` pour des tests rapides et isolés.

### 3.6 `services/` -- Logique métier

Chaque service encapsule une règle métier indépendante.

| Module | Responsabilité |
|---|---|
| `catalog.rs` | Chargement et recherche dans `catalog.toml` (auto-complétion, pré-remplissage) |
| `duplicates.rs` | Détection des doublons : même besoin couvert par 2+ services, pondération par usage |
| `recommendations.rs` | Matching alternatives par besoin, tri par pertinence (EU > open source > self-hosted), analyse d'impact bundle |
| `projections.rs` | Calcul des 4 scénarios d'économies sur 12 mois (doublons, EU, open source, combiné) en tenant compte des dates de renouvellement |
| `report.rs` | Agrégation des données pour le récapitulatif mensuel (total, évolution, alertes) |

Les services reçoivent une référence `&Connection` et retournent des structures de résultat via `anyhow::Result`.

### 3.7 `ui/` -- Interface utilisateur

Chaque fichier correspond à un écran de l'application :

| Module | Écran |
|---|---|
| `home.rs` | Dashboard -- graphiques, indicateurs clés, répartition des dépenses |
| `subscriptions.rs` | Liste CRUD des abonnements, formulaire d'ajout/édition |
| `needs.rs` | Tableau croisé besoins x services avec coûts et marquage doublons |
| `duplicates.rs` | Recouvrements détectés avec recommandations |
| `recommendations.rs` | Alternatives EU/open source par besoin |
| `projections.rs` | 4 scénarios d'économies sur 12 mois |
| `monthly_report.rs` | Récapitulatif mensuel (total, évolution, prochains renouvellements) |
| `family.rs` | Gestion des membres et matrice d'usage |
| `quickstart.rs` | Parcours premier lancement (3 abonnements -> valeur immédiate) |

Chaque module UI expose :
- Une fonction de rendu `render(frame: &mut Frame, area: Rect, state: &AppState)`
- Un handler d'événements clavier spécifique à l'écran

Widgets Ratatui utilisés : `Table`, `List`, `BarChart`, `Paragraph`, `Block`, `Tabs`.

---

## 4. Flux de données

### 4.1 Boucle principale

```
             ┌──────────────────────────────────────┐
             │                                      │
             ▼                                      │
  crossterm::event::poll()                          │
             │                                      │
             ▼                                      │
  app.handle_event(event)                           │
     ├── navigation globale (Tab, q, ?)             │
     └── dispatch vers écran actif                  │
             │                                      │
             ▼                                      │
  [écran].handle_key(key, &conn)                    │
     └── appel services si action métier            │
         └── services appellent db/queries          │
             │                                      │
             ▼                                      │
  terminal.draw(|frame| app.render(frame))          │
             │                                      │
             └──────────────────────────────────────┘
```

### 4.2 Ajout d'un abonnement (exemple de flux complet)

```
1. Utilisateur : presse 'a' sur l'écran Subscriptions
2. ui/subscriptions.rs : affiche le formulaire de saisie
3. Utilisateur : tape "Apple" dans le champ nom
4. services/catalog.rs : recherche floue dans catalog.toml → résultats
5. ui/subscriptions.rs : affiche les suggestions d'auto-complétion
6. Utilisateur : sélectionne "Apple One Premium", valide
7. services/catalog.rs : retourne les données pré-remplies (prix, composantes, besoins)
8. ui/subscriptions.rs : remplit le formulaire, l'utilisateur ajuste et valide
9. db/queries.rs : INSERT subscription + bundle_components + subscription_needs
10. services/duplicates.rs : recalcule les doublons
11. ui/subscriptions.rs : retour à la liste mise à jour
```

---

## 5. Persistance

### 5.1 SQLite

Fichier unique `data/subtracker.db`. Schéma relationnel normalisé avec 7 tables et contraintes d'intégrité référentielle (FOREIGN KEY).

Diagramme entité-relation :

```
family_members
    │
    │ 1:N
    ▼
usage_ratings ◄── subscriptions ──→ price_history
                       │
              ┌────────┼────────┐
              │        │        │
              ▼        ▼        ▼
    bundle_components  │   subscription_needs
              │        │        │
              ▼        │        ▼
            needs ◄────┘     needs
```

### 5.2 Fichiers TOML

Deux fichiers de données statiques, éditables manuellement :

**`data/catalog.toml`** -- Catalogue de services connus
- Structure : tableau de `[[services]]` avec composantes optionnelles `[[services.components]]`
- Usage : auto-complétion et pré-remplissage lors de la saisie
- Pas de mise à jour automatique dans le MVP

**`data/alternatives.toml`** -- Base d'alternatives
- Structure : tableau de `[[alternatives]]`
- Attributs : prix, localisation (EU), open source, self-hostable, effort de migration
- Lien logique avec les besoins via le champ `covers_need`

---

## 6. Navigation et raccourcis clavier

### 6.1 Structure de navigation

Les écrans sont organisés en onglets navigables via `Tab` / `Shift+Tab` :

```
Dashboard → Abonnements → Besoins → Doublons → Alternatives → Projections → Récap mensuel → Famille
```

Le parcours Quick Start s'affiche au premier lancement (base vide) et n'apparaît pas dans les onglets.

### 6.2 Raccourcis globaux

| Touche | Action |
|---|---|
| `Tab` / `Shift+Tab` | Écran suivant / précédent |
| `q` | Quitter l'application |
| `?` | Afficher l'aide |

### 6.3 Raccourcis contextuels (dans les listes)

| Touche | Action |
|---|---|
| `↑` / `↓` | Naviguer dans la liste |
| `Enter` | Sélectionner / valider |
| `a` | Ajouter |
| `e` | Éditer |
| `d` | Supprimer / archiver |
| `s` | Rechercher dans le catalogue |
| `/` | Filtrer / rechercher |

---

## 7. Conventions de code

### 7.1 Codes couleur de l'interface

| Couleur | Signification |
|---|---|
| Rouge | Doublons, services jamais utilisés, alertes |
| Vert | Économies potentielles, alternatives moins chères |
| Bleu | Informations, labels EU |
| Jaune | Avertissements, usage occasionnel |

### 7.2 Formatage

- Indentation : 4 espaces
- Longueur max par ligne : 100 caractères (fichiers Rust)
- Encodage : UTF-8, fin de ligne LF
- Formatter : `cargo fmt` (configuration par défaut)
- Linter : `cargo clippy` (configuration par défaut)

---

## 8. Stratégie de test

- **Couche `db/`** : tests unitaires avec `Connection::open_in_memory()` pour chaque opération CRUD
- **Couche `services/`** : tests unitaires avec données en mémoire, validation des règles métier (détection doublons, calcul projections, matching alternatives)
- **Couche `models/`** : tests de sérialisation/désérialisation TOML pour les structures lues depuis les fichiers de configuration

---

## 9. Roadmap itérative

| Itération | Thème | Contenu |
|---|---|---|
| 1 | Fondations | Structure projet, modèle SQLite, migrations, lecture TOML, shell TUI, CRUD abonnements |
| 2 | Intelligence | Besoins, bundles, vue par besoin, détection doublons, famille, matrice d'usage |
| 3 | Valeur | Recommandations alternatives, analyse impact bundles, dashboard graphiques, projections, récap mensuel |
| 4 | Onboarding | Quick start, auto-complétion catalogue, pré-remplissage |

### Post-MVP (v0.2+)

Extensions planifiées : import relevés bancaires (CSV/OFX), import PayPal, import emails (IMAP), synchronisation régulière, alertes de prix, suivi des économies réalisées, export (Markdown/PDF), intégration SLM local (Ollama), notifications de renouvellement.

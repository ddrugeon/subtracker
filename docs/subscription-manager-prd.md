# SubTracker — Product Requirements Document

## 1. Présentation du projet

### 1.1 Résumé

SubTracker est une application en ligne de commande (TUI) permettant de piloter les abonnements numériques d'un foyer. L'application analyse les dépenses par besoin réel, détecte les doublons et services sous-utilisés, et recommande des alternatives européennes et open source.

### 1.2 Problème

Un foyer cumule une trentaine d'abonnements numériques répartis entre divertissement, productivité, développement, apprentissage et autres. Les bundles (type Apple One Premium à 34,99€/mois) masquent la réalité : certains services inclus ne sont jamais utilisés, d'autres font doublon avec des abonnements séparés. Sans outil de pilotage, il est impossible de savoir combien on paie réellement par besoin, ce qu'on utilise vraiment, et quelles alternatives existent.

### 1.3 Objectifs

- Centraliser tous les abonnements du foyer en un seul endroit
- Raisonner par **besoin** (musique, vidéo, cloud…) et non par service
- Identifier les doublons et le gaspillage
- Proposer des alternatives européennes et open source
- Projeter les économies potentielles sur 12 mois
- Fournir un récapitulatif mensuel pour piloter les décisions

### 1.4 Périmètre

- Usage strictement personnel / familial
- Pas de commercialisation, pas de multi-utilisateurs
- Fonctionne 100% en local, aucune dépendance cloud
- Philosophie KISS

### 1.5 Contexte d'apprentissage

Ce projet sert également de terrain d'apprentissage du langage Rust. Les choix techniques privilégient l'idiomatique Rust et l'exploration de l'écosystème.

---

## 2. Architecture technique

### 2.1 Stack

| Composant | Choix | Justification |
|---|---|---|
| Langage | Rust | Apprentissage + performance + écosystème CLI/TUI solide |
| Interface | TUI via Ratatui | Standard actuel des TUI Rust, bonne documentation, charts intégrés |
| Base de données | SQLite (via rusqlite ou sqlx) | Fichier unique, aucun serveur, backup trivial, KISS |
| Fichier d'alternatives | TOML (via toml crate + serde) | Lisible, éditable manuellement, idiomatique Rust |
| Sérialisation | serde + serde_derive | Standard Rust pour la sérialisation / désérialisation |

### 2.2 Structure du projet

```
subtracker/
├── Cargo.toml
├── README.md
├── data/
│   ├── subtracker.db              # Base SQLite
│   ├── alternatives.toml          # Base d'alternatives EU / open source
│   └── catalog.toml               # Catalogue de services connus
├── src/
│   ├── main.rs                    # Point d'entrée, initialisation
│   ├── app.rs                     # État de l'application et logique de navigation
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── home.rs                # Écran d'accueil / dashboard
│   │   ├── subscriptions.rs       # Liste et saisie des abonnements
│   │   ├── needs.rs               # Vue par besoin
│   │   ├── duplicates.rs          # Écran de détection des doublons
│   │   ├── recommendations.rs     # Alternatives et scénarios
│   │   ├── monthly_report.rs      # Récapitulatif mensuel
│   │   └── quickstart.rs          # Parcours quick start
│   ├── models/
│   │   ├── mod.rs
│   │   ├── subscription.rs        # Modèle abonnement
│   │   ├── need.rs                # Modèle besoin
│   │   ├── family_member.rs       # Modèle membre de la famille
│   │   ├── alternative.rs         # Modèle alternative
│   │   └── usage.rs               # Modèle usage par membre
│   ├── db/
│   │   ├── mod.rs
│   │   ├── migrations.rs          # Création / migration du schéma
│   │   └── queries.rs             # Requêtes SQL
│   ├── services/
│   │   ├── mod.rs
│   │   ├── catalog.rs             # Gestion du catalogue de services
│   │   ├── duplicates.rs          # Logique de détection des doublons
│   │   ├── recommendations.rs     # Moteur de recommandation
│   │   ├── projections.rs         # Calcul des projections d'économies
│   │   └── report.rs              # Génération du récap mensuel
│   └── config.rs                  # Configuration de l'application
└── tests/
    └── ...
```

### 2.3 Modèle de données (SQLite)

```sql
-- Membres du foyer
CREATE TABLE family_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Besoins
CREATE TABLE needs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,              -- ex: "Streaming musique"
    family TEXT NOT NULL,            -- ex: "Divertissement"
    essential BOOLEAN DEFAULT FALSE, -- besoin essentiel vs optionnel
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Abonnements
CREATE TABLE subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,              -- ex: "Apple One Premium"
    provider TEXT,                   -- ex: "Apple"
    amount REAL NOT NULL,            -- montant en euros
    frequency TEXT NOT NULL,         -- "monthly", "yearly", "quarterly"
    monthly_cost REAL NOT NULL,      -- coût mensuel calculé
    is_bundle BOOLEAN DEFAULT FALSE,
    is_family_plan BOOLEAN DEFAULT FALSE,
    payment_source TEXT,             -- "carte", "paypal", "prelevement", "apple"
    start_date DATE,
    renewal_date DATE,
    status TEXT DEFAULT 'active',    -- "active", "archived"
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Composantes d'un bundle
CREATE TABLE bundle_components (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    name TEXT NOT NULL,              -- ex: "Apple Music"
    need_id INTEGER NOT NULL REFERENCES needs(id),
    individual_price REAL,           -- prix si souscrit séparément
    allocated_cost REAL,             -- coût réparti dans le bundle
    UNIQUE(subscription_id, name)
);

-- Lien abonnement <-> besoin (pour les abonnements simples, non-bundle)
CREATE TABLE subscription_needs (
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    need_id INTEGER NOT NULL REFERENCES needs(id),
    PRIMARY KEY (subscription_id, need_id)
);

-- Usage par membre de la famille
CREATE TABLE usage_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    component_name TEXT,             -- NULL si abonnement simple, nom de la composante si bundle
    member_id INTEGER NOT NULL REFERENCES family_members(id),
    rating TEXT NOT NULL,            -- "heavy", "occasional", "rare", "never"
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(subscription_id, component_name, member_id)
);

-- Historique des prix (pour suivi des évolutions)
CREATE TABLE price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL REFERENCES subscriptions(id),
    amount REAL NOT NULL,
    frequency TEXT NOT NULL,
    recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 2.4 Format du fichier d'alternatives (TOML)

```toml
# alternatives.toml

[[alternatives]]
name = "Deezer Premium"
covers_need = "Streaming musique"
price_monthly = 11.99
has_family_plan = true
family_price_monthly = 17.99
european = true
open_source = false
self_hostable = false
data_location = "France"
migration_effort = "low"       # low, medium, high
url = "https://www.deezer.com"
notes = "Catalogue comparable à Spotify/Apple Music"

[[alternatives]]
name = "Qobuz"
covers_need = "Streaming musique"
price_monthly = 12.99
has_family_plan = true
family_price_monthly = 21.99
european = true
open_source = false
self_hostable = false
data_location = "France"
migration_effort = "low"
url = "https://www.qobuz.com"
notes = "Qualité audio Hi-Res, catalogue orienté musique classique et jazz"

[[alternatives]]
name = "Navidrome"
covers_need = "Streaming musique"
price_monthly = 0.0
has_family_plan = false
european = false
open_source = true
self_hostable = true
data_location = "Local"
migration_effort = "high"
url = "https://www.navidrome.org"
notes = "Nécessite une bibliothèque musicale locale. Compatible Subsonic API."

[[alternatives]]
name = "Infomaniak kDrive"
covers_need = "Stockage cloud"
price_monthly = 5.54
has_family_plan = false
european = true
open_source = false
self_hostable = false
data_location = "Suisse"
migration_effort = "medium"
url = "https://www.infomaniak.com/kdrive"
notes = "3 To de stockage, suite collaborative intégrée"

[[alternatives]]
name = "Nextcloud"
covers_need = "Stockage cloud"
price_monthly = 0.0
has_family_plan = false
european = true
open_source = true
self_hostable = true
data_location = "Local"
migration_effort = "high"
url = "https://nextcloud.com"
notes = "Auto-hébergeable, riche en plugins. Hébergé par plusieurs providers EU."

[[alternatives]]
name = "Jellyfin"
covers_need = "Streaming vidéo"
price_monthly = 0.0
has_family_plan = false
european = false
open_source = true
self_hostable = true
data_location = "Local"
migration_effort = "high"
url = "https://jellyfin.org"
notes = "Alternative open source à Plex. Nécessite une bibliothèque média locale."
```

### 2.5 Format du catalogue de services (TOML)

```toml
# catalog.toml — Services connus avec pré-remplissage

[[services]]
name = "Apple One Premium"
provider = "Apple"
amount = 34.99
frequency = "monthly"
is_bundle = true
is_family_plan = true

[[services.components]]
name = "Apple Music"
need = "Streaming musique"
individual_price = 10.99

[[services.components]]
name = "Apple TV+"
need = "Streaming vidéo"
individual_price = 9.99

[[services.components]]
name = "Apple Arcade"
need = "Gaming"
individual_price = 6.99

[[services.components]]
name = "iCloud 2 To"
need = "Stockage cloud"
individual_price = 9.99

[[services.components]]
name = "Apple Fitness+"
need = "Fitness"
individual_price = 9.99

[[services.components]]
name = "Apple News+"
need = "Presse en ligne"
individual_price = 12.99

[[services]]
name = "Netflix Standard"
provider = "Netflix"
amount = 13.49
frequency = "monthly"
is_bundle = false
is_family_plan = false
needs = ["Streaming vidéo"]

[[services]]
name = "Netflix Premium"
provider = "Netflix"
amount = 19.99
frequency = "monthly"
is_bundle = false
is_family_plan = true
needs = ["Streaming vidéo"]

[[services]]
name = "YouTube Premium Famille"
provider = "Google"
amount = 17.99
frequency = "monthly"
is_bundle = true
is_family_plan = true

[[services.components]]
name = "YouTube sans publicité"
need = "Streaming vidéo"
individual_price = 12.99

[[services.components]]
name = "YouTube Music"
need = "Streaming musique"
individual_price = 10.99

[[services]]
name = "Spotify Premium Famille"
provider = "Spotify"
amount = 17.99
frequency = "monthly"
is_bundle = false
is_family_plan = true
needs = ["Streaming musique"]

[[services]]
name = "Microsoft 365 Famille"
provider = "Microsoft"
amount = 99.00
frequency = "yearly"
is_bundle = true
is_family_plan = true

[[services.components]]
name = "Office (Word, Excel, PowerPoint)"
need = "Bureautique"
individual_price = 69.00

[[services.components]]
name = "OneDrive 1 To"
need = "Stockage cloud"
individual_price = 20.00

[[services]]
name = "Duolingo Plus"
provider = "Duolingo"
amount = 6.99
frequency = "monthly"
is_bundle = false
is_family_plan = false
needs = ["Apprentissage langues"]
```

---

## 3. Fonctionnalités MVP

### 3.1 Quick Start (F10)

**Écran d'accueil au premier lancement.** L'application détecte que la base est vide et propose le parcours quick start.

**Parcours :**

1. "Bienvenue dans SubTracker. Commençons par tes 3 plus gros abonnements."
2. Pour chaque abonnement :
   - Champ de saisie avec auto-complétion depuis le catalogue
   - Si trouvé dans le catalogue : pré-remplissage automatique (prix, besoins, composantes)
   - Si non trouvé : saisie manuelle (nom, prix, fréquence, besoin)
3. Après la saisie des 3 abonnements, affichage immédiat :
   - Coût annuel cumulé
   - Décomposition en besoins (si bundles)
   - Doublons détectés
   - Alternatives EU / open source les plus pertinentes
   - Économie estimée sur 12 mois
4. Call to action : "Ajoute le reste de tes abonnements pour une vue complète"

**Objectif :** prouver la valeur en moins de 5 minutes.

### 3.2 Saisie manuelle avec catalogue (F1 + F2)

**Ajout d'un abonnement :**

- Champ de recherche avec auto-complétion dans le catalogue (`catalog.toml`)
- Si le service est trouvé : tous les champs sont pré-remplis, l'utilisateur valide et ajuste si nécessaire (formule, prix réel…)
- Si le service n'est pas trouvé : formulaire de saisie complète
  - Nom du service
  - Montant et fréquence (mensuel / annuel / trimestriel)
  - Est-ce un bundle ? Si oui, saisie des composantes et de leurs besoins
  - Est-ce une offre famille ?
  - Besoin(s) couvert(s) (sélection dans la liste des besoins existants ou création d'un nouveau besoin)
  - Date de prochain renouvellement (optionnel)
  - Notes (optionnel)

**Édition et suppression :**

- Modification de tous les champs d'un abonnement existant
- Archivage (soft delete) avec conservation dans l'historique
- Suppression définitive

**Gestion du catalogue :**

- Le catalogue (`catalog.toml`) est un fichier éditable manuellement
- L'utilisateur peut l'enrichir avec ses propres services
- Pas de mise à jour automatique dans le MVP

### 3.3 Décomposition des bundles (F3)

Quand un abonnement est marqué comme bundle :

- Chaque composante est enregistrée avec son besoin associé
- Le coût est réparti entre les composantes :
  - Par défaut, au prorata du prix individuel de chaque composante
  - L'utilisateur peut ajuster manuellement la répartition
- La vue par besoin intègre chaque composante comme une source distincte de couverture du besoin

### 3.4 Vision par besoin (F4)

**Écran dédié** affichant un tableau croisé besoins × services :

```
┌─────────────────────────────────────────────────────────────────────┐
│ Mes besoins                                                         │
├──────────────────┬──────────────────────────────┬───────────────────┤
│ Besoin           │ Couvert par                  │ Coût mensuel      │
├──────────────────┼──────────────────────────────┼───────────────────┤
│ Streaming musiq. │ Apple One (Apple Music)      │ 10,99€            │
│                  │ YouTube Premium (YT Music)   │ 10,99€            │
│                  │                      Total → │ 21,98€ ⚠ DOUBLON │
├──────────────────┼──────────────────────────────┼───────────────────┤
│ Streaming vidéo  │ Apple One (Apple TV+)        │ 9,99€             │
│                  │ Netflix Premium              │ 19,99€            │
│                  │ YouTube Premium (YT)         │ 12,99€            │
│                  │                      Total → │ 42,97€            │
├──────────────────┼──────────────────────────────┼───────────────────┤
│ Stockage cloud   │ Apple One (iCloud 2To)       │ 9,99€             │
│                  │                      Total → │ 9,99€             │
└──────────────────┴──────────────────────────────┴───────────────────┘
```

**Fonctionnalités :**

- Tri par coût décroissant, par famille de besoin, ou alphabétique
- Marquage visuel des doublons (même besoin couvert par 2+ services)
- Indicateur essentiel / optionnel par besoin
- Navigation vers le détail d'un besoin (services, usage, alternatives)

### 3.5 Détection des doublons (F5)

**Écran dédié** listant les recouvrements détectés :

Pour chaque doublon :
- Le besoin concerné
- Les services qui le couvrent (avec détail si bundle)
- Le coût cumulé pour ce besoin
- L'usage de chaque service par les membres de la famille
- Recommandation : quel service garder, lequel résilier, et l'économie associée

**Logique de détection :**

- Un doublon est détecté dès qu'un même besoin est couvert par 2 services ou plus
- Les composantes de bundles sont prises en compte
- L'alerte est pondérée par l'usage : si un des deux services est marqué "jamais utilisé", le doublon est flaggé comme prioritaire

### 3.6 Évaluation de l'usage (F6)

Pour chaque abonnement (ou composante de bundle), chaque membre de la famille renseigne son usage :

- **Beaucoup** (heavy) — usage régulier
- **De temps en temps** (occasional) — usage ponctuel
- **Rarement** (rare) — usage très occasionnel
- **Jamais** (never) — aucun usage

L'interface permet de remplir l'usage sous forme de matrice :

```
┌────────────────────────────────┬──────────┬──────────┬──────────┐
│ Service                        │ David    │ Conjointe│ Enfant 1 │
├────────────────────────────────┼──────────┼──────────┼──────────┤
│ Apple Music (Apple One)        │ ██ Bcp   │ ██ Bcp   │ ▒▒ Occ.  │
│ Apple TV+ (Apple One)          │ ░░ Rare  │ ░░ Rare  │ ░░ Rare  │
│ Apple Arcade (Apple One)       │    Jamais│    Jamais│ ▒▒ Occ.  │
│ iCloud 2 To (Apple One)       │ ██ Bcp   │ ██ Bcp   │    Jamais│
│ Fitness+ (Apple One)           │    Jamais│    Jamais│    Jamais│
│ News+ (Apple One)              │    Jamais│    Jamais│    Jamais│
├────────────────────────────────┼──────────┼──────────┼──────────┤
│ Netflix Premium                │ ▒▒ Occ.  │ ██ Bcp   │ ██ Bcp   │
│ YouTube Premium (sans pub)     │ ██ Bcp   │ ▒▒ Occ.  │ ██ Bcp   │
│ YouTube Music (YT Premium)     │    Jamais│    Jamais│    Jamais│
└────────────────────────────────┴──────────┴──────────┴──────────┘
```

### 3.7 Recommandations d'alternatives (F7)

**Écran dédié** présentant pour chaque besoin les alternatives disponibles :

**Source de données :** fichier `alternatives.toml`

**Affichage par besoin :**

- Service actuel avec coût
- Liste des alternatives triées par pertinence :
  - Priorité aux alternatives européennes
  - Puis open source
  - Puis auto-hébergeables
- Pour chaque alternative : prix, badges (🇪🇺 EU, 🔓 Open source, 🏠 Self-hosted), écart de prix
- Économie estimée sur 12 mois si migration

**Analyse d'impact bundle :**

Quand une recommandation implique de quitter un service inclus dans un bundle :

- Liste des autres services du bundle qui seraient perdus
- Coût de reconstruction à la carte des services utilisés
- Comparaison : coût actuel du bundle vs coût de la reconstruction
- Verdict : "le bundle reste avantageux" ou "la reconstruction est plus intéressante"

### 3.8 Dashboards (F8 + F9)

**Écran principal** avec les visualisations suivantes (charts Ratatui) :

**Vue globale :**

- Coût mensuel total et coût annuel total
- Graphique en barres : répartition des dépenses par famille de besoin
- Indicateurs : nombre d'abonnements actifs, nombre de doublons détectés, taux d'utilisation

**Projection d'économies sur 12 mois :**

Quatre scénarios affichés côte à côte :

| Scénario | Description | Économie 12 mois |
|---|---|---|
| Suppression des doublons | Résilier les services redondants | xxx€ |
| Alternatives européennes | Migrer vers des alternatives EU | xxx€ |
| Full open source | Maximiser l'open source et l'auto-hébergement | xxx€ |
| Combiné | Appliquer les 3 leviers | xxx€ |

Chaque scénario prend en compte les dates de renouvellement réelles pour refléter l'économie réalisable dans les 12 prochains mois (et non théorique sur 12 mois pleins).

**Récapitulatif mensuel (F17) :**

Écran dédié présentant :

- Total dépensé sur le mois en cours
- Évolution par rapport au mois précédent (en % et en €)
- Prochains renouvellements à venir avec dates
- Doublons toujours actifs
- Services sous-utilisés (usage "rare" ou "jamais" par tous les membres)
- Économies potentielles si les recommandations étaient appliquées

---

## 4. Navigation TUI

### 4.1 Structure des écrans

```
[Dashboard]  ──→  Vue globale, graphiques, indicateurs clés
    │
[Abonnements]  ──→  Liste, ajout, édition, suppression
    │
[Besoins]  ──→  Vue croisée besoins × services
    │
[Doublons]  ──→  Recouvrements détectés et recommandations
    │
[Alternatives]  ──→  Recommandations EU / open source par besoin
    │
[Projections]  ──→  Scénarios d'économie sur 12 mois
    │
[Récap mensuel]  ──→  Rapport du mois en cours
    │
[Famille]  ──→  Gestion des membres et matrice d'usage
```

### 4.2 Raccourcis clavier

| Touche | Action |
|---|---|
| `Tab` / `Shift+Tab` | Naviguer entre les écrans |
| `↑` / `↓` | Naviguer dans les listes |
| `Enter` | Sélectionner / valider |
| `a` | Ajouter un abonnement |
| `e` | Éditer l'élément sélectionné |
| `d` | Supprimer / archiver |
| `s` | Rechercher dans le catalogue |
| `/` | Filtrer / rechercher |
| `q` | Quitter |
| `?` | Aide |

### 4.3 Principes d'interface

- Navigation fluide au clavier, pas de souris nécessaire
- Feedback immédiat : tout calcul (doublons, coûts, projections) est mis à jour en temps réel
- Codes couleur cohérents :
  - Rouge : doublons, services jamais utilisés, alertes
  - Vert : économies potentielles, alternatives moins chères
  - Bleu : informations, labels EU
  - Jaune : avertissements, usage occasionnel
- Largeur minimale du terminal : 120 colonnes pour un affichage optimal

---

## 5. Roadmap

### 5.1 MVP (v0.1)

Objectif : un outil fonctionnel utilisable au quotidien avec les vrais abonnements du foyer.

| # | Fonctionnalité | Priorité |
|---|---|---|
| F1 | Saisie manuelle d'un abonnement | Essentielle |
| F2 | Catalogue de services avec auto-complétion (TOML) | Essentielle |
| F3 | Décomposition des bundles en besoins | Essentielle |
| F4 | Vision des abonnements par besoin | Essentielle |
| F5 | Détection des doublons / recouvrements | Essentielle |
| F6 | Évaluation de l'usage par membre de la famille | Essentielle |
| F7 | Recommandations d'alternatives EU et open source (TOML) | Essentielle |
| F8 | Dashboard : répartition des dépenses (charts Ratatui) | Essentielle |
| F9 | Dashboard : projection d'économies sur 12 mois (4 scénarios) | Essentielle |
| F10 | Quick start : 3 abonnements → valeur immédiate | Essentielle |
| F17 | Récapitulatif mensuel (écran dédié) | Essentielle |
| F20 | Gestion des membres de la famille et matrice d'usage | Essentielle |

### 5.2 Post-MVP (v0.2+)

| # | Fonctionnalité | Détail |
|---|---|---|
| F11 | Import depuis relevés bancaires | Parse CSV/OFX, détection des transactions récurrentes |
| F12 | Import depuis PayPal | API REST PayPal, paiements récurrents |
| F13 | Import depuis emails | Accès IMAP, parsing des factures récurrentes |
| F14 | Import depuis Apple | Export données compte Apple |
| F15 | Synchronisation régulière des sources | Tâche de fond, détection nouveaux abonnements |
| F16 | Alertes de changement de prix | Détection via synchro, recalcul recommandations |
| F18 | Analyse d'impact résiliation de bundle | Simulation coût reconstruction à la carte |
| F19 | Suivi des économies réalisées | Historique résiliations, économie cumulée réelle |
| — | Export du récap mensuel | Markdown, PDF ou texte |
| — | Export des graphiques | Images PNG ou PDF |
| — | Intégration SLM local | Ollama + Mistral/Phi pour enrichir les recommandations |
| — | Notifications | Rappels de renouvellement (cron + notification système) |
| — | Détection automatique de changements de prix | Scraping ou API des services |

### 5.3 Découpage itératif du MVP

**Itération 1 — Fondations**

- Structure du projet Rust
- Modèle de données SQLite + migrations
- Lecture des fichiers TOML (catalogue + alternatives)
- Shell TUI avec Ratatui (navigation entre écrans vides)
- Saisie manuelle d'un abonnement (formulaire complet)
- Liste des abonnements (CRUD)

**Itération 2 — Intelligence**

- Gestion des besoins
- Décomposition des bundles
- Vue par besoin
- Détection des doublons
- Gestion des membres de la famille
- Matrice d'usage

**Itération 3 — Valeur**

- Recommandations d'alternatives (lecture du TOML, matching par besoin)
- Analyse d'impact des bundles
- Dashboard avec graphiques (barres, indicateurs)
- Projections d'économies (4 scénarios)
- Récapitulatif mensuel

**Itération 4 — Onboarding**

- Quick start (parcours premier lancement)
- Auto-complétion depuis le catalogue
- Pré-remplissage des champs
- Enrichissement du catalogue avec les services réels

---

## 6. Crates Rust recommandés

| Crate | Usage |
|---|---|
| `ratatui` | Framework TUI |
| `crossterm` | Backend terminal (événements clavier, rendu) |
| `rusqlite` | Interface SQLite (ou `sqlx` pour de l'async) |
| `serde` + `serde_derive` | Sérialisation / désérialisation |
| `toml` | Lecture des fichiers TOML |
| `chrono` | Manipulation des dates |
| `fuzzy-matcher` | Auto-complétion floue pour le catalogue |
| `tui-input` | Widget de saisie de texte pour Ratatui |
| `directories` | Chemins standards pour les données utilisateur |
| `anyhow` | Gestion d'erreurs simplifiée |
| `clap` | Parsing des arguments CLI (si options de lancement) |

---

## 7. Contraintes et décisions

| Décision | Justification |
|---|---|
| TUI plutôt que web app | Pas de serveur à maintenir, usage local, cohérent avec la philosophie KISS |
| Rust plutôt que Go | Contexte d'apprentissage, écosystème TUI riche |
| SQLite plutôt que PostgreSQL | Fichier unique, aucun serveur, adapté au volume de données |
| TOML plutôt que JSON/YAML | Lisible, éditable manuellement, idiomatique Rust |
| Saisie manuelle uniquement (MVP) | Simplifie le MVP, les imports auto viendront ensuite |
| Pas de LLM dans le MVP | Le moteur de recommandation est déterministe, le SLM enrichira post-MVP |
| Données 100% locales | Aucune dépendance cloud, souveraineté des données personnelles |

use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Catalog {
    pub services: Vec<CatalogService>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CatalogService {
    pub name: String,
    pub provider: String,
    pub amount: f64,
    pub frequency: String,
    pub is_bundle: bool,
    pub is_family_plan: bool,
    pub needs: Vec<String>,
    #[serde(default)]
    pub components: Vec<CatalogComponent>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CatalogComponent {
    pub name: String,
    pub need: String,
    pub individual_price: f64,
}

fn load_catalog(path: &std::path::Path) -> Result<Catalog> {
    let content = std::fs::read_to_string(path)?;
    load_catalog_from_string(&content)
}

fn load_catalog_from_string(catalog: &str) -> Result<Catalog> {
    let catalog: Catalog = toml::from_str(catalog)?;
    Ok(catalog)
}

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

        let catalog = load_catalog_from_string(toml_content).unwrap();

        assert!(catalog.services.len() == 1);
        for service in &catalog.services {
            assert_eq!(service.name, "Netflix Standard");
            assert_eq!(service.provider, "Netflix");
            assert_eq!(service.amount, 13.49);
            assert_eq!(service.frequency, "monthly");
            assert_eq!(service.is_bundle, false);
            assert_eq!(service.is_family_plan, false);
            assert_eq!(service.needs.len(), 1);
            assert_eq!(service.needs[0], "Streaming vidéo");
        }
        // Parser et vérifier les champs
    }

    #[test]
    fn test_parse_bundle_with_components() {
        let toml_content = r#"
            [[services]]
            name = "Apple One"
            provider = "Apple"
            amount = 19.95
            frequency = "monthly"
            is_bundle = true
            is_family_plan = false
            needs = ["Streaming musique", "Streaming vidéo", "Jeux vidéo"]

            [[services.components]]
            name = "Apple Music"
            need = "Streaming musique"
            individual_price = 5.99

            [[services.components]]
            name = "Apple TV+"
            need = "Streaming vidéo"
            individual_price = 9.99

            [[services.components]]
            name = "Apple Arcade"
            need = "Jeux vidéo"
            individual_price = 6.99

        "#;

        let catalog = load_catalog_from_string(toml_content).unwrap();

        assert_eq!(catalog.services.len(), 1);
        let service = &catalog.services[0];
        assert_eq!(service.name, "Apple One");
        assert_eq!(service.is_bundle, true);
        assert_eq!(service.components.len(), 3);
        assert_eq!(service.components[0].name, "Apple Music");
        assert_eq!(service.components[0].need, "Streaming musique");
        assert_eq!(service.components[0].individual_price, 5.99);
        assert_eq!(service.components[1].name, "Apple TV+");
        assert_eq!(service.components[1].need, "Streaming vidéo");
        assert_eq!(service.components[1].individual_price, 9.99);
        assert_eq!(service.components[2].name, "Apple Arcade");
        assert_eq!(service.components[2].need, "Jeux vidéo");
        assert_eq!(service.components[2].individual_price, 6.99);
    }

    #[test]
    fn test_parse_multiple_services() {
        // Parser un TOML avec 3 services
        let toml_content = r#"
            [[services]]
            name = "Apple One"
            provider = "Apple"
            amount = 19.95
            frequency = "monthly"
            is_bundle = true
            is_family_plan = false
            needs = ["Streaming musique", "Streaming vidéo", "Jeux vidéo"]

            [[services.components]]
            name = "Apple Music"
            need = "Streaming musique"
            individual_price = 5.99

            [[services.components]]
            name = "Apple TV+"
            need = "Streaming vidéo"
            individual_price = 9.99

            [[services.components]]
            name = "Apple Arcade"
            need = "Jeux vidéo"
           individual_price = 6.99

            [[services]]
            name = "Netflix Standard"
            provider = "Netflix"
            amount = 13.49
            frequency = "monthly"
            is_bundle = false
            is_family_plan = false
            needs = ["Streaming vidéo"]
        "#;

        let catalog = load_catalog_from_string(toml_content).unwrap();

        assert_eq!(catalog.services.len(), 2);
        assert_eq!(&catalog.services[0].name, "Apple One");
        assert_eq!(catalog.services[0].is_bundle, true);
        assert_eq!(catalog.services[0].components.len(), 3);
        assert_eq!(&catalog.services[0].components[0].name, "Apple Music");
        assert_eq!(&catalog.services[0].components[0].need, "Streaming musique");
        assert_eq!(catalog.services[0].components[0].individual_price, 5.99);
        assert_eq!(&catalog.services[0].components[1].name, "Apple TV+");
        assert_eq!(&catalog.services[0].components[1].need, "Streaming vidéo");
        assert_eq!(catalog.services[0].components[1].individual_price, 9.99);
        assert_eq!(&catalog.services[0].components[2].name, "Apple Arcade");
        assert_eq!(&catalog.services[0].components[2].need, "Jeux vidéo");
        assert_eq!(catalog.services[0].components[2].individual_price, 6.99);
        assert_eq!(&catalog.services[1].name, "Netflix Standard");
        assert_eq!(&catalog.services[1].provider, "Netflix");
        assert_eq!(catalog.services[1].amount, 13.49);
        assert_eq!(&catalog.services[1].frequency, "monthly");
        assert_eq!(catalog.services[1].is_bundle, false);
        assert_eq!(catalog.services[1].is_family_plan, false);
        assert_eq!(catalog.services[1].needs.len(), 1);
        assert_eq!(&catalog.services[1].needs[0], "Streaming vidéo");
        assert_eq!(catalog.services[1].components.len(), 0);
    }

    #[test]
    fn test_parse_empty_catalog() {
        let toml_content = r#"
            services = []
        "#;

        let catalog = load_catalog_from_string(toml_content).unwrap();

        assert_eq!(catalog.services.len(), 0);
    }

    #[test]
    fn test_parse_invalid_toml_returns_error() {
        // Un TOML mal formé → Result::Err
        let bad_toml = "ceci n'est pas du TOML valide [[[";
        // Vérifier que le résultat est une erreur
        let catalog = load_catalog_from_string(bad_toml);

        assert_eq!(catalog.is_ok(), false);
    }

    #[test]
    fn test_parse_missing_required_field() {
        // Un service sans le champ "name" → erreur de désérialisation
        let toml_content = r#"
            [[services]]
            provider = "Apple"
            amount = 19.95
            frequency = "monthly"
            is_bundle = true
            is_family_plan = false
            needs = ["Streaming musique", "Streaming vidéo", "Jeux vidéo"]

            [[services.components]]
            name = "Apple Music"
            need = "Streaming musique"
            individual_price = 5.99

            [[services.components]]
            name = "Apple TV+"
            need = "Streaming vidéo"
            individual_price = 9.99

            [[services.components]]
            name = "Apple Arcade"
            need = "Jeux vidéo"
            individual_price = 6.99

        "#;

        let catalog = load_catalog_from_string(toml_content);

        assert!(catalog.is_err());
    }

    #[test]
    fn test_load_catalog_from_file() {
        // Créer un fichier TOML temporaire (avec std::fs::write dans un tempdir)
        // Appeler load_catalog(path)
        // Vérifier que le résultat est correct
        use std::io::Write;
        use tempfile::NamedTempFile;
        let mut file = NamedTempFile::new().unwrap();
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

        writeln!(file, "{}", toml_content).unwrap();

        let catalog = load_catalog(file.path()).unwrap();
        assert_eq!(catalog.services.len(), 1);
        let service = &catalog.services[0];
        assert_eq!(service.name, "Netflix Standard");
        assert_eq!(service.provider, "Netflix");
        assert_eq!(service.amount, 13.49);
        assert_eq!(service.frequency, "monthly");
        assert_eq!(service.is_bundle, false);
        assert_eq!(service.is_family_plan, false);
        assert_eq!(service.needs.len(), 1);
        assert_eq!(service.needs[0], "Streaming vidéo");
    }

    #[test]
    fn test_load_catalog_file_not_found() {
        // Appeler load_catalog avec un chemin inexistant
        // Vérifier que ça retourne une erreur (pas un panic)
        use std::path::Path;
        let path = Path::new("./foo/bar.toml");
        let catalog = load_catalog(path);

        assert!(catalog.is_err());
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Alternatives {
    pub services: Vec<AlternativeService>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct AlternativeService {
    name: String,
    covers_need: String,
    price_monthly: f64,
    has_family_plan: bool,
    family_price_monthly: Option<f64>,
    european: bool,
    open_source: bool,
    self_hostable: bool,
    data_location: Option<String>,
    migration_effort: String,
    url: String,
    notes: Option<String>,
}

fn load_alternatives(path: &std::path::Path) -> Result<Alternatives> {
    let content = std::fs::read_to_string(path)?;
    load_alternatives_from_string(&content)
}

fn load_alternatives_from_string(alternatives_toml: &str) -> Result<Alternatives> {
    let alternatives: Alternatives = toml::from_str(alternatives_toml)?;
    Ok(alternatives)
}

fn filter_by_need<'a>(alternatives: &'a Alternatives, need: &str) -> Vec<&'a AlternativeService> {
    alternatives
        .services
        .iter()
        .filter(|a| a.covers_need == need)
        .collect()
}

#[cfg(test)]
mod alternative_tests {
    use super::*;

    #[test]
    fn test_parse_alternative_european() {
        // Parser une alternative européenne (Deezer)
        // Vérifier : european = true, open_source = false, self_hostable = false
        let toml_content = r#"
            [[services]]
            name = "Deezer"
            covers_need = "Streaming musique"
            price_monthly = 9.99
            has_family_plan = false
            european = true
            open_source = false
            self_hostable = false
            data_location = "France"
            migration_effort = "low"
            url = "https://deezer.com"
        "#;
        let alternatives = load_alternatives_from_string(toml_content).unwrap();

        assert!(alternatives.services.len() == 1);
        let service = &alternatives.services[0];
        assert_eq!(service.name, "Deezer");
        assert_eq!(service.covers_need, "Streaming musique");
        assert_eq!(service.price_monthly, 9.99);
        assert!(!service.has_family_plan);
        assert!(service.european);
        assert!(!service.open_source);
        assert!(!service.self_hostable);
        assert_eq!(service.data_location, Some("France".into()));
        assert_eq!(service.migration_effort, "low");
        assert_eq!(service.url, "https://deezer.com");
    }

    #[test]
    fn test_parse_alternative_open_source() {
        // Parser une alternative open source (Navidrome)
        let toml_content = r#"
            [[services]]
            name = "Navidrome"
            covers_need = "Streaming musique"
            price_monthly = 0.0
            has_family_plan = false
            european = true
            open_source = true
            self_hostable = true
            url = "https://navidrome.org"
            migration_effort = "medium"
        "#;
        let alternatives = load_alternatives_from_string(toml_content).unwrap();

        assert!(alternatives.services.len() == 1);
        let service = &alternatives.services[0];
        assert_eq!(service.name, "Navidrome");
        assert_eq!(service.covers_need, "Streaming musique");
        assert_eq!(service.price_monthly, 0.00);
        assert!(!service.has_family_plan);
        assert!(service.european);
        assert!(service.open_source);
        assert!(service.self_hostable);
        assert_eq!(service.url, "https://navidrome.org");
        // Vérifier : open_source = true, self_hostable = true, price = 0.0
    }

    #[test]
    fn test_parse_alternative_with_family_plan() {
        // Parser une alternative avec offre famille
        // Vérifier que family_price_monthly est Some(prix)
        //
        let toml_content = r#"
            [[services]]
            name = "Deezer"
            covers_need = "Streaming musique"
            price_monthly = 9.99
            has_family_plan = true
            family_price_monthly = 14.99
            european = true
            open_source = false
            self_hostable = false
            data_location = "France"
            migration_effort = "low"
            url = "https://deezer.com"
        "#;
        let alternatives = load_alternatives_from_string(toml_content).unwrap();

        assert!(alternatives.services.len() == 1);
        let service = &alternatives.services[0];
        assert_eq!(service.name, "Deezer");
        assert_eq!(service.covers_need, "Streaming musique");
        assert_eq!(service.price_monthly, 9.99);
        assert!(service.has_family_plan);
        assert_eq!(service.family_price_monthly, Some(14.99));
        assert!(service.european);
        assert!(!service.open_source);
        assert!(!service.self_hostable);
        assert_eq!(service.data_location, Some("France".into()));
        assert_eq!(service.migration_effort, "low");
        assert_eq!(service.url, "https://deezer.com");
    }

    #[test]
    fn test_parse_alternative_without_family_plan() {
        // Parser une alternative sans offre famille
        // Vérifier que family_price_monthly est None
        let toml_content = r#"
            [[services]]
            name = "Deezer"
            covers_need = "Streaming musique"
            price_monthly = 9.99
            has_family_plan = false
            european = true
            open_source = false
            self_hostable = false
            data_location = "France"
            migration_effort = "low"
            url = "https://deezer.com"
        "#;
        let alternatives = load_alternatives_from_string(toml_content).unwrap();

        assert!(alternatives.services.len() == 1);
        let service = &alternatives.services[0];
        assert_eq!(service.name, "Deezer");
        assert_eq!(service.covers_need, "Streaming musique");
        assert_eq!(service.price_monthly, 9.99);
        assert!(!service.has_family_plan);
        assert!(service.family_price_monthly.is_none());
        assert!(service.european);
        assert!(!service.open_source);
        assert!(!service.self_hostable);
        assert_eq!(service.data_location, Some("France".into()));
        assert_eq!(service.migration_effort, "low");
        assert_eq!(service.url, "https://deezer.com");
    }

    #[test]
    fn test_alternatives_grouped_by_need() {
        let toml_content = r#"
            [[services]]
            name = "Deezer"
            covers_need = "Streaming musique"
            price_monthly = 9.99
            has_family_plan = true
            family_price_monthly = 14.99
            european = true
            open_source = false
            self_hostable = false
            migration_effort = "low"
            url = "https://deezer.com"

            [[services]]
            name = "Qobuz"
            covers_need = "Streaming musique"
            price_monthly = 12.99
            has_family_plan = false
            european = true
            open_source = false
            self_hostable = false
            migration_effort = "low"
            url = "https://qobuz.com"

            [[services]]
            name = "Navidrome"
            covers_need = "Streaming musique"
            price_monthly = 0.0
            has_family_plan = false
            european = false
            open_source = true
            self_hostable = true
            migration_effort = "medium"
            url = "https://navidrome.org"

            [[services]]
            name = "Jellyfin"
            covers_need = "Streaming vidéo"
            price_monthly = 0.0
            has_family_plan = false
            european = false
            open_source = true
            self_hostable = true
            migration_effort = "high"
            url = "https://jellyfin.org"
        "#;

        let alternatives = load_alternatives_from_string(toml_content).unwrap();
        let musique = filter_by_need(&alternatives, "Streaming musique");

        assert_eq!(musique.len(), 3);
        assert_eq!(musique[0].name, "Deezer");
        assert_eq!(musique[1].name, "Qobuz");
        assert_eq!(musique[2].name, "Navidrome");

        let video = filter_by_need(&alternatives, "Streaming vidéo");
        assert_eq!(video.len(), 1);
        assert_eq!(video[0].name, "Jellyfin");
    }
}

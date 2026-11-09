use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn tarea_de_ejemplo() -> Task {
        let fecha = Utc.with_ymd_and_hms(2026, 1, 15, 10, 30, 0).unwrap();

        Task {
            id: Uuid::nil(),
            title: "Comprar pan".to_string(),
            description: Some("Integral".to_string()),
            completed: false,
            created_at: fecha,
            updated_at: fecha,
        }
    }

    #[test]
    fn serializa_con_claves_camel_case() {
        let json = serde_json::to_value(tarea_de_ejemplo()).unwrap();
        let objeto = json.as_object().unwrap();

        for clave in [
            "id",
            "title",
            "description",
            "completed",
            "createdAt",
            "updatedAt",
        ] {
            assert!(objeto.contains_key(clave), "falta la clave {clave}");
        }

        assert!(!objeto.contains_key("created_at"));
        assert!(!objeto.contains_key("updated_at"));
    }

    #[test]
    fn serializa_fechas_en_rfc3339() {
        let json = serde_json::to_value(tarea_de_ejemplo()).unwrap();
        let creada = json["createdAt"].as_str().unwrap();

        assert!(creada.contains('T'), "la fecha debe usar T: {creada}");
        assert!(creada.ends_with('Z'), "la fecha debe ser UTC: {creada}");
        assert_eq!(json["createdAt"], json["updatedAt"]);
    }

    #[test]
    fn description_none_se_serializa_como_null() {
        let mut tarea = tarea_de_ejemplo();
        tarea.description = None;

        let json = serde_json::to_value(tarea).unwrap();

        assert!(json["description"].is_null());
    }
}

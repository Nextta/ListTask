use crate::task::Task;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl CreateTask {
    pub fn new(title: String, description: Option<String>) -> CreateTask {
        CreateTask { title, description }
    }

    pub fn into_task(self) -> Task {
        let time: DateTime<Utc> = Utc::now();

        Task {
            id: Uuid::new_v4(),
            title: self.title,
            description: self.description,
            completed: false,
            created_at: time,
            updated_at: time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializa_create_task_completo() {
        let dto: CreateTask =
            serde_json::from_str(r#"{"title":"Comprar pan","description":"Integral"}"#).unwrap();

        assert_eq!(dto.title, "Comprar pan");
        assert_eq!(dto.description.as_deref(), Some("Integral"));
    }

    #[test]
    fn deserializa_create_task_sin_description() {
        let dto: CreateTask = serde_json::from_str(r#"{"title":"Comprar pan"}"#).unwrap();

        assert_eq!(dto.title, "Comprar pan");
        assert!(dto.description.is_none());
    }

    #[test]
    fn create_task_sin_title_falla() {
        let resultado = serde_json::from_str::<CreateTask>(r#"{"description":"Integral"}"#);

        assert!(resultado.is_err(), "el título es obligatorio");
    }

    #[test]
    fn create_task_ignora_campos_desconocidos() {
        let dto: CreateTask = serde_json::from_str(
            r#"{
                "title": "Comprar pan",
                "id": "00000000-0000-0000-0000-000000000000",
                "createdAt": "2026-01-15T10:30:00Z",
                "updatedAt": "2026-01-15T10:30:00Z"
            }"#,
        )
        .unwrap();

        assert_eq!(dto.title, "Comprar pan");
        assert!(dto.description.is_none());
    }

    #[test]
    fn update_task_vacio_deja_todo_en_none() {
        let dto: UpdateTask = serde_json::from_str("{}").unwrap();

        assert!(dto.title.is_none());
        assert!(dto.description.is_none());
        assert!(dto.completed.is_none());
    }

    #[test]
    fn update_task_parcial_solo_completed() {
        let dto: UpdateTask = serde_json::from_str(r#"{"completed":true}"#).unwrap();

        assert_eq!(dto.completed, Some(true));
        assert!(dto.title.is_none());
        assert!(dto.description.is_none());
    }

    #[test]
    fn update_task_description_null_equivale_a_no_cambiar() {
        let dto: UpdateTask = serde_json::from_str(r#"{"description":null}"#).unwrap();

        assert!(dto.description.is_none());
    }

    #[test]
    fn update_task_completo() {
        let dto: UpdateTask = serde_json::from_str(
            r#"{"title":"Nuevo","description":"Nueva","completed":false}"#,
        )
        .unwrap();

        assert_eq!(dto.title.as_deref(), Some("Nuevo"));
        assert_eq!(dto.description.as_deref(), Some("Nueva"));
        assert_eq!(dto.completed, Some(false));
    }

    #[test]
    fn into_task_copia_los_campos_y_marca_pendiente() {
        let dto = CreateTask::new("Comprar pan".to_string(), Some("Integral".to_string()));

        let tarea = dto.into_task();

        assert_eq!(tarea.title, "Comprar pan");
        assert_eq!(tarea.description.as_deref(), Some("Integral"));
        assert!(!tarea.completed);
        assert_eq!(tarea.created_at, tarea.updated_at);
    }

    #[test]
    fn into_task_genera_ids_distintos() {
        let primera = CreateTask::new("A".to_string(), None).into_task();
        let segunda = CreateTask::new("B".to_string(), None).into_task();

        assert_ne!(primera.id, segunda.id);
    }
}

# ListTask — Lista de tareas (Angular + Actix Web)

Aplicación web para crear, ver, editar, completar, filtrar y eliminar tareas.

El proyecto está dividido en dos mitades independientes que se comunican mediante un contrato HTTP/JSON:

- **Frontend:** Angular (aún no iniciado).
- **Backend:** Rust + Actix Web (en desarrollo).

---

## Objetivo

Una aplicación donde el usuario gestiona tareas: CRUD completo, filtros por estado, validación, manejo de errores y persistencia **en memoria**.

### Dentro del alcance (MVP)

- CRUD completo de tareas.
- Filtro por estado (completada / pendiente).
- Validación de entrada.
- Manejo de errores con códigos HTTP correctos.
- Datos en memoria (sin base de datos).

### Fuera del alcance (extras, al final)

- Usuarios y autenticación.
- Base de datos persistente.
- Drag and drop.

---

## Modelo de datos

Una tarea (`Task`) tiene los siguientes campos:

| Campo         | Tipo              | Notas                                   |
| ------------- | ----------------- | --------------------------------------- |
| `id`          | UUID              | Generado por el servidor (UUID v4).     |
| `title`       | string            | Obligatorio, no vacío.                  |
| `description` | string \| null    | Opcional.                               |
| `completed`   | bool              | Por defecto `false`.                    |
| `created_at`  | timestamp         | Fecha de creación (servidor).           |
| `updated_at`  | timestamp         | Fecha de última modificación (servidor).|

> Los timestamps se serializan en formato ISO-8601 / RFC 3339.

---

## Contrato de la API

Base URL en desarrollo: `http://127.0.0.1:8080`

| Método   | Ruta              | Función                                   | Respuestas esperadas |
| -------- | ----------------- | ----------------------------------------- | -------------------- |
| `GET`    | `/api/health`     | Comprobar que el servidor vive            | `200`                |
| `GET`    | `/api/tasks`      | Listar tareas (filtro opcional)           | `200`                |
| `POST`   | `/api/tasks`      | Crear tarea                               | `201`, `400`         |
| `GET`    | `/api/tasks/{id}` | Obtener una tarea                         | `200`, `404`         |
| `PATCH`  | `/api/tasks/{id}` | Modificar parcialmente                    | `200`, `400`, `404`  |
| `DELETE` | `/api/tasks/{id}` | Eliminar                                  | `204`, `404`         |

### Filtro de listado

`GET /api/tasks?completed=true` o `GET /api/tasks?completed=false` filtra por estado.
Sin el parámetro, devuelve todas las tareas.

### Formato de error

Los errores (`400`, `404`) deben devolver un cuerpo JSON consistente, por ejemplo:

```json
{
  "error": "Task not found"
}
```

### Notas de diseño

- `POST` y `PATCH` validan el cuerpo de la petición; datos inválidos → `400`.
- `PATCH` es una modificación parcial: solo se actualizan los campos enviados.
- `updated_at` se actualiza automáticamente en cada modificación.
- `DELETE` responde `204 No Content` sin cuerpo.

---

## Stack técnico

### Backend

| Crate       | Uso                                  |
| ----------- | ------------------------------------ |
| `actix-web` | Servidor HTTP y routing              |
| `actix-cors`| CORS para el frontend Angular        |
| `serde`     | Serialización / deserialización JSON |
| `serde_json`| JSON                                 |
| `uuid`      | IDs de tareas (v4)                   |
| `chrono`    | Fechas de creación / modificación    |
| `thiserror` | Tipos de error del dominio           |
| `dotenvy`   | Variables de entorno (`.env`)        |
| `env_logger`/`log` | Logging                       |
| `tokio`     | Runtime asíncrono                    |

Edición de Rust: **2024**.

### Frontend

- Angular (pendiente de inicializar).

---

## Estructura del proyecto

```
ListTask/
├── .gitignore
├── README.md
└── backend/
    ├── Cargo.toml
    └── src/
        ├── main.rs   # Arranque del servidor y registro de rutas
        └── api.rs    # Handlers y DTOs de la API
```

---

## Estado actual

- [x] Proyecto backend creado con Cargo.
- [x] Dependencias base añadidas.
- [x] `GET /api/health` implementado (respuesta de texto).
- [ ] Modelo `Task` y almacenamiento en memoria (`Mutex<Vec<Task>>` / `HashMap`).
- [ ] Implementar `GET /api/tasks` con filtro `completed`.
- [ ] Implementar `POST /api/tasks` con validación.
- [ ] Implementar `GET /api/tasks/{id}`.
- [ ] Implementar `PATCH /api/tasks/{id}`.
- [ ] Implementar `DELETE /api/tasks/{id}`.
- [ ] Errores JSON consistentes (`400`, `404`).
- [ ] Configurar CORS para Angular.
- [ ] Inicializar frontend Angular.
- [ ] Conectar frontend con la API.

---

## Cómo ejecutar el backend

Requisitos: Rust estable (con soporte para edition 2024).

```bash
cd backend
cargo run
```

El servidor escucha en `http://127.0.0.1:8080`.

Probar el health check:

```bash
curl http://127.0.0.1:8080/api/health
```

> Nota: el health check actualmente devuelve texto plano. Se recomienda migrarlo a JSON
> (`{"status":"ok"}`) cuando se unifique el formato de respuestas.

---

## Convenciones

- Rutas bajo el prefijo `/api`.
- JSON en `snake_case` (campos del modelo tal como se definen en Rust).
- Fechas en RFC 3339.
- Errores siempre como JSON con la clave `error`.
- Estado compartido mediante `web::Data` (envolviendo el almacenamiento en un `Mutex`/`RwLock`).
- Handlers delgados: la lógica de negocio vive en funciones/estructuras separadas.

---

## Próximos pasos sugeridos

1. Definir el modelo `Task` y los DTOs de entrada/salida.
2. Implementar el almacenamiento en memoria y el estado compartido de la app.
3. Implementar los endpoints del CRUD siguiendo el contrato.
4. Añadir validación y errores JSON consistentes.
5. Configurar CORS y probar con `curl`/Postman.
6. Inicializar Angular y construir la UI sobre el contrato ya establecido.

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

### Decisiones de la API (MVP)

- **PATCH parcial:** los campos omitidos no se modifican.
- **`description` en PATCH:** `null` y campo omitido se tratan igual (sin cambio).
  En el MVP la descripción solo se puede reemplazar, **no vaciar**. El frontend
  debe tenerlo en cuenta (si el usuario borra el texto, enviar `""` o no ofrecer
  la acción de borrar).
- **Campos desconocidos:** se ignoran silenciosamente, incluidos `id`,
  `createdAt` y `updatedAt`. Por tanto, el cliente no puede alterarlos.
- **Fechas:** las genera siempre el servidor; el cliente nunca las envía.
- **`id`:** se envía en la ruta (`/api/tasks/{id}`), nunca en el cuerpo. En
  `POST` lo genera el servidor.

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
        ├── main.rs   # Arranque del servidor
        ├── api.rs    # Handlers de la API (health)
        ├── dtos.rs   # DTOs de entrada (CreateTask, UpdateTask)
        └── task.rs   # Modelo Task
```

---

## Seguimiento de tareas

Leyenda: `[x]` hecho · `[ ]` pendiente.

### Backend (Rust + Actix Web)

- [x] **B1. Crear el proyecto y elegir dependencias**
  - Crates: `actix-web`, `serde` (derive), `serde_json`, `tokio`, `uuid` (v4 + serde), `chrono` (serde), `actix-cors`, `log`, `env_logger`. Opcionales: `thiserror`, `dotenvy`.
  - Cuidar las features al añadir cada crate.
  - ⚠️ Pendiente: instalar `cargo-watch` o `bacon`.
  - **Hecho cuando:** `cargo run` compila y arranca sin errores. ✅

- [x] **B2. Servidor mínimo con endpoint de salud**
  - `HttpServer` + `App` + `GET /api/health` devolviendo un JSON sencillo.
  - Entender `#[actix_web::main]` (arranca el runtime de Tokio) y que `App` se construye una vez por worker.
  - **Hecho cuando:** el endpoint responde desde el navegador o un cliente HTTP. ✅

- [x] **B3. Definir modelos y DTOs**
  - Modelo `Task` separado de los DTOs de entrada: el cliente no puede enviar `id` ni fechas.
  - `CreateTask` (título obligatorio) y `UpdateTask` (todos los campos `Option` por PATCH).
  - `camelCase` decidido con serde; `uuid` v4 para el id.
  - **Hecho cuando:** se serializa una tarea de ejemplo y se deserializa un cuerpo de creación. ✅ (cubierto por tests unitarios)

- [ ] **B4. Estado compartido en memoria**
  - Crear el almacén de tareas y compartirlo con `web::Data`.
  - Investigar `Mutex` vs `RwLock`; el `RwLock` de la std basta si no hay `.await` con el lock tomado; el de `tokio::sync` solo si el lock cruza un `.await`.
  - Trampa: crear el estado **fuera** del cierre de `HttpServer::new` (si no, cada worker tiene su copia).
  - `web::Data` ya envuelve en `Arc`: no doblarlo.
  - **Hecho cuando:** un handler de prueba lee y modifica el estado y se ve en peticiones posteriores.

- [ ] **B5. Organización modular**
  - Módulos: modelos, estado, handlers, errores y configuración de rutas.
  - `web::scope("/api")` y función de configuración con `web::ServiceConfig` + `.configure()`.
  - Hacerlo ahora: permite construir la `App` desde una función reutilizable para tests (B14).
  - **Hecho cuando:** `main.rs` solo arranca el servidor y el resto vive en módulos.

- [ ] **B6. Listar tareas (`GET /api/tasks`)**
  - Devolver todas ordenadas por fecha de creación con `web::Json`.
  - Clonar lo necesario y soltar el lock cuanto antes.
  - Añadir filtro opcional por completada con `web::Query`.
  - **Hecho cuando:** devuelve lista vacía al inicio y el filtro funciona.

- [ ] **B7. Crear tarea (`POST /api/tasks`)**
  - Recibir con `web::Json`, validar, generar id y fechas, guardar y responder.
  - Título no vacío tras `trim` y con longitud máxima razonable.
  - Responder `201 Created` con la tarea creada; repasar 200 vs 201.
  - **Hecho cuando:** la tarea aparece en el listado y un título vacío devuelve `400`.

- [ ] **B8. Obtener tarea por id (`GET /api/tasks/{id}`)**
  - Buscar por el id de la ruta con `web::Path`.
  - `404` si no existe; id con formato inválido → `400`.
  - **Hecho cuando:** devuelve la tarea o un `404` claro.

- [ ] **B9. Actualizar tarea (`PATCH /api/tasks/{id}`)**
  - Aplicar solo los campos presentes y actualizar `updated_at`.
  - Repasar PUT (reemplazo) vs PATCH (parcial) y por qué PATCH encaja al completar tareas.
  - Mismas validaciones que en la creación.
  - **Hecho cuando:** se puede cambiar solo el título, solo el estado, o ambos.

- [ ] **B10. Eliminar tarea (`DELETE /api/tasks/{id}`)**
  - Eliminar del almacén y responder sin cuerpo: `204`, o `404` si no existía.
  - **Hecho cuando:** la tarea desaparece del listado.

- [ ] **B11. Manejo de errores centralizado**
  - Tipo de error propio + `ResponseError` para que Actix lo convierta en respuesta HTTP.
  - Handlers con `Result` y `?`; `thiserror` reduce código repetitivo.
  - Cuerpo de error JSON uniforme (código y mensaje).
  - Configurar `JsonConfig`, `PathConfig` y `QueryConfig` con manejador propio (si no, los JSON malformados devuelven texto plano).
  - Limitar el tamaño máximo del cuerpo.
  - **Hecho cuando:** todos los errores, incluido un JSON roto, devuelven el mismo formato.

- [ ] **B12. Middleware: logging y CORS**
  - `middleware::Logger` + `env_logger` controlado por `RUST_LOG`.
  - `actix-cors` permitiendo solo `http://localhost:4200`, los métodos usados y las cabeceras necesarias (evitar modo permisivo).
  - Entender el preflight (`OPTIONS`). Los middlewares se ejecutan en orden inverso a `.wrap()`.
  - **Hecho cuando:** se ve cada petición en consola y el navegador no bloquea las llamadas desde Angular.

- [ ] **B13. Configuración por entorno**
  - Sacar host, puerto y origen permitido a variables de entorno con valores por defecto.
  - `std::env` o `dotenvy` para un `.env` (no subirlo al repositorio).
  - **Hecho cuando:** se cambia el puerto sin recompilar.

- [ ] **B14. Tests de integración**
  - Probar endpoints sin abrir puerto real con `actix_web::test` y su atributo de test asíncrono.
  - Casos mínimos: crear y listar, `404` en id inexistente, `400` con título vacío, borrar y comprobar.
  - **Hecho cuando:** `cargo test` pasa y cubre el CRUD.

- [ ] **B15. Persistencia (opcional)**
  - Simple: archivo JSON con `tokio::fs`, guardando en cada cambio o con `tokio::spawn` + `tokio::time::interval`.
  - Seria: SQLite con `sqlx` (proyecto aparte).
  - Investigar graceful shutdown para no perder datos.

### Frontend (Angular)

- [ ] **F1. Crear el proyecto**
  - Node LTS, Angular CLI, `ng new` con routing, CSS o SCSS y modo estricto.
  - Entender `main.ts`, `app.config.ts`, `app.routes.ts` y `angular.json`; instalar Angular DevTools.
  - **Hecho cuando:** `ng serve` muestra la página inicial.

- [ ] **F2. Estructura de carpetas**
  - Organizar por funcionalidad: `core/`, `features/tasks/`, `shared/`.
  - Seguir la guía de estilo oficial y generar con `ng generate`.

- [ ] **F3. Modelos TypeScript**
  - Interfaces para tarea, creación y actualización; tipo unión de literales para el filtro.
  - Fechas ISO: decidir si se convierten a `Date` o se dejan como texto (ser consistente).
  - Nombres en el `camelCase` decidido en B3.

- [ ] **F4. Conexión con el backend**
  - `provideHttpClient`; investigar `withFetch` y `withInterceptors`.
  - `proxy.conf.json` redirigiendo `/api` a Actix y enlazado en `angular.json` (evita CORS en desarrollo).
  - Rutas relativas (`/api/tasks`) y entornos con `ng generate environments`.
  - **Hecho cuando:** una llamada al health funciona desde Angular.

- [ ] **F5. Servicio de acceso a la API**
  - Un método por operación (listar, crear, actualizar, eliminar).
  - `inject()` y `Observable` tipados; recordar que `HttpClient` es frío hasta suscribirse.
  - Solo habla con la API: sin estado ni lógica de pantalla.

- [ ] **F6. Gestión del estado de la UI**
  - Decidir dónde viven lista, filtro, carga y error; se recomiendan signals (`signal`, `computed`, `effect`) en un servicio de estado.
  - Tras cada operación exitosa, actualizar la lista local en vez de recargarla.
  - Investigar `toSignal`, `httpResource` y `rxResource` según la versión.

- [ ] **F7. Componente contenedor de la lista**
  - Carga las tareas al iniciar y las pinta.
  - Control flow moderno: `@for` (con `track` obligatorio), `@if`, `@empty`.
  - Separar contenedor/presentacional y usar `ChangeDetectionStrategy.OnPush`.
  - **Hecho cuando:** las tareas del backend aparecen en pantalla.

- [ ] **F8. Componente de ítem de tarea**
  - Presentacional: checkbox y botones; `input()` y `output()` en vez de decoradores.
  - Emite eventos hacia arriba y no sabe nada de HTTP.

- [ ] **F9. Formulario de creación**
  - Reactive Forms (`FormBuilder`, `FormControl`, `Validators`).
  - Título obligatorio, longitud máxima y validador personalizado para títulos de solo espacios.
  - Errores solo si el campo fue tocado; deshabilitar mientras guarda, vaciar al terminar y devolver el foco.
  - **Hecho cuando:** se crea una tarea y aparece en la lista sin recargar.

- [ ] **F10. Completar y eliminar**
  - Checkbox → PATCH; botón borrar → DELETE.
  - Feedback visual (tachado en completadas) y estado "operación en curso" por ítem.
  - Pedir confirmación antes de borrar (diálogo nativo y luego, opcionalmente, Angular CDK).

- [ ] **F11. Editar tarea**
  - Cambiar el título en línea (doble clic) o en panel aparte, reutilizando la lógica del formulario.
  - Enter guarda, Escape cancela; cuidar el foco.

- [ ] **F12. Filtros y contadores**
  - Botones Todas / Pendientes / Completadas y contador de pendientes.
  - Derivar la lista filtrada con `computed`; decidir filtrado en cliente o con el parámetro del backend (B6).
  - Extra: sincronizar el filtro con los query params (`ActivatedRoute`).

- [ ] **F13. Estados de carga, vacío y error**
  - Indicador de carga, mensaje de lista vacía y error con botón de reintentar.
  - `catchError` y `finalize`; interceptor funcional (`HttpInterceptorFn`) que traduzca el error del backend (B11).
  - Probar apagando el backend a propósito.

- [ ] **F14. Estilos, diseño responsive y accesibilidad**
  - Flexbox/Grid y variables CSS, enfoque mobile-first.
  - Etiquetas asociadas a inputs, ARIA donde haga falta, foco visible y buen contraste.
  - Hacer el CSS a mano primero; Material o Tailwind como segunda iteración.

- [ ] **F15. Routing (opcional, formativo)**
  - `/tasks`, redirección desde la raíz, página 404 y detalle en `/tasks/:id` si se quiere.
  - Practicar `loadComponent` y parámetros de ruta como inputs del componente.

- [ ] **F16. Tests**
  - Servicio con `provideHttpClientTesting` + `HttpTestingController`; componentes con `TestBed`.
  - Probar servicio, filtrado y validaciones del formulario.
  - Consultar el runner según la versión de Angular (Jasmine/Karma o Vitest).

- [ ] **F17. Build de producción**
  - `ng build`; revisar carpeta de salida y avisos de tamaño (budgets).

### Integración y cierre

- [ ] **I1. Prueba end-to-end manual**
  - Recorrer todos los flujos con ambos lados corriendo, incluidos los fallos: título vacío, backend caído, id inexistente.

- [ ] **I2. Servir el frontend desde Actix (opcional)**
  - `actix-files` para servir la carpeta compilada.
  - Gestionar las rutas del router de Angular para que recargar no dé 404 (investigar `default_service`).

- [ ] **I3. README y retrospectiva**
  - Documentar arranque y endpoints. Anotar qué fue difícil y qué cambiarías.

### Planificación sugerida

| Sprint | Contenido | Esfuerzo estimado |
| --- | --- | --- |
| 1. Backend núcleo | B1 a B10 | 1 a 1,5 días |
| 2. Frontend núcleo | F1 a F10 | 1 a 1,5 días |
| 3. Robustez | B11 a B14, F11 a F13 | 1 día |
| 4. Pulido y cierre | F14 a F17, I1 a I3 | 0,5 a 1 día |
| Extras | B15, F15, drag and drop, dark mode | a tu ritmo |

### Riesgos habituales

- Crear el estado compartido dentro del cierre de `HttpServer::new` (B4).
- Mantener un lock durante un `.await`.
- Que backend y frontend no coincidan en `camelCase` o en el formato de fechas.
- Olvidar el `track` en `@for` o no tratar los errores HTTP.
- Perder tiempo con CORS: usar el proxy en desarrollo.

### Criterios de aceptación finales

- [ ] Puedo crear, ver, editar, completar y borrar tareas desde la interfaz.
- [ ] El filtro y el contador son correctos.
- [ ] Cualquier error del servidor se muestra al usuario con un mensaje comprensible.
- [ ] Los tests del backend pasan y la API respeta el contrato.
- [ ] El proyecto se arranca siguiendo solo el README.

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

Respuesta:

```json
{ "status": "ok" }
```

---

## Tests

```bash
cd backend
cargo test
```

Cubren, por ahora, el modelo y los DTOs: serialización en camelCase, fechas RFC 3339,
deserialización de creación y actualización, ignorancia de campos desconocidos,
comportamiento de `null` en PATCH y la conversión `CreateTask` → `Task`.
Los tests de integración de los endpoints llegarán en B14.

---

## Convenciones

- Rutas bajo el prefijo `/api`.
- JSON en **camelCase** (`createdAt`, `updatedAt`), aunque los campos en Rust sean `snake_case`.
- Fechas en RFC 3339, siempre generadas por el servidor.
- Errores siempre como JSON con la clave `error`.
- Estado compartido mediante `web::Data` (envolviendo el almacenamiento en un `Mutex`/`RwLock`).
- Handlers delgados: la lógica de negocio vive en funciones/estructuras separadas.

---

## Próximo paso

**B4. Estado compartido en memoria.** Consulta el seguimiento de tareas para el detalle.

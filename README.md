# RustFUL-doctor

REST API для записи к врачу с использованием `Axum`, `sqlx`, `PostgreSQL`, `Docker` 

### Технологический стек
- **Axum**:  Асинхронный веб-фреймворк 
- **SQLx**:  Асинхронный SQL-клиент 
- **PostgreSQL**: Основная БД
- **Docker**: Контейнеризация

### Доступные роуты (В разработке)
```text
GET  /health

GET  /doctors
GET  /doctors?speciality=...

POST   /appointments
PATCH  /appointments/:id
DELETE /appointments/:id
...
```

### Запуск (В разработке)
```bash

```

### Структура проекта
```text
...
src/
├── app
│   ├── appointment/
│   ├── doctor
│   │   ├── handlers.rs
│   │   ├── models.rs
│   │   ├── mod.rs
│   │   ├── repository.rs
│   │   └── service.rs
│   ├── mod.rs
│   └── patient/
├── config.rs
├── main.rs
├── router.rs
└── state.rs
```
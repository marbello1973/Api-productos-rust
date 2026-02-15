# 🛒 API PRODUCTOS

### RUST - AXUM - POSTGRESQL

> **Base URL:** `http://0.0.0.0:3000`  
> **Estado Global:** `Arc<DashMap<K, V>>` para alta concurrencia.

---

## 📦 Endpoints de Productos

| Método   | Endpoint                      | Descripción        | Estado                                 |
| :------- | :---------------------------- | :----------------- | :------------------------------------- |
| `GET`    | `/api/v1/products`            | Listar todos       | [x] ![Ready](https://img.shields.io)   |
| `GET`    | `/api/v1/products/:id`        | Buscar por ID      | [x] ![Ready](https://img.shields.io)   |
| `GET`    | `/api/v1/products/slug/:slug` | Buscar por Slug    | [x] ![Ready](https://img.shields.io)   |
| `GET`    | `/api/v1/products/sku/:sku`   | Buscar por SKU     | [ ] ![Pending](https://img.shields.io) |
| `POST`   | `/api/v1/products`            | Crear nuevo        | [ ] ![Pending](https://img.shields.io) |
| `PUT`    | `/api/v1/products/:id`        | Actualizar total   | [ ] ![Pending](https://img.shields.io) |
| `PATCH`  | `/api/v1/products/:id`        | Actualizar parcial | [ ] ![Pending](https://img.shields.io) |
| `DELETE` | `/api/v1/products/:id`        | Soft Delete        | [ ] ![Pending](https://img.shields.io) |

---

## 💬 Reseñas (Reviews)

| Método | Endpoint                       | Descripción                | Estado                                     |
| :----- | :----------------------------- | :------------------------- | :----------------------------------------- |
| `GET`  | `/api/v1/products/:id/reviews` | Ver reseñas de un producto | [ ] ![In Progress](https://img.shields.io) |
| `POST` | `/api/v1/products/:id/reviews` | Agregar reseña             | [ ] ![Pending](https://img.shields.io)     |
| `GET`  | `/api/v1/reviews/:id`          | Detalle de reseña          | [ ] ![Pending](https://img.shields.io)     |

---

## 🏷️ Marcas y Categorías

| Método | Endpoint                      | Relación            | Estado                                 |
| :----- | :---------------------------- | :------------------ | :------------------------------------- |
| `GET`  | `/api/v1/brands`              | Listar marcas       | [ ] ![Pending](https://img.shields.io) |
| `GET`  | `/api/v1/categories`          | Listar categorías   | [ ] ![Pending](https://img.shields.io) |
| `GET`  | `/api/v1/brands/:id/products` | Productos por marca | [ ] ![Pending](https://img.shields.io) |

---

## 🔍 Filtros (Query Params)

_Implementados en `GET /api/v1/products?`_

- [x] `category_id` / `category_slug`
- [ ] `brand_id` / `brand_slug`
- [ ] `min_price` / `max_price`
- [ ] `search` (Búsqueda por texto)
- [ ] `status` (active/inactive/draft)

---

## 📈 Estadísticas y Especiales

- [ ] `GET /api/v1/products/stats` (Métricas DashMap)
- [ ] `GET /api/v1/products/featured` (Destacados)
- [ ] `GET /api/v1/products/recent` (Novedades)

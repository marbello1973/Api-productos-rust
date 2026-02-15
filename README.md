# 🛒 API PRODUCTOS

### RUST - AXUM - POSTGRESQL

> **Base URL:** `http://0.0.0.0:3000`  
> **Estado Global:** `Arc<DashMap<K, V>>` para alta concurrencia.

---

## 📦 Endpoints de Productos

| Método   | Endpoint                      | Descripción        | Estado                                                             |
| :------- | :---------------------------- | :----------------- | :----------------------------------------------------------------- |
| `GET`    | `/api/v1/products`            | Listar todos       | [x] ![Ready](https://img.shields.io/badge/coverage-75%25-fedcba)   |
| `GET`    | `/api/v1/products/:id`        | Buscar por ID      | [x] ![Ready](https://img.shields.io/badge/coverage-75%25-fedcba)   |
| `GET`    | `/api/v1/products/slug/:slug` | Buscar por Slug    | [x] ![Ready](https://img.shields.io/badge/coverage-75%25-fedcba)   |
| `GET`    | `/api/v1/products/sku/:sku`   | Buscar por SKU     | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `POST`   | `/api/v1/products`            | Crear nuevo        | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `PUT`    | `/api/v1/products/:id`        | Actualizar total   | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `PATCH`  | `/api/v1/products/:id`        | Actualizar parcial | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `DELETE` | `/api/v1/products/:id`        | Soft Delete        | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |

---

## 💬 Reseñas (Reviews)

| Método | Endpoint                       | Descripción                | Estado                                                             |
| :----- | :----------------------------- | :------------------------- | :----------------------------------------------------------------- |
| `GET`  | `/api/v1/products/:id/reviews` | Ver reseñas de un producto | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `POST` | `/api/v1/products/:id/reviews` | Agregar reseña             | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `GET`  | `/api/v1/reviews/:id`          | Detalle de reseña          | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |

---

## 🏷️ Marcas y Categorías

| Método | Endpoint                      | Relación            | Estado                                                             |
| :----- | :---------------------------- | :------------------ | :----------------------------------------------------------------- |
| `GET`  | `/api/v1/brands`              | Listar marcas       | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `GET`  | `/api/v1/brands/:id/products` | Productos por marca | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |
| `GET`  | `/api/v1/categories`          | Listar categorías   | [ ] ![Pending](https://img.shields.io/badge/coverage-00%25-ff00ff) |

---

## 🔍 Filtros (Query Params)

_Implementados en `GET /api/v1/products?`_

- [ ] `category_id` / `category_slug`
- [ ] `brand_id` / `brand_slug`
- [ ] `min_price` / `max_price`
- [ ] `search` (Búsqueda por texto)
- [ ] `status` (active/inactive/draft)

---

## 📈 Estadísticas y Especiales

- [ ] `GET /api/v1/products/stats` (Métricas DashMap)
- [ ] `GET /api/v1/products/featured` (Destacados)
- [ ] `GET /api/v1/products/recent` (Novedades)

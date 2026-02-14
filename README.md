# API PRODUCTOS 

## RESTful API endpoints

### Productos
- GET    /api/v1/products
- GET    /api/v1/products/:id
- GET    /api/v1/products/slug/:slug
- GET    /api/v1/products/sku/:sku
- POST   /api/v1/products
- PUT    /api/v1/products/:id
- PATCH  /api/v1/products/:id
- DELETE /api/v1/products/:id (soft delete)

### Reseñas
- GET    /api/v1/products/:id/reviews
- POST   /api/v1/products/:id/reviews
- GET    /api/v1/reviews/:id
- PUT    /api/v1/reviews/:id
- DELETE /api/v1/reviews/:id

### Marcas 
- GET     /api/v1/brands
- GET     /api/v1/brands/:id
- GET     /api/v1/brands/slug/:slug
- POST    /api/v1/brands
- PUT     /api/v1/brands/:id
- DELETE  /api/v1/brands/:id
- GET     /api/v1/brands/:id/products

 ### Categorias 
- GET     /api/v1/categories
- GET     /api/v1/categories/:id
- GET     /api/v1/categories/slug/:slug
- POST    /api/v1/categories
- PUT     /api/v1/categories/:id
- DELETE  /api/v1/categories/:id
- GET     /api/v1/categories/:id/products

### Todos en GET /api/v1/products con query params
- GET /api/v1/products?
    category_id=&          // Filtrar por categoría ID
    category_slug=&        // Filtrar por slug de categoría
    brand_id=&             // Filtrar por marca ID
    brand_slug=&           // Filtrar por slug de marca
    min_price=&            // Precio mínimo
    max_price=&            // Precio máximo
    search=&               // Búsqueda en nombre/descripción
    status=&               // active/inactive/draft
    in_stock=&             // true/false
    featured=&             // Productos destacados
    bestsellers            // Más vendidos

### Endpoints adicionals utiles 
- GET     /api/v1/products/stats                 // Estadísticas (total, por categoría, etc)
- GET     /api/v1/products/featured              // Productos destacados
- GET     /api/v1/products/bestsellers           // Más vendidos
- GET     /api/v1/products/recent                // Productos recientes
- GET     /api/v1/products/related/:id           // Productos relacionados
- 


-- Add migration script here

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(120) UNIQUE NOT NULL
);

CREATE TABLE brands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(120) UNIQUE NOT NULL
);

CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sku VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(280) UNIQUE NOT NULL,
    description TEXT,
    category_id UUID REFERENCES categories(id),
    brand_id UUID REFERENCES brands(id),
    price DECIMAL(12,2) NOT NULL,
    stock_quantity INTEGER DEFAULT 0,
    status VARCHAR(20) DEFAULT 'draft',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    content TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_products_brand ON products(brand_id);
CREATE INDEX idx_reviews_product ON reviews(product_id);

INSERT INTO categories (id, name, slug) VALUES 
('11111111-1111-1111-1111-111111111111', 'Electrónicos', 'electronicos'),
('22222222-2222-2222-2222-222222222222', 'Computadoras', 'computadoras');

INSERT INTO brands (id, name, slug) VALUES 
('44444444-4444-4444-4444-444444444444', 'NVIDIA', 'nvidia'),
('55555555-5555-5555-5555-555555555555', 'Samsung', 'samsung');

INSERT INTO products (id, sku, name, slug, price, category_id, brand_id, status) VALUES 
('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'LAPTOP-001', 'Laptop Gamer', 'laptop-gamer', 2499.99, '22222222-2222-2222-2222-222222222222', '44444444-4444-4444-4444-444444444444', 'published'),
('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 'TV-001', 'Smart TV 55"', 'smart-tv', 799.99, '11111111-1111-1111-1111-111111111111', '55555555-5555-5555-5555-555555555555', 'published');

INSERT INTO reviews (id, product_id, rating, content) VALUES 
('99999999-9999-9999-9999-999999999999', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 5, 'Excelente producto'),
('77777777-7777-7777-7777-777777777777', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', 5, 'Muy buena TV');

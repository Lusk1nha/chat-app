-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS Users (
    id CHAR(36) NOT NULL PRIMARY KEY, -- Identificador único do usuário (UUID)
    email VARCHAR(100) NOT NULL UNIQUE, -- Email único do usuário
    password_hash VARCHAR(255) NOT NULL, -- Hash da senha para autenticação segura
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Data de criação do usuário
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, -- Última atualização
    last_login TIMESTAMP NULL DEFAULT NULL, -- Data/hora do último login
    is_active BOOLEAN DEFAULT TRUE, -- Status do usuário (ativo/inativo)
    INDEX idx_email (email) -- Índice para facilitar consultas por email
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
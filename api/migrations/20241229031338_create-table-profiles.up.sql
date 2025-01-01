-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS Profiles (
    id CHAR(36) NOT NULL PRIMARY KEY, -- Índice implícito por ser a chave primária
    user_id CHAR(36) NOT NULL, -- Relacionado à tabela Users
    display_name VARCHAR(100) NOT NULL, -- Nome de exibição
    avatar_url TEXT, -- URL do avatar
    bio TEXT, -- Biografia
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Data de criação
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, -- Última atualização
    INDEX idx_user_id (user_id), -- Índice para consultas por user_id
    CONSTRAINT FK_UserProfile FOREIGN KEY (user_id) REFERENCES Users (id) ON DELETE CASCADE
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
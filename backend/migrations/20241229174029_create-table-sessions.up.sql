-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS Sessions (
    id CHAR(36) NOT NULL PRIMARY KEY, -- Identificador único da sessão (UUID)
    user_id CHAR(36) NOT NULL, -- Referência ao usuário na tabela Users
    token TEXT NOT NULL, -- Token de autenticação (JWT ou outro formato)
    expires_at TIMESTAMP NOT NULL, -- Data e hora de expiração da sessão
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Data e hora de criação da sessão
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, -- Data da última atualização
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES Users (id) ON DELETE CASCADE,
    INDEX idx_user_id (user_id) -- Índice para facilitar consultas por user_id
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
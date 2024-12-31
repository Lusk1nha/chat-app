-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS Sessions (
    id CHAR(36) NOT NULL PRIMARY KEY, -- Identificador único da sessão
    user_id CHAR(36) NOT NULL, -- Relaciona a sessão a um usuário
    access_token TEXT NOT NULL, -- Token de acesso (JWT)
    refresh_token TEXT NOT NULL, -- Token de renovação (JWT)
    access_token_expires_at TIMESTAMP NOT NULL, -- Expiração do access token
    refresh_token_expires_at TIMESTAMP NOT NULL, -- Expiração do refresh token
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Data de criação da sessão
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, -- Última atualização
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES Users (id) ON DELETE CASCADE
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
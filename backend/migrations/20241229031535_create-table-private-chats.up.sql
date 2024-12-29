-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS PrivateChats (
    id CHAR(36) NOT NULL PRIMARY KEY, -- Identificador único da conversa (UUID)
    user_one_id CHAR(36) NOT NULL, -- Primeiro usuário na conversa
    user_two_id CHAR(36) NOT NULL, -- Segundo usuário na conversa
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Data de criação da conversa
    INDEX idx_user_one_id (user_one_id), -- Índice para facilitar consultas por user_one_id
    INDEX idx_user_two_id (user_two_id), -- Índice para facilitar consultas por user_two_id
    CONSTRAINT FK_UserOne FOREIGN KEY (user_one_id) REFERENCES Users (id) ON DELETE CASCADE,
    CONSTRAINT FK_UserTwo FOREIGN KEY (user_two_id) REFERENCES Users (id) ON DELETE CASCADE
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
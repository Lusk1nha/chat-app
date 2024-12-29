-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS Messages (
    id CHAR(36) NOT NULL PRIMARY KEY, -- Identificador único da mensagem (UUID)
    chat_id CHAR(36) NOT NULL, -- Relaciona à conversa no PrivateChats
    sender_id CHAR(36) NOT NULL, -- Relaciona ao usuário que enviou a mensagem
    content TEXT, -- Texto da mensagem
    message_type ENUM ('text', 'image', 'audio') DEFAULT 'text', -- Tipo da mensagem
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Data/hora de envio da mensagem
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, -- Última atualização
    INDEX idx_chat_id (chat_id), -- Índice para consultas por chat_id
    INDEX idx_sender_id (sender_id), -- Índice para consultas por sender_id
    CONSTRAINT FK_Chat FOREIGN KEY (chat_id) REFERENCES PrivateChats (id) ON DELETE CASCADE,
    CONSTRAINT FK_Sender FOREIGN KEY (sender_id) REFERENCES Users (id) ON DELETE CASCADE
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
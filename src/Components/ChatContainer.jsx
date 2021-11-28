import { Message } from './Message';

import '../Styles/ChatContainer.scss';

export function ChatContainer() {
  return (
    <div className="ChatContainer">
      <header className="chat-contact">
        <span>Contato 1</span>
      </header>
      
      <main className="chat-content">
        <Message message={'Mensagem'} user={'Contato 1'} isMy={true} />
        <Message message={'Mensagem'} user={'Contato 1'} isMy={false} />
        <Message message={'Mensagem'} user={'Contato 1'} isMy={true} />
      </main>

      <footer className="send-message-container">
        <form>
          <input type="text" placeholder="Insira sua mensagem aqui" />
          <button type="submit">Enviar</button>
          <section className="message-toolbox">
          </section>
        </form>
      </footer>
    </div>
  );
}
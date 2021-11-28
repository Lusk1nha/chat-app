import { useState, useContext, useEffect } from 'react';
import { CurrentContactContext } from '../Contexts/CurrentContactContext';

import { MessageRender } from './MessageRender';
import { SendMessageContainer } from './SendMessage/SendMessageContainer';

import '../Styles/ChatContainer.scss';

export function ChatContainer() {
  const {current, setCurrent} = useContext(CurrentContactContext)
  const [user, setUser] = useState('')

  useEffect(() => {
    setUser(current.name)
  }, [current])

  return (
    <div className="ChatContainer">
      <header className="chat-contact">
        <span>{user}</span>
      </header>
      
      <MessageRender />
      <SendMessageContainer />
    </div>
  );
}
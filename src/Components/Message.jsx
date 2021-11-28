import { useState } from 'react';

import '../Styles/Message.scss';

export function Message(props) {
  const [text, setText] = useState(props.message)
  const [isMy, setIsMy] = useState(props.isMy)


  return (
    <div className={isMy ? 'message-container myMessage' : 'message-container'}>
      <h4 className="user">{isMy ? 'Eu' : props.user}</h4>
      <p className="message-text">
        {text}
      </p>
    </div>
  );
}
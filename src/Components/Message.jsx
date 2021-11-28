import { useState } from 'react';

import '../Styles/Message.scss';

export function Message(props) {
  const [Content, setContent] = useState({
    'Text': props.Content.message,
    'User': props.Content.user,
    'DateTime': props.Content.datetime
  })

  return (
    <div className={Content.User == 'Eu' ? 'message-container myMessage' : 'message-container'}>
      <section className="message-infos">
        <h4 className="user">{Content.User}</h4>
        <h5 className="datetime">{Content.DateTime}</h5>
      </section>
      <p className="message-text">
        {Content.Text}
      </p>
    </div>
  );
}
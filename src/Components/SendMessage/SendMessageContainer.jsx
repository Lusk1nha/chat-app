import { SendMessageInput } from "./SendMessageInput";
import { MessageToolBox } from "./MessageToolBox";

import '../../Styles/SendMessage/SendMessageContainer.scss';

export function SendMessageContainer() {
  return (
    <footer className="send-message-container">
      <SendMessageInput />
      <MessageToolBox />
    </footer>
  );
}
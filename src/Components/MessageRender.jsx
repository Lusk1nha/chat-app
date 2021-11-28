import { Message } from "./Message";

export function MessageRender(props) {
  const MESSAGE_CONTAINER = [
    {'message': 'teste1', 'user': 'teste1', 'datetime': '12:50'},
    {'message': 'teste2', 'user': 'teste1', 'datetime': '12:53'},
    {'message': 'teste3', 'user': 'Eu', 'datetime': '14:02'}
  ]

  return (
    <main className="chat-content">
      {MESSAGE_CONTAINER.map((Info, Key) => {
        return <Message Content={Info} key={Key}/>
      })}
    </main>
  );
}
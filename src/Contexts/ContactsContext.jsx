import { createContext, useState } from "react";

export const ContactsContext = createContext({})

export function ContactsProvider(props) {
  const [contacts, setContacts] = useState([
    {'name': 'Teste1', 'id': '1', 'image': 'https://avatars.githubusercontent.com/u/61957312?v=4', 'messages': {
    'id': {'text': 'teste', 'user': 'teste1', 'datetime': '15:00h'}
  }},
  {'name': 'Teste2', 'id': '2', 'image': 'https://lh3.googleusercontent.com/ogw/ADea4I6c3zNpjaGkjgmOVsWJ87b9CtiwG5AJVhMJL9ko4_Y=s83-c-mo'},
  {'name': 'Teste3', 'id': '3', 'image': 'https://lh3.googleusercontent.com/ogw/ADea4I6c3zNpjaGkjgmOVsWJ87b9CtiwG5AJVhMJL9ko4_Y=s83-c-mo'}])

  return (
    <ContactsContext.Provider value={contacts}>
      {props.children}
    </ContactsContext.Provider>
  );
}
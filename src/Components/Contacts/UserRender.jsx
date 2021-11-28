import { User } from './User';
import { ContactsContext } from '../../Contexts/ContactsContext';
import { useContext } from 'react';

export function UserRender() {
  const CONTACTS_LIST = useContext(ContactsContext)
  console.log(CONTACTS_LIST)

  return (
    <>
      {CONTACTS_LIST.map((Info, Key) => {
        return <User Content={Info} key={Key} />
      })}
    </>
  );
}
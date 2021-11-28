import { UserRender } from './UserRender';
import { useContext } from 'react';

import { ContactsContext } from '../../Contexts/ContactsContext';

import '../../Styles/Contacts/Contacts.scss';

export function Contacts() {
  return (
    <div className="contacts">
      <UserRender />
    </div>
  );
}
import { useState, useContext } from "react";

import { CurrentContactContext } from "../../Contexts/CurrentContactContext";

import '../../Styles/Contacts/User.scss';

export function User(props) {
  const {current, setCurrent} = useContext(CurrentContactContext)

  const [contact, setContact] = useState({
    'Name': props.Content.name,
    'Id': props.Content.id,
    'Photo': props.Content.image,
  })

  return (
    <div className="user-container" onClick={() => setCurrent(props.Content)}>
      <img className="user-photo" src={contact.Photo} alt={contact.Name}/>
      <h4 className="user-name">
        {contact.Name}
      </h4>
    </div>
  );
}
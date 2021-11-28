import { Contacts } from './Components/Contacts/Contacts';
import { ChatContainer } from './Components/ChatContainer';

import { ContactsProvider } from './Contexts/ContactsContext';
import { CurrentProvider } from './Contexts/CurrentContactContext';

import './Styles/App.scss';

function App() {
  return (
    <div className="App">
      <ContactsProvider>
        <CurrentProvider>
          <Contacts />
          <ChatContainer />
        </CurrentProvider>
      </ContactsProvider>
    </div>
  );
}

export default App;

import { createContext, useState } from "react";

export const CurrentContactContext = createContext({})

export function CurrentProvider(props) {
  const [current, setCurrent] = useState({})

  return (
    <CurrentContactContext.Provider value={{current, setCurrent}}>
      {props.children}
    </CurrentContactContext.Provider>
  );
}
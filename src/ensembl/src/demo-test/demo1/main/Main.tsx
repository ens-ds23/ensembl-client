import React, { useState, useEffect } from 'react';

import Field from '../field/Field';
import Results from '../results/Results';

const Main = () => {
  const style = {
    width: '50%',
    margin: '100px auto'
  };

  const [characters, setCharacters] = useState([]);

  return (
    <div style={style}>
      <Field />
      {characters.length && <Results names={characters} />}
    </div>
  );
};

export default Main;

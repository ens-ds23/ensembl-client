import React, { useState, useEffect } from 'react';

import apiService from 'src/services/api-service';

import Field from '../field/Field';
import Results from '../results/Results';

const Main = () => {
  const style = {
    width: '50%',
    margin: '100px auto'
  };

  const [repositories, setRepositories] = useState([]);

  const handleSubmit = async (value: string) => {
    const endpoint = `https://api.github.com/users/${value}/repos`;
    const repositories = await apiService.fetch(endpoint);
    setRepositories(repositories);
  };

  const repoNames = repositories.map(({ name }) => name);

  return (
    <div style={style}>
      <Field onSubmit={handleSubmit} />
      {Boolean(repositories.length) && <Results names={repoNames} />}
    </div>
  );
};

export default Main;

import React, { useState } from 'react';

import apiService from 'src/services/api-service';

import Form from '../form/Form';
import Results from '../results/Results';

const Container = () => {
  const [results, setResults] = useState<any>([]);

  const handleSubmit = async (value: string) => {
    const endpoint = `https://api.github.com/users/${value}/repos`;
    const newResults = await apiService.fetch(endpoint);
    setResults(newResults);
  };

  return (
    <div>
      <Form onSubmit={handleSubmit} />
      {results.length > 0 && <Results results={results} />}
    </div>
  );
};

export default Container;

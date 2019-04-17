import React from 'react';

type Result = {
  name: string;
};

type Props = {
  results: Result[];
};

const Results = (props: Props) => {
  return (
    <ul>
      {props.results.map((result, index) => (
        <li key={index}>{result.name}</li>
      ))}
    </ul>
  );
};

export default Results;

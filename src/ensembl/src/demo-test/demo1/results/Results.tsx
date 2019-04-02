import React from 'react';

type Props = {
  names: string[];
};

const Results = (props: Props) => {
  const results = props.names.map((name, index) => <li key={index}>{name}</li>);

  return <ul>{results}</ul>;
};

export default Results;

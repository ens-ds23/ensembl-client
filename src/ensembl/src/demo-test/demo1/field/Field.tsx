import React from 'react';

type Props = {
  onSubmit: (value: string) => void;
};

const Field = (props: Props) => {
  const handleSubmit = (event: React.SyntheticEvent<HTMLFontElement>) => {
    event.preventDefault();
    const value = event.target.value;
  };

  return (
    <form>
      <input />
      <button type="submit">Search</button>
    </form>
  );
};

export default Field;

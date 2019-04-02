import React, { useRef } from 'react';

type Props = {
  onSubmit: (value: string) => void;
};

const Field = (props: Props) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!inputRef.current) return;

    props.onSubmit(inputRef.current.value);
  };

  return (
    <form onSubmit={handleSubmit}>
      <input ref={inputRef} />
      <button type="submit">Search</button>
    </form>
  );
};

export default Field;

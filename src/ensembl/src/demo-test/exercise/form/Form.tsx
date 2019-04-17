import React, { useRef } from 'react';

type Props = {
  onSubmit: (value: string) => void;
};

const Form = (props: Props) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (e: any) => {
    e.preventDefault();

    const input = inputRef.current as HTMLInputElement;
    props.onSubmit(input.value);
  };

  return (
    <form onSubmit={handleSubmit}>
      <input ref={inputRef} />
      <button type="submit">Search</button>
    </form>
  );
};

export default Form;

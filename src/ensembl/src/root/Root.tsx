import React from 'react';

import Main from 'src/demo-test/demo1/main/Main';

import styles from './Root.scss';

const Root = () => (
  <div className={styles.root}>
    <div className={styles.example}>
      <h1>Example</h1>
      <Main />
    </div>
    <div className={styles.exercise}>
      <h1>Exercise</h1>
      Here is where our code will go
    </div>
  </div>
);

export default Root;

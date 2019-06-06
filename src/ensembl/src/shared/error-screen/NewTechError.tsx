import React from 'react';

import styles from './ErrorScreen.scss';

const NewTechError = () => {
  return (
    <div className={styles.newTechErrorWrapper}>
      <div className={styles.newTechMessage}>
        Sorry, but it seems your browser can’t display the new site
      </div>
      <a href="https://www.ensembl.org" className={styles.errorLinkButton}>
        Go to Ensembl classic
      </a>
    </div>
  );
};

export default NewTechError;

import React from 'react';

import { HomeLink, ReleaseVersion, Copyright } from 'src/header/Header';

import styles from './ErrorScreen.scss';
import headerStyles from 'src/header/Header.scss';

import generalErrorImage1 from './images/general-error-1.jpg';
import generalErrorImage2 from './images/general-error-2.jpg';
import generalErrorImage3 from './images/general-error-3.jpg';

const GeneralErrorScreen = () => (
  <section className={styles.generalErrorScreen}>
    <header className={`${headerStyles.topBar} ${styles.generalErrorHeader}`}>
      <HomeLink />
      <ReleaseVersion />
      <Copyright />
    </header>
    <div className={styles.generalErrorBody}>
      <p className={styles.generalErrorTopMessage}>
        Sorry, something seems to have gone wrong
      </p>
      <div className={styles.generalErrorImages}>
        <img src={generalErrorImage1} />
        <img src={generalErrorImage2} />
        <img src={generalErrorImage3} />
      </div>
      <div className={styles.generalErrorBottomMessage}>
        <p>We‘re trying to fix it right now</p>
        <p>Please try again later...</p>
      </div>
    </div>
  </section>
);

export default GeneralErrorScreen;

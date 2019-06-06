import React, { FunctionComponent } from 'react';

import styles from './PrivacyBanner.scss';

type PrivacyBannerProps = {
  closeBanner: () => void;
};

export const PrivacyBanner: FunctionComponent<PrivacyBannerProps> = (
  props: PrivacyBannerProps
) => {
  return (
    <section className={styles.privacyBanner}>
      <p>
        This website requires cookies, and the limited processing of your
        personal data in order to function. By using the site you are agreeing
        to this as outlined in our{' '}
        <a
          href="https://www.ebi.ac.uk/data-protection/ensembl/privacy-notice"
          target="_blank"
          rel="noopener noreferrer"
        >
          Privacy Policy
        </a>{' '}
        and
        <a
          href="https://www.ebi.ac.uk/about/terms-of-use"
          target="_blank"
          rel="noopener noreferrer"
        >
          {' '}
          Terms of Use
        </a>
        .
        <button className={styles.agreeButton} onClick={props.closeBanner}>
          I agree
        </button>
      </p>
    </section>
  );
};

export default PrivacyBanner;

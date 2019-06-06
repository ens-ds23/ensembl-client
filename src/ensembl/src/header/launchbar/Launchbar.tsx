import React from 'react';
import SlideDown from 'react-slidedown';

import ensemblIcon from 'static/img/launchbar/ensembl-logo.png'; // <-- note it's a png

import { ReactComponent as SearchIcon } from 'static/img/launchbar/search.svg';
import { ReactComponent as SpeciesSelectorIcon } from 'static/img/launchbar/species-selector.svg';
import { ReactComponent as BrowserIcon } from 'static/img/launchbar/browser.svg';
import { ReactComponent as VEPIcon } from 'static/img/launchbar/vep.svg';
import { ReactComponent as CustomDownloadIcon } from 'static/img/launchbar/custom-download.svg';
import { ReactComponent as HelpIcon } from 'static/img/launchbar/help.svg';

import LaunchbarButton from './LaunchbarButton';

import styles from './Launchbar.scss';

type LaunchbarProps = {
  launchbarExpanded: boolean;
};

export const getCategoryClass = (separator: boolean): string => {
  return separator ? 'border' : '';
};

const LaunchbarContent = () => (
  <div className={styles.launchbar}>
    <div className={styles.categoriesWrapper}>
      <div className={styles.categories}>
        <div className={styles.category}>
          <LaunchbarButton
            app="global-search"
            description="Global search"
            icon={SearchIcon}
            enabled={false}
          />
          <LaunchbarButton
            app="species-selector"
            description="Species selector"
            icon={SpeciesSelectorIcon}
            enabled={true}
          />
        </div>
        <div className={styles.category}>
          <LaunchbarButton
            app="browser"
            description="Genome browser"
            icon={BrowserIcon}
            enabled={true}
          />
        </div>
        <div className={styles.category}>
          <LaunchbarButton
            app="tools"
            description="Tools"
            icon={VEPIcon}
            enabled={false}
          />
        </div>
        <div className={styles.category}>
          <LaunchbarButton
            app="downloads"
            description="Downloads"
            icon={CustomDownloadIcon}
            enabled={false}
          />
        </div>
        <div className={styles.category}>
          <LaunchbarButton
            app="help-docs"
            description="Help & documentation"
            icon={HelpIcon}
            enabled={false}
          />
        </div>
      </div>
    </div>
    <div className={styles.about}>
      <span className={styles.aboutText}>Genome research database</span>
      <LaunchbarButton
        app="about"
        description="About Ensembl"
        icon={ensemblIcon}
        enabled={false}
      />
    </div>
  </div>
);

const Launchbar = (props: LaunchbarProps) => {
  return (
    <SlideDown transitionOnAppear={false}>
      {props.launchbarExpanded ? <LaunchbarContent /> : null}
    </SlideDown>
  );
};

export default Launchbar;

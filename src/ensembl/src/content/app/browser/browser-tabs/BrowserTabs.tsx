import React, { FunctionComponent, useEffect, useState } from 'react';

import { TrackType } from '../track-panel/trackPanelConfig';
import { EnsObject } from 'src/ens-object/ensObjectTypes';
import styles from './BrowserTabs.scss';

type BrowserTabsProps = {
  activeGenomeId: string;
  ensObjectInfo: EnsObject;
  drawerOpened: boolean;
  genomeSelectorActive: boolean;
  selectBrowserTabAndSave: (selectedBrowserTab: TrackType) => void;
  selectedBrowserTab: { [genomeId: string]: TrackType };
  toggleDrawer: (drawerOpened: boolean) => void;
  trackPanelModalOpened: boolean;
};

type ClickHandlers = {
  [key: string]: () => void;
};

const BrowserTabs: FunctionComponent<BrowserTabsProps> = (
  props: BrowserTabsProps
) => {
  const initClickHandlers: ClickHandlers = {};
  const [clickHandlers, setClickHandlers] = useState(initClickHandlers);

  const getBrowserTabClasses = (trackType: TrackType) => {
    const { activeGenomeId, drawerOpened, trackPanelModalOpened } = props;
    const selectedBrowserTab =
      props.selectedBrowserTab[activeGenomeId] || TrackType.GENOMIC;
    let classNames = styles.browserTab;

    if (
      props.ensObjectInfo.genome_id &&
      selectedBrowserTab === trackType &&
      drawerOpened === false &&
      trackPanelModalOpened === false
    ) {
      classNames += ` ${styles.browserTabActive} ${styles.browserTabArrow}`;
    } else if (!props.ensObjectInfo.genome_id) {
      classNames = styles.browserTabDisabled;
    }

    return classNames;
  };

  useEffect(() => {
    const callbacks: ClickHandlers = {};

    Object.values(TrackType).forEach((value: TrackType) => {
      callbacks[value] = () => {
        if (
          props.genomeSelectorActive === true ||
          !props.ensObjectInfo.genome_id
        ) {
          return;
        }

        if (props.drawerOpened === true) {
          props.toggleDrawer(false);
        }

        props.selectBrowserTabAndSave(value);
      };
    });

    setClickHandlers(callbacks);
  }, [props.drawerOpened, props.genomeSelectorActive]);

  return (
    <dl className={`${styles.browserTabs} show-for-large`}>
      {Object.values(TrackType).map((value: TrackType) => (
        <dd
          className={getBrowserTabClasses(value)}
          key={value}
          onClick={clickHandlers[value]}
        >
          <button>{value}</button>
        </dd>
      ))}
    </dl>
  );
};

export default BrowserTabs;

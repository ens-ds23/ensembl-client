import React, { FunctionComponent, useCallback } from 'react';

import { ChrLocation } from '../browserState';
import { BrowserInfoItem } from '../browserConfig';

import styles from './BrowserReset.scss';
import { getChrLocationStr } from '../browserHelper';

type BrowserResetProps = {
  chrLocation: ChrLocation;
  defaultChrLocation: ChrLocation;
  details: BrowserInfoItem;
  dispatchBrowserLocation: (chrLocation: ChrLocation) => void;
  drawerOpened: boolean;
};

export const BrowserReset: FunctionComponent<BrowserResetProps> = (
  props: BrowserResetProps
) => {
  const { chrLocation, defaultChrLocation, details, drawerOpened } = props;

  const getResetIcon = (): string => {
    const chrLocationStr = getChrLocationStr(chrLocation);
    const defaultChrLocationStr = getChrLocationStr(defaultChrLocation);

    if (chrLocationStr === defaultChrLocationStr || drawerOpened === true) {
      return details.icon.grey as string;
    }

    return details.icon.default;
  };

  const resetBrowser = useCallback(() => {
    if (drawerOpened === true) {
      return;
    }

    props.dispatchBrowserLocation(props.defaultChrLocation);
  }, [chrLocation, drawerOpened]);

  return (
    <dd className={styles.resetButton} onClick={resetBrowser}>
      <button>
        <img src={getResetIcon()} alt={details.description} />
      </button>
    </dd>
  );
};

export default BrowserReset;

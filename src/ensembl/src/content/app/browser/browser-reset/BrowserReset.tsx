import React, { FunctionComponent, useCallback } from 'react';

import { ChrLocation } from '../browserState';
import { ReactComponent as resetIcon } from 'static/img/browser/track-reset.svg';
import ImageButton, {
  ImageButtonStatus
} from 'src/shared/image-button/ImageButton';

import styles from './BrowserReset.scss';
import { getChrLocationStr } from '../browserHelper';

type BrowserResetProps = {
  activeGenomeId: string;
  chrLocation: { [genomeId: string]: ChrLocation };
  defaultChrLocation: { [genomeId: string]: ChrLocation };
  dispatchBrowserLocation: (chrLocation: ChrLocation) => void;
  drawerOpened: boolean;
};

export const BrowserReset: FunctionComponent<BrowserResetProps> = (
  props: BrowserResetProps
) => {
  const {
    activeGenomeId,
    chrLocation,
    defaultChrLocation,
    drawerOpened
  } = props;

  const getResetIconStatus = (): ImageButtonStatus => {
    const chrLocationStr = getChrLocationStr(chrLocation[activeGenomeId]);
    const defaultChrLocationStr = getChrLocationStr(
      defaultChrLocation[activeGenomeId]
    );

    if (chrLocationStr === defaultChrLocationStr || drawerOpened === true) {
      return ImageButtonStatus.INACTIVE;
    }

    return ImageButtonStatus.ACTIVE;
  };

  const resetBrowser = useCallback(() => {
    if (drawerOpened === true) {
      return;
    }

    props.dispatchBrowserLocation(
      props.defaultChrLocation[props.activeGenomeId]
    );
  }, [chrLocation, drawerOpened]);

  return (
    <dd className={styles.resetButton}>
      <div className={styles.imageWrapper}>
        <ImageButton
          buttonStatus={getResetIconStatus()}
          description={'Reset browser image'}
          image={resetIcon}
          onClick={resetBrowser}
          classNames={{ inactive: styles.imageButtonInactive }}
        />
      </div>
    </dd>
  );
};

export default BrowserReset;

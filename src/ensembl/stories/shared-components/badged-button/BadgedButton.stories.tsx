import React from 'react';
import { storiesOf } from '@storybook/react';
import { action } from '@storybook/addon-actions';

import Roundbutton from 'src/shared/round-button/RoundButton';
import BadgedButton from 'src/shared/badged-button/BadgedButton';
import { SecondaryButton } from 'src/shared/button/Button';
import { ReactComponent as DownloadIcon } from 'static/img/track-panel/download.svg';
import styles from './BadgedButton.stories.scss';
import ImageButton, {
  ImageButtonStatus
} from 'src/shared/image-button/ImageButton';

const onClick = action('button-click');

storiesOf('Components|Shared Components/BadgedButton', module)
  .add('badged Button', () => (
    <div className={styles.wrapper}>
      <BadgedButton badgeContent={':)'}>
        <SecondaryButton onClick={onClick}>Secondary button</SecondaryButton>
      </BadgedButton>
    </div>
  ))
  .add('badged RoundButton', () => (
    <div className={styles.wrapper}>
      <BadgedButton badgeContent={':)'}>
        <Roundbutton onClick={onClick}>Badged RoundButton</Roundbutton>
      </BadgedButton>
    </div>
  ))
  .add('badged ImageButton', () => (
    <div className={styles.imageButtonWrapper}>
      <BadgedButton badgeContent={':)'}>
        <ImageButton
          buttonStatus={ImageButtonStatus.HIGHLIGHTED}
          description={'enable/disable'}
          image={DownloadIcon}
          onClick={onClick}
        />
      </BadgedButton>
    </div>
  ))
  .add('custom styling', () => (
    <div className={styles.imageButtonWrapper}>
      <BadgedButton badgeContent={':)'} className={styles.badge}>
        <ImageButton
          buttonStatus={ImageButtonStatus.HIGHLIGHTED}
          description={'enable/disable'}
          image={DownloadIcon}
          onClick={onClick}
        />
      </BadgedButton>
    </div>
  ));

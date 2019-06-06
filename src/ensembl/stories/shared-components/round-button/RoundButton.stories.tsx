import React from 'react';
import { storiesOf } from '@storybook/react';
import { action } from '@storybook/addon-actions';

import Roundbutton, {
  RoundButtonStatus
} from 'src/shared/round-button/RoundButton';
import BadgedButton from 'src/shared/badged-button/BadgedButton';

import styles from './RoundButton.stories.scss';

const onClick = action('button-click');

storiesOf('Components|Shared Components/RoundButton', module)
  .add('default', () => (
    <div className={styles.wrapper}>
      <Roundbutton onClick={onClick}>I am default</Roundbutton>
    </div>
  ))
  .add('active', () => (
    <div className={styles.wrapper}>
      <Roundbutton status={RoundButtonStatus.ACTIVE} onClick={onClick}>
        I am active
      </Roundbutton>
    </div>
  ))
  .add('inactive', () => (
    <div className={styles.wrapper}>
      <Roundbutton status={RoundButtonStatus.INACTIVE} onClick={onClick}>
        I am inactive
      </Roundbutton>
    </div>
  ))
  .add('disabled', () => (
    <div className={styles.wrapper}>
      <Roundbutton status={RoundButtonStatus.DISABLED} onClick={onClick}>
        I am disabled
      </Roundbutton>
    </div>
  ))
  .add('custom', () => (
    <div className={styles.wrapper}>
      <Roundbutton
        classNames={styles}
        status={RoundButtonStatus.ACTIVE}
        onClick={onClick}
      >
        Custom Active
      </Roundbutton>
    </div>
  ))
  .add('badged', () => (
    <div className={styles.wrapper}>
      <BadgedButton badgeContent={'10'}>
        <Roundbutton status={RoundButtonStatus.ACTIVE} onClick={onClick}>
          I have a badge
        </Roundbutton>
      </BadgedButton>
    </div>
  ));

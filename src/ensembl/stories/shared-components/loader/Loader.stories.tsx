import React from 'react';
import { storiesOf } from '@storybook/react';

import { CircleLoader } from 'src/shared/loader/Loader';

import styles from './Loader.stories.scss';

storiesOf('Components|Shared Components/Loader', module).add(
  'full-page',
  () => (
    <div className={styles.fullPageWrapper}>
      <CircleLoader />
    </div>
  )
);

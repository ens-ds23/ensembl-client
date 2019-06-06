import React from 'react';

import styles from './ImageHolder.scss';

type Props = {
  image: string;
  filePath: string;
};

const ImageHolder = (props: Props) => {
  return (
    <div className={styles.imageHolder}>
      <div className={styles.imageArea}>
        <img src={props.image} />
      </div>
      <div className={styles.imagePath}>{props.filePath}</div>
    </div>
  );
};

export default ImageHolder;

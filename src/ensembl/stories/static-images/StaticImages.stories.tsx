import React from 'react';
import { storiesOf } from '@storybook/react';

import storyStyles from '../common.scss';
import ImageHolder from './ImageHolder';

function importAllImages(fileType: string) {
  let allImages: any = [];

  if (fileType === 'svg') {
    allImages = require.context('static/img', true, /\.(svg)$/);
  } else if (fileType === 'png') {
    allImages = require.context('static/img', true, /\.(png)$/);
  } else if (fileType === 'jpeg') {
    allImages = require.context('static/img', true, /\.(jpe?g)$/);
  }

  const imagePaths = allImages.keys();

  const imageComponents = imagePaths.map(allImages);

  const images: any = [];

  imagePaths.forEach(({  }: string, key: number) => {
    const filePath =
      'static/img' + imagePaths[key].substring(1, imagePaths[key].length);
    images.push([filePath, imageComponents[key]]);
  });

  return images;
}

storiesOf('Static Images / All', module)
  .add('SVGs', () => {
    return (
      <div className={storyStyles.page}>
        {importAllImages('svg').map((image: any, key: number) => {
          return (
            <ImageHolder
              key={key}
              image={image[1].default}
              filePath={image[0]}
            />
          );
        })}
      </div>
    );
  })
  .add('PNGs', () => {
    return (
      <div className={storyStyles.page}>
        {importAllImages('png').map((image: any, key: number) => {
          return <ImageHolder key={key} image={image[1]} filePath={image[0]} />;
        })}
      </div>
    );
  })
  .add('JPEGs', () => {
    return (
      <div className={storyStyles.page}>
        {importAllImages('jpeg').map((image: any, key: number) => {
          return <ImageHolder key={key} image={image[1]} filePath={image[0]} />;
        })}
      </div>
    );
  });

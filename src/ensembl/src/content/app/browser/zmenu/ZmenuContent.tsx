import React from 'react';
import classNames from 'classnames';
import { Link } from 'react-router-dom';

import styles from './Zmenu.scss';

import {
  ZmenuContentFeature as ZmenuContentFeatureType,
  ZmenuContentLine as ZmenuContentLineType,
  ZmenuContentBlock as ZmenuContentBlockType,
  ZmenuContentItem as ZmenuContentItemType,
  Markup
} from './zmenu-types';

type ZmenuContentProps = {
  content: ZmenuContentFeatureType[];
};

type ZmenuContentFeatureProps = {
  id: string;
  lines: ZmenuContentLineType[];
};

type ZmenuContentLineProps = {
  blocks: ZmenuContentBlockType[];
  id: string;
};

type ZmenuContentBlockProps = {
  items: ZmenuContentItemType[];
  id: string;
};

type ZmenuContentItemProps = ZmenuContentItemType & {
  id: string;
};

const ZmenuContent = (props: ZmenuContentProps) => {
  const features = props.content;
  const renderedContent = features.map((feature) => (
    <ZmenuContentFeature
      key={feature.id}
      id={feature.id}
      lines={feature.lines}
    />
  ));
  return <>{renderedContent}</>;
};

const ZmenuContentFeature = (props: ZmenuContentFeatureProps) => {
  return (
    <p className={styles.zmenuContentFeature}>
      {props.lines.map((line, index) => (
        <ZmenuContentLine key={index} id={props.id} blocks={line} />
      ))}
    </p>
  );
};

const ZmenuContentLine = (props: ZmenuContentLineProps) => {
  return (
    <span className={styles.zmenuContentLine}>
      {props.blocks.map((items, index) => (
        <ZmenuContentBlock key={index} id={props.id} items={items} />
      ))}
    </span>
  );
};

const ZmenuContentBlock = (props: ZmenuContentBlockProps) => {
  return (
    <span className={styles.zmenuContentBlock}>
      {props.items.map((item, index) => (
        <ZmenuContentItem key={index} id={props.id} {...item} />
      ))}
    </span>
  );
};

const ZmenuContentItem = (props: ZmenuContentItemProps) => {
  const { text, markup } = props;

  const className = classNames({
    [styles.markupLight]: markup.includes(Markup.LIGHT),
    [styles.markupStrong]: markup.includes(Markup.STRONG),
    [styles.markupEmphasis]: markup.includes(Markup.EMPHASIS),
    [styles.markupFocus]: markup.includes(Markup.FOCUS)
  });

  const item = <span className={className}>{text}</span>;

  // TODO: build correct link to the ensembl object in genome browser
  // after the browser chrome learns to support multiple genomes
  return markup.includes(Markup.FOCUS) ? <Link to="/">{item}</Link> : item;
};

export default ZmenuContent;

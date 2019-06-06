import React from 'react';
import { storiesOf } from '@storybook/react';

import {
  Accordion,
  AccordionItem,
  AccordionItemHeading,
  AccordionItemPanel,
  AccordionItemButton,
  AccordionItemPermanentBlock
} from 'src/shared/accordion/';
import styles from './Accordion.stories.scss';

const onChange = () => {
  console.log(1);
};
storiesOf('Components|Shared Components/Accordion', module)
  .add('default', () => {
    return (
      <div className={styles.accordionContainer}>
        <Accordion onChange={onChange}>
          <AccordionItem>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item One</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item One content</div>
            </AccordionItemPanel>
          </AccordionItem>
          <AccordionItem>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Two</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Two content</div>
            </AccordionItemPanel>
          </AccordionItem>
        </Accordion>
      </div>
    );
  })
  .add('allow multiple', () => {
    return (
      <div className={styles.accordionContainer}>
        <Accordion allowMultipleExpanded={true}>
          <AccordionItem>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item One</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item One content</div>
            </AccordionItemPanel>
          </AccordionItem>
          <AccordionItem>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Two</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Two content</div>
            </AccordionItemPanel>
          </AccordionItem>
        </Accordion>
      </div>
    );
  })
  .add('pre-expanded Item', () => {
    return (
      <div className={styles.accordionContainer}>
        <Accordion allowMultipleExpanded={true} preExpanded={[1]}>
          <AccordionItem uuid={1}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item One</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item One content</div>
            </AccordionItemPanel>
          </AccordionItem>
          <AccordionItem uuid={2}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Two</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Two content</div>
            </AccordionItemPanel>
          </AccordionItem>
        </Accordion>
      </div>
    );
  })
  .add('with permanent block', () => {
    return (
      <div className={styles.accordionContainer}>
        <Accordion allowMultipleExpanded={true}>
          <AccordionItem uuid={1}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item One</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item One content</div>
            </AccordionItemPanel>
            <AccordionItemPermanentBlock>
              <div className={styles.accordionPermanentBlock}>
                This content belongs to Item One and will always be visible.
              </div>
            </AccordionItemPermanentBlock>
          </AccordionItem>
          <AccordionItem uuid={2}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Two</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Two content</div>
            </AccordionItemPanel>
          </AccordionItem>
        </Accordion>
      </div>
    );
  })
  .add('bottom right', () => {
    return (
      <div className={styles.accordionBottomRight}>
        <Accordion allowMultipleExpanded={true}>
          <AccordionItem uuid={1}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item One</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item One content</div>
            </AccordionItemPanel>
          </AccordionItem>
          <AccordionItem uuid={2}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Two</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Two content</div>
            </AccordionItemPanel>
          </AccordionItem>
          <AccordionItem uuid={3}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Three</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Three content</div>
            </AccordionItemPanel>
          </AccordionItem>
          <AccordionItem uuid={4}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Four</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Four content</div>
            </AccordionItemPanel>
          </AccordionItem>
          <AccordionItem uuid={5}>
            <AccordionItemHeading>
              <AccordionItemButton>Accordion Item Five</AccordionItemButton>
            </AccordionItemHeading>
            <AccordionItemPanel>
              <div>Item Five content</div>
            </AccordionItemPanel>
          </AccordionItem>
        </Accordion>
      </div>
    );
  });

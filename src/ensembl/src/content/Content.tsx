import React, { FunctionComponent } from 'react';
import { Route } from 'react-router-dom';
import { connect } from 'react-redux';

import { RootState } from '../store';
import { getLaunchbarExpanded } from '../header/headerSelectors';

import Home from './home/Home';
import App from './app/App';

import styles from './Content.scss';

type StateProps = {
  launchbarExpanded: boolean;
};

type OwnProps = {
  children: React.ReactNode;
};

type ContentProps = StateProps & OwnProps;

const getHeightClass = (launchbarExpanded: boolean): string => {
  return launchbarExpanded ? styles.shorter : styles.taller;
};

const ContentRoutes = () => (
  <>
    <Route path="/" component={Home} exact={true} />
    <Route path="/app" component={App} />
  </>
);

export const Content: FunctionComponent<ContentProps> = (
  props: ContentProps
) => {
  return (
    <main
      className={`${styles.content} ${getHeightClass(props.launchbarExpanded)}`}
    >
      {props.children}
    </main>
  );
};

// helper for making the Content component testable (no need to render the whole component tree nested in Content)
export const withInnerContent = (innerContent: React.ReactNode) => (
  props: StateProps
) => <Content {...props}>{innerContent}</Content>;

const mapStateToProps = (state: RootState): StateProps => ({
  launchbarExpanded: getLaunchbarExpanded(state)
});

export default connect(mapStateToProps)(withInnerContent(<ContentRoutes />));

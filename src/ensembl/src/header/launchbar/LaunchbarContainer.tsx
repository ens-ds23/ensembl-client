import React, { FunctionComponent, memo } from 'react';
import { connect } from 'react-redux';

import { RootState } from 'src/store';
import Launchbar from './Launchbar';
import { getLaunchbarExpanded } from '../headerSelectors';

type StateProps = {
  launchbarExpanded: boolean;
};

type OwnProps = {};

type LaunchbarContainerProps = StateProps & OwnProps;

export const LaunchbarContainer: FunctionComponent<
  LaunchbarContainerProps
> = memo((props) => <Launchbar {...props} />);

const mapStateToProps = (state: RootState): StateProps => ({
  launchbarExpanded: getLaunchbarExpanded(state)
});

export default connect(mapStateToProps)(LaunchbarContainer);

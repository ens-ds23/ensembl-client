import React, { FunctionComponent, memo } from 'react';
import { connect } from 'react-redux';

import { getEnabledCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';

import Launchbar from './Launchbar';
import { getLaunchbarExpanded } from '../headerSelectors';

import { RootState } from 'src/store';
import { CommittedItem } from 'src/content/app/species-selector/types/species-search';

type StateProps = {
  launchbarExpanded: boolean;
  committedSpecies: CommittedItem[];
};

type OwnProps = {};

type LaunchbarContainerProps = StateProps & OwnProps;

export const LaunchbarContainer: FunctionComponent<
  LaunchbarContainerProps
> = memo((props) => <Launchbar {...props} />);

const mapStateToProps = (state: RootState): StateProps => ({
  launchbarExpanded: getLaunchbarExpanded(state),
  committedSpecies: getEnabledCommittedSpecies(state)
});

export default connect(mapStateToProps)(LaunchbarContainer);

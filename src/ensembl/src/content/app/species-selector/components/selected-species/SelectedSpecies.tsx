import React, { FunctionComponent, useState } from 'react';
import classNames from 'classnames';
import noop from 'lodash/noop';

import CloseButton from 'src/shared/close-button/CloseButton';

import styles from './SelectedSpecies.scss';

import { CommittedItem } from 'src/content/app/species-selector/types/species-search';

type Props = {
  species: CommittedItem;
  onToggleUse: (genomeId: string) => void;
  onRemove: (genomeId: string) => void;
};

const SelectedSpecies: FunctionComponent<Props> = (props) => {
  const [hovered, setHovered] = useState(false);
  const setHoveredState = () => setHovered(true);
  const setUnhoveredState = () => setHovered(false);

  const {
    common_name,
    scientific_name,
    assembly_name,
    isEnabled
  } = props.species;

  const displayName = common_name || scientific_name;

  const className = classNames(styles.selectedSpecies, {
    [styles.selectedSpeciesDisabled]: !isEnabled
  });

  return (
    <div
      className={className}
      onMouseOver={setHoveredState}
      onMouseLeave={setUnhoveredState}
    >
      <span className={styles.name}>{displayName}</span>
      <span className={styles.assembly}>{assembly_name}</span>
      {hovered && <SelectedSpeciesOverlay {...props} />}
    </div>
  );
};

const SelectedSpeciesOverlay: FunctionComponent<Props> = (props) => {
  const { genome_id, isEnabled } = props.species;

  const text = isEnabled ? 'Do not use' : 'Use';

  const handleClick = () => {
    props.onToggleUse(genome_id);
  };
  const removeSpecies = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    props.onRemove(genome_id);
  };

  return (
    <div className={styles.selectedSpeciesOverlay}>
      <span className={styles.overlayText} onClick={handleClick}>
        {text}
      </span>
      <div className={styles.closeButtonContainer} onClick={removeSpecies}>
        <CloseButton inverted={true} onClick={noop} />
      </div>
    </div>
  );
};

export default SelectedSpecies;

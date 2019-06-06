import React from 'react';
import classNames from 'classnames';

import StrainSelector, {
  Strain
} from 'src/content/app/species-selector/components/strain-selector/StrainSelector';

import styles from './PopularSpeciesButton.scss';

type Props = {
  isSelected: boolean;
  species: string;
  strains: Strain[];
  onClick: () => void;
  onStrainSelect: () => void;
};

const PopularSpeciesButton = (props: Props) => {
  const { isSelected, species, strains, onClick, onStrainSelect } = props;

  const { ReactComponent: Icon } =
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    require(`src/content/app/species-selector/assets/icons/${species}.svg`);

  const className = classNames(styles.popularSpeciesButton, {
    [styles.popularSpeciesButtonActive]: isSelected
  });

  return (
    <div className={className} onClick={onClick}>
      <Icon />
      {isSelected && Boolean(strains.length) && (
        <StrainSelector strains={strains} onSelect={onStrainSelect} />
      )}
    </div>
  );
};

PopularSpeciesButton.defaultProps = {
  strains: []
};

export default PopularSpeciesButton;

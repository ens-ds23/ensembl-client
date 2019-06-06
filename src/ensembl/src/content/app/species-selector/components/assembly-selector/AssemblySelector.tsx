import React from 'react';
import { connect } from 'react-redux';
import find from 'lodash/find';

import { changeAssembly } from 'src/content/app/species-selector/state/speciesSelectorActions';
import {
  getCurrentSpeciesGenomeId,
  getCurrentSpeciesAssemblies
} from 'src/content/app/species-selector/state/speciesSelectorSelectors';

import Select from 'src/shared/select/Select';

import { RootState } from 'src/store';
import { Assembly } from 'src/content/app/species-selector/types/species-search';

import styles from './AssemblySelector.scss';

type Props = {
  genomeId: string | null; // id of selected species; will correspond to genome_id field of an Assembly
  assemblies: Assembly[];
  onSelect: (genomeId: string) => void;
};

const label = <span className={styles.assemblySelectorLabel}>Assembly</span>;

export const AssemblySelector = (props: Props) => {
  const selectedAssembly = find(
    props.assemblies,
    (assembly) => assembly.genome_id === props.genomeId
  );

  if (!props.genomeId || !selectedAssembly) {
    // this branch covers the case when there are no items in props.assemblies
    return null;
  } else if (props.assemblies.length === 1) {
    // if there's just one assembly, there is no point in showing the select element
    return (
      <>
        {label}
        <span>{selectedAssembly.assembly_name}</span>
      </>
    );
  } else {
    // more than one item in props.assemblies
    const options = props.assemblies.map((assembly) => ({
      value: assembly.genome_id,
      label: assembly.assembly_name,
      isSelected: props.genomeId === assembly.genome_id
    }));
    return (
      <>
        {label}
        <Select options={options} onSelect={props.onSelect} />
      </>
    );
  }
};

const mapStateToProps = (state: RootState) => ({
  genomeId: getCurrentSpeciesGenomeId(state),
  assemblies: getCurrentSpeciesAssemblies(state)
});

const mapDispatchToProps = {
  onSelect: changeAssembly
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(AssemblySelector);

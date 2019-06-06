import React from 'react';
import { storiesOf } from '@storybook/react';
import { action } from '@storybook/addon-actions';

import SelectedSpecies from 'src/content/app/species-selector/components/selected-species/SelectedSpecies';

const speciesData = {
  genome_id: 'some_genome_id',
  reference_genome_id: null,
  common_name: 'Human',
  scientific_name: 'Homo sapiens',
  assembly_name: 'GRCh38',
  isEnabled: true
};

const Container = (props: { children: React.ReactNode }) => (
  <div style={{ padding: '40px' }}>{props.children}</div>
);

storiesOf('Components|Species Selector/Selected Species', module)
  .add('enabled', () => (
    <Container>
      <SelectedSpecies
        species={speciesData}
        onToggleUse={action('toggle use')}
        onRemove={action('remove species')}
      />
    </Container>
  ))
  .add('disabled', () => (
    <Container>
      <SelectedSpecies
        species={{ ...speciesData, isEnabled: false }}
        onToggleUse={action('toggle use')}
        onRemove={action('remove species')}
      />
    </Container>
  ));

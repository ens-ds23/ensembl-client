/*

Shape of a strain object:

{
  genome_id: string;
  display_name: string;
};

*/

// result of request for strains of the reference mouse species

// QUESTION: what does the endpoint look like? It should accept reference species as a parameeter

export default {
  reference_genome_id: 'mus_musculus', // <-- genome id of reference species
  strains: [
    {
      genome_id: 'mus_musculus', // <-- I guess the reference species should also be included
      display_name: 'Mouse'
    },
    {
      genome_id: 'mus_musculus_1',
      display_name: 'Mouse 129S1/SvImJ'
    },
    {
      genome_id: 'mus_musculus_2',
      display_name: 'Mouse A/J'
    },
    {
      genome_id: 'mus_musculus_3',
      display_name: 'Mouse AKR/J'
    },
    {
      genome_id: 'mus_musculus_4',
      display_name: 'Mouse BALB/cJ'
    },
    {
      genome_id: 'mus_musculus_5',
      display_name: 'Mouse C3H/HeJ'
    },
    {
      genome_id: 'mus_musculus_6',
      display_name: 'Mouse C57BL/6NJ'
    },
    {
      genome_id: 'mus_musculus_7',
      display_name: 'Mouse CAST/EiJ'
    },
    {
      genome_id: 'mus_musculus_8',
      display_name: 'Mouse CBA/J'
    },
    {
      genome_id: 'mus_musculus_9',
      display_name: 'Mouse DBA/2'
    },
    {
      genome_id: 'mus_musculus_10',
      display_name: 'Mouse FVB/NJ'
    },
    {
      genome_id: 'mus_musculus_11',
      display_name: 'Mouse LP/J'
    },
    {
      genome_id: 'mus_musculus_12',
      display_name: 'Mouse NOD/ShiLtJ'
    },
    {
      genome_id: 'mus_musculus_13',
      display_name: 'Mouse NZO/HlLtJ'
    }
  ]
};

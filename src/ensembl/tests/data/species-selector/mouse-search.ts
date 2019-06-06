/*

REFERENCE - shape of a single match:

{
  "genome_id": string, // a unique identifier (provisional field name)
  "reference_genome_id": string | null, // nullable field; present if the match is a subgroup (e.g. strain) of a species
  "common_name": string | null, // common name (if present)
  "scientific_name": string, // should always be present
  "subtype": string | null, // any extra information about the species (e.g. assembly_name for Human)
  "matched_substrings" : [
      {
          "length" : number,
          "offset" : number,
          "match": "common_name" | "scientific_name" | "subtype" // name of the matched field
      }
    ]
}

*/

// sample response for typing "mou" in the search box

import { MatchedFieldName } from 'src/content/app/species-selector/types/species-search';

export default {
  // notice that it is an array of groups of matches
  matches: [
    [
      {
        genome_id: 'mus_musculus',
        reference_genome_id: null,
        common_name: 'Mouse',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_1',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse 129S1/SvImJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_2',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse A/J',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_3',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse AKR/J',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_4',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse BALB/cJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_5',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse C3H/HeJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_6',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse C57BL/6NJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_7',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse CAST/EiJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_8',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse CBA/J',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_9',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse DBA/2',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_10',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse FVB/NJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_11',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse LP/J',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'microcebus_murinus',
        reference_genome_id: null,
        common_name: 'Mouse Lemur',
        scientific_name: 'Microcebus murinus', // <-- a lemur!
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_12',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse NOD/ShiLtJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'mus_musculus_13',
        reference_genome_id: 'mus_musculus',
        common_name: 'Mouse NZO/HlLtJ',
        scientific_name: 'Mus musculus',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      }
    ]
  ]
};

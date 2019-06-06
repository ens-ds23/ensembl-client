/*

Shape of an assembly data object

{
  assembly_name: string;
};

*/

// should return a list of assemblies for a given strand

// Question: what does the endpoint look like?
// Question: an array or an array of arrays (i.e. of groups)?

export default {
  alternative_assemblies: [
    {
      assembly_name: 'GRCm38.p6',
      genome_id: 'mus_musculus'
    },
    {
      assembly_name: 'GRCm38.p5',
      genome_id: 'mus_musculus_bdc'
    }
  ]
};

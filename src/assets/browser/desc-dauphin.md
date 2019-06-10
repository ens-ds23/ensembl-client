# Dauphin

## Types

Dauphin types are built from atoms. The currently defined atomic types are

  * **bytes** : a sequence of bytes (note that the *atom* is itself a byte sequence)
  * **float** : a float
  * **string** : a UTF8-string
  * **boolean** : true/false

Atoms may be assembled into structured types. Permissible structures are:

  * **structs**: records of a defined shape accessed by keys referencing values;
  * **tuples**: records of a defined length accessed by indexes referencing values;
  * **enums**: descriminated unions of values;
  * **vectors**: sequences of values of the same type;

The above structured types may be nested but not defined recursively (ie they must be of finite, specified depth).

## Types

Every variable in dapuhin has a known type which can be explicit or implicit. The syntax for explicit types is:

 * **atoms**: *bytes*, *float*, *string*, *boolean*
 * **structs**: *{ key: type, key, type, ...}*
 * **enums**: *\<name(type), name(type), ...\>*
 * **tuples**: *(type,type)*
 * **vectors**: *\[type\]*

For example:

 * `boolean`: a single boolean
 * `[float]` : vector of floats
 * `[[float]]`: a two-dimensional vector of floats
 * `[{ start: float, end: float, strand: boolean }]` a vector of start/end/strand structs
 * `[(float,float)]` a vector of float pairs
 * `[<gene(string), transcript((string,string))>]` a vector of genes or transcripts, genes having one string argument, transcripts a tuple of two.

Note that brackets always introduce a tuple and so `(string)` is different to `string`, for example.

## Vectors

Vectors are the fundamental structuring type of dauphin, the only structure which is carried over to dáuphin, the only structure which is not mere syntactic-sugar, and the only structure which can represent an aribtrary length of data.

### Driving Arguments

As vector operations take as arguments quantities of variable length, a strategy is needed to handle vectors of varying length. The *driving* argument of an operation determines the size of the ultimate output. Other sequences are either incompletely used (if longer than the driving sequence), or wrap around (if shorter).

For exmaple, consider a hypothetical opertaion `(+)` which adds its arguments and
for which the first value is its driving argument. In this case 

  * `[1,2,3,4,5] (+) [1] = [2,3,4,5,6]`
  * `[1] (+) [1,2,3,4,5] = [2]`
  * `[1,2,3,4,5] (+) [1,0] = [2,2,4,4,6]`
  * `[1,0] (+) [1,2,3,4,5] = [2,3]`

For assignment this rule is generally also held. A special assignment operator `::=` switches the driving argument to the target of the assignment. The current length of the assignment target then being used as the driving length.

### Vector Predicates

Vector predicates are fundamental to dauphin. Under the hood most types are mappable to sets of vectors and many apparently unrelated syntaxes to vector predicates.

A vector predicate is a predicate which filters the indices of a vector of some operation. This predicated can be expressed in terms of its index (through the use of `@`) or its value (throught the use of `$`). For example, for the value `x`

* `x[@==1] := 3` sets index 1 of x to 3
* `x[$==1] := 3` sets all values of x which are 1 to 3
* `x[@%2==1 && $!=0] ::= y[@%2==0]` sets the non-zero odd values of x to the even values of y.

# Tánaiste (Dauphin bytecode)

## Design Considerations

## Types

### Atomic Types, Varieties and Values

Tánaiste values are (one-dimensional) sequences of atoms. Atoms can never be accessed directly, only on a per-sequence basis.

There are a number of atomic types. These types are, in general, not interchangable in opcodes. They correspond to the atomic types of dauhpin. 

Additionally, the boolean type comes in three varieties. These varieties *are* interchangable in all circumstances and exist for performance reasons to retain a compact representation of a potentially sparse sequence.

Each atomic type has a unique mapping of its values to **true** or to **false**, called its *truthiness*.

The currently defined atomic types are:

  * **bytes** : a sequence of bytes (note that the *atom* is itself a byte sequence)
  * **float** : a float
  * **string** : a UTF8-string
  * **boolean** : true/false
      *  **direct boolean**: a simple array of booleans
      *  **run-length boolean**: an array of runs of booleans
      *  **indexed boolean**: an array which is entirely of one boolean value with the exception of given indices.
  
The atoms are false if and only if they are:

  * **bytes**: length zero
  * **float**: zero or NaN
  * **string**: length zero
  * **boolean**: false.

Where it makes sense to interconvert atoms, dedicated opcodes are provided
for such.

Tánaiste values are sequences of atoms. Tánaiste allows other operations to be performed through providing methods which allow a subsequence to be manipulated at negligible overhead.

There are no multi-dimensional arrays in tánaiste. The multi-dimensional
arrays of dauphin are emulated with sets of registers containing indexes,
lengths etc, and handled by the dauphin compiler.

In most opcodes, the empty sequence functions effectively as a null argument in other languages: invoking special behaviour (such as non-application or default behaviour)
or invokes a fault. The behaviour on an argument being null is up to each individual opcode.

### Driving argument

Most tánaiste opcodes take multiple values which, being sequences, can be of different
length. For most opcodes a single argument, known as the *driving argument* is used to determine the number of operations performed. This ocncept is inherited from dauphin (see the explanation there).

### Register File, Continuation File, and Stack

A conceptually-infinite *main register file* and a stack are implemented for
storing values. The main register file is zero-indexed and implemented as a
vector, so register usage should be managed during compilation to keep values small.

In addition to the main register file is a *continuation register file*. This register file is also zero-indexed and conceptually-infinite. However, no evaluation is performed until the read, but is re-performed *each* read. Opcodes with only continuation inputs can generate continuation outputs, none of them evaluated until each read. (They therefore work like chained iterators). Literals can be created directly within the continuation register file and values copied between it and the main register file.

The continuation register file is desgined:

* for expression intermediates generated by a compiler and used just once;
* for large data streams;
* for largely linear sequences of data manipulation.

There is no other storage.

### Minimal conditionals and loops

Conditionals and loops are best avoided in tánaiste and dauphin alike as they are shockingly inefficient. Instead, an alternative construction can usually be found using sequence index or value predicates.

## Mapping dauphin types

A dauphin value in general maps to a set of tánaiste registers.

The component parts of a dauphin value are separated into vector and non-vector parts. This is known as resolving into *primes*.
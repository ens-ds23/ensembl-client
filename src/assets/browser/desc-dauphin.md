# Tánaiste (Dauphin bytecode)

## Design Considerations

## Types

### Atoms

Tánaiste values are sequences of atoms. Atoms can never be accessed
directly, only on a per-sequence basis. There are a number of atomic
types which are, in general not interchangable in opcodes.

Each type of atom has a unique mapping to **true** or **false**.

The currently defined atoms are

  * **bytes** : a sequence of bytes (note that the *atom* is itself a byte sequence)
  * **float** : a float
  * **string** : a UTF8-string
  * **boolean** : true/false
  
The atoms are false if and only if they are:

  * **bytes**: length zero
  * **float**: zero or NaN
  * **string**: length zero
  * **boolean**: false.
  
### Values

Tánaiste values are sequences of atoms. Tánaiste provides methods to allow
a subsequence of a sequence to be manipulated at negligible overhead.
These methods can be based on position or on contained value. 

For simplicity, tánaiste represents this filtering as separate opcodes
which gives the appearance of inefficiency. However, the interpreter
taks care to lazily implement these methods to ensure efficiency.

### Driver argument

Most opcodes take multiple values which, being sequences can be different
lengths. For most opcodes a single value, known as the driver argument is
used to determine the number of operations executed (atoms generated, etc).
Other sequences are either incompletely used (if longer) or wrap around
(if shorter).

For exmaple, consider a hypothetical opertaion (+) which adds values and
for which the first value is the driver argument. In this case 

  * `[1,2,3,4,5] (+) [1] = [2,3,4,5,6]`
  * `[1] (+) [1,2,3,4,5] = [2]`
  * `[1,2,3,4,5] (+) [1,0] = [2,2,4,4,5]`
  * `[1,0] (+) [1,2,3,4,5] = [2,3]`
  
### Empty Sequence

The empty sequence functions effectively as a null argument which either
invokes special behaviour (such as non-applicaiton or default behaviour)
or invokes a fault. Which occurs is up to each individual opcode.

### Atom Interconversion

Where it makes sense to interconvert atoms, dedicated opcodes are provided
for such.

### No multi-dimensional arrays

There are no multi-dimensional arrays in tánaiste. The multi-dimensional
arrays of dauphin are emulated with sets of registers containing indexes,
lengths etc, and handled by the dauphin compiler.

### Registers and Stack

A conceptually-infinite register file and stack are implemented for
storing values. The register file is zero-indexed and implemented as a
vector so register usage should be managed. There is no other storage.

### Minimal conditionals and loops



## Interpretter implementation

### Continuation Values

Many tánaiste opcodes generate not values, but continuations. Continuations
work like iterators in that they yield values but may also be composed.
Composition can be implemented more efficiently than the naive approach.

Essentially, any tánaiste opcode which only internally transforms data
(pure opcodes) is implemented only in terms of continuations. The exception to
this is jumps which also cause reification of values to avoid the need
for very large continutation compositions in loops. An explicit reification
step allows a continuation to be replaced by its value for memoization
purposes, which is obviously more important here.


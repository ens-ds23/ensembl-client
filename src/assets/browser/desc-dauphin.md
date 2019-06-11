# Dauphin Language Specification

## Design Considerations

Dauphin is an odd language, and it is odd for a reason that should be explained up front.

The design is driven by the need for efficient handling of data in the *interpreter* (tánaiste) in a challenging environment. We have to work our way through an awful lot of data in a real-time, embeded environment, to be able to manipulate the data arbitrarily, but to minimise implementation effort. Without these constraints, many other languages would serve our purposes and dauphin would not exist at all.

The main efficiency bound on tánaiste is to minimise the number of instruction dispatches. Between dispatches, time-consuming operations must take place (such as checking timing). We cannot afford anything which scales the number of dispatches by the data size: we need a vector language.

At the same time, our data is moderately rich and of various complex, structured types (it's not, for example, merely real-valued time series data).

Dauphin is designed to paper over the complexity and strangeness of tánaiste. As a domain-specific application, some consideration is made to making common tasks easy even if it makes rare ones rather odd.

Much of the discussion below specifies the behaviour of the odd corners of tánaiste and dauphin and this document is not a good place to learn the language.

## Types

Dauphin types are built from atoms. The currently defined atomic types are

  * **bytes** : a sequence of bytes (note that the *atom* is itself a byte sequence)
  * **float** : a float
  * **string** : a UTF8-string
  * **boolean** : true/false

Atoms may be assembled into structured types. Permissible structures are:

  * **structs**: records of a defined shape, accessed by keys referencing values;
  * **tuples**: records of a defined length, accessed by indexes referencing values;
  * **enums**: descriminated unions of values;
  * **vectors**: sequences of values of the same type;

The above structured types may be nested, but not defined recursively (ie they must be of finite, specified depth).

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
 * `[<gene(string), transcript((string,string))>]` a vector of genes or transcripts, genes having one string argument, transcripts a tuple of two strings.

Note that brackets always introduce a tuple, and so `(string)` is different to `string`, for example.

### Reference Expressions

A reference expression accesses the inside of a (potentially complex, structured) type to retrieve, set, update or delete values. In dauphin, reference expressions are always sequences of zero-or-more references. Reference expressions are represented in documentation by the meta-syntactic convention `«x,x,...»` to distinguish it from dauphin syntax.

In particular, the reference expression for a variable `x` is the set `«x»` and qualifiers of 

  * a **struct** replace each value in the reference expression by the value of the corresponding key;
  * a **tuple** replace each value in the reference expression by the value of the corresponding index;
  * an **enum** replaces each value in the reference expression which matches the enum with the enum contents and removes any which do not match
  * a **vector** replaces each value in the reference expression by the union of the set of values matching the given predicate.
  
For example, for a variable `x` of type `(float,[<a(string),b(string)>])` and value `(42,[a(1),b(2),a(3)])`:

  * `x` matches `«(42)»`
  * `x.0` matches `«42»`
  * `x.1` matches `«[a(1),b(2),a(3)]»`
  * `x.1[0]` matches `«a(1)]»`
  * `x.1[@<2]` matches `«a(1),b(2)»`
  * `x.1[@<2].b` matches `«2»`
  * `x.1[@=1]` matches `«b(2)»`
  * `x.1[@=1].a` matches `«»`

Operations are defined in terms of their behaviour with respect to reference expressions, including their behaviour when vectors differ in length. (See the discussion on *driving expressions* below).


## Vectors

Vectors are the fundamental structuring type of dauphin. They are

  * the only structure which is carried over to tánaiste, 
  * the only structure which is not mere syntactic-sugar, and 
  * the only structure which can represent an aribtrary length of data.

### Driving Expressions

As expressions take as arguments quantities of variable length (reference expressions), a strategy is needed to handle arguments of reference expressions of varying length. Most operations use *driving expressions* to manage this. One argument is identified as the driving expression in the operation definition. This driving expression determines the size of the ultimate output. Other expressions are either incompletely used (if longer than the driving expression), or wrap around (if shorter).

For exmaple, consider the opertaion `(+)` which adds its arguments and for which the *first* value is its driving expression. In this case 

  * `«1,2,3,4,5» (+) «1» = «2,3,4,5,6»`
  * `«1» (+) «1,2,3,4,5» = «2»`
  * `«1,2,3,4,5» (+) «1,0» = «2,2,4,4,6»`
  * `«1,0» (+) «1,2,3,4,5» = «2,3»`
 
 For the above examples in concrete syntax (see later secion on *vector predicates*), if `x := [0,1,2,3,4,5]`:
 
  * `x := x[@>0] (+) x[$=1]` then `x: [2,3,4,5,6]`
  * `x := x[$=1] (+) x[@>0]` then `x: [2]`
  * `x := x[@>0] (+) x[$=1,$=0]` then `x: [2,2,4,4,6]`
  * `x := x[$=1,$=0] (+) x[@>0]` then `x: [2,3]`
 
### Vector Predicates

A vector predicate is a predicate which filters the indices of a vector of some operation. This predicate can be expressed in terms of its index (through the use of `@`) or its value (throught the use of `$`). For example, for the value `x`

* `x[@=1] := 3` sets index 1 of x to 3
* `x[$=1] := 3` sets all values of x which are 1 to 3

A sequence of predicates, separated by comma, concatenates the lists generated by each predicate in turn. For example, if `x := [1,2,3,4,5]` then `x[@>2,$>1]` creates reference expression `«4,5,2,3,4,5»` where the `4` and `5` refer to the same cell. In this case, if used as an lvalue, those cells would be updated twice! For example, `x := x[@>2,$>1] (+) 1` would set x to `[1,3,4,6,7]`. This behaviour is defined, but is not behaviour to deliberately exploit in clean code.

### Vector Predicate Mini-Language

Essentially all dauphin conditionals occur inside vector predicates. The following values are provided:

 * `@` current position (zero-based)
 * `$` current value
 
and the following predicates:

 * `=` compare for equality
 * `>`, `<` greater, less (resp)
 * `!(pred)` negation of pred
 * `!=` shorthand for `!(..=..)`
 * `&`,`|` and, or (resp)

**TODO**: expression mini-language or inherit. Inherit simpler but can we do it given indexes?

# Tánaiste (Dauphin bytecode)

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
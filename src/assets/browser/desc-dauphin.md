# Dauphin Language Specification

## Design Considerations

Dauphin is an odd language, and it is odd for a reason that should be explained up front, unless the motivation hangs over the user or implementer.

The design is driven by the need for efficient handling of data in the *interpreter* (tánaiste), which runs in a challenging environment. The interpreter has to work its way through large volumes of data in a real-time, embeded environment and to be able to manipulate the data arbitrarily but at minimal implementation effort or code size.

Without these constraints, many other languages would serve our purposes and dauphin would not exist at all. (For example Lua is a somewhat close fit as, in its own way is R)

That said, Dauphin is not as odd as it might seem to generalist software developers. It is clearly inspired by languages like Matlab, R, and so on, which are common in scientific computing.

The main efficiency bound on the tánaiste interpereter is to minimise the number of instruction dispatches. Between dispatches, time-consuming operations must take place (such as checking timing which can easily amount to a significant fraction of a millisecond). In addition, optimising inter-instruction overhead is a classic pinch-point of interpreted languages. We cannot afford anything which scales the number of dispatches by the data size (which would, in turn, scale to tens of seconds). We need a vector language.

At the same time, our data is moderately rich and comprises various complex, structured types (it's not, for example, merely real-valued time-series data).

Dauphin is designed to somewhat paper over the complexity and strangeness of tánaiste and to allow richer type expression. As a domain-specific application, some consideration is also made to making common in-domain tasks easy even if it makes rare ones rather odd.

Much of the discussion below specifies the behaviour of the odd corners of tánaiste and dauphin, and so this document is not a good place to learn the language and makes both look horribly complex.

## Values

Dauphin types are built from atoms each of an *atomic type*. The currently defined atomic types are:

  * **bytes** : a sequence of bytes (note that the *atom* is itself a byte sequence)
  * **float** : a float
  * **string** : a UTF8-string
  * **boolean** : true/false

Atoms may be assembled into structured types. Permissible structures are:

  * **structs**: records of a defined shape, accessed by keys referencing values;
  * **tuples**: records of a defined length, accessed by indexes referencing values;
  * **enums**: descriminated unions of values;
  * **vectors**: sequences of values of the same type;

The above structured types may be nested, but not defined recursively (ie they must be of finite, explicitly-specified depth without placeholders).

### Types

Every variable in dapuhin has a known typeat all times which can be annotated or inferred. The syntax for explicit types is:

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

### Constants

A constant which does not contain an enum nor empty vectors is always unambiguously determined by its syntax. Constants which *do* contain enums or empty vectors are ambiguous and may need annotation, or may be deduced from their use.

 * `true`, `false`: *boolean values*
 * any number: *float*
 * `(constant,constant)`: *tuple*
 * `{ key: constant, key: constant }`: a *struct*
 * `<type(data)>` : an `enum` value
 * `[value,value,...]`: a vector

### Constructiong Expressions

A constructing expression can be constructed as a constant with the addition to

 * a reference expression
 * a variable.

For example if `x := 2` then `[2]` is both a constant *and* a constructing expression, but `[x]` is *only* a constructing expression, not a constant. Note that when used as lvalues, constructing expressions are actually *deconstructing* For example `z := (0,1); (x,y) := z` is equivalent to `x:=0; y:= 1`.

### Reference Expressions

A reference expression accesses the inside of a (potentially complex, structured) type to retrieve, set, update or delete values. In dauphin, reference expressions are always sequences of zero-or-more references. Reference expressions are represented in this documentation and elsewhere by the meta-syntactic convention `«x,x,...»` to distinguish it from dauphin syntax and the `«»` are never literally valid in tánaiste or Dauphin.

In particular, the constructing expression which constructs `x` is the set `«x»`, and qualifiers of: 

  * a **struct** replace each value in the reference expression by the value of the corresponding key;
  * a **tuple** replace each value in the reference expression by the value of the corresponding index;
  * an **enum** replaces each value in the reference expression which matches the enum with the enum contents and removes any which do not match
  * a **vector** replaces each value in the reference expression by the union of the set of values matching the given predicate in the order in which they occur in the vector.
  
For example, for a variable `x` of type `(float,[<a(string),b(string)>])` and value `(42,[a(1),b(2),a(3)])`:

  * `x` matches `«(42,[a(1),b(2),a(3)])»`
  * `x.0` matches `«42»`
  * `x.1` matches `«[a(1),b(2),a(3)]»`
  * `x.1[0]` matches `«a(1)]»`
  * `x.1[@<2]` matches `«a(1),b(2)»`
  * `x.1[@<2].b` matches `«2»`
  * `x.1[@==1]` matches `«b(2)»`
  * `x.1[@==1].a` matches `«»`

Operations are individually defined in terms of their behaviour with respect to reference expressions, including their behaviour when vectors differ in length. (See the discussion on *driving expressions* below).

### Driving Expressions

As expressions take as their arguments quantities of variable length (reference expressions), a generally consistent strategy is needed to define how operations handle arguments of reference expressions of varying length. Most operations use *driving expressions* to define this. One argument is identified as the driving expression, in the operation definition. This driving expression determines the size of the ultimate output. Other expressions are either incompletely used (if longer than the driving expression), or wrap around (if shorter).

For example, consider the opertaion `+` which adds its arguments, and for which the *first* value is its driving expression. In this case 

  * `«1,2,3,4,5» + «1» = «2,3,4,5,6»`
  * `«1» + «1,2,3,4,5» = «2»`
  * `«1,2,3,4,5» + «1,0» = «2,2,4,4,6»`
  * `«1,0» + «1,2,3,4,5» = «2,3»`
 
 For the above examples in concrete syntax (see later secion on *vector predicates*), if `x := [0,1,2,3,4,5]`:
 
  * `x := x[@>0] (+) x[$==1]` then `x: [2,3,4,5,6]`
  * `x := x[$==1] (+) x[@>0]` then `x: [2]`
  * `x := x[@>0] (+) x[$==1,$==0]` then `x: [2,2,4,4,6]`
  * `x := x[$==1,$==0] (+) x[@>0]` then `x: [2,3]`

## Vectors

Vectors are the fundamental structuring type of dauphin. They are

  * the only structure which is carried over to tánaiste, 
  * the only structure which is not mere syntactic-sugar, and 
  * the only structure which can represent an aribtrary length of data.
 
### Vector Predicates

A vector predicate is a predicate which filters the indices of a vector. This predicate can be expressed in terms of its index (through the use of `@`) or its value (throught the use of `$`). For example, for the value `x`

* `x[@==1] := 3` sets index 1 of x to 3
* `x[$==1] := 3` sets all values of x which have value 1 to 3

Predicates always retain the order of the underlying vector.

### Vector Predicate Mini-Language

Essentially all dauphin conditional behaviour occurs inside vector predicates. The following values are provided:

 * `@` current position (zero-based)
 * `$` current value
 * a constant
 * any expression defined at the start of the statement which evaluates to a boolean-valued reference expression (true if *any* value in the reference expression is true)
 
and the following predicates:

 * `==` compare for equality
 * `>`, `<` greater, less (resp)
 * `!` negation of pred
 * `!=` shorthand for `!(..=..)`
 * `&`,`|` and, or (resp)

An empty mini-language expression denotes "all".

### Reference Expression Syntax

A reference expression is expressed as:

 * a structuring expression;
 * a variable;
 * a reference expression followed by `[(mini-language)]` 
 * a reference expression followed by `.selector` where selector is
	 - a positive or zero integer for tuples
	 - a key in standard-id-syntax (see below) for structs or enums

### General Expressions

A general expression is equivalent to a reference expression and is built from zero-or-more reference expressions which are computed as functions.

### Operations

An operation is a dauphin statement. Most operations are represented by 

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
# Getting started with flare
Flare is statically typed compiled language so i'll start with types.


## Types
There are only 3. primitive types in flare:
1. **bool**:```true```/```false```
2. **string**:text values
3. **numb**:floats and intigers in one

And only 2. primitive values:

1. ```true```/```false```:**bool** values

Flare is statically type so you can't multiply bool by string etc. So this would be invalid:

```flare
"hello" + 5
//or
true * "flare"
//you probably know how it works now
```

## Variables

### Variable decleration
```flare
var foo = "Hello world";

// Or you can do
var bar:string = "Hello world";

// Or
var hello:string;

//But you can't do this becouse flarec cannot infer type
var this_wont_work;
```
### Constant decleration
```flare
const bar = "Hello";
//etc.
//but you cant do this
const x:string;//since it doesn't have value it's useless
```
### Assigning values to variables

```flare
var x:string;
x = "hello";
// but this is invalid
x = true //becouse x is of typed string but true is of type bool
//etc.
```

# Macros
### What are Macros:
Macros are build-in functions hard coded to the language it self. You can tell them apart from normal functions becouse they have '!' at the end of identifier. 

---

## Output and  input macros
### Input
For input, there's ```readInput!()``` macros. It takes one argument of type **printable**. Then it print out the argument without \n. Then it reads input from console  and returns it as a **string**

---

**Example:**
-

```flare
const userName:string = readInput!("Whats your name?");
writeLn!("Your name is",userName)
```
```bash
$ flarec exec input.flare input.out
```
```
Whats your name?simon
Your name is simon
```
### Output
For output, there are ```writeLn!()```/```write!()``` macros that takes any amount of printable aruments and writes them to the console.

---

**Example:**
-

```flare
write!("Hello ","world!") // Hello world
writeLn!("Hello ","world!") // Hello
                            // world!
```

## Process macros
### Exiting process
For this, there is ```processExit!()``` macros that take one **numb** argument and exits the program with that code.

--- 

**Example**
-
```flare
const x:numb=5;
if(x>7){
  processExit!(195)
}
else{
  processExit!(x)
}
```
```bash
$ flarec exec process.flare process.out
```
```
Exited with code `5`
```

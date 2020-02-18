### Fibonacci ###
The Fibonacci sequence is a sequence `Fn` of natural numbers defined recursively:

```
 F0 = 0 
      F1 = 1 
      Fn = Fn-1 + Fn-2, if n>1 
```

##### Task #####
Write a function to generate the nth Fibonacci number.
Solutions can be iterative or recursive (though recursive solutions are generally considered too slow and are mostly used as an exercise in recursion).
```
    Fn = Fn+2 - Fn+1, if n<0   
```
support for negative `n` in the solution is optional. 
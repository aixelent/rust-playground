### Password generator ###

##### Task #####
Create a password generation program which will generate passwords containing random ASCII characters from the following groups: 

```
lower-case letters:  a ──► z
         upper-case letters:  A ──► Z
                     digits:  0 ──► 9
Other printable characters:  !"#$%&'()*+,-./:;<=>?@[]^_{|}~ 
(the above character list excludes white-space, backslash and grave) 
```

##### Input data #####
```
Input : arr[] = {10, 20, 4}
Input : arr[] = {20, 10, 20, 4, 100}
```

##### Output data #####
The generated password(s) must include   at least one (of each of the four groups): 
```
lower-case letter, upper-case letter, digit  (numeral), and one  "other"  character. 
```
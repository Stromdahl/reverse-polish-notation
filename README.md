# Reverse Polish Notation Calculator
https://en.wikipedia.org/wiki/Reverse_Polish_notation

Reverse Polish Notation(RPN) is a stack based mathimatical notation which the operators follow their operands.

For example the expression 25 17 + => 42 will add 25 and 17, with is equivilant to 25 + 17 in tradisional notation.

## Implementation Explaniation
When performing the expression, a stack is used to store the current values

When adding two numbers,
```
( 25 17 + )
25 is pushed on the stack
stack: 25
17 is pushed on the stack
stack: 25 17
perform + operation
    17 is popped from the stack
    25 is popped from the stack
    result 42 is pushed on the stack
end of expression
remaining stack prined 42
```

If there is more than one value left on the stack, the stack entire stack will be printed
```
( 4 14 6 + )
results in
4 20
```

# DD1338 Week 15
Edited by: Benjamin Widman

So long for-loops, hello recursion!
This week we are going to learn a language from the functional paradigm: Haskell.

## Task
Solve this Kattis-problem: https://kth.kattis.com/courses/DD1360/progp24/assignments/fymgmo/problems/kth.progp.warmup

## Questions
Know the answers to these questions and be prepared to be able to answer them.

1. Why can you write expressions like this one in Haskell?
```haskell
let evenNumbers = [ 2 * n | n <- [1..]]
print (take 1000000 evenNumbers)
```

2. Will this code work?
```haskell
addOne :: [Int] -> [Int]
addOne (l:ls) = (l+1):(addOne ls)
```

3. What is a pure function? Give an example of a pure and a non-pure function.
 
## Prepare assignment
1. As usual create a repo with the name `<kth-id>-haskell`.
2. Install Haskell via this excellent link: https://www.haskell.org/ghcup/
3. Write solutions to the Kattis problems and screenshot that you've passed the judge.

## Reading material
Up to chapter 6 of the "Haskell book": https://learnyouahaskell.com

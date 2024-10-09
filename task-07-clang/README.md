# DD1337 Week 7

Modified: Dmitry Chirin

## Getting started with C

### Install GCC toolchain
> Note: This is already installed in most linux-distros, so if you are using something like Ubuntu (hint for windows users) you can skip this
1) Install [MingW](https://www.mingw-w64.org/). *Windows-users are recommended to just use WSL*. 
2) Test your installation by entering `gcc --help` in your terminal or command prompt(*host pleb *host).

Congrats dear programmer. You can now compile your C files by entering the following command.
```
gcc ./program.c -o name_of_executable
```
To run: 
```
./name_of_executable
```

### Prepare for your assigment

1) Create a repository named `<KTH_ID>-task-7`.
2) Clone your newly created repository.
3) Create one `.c` file per subassignment. Name then descriptivly.  

For help with code setup, begin by copying the contents of `./kattis_template/main.c` into your `.c` file.

If you're not familiar with the C language I recommend reading up on it online, for example [GeeksForGekks](https://www.geeksforgeeks.org/c-programming-language/).

## Assignment 1 - Kattis

Solve [A Different Problem](https://open.kattis.com/problems/different), which is now on Open Kattis and not KTH's Kattis (i.e. not locked behind courses😄).

See `./minimal_scalar_product` for a Kattis solution example.

**Don't forget** to include screenshots of Kattis to prove that your solutions work.

## Assignment 2.1 - Encryption

Create a program called `encrypt.c` which takes the name of a text file as a command line argument and encrypts the content into a new file prefixed with "encrypted": `encrypted_<previous name>.txt`.

It is optional what encryption algorithm to use but it needs to have some choosible setting and not always encrypt exactly the same way. Some simple recommended ones are:

- [Ceasar cipher](https://en.wikipedia.org/wiki/Caesar_cipher)
- [XOR cipher](https://en.wikipedia.org/wiki/XOR_cipher)

> Note: the algorithm only has to support upper and lowercase A-Z. However, I'm not stopping you from having further support :)

With encryption settings I mean for example ceasar cipher having a specified amount of rotation to apply or XOR cipher having a key. These should be specified as command line arguments along with the name of the text file:
```
./encrypt text.txt 12
```
Write in your `README.md` what algorithm you've implemented and how to use the program.

Which outputs: `encrypted_text.txt`

Command line arguments are retrieved from `argv` when using the following function header:
```c
int main(int argc, char **argv) {}
```
- `int argc` is the argument count, i.e. the number of arguments supplied. This is useful for either error checking that the correct number of arguments where supplied or looping through the arguments when there can be a varied amount of them.
- `char **argv` are the argument values and is a C-style string array since `char*` is a string and arrays are just pointers, hence the double pointer.


## Assignment 2.2 - Decryption

Create a new program called `decrypt.c` which takes the same command line arguments as `encrypt.c` but instead reverses the encryption of the supplied text file. The result shall be in a new file with either the prefix "encrypted" removed or with a new prefix "decrypted" for fun growing names🤪.

## Assignment 3 - Let's up the game

Choose and implement one of the following:
* Combine both ciphers
* Implement another cipher of your choosing
* Continue development on one of your ciphers, and make it more secure and advanced

## Questions

You don't have to write the answers to these but think about them and know the answers during our next lesson.

### Data control

Observe the following code:

```c
int length = 0;
scanf("%d", &length); 

int* vector = malloc(length * sizeof(int));
for (int i = 0; i < length; i++) 
    scanf("%d", &vector[i]);

free(vector);
```

Know the answer of the following question:
- What is happening line for line?

### Gammal hederlig läsförståelse

Observe the following function:

```c
#include <stdio.h>
#include <time.h>

// Assume that foo is a function which takes longer time to execute
// for a larger value n.

void someFunction(void (*f)(int), int milliseconds, int n) {
    int milliseconds_since = 1;
    int end = 1;

    do {
        n = (int)(n * (double)end / milliseconds_since);

        milliseconds_since = clock() * 1000 / CLOCKS_PER_SEC;
        end = milliseconds_since + milliseconds;

        foo(n);

        milliseconds_since = clock() * 1000 / CLOCKS_PER_SEC;
    } while (end - milliseconds_since > 100);

    printf("\nLargest n: %d", n);
}
```

Know the answer of the following question:
- What does `someFunction` do?

### Data types are like whaaat?

Observe the following function:

```c
int x;

printf("Nothing: %d\nGive x a value: ", x);

scanf("%d", x);

printf("\nYour value is: %d\n", x);
```

Know the answer of the following question:
- What is printed?

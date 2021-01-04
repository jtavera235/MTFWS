### Multi-threaded file word searcher

Given a list of words, this program will search the provided files and count how many times each word appears.

The list of words is given as a file, and must be the first command line argument. The next arguments consist of the file/files that the program
will search. Any file that gets specified with the `-` symbol is considered the output file, and is where the result will be displayed. 

An example would be:
```shell 
cargo run words.txt t1.txt t2.txt t3.txt t4.txt -out.txt
```

In this example, the program will use the list of words that is in the words.txt file, then search for these words in t1.txt, t2.txt, t3.txt, and t4.txt. The result of the program will be displayed in the out.txt file.

In the meantime, this program only supports files that have one word per line. It cannot search sentences and only counts exact matches. These are
both features that will be implemented in the future.

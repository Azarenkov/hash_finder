Using!

First of all, you should build project:

    cargo build

after building go to your build:

    cd target/debug

now you can run this build:

    ./hash_finder -N 3 -F 6

-N < Numbers of zeros at the end of hash >

-F < Numbers of hashes >

OR 

you can just run 

     cargo run -- -N 3 -F 6

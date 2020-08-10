# zero-knowledge-sets
Partial implementation of Micali, Rabin, and Kilian Zero-Knowledge Sets
Based on [this Paper](https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Zero%20Knowledge/Zero-Knowledge_Sets.pdf)

## Motivation
I was thinking about implementing an authorization system for online voting that would emulate the security and privacy of in-person paper ballot.
The 3 components are:
- anonymity (given a ballot one cannot find the voter)
- security (only authorized voters can vote and they can vote only once)
- verifiability (given published results voters can check if their vote was properly recorded)

Zero-Knoledge sets provide a framework where implementing this aouthorization system (Full implementation of the voting software is a WIP)

## Result
### Parts implemented
Even though the paper describes Zero-Knowledge Databases, this only implement Zero-knowledge sets of char strings read from a newline-separated list.
Even though the paper describe proof and verification of both membership and non-membership, only membership has been implemented (as non membership is useless for the motivated use case)
### Languages used
The languages used in this impemantation have been choosen by considering the use cases:
- Commiter was written in Rust for speed and safety to convert the list to a zero-knowledge set (secret) and public key
- Prover was written in JS as is is ment to be executed on the voter's computer
- Verifier was written in Python as the Polling Server will be written in Python+Django
- Json is used as the common data interchange between the three programs
### Structure
The goal is to implpement the algorithms described in the paper and to provide inpoint functions that can be called by another program 
(as of now only command line input has been implemented)

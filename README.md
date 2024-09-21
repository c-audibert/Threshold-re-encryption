# Threshold-re-encryption

Secure communications and data storage are major challenges in the
challenges in the field of cryptography. Algorithms based on the
"Learning With Errors" (LWE) problem and Shamir's secret sharing have
have emerged as robust solutions for ensuring information confidentiality and integrity. In this paper, we explore an approach that
combines these two cryptographic techniques to create a 
threshold-based re-encryption system.
Our main objective is to answer the following problem: “A company
needs to secure its vault password. It could use a standard method
but what if the key holder is unavailable or dies?
 What happens if the key is compromised by a malicious hacker
or if the key holder decides to betray the company, and uses his power over the safe to his own advantage?“
To guarantee the security of this password, n colleagues in the company have access to what we call a “share”, a part of the password. It takes _t_ + 1 shares to reconstruct this password,
so that a number of shares lower than _t_ does not allow access
the password.

In the first two sections of the paper, we provide the mathematical basis for the implementation. These include proofs of _t_-privacy, as well as
the formula for reconstruction from shares.
In the next two sections, we look at the 0 and 1-privacy cases for their implementation, including definitions of the _Encryption_ and _Decryption_ functions, and the affine form corresponding to the machine system.

The attached scripts, written by Alexis Mellier, give the implementation of the problem. 


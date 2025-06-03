# Encryption algorithms

## AES encryption

### Key Components

- AES takes a secret key (128, 192, or 256 bits) and expands it into multiple round keys.
- Uses the Rijndael Key Schedule algorithm.
- Each round key is derived the previous key using S-Box substition, XOR operation, and RotWord (byte shifting)

### Initial Round

- AddRoundKey: The plaintext is XORed with the first round key.

### Main Rounds

Each round consists of four transformations:

🔹 1. SubBytes (Non-linear Byte Substitution)
Each byte is replaced using an S-Box (Substitution Box).
The S-Box is designed to resist cryptanalysis attacks.
🔹 2. ShiftRows (Row Shifting for Diffusion)
Rows of the 4x4 state matrix are shifted left.
Row 0: No shift
Row 1: 1-byte shift
Row 2: 2-byte shift
Row 3: 3-byte shift
🔹 3. MixColumns (Mixing for Confusion)
Each column of the state matrix is multiplied by a fixed matrix in Galois Field (GF(2⁸)).
Strengthens diffusion by mixing bytes across columns.
Not used in the final round.
🔹 4. AddRoundKey (XOR with Round Key)
The state matrix is XORed with the round key.

### Final Round

- SubBytes
- ShiftRows
- AddRoundKey

## AES Block Structure

AES processes data in 16-byte (128-bit) blocks, represented as a 4x4 matrix (state array) each transformation modifies this state matrix during encryption.

AES-128 128 bits 10 rounds
AES-192 192 bits 12 rounds
AES-256 256 bits 14 rounds

## AES Encryption & Decryption Process

Encryption follows:

Key Expansion
Initial Round (AddRoundKey)
Main Rounds (SubBytes → ShiftRows → MixColumns → AddRoundKey)
Final Round (SubBytes → ShiftRows → AddRoundKey)
🔹 Decryption reverses the process:

Inverse ShiftRows
Inverse SubBytes (using inverse S-Box)
Inverse MixColumns (except in final round)
AddRoundKey

## AES Modes of Operation

ECB (Electronic Codebook) Weak, encrypts each block independently. Not recommended.
CBC (Cipher Block Chaining) Uses an IV (Initialization Vector) to ensure uniqueness.
CFB (Cipher Feedback) Turns AES into a stream cipher.
OFB (Output Feedback) Similar to CFB but avoids error propagation.
GCM (Galois Counter Mode) Adds authentication, best for modern encryption.

GCM mode is the most secure and widely used today.

## AES Security Strength

AES-128 is secure for general use.
✅ AES-256 is used in military and government applications.
✅ Resistant to brute-force attacks (takes billions of years to break).

🚫 Vulnerabilities:

AES is secure, but poor key management can make it weak.
Side-channel attacks (power analysis, timing attacks) can expose keys

## S-box in AES

The AES S-Box is a 16x16 lookup table that maps each 8-bit input (0x00 to 0xFF) to a unique 8-bit output.
It is designed to be highly non-linear to make AES resistant to algebraic attacks.
The S-Box is not a simple permutation but is mathematically generated using finite field (Galois Field GF(2⁸)) operations.

### Purpose of the S-Box in AES

Confusion (Shuffling bits in a complex way) → Makes it difficult to reverse-engineer the key.
✅ Resistance against Cryptanalysis → Prevents attacks like differential cryptanalysis.
✅ Non-linearity → Ensures that small changes in input result in unpredictable output.

### How the AES S-Box works.

The S-Box transformation consists of two steps:

#### Step 1: Multiplicative Inverse in GF(2⁸)

Each byte 𝑋 (except 0) is replaced with its multiplicative inverse in the finite field GF(2⁸).
The field GF(2⁸) is defined using the irreducible polynomial:

`x^8 + x^4 + x^3 + x + 1`

Example:

`0x53*x = 1 mod (x^8 + x^4 + x^3 + x + 1)`

What is GF(2^8):
`It consists of polynomials of degree <= 7 with coefficients in {0,1} (binary values). Each byte in AES is interpreteed as a polynomial modula an irreducible polunomial`

Example of code with explanation:

<mark>

fn gf*mult(mut a: u8, mut bL u8) -> u8 {
let mut p: u8 = 0; // Result of multiplication
let irreducible: u8 = 0x1B; // Irreducible polynomial (AES standard)
for * in 0..8 {
if (b & 1) != 0 { // If the least significant bit of b is 1
p ^= a; // Add a to the result (XOR for addition in GF(2^8))
}
let carry = (a & 0x80) != 0; // Check if MSB (8th bit) is set
a <<- 1; // Multiply a by x (left shift by 1)
if carry {
a ^= irreducible; // Perform modula reduction by x^8 + x^4 + x^3 + x + 1
}
b >>=1; // Move to the next bit in b
}
}

</mark>

The multiplicative inverse in GF(2^8):
Is inverse of byte b is a value b^-1 such that:
`b x b^-1=1 mod P(x)`

Thie means that multiplying a byte by its inverse in GF(2^8) will produce 1 (identity elemnt for multiplication).

`Example:`

- For b = 0x53, we must find a such that:
  0x53 \* a mod 0a11B = 1
- the result is 0xCA, meaning:
  0x53 x 0xCA = 1 mod 0x11B

##### How compute

1. Initialize two values
   - The irreducible polynomial r0 = 0x11B
   - The given byte r1 = b
   - Two tes t0 = 0 and t1 = 1
2. Perform successive division and remainder calculations:
   - Keep computing r0 / r1 to find quotient q
   - Compute remainder r = r0 XOR (1 \* r1)
   - Update t0 and t1 using modular arithmetic.
3. Stop when r1 = 1, meaning we've found the inverse.

4. Convert 0x53 to binary
5. Find the inverse using extended euclideain algorithm in GF(2^8). Multiplicative inverse of 0x53 in GF(2^8) is 0xCA.

AES S-Box values are 8-bit (1 byte) and padding to 0041 like four digits is unnecessary for that.

b\*x = mod (x^8 + x^4 + x^3 + x + 1) = 1

The inverse ensures non-linearity, which is essential for security.

#### Affine Transformation (Bitwise Mixing)

- After computing the multiplicative inverse, the result is transformed with an affine function (XOR with a fixed constant matrix).
- The affine transformation ensures better diffusion and prevents simple algebraic relations.
  Mathematically, the affine transformation is:

      S(x)=B⊕(M×X)

where:

X is the 8-bit input.
M is a fixed 8×8 matrix.
B is a fixed 8-bit constant (0x63 in hex or 01100011 in binary).
⊕ (XOR) ensures further scrambling.

The equation for each output bit is:

# Theory of code

## Handling Edge Cases

`format!("{:02x}", c as u8)` - Use lowecase "x"
`.join("")` - remoces spaces for a compact format
`.collect::<Vec<String>>()` // collect into a vector of hex strings
`join(" ")` - join with spaces for readability

## ShiftRows (Row Shifting)

Each row of the state matrix is shifted left cuclically.
Mathematical Operations:

- Cyclic Shift (Permutation).
- Formula (for row i):
  Sij = Si(j + i) mod 4
  Example of Calculation:

Before ShiftRows:
0x19 0xA0 0x9A 0xE9
0x3D 0xF4 oxC6 0xF8

After ShiftRows:
0x19 0xA0 0x9A 0xE9
0xF4 0xC6 0xF8 0x3D

### Rust implementation

<mark>

fn shift_rows(state: &mut [[u8; 4]; 4]) {
state[1].rotate_left(1);
state[2].rotate_left(2);
state[3].rotate_left(3);
}

</mark>

### MixColumns (Matrix Multiplication in GF(2^8))

#### Purpose of MixColumns

- Introduce diffusion: One byte change affects the whole column.
- Uses Galois Field (GF(2^8)) multiplication to make encryption stronger.
- Makes AES resistant to linear and differential cryptanalysis.

#### Mathmetical Definition

AES represents data as 4x4 byte matrix. Each column undergoes matrix multiplication with a fixed transformation matrix M in GF(2^8)

Transformation Formula
Each column vector C in the state matrix is multiplied by the MixColumns transformation matrix.

MixColumns transformation matrix M:
C` = M x C

Where
M = [02 03 01 01]
[01 02 03 01]
[01 01 02 03]
[03 01 01 02]

and each column C is:
C = [b0]
[b1]
[b2]
[b3]

After multiplication, we get the new column C`.
C` = [b0`]
[b1`]
[b2`]
[b3`]

# Components of AES

1. generating S_BOX
2. Implement addroundkey
3. Implementing Key_expansion
4. Implementing shift_rows
5. Implementing mix_columns.
6. Implementing sub_bytes.
7. Implementing shift_rows.





# RSA implementation 

## Key generation 
1. Select two large prime numbers p and q(e,g 1024-bit or 2048-bit primes).
2. Compute n (modulus).
    n = p x q
3. Calculate Euler's totient function ()
    f(n) = (p - 1) x (q - 1)
4. Choose a public exponent e such  that:
- 1 < e < f(n)
- e is coprime to f(n), meaning gcd(e, f(n)) = 1 Common choive: e = 65537
5. Compute the private exponent d:
- d is the modular inverse of e modulo f(n) 
    d = e^01 mod f(n)
Key pair: 
- Public Key: (e, n)
- Private Key: (d, n)
### Encryption 
Goal: Convert plaintext message into ciphertext.
Steps: 
1. Convert plaintext into a numerical representation.
2. Compute ciphertext C using the key (e, n);
C = M^e mod n
### Deecryption 
Goal: Recover the original message from the chipertext.
Steps: 
1. Compute the original message < using the private key (d, n)
M = C^d mod n 
### Digital Signature (Optional)
RSA can also be used for signing messages to ensure authenticity.
Signing Process:
1. Compute the message hash (e.g., SHA-256)
2. Encrypt the hash with the private key (d,n):
    S = H(M)^d mod n 
3. Send (M, S) (message  and signature).
Verification Process: 
1. DDecrypt the signature using the public key (e, n)
H` = S^e mod n 
2. Compute the hash of received message.
3. If H` == H(M), the signature is valid.

## Summary of operations
- Key Generation (generate public (e,n) and private (d,n) keys)
- Encryption (convert plaintext M -> chipertext c = M^e mod n)
- Decryption (retrieve plaintext M = c^d mod n)
- Signing (Optional) (Encrypt hash with private key prove  authenticity)
- Verification(Optional) (Decrypt signature and compare hash)
RSA is secure but slow compared modern cryptosystems. It is mostly used for key exchange and signatures, mostly use AES.

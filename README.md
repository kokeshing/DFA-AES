# 128bitAES暗号のDFA実装

## 動作

Rustをインストールした上で以下のように実行.
平文・鍵は先頭の0xを外して入力する.

```
$ cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/dfs_ase`
Input the ciphertext.
456c6b2eb68108431e20e253174f739d
Input the broken ciphertexts.(0)
45266b2e168108431e20e2f4174f379d
Input the broken ciphertexts.(1)
45b56b2ed68108431e20e2c1174f009d
Input the broken ciphertexts.(2)
456caa2eb6ac08435320e253174f73ef
Input the broken ciphertexts.(3)
456c312eb68b0843cf20e253174f73d7
Input the broken ciphertexts.(4)
ad6c6b2eb681085b1e20d2531785739d
Input the broken ciphertexts.(5)
b56c6b2eb68108221e201c5317da739d
Input the broken ciphertexts.(6)
456c6ba5b68175431e00e253f34f739d
Input the broken ciphertexts.(7)
456c6b64b68162431eb4e2537d4f739d
y:
[[6a, 41, 74, 8b]
 [2f, b6, 14, a4]
 [7c, 0b, c8, 54]
 [28, 95, 80, ae]]

The key of 10 round
0x47797b1a35cf23698cda0a9e2a065379
[[47, 35, 8c, 2a]
 [79, cf, da, 06]
 [7b, 23, 0a, 53]
 [1a, 69, 9e, 79]]
```

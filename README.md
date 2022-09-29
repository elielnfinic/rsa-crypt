# RSA-Crypt

Allows to generate RSA public and private keys
You can choose to output them to local files or simply use them on the runtime.

## Structure 

`lib.rs` exposes the following methods : 
- `generate_public_private_keys` : Allows to generate public and private keys. You pass a bytes parameter to force the complexity of your keys and the encryption. You can use `32 bits`, `64 bits`, `128 bits`, `2048 bits` even `4096 bits`. It will take some time for `4096 bits` but it will work. If you pass a second parameter as `true` then the keys will be saved on the files `private.pem` and `public.pem`.<br/>Example 
```
let keys = generate_public_private_keys(256, true);
//or 
let keys = generate_public_private_keys(4096, false);
```
By the way, `generate_public_private_keys` returns a struct of type `Key` that implements `Display` of `std::fmt::Display` to nicely display your keys as string like here :  
```
-----BEGIN PUBLIC KEY-----
MDwwDQYJKoZIhvcNAQEBBQADKwAwKAIhAJeVjSZgKObsBIqkZVczUsMU9n2A6D9S
gdLV0GDkAG7xAgMBAAE=
-----END PUBLIC KEY-----

-----BEGIN PRIVATE KEY-----
MIHCAgEAMA0GCSqGSIb3DQEBAQUABIGtMIGqAgEAAiEAl5WNJmAo5uwEiqRlVzNS
wxT2fYDoP1KB0tXQYOQAbvECAwEAAQIgJ+5NcH6ER9CEocEMsRvkAPgG4s9dfm1S
eBSb+OfFn8ECEQDBWLvA79GAEevo+V/AKwjvAhEAyLRfbtL5pKIe0BJbATdGHwIQ
KDkPhknd6ajQzTzj4JSTXQIRAKabbLkvs4Jh/hgVlKv+uVsCEFnXX13H59xs4Kuc
uaflP6o=
-----END PRIVATE KEY-----

```

- `get_private_key` : Allows to get the private key that was saved in a `pem` file(i.e private.pem, here). I believe, you know `.pem` files, if not, try to search and you will learn about `pkcs` format like `pkcs1` and `pkcs8` that I am currently using.

```
let private_key = get_private_key();
```

- `get_public_key` : Allows to get the public key that was saved in a `pem` file(i.e private.pem, here). I have talked about `pem` and `pkcs` in the previous paragraph.

```
let public_key = get_public_key();
```

- `encrypt_data` : Allows to encrypt data by passing the public key and the data. The output is a byte array that you can convert to string. Please, convert data to `bytes` so that this works. Also, the result will be a `bytes` vector(don't forget to convert it to any format you want for storage and transfer).
E.g 
```
let data = b"Hello world";
let private_key = encrypt_data();
encrypt_data(private_key, data);
```
Do you suggest that I take reference to private_key and to data because of variable management in rust? [Let me know](https://twitter.com/elielmathe).

- `decrypt_data` : Yes, this being symetric encryption/decryption, you need to enter the private key to be able to decrypt data. So, for decryption, simply enter the `private key` and the `data` to decrypt(many times called cipher text :) ). Keep in mind, the return is a bytes vector so, don't forget to convert back to the format you use.


## What is RSA ?

RSA is a public-key cryptosystem that is widely used for secure data transmission. It is also one of the oldest. The acronym "RSA" comes from the surnames of Ron Rivest, Adi Shamir and Leonard Adleman, who publicly described the algorithm in 1977.  

## Where can you use it ? 

From Google search :) : RSA encryption, in full Rivest-Shamir-Adleman encryption, type of public-key cryptography widely used for data encryption of e-mail and other digital transactions over the Internet.   

## Read more

Here is my [Twitter](https://twitter.com/elielmathe)  
Here is my [Blog](https://eliel.nfinic.com)  
Here is my [email](eliel@nfinic.com)  

## Contribute 

Yes, please.

<i>Best regards.</i>

El.

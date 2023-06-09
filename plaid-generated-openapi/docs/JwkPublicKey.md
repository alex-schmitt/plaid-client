# JwkPublicKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alg** | **String** | The alg member identifies the cryptographic algorithm family used with the key. | 
**crv** | **String** | The crv member identifies the cryptographic curve used with the key. | 
**kid** | **String** | The kid (Key ID) member can be used to match a specific key. This can be used, for instance, to choose among a set of keys within the JWK during key rollover. | 
**kty** | **String** | The kty (key type) parameter identifies the cryptographic algorithm family used with the key, such as RSA or EC. | 
**r#use** | **String** | The use (public key use) parameter identifies the intended use of the public key. | 
**x** | **String** | The x member contains the x coordinate for the elliptic curve point, provided as a base64url-encoded string of the coordinate's big endian representation. | 
**y** | **String** | The y member contains the y coordinate for the elliptic curve point, provided as a base64url-encoded string of the coordinate's big endian representation. | 
**created_at** | **i32** | The timestamp when the key was created, in Unix time. | 
**expired_at** | Option<**i32**> | The timestamp when the key expired, in Unix time. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



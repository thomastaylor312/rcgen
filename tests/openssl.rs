extern crate openssl;
extern crate rcgen;

use rcgen::{Certificate, CertificateParams, DnType};
use openssl::pkey::PKey;
use openssl::x509::{X509, X509Req, X509StoreContext};
use openssl::x509::store::{X509StoreBuilder, X509Store};
use openssl::stack::Stack;

#[test]
fn test_openssl() {
	let mut params = CertificateParams::new(vec![
		"crabs.crabs".to_string(), "localhost".to_string(),
	]);
	params.distinguished_name.push(DnType::OrganizationName, "Crab widgits SE");
	params.distinguished_name.push(DnType::CommonName, "Master CA");
	let cert = Certificate::from_params(params);

	println!("{}", cert.serialize_pem());

	// Now verify the certificate.
	let x509 = X509::from_pem(&cert.serialize_pem().as_bytes()).unwrap();
	let mut builder = X509StoreBuilder::new().unwrap();
	builder.add_cert(x509.clone()).unwrap();

	let store :X509Store = builder.build();
	let mut ctx = X509StoreContext::new().unwrap();
	let mut stack = Stack::new().unwrap();
	stack.push(x509.clone()).unwrap();
	ctx.init(&store, &x509, &stack.as_ref(), |ctx| {
		ctx.verify_cert().unwrap();
		Ok(())
	}).unwrap();
}

#[test]
fn test_request() {
	let params = CertificateParams::new(vec!["crabs.crabs".to_string(), "localhost".to_string()]);
	let cert = Certificate::from_params(params);

	let key = cert.serialize_private_key_der();
	let pkey = PKey::private_key_from_der(&key).unwrap();

	let req = X509Req::from_pem(&cert.serialize_request_pem().as_bytes()).unwrap();
	req.verify(&pkey).unwrap();
}

/*
Generated by: openssl genpkey -algorithm RSA \
 -pkeyopt rsa_keygen_bits:2048 \
 -pkeyopt rsa_keygen_pubexp:65537 | \
 openssl pkcs8 -topk8 -nocrypt -outform pem
*/
#[cfg(feature = "pem")]
const RSA_TEST_KEY_PAIR_PEM :&str = r#"
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDYjmgyV3/LSizJ
XrYrATZrrPr2Edo8yiOgBLFmi4sgeGdQ5n6nhjTGfBEIP2Ia6z+hbiGOMncabEBc
zkdME+JFYVCSkS7r4ivMOzp2egxLgcPKcerBoXI8DUbHhIR9z89lHiPHDJv3+d0A
c1b9bz9b8OAeZWiQmFvmjpbc2DfhQ2OFx2MwFZCYF196rrXOc6/SR2esZVRrkW22
RBKFTgz6GIA5A/5VWKIISSqEB1gOcMz2iq5987I28+Ez4rcLZ2lB7cZ7TbNxkAwt
0fPL+EuyP7XOzbIj4/kSAlU5xfwNERa3BEuOFro4i5EmSDj+lR5xdRpFnx0j5zOo
zUL2lHG9AgMBAAECggEARpV8DtSIOcmOeYAeXjwB8eyqy+Obv26fV/vPmr3m9glo
m2zVYWMT9pHft1F5d46v6b0MwN1gBsO74sP1Zy2f9b83VN5vbcEFR4cSkiVLtpyw
JV8mBkDKDBrDtCpUSPGgBrRhMvLAL35Ic2oks2w8OYp0clPZVi/i3G4jbA4pgIkt
yB6k79Uhzz2nfZ0VpPORGNsBOl5UK1LkmIhTJ6S0LsLj7XSet9YHR0k0F0/NOSzz
+jMUzfjOPm8M+b3wk9yAQP7qT9Iy3MHbGAad4gNXGu1LqeDRkfmM5pnoG0ASP3+B
IvX2l0ZLeCtg+GRLlGvUVI1HSQHCsuiC6/g2bq7JAQKBgQD3/Eb58VjpdwJYPrg/
srfnC9sKSf5C0Q8YSmkfvOmeD6Vqe0EXRuMyhwTkkVdz04yPiB2j0fXdeB9h16Ic
9HWb/UNGWNpV7Ul1MSHbeu32Xor+5IkqCGgSoMznlt9QPR4PxfIOgO8cVL1HgNAZ
JnBDzhTG0FfY75hqpCDmFGAZwQKBgQDfjhk5aM0yGLYgZfw/K9BrwjctQBWdrps2
4TtkG7Kuj0hsimCdrqJQ5JN8aUM41zDUr3Px1uN5gUAZ3dE9DoGsgj15ZwgVkAMM
E54bfzOqkbh+mRpptIxL4HmHB45vgvz0YljeRoOEQvPF/OSGLti7VIkD4898PFKl
cU+P9m5+/QKBgDi8XTi+AQuZEM5Duz/Hkc+opLqb5zI+RmfWTmrWe9SP29aa0G+U
5lIfFf19SzbSxavpBm7+kHPVEcj+3rYlL+s6bHPhzEIwgcfwL8DZRSxCwSZD/yXA
up7Yb0jk+b6P3RravOCYmxwuPwfm7rVyV+kLczFxZUfauVJcrrI1Iy+BAoGBAJjG
MEDGeSxaLOS5LYgyNg3ePPzkhaEruRDpHUBNmW+npZPfgSVhObXUb2IfQXwvu0Qt
3yuPcgcQKDFFIH/8UOwGWWKE4cZyk1KGeY9K/5D6Yr3JfX5tj08vSX3Y0SMtvhZ4
u0izoZ8abiOIrtdwXlau76/D2ICLbON5Kykz/NE1AoGAId2+pO9p8jBt9l+5jZo7
Rw/mb5icMaG2hqAzs37gUPbpSwQFOmGhQmNM+WvYEvUUuiTxI3AOeEK8Mj+BVB4+
uE3X/fWK/JR9iOzH9OM31Nua8/EJzr7BmUpXeRr4dAtVimeQ+5HY6IgRsFGPKKwv
YPTHy8SWRA2sMII3ArhHJ8A=
-----END PRIVATE KEY-----
"#;

#[test]
fn test_request_rsa_given() {
	let mut params = CertificateParams::new(vec!["crabs.crabs".to_string(), "localhost".to_string()]);
	params.alg = &rcgen::PKCS_RSA_SHA256;

	params.key_pair = Some(rcgen::KeyPair::from_pem(RSA_TEST_KEY_PAIR_PEM).into());
	let cert = Certificate::from_params(params);

	let key = cert.serialize_private_key_der();
	let pkey = PKey::private_key_from_der(&key).unwrap();

	let req = X509Req::from_pem(&cert.serialize_request_pem().as_bytes()).unwrap();
	req.verify(&pkey).unwrap();
}

#[test]
#[cfg(feature = "pem")]
fn test_openssl_rsa_given() {
	let mut params = CertificateParams::new(vec![
		"crabs.crabs".to_string(), "localhost".to_string(),
	]);
	params.alg = &rcgen::PKCS_RSA_SHA256;

	params.key_pair = Some(rcgen::KeyPair::from_pem(RSA_TEST_KEY_PAIR_PEM).into());

	params.distinguished_name.push(DnType::OrganizationName, "Crab widgits SE");
	params.distinguished_name.push(DnType::CommonName, "Master CA");
	let cert = Certificate::from_params(params);

	println!("{}", cert.serialize_pem());

	// Now verify the certificate.
	let x509 = X509::from_pem(&cert.serialize_pem().as_bytes()).unwrap();
	let mut builder = X509StoreBuilder::new().unwrap();
	builder.add_cert(x509.clone()).unwrap();

	let store :X509Store = builder.build();
	let mut ctx = X509StoreContext::new().unwrap();
	let mut stack = Stack::new().unwrap();
	stack.push(x509.clone()).unwrap();
	ctx.init(&store, &x509, &stack.as_ref(), |ctx| {
		ctx.verify_cert().unwrap();
		Ok(())
	}).unwrap();
}

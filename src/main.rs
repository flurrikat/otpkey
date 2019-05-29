mod keygen;

fn main() {
    let secret = keygen::OTPSecret::new(32).expect("otp secret gen failed");

    let uridata = keygen::URIData {
        label: "snow",
        issuer: "Scarlet",
    };

    println!("uri: {}", secret.into_uri(&uridata));

    println!("img: {}", secret.into_qrcode_uri(&uridata).unwrap());
}

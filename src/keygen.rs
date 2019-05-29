use ring::rand::{SecureRandom, SystemRandom};
use failure::{bail, ensure, Fallible};
use base32;
use base64;
use qrcode::QrCode;
use image::{png, ColorType, Luma};

pub struct OTPSecret {
    key: Vec<u8>,
}

pub struct URIData<'a> {
    pub label: &'a str,
    pub issuer: &'a str,
}

impl OTPSecret {
    pub fn new() -> Fallible<OTPSecret> {
        let rand = SystemRandom::new();
        let mut new_secret = OTPSecret {
            key: Vec::new(),
        };
        // XXX: should this be defined by the user?
        new_secret.key.resize(160, 0);
        ensure!(rand.fill(new_secret.key.as_mut_slice()).is_ok(), "couldn't generate enough random numbers");
        Ok(new_secret)
    }

    pub fn into_base32(&self) -> String {
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, self.key.as_slice())
    }

    pub fn into_uri(&self, uri_data: &URIData) -> String {
        format!("otpauth://totp/{}?secret={}&issuer={}", uri_data.label, self.into_base32(), uri_data.issuer)
    }

    pub fn into_qrcode_uri(&self, uri_data: &URIData) -> Fallible<String> {
        let code = match QrCode::new(self.into_uri(uri_data).as_bytes()) {
            Ok(c) => c,
            Err(err) => bail!("Error while generating QR Code"),
        };

        let qrimage = code.render::<Luma<u8>>()
            .build();

        let mut pngdata: Vec<u8> = Vec::new();

        png::PNGEncoder::new(pngdata).encode(qrimage.into_raw(), qrimage.width(), qrimage.height(), ColorType::Gray(0));

        let pngdata = base64::encode(&pngdata);

        Ok(format!("data:image/png;base64,{}", pngdata))
    }
}

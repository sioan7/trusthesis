use cesrox::group::Group;
use cesrox::payload::Payload;
use cesrox::primitives::codes::basic::Basic;
use cesrox::primitives::codes::self_signing::SelfSigning;
use cesrox::ParsedData;
use ed25519_dalek::SigningKey;
use tracing::info;

fn main() {
    let _guard = util::log::set_up_logging("cesr.log");

    let mut csprng = rand::rngs::OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    use ed25519_dalek::{Signature, Signer};

    let message = br#"{"name":"John","surname":"Doe"}"#;
    let ed_signature: Signature = signing_key.sign(message);

    let public_key = (
        Basic::Ed25519Nontrans,
        signing_key.verifying_key().to_bytes().to_vec(),
    );
    info!(
        "{}",
        String::from_utf8_lossy(&signing_key.verifying_key().to_bytes())
    );
    let signature = (SelfSigning::Ed25519Sha512, ed_signature.to_bytes().to_vec());
    info!("{}", String::from_utf8_lossy(&ed_signature.to_bytes()));

    let attachment = Group::NontransReceiptCouples(vec![(public_key.clone(), signature.clone())]);
    let data = ParsedData {
        payload: Payload::JSON(message.to_vec()),
        attachments: vec![attachment],
    };
    let cesr_stream = data.to_cesr().unwrap();
    info!("{}", String::from_utf8_lossy(&cesr_stream));
    // assert_eq!(&cesr_stream, br#"{"name":"John","surname":"Doe"}-CABBNdamAGCsQq31Uv-08lkBzoO4XLz2qYjJa8CGmj3B1Ea0BDkGKpYn5i5fhRrE57RGGonHMlwmfZBmsIAex6rPXuZqScZY3NPdyP60fDHmGjLy7kQj04vZsFBAyid1XOJxBgG"#);
    //
    // let (_rest, parsed_data) = parse(&cesr_stream).unwrap();
    // assert_eq!(
    //     parsed_data.payload,
    //     Payload::JSON(br#"{"name":"John","surname":"Doe"}"#.to_vec())
    // );
    // assert_eq!(
    //     parsed_data.attachments,
    //     vec![Group::NontransReceiptCouples(vec![(public_key, signature)])]
    // );
}

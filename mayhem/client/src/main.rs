use lettre::{Message, SmtpTransport, Transport};

fn main() -> Result<(), ()> {
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!"))
        .unwrap();

    let sender = SmtpTransport::builder_dangerous("127.0.0.1").build();

    let _result = dbg!(sender.send(&email));

    Ok(())
}

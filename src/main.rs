use lettre::{
    Message,
    stub::StubTransport
};

fn main() {
    println!("Hello, world!");

    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new async year")
        .body(String::from("Be happy with async!"))
        .unwrap();
}

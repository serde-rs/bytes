use serde_bytes::{ByteArray, ByteBuf, Bytes};
use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

#[cfg(feature = "heapless")]
use serde_bytes::HeaplessByteBuf;

#[test]
fn test_bytes() {
    let empty = Bytes::new(&[]);
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_ser_tokens(&empty, &[Token::Bytes(b"")]);
    assert_ser_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);

    let buf = vec![65, 66, 67];
    let bytes = Bytes::new(&buf);
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
}

#[test]
fn test_byte_buf() {
    let empty = ByteBuf::new();
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_tokens(&empty, &[Token::Bytes(b"")]);
    assert_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);
    assert_de_tokens(&empty, &[Token::Str("")]);
    assert_de_tokens(&empty, &[Token::String("")]);
    assert_de_tokens(&empty, &[Token::Seq { len: None }, Token::SeqEnd]);
    assert_de_tokens(&empty, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);

    let buf = ByteBuf::from(vec![65, 66, 67]);
    assert_tokens(&buf, &[Token::BorrowedBytes(b"ABC")]);
    assert_tokens(&buf, &[Token::Bytes(b"ABC")]);
    assert_tokens(&buf, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&buf, &[Token::BorrowedStr("ABC")]);
    assert_de_tokens(&buf, &[Token::Str("ABC")]);
    assert_de_tokens(&buf, &[Token::String("ABC")]);
    assert_de_tokens(
        &buf,
        &[
            Token::Seq { len: None },
            Token::U8(65),
            Token::U8(66),
            Token::U8(67),
            Token::SeqEnd,
        ],
    );
    assert_de_tokens(
        &buf,
        &[
            Token::Seq { len: Some(3) },
            Token::U8(65),
            Token::U8(66),
            Token::U8(67),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_bytearray() {
    let empty = ByteArray::new([]);
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_ser_tokens(&empty, &[Token::Bytes(b"")]);
    assert_ser_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);

    let buf = [65, 66, 67];
    let bytes = ByteArray::new(buf);
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
}

#[test]
#[cfg(feature = "heapless")]
fn test_heapless_byte_buf() {
    let empty = HeaplessByteBuf::<3>::new();
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_tokens(&empty, &[Token::Bytes(b"")]);
    assert_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::Seq { len: None }, Token::SeqEnd]);
    assert_de_tokens(&empty, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);

    let buf = HeaplessByteBuf::<3>::try_from(vec![65, 66, 67])
        .expect("Size of tested `HeaplessByteBuf`to be >= size of test data");
    assert_tokens(&buf, &[Token::BorrowedBytes(b"ABC")]);
    assert_tokens(&buf, &[Token::Bytes(b"ABC")]);
    assert_tokens(&buf, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(
        &buf,
        &[
            Token::Seq { len: None },
            Token::U8(65),
            Token::U8(66),
            Token::U8(67),
            Token::SeqEnd,
        ],
    );
    assert_de_tokens(
        &buf,
        &[
            Token::Seq { len: Some(3) },
            Token::U8(65),
            Token::U8(66),
            Token::U8(67),
            Token::SeqEnd,
        ],
    );
}

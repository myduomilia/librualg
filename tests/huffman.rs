use librualg::huffman;
use librualg::huffman::Huffman;

#[test]
fn test(){

    assert_eq!(Huffman::encode(""), None);

    let (bytes, table) = Huffman::encode("abracadabra").unwrap();
    let msg = Huffman::decode(&bytes, &table);
    assert_eq!(msg, "abracadabra");

    let (bytes, table) = Huffman::encode("aaa").unwrap();
    let msg = Huffman::decode(&bytes, &table);
    assert_eq!(msg, "aaa");

    let (bytes, table) = Huffman::encode("a").unwrap();
    let msg = Huffman::decode(&bytes, &table);
    assert_eq!(msg, "a");

    let (bytes, table) = Huffman::encode("aaaaaaaa").unwrap();
    let msg = Huffman::decode(&bytes, &table);
    assert_eq!(msg, "aaaaaaaa");
    assert_eq!(bytes.len(), 2);
}
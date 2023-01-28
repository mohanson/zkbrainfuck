use zkbrainfuck::code;

#[test]
fn test_code_neptune() {
    let output = code::compile("++>,<[>+.<-]".as_bytes().to_vec());
    let expect: Vec<u16> = vec![
        '+' as u16, '+' as u16, '>' as u16, ',' as u16, '<' as u16, '[' as u16, 14, '>' as u16, '+' as u16, '.' as u16,
        '<' as u16, '-' as u16, ']' as u16, 7,
    ];
    assert_eq!(output, expect);
}

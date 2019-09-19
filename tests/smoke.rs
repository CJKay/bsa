use bsa;

#[test]
fn hello_world_bsa() {
    let data = include_bytes!("assets/hello_world.bsa");

    let bsa = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(bsa, bsa::Archive);
}

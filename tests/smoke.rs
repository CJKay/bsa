#[test]
fn hello_world_bsa() {
    let data = include_bytes!("assets/hello_world.bsa");

    let bsa = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(bsa, bsa::Archive);
}

#[test]
fn hello_world_ba2() {
    let data = include_bytes!("assets/hello_world.ba2");

    let ba2 = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(ba2, bsa::Archive);
}

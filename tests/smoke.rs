#[test]
fn hello_world_tes3() {
    let data = include_bytes!("assets/hello-world-tes3.bsa");

    let bsa = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(bsa, bsa::Archive::Morrowind);
}

#[test]
fn hello_world_tes4() {
    let data = include_bytes!("assets/hello-world-tes4.bsa");

    let bsa = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(bsa, bsa::Archive::Oblivion);
}

#[test]
fn hello_world_fo3() {
    let data = include_bytes!("assets/hello-world-fo3.bsa");

    let bsa = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(bsa, bsa::Archive::Fallout3);
}

#[test]
fn hello_world_tes5se() {
    let data = include_bytes!("assets/hello-world-tes5se.bsa");

    let bsa = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(bsa, bsa::Archive::SkyrimSe);
}

#[test]
fn hello_world_fo4() {
    let data = include_bytes!("assets/hello-world-fo4.ba2");

    let bsa = bsa::Archive::new(&data[..]).unwrap();

    assert_eq!(bsa, bsa::Archive::Fallout4);
}

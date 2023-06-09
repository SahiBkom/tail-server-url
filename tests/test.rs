use tail_server_url::*;

#[test]
fn it_works() {
    assert_eq!(
        "https://tile.openstreetmap.org/17/67400/43241.png".to_string(),
        TailServerUrl::new_openstreetmap(17).deg(52.090752, 5.121630)
    );
    assert_eq!((526, 337), TailServerUrl::deg2num(52.090752, 5.121630, 10));

    println!("{}", TailServerUrl::new_openstreetmap(8).deg(50.75, 3.2));

    println!("{:?}", TailServerUrl::deg2num(50.75, 3.2, 8));
    println!("{:?}", TailServerUrl::deg2num(53.7, 7.22, 8));

    let iter = TailServerUrl::new_openstreetmap(8).deg_box(50.75, 3.2, 53.7, 7.22);

    assert_eq!((4, 4), iter.size());
    println!("{:?}", iter);
    for i in iter {
        println!("{}", i);
    }
}

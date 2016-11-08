use safe_transmute::guarded_transmute;


#[test]
#[should_panic]
fn too_short() {
    unsafe {
        guarded_transmute::<u32>(&[]);
    }
}

#[test]
fn just_enough() {
    unsafe {
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01]), 0x01000000);
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00]), 0x01000000);
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00]), 0x01000000);
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]), 0x01000000);
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]), 0x01000000);
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00]), 0x01000000);
    }
}

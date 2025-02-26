fn main() {
    type LowAlign = u64;
    nexon::Channel::<LowAlign, 8>::print_layout();

    println!();

    #[repr(align(256))]
    #[allow(unused)]
    struct HighAlign(u64);
    nexon::Channel::<HighAlign, 8>::print_layout();
}

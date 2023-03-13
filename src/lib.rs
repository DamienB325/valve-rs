pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn bsp_to_json_str(bsp: &vbsp::Bsp) {
    print_type_of(&bsp);
}

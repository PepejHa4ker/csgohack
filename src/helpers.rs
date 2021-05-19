named!(

    pub parse_string<&str>,
    map_res!(take_until_and_consume!("\0"), ::std::str::from_utf8)
);
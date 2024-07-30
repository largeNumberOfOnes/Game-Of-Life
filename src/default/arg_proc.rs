fn get_parm(s: &str) -> Option<u32> {
    if s.len() < 3 { return None; }
    match s[2..].parse() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn read_and_decode(
    prelude: &str,
    s: &str,
    var: &mut u32,
) {

    match get_parm(s) {
        Option::Some(v) => { *var = v; },
        Option::None => {
            println!(
                "Parameter '{}' is poor and cannot be perceived in its entirety!\n\
                The argument has the following form: '{}<NUMBER>'!",
                s,
                prelude
            );
        },
    }
    

}
pub fn read_command_line_args(
    width : &mut u32,
    height: &mut u32,
    cellx : &mut u32,
    celly : &mut u32,
) {
    let args = std::env::args().skip(1);

    for q in args {
        if q.len() < 3 { continue; }
        match &q[..2] {
            e @ "-w" => { read_and_decode(e, &q, width ); continue;}
            e @ "-h" => { read_and_decode(e, &q, height); continue;}
            e @ "-x" => { read_and_decode(e, &q, cellx ); continue;}
            e @ "-y" => { read_and_decode(e, &q, celly ); continue;}
            _ => {}
        }
        println!("'{}' is not a parametr!", q);
        // println!("{}", q);
    }
}

mod test {
    
    #[test]
    fn test_get_parm() {
        use super::get_parm;
        assert_eq!(get_parm("ww234"), Some(234));
        assert_eq!(get_parm("wwewbifc"), None);
        assert_eq!(get_parm("ww-234"), None);
        assert_eq!(get_parm("ww"), None);
        assert_eq!(get_parm("w"), None);
    }

}
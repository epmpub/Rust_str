


fn get_string() -> std::io::Result<String> {
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}


fn foo() -> std::io::Result<String> {
    let a = std::env::current_dir()?;
    let ret = a.as_path().display().to_string();
    Ok(ret)
}



fn main() {

    let p = foo();
    match p {
        Ok(v) => {
            let flags = "hello world";
            let res = v + " THIS IS A TEST: "+ &flags;
            println!("{}",res);
        },
        Err(e) => {
            println!("{}",e)
        }
    }
    match get_string() {
        Ok(v) => println!("{}",v),
        Err(e) => println!("{}",e),
    }

    let path = "http://google.com.hk";

    match open::that(path) {
        Ok(()) => println!("Opened '{}' successfully.", path),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }

}
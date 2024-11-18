pub fn documentation() -> () {
    println!("Usage:");
    println!("  vault [option] [...params]");
    println!("");
    println!("Options");
    println!("  encrypt           Encrypt given (-file) and generate key file (*.bzk), params = [ -file ]");
    println!("  decrypt           Decrypt given encrypted file (*.bzf) with the given key (*.bzk), params = [ -file , -key , -output ]");
    println!("  help              Print this help documentation");
    println!("");
    println!("Params:");
    println!("  -file             Target file to be process");
    println!("  -key              Key file that contain encryption data");
    println!("  -output           Output of decryption result, will be printed if not provided");
}

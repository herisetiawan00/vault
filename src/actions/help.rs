pub fn documentation() -> () {
    println!("Usage:");
    println!("  vault [option] [...params]");
    println!("");
    println!("Options");
    println!("  encrypt       e    Encrypt given (-file) and generate key file (*.bzk), params = [ --file , --output-path, --passkey, --key-length ]");
    println!("  decrypt       d    Decrypt given encrypted file (*.bzf) with the given key (*.bzk), params = [ --file , --key , --output ]");
    println!("  help          h    Print this help documentation");
    println!("");
    println!("Params:");
    println!("  --file        -f   Target file to be process");
    println!("  --key         -k   Key file that contain encryption data");
    println!("  --key-length  -l   Length of key, default is 1KB [..B, ..KB]");
    println!("  --passkey     -p   Passkey for encrypted data");
    println!("  --output      -o   Output of decryption result, will be printed if not provided");
    println!("  --output-path -op  Output path of current encryption process");
}

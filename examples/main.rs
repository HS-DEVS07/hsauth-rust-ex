use hsauth::HSAuth;

fn main() {
    let mut auth = HSAuth::new(
        "YOUR_APP_NAME",
        "YOUR_OWNER_ID",
        "YOUR_SECRET",
        "1.0"
    );

    println!("Initializing HS DEVS AUTH...");
    if !auth.init() {
        eprintln!("Auth initialization error: {}", auth.response.message);
        return;
    }

    println!("[+] Initialized successfully! Session: {}", auth.session_id);

    // Example License Login
    let test_key = "HSAUTH-TEST-KEY-0001";
    if auth.license(test_key) {
        println!("[+] License Verified! Expires: {}", auth.user_data.expires_at);
    } else {
        println!("[-] License check: {}", auth.response.message);
    }
}

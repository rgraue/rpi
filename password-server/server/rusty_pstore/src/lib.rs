use pyo3::prelude::pymodule;

mod crypto;
mod utils;

#[pymodule]
mod rusty_pstore {

    use base64::Engine;
    use base64::engine::general_purpose;
    use colored::Colorize;
    use pyo3::prelude::pyfunction;
    use rand::RngCore;
    use rand::rng;
    use rayon::prelude::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    use crate::crypto::{decrypt, encrypt, gen_key, encode, decode};
    use crate::utils::{PassInfo, read_pass_file, write_pass_file};

    // create pass file
    #[pyfunction]
    fn init_pass_file(client_id: &str, password: &str) -> Option<bool> {
        println!(
            "{}",
            "Pass file not found.\nCreating new one with entered password".red()
        );
        let mut iv = [0u8; 12];
        let mut salt = [0u8; 16];

        rng().fill_bytes(&mut iv);
        rng().fill_bytes(&mut salt);

        let iv_b64 = general_purpose::STANDARD.encode(&iv);
        let salt_b64 = general_purpose::STANDARD.encode(&salt);

        let key = gen_key(&String::from(password), &salt_b64.as_bytes());

        let encrypt_check = match encrypt(&String::from("__check"), &iv_b64, &key) {
            Ok(r) => r,
            Err(_) => {
                println!("{}", "Error creating pass file".red());
                return None;
            }
        };

        let mut map: HashMap<String, PassInfo> = HashMap::new();
        map.insert(
            encrypt_check,
            PassInfo {
                username: iv_b64,
                password: salt_b64,
                url: None,
            },
        );

        return match write_pass_file(&String::from(client_id), &map) {
            Ok(_) => {
                println!("{}", "Successfully created pass file".green());
                return Some(true);
            }
            Err(_) => None,
        };
    }

    // Login. return [key, iv]
    #[pyfunction]
    fn login(client_id: &str, password: &str) -> Option<Vec<String>> {
        let found = Arc::new(AtomicBool::new(false));
        let found_clone = found.clone();

        let atomic_check_key = Arc::new(Mutex::new(String::new()));
        let atomic_check_key_clone = atomic_check_key.clone();

        let data = match read_pass_file(&String::from(client_id)) {
            Ok(result) => result.data,
            Err(_) => {
                println!("{}", "Passfile not found".red());
                return None;
            }
        };

        let keys: Vec<[u8; 32]> = data
            .par_iter()
            .map(|(name, value)| {
                let key = login_helper(
                    &String::from(password),
                    &name,
                    &value,
                    &atomic_check_key_clone,
                    &found_clone,
                );

                return key;
            })
            .collect();

        let check_key = String::from(atomic_check_key_clone.lock().unwrap().as_str());
        let filtered: Vec<[u8; 32]> = keys
            .into_iter()
            .filter(|b| !b.iter().all(|&byte| byte == 0)).collect();

        // auth check
        if check_key.len() == 0 || filtered.len() == 0 {
            println!("{}", "Password Incorrect".red());
            return None;
        }

        let check_pass_info = match data.get(&check_key) {
            Some(s) => s,
            None => {
                println!("{}", "can't find check info".red());
                return None;
            },
        };

        let iv = check_pass_info.username.to_string();
        let key_bytes = filtered[0];

        let key = encode(&key_bytes);

        let mut response: Vec<String> = Vec::new();
        response.push(key);
        response.push(iv);

        return Some(response);
    }

    // get a list of all decrypted password names
    #[pyfunction]
    fn get_names(client_id: &str, key: &str, iv: &str) -> Vec<String> {
        let key_bytes = match decode(&key) {
            Ok(b) => b,
            Err(_) => {
                println!("{}", "Error decoding key".red(), );
                return Vec::new();
            }
        };

        let pf = match read_pass_file(&String::from(client_id)) {
            Ok(result) => result.data,
            Err(_) => {
                println!("{}", "Error reading passfile".red());
                return Vec::new()
            },
        };

        return get_names_helper(&pf, &key_bytes, &String::from(iv));
    }

    // get the details for a password entry
    // [username, password, url]
    #[pyfunction]
    fn get_pass_info(client_id: &str, key: &str, iv: &str, looking_for: &str) -> Vec<String> {
        let key_bytes = match decode(&key) {
            Ok(b) => b,
            Err(_) => {
                println!("{}", "Error decoding key".red(), );
                return Vec::new();
            }
        };

        let pf = match read_pass_file(&String::from(client_id)) {
            Ok(result) => result.data,
            Err(_) => return Vec::new(),
        };

        let mut pass_details: Vec<String> = Vec::new();

        match get_pass_info_helper(
            &pf,
            &String::from(looking_for),
            &key_bytes,
            &String::from(iv),
        ) {
            Some(pass_info) => {
                pass_details.push(pass_info.username);
                pass_details.push(pass_info.password);
                pass_details.push(match pass_info.url {
                    Some(s) => s,
                    None => String::new(),
                });
            }
            None => return pass_details,
        };

        return pass_details;
    }

    // add a password info entry
    #[pyfunction]
    fn add_pass(
        client_id: &str,
        key: &str,
        iv: &str,
        name: &str,
        username: &str,
        password: &str,
        url: &str,
    ) -> bool {
        let key_bytes = match decode(&key) {
            Ok(b) => b,
            Err(_) => {
                println!("{}", "Error decoding key".red(), );
                return false;
            }
        };

        let mut pf = match read_pass_file(&String::from(client_id)) {
            Ok(result) => result.data,
            Err(_) => return false,
        };

        let info = PassInfo {
            username: String::from(username),
            password: String::from(password),
            url: Some(String::from(url)),
        };

        return add_pass_helper(
            &String::from(client_id),
            &mut pf,
            &String::from(name),
            &info,
            &String::from(iv),
            &key_bytes,
        );
    }

    fn get_names_helper(pf: &HashMap<String, PassInfo>, key: &[u8], iv: &String) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for (name, _) in pf.iter() {
            match decrypt(&name, &iv, &key) {
                Ok(x) => {
                    if x != "__check" {
                        names.push(x);
                    }
                }
                Err(_) => continue,
            }
        }

        return names;
    }

    fn login_helper(
        password: &String,
        name: &String,
        value: &PassInfo,
        check_key: &Arc<Mutex<String>>,
        found: &Arc<AtomicBool>,
    ) -> [u8; 32] {
        if found.load(Ordering::Relaxed) {
            return [0; 32];
        }
        let key = gen_key(&password, &value.password.as_bytes());
        match decrypt(name, &value.username, &key) {
            Ok(maybe_check) => {
                if maybe_check == "__check" {
                    found.store(true, Ordering::Relaxed);
                    let mut guard = check_key.lock().unwrap();
                    guard.push_str(name);
                    return key;
                }

                return [0; 32];
            }
            Err(_) => return [0; 32],
        }
    }

    fn get_pass_info_helper(
        data: &HashMap<String, PassInfo>,
        looking_for: &String,
        key: &[u8],
        iv: &String,
    ) -> Option<PassInfo> {
        if looking_for.is_empty() {
            println!("{}", "Name argument required".red());
            return None;
        }

        let username: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        let username_clone = username.clone();

        let password: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        let password_clone = password.clone();

        let url: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        let url_clone = url.clone();

        data.par_iter().for_each(|(name, value)| {
            return match decrypt(&name, &iv, &key) {
                Ok(result) => {
                    if *looking_for == result {
                        let username_guard = match decrypt(&value.username, &iv, &key) {
                            Ok(r) => r,
                            Err(_) => String::from("Unable to decrypt username"),
                        };
                        let password_guard = match decrypt(&value.password, &iv, &key) {
                            Ok(r) => r,
                            Err(_) => String::from("Unable to decrypt password"),
                        };
                        let url_guard = match &value.url {
                            Some(r) => match decrypt(r, &iv, &key) {
                                Ok(u) => u,
                                Err(_) => String::from("Unable to decrypt url"),
                            },
                            None => String::from("no url"),
                        };

                        username_clone.lock().unwrap().push_str(&username_guard);
                        password_clone.lock().unwrap().push_str(&password_guard);
                        url_clone.lock().unwrap().push_str(&url_guard);
                    }
                }
                Err(_) => (),
            };
        });

        // if all empty assume somethig went wrong.
        if username_clone.lock().unwrap().is_empty()
            && password_clone.lock().unwrap().is_empty()
            && url_clone.lock().unwrap().is_empty()
        {
            return None;
        }

        return Some(PassInfo {
            username: username_clone.lock().unwrap().to_string(),
            password: password_clone.lock().unwrap().to_string(),
            url: Some(url_clone.lock().unwrap().to_string()),
        });
    }

    fn add_pass_helper(
        client_id: &String,
        data: &mut HashMap<String, PassInfo>,
        name: &String,
        info: &PassInfo,
        iv: &String,
        key: &[u8],
    ) -> bool {
        let encrypted_name = match encrypt(&name, &iv, &key) {
            Ok(s) => s,
            Err(_) => {
                println!("Error! Unable to encrypt password info.");
                return false;
            }
        };

        let encrypted_username = match encrypt(&info.username, &iv, &key) {
            Ok(s) => s,
            Err(_) => {
                println!("Error! Unable to encrypt password info.");
                return false;
            }
        };
        let encrypted_password = match encrypt(&info.password, &iv, &key) {
            Ok(s) => s,
            Err(_) => {
                println!("Error! Unable to encrypt password info.");
                return false;
            }
        };

        let url = match &info.url {
            Some(s) => s,
            None => &String::new(),
        };

        let encrypted_url = match encrypt(&url, &iv, &key) {
            Ok(s) => s,
            Err(_) => {
                println!("Error! Unable to encrypt password info.");
                return false;
            }
        };

        let pass = PassInfo {
            username: encrypted_username,
            password: encrypted_password,
            url: Some(encrypted_url),
        };

        data.insert(encrypted_name, pass);
        return match write_pass_file(client_id, data) {
            Ok(_) => true,
            Err(e) => {
                println!("{}, {}", "Unable to save password update".red(), e);
                return false;
            }
        };
    }
}

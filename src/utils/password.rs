use leptos::prelude::*;
use argon2::Argon2;
use argon2::password_hash::{
    SaltString,
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
    rand_core::OsRng,
};

// I dont really need this function because I only allow logging in so ?
pub fn salter(password: &str) -> Result<String, ServerFnError> {
	let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default()
		.hash_password(password_bytes,&salt) 
	{
		Ok(hashed) => Ok(hashed.to_string()),
		Err(e) => Err(ServerFnError::new(e))
	}
}
// the password and hashed pass would be extracted already in the 
pub fn validate(password: &str, hash: &str) -> Result<(), ServerFnError> {
	let parsed_hash = &PasswordHash::new(hash).unwrap();
	let is_valid = Argon2::default().verify_password(
    	password.as_bytes(), parsed_hash
   	).is_ok();

   	if !is_valid {
   		return Err(ServerFnError::new("Invalid password"))
   	}

   	Ok(())
}

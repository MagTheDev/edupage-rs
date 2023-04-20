use crate::{errors::EdupageError, parsers};




#[derive(Debug)]
pub struct Edupage {

    client: reqwest::Client,
    isLoggedIn: bool,
    username: Option<String>,
    password: Option<String>,
    gsec_hash: Option<String>,
    subdomain: Option<String>,
    _raw_data: Option<String>

}


impl Edupage {

    pub fn new() -> Edupage {
        Edupage {
            client: reqwest::Client::builder().cookie_store(true).build().unwrap(),
            isLoggedIn: false,
            username: None,
            password: None,
            gsec_hash: None,
            subdomain: None,
            _raw_data: None
        }
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    pub fn subdomain(mut self, subdomain: String) -> Self {
        self.subdomain = Some(subdomain);
        self
    }

    pub async fn login<'a>(&mut self, subdomain: String, username: String, password: String) -> Result<String, EdupageError> {


        let request_url: String = format!("https://{subdomain}.edupage.org/login/index.php");
        let csrf_response = self.client.get(request_url).send().await?.text().await?;

        // The unwraps here are safe because the response is always the same
        let csrf_token = csrf_response.split("name=\"csrfauth\"").nth(1).unwrap().split("\"").next().unwrap();

        let body = format!("csrfauth={csrf_token}&username={username}&password={password}");
    
        let request_url = format!("https://{subdomain}.edupage.org/login/edubarLogin.php");
        let login_response = self.client.post(request_url).body(body).header("Content-Type", "application/x-www-form-urlencoded").send().await.unwrap();

        println!("Response: {:#?}", login_response);

        if login_response.url().as_str().contains("bad=1") {
            return Err(EdupageError::LoginError);
        }

        let body = login_response.text().await.unwrap();
        let parsed_body = parsers::login::get_json(body);
        

        Ok("test".to_string())
    }

    


}
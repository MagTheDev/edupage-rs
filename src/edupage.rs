
use std::{fs::OpenOptions, io::Write};




#[derive(Debug)]
pub struct Edupage {

    client: reqwest::Client,
    isLoggedIn: bool,
    username: Option<String>,
    password: Option<String>,
    gsec_hash: Option<String>,
    subdomain: Option<String>,

}
// TODO: Move these to separate module
#[derive(Debug)]
pub enum EdupageError {
    LoginError,
    RequestError(reqwest::Error)
}

impl From<reqwest::Error> for EdupageError {
    fn from(value: reqwest::Error) -> Self {
        return Self::RequestError(value);
    }
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

    fn parse_login_data(self, data: String) -> String {
        
        let json_string = data.split("userhome(").nth(1).unwrap().split(");").nth(0).unwrap().replace("\t", "").replace("\n", "").replace("\r", "");
        
        let mut f = OpenOptions::new().write(true).create(true).open("data.dump.json").unwrap();
        f.write_all(json_string.as_bytes()).unwrap();

        return json_string;
    }

    pub async fn login<'a>(mut self, subdomain: String, username: String, password: String) -> Result<String, EdupageError> {


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

        self.parse_login_data(body);
        

        Ok("test".to_string())
    }

    


}
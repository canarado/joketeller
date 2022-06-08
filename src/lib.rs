#![allow(unused_imports)]

//! # joketeller
//! joketeller is a simple lib to make requests to the [jokeapi](https://jokeapi.dev/) written by [sv443](https://sv443.net).
//! 
//! A simple use case may to get a random joke from the API with no filters or cases
//! ```rust
//! use joketeller::Joker;
//! 
//! let mut joker_client: Joker = Joker::new();
//! 
//! let joke = joker_client.get_joke().unwrap();
//! ```
//! 
//! Get started with the [client](crate::Joker).

use std::string::ToString;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Display;
use ureq;
pub use serde_json;

/// The base URL for the jokeapi
pub const BASE_URL: &'static str = "https://v2.jokeapi.dev/";

/// The main client struct that connects to the jokeapi
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Joker {
    categories: Vec<Category>,
    language: Option<Language>,
    blacklist_flags: Vec<BlacklistFlag>,
    format: Option<ResponseFormat>,
    joke_type: Option<JokeType>,
    search_string: Option<String>,
    id_range: Vec<u32>,
    amount: Option<u32>,
    safe_mode: Option<bool>,
    authorization_key: Option<String>,
}
/// The implementation for the jokeapi client
impl Joker {
    /// Basic Usage:
    /// 
    /// ```rust
    /// let joker_client: Joker = Joker::new();
    /// ```
    pub fn new() -> Joker {
        Joker {
            categories: Vec::new(),
            language: None,
            blacklist_flags: Vec::new(),
            format: None,
            joke_type: None,
            search_string: None,
            id_range: Vec::new(),
            amount: None,
            safe_mode: None,
            authorization_key: None,
        }
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// use joketeller::Category;
    /// 
    /// 
    /// joker_client.add_categories(&mut vec![Category::Any, Category::Programming]);
    /// ```
    /// A list of categories can be found [here](crate::Category)
    pub fn add_categories(&mut self, categories: &mut Vec<Category>) -> &mut Self {
        dedup(categories);
        
        self.categories.append(categories);

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// use joketeller::Language;
    /// 
    /// // english is the default, so only needs set if not english
    /// joker_client.set_language(Language::German);
    /// ```
    /// A list of languages can be found [here](crate::Language)
    pub fn set_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// use joketeller::BlacklistFlag;
    /// 
    /// joker_client.add_blacklist_flags(&mut vec![BlacklistFlag::Political]);
    /// ```
    /// A list of flags can be found [here](crate::BlacklistFlag)
    pub fn add_blacklist_flags(&mut self, flags: &mut Vec<BlacklistFlag>) -> &mut Self {
        dedup(flags);

        self.blacklist_flags.append(flags);

        self
    }

    /// This method is going to most likely going to be deprecated in favor of only support JSON
    /// as it seems to be the strongest serialized format in the Rust atm.
    /// 
    /// Basic Usage:
    /// 
    /// ```rust
    /// use joketeller::ResponseFormat;
    /// 
    /// joker_client.set_format(ResponseFormat::Xml)
    /// ```
    /// Right now, only json is fully supported, but a list of formats can be found [here](crate::ResponseFormat) nonetheless
    pub fn set_format(&mut self, format: ResponseFormat) -> &mut Self {
        self.format = Some(format);

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// use joketeller::JokeType;
    /// 
    /// joker_client.set_joke_type(JokeType::Single);
    /// ```
    /// A list of joke types can be found [here](crate::JokeType)
    pub fn set_joke_type(&mut self, joketype: JokeType) -> &mut Self {
        self.joke_type = Some(joketype);

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// joker_client.set_search_string("string to search for");
    /// ```
    /// The search string is a word or phrase you want to be in the joke, and the API will try to return a result to you if it has a joke with that phrase.
    pub fn set_search_string(&mut self, searchstring: &'static str) -> &mut Self {
        self.search_string = Some(String::from(searchstring));

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// joker_client.set_id_range(2, 5);
    /// ```
    pub fn set_id_range(&mut self, start: u32, end: u32) -> &mut Self {
        self.id_range.push(start);
        self.id_range.push(end);

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// // get 5 jokes
    /// joker_client.set_amount(5);
    /// ```
    pub fn set_amount(&mut self, amount: u32) -> &mut Self {
        self.amount = Some(amount);

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// joker_client.safe_mode(true);
    /// ```
    /// This will turn off some categories such as racist or dark
    pub fn safe_mode(&mut self, s: bool) -> &mut Self {
        self.safe_mode = Some(s);

        self
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// joker_client.set_authorization("auth-key");
    /// ```
    pub fn set_authorization(&mut self, authorization_key: &'static str) -> &mut Self {
        self.authorization_key = Some(String::from(authorization_key));

        self
    }

    /// This is a mostly internal function, and not needed unless you want to implement your own API call
    /// 
    /// Basic Usage:
    /// 
    /// ```rust
    /// let uri_string = joker_client.build_url().unwrap();
    /// ```
    pub fn build_url(&mut self) -> Result<String, &'static str> {
        let mut url: String = BASE_URL.to_string();
        url.push_str("joke/");
        
        let mut url_params: Vec<String> = Vec::new();

        // somewhere temporary to hold param values before we build a string for the url
        let mut temp: Vec<String> = Vec::new();

        // Add default "Any" category is user didnt add any categories
        if self.categories.is_empty() {
            self.categories.push(Category::Any);
        }

        // add Category items to temporary vector to be turned into the appropriate string value
        for item in self.categories.iter() {
            temp.push(item.to_string());
        }
        
        // // push our final Category string to the url_parameters and clear our temporary vector, we will
        // // be doing this often for vectorized items and is only commented here for reference.
        let cat_string = temp.join(",");
        temp.clear();

        // Language
        if self.language.is_some() {
            let mut language = String::from("lang=");
            language.push_str(&self.language.unwrap().to_string());

            url_params.push(language);
        }

        // Blacklist Flags
        if !self.blacklist_flags.is_empty() {

            for item in self.blacklist_flags.iter() {
                temp.push(item.to_string());
            }

            let mut joined = temp.join(",");
            joined.insert_str(0, "blacklistFlags=");

            url_params.push(joined);

            temp.clear();
        }

        // Format
        if self.format.is_some() {
            let mut format = String::from("format=");
            format.push_str(&self.format.unwrap().to_string());

            url_params.push(format);
        }

        // JokeType
        if self.joke_type.is_some() {
            let mut type_ = String::from("type=");
            type_.push_str(&self.joke_type.unwrap().to_string());

            url_params.push(type_)
        }

        // SearchString
        if self.search_string.is_some() {
            let mut search = String::from("contains=");
            search.push_str(&self.search_string.as_ref().unwrap().to_string());

            url_params.push(search);
        }

        // IDRange
        if self.id_range.len() == 2 {
            let mut range = String::from("idRange=");
            range.push_str(&self.id_range[0].to_string());
            range.push_str("-");
            range.push_str(&self.id_range[1].to_string());

            url_params.push(range);
        }

        // Amount
        if self.amount.is_some() {
            let mut amount = String::from("amount=");
            amount.push_str(&self.amount.unwrap().to_string());

            url_params.push(amount);
        }

        // SafeMode
        if self.safe_mode.is_some() {
            url_params.push(String::from("safe-mode"))
        }

        // creating final string to append to the url
        if url_params.is_empty() {
            url.push_str(&cat_string)
        } else {
            let mut final_params = url_params.join("&");

            final_params.insert_str(0, &format!("{}?", cat_string));
            
            url.push_str(&final_params);
        }

        Ok(url)
    }

    /// Basic Usage:
    /// 
    /// ```rust
    /// let joke = joker_client.get_joke().unwrap();
    /// ```
    pub fn get_joke(&mut self) -> Result<serde_json::Value, serde_json::Value> {
        let url_string: String = self.build_url().unwrap();

        let req;

        if self.authorization_key.is_some() {
            req = ureq::get(&url_string).set("Authorization", &self.authorization_key.as_ref().unwrap());
        } else {
            req = ureq::get(&url_string);
        }

        match req.call() {
            Ok(response) => {
                let json: serde_json::Value = response.into_json().unwrap();

                Ok(json)
            },
            Err(ureq::Error::Status(_code, response)) => {
                Err(response.into_json().unwrap())
            },
            Err(_) => {
                Err(serde_json::json!({ "err": "Transport Error"}))
            }
        }
    }

    /// See the [official docs](https://jokeapi.dev/#submit-endpoint) to verify the format for submissions
    ///
    /// Basic Usage:
    /// 
    /// ```rust
    /// let joke = serde_json::json!({
    ///                     "formatVersion": 3,
    ///                     "category": "Misc",
    ///                     "type": "single",
    ///                     "joke": "A horse walks into a bar...",
    ///                     "flags": {
    ///                         "nsfw": true,
    ///                         "religious": false,
    ///                         "political": true,
    ///                         "racist": false,
    ///                         "sexist": false,
    ///                         "explicit": false
    ///                     },
    ///                     "lang": "en"
    /// });
    /// 
    /// match Joker::submit_joke(joke) {
    ///     Ok(response) => {
    ///         ...
    ///     },
    ///     Err(response) => {
    ///         ...
    ///     },
    ///     Err(_) => {
    ///         ...
    ///     }
    /// }
    /// ```
    pub fn submit_joke(json: serde_json::Value) -> Result<serde_json::Value, serde_json::Value> {
        let mut submission_url = BASE_URL.to_string();
        submission_url.push_str("submit");

        match ureq::post(&submission_url).send_json(json) {
            Ok(response) => {
                let json: serde_json::Value = response.into_json().unwrap();

                Ok(json)
            },
            Err(ureq::Error::Status(_code, response)) => {
                Err(response.into_json().unwrap())
            },
            Err(_) => {
                Err(serde_json::json!({ "err": "Transport Error" }))
            }
        }
    }

    /// Usage is the same as the [submit](crate::Joker::submit_joke) function listed above, please refer to it.
    /// 
    /// Only difference between the two is that this function does not write anything to the API and is simply a test for verification purposes, and to avoid rate-limits for submission verification
    pub fn submit_joke_dryrun(json: serde_json::Value) -> Result<serde_json::Value, serde_json::Value> {
        let mut submission_url = BASE_URL.to_string();
        submission_url.push_str("submit?dry-run");

        match ureq::post(&submission_url).send_json(json) {
            Ok(response) => {
                let json: serde_json::Value = response.into_json().unwrap();

                Ok(json)
            },
            Err(ureq::Error::Status(_code, response)) => {
                Err(response.into_json().unwrap())
            },
            Err(_) => {
                Err(serde_json::json!({ "err": "Transport Error" }))
            }
        }
    }
}

// Create Joke API Parameter Types and implement string on them for ease of use in the url builder,
// We do this instead of deriving Display for custom string conversions to conform to JokeAPI
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Category {
    Any,
    Programming,
    Misc,
    Dark,
    Pun,
    Spooky,
    Christmas,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Category::Any => String::from("Any"),
            Category::Programming => String::from("Programming"),
            Category::Misc => String::from("Misc"),
            Category::Dark => String::from("Dark"),
            Category::Pun => String::from("Pun"),
            Category::Spooky => String::from("Spooky"),
            Category::Christmas => String::from("Christmas"),
        }
    }
}

// English is the default, so we dont need to declare it
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum Language {
    Czech,
    German,
    Spanish,
    French,
    Portuguese,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Czech => String::from("cs"),
            Language::German => String::from("de"),
            Language::Spanish => String::from("es"),
            Language::French => String::from("fr"),
            Language::Portuguese => String::from("pt"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BlacklistFlag {
    Nsfw,
    Religious,
    Political,
    Racist,
    Sexist,
    Explicit,
}

impl ToString for BlacklistFlag {
    fn to_string(&self) -> String {
        match self {
            BlacklistFlag::Nsfw => String::from("nsfw"),
            BlacklistFlag::Religious => String::from("religious"),
            BlacklistFlag::Political => String::from("political"),
            BlacklistFlag::Racist => String::from("racist"),
            BlacklistFlag::Sexist => String::from("sexist"),
            BlacklistFlag::Explicit => String::from("explicit"),
        }
    }
}

// json is default
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum ResponseFormat {
    Xml,
    Yaml,
    Txt,
}

impl ToString for ResponseFormat {
    fn to_string(&self) -> String {
        match self {
            ResponseFormat::Xml => String::from("xml"),
            ResponseFormat::Yaml => String::from("yaml"),
            ResponseFormat::Txt => String::from("txt"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum JokeType {
    Single,
    TwoPart,
}

impl ToString for JokeType {
    fn to_string(&self) -> String {
        match self {
            JokeType::Single => String::from("single"),
            JokeType::TwoPart => String::from("twopart"),
        }
    }
}

pub enum StatusCode {
    Ok,
    Created,
    BadRequest,
    Forbidden,
    NotFound,
    PayloadTooLarge,
    URITooLong,
    TooManyRequests,
    InternalServerError,
    OriginUnreachable,
}

fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn builder_working() {
        let mut joker = Joker::new();

        assert_eq!(joker.build_url().unwrap(), "https://v2.jokeapi.dev/joke/Any")
    }

    #[test]
    fn categories() {
        let mut joker = Joker::new();

        joker.add_categories(&mut vec![Category::Christmas, Category::Dark]);

        assert_eq!(joker.build_url().unwrap(), "https://v2.jokeapi.dev/joke/Christmas,Dark")
    }

    #[test]
    fn consistent_building() {
        let mut joker1 = Joker::new();
        let mut joker2 = Joker::new();

        joker1.add_blacklist_flags(&mut vec![BlacklistFlag::Political, BlacklistFlag::Racist]);
        joker1.set_amount(4);

        joker2.add_blacklist_flags(&mut vec![BlacklistFlag::Political, BlacklistFlag::Racist]);
        joker2.set_amount(4);

        assert_eq!(joker1.build_url().unwrap(), joker2.build_url().unwrap())
    }

    #[test]
    fn joke_ranges() {
        let mut joker1 = Joker::new();

        joker1.set_id_range(2, 5);

        assert_eq!(joker1.build_url().unwrap(), "https://v2.jokeapi.dev/joke/Any?idRange=2-5")
    }

    #[test]
    fn getjoke() {
        let mut joker = Joker::new();

        joker
            .add_categories(&mut vec![Category::Programming, Category::Pun])
            .set_amount(3)
            .add_blacklist_flags(&mut vec![BlacklistFlag::Political, BlacklistFlag::Racist]);

        println!("{}", joker.build_url().unwrap());

        let joke = joker.get_joke().unwrap();

        println!("GETJOKE\n\n{:?}\n\nEND GET JOKE\n", joke);
    }

    #[test]
    pub fn auth() {
        let mut joker = Joker::new();

        joker
            .add_categories(&mut vec![Category::Programming, Category::Pun])
            .set_amount(3)
            .set_authorization("testkey")
            .add_blacklist_flags(&mut vec![BlacklistFlag::Political, BlacklistFlag::Racist]);

        let joke = joker.get_joke().unwrap();

        println!("{:?}", joke);
    }

    #[test]
    pub fn submit_dryrun() {
        let submission = Joker::submit_joke_dryrun(serde_json::json!({
                "formatVersion": 3,
                "category": "Misc",
                "type": "single",
                "joke": "A horse walks into a bar...",
                "flags": {
                    "nsfw": true,
                    "religious": false,
                    "political": true,
                    "racist": false,
                    "sexist": false,
                    "explicit": false
                },
                "lang": "en"
        }));

        println!("{:?}", submission);
    }
}
#![allow(unused_imports)]

use std::string::ToString;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Display;

pub const BASE_URL: &'static str = "https://v2.jokeapi.dev/joke/";

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
}

impl Joker {
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
        }
    }

    pub fn add_categories(&mut self, categories: &mut Vec<Category>) -> &mut Self {
        dedup(categories);

        self.categories.append(categories);

        self
    }

    pub fn set_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);

        self
    }

    pub fn add_blacklist_flags(&mut self, flags: &mut Vec<BlacklistFlag>) -> &mut Self {
        dedup(flags);

        self.blacklist_flags.append(flags);

        self
    }

    pub fn set_format(&mut self, format: ResponseFormat) -> &mut Self {
        self.format = Some(format);

        self
    }

    pub fn set_joke_type(&mut self, joketype: JokeType) -> &mut Self {
        self.joke_type = Some(joketype);

        self
    }

    pub fn set_search_string(&mut self, searchstring: &'static str) -> &mut Self {
        self.search_string = Some(String::from(searchstring));

        self
    }

    pub fn set_id_range(&mut self, start: u32, end: u32) -> &mut Self {
        self.id_range.push(start);
        self.id_range.push(end);

        self
    }

    pub fn set_amount(&mut self, amount: u32) -> &mut Self {
        self.amount = Some(amount);

        self
    }

    pub fn safe_mode(&mut self, s: bool) -> &mut Self {
        self.safe_mode = Some(s);

        self
    }

    pub fn build_url(&mut self) -> Result<String, &'static str> {
        let mut url: String = BASE_URL.to_string();
        
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

        // push our final Category string to the url_parameters and clear our temporary vector, we will
        // be doing this often for vectorized items and is only commented here for reference.
        url_params.push(temp.join(","));
        temp.clear();

        // Language
        if self.language.is_some() {
            let mut language = String::from("lang=");
            language.push_str(&self.language.unwrap().to_string());

            url_params.push(language);
        }

        // Blacklist Flags
        if !self.blacklist_flags.is_empty() {

            temp.push(String::from("blacklistFlags="));

            for item in self.blacklist_flags.iter() {
                temp.push(item.to_string());
            }

            url_params.push(temp.join(","));
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

        // Join Parameters with the URL Query Delimiter and add the query to the API url
        let final_params = url_params.join("?");
        url.push_str(&final_params);
        Ok(url)
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
    fn working() {
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
}
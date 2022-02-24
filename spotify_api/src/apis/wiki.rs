use super::super::model;
use super::super::EasyAPI;
use serde_json::Value;

impl EasyAPI {
    ///  Gets the track names from a given album id
    pub fn  get_wiki_description(
        &mut self,
        search: String,
    ) -> Result<String, std::io::Error> {
        let mut result = String::new();
        let mut final_result = String::new();
        match self.command.get_wiki_description(search, &mut result) {
            Ok(_ok) => {}
            Err(error) => return Err(error),
        }
        //println!("{}", result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let extract = &v["query"]["pages"].as_object().unwrap().values().next().unwrap()["extract"];
        final_result = serde_json::to_string(&extract).unwrap();
        Ok(final_result)
    }
}
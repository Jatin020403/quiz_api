# Quiz API

## deploy:
1.generate api endpoint from Gemini
2. create folder .config with file config.toml
3. insert the following data 
  ```toml
  [env]
  API_ENDPOINT=""
  PROJECT_ID=""
  LOCATION_ID=""
  ```
4. run with
  ```sh
  cargo run 
  ```

## todo:
- [X] Connect the API for gemini
- [ ] Connect Mongo
- [ ] Design queries
- [ ] Make crud for all users

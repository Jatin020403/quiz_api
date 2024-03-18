# Quiz API

## deploy:
1.generate api endpoint from Gemini
2. create folder .config with file config.toml
3. insert the following data 
  ```toml
  [env]
  API_ENDPOINT="us-central1-aiplatform.googleapis.com"
  PROJECT_ID="united-helix-416211"
  LOCATION_ID="us-central1"
  ```
4. run with
  ```sh
  cargpo run 
  ```

## todo:
- [X] Connect the API for gemini
- [ ] Connect Mongo
- [ ] Design queries
- [ ] Make crud for all users

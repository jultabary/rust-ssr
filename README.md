# Requirements
- install rust and cargo (https://www.rust-lang.org/tools/install)

# Start the app

launch in a terminal `make build` then `make start` then open `http://localhost:8000`

# Objectives
Why do i not use Dioxus WASM to build a SPA ?

The WASM bundle is very heavy. I prefer a SSR approach which renders very light page.

# Description
![rust-ssr](https://github.com/jultabary/rust-ssr/assets/11232605/bdd6bfc3-e26b-4119-92b2-3ae87d3791db)

# Folder Description

- client:
  - api: controllers
  - web_app: html, javascript and css codes
- infrastructre: persistence layer
- domain: business object
- usecases

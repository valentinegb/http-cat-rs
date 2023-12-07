# `http_cat`

Rust wrapper around [http.cat](https://http.cat)

The simplest way to use `http_cat` is like so:

```rs
use http_cat::HttpCat;

#[tokio::main]
async fn main() {
    HttpCat::ImATeapot.get().await.unwrap().save("418.jpeg").unwrap();
}
```

This is the resulting image:

*place image here*

You can also optionally enable the `http` and/or `rocket` features to convert
those libraries' own status structures to `HttpCat` with `::try_from()`.

```toml
[dependencies]
http_cat = { version = "0.1.0", features = ["rocket"] }
```

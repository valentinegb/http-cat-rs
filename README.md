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

![418](https://github.com/valentinegb/http-cat-rs/assets/35977727/450dd2dc-5cf5-45de-a2a4-0c1d3e276efe)

You can also optionally enable the [`http`](https://github.com/hyperium/http)
and/or [`rocket`](https://github.com/SergioBenitez/Rocket) features to convert
those libraries' own status structures to `HttpCat` with `::try_from()`.

```toml
[dependencies]
http_cat = { version = "0.1.0", features = ["rocket"] }
```
